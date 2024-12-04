// TODO: in this file:
// - Remove all calls to `Default::default()`. Instead, manually set them, and link to the spec stating what defaults should be used.
// - Implement all todos.
// - Remove all unwraps.
// - Implement all the drop handlers.

use std::{future::Future, sync::Arc};

use wasi_graphics_context_wasmtime::{AbstractBuffer, DisplayApi, DrawApi};
use wasmtime_wasi::WasiView;
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

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type Backend = wgpu_core::api::Vulkan;

#[cfg(target_os = "windows")]
pub(crate) type Backend = wgpu_core::api::Dx12;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) type Backend = wgpu_core::api::Metal;

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
    async: {
        only_imports: [
            "[method]gpu-buffer.map-async",
        ],
    },
    with: {
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
        "wasi:webgpu/webgpu/non-standard-buffer": wrapper_types::BufferPtr,
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
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime,
    },
});

fn type_annotate<T, F>(val: F) -> F
where
    F: Fn(&mut T) -> WasiWebGpuImpl<&mut T>,
{
    val
}
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiWebGpuView,
{
    let closure = type_annotate::<T, _>(|t| WasiWebGpuImpl(t));
    wasi::webgpu::webgpu::add_to_linker_get_host(l, closure)?;
    Ok(())
}

pub trait WasiWebGpuView: WasiView {
    fn instance(&self) -> Arc<wgpu_core::global::Global>;

    /// Provide the ability to run closure on the UI thread.
    /// On platforms that don't require UI to run on the UI thread, this can just execute in place.
    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner>;
}

pub struct WasiWebGpuImpl<T>(pub T);

impl<T: WasiView> WasiView for WasiWebGpuImpl<T> {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        self.0.table()
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        self.0.ctx()
    }
}

impl<T: WasiWebGpuView> WasiWebGpuView for WasiWebGpuImpl<T> {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        self.0.instance()
    }

    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner + 'static> {
        self.0.ui_thread_spawner()
    }
}

impl<T: ?Sized + WasiWebGpuView> WasiWebGpuView for &mut T {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        T::instance(self)
    }

    fn ui_thread_spawner(&self) -> Box<impl MainThreadSpawner + 'static> {
        T::ui_thread_spawner(self)
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
    adapter_id: wgpu_core::id::AdapterId,
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
            .surface_get_current_texture::<crate::Backend>(self.surface_id.unwrap(), None)
            .unwrap()
            .texture_id
            .unwrap();
        let buff = Box::new(texture);
        let buff: AbstractBuffer = buff.into();
        Ok(buff)
    }

    fn present(&mut self) -> wasmtime::Result<()> {
        (self.get_instance)()
            .as_ref()
            .surface_present::<crate::Backend>(self.surface_id.unwrap())
            .unwrap();
        Ok(())
    }

    fn display_api_ready(&mut self, display: &Box<dyn DisplayApi + Send + Sync>) {
        let surface_id = (self.create_surface)(display.as_ref());

        let swapchain_capabilities = (self.get_instance)()
            .as_ref()
            .surface_get_capabilities::<crate::Backend>(surface_id, self.adapter_id)
            .unwrap();
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu_types::SurfaceConfiguration {
            usage: wgpu_types::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: display.width(),
            height: display.height(),
            present_mode: wgpu_types::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![swapchain_format],
            // TODO: not sure what the correct value is
            desired_maximum_frame_latency: 2,
        };

        (self.get_instance)()
            .as_ref()
            .surface_configure::<crate::Backend>(surface_id, self.device_id, &config);

        self.surface_id = Some(surface_id);
    }
}
