// TODO: in this file:
// - Remove all calls to `Default::default()`. Instead, manually set them, and link to the spec stating what defaults should be used.
// - Implement all todos.
// - Remove all unwraps.
// - Implement all the drop handlers.

use callback_future::CallbackFuture;
use core::slice;
use std::borrow::Cow;
use std::sync::Arc;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::WasiView;

use crate::wasi::webgpu::webgpu;
use wasi_graphics_context_wasmtime::{DisplayApi, DrawApi, GraphicsContext, GraphicsContextBuffer};

use self::to_core_conversions::ToCore;

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

pub use wasi::webgpu::webgpu::add_to_linker;

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
        "wasi:webgpu/webgpu/gpu-device": Device,
        // queue is same as device
        "wasi:webgpu/webgpu/gpu-queue": Device,
        "wasi:webgpu/webgpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "wasi:webgpu/webgpu/gpu-render-pass-encoder": wgpu_core::command::RenderPass,
        "wasi:webgpu/webgpu/gpu-compute-pass-encoder": wgpu_core::command::ComputePass,
        "wasi:webgpu/webgpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "wasi:webgpu/webgpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "wasi:webgpu/webgpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        // "wasi:webgpu/webgpu/gpu-buffer": wgpu_core::id::BufferId,
        "wasi:webgpu/webgpu/gpu-buffer": Buffer,
        "wasi:webgpu/webgpu/remote-buffer": Buffer,
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

pub trait HasGpuInstance {
    fn instance(
        &self,
    ) -> Arc<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>;
}

pub struct WebGpuSurface<F, I>
where
    I: AsRef<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>,
    F: Fn() -> I,
{
    get_instance: F,
    device_id: wgpu_core::id::DeviceId,
    adapter_id: wgpu_core::id::AdapterId,
    surface_id: Option<wgpu_core::id::SurfaceId>,
}

impl<F, I> DrawApi for WebGpuSurface<F, I>
where
    I: AsRef<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>,
    F: Fn() -> I,
{
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer> {
        let texture: wgpu_core::id::TextureId = (self.get_instance)()
            .as_ref()
            .surface_get_current_texture::<crate::Backend>(self.surface_id.unwrap(), ())
            .unwrap()
            .texture_id
            .unwrap();
        let buff = Box::new(texture);
        let buff: GraphicsContextBuffer = buff.into();
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
        // self.insert(value)
        let surface_id = (self.get_instance)().as_ref().instance_create_surface(
            display.raw_display_handle(),
            display.raw_window_handle(),
            (),
        );

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
    pub(crate) ptr: *mut u8,
    pub(crate) len: u64,
}
impl BufferPtr {
    pub fn slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len as usize) }
    }
    pub fn slice_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len as usize) }
    }
}
unsafe impl Send for BufferPtr {}
unsafe impl Sync for BufferPtr {}

pub struct Buffer {
    buffer: wgpu_core::id::BufferId,
    mapped: Option<BufferPtr>,
}

#[derive(Clone, Copy)]
pub struct Device {
    pub device: wgpu_core::id::DeviceId,
    // only needed when calling surface.get_capabilities in connect_graphics_context. If table would have a way to get parent from child, we could get it from device.
    pub adapter: wgpu_core::id::AdapterId,
}

