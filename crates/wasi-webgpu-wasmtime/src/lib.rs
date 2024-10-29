// TODO: in this file:
// - Remove all calls to `Default::default()`. Instead, manually set them, and link to the spec stating what defaults should be used.
// - Implement all todos.
// - Remove all unwraps.
// - Implement all the drop handlers.

use callback_future::CallbackFuture;
use core::slice;
use futures::executor::block_on;
use std::borrow::Cow;
use std::ptr::NonNull;
use std::sync::Arc;
use std::{future::Future, mem};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;
use wgpu_core::id::SurfaceId;

use crate::wasi::webgpu::webgpu;
use wasi_graphics_context_wasmtime::{AbstractBuffer, Context, DisplayApi, DrawApi};

use self::to_core_conversions::ToCore;

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

pub type RenderPass = wgpu_core::command::RenderPass<crate::Backend>;
pub type ComputePass = wgpu_core::command::ComputePass<crate::Backend>;

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
        "wasi:webgpu/webgpu/gpu-device": Device,
        "wasi:webgpu/webgpu/gpu-queue": wgpu_core::id::QueueId,
        "wasi:webgpu/webgpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "wasi:webgpu/webgpu/gpu-render-pass-encoder": RenderPass,
        "wasi:webgpu/webgpu/gpu-compute-pass-encoder": ComputePass,
        "wasi:webgpu/webgpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "wasi:webgpu/webgpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "wasi:webgpu/webgpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        "wasi:webgpu/webgpu/gpu-buffer": Buffer,
        "wasi:webgpu/webgpu/non-standard-buffer": BufferPtr,
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

pub struct WebGpuSurface<GI, CS, I>
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

// ToCore trait used for resources, records, and variants.
// Into trait used for enums, since they never need table access.
mod enum_conversions;
mod to_core_conversions;

pub struct BufferPtr {
    // See https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general/topic/Should.20wasi.20resources.20be.20stored.20behind.20a.20mutex.3F
    pub(crate) ptr: NonNull<u8>,
    pub(crate) len: u64,
}
impl BufferPtr {
    pub fn slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len as usize) }
    }
    pub fn slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len as usize) }
    }
}
unsafe impl Send for BufferPtr {}
unsafe impl Sync for BufferPtr {}

pub struct Buffer {
    buffer: wgpu_core::id::BufferId,
    size: u64,
}

#[derive(Clone, Copy)]
pub struct Device {
    pub device: wgpu_core::id::DeviceId,
    pub queue: wgpu_core::id::QueueId,
    // only needed when calling surface.get_capabilities in connect_graphics_context. If table would have a way to get parent from child, we could get it from device.
    pub adapter: wgpu_core::id::AdapterId,
}

