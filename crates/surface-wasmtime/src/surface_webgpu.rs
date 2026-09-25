use crate::surface::MainThreadSpawner;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::marker::PhantomData;
use std::sync::Arc;
use wasi_gfx::surface::surface_webgpu;
use wasi_webgpu_wasmtime::reexports::{wgpu_core, wgpu_types};
use wasmtime::{
    bail,
    component::{HasData, Resource},
};

wasmtime::component::bindgen!({
    world: "wasi-gfx:surface/webgpu-imports",
    require_store_data_send: true,
    imports: {
        default: trappable,
    },
    with: {
        "wasi-gfx:surface/surface": crate::surface::wasi_gfx::surface::surface,
        "wasi:webgpu/webgpu": wasi_webgpu_wasmtime::wasi::webgpu::webgpu,
        "wasi-gfx:surface/surface-webgpu.context": Context,
    },
});

// types
pub struct Context {
    pub(crate) surface: surface_webgpu::Surface,
    pub(crate) surface_id: wgpu_core::id::SurfaceId,
    pub(crate) configuration: Option<ContextConfiguration>,
    pub(crate) has_acquired_surface_texture: bool,
}

pub(crate) struct ContextConfiguration {
    device: wasi_webgpu_wasmtime::Device,
    format: wgpu_types::TextureFormat,
    usage: wgpu_types::TextureUsages,
    view_formats: Vec<wgpu_types::TextureFormat>,
}

// linker connection
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: SurfaceWebgpuCtxView,
{
    wasi_gfx::surface::surface_webgpu::add_to_linker::<_, HasSurfaceWebgpu<T::Spawner>>(
        l,
        T::surface_webgpu_ctx,
    )?;
    Ok(())
}

pub trait SurfaceWebgpuCtxView: Send {
    /// Spawner used to run main-thread-only wgpu calls (e.g. surface creation).
    type Spawner: MainThreadSpawner;
    fn surface_webgpu_ctx(&mut self) -> SurfaceWebgpuCtx<'_, Self::Spawner>;
}

pub struct SurfaceWebgpuCtx<'a, S: MainThreadSpawner> {
    pub table: &'a mut wasmtime_wasi::ResourceTable,
    pub instance: &'a Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global>,
    pub main_thread_spawner: &'a S,
}

struct HasSurfaceWebgpu<S>(PhantomData<S>);

impl<S: MainThreadSpawner> HasData for HasSurfaceWebgpu<S> {
    type Data<'a> = SurfaceWebgpuCtx<'a, S>;
}

// wasmtime trait impls
impl<'a, S: MainThreadSpawner> surface_webgpu::Host for SurfaceWebgpuCtx<'a, S> {}

impl<'a, S: MainThreadSpawner> surface_webgpu::HostContext for SurfaceWebgpuCtx<'a, S> {
    fn new(
        &mut self,
        surface: Resource<surface_webgpu::Surface>,
    ) -> wasmtime::Result<Resource<surface_webgpu::Context>> {
        let surface = self.table.get(&surface)?;
        let instance = Arc::clone(self.instance);

        let surface_id = futures::executor::block_on({
            let surface = surface.arc_clone();
            self.main_thread_spawner.spawn(move || {
                // SAFETY: The raw handles remain valid for the lifetime of the wgpu surface because
                // `Context` holds an `arc_clone()` of the surface alongside the `surface_id`.
                unsafe {
                    instance.instance_create_surface(
                        Some(surface.display_handle().unwrap().as_raw()),
                        surface.window_handle().unwrap().as_raw(),
                        None,
                    )
                }
            })
        })?;

        Ok(self.table.push(Context {
            surface: surface.arc_clone(),
            surface_id,
            configuration: None,
            has_acquired_surface_texture: false,
        })?)
    }

    fn configure(
        &mut self,
        context: Resource<surface_webgpu::Context>,
        configuration: surface_webgpu::ContextConfiguration,
    ) -> wasmtime::Result<()> {
        let device = self.table.get(&configuration.device)?.clone();
        let device_id = *device.device_id();

        let context = self.table.get_mut(&context)?;

        let format = configuration.format.into();
        let usage = match configuration
            .usage
            .unwrap_or(wasi_webgpu_wasmtime::wasi::webgpu::webgpu::GpuTextureUsage::RENDER_ATTACHMENT)
            .try_into()
        {
            Ok(usage) => usage,
            Err(e) => bail!("{e:#?}"),
        };
        let view_formats: Vec<wgpu_types::TextureFormat> = configuration
            .view_formats
            .into_iter()
            .flatten()
            .map(|f| f.into())
            .collect();
        let alpha_mode = configuration
            .alpha_mode
            .unwrap_or(wasi_webgpu_wasmtime::wasi::webgpu::webgpu::GpuCanvasAlphaMode::Opaque)
            .into();

        let err = self.instance.surface_configure(
            context.surface_id,
            device_id,
            &wgpu_types::SurfaceConfiguration {
                // present in WebGPU, same defaults https://www.w3.org/TR/webgpu/#dictdef-gpucanvasconfiguration
                format,
                usage,
                view_formats: view_formats.clone(),
                alpha_mode,
                // not present in WebGPU
                width: context.surface.width().max(1),
                height: context.surface.height().max(1),
                present_mode: wgpu_types::PresentMode::default(),
                desired_maximum_frame_latency: 2,
            },
        );
        if let Some(err) = err {
            bail!("{err:#?}")
        }

        context.configuration = Some(ContextConfiguration {
            device,
            format,
            usage,
            view_formats,
        });
        context.has_acquired_surface_texture = false;
        Ok(())
    }