impl<T: WasiView + HasGpuInstance> webgpu::Host for T {
    fn get_gpu(&mut self) -> wasmtime::Result<Resource<webgpu::Gpu>> {
        Ok(Resource::new_own(0))
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostRemoteBuffer for T {
    fn length(&mut self, buffer: Resource<webgpu::RemoteBuffer>) -> wasmtime::Result<u32> {
        let buffer = self.table().get(&buffer).unwrap();
        let len = buffer.mapped.as_ref().unwrap().len;
        Ok(len as u32)
    }

    fn get(&mut self, buffer: Resource<webgpu::RemoteBuffer>, i: u32) -> wasmtime::Result<u8> {
        let buffer = self.table().get(&buffer).unwrap();
        let remote_buffer = buffer.mapped.as_ref().unwrap();
        let val = remote_buffer.slice()[i as usize];
        Ok(val)
    }

    fn set(
        &mut self,
        buffer: Resource<webgpu::RemoteBuffer>,
        i: u32,
        val: u8,
    ) -> wasmtime::Result<()> {
        let buffer = self.table_mut().get_mut(&buffer).unwrap();
        let remote_buffer = buffer.mapped.as_mut().unwrap();
        remote_buffer.slice_mut()[i as usize] = val;
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<webgpu::RemoteBuffer>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuDevice for T {
    fn connect_graphics_context(
        &mut self,
        device: Resource<Device>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let device = self.table().get(&device).unwrap();
        let device_id = device.device;
        let adapter_id = device.adapter;

        let instance = Arc::downgrade(&self.instance());

        let context = self.table_mut().get_mut(&context).unwrap();

        let surface = WebGpuSurface {
            get_instance: move || instance.upgrade().unwrap(),
            device_id,
            adapter_id,
            surface_id: None,
        };

        context.connect_draw_api(Box::new(surface));

        Ok(())
    }

    fn create_command_encoder(
        &mut self,
        device: Resource<Device>,
        descriptor: Option<webgpu::GpuCommandEncoderDescriptor>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::CommandEncoderId>> {
        let host_daq = self.table().get(&device).unwrap();

        let command_encoder = core_result(
            self.instance()
                .device_create_command_encoder::<crate::Backend>(
                    host_daq.device,
                    &descriptor
                        .map(|d| d.to_core(&self.table()))
                        .unwrap_or_default(),
                    (),
                ),
        )
        .unwrap();

        Ok(self
            .table_mut()
            .push_child(command_encoder, &device)
            .unwrap())
    }

    fn create_shader_module(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuShaderModuleDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuShaderModule>> {
        let device = self.table().get(&device).unwrap();

        let code =
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Owned(descriptor.code.to_owned()));
        let shader = core_result(
            self.instance()
                .device_create_shader_module::<crate::Backend>(
                    device.device,
                    &descriptor.to_core(&self.table()),
                    code,
                    (),
                ),
        )
        .unwrap();

        Ok(self.table_mut().push(shader).unwrap())
    }

    fn create_render_pipeline(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Resource<wgpu_core::id::RenderPipelineId>> {
        let host_device = self.table().get(&device).unwrap();

        let descriptor = descriptor.to_core(&self.table());

        let implicit_pipeline_ids = match descriptor.layout {
            Some(_) => None,
            None => Some(wgpu_core::device::ImplicitPipelineIds {
                root_id: (),
                group_ids: &[(); wgpu_core::MAX_BIND_GROUPS],
            }),
        };
        let render_pipeline = core_result(
            self.instance()
                .device_create_render_pipeline::<crate::Backend>(
                    host_device.device,
                    &descriptor,
                    (),
                    implicit_pipeline_ids,
                ),
        )
        .unwrap();

        Ok(self
            .table_mut()
            .push_child(render_pipeline, &device)
            .unwrap())
    }

    fn queue(&mut self, device: Resource<Device>) -> wasmtime::Result<Resource<Device>> {
        Ok(Resource::new_own(device.rep()))
    }

    fn features(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedFeatures>> {
        let device = self.table().get(&device).unwrap();
        let features = self
            .instance()
            .device_features::<crate::Backend>(device.device)
            .unwrap();
        Ok(self.table_mut().push(features).unwrap())
    }

    fn limits(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedLimits>> {
        let device = self.table().get(&device).unwrap();
        let limits = self
            .instance()
            .device_limits::<crate::Backend>(device.device)
            .unwrap();
        Ok(self.table_mut().push(limits).unwrap())
    }

    fn destroy(&mut self, _device: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        todo!()
    }

    fn create_buffer(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBufferDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBuffer>> {
        let device = self.table().get(&device).unwrap();

        let buffer = core_result(self.instance().device_create_buffer::<crate::Backend>(
            device.device,
            &descriptor.to_core(&self.table()),
            (),
        ))
        .unwrap();

        let buffer = Buffer {
            buffer,
            mapped: None,
        };

        Ok(self.table_mut().push(buffer).unwrap())
    }

    fn create_texture(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuTextureDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuTexture>> {
        let device = *self.table().get(&device).unwrap();
        let texture = core_result(self.instance().device_create_texture::<crate::Backend>(
            device.device,
            &descriptor.to_core(&self.table()),
            (),
        ))
        .unwrap();

        Ok(self.table_mut().push(texture).unwrap())
    }

    fn create_sampler(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: Option<webgpu::GpuSamplerDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSampler>> {
        let device = self.table().get(&device).unwrap();

        let descriptor = descriptor.unwrap();

        let sampler = core_result(self.instance().device_create_sampler::<crate::Backend>(
            device.device,
            &descriptor.to_core(&self.table()),
            (),
        ))
        .unwrap();

        Ok(self.table_mut().push(sampler).unwrap())
    }

    fn import_external_texture(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuExternalTextureDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuExternalTexture>> {
        todo!()
    }

    fn create_bind_group_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        let device = self.table().get(&device).unwrap();

        let bind_group_layout = core_result(
            self.instance()
                .device_create_bind_group_layout::<crate::Backend>(
                    device.device,
                    &descriptor.to_core(&self.table()),
                    (),
                ),
        )
        .unwrap();

        Ok(self.table_mut().push(bind_group_layout).unwrap())
    }

    fn create_pipeline_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuPipelineLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuPipelineLayout>> {
        let device = *self.table().get(&device).unwrap();

        let pipeline_layout = core_result(
            self.instance()
                .device_create_pipeline_layout::<crate::Backend>(
                    device.device,
                    &descriptor.to_core(&self.table()),
                    (),
                ),
        )
        .unwrap();

        Ok(self.table_mut().push(pipeline_layout).unwrap())
    }

    fn create_bind_group(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroup>> {
        let device = *self.table().get(&device).unwrap();

        let bind_group = core_result(self.instance().device_create_bind_group::<crate::Backend>(
            device.device,
            &descriptor.to_core(&self.table()),
            (),
        ))
        .unwrap();

        Ok(self.table_mut().push(bind_group).unwrap())
    }

    fn create_compute_pipeline(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePipeline>> {
        let device = self.table().get(&device).unwrap();

        let implicit_pipeline_ids = match &descriptor.layout {
            webgpu::GpuPipelineLayoutOrGpuAutoLayoutMode::GpuPipelineLayout(_) => None,
            webgpu::GpuPipelineLayoutOrGpuAutoLayoutMode::GpuAutoLayoutMode(mode) => match mode {
                webgpu::GpuAutoLayoutMode::Auto => Some(wgpu_core::device::ImplicitPipelineIds {
                    root_id: (),
                    group_ids: &[(); wgpu_core::MAX_BIND_GROUPS],
                }),
            },
        };

        let compute_pipeline = core_result(
            self.instance()
                .device_create_compute_pipeline::<crate::Backend>(
                    device.device,
                    &descriptor.to_core(&self.table()),
                    (),
                    implicit_pipeline_ids,
                ),
        )
        .unwrap();
        Ok(self.table_mut().push(compute_pipeline).unwrap())
    }

    // fn create_compute_pipeline_async(
    //     &mut self,
    //     self_: Resource<webgpu::GpuDevice>,
    //     descriptor: webgpu::GpuComputePipelineDescriptor,
    // ) -> wasmtime::Result<Resource<webgpu::GpuComputePipeline>> {
    //     todo!()
    // }

    // fn create_render_pipeline_async(
    //     &mut self,
    //     self_: Resource<webgpu::GpuDevice>,
    //     descriptor: webgpu::GpuRenderPipelineDescriptor,
    // ) -> wasmtime::Result<Resource<wgpu_core::id::RenderPipelineId>> {
    //     todo!()
    // }

    fn create_render_bundle_encoder(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuRenderBundleEncoderDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderBundleEncoder>> {
        todo!()
    }

    fn create_query_set(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuQuerySetDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuQuerySet>> {
        todo!()
    }

    fn label(&mut self, _device: Resource<webgpu::GpuDevice>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn lost(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDeviceLostInfo>> {
        todo!()
    }

    fn push_error_scope(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
        _filter: webgpu::GpuErrorFilter,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_error_scope(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuError>> {
        todo!()
    }

    fn onuncapturederror(
        &mut self,
        _device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::EventHandler>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuTexture for T {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<GraphicsContextBuffer>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::TextureId>> {
        let host_buffer = self.table_mut().delete(buffer).unwrap();
        let host_buffer: wgpu_core::id::TextureId = host_buffer.inner_type();
        Ok(self.table_mut().push(host_buffer).unwrap())
    }

    fn create_view(
        &mut self,
        texture: Resource<wgpu_core::id::TextureId>,
        descriptor: Option<webgpu::GpuTextureViewDescriptor>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::TextureViewId>> {
        let texture_id = *self.table().get(&texture).unwrap();
        let texture_view = core_result(
            self.instance().texture_create_view::<crate::Backend>(
                texture_id,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or_default(),
                (),
            ),
        )
        .unwrap();
        Ok(self.table_mut().push(texture_view).unwrap())
    }

    fn drop(&mut self, _rep: Resource<wgpu_core::id::TextureId>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }

    fn destroy(&mut self, _self_: Resource<webgpu::GpuTexture>) -> wasmtime::Result<()> {
        todo!()
    }

    fn width(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuIntegerCoordinateOut> {
        todo!()
    }

    fn height(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuIntegerCoordinateOut> {
        todo!()
    }

    fn depth_or_array_layers(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuIntegerCoordinateOut> {
        todo!()
    }

    fn mip_level_count(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuIntegerCoordinateOut> {
        todo!()
    }

    fn sample_count(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuSize32Out> {
        todo!()
    }

    fn dimension(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuTextureDimension> {
        todo!()
    }

    fn format(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuTextureFormat> {
        todo!()
    }

    fn usage(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
    ) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuTexture>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuTexture>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuTextureView for T {
    fn drop(&mut self, _rep: Resource<wgpu_core::id::TextureViewId>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::id::TextureViewId>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::TextureViewId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuCommandBuffer for T {
    fn drop(&mut self, _rep: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        // self.web_gpu_host.command_buffers.remove(&rep.rep());
        Ok(())
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandBufferId>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandBufferId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuShaderModule for T {
    fn drop(&mut self, _rep: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        // self.web_gpu_host.shaders.remove(&rep.rep());
        Ok(())
    }

    fn get_compilation_info(
        &mut self,
        _self_: Resource<wgpu_core::id::ShaderModuleId>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCompilationInfo>> {
        todo!()
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::id::ShaderModuleId>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::ShaderModuleId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuRenderPipeline for T {
    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::id::RenderPipelineId>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::RenderPipelineId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_bind_group_layout(
        &mut self,
        _self_: Resource<wgpu_core::id::RenderPipelineId>,
        _index: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuAdapter for T {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
        descriptor: Option<webgpu::GpuDeviceDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDevice>> {
        let adapter_id = *self.table().get(&adapter).unwrap();

        let device_id = core_result(
            self.instance().adapter_request_device::<crate::Backend>(
                adapter_id,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or_default(),
                None,
                (),
            ),
        )
        .unwrap();

        let device = self
            .table_mut()
            .push_child(
                Device {
                    device: device_id,
                    adapter: adapter_id,
                },
                &adapter,
            )
            .unwrap();

        Ok(device)
    }

    fn drop(&mut self, _adapter: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn features(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<wasmtime::component::Resource<webgpu::GpuSupportedFeatures>> {
        todo!()
    }

    fn limits(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedLimits>> {
        let adapter = self.table().get(&adapter).unwrap();
        let limits = self
            .instance()
            .adapter_limits::<crate::Backend>(*adapter)
            .unwrap();
        Ok(self.table_mut().push(limits).unwrap())
    }

    fn is_fallback_adapter(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn request_adapter_info(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<Resource<webgpu::GpuAdapterInfo>> {
        let adapter_id = self.table().get(&adapter).unwrap();
        let info = self
            .instance()
            .adapter_get_info::<crate::Backend>(*adapter_id)
            .unwrap();
        let info = self.table_mut().push(info).unwrap();
        Ok(info)
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuQueue for T {
    fn submit(
        &mut self,
        daq: Resource<Device>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) -> wasmtime::Result<()> {
        let command_buffers = val
            .into_iter()
            .map(|buffer| self.table_mut().delete(buffer).unwrap())
            .collect::<Vec<_>>();

        let daq = self.table().get(&daq).unwrap();
        self.instance()
            .queue_submit::<crate::Backend>(daq.device, &command_buffers)
            .unwrap();

        Ok(())
    }

    fn drop(&mut self, _rep: Resource<Device>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
    }

    fn on_submitted_work_done(&mut self, _self_: Resource<Device>) -> wasmtime::Result<()> {
        todo!()
    }

    fn write_buffer(
        &mut self,
        queue: Resource<Device>,
        buffer: Resource<webgpu::GpuBuffer>,
        buffer_offset: webgpu::GpuSize64,
        data_offset: Option<webgpu::GpuSize64>,
        data: Vec<u8>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let queue = self.table().get(&queue).unwrap();
        let buffer = self.table().get(&buffer).unwrap();
        let mut data = &data[..];
        if let Some(data_offset) = data_offset {
            let data_offset = data_offset as usize;
            data = &data[data_offset..];
        }
        if let Some(size) = size {
            let size = size as usize;
            data = &data[..size];
        }
        self.instance()
            .queue_write_buffer::<crate::Backend>(queue.device, buffer.buffer, buffer_offset, &data)
            .unwrap();

        Ok(())
    }

    fn write_texture(
        &mut self,
        device: Resource<Device>,
        destination: webgpu::GpuImageCopyTexture,
        data: Vec<u8>,
        data_layout: webgpu::GpuImageDataLayout,
        size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        let device = self.table().get(&device).unwrap();
        self.instance()
            .queue_write_texture::<crate::Backend>(
                device.device,
                &destination.to_core(&self.table()),
                &data,
                &data_layout.to_core(&self.table()),
                &size.to_core(&self.table()),
            )
            .unwrap();
        Ok(())
    }

    fn copy_external_image_to_texture(
        &mut self,
        _self_: Resource<Device>,
        _source: webgpu::GpuImageCopyExternalImage,
        _destination: webgpu::GpuImageCopyTextureTagged,
        _copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<Device>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<Device>, _label: String) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuCommandEncoder for T {
    fn begin_render_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: webgpu::GpuRenderPassDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPassEncoder>> {
        // can't use to_core because depth_stencil_attachment is Option<&x>.
        let depth_stencil_attachment = descriptor
            .depth_stencil_attachment
            .map(|d| d.to_core(&self.table()));
        let descriptor = wgpu_core::command::RenderPassDescriptor {
            label: descriptor.label.map(|l| l.into()),
            color_attachments: descriptor
                .color_attachments
                .into_iter()
                .map(|c| Some(c.to_core(&self.table())))
                .collect::<Vec<_>>()
                .into(),
            depth_stencil_attachment: depth_stencil_attachment.as_ref(),
            // timestamp_writes: self.timestamp_writes,
            // occlusion_query_set: self.occlusion_query_set,
            // TODO: self.max_draw_count not used
            // TODO: remove default
            ..Default::default()
        };
        let render_pass = wgpu_core::command::RenderPass::new(
            command_encoder.to_core(&self.table()),
            &descriptor,
        );

        Ok(self.table_mut().push(render_pass).unwrap())
    }

    fn finish(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuCommandBufferDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        let command_encoder = self.table_mut().delete(command_encoder).unwrap();
        let command_buffer = core_result(
            self.instance().command_encoder_finish::<crate::Backend>(
                command_encoder,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or_default(),
            ),
        )
        .unwrap();
        Ok(self.table_mut().push(command_buffer).unwrap())
    }

    fn drop(&mut self, _rep: Resource<wgpu_core::id::CommandEncoderId>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn begin_compute_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuComputePassDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePassEncoder>> {
        let command_encoder = self.table().get(&command_encoder).unwrap();
        let compute_pass = wgpu_core::command::ComputePass::new(
            *command_encoder,
            &wgpu_core::command::ComputePassDescriptor {
                label: Default::default(),
                timestamp_writes: descriptor
                    .map(|d| d.timestamp_writes.map(|tw| tw.to_core(&self.table())))
                    .flatten()
                    .as_ref(),
            },
        );
        Ok(self.table_mut().push(compute_pass).unwrap())
    }

    fn copy_buffer_to_buffer(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        source: Resource<webgpu::GpuBuffer>,
        source_offset: webgpu::GpuSize64,
        destination: Resource<webgpu::GpuBuffer>,
        destination_offset: webgpu::GpuSize64,
        size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        let source = self.table().get(&source).unwrap();
        let destination = self.table().get(&destination).unwrap();
        self.instance()
            .command_encoder_copy_buffer_to_buffer::<crate::Backend>(
                command_encoder,
                source.buffer,
                source_offset,
                destination.buffer,
                destination_offset,
                size,
            )
            .unwrap();
        Ok(())
    }

    fn copy_buffer_to_texture(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: webgpu::GpuImageCopyBuffer,
        _destination: webgpu::GpuImageCopyTexture,
        _copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn copy_texture_to_buffer(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: webgpu::GpuImageCopyTexture,
        _destination: webgpu::GpuImageCopyBuffer,
        _copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn copy_texture_to_texture(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: webgpu::GpuImageCopyTexture,
        _destination: webgpu::GpuImageCopyTexture,
        _copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn clear_buffer(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
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
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn label(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<String> {
        let _command_encoder = self.table().get(&command_encoder).unwrap();
        // TODO: return real label
        Ok(String::new())
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _group_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _marker_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuRenderPassEncoder for T {
    fn set_pipeline(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass>,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) -> wasmtime::Result<()> {
        let pipeline = pipeline.to_core(&self.table());
        let render_pass = self.table_mut().get_mut(&render_pass).unwrap();
        wgpu_core::command::render_ffi::wgpu_render_pass_set_pipeline(render_pass, pipeline);
        Ok(())
    }

    fn draw(
        &mut self,
        cwr: Resource<wgpu_core::command::RenderPass>,
        vertex_count: webgpu::GpuSize32,
        instance_count: webgpu::GpuSize32,
        first_vertex: webgpu::GpuSize32,
        first_instance: webgpu::GpuSize32,
    ) -> wasmtime::Result<()> {
        let cwr = self.table_mut().get_mut(&cwr).unwrap();

        wgpu_core::command::render_ffi::wgpu_render_pass_draw(
            cwr,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );

        Ok(())
    }

    fn end(
        &mut self,
        rpass: Resource<wgpu_core::command::RenderPass>,
        non_standard_encoder: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<()> {
        let rpass = self.table_mut().delete(rpass).unwrap();
        let encoder = self.table().get(&non_standard_encoder).unwrap();
        self.instance()
            .command_encoder_run_render_pass::<crate::Backend>(*encoder, &rpass)
            .unwrap();
        Ok(())
    }

    fn drop(&mut self, cwr: Resource<wgpu_core::command::RenderPass>) -> wasmtime::Result<()> {
        self.table_mut().delete(cwr).unwrap();
        Ok(())
    }

    fn set_viewport(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
        _min_depth: f32,
        _max_depth: f32,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_scissor_rect(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _x: webgpu::GpuIntegerCoordinate,
        _y: webgpu::GpuIntegerCoordinate,
        _width: webgpu::GpuIntegerCoordinate,
        _height: webgpu::GpuIntegerCoordinate,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_blend_constant(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _color: webgpu::GpuColor,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_stencil_reference(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _reference: webgpu::GpuStencilValue,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn begin_occlusion_query(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _query_index: webgpu::GpuSize32,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn end_occlusion_query(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn execute_bundles(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _bundles: Vec<Resource<webgpu::GpuRenderBundle>>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn label(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _group_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_debug_group(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _marker_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass>,
        index: webgpu::GpuIndex32,
        bind_group: Resource<webgpu::GpuBindGroup>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) -> wasmtime::Result<()> {
        let bind_group = *self.table().get(&bind_group).unwrap();
        let mut render_pass = self.table_mut().get_mut(&render_pass).unwrap();

        let dynamic_offsets = dynamic_offsets.unwrap();
        // TODO: validate safety.
        unsafe {
            wgpu_core::command::render_ffi::wgpu_render_pass_set_bind_group(
                &mut render_pass,
                index,
                bind_group,
                // TODO: Not sure that these are correct. Verify please.
                dynamic_offsets.as_ptr(),
                dynamic_offsets.len(),
            )
        };

        Ok(())
    }

    fn set_index_buffer(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _index_format: webgpu::GpuIndexFormat,
        _offset: webgpu::GpuSize64,
        _size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_vertex_buffer(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass>,
        slot: webgpu::GpuIndex32,
        buffer: Resource<webgpu::GpuBuffer>,
        offset: webgpu::GpuSize64,
        size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let buffer_id = self.table().get(&buffer).unwrap().buffer;
        let mut render_pass = self.table_mut().get_mut(&render_pass).unwrap();

        wgpu_core::command::render_ffi::wgpu_render_pass_set_vertex_buffer(
            &mut render_pass,
            slot,
            buffer_id,
            offset,
            Some(size.try_into().unwrap()),
        );

        Ok(())
    }

    fn draw_indexed(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _index_count: webgpu::GpuSize32,
        _instance_count: webgpu::GpuSize32,
        _first_index: webgpu::GpuSize32,
        _base_vertex: webgpu::GpuSignedOffset32,
        _first_instance: webgpu::GpuSize32,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn draw_indirect(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn draw_indexed_indirect(
        &mut self,
        _self_: Resource<wgpu_core::command::RenderPass>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }
}

impl<T: WasiView + HasGpuInstance> webgpu::HostGpuUncapturedErrorEvent for T {
    fn new(
        &mut self,
        _type_: String,
        _gpu_uncaptured_error_event_init_dict: webgpu::GpuUncapturedErrorEventInit,
    ) -> wasmtime::Result<Resource<webgpu::GpuUncapturedErrorEvent>> {
        todo!()
    }

    fn error(
        &mut self,
        _self_: Resource<webgpu::GpuUncapturedErrorEvent>,
    ) -> wasmtime::Result<Resource<webgpu::GpuError>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuUncapturedErrorEvent>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuInternalError for T {
    fn new(&mut self, _message: String) -> wasmtime::Result<Resource<webgpu::GpuInternalError>> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuInternalError>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuInternalError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuOutOfMemoryError for T {
    fn new(&mut self, _message: String) -> wasmtime::Result<Resource<webgpu::GpuOutOfMemoryError>> {
        todo!()
    }

    fn message(
        &mut self,
        _self_: Resource<webgpu::GpuOutOfMemoryError>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuOutOfMemoryError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuValidationError for T {
    fn new(&mut self, _message: String) -> wasmtime::Result<Resource<webgpu::GpuValidationError>> {
        todo!()
    }

    fn message(
        &mut self,
        _self_: Resource<webgpu::GpuValidationError>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuValidationError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuError for T {
    fn message(&mut self, _self_: Resource<webgpu::GpuError>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuDeviceLostInfo for T {
    fn reason(
        &mut self,
        _self_: Resource<webgpu::GpuDeviceLostInfo>,
    ) -> wasmtime::Result<webgpu::GpuDeviceLostReason> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuCanvasContext for T {
    fn canvas(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
    ) -> wasmtime::Result<webgpu::HtmlCanvasElementOrOffscreenCanvas> {
        todo!()
    }

    fn configure(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
        _configuration: webgpu::GpuCanvasConfiguration,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn unconfigure(&mut self, _self_: Resource<webgpu::GpuCanvasContext>) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_current_texture(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
    ) -> wasmtime::Result<Resource<webgpu::GpuTexture>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCanvasContext>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuRenderBundle for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuRenderBundle>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundle>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderBundle>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuComputePassEncoder for T {
    fn set_pipeline(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        pipeline: Resource<webgpu::GpuComputePipeline>,
    ) -> wasmtime::Result<()> {
        let pipeline = *self.table().get(&pipeline).unwrap();
        let encoder = self.table_mut().get_mut(&encoder).unwrap();
        wgpu_core::command::compute_ffi::wgpu_compute_pass_set_pipeline(encoder, pipeline);
        Ok(())
    }

    fn dispatch_workgroups(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        workgroup_count_x: webgpu::GpuSize32,
        workgroup_count_y: Option<webgpu::GpuSize32>,
        workgroup_count_z: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let encoder = self.table_mut().get_mut(&encoder).unwrap();
        wgpu_core::command::compute_ffi::wgpu_compute_pass_dispatch_workgroups(
            encoder,
            workgroup_count_x,
            workgroup_count_y.unwrap(),
            workgroup_count_z.unwrap(),
        );
        Ok(())
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn end(
        &mut self,
        cpass: Resource<wgpu_core::command::ComputePass>,
        non_standard_encoder: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<()> {
        let encoder = *self.table().get(&non_standard_encoder).unwrap();
        let cpass = self.table_mut().delete(cpass).unwrap();
        self.instance()
            .command_encoder_run_compute_pass::<crate::Backend>(encoder, &cpass)
            .unwrap();
        Ok(())
    }

    fn label(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _group_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        cpass: Resource<webgpu::GpuComputePassEncoder>,
        label: String,
    ) -> wasmtime::Result<()> {
        let cpass = self.table_mut().get_mut(&cpass).unwrap();
        unsafe {
            let label = std::ffi::CString::new(label).unwrap();
            wgpu_core::command::compute_ffi::wgpu_compute_pass_insert_debug_marker(
                cpass,
                label.as_ptr(),
                0,
            );
        }
        Ok(())
    }

    fn set_bind_group(
        &mut self,
        encoder: Resource<webgpu::GpuComputePassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Resource<webgpu::GpuBindGroup>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) -> wasmtime::Result<()> {
        let bind_group = *self.table().get(&bind_group).unwrap();
        let encoder = self.table_mut().get_mut(&encoder).unwrap();
        let dynamic_offsets = dynamic_offsets.unwrap();
        unsafe {
            wgpu_core::command::compute_ffi::wgpu_compute_pass_set_bind_group(
                encoder,
                index,
                bind_group,
                dynamic_offsets.as_ptr(),
                dynamic_offsets.len(),
            )
        }
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePassEncoder>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuPipelineError for T {
    fn new(
        &mut self,
        _message: Option<String>,
        _options: webgpu::GpuPipelineErrorInit,
    ) -> wasmtime::Result<Resource<webgpu::GpuPipelineError>> {
        todo!()
    }

    fn reason(
        &mut self,
        _self_: Resource<webgpu::GpuPipelineError>,
    ) -> wasmtime::Result<webgpu::GpuPipelineErrorReason> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuPipelineError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuCompilationMessage for T {
    fn message(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationMessage>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn type_(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationMessage>,
    ) -> wasmtime::Result<webgpu::GpuCompilationMessageType> {
        todo!()
    }

    fn line_num(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationMessage>,
    ) -> wasmtime::Result<u64> {
        todo!()
    }

    fn line_pos(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationMessage>,
    ) -> wasmtime::Result<u64> {
        todo!()
    }

    fn offset(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<u64> {
        todo!()
    }

    fn length(&mut self, _self_: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<u64> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuCompilationInfo for T {
    fn drop(&mut self, _rep: Resource<webgpu::GpuCompilationInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuQuerySet for T {
    fn destroy(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        todo!()
    }

    fn type_(
        &mut self,
        _self_: Resource<webgpu::GpuQuerySet>,
    ) -> wasmtime::Result<webgpu::GpuQueryType> {
        todo!()
    }

    fn count(
        &mut self,
        _self_: Resource<webgpu::GpuQuerySet>,
    ) -> wasmtime::Result<webgpu::GpuSize32Out> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuQuerySet>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuRenderBundleEncoder for T {
    fn finish(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _descriptor: Option<webgpu::GpuRenderBundleDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderBundle>> {
        todo!()
    }

    fn label(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _group_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_debug_group(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn insert_debug_marker(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _marker_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _index: webgpu::GpuIndex32,
        _bind_group: Resource<webgpu::GpuBindGroup>,
        _dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_pipeline(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _pipeline: Resource<wgpu_core::id::RenderPipelineId>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_index_buffer(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _index_format: webgpu::GpuIndexFormat,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_vertex_buffer(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _slot: webgpu::GpuIndex32,
        _buffer: Resource<webgpu::GpuBuffer>,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn draw(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _vertex_count: webgpu::GpuSize32,
        _instance_count: Option<webgpu::GpuSize32>,
        _first_vertex: Option<webgpu::GpuSize32>,
        _first_instance: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
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
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn draw_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn draw_indexed_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuRenderBundleEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderBundleEncoder>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuComputePipeline for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuComputePipeline>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_bind_group_layout(
        &mut self,
        compute_pipeline: Resource<webgpu::GpuComputePipeline>,
        index: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        let pipeline_id = self.table().get(&compute_pipeline).unwrap();
        let bind_group_layout = core_result(
            self.instance()
                .compute_pipeline_get_bind_group_layout::<crate::Backend>(*pipeline_id, index, ()),
        )
        .unwrap();
        Ok(self.table_mut().push(bind_group_layout).unwrap())
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuBindGroup for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuBindGroup>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuBindGroup>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBindGroup>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuPipelineLayout for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuPipelineLayout>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuPipelineLayout>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuPipelineLayout>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuBindGroupLayout for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuBindGroupLayout>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuBindGroupLayout>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBindGroupLayout>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuExternalTexture for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuExternalTexture>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuExternalTexture>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuExternalTexture>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuSampler for T {
    fn label(&mut self, _self_: Resource<webgpu::GpuSampler>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuSampler>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSampler>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuBuffer for T {
    fn size(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuSize64Out> {
        todo!()
    }

    fn usage(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        todo!()
    }

    fn map_state(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuBufferMapState> {
        todo!()
    }

    async fn map_async(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        mode: webgpu::GpuMapModeFlags,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let buffer = self.table().get(&buffer).unwrap().buffer;
        let instance = self.instance();
        CallbackFuture::new(Box::new(
            move |resolve: Box<
                dyn FnOnce(Box<Result<(), wgpu_core::resource::BufferAccessError>>) + Send,
            >| {
                // source: https://www.w3.org/TR/webgpu/#typedefdef-gpumapmodeflags
                let host = match mode {
                    0 => wgpu_core::device::HostMap::Read,
                    1 => wgpu_core::device::HostMap::Write,
                    _ => panic!(),
                };
                let op = wgpu_core::resource::BufferMapOperation {
                    host,
                    callback: wgpu_core::resource::BufferMapCallback::from_rust(Box::new(
                        move |result| {
                            resolve(Box::new(result));
                        },
                    )),
                };

                let offset = offset.unwrap();
                let size = size.unwrap();
                instance
                    .buffer_map_async::<crate::Backend>(buffer, offset..offset + size, op)
                    .unwrap();
                // TODO: only poll this device.
                instance.poll_all_devices(true).unwrap();
            },
        ))
        .await
        .unwrap();
        Ok(())
    }

    fn get_mapped_range(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<Resource<webgpu::GpuBuffer>> {
        let buffer_rep = buffer.rep();
        let buffer_id = self.table().get(&buffer).unwrap().buffer;
        let (ptr, len) = self
            .instance()
            .buffer_get_mapped_range::<crate::Backend>(buffer_id, offset.unwrap_or(0), size)
            .unwrap();
        let remote_buffer = BufferPtr { ptr, len };
        let buffer = self.table_mut().get_mut(&buffer).unwrap();
        buffer.mapped = Some(remote_buffer);
        Ok(Resource::new_own(buffer_rep))
    }

    fn unmap(&mut self, buffer: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        let buffer = self.table_mut().get_mut(&buffer).unwrap();
        buffer.mapped.take().unwrap();
        let buffer_id = buffer.buffer;
        self.instance()
            .buffer_unmap::<crate::Backend>(buffer_id)
            .unwrap();
        Ok(())
    }

    fn destroy(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpu for T {
    fn request_adapter(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
        _options: Option<webgpu::GpuRequestAdapterOptions>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::AdapterId>> {
        let adapter = self
            .instance()
            .request_adapter(
                &Default::default(),
                wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| ()),
            )
            .unwrap();
        Ok(self.table_mut().push(adapter).unwrap())
    }

    fn get_preferred_canvas_format(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
    ) -> wasmtime::Result<webgpu::GpuTextureFormat> {
        todo!()
    }

    fn wgsl_language_features(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
    ) -> wasmtime::Result<Resource<webgpu::WgslLanguageFeatures>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::Gpu>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuAdapterInfo for T {
    fn vendor(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<String> {
        todo!()
    }

    fn architecture(
        &mut self,
        _self_: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn device(&mut self, _self_: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<String> {
        todo!()
    }

    fn description(
        &mut self,
        _self_: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostWgslLanguageFeatures for T {
    fn has(
        &mut self,
        _self_: Resource<webgpu::WgslLanguageFeatures>,
        _key: String,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::WgslLanguageFeatures>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuSupportedFeatures for T {
    fn has(
        &mut self,
        features: Resource<webgpu::GpuSupportedFeatures>,
        query: String,
    ) -> wasmtime::Result<bool> {
        let features = self.table().get(&features).unwrap();
        Ok(match query.as_str() {
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
            // "float32-filterable" => features.contains(wgpu_types::Features::FLOAT32_FILTERABLE),
            _ => todo!(),
        })
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedFeatures>) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostGpuSupportedLimits for T {
    fn max_texture_dimension1_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_texture_dimension_1d)
    }

    fn max_texture_dimension2_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_texture_dimension_2d)
    }

    fn max_texture_dimension3_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_texture_dimension_3d)
    }

    fn max_texture_array_layers(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_texture_array_layers)
    }

    fn max_bind_groups(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_bind_groups)
    }

    fn max_bind_groups_plus_vertex_buffers(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_bindings_per_bind_group(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_bindings_per_bind_group)
    }

    fn max_dynamic_uniform_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_dynamic_uniform_buffers_per_pipeline_layout)
    }

    fn max_dynamic_storage_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_dynamic_storage_buffers_per_pipeline_layout)
    }

    fn max_sampled_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_sampled_textures_per_shader_stage)
    }

    fn max_samplers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_samplers_per_shader_stage)
    }

    fn max_storage_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_storage_buffers_per_shader_stage)
    }

    fn max_storage_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_storage_textures_per_shader_stage)
    }

    fn max_uniform_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_uniform_buffers_per_shader_stage)
    }

    fn max_uniform_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_uniform_buffer_binding_size as u64)
    }

    fn max_storage_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_storage_buffer_binding_size as u64)
    }

    fn min_uniform_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.min_uniform_buffer_offset_alignment)
    }

    fn min_storage_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.min_storage_buffer_offset_alignment)
    }

    fn max_vertex_buffers(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_vertex_buffers)
    }

    fn max_buffer_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_buffer_size)
    }

    fn max_vertex_attributes(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_vertex_attributes)
    }

    fn max_vertex_buffer_array_stride(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_vertex_buffer_array_stride)
    }

    fn max_inter_stage_shader_components(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_inter_stage_shader_components)
    }

    fn max_inter_stage_shader_variables(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_color_attachments(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_color_attachment_bytes_per_sample(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroup_storage_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_workgroup_storage_size)
    }

    fn max_compute_invocations_per_workgroup(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_invocations_per_workgroup)
    }

    fn max_compute_workgroup_size_x(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_workgroup_size_x)
    }

    fn max_compute_workgroup_size_y(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_workgroup_size_y)
    }

    fn max_compute_workgroup_size_z(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_workgroup_size_z)
    }

    fn max_compute_workgroups_per_dimension(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits).unwrap();
        Ok(limits.max_compute_workgroups_per_dimension)
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedLimits>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostAllowSharedBufferSource for T {
    fn drop(&mut self, _rep: Resource<webgpu::AllowSharedBufferSource>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostPredefinedColorSpace for T {
    fn drop(&mut self, _rep: Resource<webgpu::PredefinedColorSpace>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostEventHandler for T {
    fn drop(&mut self, _rep: Resource<webgpu::EventHandler>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostOffscreenCanvas for T {
    fn drop(&mut self, _rep: Resource<webgpu::OffscreenCanvas>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostHtmlCanvasElement for T {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlCanvasElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostVideoFrame for T {
    fn drop(&mut self, _rep: Resource<webgpu::VideoFrame>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostHtmlVideoElement for T {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlVideoElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostHtmlImageElement for T {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlImageElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostImageData for T {
    fn drop(&mut self, _rep: Resource<webgpu::ImageData>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostImageBitmap for T {
    fn drop(&mut self, _rep: Resource<webgpu::ImageBitmap>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostArrayBuffer for T {
    fn drop(&mut self, _rep: Resource<webgpu::ArrayBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiView + HasGpuInstance> webgpu::HostUint32Array for T {
    fn drop(&mut self, _rep: Resource<webgpu::Uint32Array>) -> wasmtime::Result<()> {
        todo!()
    }
}

fn core_result<I, E>(
    (id, error): (wgpu_core::id::Id<I>, Option<E>),
) -> Result<wgpu_core::id::Id<I>, E> {
    match error {
        Some(error) => Err(error),
        None => Ok(id),
    }
}
