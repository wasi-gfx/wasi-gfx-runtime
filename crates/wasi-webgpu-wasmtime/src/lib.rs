// TODO: in this file:
// - Remove all calls to `Default::default()`. Instead, manually set them, and link to the spec stating what defaults should be used.
// - Implement all todos.
// - Remove all unwraps.
// - Implement all the drop handlers.

use std::{future::Future, sync::Arc};

use wasi_graphics_context_wasmtime::{AbstractBuffer, DisplayApi, DrawApi};
use wasmtime::component::HasData;
use wasmtime_wasi_io::IoView;
use wgpu_core::id::SurfaceId;

// ToCore trait used for resources, records, and variants.
// Into trait used for enums, since they never need table access.
mod enum_conversions;
mod to_core_conversions;

mod trait_impls;
mod wrapper_types;

/// Re-export of `wgpu_core` and `wgpu_types` so that runtime implementors don't need to keep track of what version of wgpu this crate is using.
pub mod reexports {
    pub use wgpu_core;
    pub use wgpu_types;
}

#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "android"),
    not(target_os = "windows"),
    not(target_os = "macos"),
    not(target_os = "ios"),
))]
pub(crate) type Backend = wgpu_core::api::Gl;

// needed for wasmtime::component::bindgen! as it only looks in the current crate.
pub(crate) use wgpu_core;
pub(crate) use wgpu_types;

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    imports: {
        "wasi:webgpu/webgpu/[method]gpu-buffer.map-async": async
    },
    with: {
        "wasi:io": wasmtime_wasi_io::bindings::wasi::io,
        "wasi:webgpu/webgpu/gpu-adapter": wgpu_core::id::AdapterId,
        "wasi:webgpu/webgpu/gpu-device": wrapper_types::Device,
        "wasi:webgpu/webgpu/gpu-queue": wgpu_core::id::QueueId,
        "wasi:webgpu/webgpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "wasi:webgpu/webgpu/gpu-render-pass-encoder": wrapper_types::RenderPassEncoder,
        "wasi:webgpu/webgpu/gpu-compute-pass-encoder": wrapper_types::ComputePassEncoder,
        "wasi:webgpu/webgpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "wasi:webgpu/webgpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "wasi:webgpu/webgpu/gpu-render-bundle-encoder": wrapper_types::RenderBundleEncoder,
        "wasi:webgpu/webgpu/gpu-render-bundle": wgpu_core::id::RenderBundleId,
        "wasi:webgpu/webgpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        "wasi:webgpu/webgpu/gpu-buffer": wrapper_types::Buffer,
        // "wasi:webgpu/webgpu/non-standard-buffer": wrapper_types::BufferPtr,
        "wasi:webgpu/webgpu/gpu-pipeline-layout": wgpu_core::id::PipelineLayoutId,
        "wasi:webgpu/webgpu/gpu-bind-group-layout": wgpu_core::id::BindGroupLayoutId,
        "wasi:webgpu/webgpu/gpu-sampler": wgpu_core::id::SamplerId,
        "wasi:webgpu/webgpu/gpu-supported-features": wgpu_types::Features,
        "wasi:webgpu/webgpu/gpu-texture": wgpu_core::id::TextureId,
        "wasi:webgpu/webgpu/gpu-compute-pipeline": wgpu_core::id::ComputePipelineId,
        "wasi:webgpu/webgpu/gpu-bind-group": wgpu_core::id::BindGroupId,
        "wasi:webgpu/webgpu/gpu-texture-view": wgpu_core::id::TextureViewId,
        "wasi:webgpu/webgpu/gpu-adapter-info": wgpu_types::AdapterInfo,
        "wasi:webgpu/webgpu/gpu-query-set": wgpu_core::id::QuerySetId,
        "wasi:webgpu/webgpu/gpu-supported-limits": wgpu_types::Limits,
        "wasi:webgpu/webgpu/record-gpu-pipeline-constant-value": wrapper_types::RecordGpuPipelineConstantValue,
        "wasi:graphics-context/graphics-context": wasi_graphics_context_wasmtime::wasi::graphics_context::graphics_context,
    },
});