    fn unconfigure(&mut self, context: Resource<surface_webgpu::Context>) -> wasmtime::Result<()> {
        let context = self.table.get_mut(&context)?;
        context.configuration = None;
        context.has_acquired_surface_texture = false;
        Ok(())
    }

    fn get_current_texture(
        &mut self,
        context: Resource<surface_webgpu::Context>,
    ) -> wasmtime::Result<Resource<surface_webgpu::GpuTexture>> {
        let (surface_id, device, current_width, current_height, format, usage, view_formats) = {
            let context = self.table.get(&context)?;
            let Some(configuration) = &context.configuration else {
                bail!("Not configured")
            };
            (
                context.surface_id,
                configuration.device.clone(),
                context.surface.width().max(1),
                context.surface.height().max(1),
                configuration.format,
                configuration.usage,
                configuration.view_formats.clone(),
            )
        };

        let device_id = *device.device_id();

        let surface_output = self
            .instance
            .surface_get_current_texture(surface_id, None);

        let (texture_id, has_acquired_surface_texture) = match surface_output {
            Ok(output) if output.texture.is_some() => {
                (output.texture.unwrap(), true)
            }
            Ok(_) => {
                // When occluded or backgrounded, Metal/wgpu skips surface texture acquisition.
                // Return an offscreen fallback texture so guest render passes still succeed.
                let fallback = create_fallback_texture(
                    self.instance,
                    device_id,
                    current_width,
                    current_height,
                    format,
                    usage,
                    &view_formats,
                )?;
                (fallback, false)
            }
            Err(wgpu_core::present::SurfaceError::AlreadyAcquired) => {
                // Fallback if called multiple times in one frame without present.
                let fallback = create_fallback_texture(
                    self.instance,
                    device_id,
                    current_width,
                    current_height,
                    format,
                    usage,
                    &view_formats,
                )?;
                (fallback, false)
            }
            Err(err) => {
                bail!("{err:#?}")
            }
        };

        {
            let context = self.table.get_mut(&context)?;
            context.has_acquired_surface_texture = has_acquired_surface_texture;
        }

        // SAFETY: Both real surface texture and fallback texture belong to this device.
        let texture = unsafe { device.connect_texture(texture_id) };

        Ok(self.table.push(texture)?)
    }

    fn present(&mut self, context: Resource<surface_webgpu::Context>) -> wasmtime::Result<()> {
        let context = self.table.get_mut(&context)?;

        // Only present if a real surface texture was acquired for this frame.
        if context.has_acquired_surface_texture {
            context.has_acquired_surface_texture = false;
            match self.instance.surface_present(context.surface_id) {
                Ok(_) => Ok(()),
                Err(wgpu_core::present::SurfaceError::AlreadyAcquired) => Ok(()),
                Err(err) => bail!("{err:#?}"),
            }
        } else {
            Ok(())
        }
    }

    fn drop(&mut self, surface: Resource<surface_webgpu::Context>) -> wasmtime::Result<()> {
        self.table.delete(surface)?;
        Ok(())
    }
}

fn create_fallback_texture(
    instance: &wgpu_core::global::Global,
    device_id: wgpu_core::id::DeviceId,
    width: u32,
    height: u32,
    format: wgpu_types::TextureFormat,
    usage: wgpu_types::TextureUsages,
    view_formats: &[wgpu_types::TextureFormat],
) -> wasmtime::Result<wgpu_core::id::TextureId> {
    let desc = wgpu_types::TextureDescriptor {
        label: Some(std::borrow::Cow::Borrowed("surface fallback texture")),
        size: wgpu_types::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu_types::TextureDimension::D2,
        format,
        usage,
        view_formats: view_formats.to_vec(),
    };
    let (texture_id, err) = instance.device_create_texture(device_id, &desc, None);
    if let Some(err) = err {
        bail!("Failed to create fallback texture for surface: {err:#?}");
    }
    Ok(texture_id)
}