impl<T: WasiWebGpuView> webgpu::Host for WasiWebGpuImpl<T> {
    fn get_gpu(&mut self) -> Resource<webgpu::Gpu> {
        Resource::new_own(0)
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuColorWrite for WasiWebGpuImpl<T> {
    fn red(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ColorWrites::RED.bits()
    }

    fn green(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ColorWrites::GREEN.bits()
    }

    fn blue(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ColorWrites::BLUE.bits()
    }

    fn alpha(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ColorWrites::ALPHA.bits()
    }

    fn all(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ColorWrites::ALL.bits()
    }

    fn drop(&mut self, _self_: Resource<webgpu::GpuColorWrite>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostRecordGpuPipelineConstantValue for WasiWebGpuImpl<T> {
    fn new(&mut self) -> Resource<webgpu::RecordGpuPipelineConstantValue> {
        todo!()
    }

    fn add(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
        _value: webgpu::GpuPipelineConstantValue,
    ) {
        todo!()
    }

    // fn get(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>, _key: String) -> Option<webgpu::GpuPipelineConstantValue> {
    fn get(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
    ) -> webgpu::GpuPipelineConstantValue {
        todo!()
    }

    fn has(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
    ) -> bool {
        todo!()
    }

    fn remove(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>, _key: String) {
        todo!()
    }

    fn keys(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>) -> Vec<String> {
        todo!()
    }

    fn values(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> Vec<webgpu::GpuPipelineConstantValue> {
        todo!()
    }

    // fn entries(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>) -> Vec<(String, webgpu::GpuPipelineConstantValue)> {
    fn entries(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> (String, webgpu::GpuPipelineConstantValue) {
        todo!()
    }

    fn drop(
        &mut self,
        _self_: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuShaderStage for WasiWebGpuImpl<T> {
    fn vertex(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ShaderStages::VERTEX.bits()
    }

    fn fragment(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ShaderStages::FRAGMENT.bits()
    }

    fn compute(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::ShaderStages::COMPUTE.bits()
    }

    fn drop(&mut self, _: Resource<webgpu::GpuShaderStage>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuTextureUsage for WasiWebGpuImpl<T> {
    fn copy_src(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::TextureUsages::COPY_SRC.bits()
    }
    fn copy_dst(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::TextureUsages::COPY_DST.bits()
    }
    fn texture_binding(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::TextureUsages::TEXTURE_BINDING.bits()
    }
    fn storage_binding(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::TextureUsages::STORAGE_BINDING.bits()
    }
    fn render_attachment(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::TextureUsages::RENDER_ATTACHMENT.bits()
    }
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<webgpu::GpuTextureUsage>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuMapMode for WasiWebGpuImpl<T> {
    fn read(&mut self) -> webgpu::GpuFlagsConstant {
        // https://www.w3.org/TR/webgpu/#buffer-mapping
        0x0001
    }
    fn write(&mut self) -> webgpu::GpuFlagsConstant {
        // https://www.w3.org/TR/webgpu/#buffer-mapping
        0x0002
    }
    fn drop(&mut self, _rep: Resource<webgpu::GpuMapMode>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBufferUsage for WasiWebGpuImpl<T> {
    fn map_read(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::MAP_READ.bits()
    }
    fn map_write(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::MAP_WRITE.bits()
    }
    fn copy_src(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::COPY_SRC.bits()
    }
    fn copy_dst(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::COPY_DST.bits()
    }
    fn index(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::INDEX.bits()
    }
    fn vertex(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::VERTEX.bits()
    }
    fn uniform(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::UNIFORM.bits()
    }
    fn storage(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::STORAGE.bits()
    }
    fn indirect(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::INDIRECT.bits()
    }
    fn query_resolve(&mut self) -> webgpu::GpuFlagsConstant {
        wgpu_types::BufferUsages::QUERY_RESOLVE.bits()
    }
    fn drop(&mut self, _rep: Resource<webgpu::GpuBufferUsage>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostRecordGpuSize64 for WasiWebGpuImpl<T> {
    fn new(&mut self) -> Resource<webgpu::RecordGpuSize64> {
        todo!()
    }
    fn add(
        &mut self,
        _self_: Resource<webgpu::RecordGpuSize64>,
        _key: String,
        _value: webgpu::GpuSize64,
    ) {
        todo!()
    }
    fn get(
        &mut self,
        _self_: Resource<webgpu::RecordGpuSize64>,
        _key: String,
    ) -> webgpu::GpuSize64 {
        todo!()
    }
    fn has(&mut self, _self_: Resource<webgpu::RecordGpuSize64>, _key: String) -> bool {
        todo!()
    }
    fn remove(&mut self, _self_: Resource<webgpu::RecordGpuSize64>, _key: String) {
        todo!()
    }
    fn keys(&mut self, _self_: Resource<webgpu::RecordGpuSize64>) -> Vec<String> {
        todo!()
    }
    fn values(&mut self, _self_: Resource<webgpu::RecordGpuSize64>) -> Vec<webgpu::GpuSize64> {
        todo!()
    }
    fn entries(
        &mut self,
        _self_: Resource<webgpu::RecordGpuSize64>,
    ) -> (String, webgpu::GpuSize64) {
        todo!()
    }
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<webgpu::RecordGpuSize64>,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostNonStandardBuffer for WasiWebGpuImpl<T> {
    fn get(&mut self, buffer: Resource<webgpu::NonStandardBuffer>) -> Vec<u8> {
        let buffer = self.0.table().get_mut(&buffer).unwrap();
        buffer.slice_mut().to_vec()
    }

    fn set(&mut self, buffer: Resource<webgpu::NonStandardBuffer>, val: Vec<u8>) {
        let buffer = self.0.table().get_mut(&buffer).unwrap();
        buffer.slice_mut().copy_from_slice(&val);
    }

    fn drop(&mut self, _rep: Resource<webgpu::NonStandardBuffer>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuDevice for WasiWebGpuImpl<T> {
    fn connect_graphics_context(&mut self, device: Resource<Device>, context: Resource<Context>) {
        let device = self.0.table().get(&device).unwrap();
        let device_id = device.device;
        let adapter_id = device.adapter;

        let instance = Arc::downgrade(&self.0.instance());
        let surface_creator = self.0.ui_thread_spawner();

        let context = self.0.table().get_mut(&context).unwrap();

        let surface = WebGpuSurface {
            get_instance: {
                let instance = instance.clone();
                move || instance.upgrade().unwrap()
            },
            create_surface: {
                let instance = instance.clone();
                move |display: &(dyn DisplayApi + Send + Sync)| {
                    let instance = instance.upgrade().unwrap();

                    // TODO: make spawn behave similar to `std::thread::scope` so that we don't have to unsafely transmute display to `&'static`.
                    // Something like the following:
                    // ```rust
                    // let surface_id = std::thread::scope(|s| {
                    //     s.spawn(move || unsafe {
                    //         instance
                    //             .instance_create_surface(
                    //                 display.display_handle().unwrap().as_raw(),
                    //                 display.window_handle().unwrap().as_raw(),
                    //                 None,
                    //             )
                    //             .unwrap()
                    //     }).join().unwrap()
                    // });
                    // surface_id
                    // ```

                    let display: &'static (dyn DisplayApi + Send + Sync) =
                        unsafe { mem::transmute(display) };
                    block_on(surface_creator.spawn(move || unsafe {
                        instance
                            .instance_create_surface(
                                display.display_handle().unwrap().as_raw(),
                                display.window_handle().unwrap().as_raw(),
                                None,
                            )
                            .unwrap()
                    }))
                }
            },
            device_id,
            adapter_id,
            surface_id: None,
        };

        context.connect_draw_api(Box::new(surface));
    }

    fn configure(
        &mut self,
        _device: Resource<Device>,
        _descriptor: webgpu::GpuDeviceConfiguration,
    ) {
        todo!()
    }

    fn create_command_encoder(
        &mut self,
        device: Resource<Device>,
        descriptor: Option<webgpu::GpuCommandEncoderDescriptor>,
    ) -> Resource<wgpu_core::id::CommandEncoderId> {
        let device = self.0.table().get(&device).unwrap().device;

        let command_encoder = core_result(
            self.0
                .instance()
                .device_create_command_encoder::<crate::Backend>(
                    device,
                    &descriptor
                        .map(|d| d.to_core(&self.0.table()))
                        .unwrap_or_default(),
                    None,
                ),
        )
        .unwrap();

        self.0.table().push(command_encoder).unwrap()
    }

    fn create_shader_module(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuShaderModuleDescriptor,
    ) -> Resource<webgpu::GpuShaderModule> {
        let device = self.0.table().get(&device).unwrap().device;

        let code =
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Owned(descriptor.code.to_owned()));
        let shader = core_result(
            self.0
                .instance()
                .device_create_shader_module::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.0.table()),
                    code,
                    None,
                ),
        )
        .unwrap();

        self.0.table().push(shader).unwrap()
    }

    fn create_render_pipeline(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> Resource<wgpu_core::id::RenderPipelineId> {
        let host_device = self.0.table().get(&device).unwrap().device;
        let render_pipeline = core_result(
            self.0
                .instance()
                .device_create_render_pipeline::<crate::Backend>(
                    host_device,
                    &descriptor.to_core(&self.0.table()),
                    None,
                    None,
                ),
        )
        .unwrap();

        self.0.table().push_child(render_pipeline, &device).unwrap()
    }

    fn queue(&mut self, device: Resource<Device>) -> Resource<wgpu_core::id::QueueId> {
        let queue = self.0.table().get(&device).unwrap().queue;
        self.0.table().push(queue).unwrap()
    }

    fn features(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> Resource<webgpu::GpuSupportedFeatures> {
        let device = self.0.table().get(&device).unwrap().device;
        let features = self
            .0
            .instance()
            .device_features::<crate::Backend>(device)
            .unwrap();
        self.0.table().push(features).unwrap()
    }

    fn limits(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> Resource<webgpu::GpuSupportedLimits> {
        let device = self.0.table().get(&device).unwrap().device;
        let limits = self
            .0
            .instance()
            .device_limits::<crate::Backend>(device)
            .unwrap();
        self.0.table().push(limits).unwrap()
    }

    fn destroy(&mut self, _device: Resource<webgpu::GpuDevice>) {
        todo!()
    }

    fn create_buffer(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBufferDescriptor,
    ) -> Resource<webgpu::GpuBuffer> {
        let device = self.0.table().get(&device).unwrap().device;

        let size = descriptor.size;
        let buffer = core_result(self.0.instance().device_create_buffer::<crate::Backend>(
            device,
            &descriptor.to_core(&self.0.table()),
            None,
        ))
        .unwrap();

        let buffer = Buffer { buffer, size };

        self.0.table().push(buffer).unwrap()
    }

    fn create_texture(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuTextureDescriptor,
    ) -> Resource<webgpu::GpuTexture> {
        let device = self.0.table().get(&device).unwrap().device;
        let texture = core_result(self.0.instance().device_create_texture::<crate::Backend>(
            device,
            &descriptor.to_core(&self.0.table()),
            None,
        ))
        .unwrap();

        self.0.table().push(texture).unwrap()
    }

    fn create_sampler(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: Option<webgpu::GpuSamplerDescriptor>,
    ) -> Resource<webgpu::GpuSampler> {
        let device = self.0.table().get(&device).unwrap().device;

        let descriptor = descriptor.unwrap();

        let sampler = core_result(self.0.instance().device_create_sampler::<crate::Backend>(
            device,
            &descriptor.to_core(&self.0.table()),
            None,
        ))
        .unwrap();

        self.0.table().push(sampler).unwrap()
    }

    fn create_bind_group_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupLayoutDescriptor,
    ) -> Resource<webgpu::GpuBindGroupLayout> {
        let device = self.0.table().get(&device).unwrap().device;

        let bind_group_layout = core_result(
            self.0
                .instance()
                .device_create_bind_group_layout::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.0.table()),
                    None,
                ),
        )
        .unwrap();

        self.0.table().push(bind_group_layout).unwrap()
    }

    fn create_pipeline_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuPipelineLayoutDescriptor,
    ) -> Resource<webgpu::GpuPipelineLayout> {
        let device = self.0.table().get(&device).unwrap().device;

        let pipeline_layout = core_result(
            self.0
                .instance()
                .device_create_pipeline_layout::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.0.table()),
                    None,
                ),
        )
        .unwrap();

        self.0.table().push(pipeline_layout).unwrap()
    }

    fn create_bind_group(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupDescriptor,
    ) -> Resource<webgpu::GpuBindGroup> {
        let device = self.0.table().get(&device).unwrap().device;

        let bind_group = core_result(
            self.0
                .instance()
                .device_create_bind_group::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.0.table()),
                    None,
                ),
        )
        .unwrap();

        self.0.table().push(bind_group).unwrap()
    }

    fn create_compute_pipeline(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> Resource<webgpu::GpuComputePipeline> {
        let device = self.0.table().get(&device).unwrap().device;
        let compute_pipeline = core_result(
            self.0
                .instance()
                .device_create_compute_pipeline::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.0.table()),
                    None,
                    None,
                ),
        )
        .unwrap();
        self.0.table().push(compute_pipeline).unwrap()
    }

    fn create_compute_pipeline_async(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> Resource<webgpu::GpuComputePipeline> {
        todo!()
    }

    fn create_render_pipeline_async(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> Resource<wgpu_core::id::RenderPipelineId> {
        todo!()
    }

    fn create_render_bundle_encoder(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuRenderBundleEncoderDescriptor,
    ) -> Resource<webgpu::GpuRenderBundleEncoder> {
        todo!()
    }

    fn create_query_set(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuQuerySetDescriptor,
    ) -> Resource<webgpu::GpuQuerySet> {
        todo!()
    }

    fn label(&mut self, _device: Resource<webgpu::GpuDevice>) -> String {
        todo!()
    }

    fn set_label(&mut self, _device: Resource<webgpu::GpuDevice>, _label: String) -> () {
        todo!()
    }

    fn lost(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
    ) -> Resource<webgpu::GpuDeviceLostInfo> {
        todo!()
    }

    fn push_error_scope(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _filter: webgpu::GpuErrorFilter,
    ) -> () {
        todo!()
    }

    fn pop_error_scope(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
    ) -> Option<Resource<webgpu::GpuError>> {
        todo!()
    }

    fn uncaptured_errors(&mut self, _device: Resource<webgpu::GpuDevice>) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTexture for WasiWebGpuImpl<T> {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<AbstractBuffer>,
    ) -> Resource<wgpu_core::id::TextureId> {
        let host_buffer = self.0.table().delete(buffer).unwrap();
        let host_buffer: wgpu_core::id::TextureId = host_buffer.inner_type();
        self.0.table().push(host_buffer).unwrap()
    }

    fn create_view(
        &mut self,
        texture: Resource<wgpu_core::id::TextureId>,
        descriptor: Option<webgpu::GpuTextureViewDescriptor>,
    ) -> Resource<wgpu_core::id::TextureViewId> {
        let texture_id = *self.0.table().get(&texture).unwrap();
        let texture_view = core_result(
            self.0.instance().texture_create_view::<crate::Backend>(
                texture_id,
                &descriptor
                    .map(|d| d.to_core(&self.0.table()))
                    .unwrap_or_default(),
                None,
            ),
        )
        .unwrap();
        self.0.table().push(texture_view).unwrap()
    }

    fn drop(&mut self, _rep: Resource<wgpu_core::id::TextureId>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }

    fn destroy(&mut self, _self_: Resource<webgpu::GpuTexture>) {
        todo!()
    }

    fn width(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuIntegerCoordinateOut {
        todo!()
    }

    fn height(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuIntegerCoordinateOut {
        todo!()
    }

    fn depth_or_array_layers(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> webgpu::GpuIntegerCoordinateOut {
        todo!()
    }

    fn mip_level_count(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> webgpu::GpuIntegerCoordinateOut {
        todo!()
    }

    fn sample_count(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuSize32Out {
        todo!()
    }

    fn dimension(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuTextureDimension {
        todo!()
    }

    fn format(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuTextureFormat {
        todo!()
    }

    fn usage(&mut self, _self_: Resource<webgpu::GpuTexture>) -> webgpu::GpuFlagsConstant {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuTexture>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuTexture>, _label: String) {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTextureView for WasiWebGpuImpl<T> {
    fn drop(&mut self, _rep: Resource<wgpu_core::id::TextureViewId>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn label(&mut self, _self_: Resource<wgpu_core::id::TextureViewId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::TextureViewId>, _label: String) {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandBuffer for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<wgpu_core::id::CommandBufferId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::CommandBufferId>, _label: String) {
        todo!()
    }

    fn drop(&mut self, command_buffer: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        self.0.table().delete(command_buffer).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuShaderModule for WasiWebGpuImpl<T> {
    fn get_compilation_info(
        &mut self,
        _self_: Resource<wgpu_core::id::ShaderModuleId>,
    ) -> Resource<webgpu::GpuCompilationInfo> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<wgpu_core::id::ShaderModuleId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::ShaderModuleId>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuRenderPipeline for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<wgpu_core::id::RenderPipelineId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::RenderPipelineId>, _label: String) {
        todo!()
    }

    fn get_bind_group_layout(
        &mut self,
        _self_: Resource<wgpu_core::id::RenderPipelineId>,
        _index: u32,
    ) -> Resource<webgpu::GpuBindGroupLayout> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuAdapter for WasiWebGpuImpl<T> {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
        descriptor: Option<webgpu::GpuDeviceDescriptor>,
    ) -> Resource<webgpu::GpuDevice> {
        let adapter_id = *self.0.table().get(&adapter).unwrap();

        let (device_id, queue_id) = core_results_2(
            self.0.instance().adapter_request_device::<crate::Backend>(
                adapter_id,
                &descriptor
                    .map(|d| d.to_core(&self.0.table()))
                    .unwrap_or_default(),
                None,
                None,
                None,
            ),
        )
        .unwrap();

        let device = self
            .0
            .table()
            .push(Device {
                device: device_id,
                queue: queue_id,
                adapter: adapter_id,
            })
            .unwrap();

        device
    }

    fn drop(&mut self, _adapter: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn features(
        &mut self,
        adapter: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::component::Resource<webgpu::GpuSupportedFeatures> {
        let adapter = *self.0.table().get(&adapter).unwrap();
        let features = self
            .instance()
            .adapter_features::<crate::Backend>(adapter)
            .unwrap();
        self.0.table().push(features).unwrap()
    }

    fn limits(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> Resource<webgpu::GpuSupportedLimits> {
        let adapter = *self.0.table().get(&adapter).unwrap();
        let limits = self
            .0
            .instance()
            .adapter_limits::<crate::Backend>(adapter)
            .unwrap();
        self.0.table().push(limits).unwrap()
    }

    fn is_fallback_adapter(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> bool {
        todo!()
    }

    fn info(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> Resource<webgpu::GpuAdapterInfo> {
        let adapter_id = *self.0.table().get(&adapter).unwrap();
        let info = self
            .0
            .instance()
            .adapter_get_info::<crate::Backend>(adapter_id)
            .unwrap();
        let info = self.0.table().push(info).unwrap();
        info
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuQueue for WasiWebGpuImpl<T> {
    fn submit(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) {
        let command_buffers = val
            .into_iter()
            .map(|buffer| *self.0.table().get(&buffer).unwrap())
            .collect::<Vec<_>>();
        let queue = *self.0.table().get(&queue).unwrap();
        self.0
            .instance()
            .queue_submit::<crate::Backend>(queue, &command_buffers)
            .unwrap();
    }

    fn on_submitted_work_done(&mut self, _self_: Resource<wgpu_core::id::QueueId>) {
        todo!()
    }

    fn write_buffer(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        buffer: Resource<webgpu::GpuBuffer>,
        buffer_offset: webgpu::GpuSize64,
        data_offset: Option<webgpu::GpuSize64>,
        data: Vec<u8>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let queue = *self.0.table().get(&queue).unwrap();
        let buffer = self.0.table().get(&buffer).unwrap().buffer;
        let mut data = &data[..];
        if let Some(data_offset) = data_offset {
            let data_offset = data_offset as usize;
            data = &data[data_offset..];
        }
        if let Some(size) = size {
            let size = size as usize;
            data = &data[..size];
        }
        self.0
            .instance()
            .queue_write_buffer::<crate::Backend>(queue, buffer, buffer_offset, &data)
            .unwrap();
    }

    fn write_texture(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        destination: webgpu::GpuImageCopyTexture,
        data: Vec<u8>,
        data_layout: webgpu::GpuImageDataLayout,
        size: webgpu::GpuExtent3D,
    ) {
        let queue = *self.0.table().get(&queue).unwrap();
        self.0
            .instance()
            .queue_write_texture::<crate::Backend>(
                queue,
                &destination.to_core(&self.0.table()),
                &data,
                &data_layout.to_core(&self.0.table()),
                &size.to_core(&self.0.table()),
            )
            .unwrap();
    }

    fn label(&mut self, _self_: Resource<wgpu_core::id::QueueId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::QueueId>, _label: String) {
        todo!()
    }

    fn drop(&mut self, queue: Resource<wgpu_core::id::QueueId>) -> wasmtime::Result<()> {
        self.0.table().delete(queue).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandEncoder for WasiWebGpuImpl<T> {
    fn begin_render_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: webgpu::GpuRenderPassDescriptor,
    ) -> Resource<webgpu::GpuRenderPassEncoder> {
        let command_encoder = *self.0.table().get(&command_encoder).unwrap();
        // can't use to_core because depth_stencil_attachment is Option<&x>.
        let depth_stencil_attachment = descriptor
            .depth_stencil_attachment
            .map(|d| d.to_core(&self.0.table()));
        let descriptor = wgpu_core::command::RenderPassDescriptor {
            label: descriptor.label.map(|l| l.into()),
            color_attachments: descriptor
                .color_attachments
                .into_iter()
                .map(|c| c.map(|c| c.to_core(&self.0.table())))
                .collect::<Vec<_>>()
                .into(),
            depth_stencil_attachment: depth_stencil_attachment.as_ref(),
            // timestamp_writes: self.timestamp_writes,
            // occlusion_query_set: self.occlusion_query_set,
            // TODO: self.max_draw_count not used
            // TODO: remove default
            ..Default::default()
        };
        let render_pass = core_result_t(
            self.0
                .instance()
                .command_encoder_create_render_pass::<crate::Backend>(command_encoder, &descriptor),
        )
        .unwrap();

        self.0.table().push(render_pass).unwrap()
    }

    fn finish(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuCommandBufferDescriptor>,
    ) -> Resource<webgpu::GpuCommandBuffer> {
        let command_encoder = *self.0.table().get(&command_encoder).unwrap();
        let command_buffer = core_result(
            self.0.instance().command_encoder_finish::<crate::Backend>(
                command_encoder,
                &descriptor
                    .map(|d| d.to_core(&self.0.table()))
                    .unwrap_or_default(),
            ),
        )
        .unwrap();
        self.0.table().push(command_buffer).unwrap()
    }

    fn drop(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<()> {
        self.0.table().delete(command_encoder).unwrap();
        Ok(())
    }

    fn begin_compute_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuComputePassDescriptor>,
    ) -> Resource<webgpu::GpuComputePassEncoder> {
        let command_encoder = *self.0.table().get(&command_encoder).unwrap();
        let compute_pass = core_result_t(
            self.0
                .instance()
                .command_encoder_create_compute_pass::<crate::Backend>(
                    command_encoder,
                    &wgpu_core::command::ComputePassDescriptor {
                        label: Default::default(),
                        timestamp_writes: descriptor
                            .map(|d| d.timestamp_writes.map(|tw| tw.to_core(&self.0.table())))
                            .flatten()
                            .as_ref(),
                    },
                ),
        )
        .unwrap();
        self.0.table().push(compute_pass).unwrap()
    }

    fn copy_buffer_to_buffer(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        source: Resource<webgpu::GpuBuffer>,
        source_offset: webgpu::GpuSize64,
        destination: Resource<webgpu::GpuBuffer>,
        destination_offset: webgpu::GpuSize64,
        size: webgpu::GpuSize64,
    ) {
        let command_encoder = *self.0.table().get(&command_encoder).unwrap();
        let source = self.0.table().get(&source).unwrap().buffer;
        let destination = self.0.table().get(&destination).unwrap().buffer;
        self.0
            .instance()
            .command_encoder_copy_buffer_to_buffer::<crate::Backend>(
                command_encoder,
                source,
                source_offset,
                destination,
                destination_offset,
                size,
            )
            .unwrap();
    }

    fn copy_buffer_to_texture(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: webgpu::GpuImageCopyBuffer,
        _destination: webgpu::GpuImageCopyTexture,
        _copy_size: webgpu::GpuExtent3D,
    ) {
        todo!()
    }

    fn copy_texture_to_buffer(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        source: webgpu::GpuImageCopyTexture,
        destination: webgpu::GpuImageCopyBuffer,
        copy_size: webgpu::GpuExtent3D,
    ) {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        self.instance()
            .command_encoder_copy_texture_to_buffer::<crate::Backend>(
                command_encoder,
                &source.to_core(&self.table()),
                &destination.to_core(&self.table()),
                &copy_size.to_core(&self.table()),
            )
            .unwrap();
    }

    fn copy_texture_to_texture(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: webgpu::GpuImageCopyTexture,
        _destination: webgpu::GpuImageCopyTexture,
        _copy_size: webgpu::GpuExtent3D,
    ) {
        todo!()
    }

    fn clear_buffer(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) {
        todo!()
    }

    fn resolve_query_set(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _query_set: Resource<webgpu::GpuQuerySet>,
        _first_query: webgpu::GpuSize32,
        _query_count: webgpu::GpuSize32,
        _destination: Resource<webgpu::GpuBuffer>,
        _destination_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn label(&mut self, command_encoder: Resource<wgpu_core::id::CommandEncoderId>) -> String {
        let _command_encoder = self.0.table().get(&command_encoder).unwrap();
        // TODO: return real label
        String::new()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::CommandEncoderId>, _label: String) {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        group_label: String,
    ) {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        self.instance()
            .command_encoder_push_debug_group::<crate::Backend>(command_encoder, &group_label)
            .unwrap();
    }

    fn pop_debug_group(&mut self, command_encoder: Resource<wgpu_core::id::CommandEncoderId>) {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        self.instance()
            .command_encoder_pop_debug_group::<crate::Backend>(command_encoder)
            .unwrap();
    }

    fn insert_debug_marker(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        marker_label: String,
    ) {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        self.instance()
            .command_encoder_insert_debug_marker::<crate::Backend>(command_encoder, &marker_label)
            .unwrap();
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuRenderPassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) {
        let instance = self.0.instance();
        let pipeline = pipeline.to_core(&self.0.table());
        let render_pass = self.0.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_set_pipeline(render_pass, pipeline)
            .unwrap()
    }

    fn draw(
        &mut self,
        rpass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        vertex_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_vertex: Option<webgpu::GpuSize32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.0.instance();
        let rpass = self.0.table().get_mut(&rpass).unwrap();
        instance
            .render_pass_draw(
                rpass,
                vertex_count,
                instance_count.unwrap_or(1),
                first_vertex.unwrap_or(0),
                first_instance.unwrap_or(0),
            )
            .unwrap()
    }

    fn end(&mut self, rpass: Resource<wgpu_core::command::RenderPass<crate::Backend>>) {
        let instance = self.0.instance();
        let mut rpass = self.0.table().get_mut(&rpass).unwrap();
        instance
            .render_pass_end::<crate::Backend>(&mut rpass)
            .unwrap();
    }

    fn drop(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
    ) -> wasmtime::Result<()> {
        self.0.table().delete(render_pass).unwrap();
        Ok(())
    }

    fn set_viewport(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        let instance = self.0.instance();
        let render_pass = self.0.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_set_viewport(render_pass, x, y, width, height, min_depth, max_depth)
            .unwrap();
    }

    fn set_scissor_rect(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        x: webgpu::GpuIntegerCoordinate,
        y: webgpu::GpuIntegerCoordinate,
        width: webgpu::GpuIntegerCoordinate,
        height: webgpu::GpuIntegerCoordinate,
    ) {
        let instance = self.0.instance();
        let render_pass = self.0.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_set_scissor_rect(render_pass, x, y, width, height)
            .unwrap();
    }

    fn set_blend_constant(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _color: webgpu::GpuColor,
    ) {
        todo!()
    }

    fn set_stencil_reference(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _reference: webgpu::GpuStencilValue,
    ) {
        todo!()
    }

    fn begin_occlusion_query(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _query_index: webgpu::GpuSize32,
    ) {
        todo!()
    }

    fn end_occlusion_query(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
    ) {
        todo!()
    }

    fn execute_bundles(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _bundles: Vec<Resource<webgpu::GpuRenderBundle>>,
    ) {
        todo!()
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
    ) -> String {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _label: String,
    ) {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _group_label: String,
    ) {
        todo!()
    }

    fn pop_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
    ) {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _marker_label: String,
    ) {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) {
        let instance = self.0.instance();
        let bind_group = *self
            .0
            .table()
            .get(&bind_group.expect("TODO: deal with null bind_groups"))
            .unwrap();
        let mut render_pass = self.0.table().get_mut(&render_pass).unwrap();
        let dynamic_offsets = dynamic_offsets.unwrap();
        instance
            .render_pass_set_bind_group(&mut render_pass, index, bind_group, &dynamic_offsets)
            .unwrap()
    }

    fn set_index_buffer(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        buffer: Resource<webgpu::GpuBuffer>,
        index_format: webgpu::GpuIndexFormat,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let instance = self.0.instance();
        let (buffer_id, buffer_size) = {
            let buffer = self.table().get(&buffer).unwrap();
            (buffer.buffer, buffer.size)
        };
        let render_pass = self.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_set_index_buffer(
                render_pass,
                buffer_id,
                index_format.into(),
                offset.unwrap_or(0),
                core::num::NonZeroU64::new(size.unwrap_or(buffer_size)),
            )
            .unwrap()
    }

    fn set_vertex_buffer(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        slot: webgpu::GpuIndex32,
        buffer: Option<Resource<webgpu::GpuBuffer>>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let instance = self.0.instance();
        let (buffer_id, buffer_size) = {
            let buffer = self
                .table()
                .get(&buffer.expect("TODO: deal null buffers"))
                .unwrap();
            (buffer.buffer, buffer.size)
        };
        let mut render_pass = self.0.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_set_vertex_buffer(
                &mut render_pass,
                slot,
                buffer_id,
                offset.unwrap_or(0),
                core::num::NonZeroU64::new(size.unwrap_or(buffer_size)),
            )
            .unwrap()
    }

    fn draw_indexed(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        index_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_index: Option<webgpu::GpuSize32>,
        base_vertex: Option<webgpu::GpuSignedOffset32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.0.instance();
        let render_pass = self.table().get_mut(&render_pass).unwrap();
        instance
            .render_pass_draw_indexed(
                render_pass,
                index_count,
                instance_count.unwrap_or(1),
                first_index.unwrap_or(0),
                base_vertex.unwrap_or(0),
                first_instance.unwrap_or(0),
            )
            .unwrap()
    }

    fn draw_indirect(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn draw_indexed_indirect(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass<crate::Backend>>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuUncapturedErrorEvent for WasiWebGpuImpl<T> {
    fn new(
        &mut self,
        _type_: String,
        _gpu_uncaptured_error_event_init_dict: webgpu::GpuUncapturedErrorEventInit,
    ) -> Resource<webgpu::GpuUncapturedErrorEvent> {
        todo!()
    }

    fn error(
        &mut self,
        _self_: Resource<webgpu::GpuUncapturedErrorEvent>,
    ) -> Resource<webgpu::GpuError> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuUncapturedErrorEvent>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuInternalError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuInternalError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuInternalError>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuInternalError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuOutOfMemoryError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuOutOfMemoryError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuOutOfMemoryError>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuOutOfMemoryError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuValidationError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuValidationError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuValidationError>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuValidationError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuError for WasiWebGpuImpl<T> {
    fn message(&mut self, _self_: Resource<webgpu::GpuError>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuDeviceLostInfo for WasiWebGpuImpl<T> {
    fn reason(
        &mut self,
        _self_: Resource<webgpu::GpuDeviceLostInfo>,
    ) -> webgpu::GpuDeviceLostReason {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuDeviceLostInfo>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCanvasContext for WasiWebGpuImpl<T> {
    fn configure(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
        _configuration: webgpu::GpuCanvasConfiguration,
    ) {
        todo!()
    }

    fn unconfigure(&mut self, _self_: Resource<webgpu::GpuCanvasContext>) {
        todo!()
    }

    fn get_current_texture(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
    ) -> Resource<webgpu::GpuTexture> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCanvasContext>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuRenderBundle for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuRenderBundle>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuRenderBundle>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderBundle>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuComputePassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        pipeline: Resource<webgpu::GpuComputePipeline>,
    ) {
        let instance = self.0.instance();
        let pipeline = *self.0.table().get(&pipeline).unwrap();
        let encoder = self.0.table().get_mut(&encoder).unwrap();
        instance
            .compute_pass_set_pipeline(encoder, pipeline)
            .unwrap();
    }

    fn dispatch_workgroups(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        workgroup_count_x: webgpu::GpuSize32,
        workgroup_count_y: Option<webgpu::GpuSize32>,
        workgroup_count_z: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.0.instance();
        let encoder = self.0.table().get_mut(&encoder).unwrap();
        instance
            .compute_pass_dispatch_workgroups(
                encoder,
                workgroup_count_x,
                workgroup_count_y.unwrap(),
                workgroup_count_z.unwrap(),
            )
            .unwrap()
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn end(&mut self, cpass: Resource<wgpu_core::command::ComputePass<crate::Backend>>) {
        let instance = self.0.instance();
        let mut cpass = self.0.table().get_mut(&cpass).unwrap();
        instance
            .compute_pass_end::<crate::Backend>(&mut cpass)
            .unwrap();
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuComputePassEncoder>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuComputePassEncoder>, _label: String) {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _group_label: String,
    ) {
        todo!()
    }

    fn pop_debug_group(&mut self, _self_: Resource<webgpu::GpuComputePassEncoder>) {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        cpass: Resource<webgpu::GpuComputePassEncoder>,
        label: String,
    ) {
        let instance = self.0.instance();
        let cpass = self.0.table().get_mut(&cpass).unwrap();
        instance
            .compute_pass_insert_debug_marker(cpass, &label, 0)
            .unwrap()
    }

    fn set_bind_group(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) {
        let instance = self.0.instance();
        let bind_group = *self
            .0
            .table()
            .get(&bind_group.expect("TODO: deal with null bind_groups"))
            .unwrap();
        let encoder = self.0.table().get_mut(&encoder).unwrap();
        let dynamic_offsets = dynamic_offsets.unwrap();
        instance
            .compute_pass_set_bind_group(encoder, index, bind_group, &dynamic_offsets)
            .unwrap()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePassEncoder>) -> wasmtime::Result<()> {
        self.0.table().delete(_rep).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuPipelineError for WasiWebGpuImpl<T> {
    fn new(
        &mut self,
        _message: Option<String>,
        _options: webgpu::GpuPipelineErrorInit,
    ) -> Resource<webgpu::GpuPipelineError> {
        todo!()
    }

    fn reason(
        &mut self,
        _self_: Resource<webgpu::GpuPipelineError>,
    ) -> webgpu::GpuPipelineErrorReason {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuPipelineError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCompilationMessage for WasiWebGpuImpl<T> {
    fn message(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> String {
        todo!()
    }

    fn type_(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationMessage>,
    ) -> webgpu::GpuCompilationMessageType {
        todo!()
    }

    fn line_num(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> u64 {
        todo!()
    }

    fn line_pos(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> u64 {
        todo!()
    }

    fn offset(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> u64 {
        todo!()
    }

    fn length(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> u64 {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCompilationInfo for WasiWebGpuImpl<T> {
    fn messages(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationInfo>,
    ) -> Vec<Resource<webgpu::GpuCompilationMessage>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCompilationInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuQuerySet for WasiWebGpuImpl<T> {
    fn destroy(&mut self, _self_: Resource<webgpu::GpuQuerySet>) {
        todo!()
    }

    fn type_(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> webgpu::GpuQueryType {
        todo!()
    }

    fn count(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> webgpu::GpuSize32Out {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuQuerySet>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuRenderBundleEncoder for WasiWebGpuImpl<T> {
    fn finish(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _descriptor: Option<webgpu::GpuRenderBundleDescriptor>,
    ) -> Resource<webgpu::GpuRenderBundle> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuRenderBundleEncoder>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuRenderBundleEncoder>, _label: String) {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _group_label: String,
    ) {
        todo!()
    }

    fn pop_debug_group(&mut self, _self_: Resource<webgpu::GpuRenderBundleEncoder>) {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _marker_label: String,
    ) {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _index: webgpu::GpuIndex32,
        _bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        _dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) {
        todo!()
    }

    fn set_pipeline(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _pipeline: Resource<wgpu_core::id::RenderPipelineId>,
    ) {
        todo!()
    }

    fn set_index_buffer(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _index_format: webgpu::GpuIndexFormat,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) {
        todo!()
    }

    fn set_vertex_buffer(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _slot: webgpu::GpuIndex32,
        _buffer: Option<Resource<webgpu::GpuBuffer>>,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) {
        todo!()
    }

    fn draw(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _vertex_count: webgpu::GpuSize32,
        _instance_count: Option<webgpu::GpuSize32>,
        _first_vertex: Option<webgpu::GpuSize32>,
        _first_instance: Option<webgpu::GpuSize32>,
    ) {
        todo!()
    }

    fn draw_indexed(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _index_count: webgpu::GpuSize32,
        _instance_count: Option<webgpu::GpuSize32>,
        _first_index: Option<webgpu::GpuSize32>,
        _base_vertex: Option<webgpu::GpuSignedOffset32>,
        _first_instance: Option<webgpu::GpuSize32>,
    ) {
        todo!()
    }

    fn draw_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn draw_indexed_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderBundleEncoder>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuComputePipeline for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuComputePipeline>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuComputePipeline>, _label: String) {
        todo!()
    }

    fn get_bind_group_layout(
        &mut self,
        compute_pipeline: Resource<webgpu::GpuComputePipeline>,
        index: u32,
    ) -> Resource<webgpu::GpuBindGroupLayout> {
        let pipeline_id = *self.0.table().get(&compute_pipeline).unwrap();
        let bind_group_layout = core_result(
            self.0
                .instance()
                .compute_pipeline_get_bind_group_layout::<crate::Backend>(pipeline_id, index, None),
        )
        .unwrap();
        self.0.table().push(bind_group_layout).unwrap()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBindGroup for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuBindGroup>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuBindGroup>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBindGroup>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuPipelineLayout for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuPipelineLayout>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuPipelineLayout>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuPipelineLayout>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBindGroupLayout for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuBindGroupLayout>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuBindGroupLayout>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBindGroupLayout>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuSampler for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuSampler>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuSampler>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSampler>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: WasiWebGpuView> webgpu::HostGpuBuffer for WasiWebGpuImpl<T> {
    fn size(&mut self, buffer: Resource<webgpu::GpuBuffer>) -> webgpu::GpuSize64Out {
        let buffer = self.table().get(&buffer).unwrap();
        buffer.size
    }

    fn usage(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> webgpu::GpuFlagsConstant {
        todo!()
    }

    fn map_state(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> webgpu::GpuBufferMapState {
        todo!()
    }

    async fn map_async(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        mode: webgpu::GpuMapModeFlags,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let buffer = self.0.table().get(&buffer).unwrap().buffer;
        let instance = self.0.instance();
        CallbackFuture::new(Box::new(
            move |resolve: Box<
                dyn FnOnce(Box<Result<(), wgpu_core::resource::BufferAccessError>>) + Send,
            >| {
                // TODO: move to convertion function
                // source: https://www.w3.org/TR/webgpu/#typedefdef-gpumapmodeflags
                let host = match mode {
                    1 => wgpu_core::device::HostMap::Read,
                    2 => wgpu_core::device::HostMap::Write,
                    _ => panic!(),
                };
                let op = wgpu_core::resource::BufferMapOperation {
                    host,
                    callback: Some(wgpu_core::resource::BufferMapCallback::from_rust(Box::new(
                        move |result| {
                            resolve(Box::new(result));
                        },
                    ))),
                };

                let offset = offset.unwrap();
                instance
                    .buffer_map_async::<crate::Backend>(buffer, offset, size, op)
                    .unwrap();
                // TODO: only poll this device.
                instance.poll_all_devices(true).unwrap();
            },
        ))
        .await
        .unwrap();
    }

    fn get_mapped_range(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> Resource<webgpu::NonStandardBuffer> {
        let buffer_id = self.0.table().get(&buffer).unwrap().buffer;
        let (ptr, len) = self
            .0
            .instance()
            .buffer_get_mapped_range::<crate::Backend>(buffer_id, offset.unwrap_or(0), size)
            .unwrap();
        let remote_buffer = BufferPtr { ptr, len };
        self.0.table().push(remote_buffer).unwrap()
    }

    fn unmap(&mut self, buffer: Resource<webgpu::GpuBuffer>) {
        let buffer = self.0.table().get_mut(&buffer).unwrap();
        let buffer_id = buffer.buffer;
        self.0
            .instance()
            .buffer_unmap::<crate::Backend>(buffer_id)
            .unwrap();
    }

    fn destroy(&mut self, _self_: Resource<webgpu::GpuBuffer>) {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuBuffer>, _label: String) {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpu for WasiWebGpuImpl<T> {
    fn request_adapter(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
        _options: Option<webgpu::GpuRequestAdapterOptions>,
    ) -> Option<Resource<wgpu_core::id::AdapterId>> {
        let adapter = self.0.instance().request_adapter(
            &Default::default(),
            wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| None),
        );
        if let Err(wgpu_core::instance::RequestAdapterError::NotFound) = adapter {
            return None;
        }
        let adapter = adapter.unwrap();
        Some(self.0.table().push(adapter).unwrap())
    }

    fn get_preferred_canvas_format(
        &mut self,
        _gpu: Resource<webgpu::Gpu>,
    ) -> webgpu::GpuTextureFormat {
        // https://searchfox.org/mozilla-central/source/dom/webgpu/Instance.h#42
        #[cfg(target_os = "android")]
        return webgpu::GpuTextureFormat::Rgba8unorm;
        #[cfg(not(target_os = "android"))]
        return webgpu::GpuTextureFormat::Bgra8unorm;
    }

    fn wgsl_language_features(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
    ) -> Resource<webgpu::WgslLanguageFeatures> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::Gpu>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuAdapterInfo for WasiWebGpuImpl<T> {
    fn vendor(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> String {
        todo!()
    }

    fn architecture(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> String {
        todo!()
    }

    fn device(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> String {
        todo!()
    }

    fn description(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> String {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostWgslLanguageFeatures for WasiWebGpuImpl<T> {
    fn has(&mut self, _self_: Resource<webgpu::WgslLanguageFeatures>, _key: String) -> bool {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::WgslLanguageFeatures>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedFeatures for WasiWebGpuImpl<T> {
    fn has(&mut self, features: Resource<webgpu::GpuSupportedFeatures>, query: String) -> bool {
        let features = self.0.table().get(&features).unwrap();
        match query.as_str() {
            "depth-clip-control" => features.contains(wgpu_types::Features::DEPTH_CLIP_CONTROL),
            "timestamp-query" => features.contains(wgpu_types::Features::TIMESTAMP_QUERY),
            "indirect-first-instance" => {
                features.contains(wgpu_types::Features::INDIRECT_FIRST_INSTANCE)
            }
            "shader-f16" => features.contains(wgpu_types::Features::SHADER_F16),
            "depth32float-stencil8" => {
                features.contains(wgpu_types::Features::DEPTH32FLOAT_STENCIL8)
            }
            "texture-compression-bc" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_BC)
            }
            "texture-compression-etc2" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_ETC2)
            }
            "texture-compression-astc" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_ASTC)
            }
            "rg11b10ufloat-renderable" => {
                features.contains(wgpu_types::Features::RG11B10UFLOAT_RENDERABLE)
            }
            "bgra8unorm-storage" => features.contains(wgpu_types::Features::BGRA8UNORM_STORAGE),
            "float32-filterable" => features.contains(wgpu_types::Features::FLOAT32_FILTERABLE),
            _ => todo!(),
        }
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedFeatures>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedLimits for WasiWebGpuImpl<T> {
    fn max_texture_dimension1_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_texture_dimension_1d
    }

    fn max_texture_dimension2_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_texture_dimension_2d
    }

    fn max_texture_dimension3_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_texture_dimension_3d
    }

    fn max_texture_array_layers(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_texture_array_layers
    }

    fn max_bind_groups(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_bind_groups
    }

    fn max_bind_groups_plus_vertex_buffers(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        todo!()
    }

    fn max_bindings_per_bind_group(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_bindings_per_bind_group
    }

    fn max_dynamic_uniform_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_dynamic_uniform_buffers_per_pipeline_layout
    }

    fn max_dynamic_storage_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_dynamic_storage_buffers_per_pipeline_layout
    }

    fn max_sampled_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_sampled_textures_per_shader_stage
    }

    fn max_samplers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_samplers_per_shader_stage
    }

    fn max_storage_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_storage_buffers_per_shader_stage
    }

    fn max_storage_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_storage_textures_per_shader_stage
    }

    fn max_uniform_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_uniform_buffers_per_shader_stage
    }

    fn max_uniform_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u64 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_uniform_buffer_binding_size as u64
    }

    fn max_storage_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u64 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_storage_buffer_binding_size as u64
    }

    fn min_uniform_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.min_uniform_buffer_offset_alignment
    }

    fn min_storage_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.min_storage_buffer_offset_alignment
    }

    fn max_vertex_buffers(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_vertex_buffers
    }

    fn max_buffer_size(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u64 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_buffer_size
    }

    fn max_vertex_attributes(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_vertex_attributes
    }

    fn max_vertex_buffer_array_stride(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_vertex_buffer_array_stride
    }

    fn max_inter_stage_shader_variables(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        todo!()
    }

    fn max_color_attachments(&mut self, _limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        todo!()
    }

    fn max_color_attachment_bytes_per_sample(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        todo!()
    }

    fn max_compute_workgroup_storage_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_workgroup_storage_size
    }

    fn max_compute_invocations_per_workgroup(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_invocations_per_workgroup
    }

    fn max_compute_workgroup_size_x(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_x
    }

    fn max_compute_workgroup_size_y(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_y
    }

    fn max_compute_workgroup_size_z(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_z
    }

    fn max_compute_workgroups_per_dimension(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.0.table().get(&limits).unwrap();
        limits.max_compute_workgroups_per_dimension
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedLimits>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

fn core_result<I, E>(
    (id, error): (wgpu_core::id::Id<I>, Option<E>),
) -> Result<wgpu_core::id::Id<I>, E>
where
    I: wgpu_core::id::Marker,
{
    match error {
        Some(error) => Err(error),
        None => Ok(id),
    }
}

// same as core_result, but but result doesn't need to be id.
fn core_result_t<T, E>((t, error): (T, Option<E>)) -> Result<T, E> {
    match error {
        Some(error) => Err(error),
        None => Ok(t),
    }
}

// same as core_result, but handles tuple of two ids for Ok.
fn core_results_2<I1, I2, E>(
    (a, b, error): (wgpu_core::id::Id<I1>, wgpu_core::id::Id<I2>, Option<E>),
) -> Result<(wgpu_core::id::Id<I1>, wgpu_core::id::Id<I2>), E>
where
    I1: wgpu_core::id::Marker,
    I2: wgpu_core::id::Marker,
{
    match error {
        Some(error) => Err(error),
        None => Ok((a, b)),
    }
}