struct WasiWebGpu<T: Send>(T);
impl<T: Send + 'static> HasData for WasiWebGpu<T> {
    type Data<'a> = WasiWebGpuImpl<&'a mut T>;
}
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiWebGpuView,
{
    wasi::webgpu::webgpu::add_to_linker::<_, WasiWebGpu<T>>(l, |x| WasiWebGpuImpl(x))?;
    Ok(())
}

pub trait WasiWebGpuView: IoView + Send {
    fn instance(&self) -> Arc<wgpu_core::global::Global>;

    /// Provide the ability to run closure on the UI thread.
    /// On platforms that don't require UI to run on the UI thread, this can just execute in place.
    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner>;
}

#[repr(transparent)]
pub struct WasiWebGpuImpl<T>(pub T);
impl<T: WasiWebGpuView> wasmtime_wasi_io::IoView for WasiWebGpuImpl<T> {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        T::table(&mut self.0)
    }
}

impl<T: ?Sized + WasiWebGpuView> WasiWebGpuView for &mut T {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        T::instance(self)
    }

    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner> {
        T::ui_thread_spawner(&self)
    }
}
impl<T: ?Sized + WasiWebGpuView> WasiWebGpuView for Box<T> {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        T::instance(self)
    }

    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner> {
        T::ui_thread_spawner(&self)
    }
}
impl<T: WasiWebGpuView> WasiWebGpuView for WasiWebGpuImpl<T> {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        T::instance(&self.0)
    }

    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner> {
        T::ui_thread_spawner(&self.0)
    }
}

pub trait MainThreadSpawner: Send + Sync + 'static {
    fn spawn<F, T>(&self, f: F) -> impl Future<Output = T>
    where
        F: FnOnce() -> T + Send + Sync + 'static,
        T: Send + Sync + 'static;
}

struct WebGpuSurface<GI, CS, I>
where
    I: AsRef<wgpu_core::global::Global>,
    GI: Fn() -> I,
    CS: Fn(&(dyn DisplayApi + Send + Sync)) -> SurfaceId,
{
    get_instance: GI,
    create_surface: CS,
    device_id: wgpu_core::id::DeviceId,
    // Might be needed one day for surface configuration.
    _adapter_id: wgpu_core::id::AdapterId,
    surface_id: Option<wgpu_core::id::SurfaceId>,
}

impl<GI, CS, I> DrawApi for WebGpuSurface<GI, CS, I>
where
    I: AsRef<wgpu_core::global::Global>,
    GI: Fn() -> I,
    CS: Fn(&(dyn DisplayApi + Send + Sync)) -> SurfaceId,
{
    fn get_current_buffer(&mut self) -> wasmtime::Result<AbstractBuffer> {
        let texture: wgpu_core::id::TextureId = (self.get_instance)()
            .as_ref()
            .surface_get_current_texture(self.surface_id.unwrap(), None)
            .unwrap()
            .texture
            .unwrap();
        let buff = Box::new(texture);
        let buff: AbstractBuffer = buff.into();
        Ok(buff)
    }

    fn present(&mut self) -> wasmtime::Result<()> {
        (self.get_instance)()
            .as_ref()
            .surface_present(self.surface_id.unwrap())
            .unwrap();
        Ok(())
    }

    fn display_api_ready(&mut self, display: &Box<dyn DisplayApi + Send + Sync>) {
        let surface_id = (self.create_surface)(display.as_ref());
        // TODO: fix this once user can pass in configuration options. For now just taking from `gpu.get-preferred-canvas-format()`.
        #[cfg(target_os = "android")]
        let swapchain_format = wgpu_types::TextureFormat::Rgba8Unorm;
        #[cfg(not(target_os = "android"))]
        let swapchain_format = wgpu_types::TextureFormat::Bgra8Unorm;

        // https://www.w3.org/TR/webgpu/#dictdef-gpucanvasconfiguration
        let config = wgpu_types::SurfaceConfiguration {
            format: swapchain_format,
            usage: wgpu_types::TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![],
            alpha_mode: wgpu_types::CompositeAlphaMode::Opaque,
            width: display.width(),
            height: display.height(),
            present_mode: wgpu_types::PresentMode::Fifo,
            // TODO: not sure what the correct value is
            desired_maximum_frame_latency: 2,
        };

        (self.get_instance)()
            .as_ref()
            .surface_configure(surface_id, self.device_id, &config);

        self.surface_id = Some(surface_id);
    }
}
