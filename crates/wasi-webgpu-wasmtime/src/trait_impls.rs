use std::{borrow::Cow, mem, num::NonZeroU64, sync::Arc};

use callback_future::CallbackFuture;
use futures::executor::block_on;
use wasi_graphics_context_wasmtime::{Context, DisplayApi};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::{
    to_core_conversions::ToCore,
    wasi::webgpu::webgpu,
    wrapper_types::{Buffer, BufferPtr, ComputePassEncoder, Device, RenderPassEncoder},
    AbstractBuffer, MainThreadSpawner, WasiWebGpuImpl, WasiWebGpuView, WebGpuSurface,
};

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
        let buffer = self.table().get_mut(&buffer).unwrap();
        buffer.slice_mut().to_vec()
    }

    fn set(&mut self, buffer: Resource<webgpu::NonStandardBuffer>, val: Vec<u8>) {
        let buffer = self.table().get_mut(&buffer).unwrap();
        buffer.slice_mut().copy_from_slice(&val);
    }

    fn drop(&mut self, buffer: Resource<webgpu::NonStandardBuffer>) -> wasmtime::Result<()> {
        self.table().delete(buffer).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuDevice for WasiWebGpuImpl<T> {
    fn connect_graphics_context(&mut self, device: Resource<Device>, context: Resource<Context>) {
        let device = self.table().get(&device).unwrap();
        let device_id = device.device;
        let adapter_id = device.adapter;

        let instance = Arc::downgrade(&self.instance());
        let surface_creator = self.ui_thread_spawner();

        let context = self.table().get_mut(&context).unwrap();

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
        let device = self.table().get(&device).unwrap().device;

        let command_encoder = core_result(
            self.instance()
                .device_create_command_encoder::<crate::Backend>(
                    device,
                    &descriptor
                        .map(|d| d.to_core(&self.table()))
                        .unwrap_or(wgpu_types::CommandEncoderDescriptor::default()),
                    None,
                ),
        )
        .unwrap();

        self.table().push(command_encoder).unwrap()
    }

    fn create_shader_module(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuShaderModuleDescriptor,
    ) -> Resource<webgpu::GpuShaderModule> {
        let device = self.table().get(&device).unwrap().device;

        let code =
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Owned(descriptor.code.to_owned()));
        let shader = core_result(
            self.instance()
                .device_create_shader_module::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.table()),
                    code,
                    None,
                ),
        )
        .unwrap();

        self.table().push(shader).unwrap()
    }

    fn create_render_pipeline(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> Resource<wgpu_core::id::RenderPipelineId> {
        let host_device = self.table().get(&device).unwrap().device;
        let render_pipeline = core_result(
            self.instance()
                .device_create_render_pipeline::<crate::Backend>(
                    host_device,
                    &descriptor.to_core(&self.table()),
                    None,
                    None,
                ),
        )
        .unwrap();

        self.table().push_child(render_pipeline, &device).unwrap()
    }

    fn queue(&mut self, device: Resource<Device>) -> Resource<wgpu_core::id::QueueId> {
        let queue = self.table().get(&device).unwrap().queue;
        self.table().push(queue).unwrap()
    }

    fn features(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> Resource<webgpu::GpuSupportedFeatures> {
        let device = self.table().get(&device).unwrap().device;
        let features = self
            .instance()
            .device_features::<crate::Backend>(device)
            .unwrap();
        self.table().push(features).unwrap()
    }

    fn limits(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> Resource<webgpu::GpuSupportedLimits> {
        let device = self.table().get(&device).unwrap().device;
        let limits = self
            .instance()
            .device_limits::<crate::Backend>(device)
            .unwrap();
        self.table().push(limits).unwrap()
    }

    fn destroy(&mut self, device: Resource<webgpu::GpuDevice>) {
        let device_id = self.table().get(&device).unwrap().device;
        self.instance().device_destroy::<crate::Backend>(device_id);
    }

    fn create_buffer(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBufferDescriptor,
    ) -> Resource<webgpu::GpuBuffer> {
        let device = self.table().get(&device).unwrap().device;

        let size = descriptor.size;
        let buffer_id = core_result(self.instance().device_create_buffer::<crate::Backend>(
            device,
            &descriptor.to_core(&self.table()),
            None,
        ))
        .unwrap();

        let buffer = Buffer { buffer_id, size };

        self.table().push(buffer).unwrap()
    }

    fn create_texture(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuTextureDescriptor,
    ) -> Resource<webgpu::GpuTexture> {
        let device = self.table().get(&device).unwrap().device;
        let texture = core_result(self.instance().device_create_texture::<crate::Backend>(
            device,
            &descriptor.to_core(&self.table()),
            None,
        ))
        .unwrap();

        self.table().push(texture).unwrap()
    }

    fn create_sampler(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: Option<webgpu::GpuSamplerDescriptor>,
    ) -> Resource<webgpu::GpuSampler> {
        let device = self.table().get(&device).unwrap().device;

        let descriptor = descriptor
            .map(|d| d.to_core(&self.table()))
            // https://www.w3.org/TR/webgpu/#dictdef-gpusamplerdescriptor
            .unwrap_or_else(|| wgpu_core::resource::SamplerDescriptor {
                label: None,
                address_modes: [wgpu_types::AddressMode::ClampToEdge; 3],
                mag_filter: wgpu_types::FilterMode::Nearest,
                min_filter: wgpu_types::FilterMode::Nearest,
                mipmap_filter: wgpu_types::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 32.0,
                compare: None,
                // TODO: make sure that anisotropy_clamp actually corresponds to maxAnisotropy
                anisotropy_clamp: 1,
                // border_color is not present in WebGPU
                border_color: None,
            });

        let sampler = core_result(self.instance().device_create_sampler::<crate::Backend>(
            device,
            &descriptor,
            None,
        ))
        .unwrap();

        self.table().push(sampler).unwrap()
    }

    fn create_bind_group_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupLayoutDescriptor,
    ) -> Resource<webgpu::GpuBindGroupLayout> {
        let device = self.table().get(&device).unwrap().device;

        let bind_group_layout = core_result(
            self.instance()
                .device_create_bind_group_layout::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.table()),
                    None,
                ),
        )
        .unwrap();

        self.table().push(bind_group_layout).unwrap()
    }

    fn create_pipeline_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuPipelineLayoutDescriptor,
    ) -> Resource<webgpu::GpuPipelineLayout> {
        let device = self.table().get(&device).unwrap().device;

        let pipeline_layout = core_result(
            self.instance()
                .device_create_pipeline_layout::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.table()),
                    None,
                ),
        )
        .unwrap();

        self.table().push(pipeline_layout).unwrap()
    }

    fn create_bind_group(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupDescriptor,
    ) -> Resource<webgpu::GpuBindGroup> {
        let device = self.table().get(&device).unwrap().device;

        let bind_group = core_result(self.instance().device_create_bind_group::<crate::Backend>(
            device,
            &descriptor.to_core(&self.table()),
            None,
        ))
        .unwrap();

        self.table().push(bind_group).unwrap()
    }

    fn create_compute_pipeline(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> Resource<webgpu::GpuComputePipeline> {
        let device = self.table().get(&device).unwrap().device;
        let compute_pipeline = core_result(
            self.instance()
                .device_create_compute_pipeline::<crate::Backend>(
                    device,
                    &descriptor.to_core(&self.table()),
                    None,
                    None,
                ),
        )
        .unwrap();
        self.table().push(compute_pipeline).unwrap()
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
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuRenderBundleEncoderDescriptor,
    ) -> Resource<webgpu::GpuRenderBundleEncoder> {
        let device = self.table().get(&device).unwrap().device;
        let render_bundle_encoder = wgpu_core::command::RenderBundleEncoder::new(
            &descriptor.to_core(&self.table()),
            device,
            None,
        )
        .unwrap();
        self.table().push(render_bundle_encoder).unwrap()
    }

    fn create_query_set(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuQuerySetDescriptor,
    ) -> Resource<webgpu::GpuQuerySet> {
        let device = self.table().get(&device).unwrap().device;
        let query_set = core_result(self.instance().device_create_query_set::<crate::Backend>(
            device,
            &descriptor.to_core(&self.table()),
            None,
        ))
        .unwrap();
        self.table().push(query_set).unwrap()
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

    fn drop(&mut self, device: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        self.table().delete(device).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTexture for WasiWebGpuImpl<T> {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<AbstractBuffer>,
    ) -> Resource<wgpu_core::id::TextureId> {
        let host_buffer = self.table().delete(buffer).unwrap();
        let host_buffer: wgpu_core::id::TextureId = host_buffer.inner_type();
        self.table().push(host_buffer).unwrap()
    }

    fn create_view(
        &mut self,
        texture: Resource<wgpu_core::id::TextureId>,
        descriptor: Option<webgpu::GpuTextureViewDescriptor>,
    ) -> Resource<wgpu_core::id::TextureViewId> {
        let texture_id = *self.table().get(&texture).unwrap();
        let texture_view = core_result(
            self.instance().texture_create_view::<crate::Backend>(
                texture_id,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or(wgpu_core::resource::TextureViewDescriptor::default()),
                None,
            ),
        )
        .unwrap();
        self.table().push(texture_view).unwrap()
    }

    fn destroy(&mut self, texture: Resource<webgpu::GpuTexture>) {
        let texture_id = *self.table().get(&texture).unwrap();
        self.instance()
            .texture_destroy::<crate::Backend>(texture_id)
            .unwrap();
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

    fn drop(&mut self, texture: Resource<webgpu::GpuTexture>) -> wasmtime::Result<()> {
        self.table().delete(texture).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTextureView for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<wgpu_core::id::TextureViewId>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<wgpu_core::id::TextureViewId>, _label: String) {
        todo!()
    }

    fn drop(&mut self, view: Resource<wgpu_core::id::TextureViewId>) -> wasmtime::Result<()> {
        self.table().delete(view).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandBuffer for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuCommandBuffer>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuCommandBuffer>, _label: String) {
        todo!()
    }

    fn drop(&mut self, command_buffer: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        self.table().delete(command_buffer).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuShaderModule for WasiWebGpuImpl<T> {
    fn get_compilation_info(
        &mut self,
        _self_: Resource<webgpu::GpuShaderModule>,
    ) -> Resource<webgpu::GpuCompilationInfo> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuShaderModule>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<webgpu::GpuShaderModule>, _label: String) {
        todo!()
    }

    fn drop(&mut self, shader: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        self.table().delete(shader).unwrap();
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
        pipeline: Resource<wgpu_core::id::RenderPipelineId>,
        index: u32,
    ) -> Resource<webgpu::GpuBindGroupLayout> {
        let pipeline_id = *self.table().get(&pipeline).unwrap();
        let layout = core_result(
            self.instance()
                .render_pipeline_get_bind_group_layout::<crate::Backend>(pipeline_id, index, None),
        )
        .unwrap();
        self.table().push(layout).unwrap()
    }

    fn drop(&mut self, pipeline: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        self.table().delete(pipeline).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuAdapter for WasiWebGpuImpl<T> {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
        descriptor: Option<webgpu::GpuDeviceDescriptor>,
    ) -> Resource<webgpu::GpuDevice> {
        let adapter_id = *self.table().get(&adapter).unwrap();

        let (device_id, queue_id) = core_results_2(
            self.instance().adapter_request_device::<crate::Backend>(
                adapter_id,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or(wgpu_types::DeviceDescriptor::default()),
                None,
                None,
                None,
            ),
        )
        .unwrap();

        let device = self
            .table()
            .push(Device {
                device: device_id,
                queue: queue_id,
                adapter: adapter_id,
            })
            .unwrap();

        device
    }

    fn features(
        &mut self,
        adapter: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::component::Resource<webgpu::GpuSupportedFeatures> {
        let adapter = *self.table().get(&adapter).unwrap();
        let features = self
            .instance()
            .adapter_features::<crate::Backend>(adapter)
            .unwrap();
        self.table().push(features).unwrap()
    }

    fn limits(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> Resource<webgpu::GpuSupportedLimits> {
        let adapter = *self.table().get(&adapter).unwrap();
        let limits = self
            .instance()
            .adapter_limits::<crate::Backend>(adapter)
            .unwrap();
        self.table().push(limits).unwrap()
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
        let adapter_id = *self.table().get(&adapter).unwrap();
        let info = self
            .instance()
            .adapter_get_info::<crate::Backend>(adapter_id)
            .unwrap();
        let info = self.table().push(info).unwrap();
        info
    }

    fn drop(&mut self, adapter: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        self.table().delete(adapter).unwrap();
        Ok(())
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
            .map(|buffer| *self.table().get(&buffer).unwrap())
            .collect::<Vec<_>>();
        let queue = *self.table().get(&queue).unwrap();
        self.instance()
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
        let queue = *self.table().get(&queue).unwrap();
        let buffer_id = self.table().get(&buffer).unwrap().buffer_id;
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
            .queue_write_buffer::<crate::Backend>(queue, buffer_id, buffer_offset, &data)
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
        let queue = *self.table().get(&queue).unwrap();
        self.instance()
            .queue_write_texture::<crate::Backend>(
                queue,
                &destination.to_core(&self.table()),
                &data,
                &data_layout.to_core(&self.table()),
                &size.to_core(&self.table()),
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
        self.table().delete(queue).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandEncoder for WasiWebGpuImpl<T> {
    fn begin_render_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: webgpu::GpuRenderPassDescriptor,
    ) -> Resource<webgpu::GpuRenderPassEncoder> {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        let timestamp_writes = descriptor
            .timestamp_writes
            .map(|tw| tw.to_core(&self.table()));
        // can't use to_core because depth_stencil_attachment is Option<&x>.
        let depth_stencil_attachment = descriptor
            .depth_stencil_attachment
            .map(|d| d.to_core(&self.table()));
        let descriptor = wgpu_core::command::RenderPassDescriptor {
            label: descriptor.label.map(|l| l.into()),
            color_attachments: descriptor
                .color_attachments
                .into_iter()
                .map(|c| c.map(|c| c.to_core(&self.table())))
                .collect::<Vec<_>>()
                .into(),
            depth_stencil_attachment: depth_stencil_attachment.as_ref(),
            timestamp_writes: timestamp_writes.as_ref(),
            occlusion_query_set: descriptor
                .occlusion_query_set
                .map(|oqs| oqs.to_core(&self.table())),
            // TODO: self.max_draw_count not used
        };
        let render_pass = core_result_t(
            self.instance()
                .command_encoder_create_render_pass::<crate::Backend>(command_encoder, &descriptor),
        )
        .unwrap();

        self.table()
            .push(RenderPassEncoder::new(render_pass))
            .unwrap()
    }

    fn finish(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuCommandBufferDescriptor>,
    ) -> Resource<webgpu::GpuCommandBuffer> {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        let command_buffer = core_result(
            self.instance().command_encoder_finish::<crate::Backend>(
                command_encoder,
                &descriptor
                    .map(|d| d.to_core(&self.table()))
                    .unwrap_or(wgpu_types::CommandBufferDescriptor::default()),
            ),
        )
        .unwrap();
        self.table().push(command_buffer).unwrap()
    }

    fn begin_compute_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: Option<webgpu::GpuComputePassDescriptor>,
    ) -> Resource<webgpu::GpuComputePassEncoder> {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        let compute_pass = core_result_t(
            self.instance()
                .command_encoder_create_compute_pass::<crate::Backend>(
                    command_encoder,
                    // can't use to_core because timestamp_writes is Option<&x>.
                    &wgpu_core::command::ComputePassDescriptor {
                        // TODO: can we get rid of the clone here?
                        label: descriptor
                            .as_ref()
                            .map(|d| d.label.clone().map(|l| l.into()))
                            .flatten(),
                        timestamp_writes: descriptor
                            .map(|d| d.timestamp_writes.map(|tw| tw.to_core(&self.table())))
                            .flatten()
                            .as_ref(),
                    },
                ),
        )
        .unwrap();
        self.table()
            .push(ComputePassEncoder::new(compute_pass))
            .unwrap()
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
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        let source = self.table().get(&source).unwrap().buffer_id;
        let destination = self.table().get(&destination).unwrap().buffer_id;
        self.instance()
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
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        source: webgpu::GpuImageCopyBuffer,
        destination: webgpu::GpuImageCopyTexture,
        copy_size: webgpu::GpuExtent3D,
    ) {
        let command_encoder = *self.table().get(&command_encoder).unwrap();
        self.instance()
            .command_encoder_copy_buffer_to_texture::<crate::Backend>(
                command_encoder,
                &source.to_core(&self.table()),
                &destination.to_core(&self.table()),
                &copy_size.to_core(self.table()),
            )
            .unwrap();
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
        let _command_encoder = self.table().get(&command_encoder).unwrap();
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

    fn drop(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<()> {
        self.table().delete(command_encoder).unwrap();
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuRenderPassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) {
        let instance = self.instance();
        let pipeline = pipeline.to_core(&self.table());
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_set_pipeline(render_pass, pipeline)
            .unwrap()
    }

    fn draw(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        vertex_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_vertex: Option<webgpu::GpuSize32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        instance
            .render_pass_draw(
                render_pass,
                vertex_count,
                instance_count.unwrap_or(1),
                first_vertex.unwrap_or(0),
                first_instance.unwrap_or(0),
            )
            .unwrap()
    }

    fn end(&mut self, render_pass: Resource<RenderPassEncoder>) {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let mut render_pass = render_pass.take().unwrap();
        instance
            .render_pass_end::<crate::Backend>(&mut render_pass)
            .unwrap();
    }

    fn set_viewport(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_set_viewport(render_pass, x, y, width, height, min_depth, max_depth)
            .unwrap();
    }

    fn set_scissor_rect(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        x: webgpu::GpuIntegerCoordinate,
        y: webgpu::GpuIntegerCoordinate,
        width: webgpu::GpuIntegerCoordinate,
        height: webgpu::GpuIntegerCoordinate,
    ) {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_set_scissor_rect(render_pass, x, y, width, height)
            .unwrap();
    }

    fn set_blend_constant(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _color: webgpu::GpuColor,
    ) {
        todo!()
    }

    fn set_stencil_reference(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _reference: webgpu::GpuStencilValue,
    ) {
        todo!()
    }

    fn begin_occlusion_query(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _query_index: webgpu::GpuSize32,
    ) {
        todo!()
    }

    fn end_occlusion_query(&mut self, _self_: Resource<RenderPassEncoder>) {
        todo!()
    }

    fn execute_bundles(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _bundles: Vec<Resource<webgpu::GpuRenderBundle>>,
    ) {
        todo!()
    }

    fn label(&mut self, _self_: Resource<RenderPassEncoder>) -> String {
        todo!()
    }

    fn set_label(&mut self, _self_: Resource<RenderPassEncoder>, _label: String) {
        todo!()
    }

    fn push_debug_group(&mut self, _self_: Resource<RenderPassEncoder>, _group_label: String) {
        todo!()
    }

    fn pop_debug_group(&mut self, _self_: Resource<RenderPassEncoder>) {
        todo!()
    }

    fn insert_debug_marker(&mut self, _self_: Resource<RenderPassEncoder>, _marker_label: String) {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) {
        let instance = self.instance();
        let bind_group = *self
            .table()
            .get(&bind_group.expect("TODO: deal with null bind_groups"))
            .unwrap();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let mut render_pass = render_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#programmable-passes
        instance
            .render_pass_set_bind_group(
                &mut render_pass,
                index,
                bind_group,
                &dynamic_offsets.unwrap_or(vec![]),
            )
            .unwrap()
    }

    fn set_index_buffer(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        buffer: Resource<webgpu::GpuBuffer>,
        index_format: webgpu::GpuIndexFormat,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let instance = self.instance();
        let buffer_id = self.table().get(&buffer).unwrap().buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_set_index_buffer(
                render_pass,
                buffer_id,
                index_format.into(),
                // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
                offset.unwrap_or(0),
                size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
            )
            .unwrap()
    }

    fn set_vertex_buffer(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        slot: webgpu::GpuIndex32,
        buffer: Option<Resource<webgpu::GpuBuffer>>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) {
        let instance = self.instance();
        let buffer_id = self
            .table()
            .get(&buffer.expect("TODO: deal null buffers"))
            .unwrap()
            .buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let mut render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_set_vertex_buffer(
                &mut render_pass,
                slot,
                buffer_id,
                // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
                offset.unwrap_or(0),
                size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
            )
            .unwrap()
    }

    fn draw_indexed(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        index_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_index: Option<webgpu::GpuSize32>,
        base_vertex: Option<webgpu::GpuSignedOffset32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass).unwrap().lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance
            .render_pass_draw_indexed(
                render_pass,
                index_count,
                // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
                instance_count.unwrap_or(1),
                first_index.unwrap_or(0),
                base_vertex.unwrap_or(0),
                first_instance.unwrap_or(0),
            )
            .unwrap()
    }

    fn draw_indirect(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn draw_indexed_indirect(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) {
        todo!()
    }

    fn drop(&mut self, render_pass: Resource<RenderPassEncoder>) -> wasmtime::Result<()> {
        self.table().delete(render_pass).unwrap();
        Ok(())
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

    fn drop(&mut self, error: Resource<webgpu::GpuUncapturedErrorEvent>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuInternalError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuInternalError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuInternalError>) -> String {
        todo!()
    }

    fn drop(&mut self, error: Resource<webgpu::GpuInternalError>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuOutOfMemoryError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuOutOfMemoryError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuOutOfMemoryError>) -> String {
        todo!()
    }

    fn drop(&mut self, error: Resource<webgpu::GpuOutOfMemoryError>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuValidationError for WasiWebGpuImpl<T> {
    fn new(&mut self, _message: String) -> Resource<webgpu::GpuValidationError> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuValidationError>) -> String {
        todo!()
    }

    fn drop(&mut self, error: Resource<webgpu::GpuValidationError>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuError for WasiWebGpuImpl<T> {
    fn message(&mut self, _self_: Resource<webgpu::GpuError>) -> String {
        todo!()
    }

    fn drop(&mut self, error: Resource<webgpu::GpuError>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
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

    fn drop(&mut self, info: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<()> {
        self.table().delete(info).unwrap();
        Ok(())
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

    fn drop(&mut self, bundle: Resource<webgpu::GpuRenderBundle>) -> wasmtime::Result<()> {
        self.table().delete(bundle).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuComputePassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        pipeline: Resource<webgpu::GpuComputePipeline>,
    ) {
        let instance = self.instance();
        let pipeline = *self.table().get(&pipeline).unwrap();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance
            .compute_pass_set_pipeline(compute_pass, pipeline)
            .unwrap();
    }

    fn dispatch_workgroups(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        workgroup_count_x: webgpu::GpuSize32,
        workgroup_count_y: Option<webgpu::GpuSize32>,
        workgroup_count_z: Option<webgpu::GpuSize32>,
    ) {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpucomputepassencoder
        instance
            .compute_pass_dispatch_workgroups(
                compute_pass,
                workgroup_count_x,
                workgroup_count_y.unwrap_or(1),
                workgroup_count_z.unwrap_or(1),
            )
            .unwrap()
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) {
        let instance = self.instance();
        let indirect_buffer = self.table().get(&indirect_buffer).unwrap().buffer_id;
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance
            .compute_pass_dispatch_workgroups_indirect(
                compute_pass,
                indirect_buffer,
                indirect_offset,
            )
            .unwrap();
    }

    fn end(&mut self, compute_pass: Resource<webgpu::GpuComputePassEncoder>) {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let mut compute_pass = compute_pass.take().unwrap();
        instance
            .compute_pass_end::<crate::Backend>(&mut compute_pass)
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
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        group_label: String,
    ) {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance
            .compute_pass_push_debug_group(compute_pass, &group_label, 0)
            .unwrap();
    }

    fn pop_debug_group(&mut self, compute_pass: Resource<webgpu::GpuComputePassEncoder>) {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_pop_debug_group(compute_pass).unwrap();
    }

    fn insert_debug_marker(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        label: String,
    ) {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance
            .compute_pass_insert_debug_marker(compute_pass, &label, 0)
            .unwrap()
    }

    fn set_bind_group(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) {
        let instance = self.instance();
        let bind_group = *self
            .table()
            .get(&bind_group.expect("TODO: deal with null bind_groups"))
            .unwrap();
        let mut compute_pass = self.table().get_mut(&compute_pass).unwrap().lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#programmable-passes
        instance
            .compute_pass_set_bind_group(
                compute_pass,
                index,
                bind_group,
                &dynamic_offsets.unwrap_or(vec![]),
            )
            .unwrap()
    }

    fn drop(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<()> {
        self.table().delete(compute_pass).unwrap();
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

    fn drop(&mut self, error: Resource<webgpu::GpuPipelineError>) -> wasmtime::Result<()> {
        self.table().delete(error).unwrap();
        Ok(())
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

    fn drop(&mut self, cm: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<()> {
        self.table().delete(cm).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCompilationInfo for WasiWebGpuImpl<T> {
    fn messages(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationInfo>,
    ) -> Vec<Resource<webgpu::GpuCompilationMessage>> {
        todo!()
    }

    fn drop(&mut self, info: Resource<webgpu::GpuCompilationInfo>) -> wasmtime::Result<()> {
        self.table().delete(info).unwrap();
        Ok(())
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

    fn drop(&mut self, query_set: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        self.table().delete(query_set).unwrap();
        Ok(())
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

    fn drop(&mut self, encoder: Resource<webgpu::GpuRenderBundleEncoder>) -> wasmtime::Result<()> {
        self.table().delete(encoder).unwrap();
        Ok(())
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
        let pipeline_id = *self.table().get(&compute_pipeline).unwrap();
        let bind_group_layout = core_result(
            self.instance()
                .compute_pipeline_get_bind_group_layout::<crate::Backend>(pipeline_id, index, None),
        )
        .unwrap();
        self.table().push(bind_group_layout).unwrap()
    }

    fn drop(&mut self, pipeline: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<()> {
        self.table().delete(pipeline).unwrap();
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

    fn drop(&mut self, bind_group: Resource<webgpu::GpuBindGroup>) -> wasmtime::Result<()> {
        self.table().delete(bind_group).unwrap();
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

    fn drop(&mut self, layout: Resource<webgpu::GpuPipelineLayout>) -> wasmtime::Result<()> {
        self.table().delete(layout).unwrap();
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

    fn drop(&mut self, layout: Resource<webgpu::GpuBindGroupLayout>) -> wasmtime::Result<()> {
        self.table().delete(layout).unwrap();
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

    fn drop(&mut self, sampler: Resource<webgpu::GpuSampler>) -> wasmtime::Result<()> {
        self.table().delete(sampler).unwrap();
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
        let buffer_id = self.table().get(&buffer).unwrap().buffer_id;
        let instance = self.instance();
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

                // https://www.w3.org/TR/webgpu/#gpubuffer
                let offset = offset.unwrap_or(0);
                instance
                    .buffer_map_async::<crate::Backend>(buffer_id, offset, size, op)
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
        let buffer_id = self.table().get(&buffer).unwrap().buffer_id;
        let (ptr, len) = self
            .instance()
            // https://www.w3.org/TR/webgpu/#gpubuffer
            .buffer_get_mapped_range::<crate::Backend>(buffer_id, offset.unwrap_or(0), size)
            .unwrap();
        let remote_buffer = BufferPtr { ptr, len };
        self.table().push(remote_buffer).unwrap()
    }

    fn unmap(&mut self, buffer: Resource<webgpu::GpuBuffer>) {
        let buffer_id = self.table().get_mut(&buffer).unwrap().buffer_id;
        self.instance()
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

    fn drop(&mut self, buffer: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        self.table().delete(buffer).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpu for WasiWebGpuImpl<T> {
    fn request_adapter(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
        options: Option<webgpu::GpuRequestAdapterOptions>,
    ) -> Option<Resource<wgpu_core::id::AdapterId>> {
        let adapter = self.instance().request_adapter(
            &options
                .map(|o| o.to_core(self.table()))
                .unwrap_or(wgpu_types::RequestAdapterOptions::default()),
            wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| None),
        );
        if let Err(wgpu_core::instance::RequestAdapterError::NotFound) = adapter {
            return None;
        }
        adapter.ok().map(|a| self.table().push(a).unwrap())
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

    fn drop(&mut self, _gpu: Resource<webgpu::Gpu>) -> wasmtime::Result<()> {
        // not actually a resource in the table
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

    fn drop(&mut self, info: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<()> {
        self.table().delete(info).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostWgslLanguageFeatures for WasiWebGpuImpl<T> {
    fn has(&mut self, _self_: Resource<webgpu::WgslLanguageFeatures>, _key: String) -> bool {
        todo!()
    }

    fn drop(&mut self, features: Resource<webgpu::WgslLanguageFeatures>) -> wasmtime::Result<()> {
        self.table().delete(features).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedFeatures for WasiWebGpuImpl<T> {
    fn has(&mut self, features: Resource<webgpu::GpuSupportedFeatures>, query: String) -> bool {
        let features = self.table().get(&features).unwrap();
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

    fn drop(&mut self, features: Resource<webgpu::GpuSupportedFeatures>) -> wasmtime::Result<()> {
        self.table().delete(features).unwrap();
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedLimits for WasiWebGpuImpl<T> {
    fn max_texture_dimension1_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_texture_dimension_1d
    }

    fn max_texture_dimension2_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_texture_dimension_2d
    }

    fn max_texture_dimension3_d(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_texture_dimension_3d
    }

    fn max_texture_array_layers(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_texture_array_layers
    }

    fn max_bind_groups(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_bind_groups
    }

    fn max_bind_groups_plus_vertex_buffers(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        todo!()
    }

    fn max_bindings_per_bind_group(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_bindings_per_bind_group
    }

    fn max_dynamic_uniform_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_dynamic_uniform_buffers_per_pipeline_layout
    }

    fn max_dynamic_storage_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_dynamic_storage_buffers_per_pipeline_layout
    }

    fn max_sampled_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_sampled_textures_per_shader_stage
    }

    fn max_samplers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_samplers_per_shader_stage
    }

    fn max_storage_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_storage_buffers_per_shader_stage
    }

    fn max_storage_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_storage_textures_per_shader_stage
    }

    fn max_uniform_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_uniform_buffers_per_shader_stage
    }

    fn max_uniform_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u64 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_uniform_buffer_binding_size as u64
    }

    fn max_storage_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u64 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_storage_buffer_binding_size as u64
    }

    fn min_uniform_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.min_uniform_buffer_offset_alignment
    }

    fn min_storage_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.min_storage_buffer_offset_alignment
    }

    fn max_vertex_buffers(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_vertex_buffers
    }

    fn max_buffer_size(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u64 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_buffer_size
    }

    fn max_vertex_attributes(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_vertex_attributes
    }

    fn max_vertex_buffer_array_stride(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
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
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_workgroup_storage_size
    }

    fn max_compute_invocations_per_workgroup(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_invocations_per_workgroup
    }

    fn max_compute_workgroup_size_x(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_x
    }

    fn max_compute_workgroup_size_y(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_y
    }

    fn max_compute_workgroup_size_z(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_workgroup_size_z
    }

    fn max_compute_workgroups_per_dimension(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> u32 {
        let limits = self.table().get(&limits).unwrap();
        limits.max_compute_workgroups_per_dimension
    }

    fn drop(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> wasmtime::Result<()> {
        self.table().delete(limits).unwrap();
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
