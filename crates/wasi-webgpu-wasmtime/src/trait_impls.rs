use core::slice;
use std::{
    borrow::Cow,
    num::NonZeroU64,
    sync::{Arc, Weak},
};

use callback_future::CallbackFuture;
use futures::executor::block_on;
use shared::Listener;
use wasi_graphics_context_wasmtime::{Context, DisplayApi};
use wasmtime::component::Resource;
use wasmtime_wasi_io::IoView;

use crate::{
    to_core_conversions::ToCore,
    wasi::{io::poll, webgpu::webgpu},
    wrapper_types::{
        Buffer, CommandEncoder, ComputePassEncoder, ComputePipeline, Device, ErrorHandler,
        RenderBundleEncoder, RenderBundleEncoderInner, RenderPassEncoder, RenderPipeline, Texture,
    },
    AbstractBuffer, MainThreadSpawner, WasiWebGpuImpl, WasiWebGpuView, WebGpuSurface,
    PREFERRED_CANVAS_FORMAT,
};

impl<T: WasiWebGpuView> webgpu::Host for WasiWebGpuImpl<T> {
    fn get_gpu(&mut self) -> wasmtime::Result<Resource<webgpu::Gpu>> {
        Ok(Resource::new_own(0))
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuColorWrite for WasiWebGpuImpl<T> {
    fn red(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ColorWrites::RED.bits())
    }

    fn green(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ColorWrites::GREEN.bits())
    }

    fn blue(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ColorWrites::BLUE.bits())
    }

    fn alpha(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ColorWrites::ALPHA.bits())
    }

    fn all(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ColorWrites::ALL.bits())
    }

    fn drop(&mut self, _self_: Resource<webgpu::GpuColorWrite>) -> wasmtime::Result<()> {
        unreachable!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostRecordGpuPipelineConstantValue for WasiWebGpuImpl<T> {
    fn new(&mut self) -> wasmtime::Result<Resource<webgpu::RecordGpuPipelineConstantValue>> {
        todo!()
    }

    fn add(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
        _value: webgpu::GpuPipelineConstantValue,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    // fn get(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>, _key: String) -> Option<webgpu::GpuPipelineConstantValue> {
    fn get(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
    ) -> wasmtime::Result<Option<webgpu::GpuPipelineConstantValue>> {
        todo!()
    }

    fn has(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn remove(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
        _key: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn keys(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> wasmtime::Result<Vec<String>> {
        todo!()
    }

    fn values(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> wasmtime::Result<Vec<webgpu::GpuPipelineConstantValue>> {
        todo!()
    }

    // fn entries(&mut self, _record: Resource<webgpu::RecordGpuPipelineConstantValue>) -> Vec<(String, webgpu::GpuPipelineConstantValue)> {
    fn entries(
        &mut self,
        _record: Resource<webgpu::RecordGpuPipelineConstantValue>,
    ) -> wasmtime::Result<Vec<(String, webgpu::GpuPipelineConstantValue)>> {
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
    fn vertex(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ShaderStages::VERTEX.bits())
    }

    fn fragment(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ShaderStages::FRAGMENT.bits())
    }

    fn compute(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::ShaderStages::COMPUTE.bits())
    }

    fn drop(&mut self, _: Resource<webgpu::GpuShaderStage>) -> wasmtime::Result<()> {
        unreachable!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuTextureUsage for WasiWebGpuImpl<T> {
    fn copy_src(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::TextureUsages::COPY_SRC.bits())
    }
    fn copy_dst(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::TextureUsages::COPY_DST.bits())
    }
    fn texture_binding(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::TextureUsages::TEXTURE_BINDING.bits())
    }
    fn storage_binding(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::TextureUsages::STORAGE_BINDING.bits())
    }
    fn render_attachment(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::TextureUsages::RENDER_ATTACHMENT.bits())
    }
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<webgpu::GpuTextureUsage>,
    ) -> wasmtime::Result<()> {
        unreachable!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuMapMode for WasiWebGpuImpl<T> {
    fn read(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        // https://www.w3.org/TR/webgpu/#buffer-mapping
        Ok(0x0001)
    }
    fn write(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        // https://www.w3.org/TR/webgpu/#buffer-mapping
        Ok(0x0002)
    }
    fn drop(&mut self, _rep: Resource<webgpu::GpuMapMode>) -> wasmtime::Result<()> {
        unreachable!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBufferUsage for WasiWebGpuImpl<T> {
    fn map_read(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::MAP_READ.bits())
    }
    fn map_write(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::MAP_WRITE.bits())
    }
    fn copy_src(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::COPY_SRC.bits())
    }
    fn copy_dst(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::COPY_DST.bits())
    }
    fn index(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::INDEX.bits())
    }
    fn vertex(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::VERTEX.bits())
    }
    fn uniform(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::UNIFORM.bits())
    }
    fn storage(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::STORAGE.bits())
    }
    fn indirect(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::INDIRECT.bits())
    }
    fn query_resolve(&mut self) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        Ok(wgpu_types::BufferUsages::QUERY_RESOLVE.bits())
    }
    fn drop(&mut self, _rep: Resource<webgpu::GpuBufferUsage>) -> wasmtime::Result<()> {
        unreachable!()
    }
}

impl<T: WasiWebGpuView> webgpu::HostRecordOptionGpuSize64 for WasiWebGpuImpl<T> {
    fn new(&mut self) -> wasmtime::Result<Resource<webgpu::RecordOptionGpuSize64>> {
        let record = std::collections::HashMap::new();
        Ok(self.table().push(record)?)
    }
    fn add(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
        key: String,
        value: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let record = self.table().get_mut(&record)?;
        record.insert(key, value);
        Ok(())
    }
    fn get(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
        key: String,
    ) -> wasmtime::Result<Option<Option<webgpu::GpuSize64>>> {
        let record = self.table().get(&record)?;
        Ok(record.get(&key).copied())
    }
    fn has(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
        key: String,
    ) -> wasmtime::Result<bool> {
        let record = self.table().get(&record)?;
        Ok(record.contains_key(&key))
    }
    fn remove(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
        key: String,
    ) -> wasmtime::Result<()> {
        let record = self.table().get_mut(&record)?;
        record.remove(&key);
        Ok(())
    }
    fn keys(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
    ) -> wasmtime::Result<Vec<String>> {
        let record = self.table().get(&record)?;
        Ok(record.keys().cloned().collect())
    }
    fn values(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
    ) -> wasmtime::Result<Vec<Option<webgpu::GpuSize64>>> {
        let record = self.table().get(&record)?;
        Ok(record.values().cloned().collect())
    }
    fn entries(
        &mut self,
        record: Resource<webgpu::RecordOptionGpuSize64>,
    ) -> wasmtime::Result<Vec<(String, Option<webgpu::GpuSize64>)>> {
        let record = self.table().get(&record)?;
        Ok(record.iter().map(|(k, v)| (k.clone(), *v)).collect())
    }
    fn drop(
        &mut self,
        record: wasmtime::component::Resource<webgpu::RecordOptionGpuSize64>,
    ) -> wasmtime::Result<()> {
        self.table().delete(record)?;
        Ok(())
    }
}

// impl<T: WasiWebGpuView> webgpu::HostNonStandardBuffer for WasiWebGpuImpl<T> {
//     fn get(&mut self, buffer: Resource<webgpu::NonStandardBuffer>) -> Vec<u8> {
//         let buffer = self.table().get_mut(&buffer)?;
//         buffer.slice_mut().to_vec()
//     }

//     fn set(&mut self, buffer: Resource<webgpu::NonStandardBuffer>, val: Vec<u8>) {
//         let buffer = self.table().get_mut(&buffer)?;
//         buffer.slice_mut().copy_from_slice(&val);
//     }

//     fn drop(&mut self, buffer: Resource<webgpu::NonStandardBuffer>) -> wasmtime::Result<()> {
//         self.table().delete(buffer)?;
//         Ok(())
//     }
// }

impl<T: WasiWebGpuView> webgpu::HostGpuDevice for WasiWebGpuImpl<T> {
    fn connect_graphics_context(
        &mut self,
        device: Resource<Device>,
        context: Resource<Context>,
    ) -> wasmtime::Result<()> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);
        let _adapter_id = device.adapter;

        let instance = Arc::downgrade(&self.instance());
        let surface_creator = self.ui_thread_spawner();

        let context = self.table().get_mut(&context)?;

        let surface = WebGpuSurface {
            get_instance: {
                let instance = Weak::clone(&instance);
                move || instance.upgrade().unwrap()
            },
            create_surface: {
                let instance = Weak::clone(&instance);
                move |display: &Arc<dyn DisplayApi + Send + Sync>| {
                    let instance = instance.upgrade().unwrap();
                    let display = Arc::downgrade(display);
                    block_on(surface_creator.spawn(move || {
                        let display = display.upgrade().expect("display dropped");
                        unsafe {
                            instance
                                .instance_create_surface(
                                    Some(display.display_handle().unwrap().as_raw()),
                                    display.window_handle().unwrap().as_raw(),
                                    None,
                                )
                                .unwrap()
                        }
                    }))
                }
            },
            device_id,
            error_handler,
            _adapter_id,
            surface_id: None,
        };

        context.connect_draw_api(Box::new(surface));
        Ok(())
    }

    // fn configure(
    //     &mut self,
    //     _device: Resource<Device>,
    //     _descriptor: webgpu::GpuDeviceConfiguration,
    // ) {
    //     todo!()
    // }

    fn adapter_info(
        &mut self,
        device: Resource<Device>,
    ) -> wasmtime::Result<Resource<webgpu::GpuAdapterInfo>> {
        let adapter_id = self.table().get(&device)?.adapter;
        let info = self.instance().adapter_get_info(adapter_id);
        let info = self.table().push(info)?;
        Ok(info)
    }

    fn create_command_encoder(
        &mut self,
        device: Resource<Device>,
        descriptor: Option<webgpu::GpuCommandEncoderDescriptor>,
    ) -> wasmtime::Result<Resource<CommandEncoder>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (command_encoder_id, err) = self.instance().device_create_command_encoder(
            device_id,
            &descriptor
                .map(|d| d.to_core(self.table()))
                .unwrap_or(wgpu_types::CommandEncoderDescriptor::default()),
            None,
        );

        error_handler.handle_possible_error(err);

        let command_encoder = self.table().push(CommandEncoder {
            command_encoder_id,
            error_handler,
        })?;
        Ok(command_encoder)
    }

    fn create_shader_module(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuShaderModuleDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuShaderModule>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let code =
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Owned(descriptor.code.to_owned()));
        let (shader, err) = self.instance().device_create_shader_module(
            device_id,
            &descriptor.to_core(self.table()),
            code,
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(shader)?)
    }

    fn create_render_pipeline(
        &mut self,
        device: Resource<Device>,
        descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Resource<RenderPipeline>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (render_pipeline_id, err) = self.instance().device_create_render_pipeline(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        let render_pipeline = self.table().push(RenderPipeline {
            render_pipeline_id,
            error_handler,
        })?;
        Ok(render_pipeline)
    }

    fn queue(
        &mut self,
        device: Resource<Device>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::QueueId>> {
        let queue = self.table().get(&device)?.queue;
        Ok(self.table().push(queue)?)
    }

    fn features(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedFeatures>> {
        let device = self.table().get(&device)?.device;
        let features = self.instance().device_features(device);
        Ok(self.table().push(features)?)
    }

    fn limits(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedLimits>> {
        let device = self.table().get(&device)?.device;
        let limits = self.instance().device_limits(device);
        Ok(self.table().push(limits)?)
    }

    fn destroy(&mut self, device: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        let device_id = self.table().get(&device)?.device;
        self.instance().device_destroy(device_id);
        Ok(())
    }

    fn create_buffer(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBufferDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBuffer>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);
        let descriptor = descriptor.to_core(self.table());

        let size = descriptor.size;
        let usage = descriptor.usage;
        let map_state = match descriptor.mapped_at_creation {
            true => webgpu::GpuBufferMapState::Mapped,
            false => webgpu::GpuBufferMapState::Unmapped,
        };

        let (buffer_id, err) = self
            .instance()
            .device_create_buffer(device_id, &descriptor, None);

        error_handler.handle_possible_error(err);

        let buffer = Buffer {
            buffer_id,
            size,
            usage,
            map_state,
        };

        Ok(self.table().push(buffer)?)
    }

    fn create_texture(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuTextureDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuTexture>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (texture_id, err) = self.instance().device_create_texture(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(Texture {
            texture_id,
            error_handler,
        })?)
    }

    fn create_sampler(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: Option<webgpu::GpuSamplerDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSampler>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let descriptor = descriptor
            .map(|d| d.to_core(self.table()))
            // https://www.w3.org/TR/webgpu/#dictdef-gpusamplerdescriptor
            .unwrap_or_else(|| wgpu_core::resource::SamplerDescriptor {
                label: None,
                address_modes: [wgpu_types::AddressMode::ClampToEdge; 3],
                mag_filter: wgpu_types::FilterMode::Nearest,
                min_filter: wgpu_types::FilterMode::Nearest,
                mipmap_filter: wgpu_types::MipmapFilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 32.0,
                compare: None,
                // TODO: make sure that anisotropy_clamp actually corresponds to maxAnisotropy
                anisotropy_clamp: 1,
                // border_color is not present in WebGPU
                border_color: None,
            });

        let (sampler, err) = self
            .instance()
            .device_create_sampler(device_id, &descriptor, None);

        error_handler.handle_possible_error(err);

        Ok(self.table().push(sampler)?)
    }

    fn create_bind_group_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (bind_group_layout, err) = self.instance().device_create_bind_group_layout(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(bind_group_layout)?)
    }

    fn create_pipeline_layout(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuPipelineLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuPipelineLayout>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (pipeline_layout, err) = self.instance().device_create_pipeline_layout(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(pipeline_layout)?)
    }

    fn create_bind_group(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuBindGroupDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroup>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (bind_group, err) = self.instance().device_create_bind_group(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(bind_group)?)
    }

    fn create_compute_pipeline(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePipeline>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (compute_pipeline_id, err) = self.instance().device_create_compute_pipeline(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(self.table().push(ComputePipeline {
            compute_pipeline_id,
            error_handler,
        })?)
    }

    fn create_compute_pipeline_async(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> wasmtime::Result<Result<Resource<webgpu::GpuComputePipeline>, webgpu::CreatePipelineError>>
    {
        Ok(Ok(self.create_compute_pipeline(device, descriptor)?))
    }

    fn create_render_pipeline_async(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Result<Resource<webgpu::GpuRenderPipeline>, webgpu::CreatePipelineError>>
    {
        Ok(Ok(self.create_render_pipeline(device, descriptor)?))
    }

    fn create_render_bundle_encoder(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuRenderBundleEncoderDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderBundleEncoder>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);
        let render_bundle_encoder = wgpu_core::command::RenderBundleEncoder::new(
            &descriptor.to_core(self.table()),
            device_id,
        )?;
        let render_bundle_encoder =
            self.table()
                .push(RenderBundleEncoder::new(RenderBundleEncoderInner {
                    render_bundle_encoder,
                    error_handler,
                }))?;
        Ok(render_bundle_encoder)
    }

    fn create_query_set(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
        descriptor: webgpu::GpuQuerySetDescriptor,
    ) -> wasmtime::Result<Result<Resource<webgpu::GpuQuerySet>, webgpu::CreateQuerySetError>> {
        let device = self.table().get(&device)?;
        let device_id = device.device;
        let error_handler = Arc::clone(&device.error_handler);

        let (query_set, err) = self.instance().device_create_query_set(
            device_id,
            &descriptor.to_core(self.table()),
            None,
        );

        error_handler.handle_possible_error(err);

        Ok(Ok(self.table().push(query_set)?))
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
        device: Resource<webgpu::GpuDevice>,
        filter: webgpu::GpuErrorFilter,
    ) -> wasmtime::Result<()> {
        let device = self.table().get(&device)?;
        device.error_handler.push_scope(filter);
        Ok(())
    }

    fn pop_error_scope(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Result<Option<Resource<webgpu::GpuError>>, webgpu::PopErrorScopeError>>
    {
        let device = self.table().get(&device)?;
        let error = device.error_handler.pop_scope();

        let error = error.map(|error| error.map(|error| self.table().push(error).unwrap()));

        Ok(error)
    }

    fn onuncapturederror_subscribe(
        &mut self,
        device: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<poll::Pollable>> {
        let device = self.table().get_mut(&device)?;
        let receiver = device.error_handler.new_error_receiver();
        let listener = self.table().push(Listener::new(receiver, move |_data| {
            // Need a onuncapturederror_get to actually give the error to the user.
            // For now just dropping them.
        }))?;
        wasmtime_wasi_io::poll::subscribe(self.table(), listener)
    }

    fn drop(&mut self, device: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        let device = self.table().delete(device)?;
        self.instance().device_drop(device.device);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTexture for WasiWebGpuImpl<T> {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<AbstractBuffer>,
    ) -> wasmtime::Result<Resource<Texture>> {
        let host_buffer = self.table().delete(buffer)?;
        let host_buffer: Texture = host_buffer.inner_type();
        Ok(self.table().push(host_buffer)?)
    }

    fn create_view(
        &mut self,
        texture: Resource<Texture>,
        descriptor: Option<webgpu::GpuTextureViewDescriptor>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::TextureViewId>> {
        let texture = self.table().get(&texture)?;
        let texture_id = texture.texture_id;
        let error_handler = Arc::clone(&texture.error_handler);
        let (texture_view, err) = self.instance().texture_create_view(
            texture_id,
            &descriptor
                .map(|d| d.to_core(self.table()))
                .unwrap_or(wgpu_core::resource::TextureViewDescriptor::default()),
            None,
        );
        error_handler.handle_possible_error(err);
        Ok(self.table().push(texture_view)?)
    }

    fn destroy(&mut self, texture: Resource<webgpu::GpuTexture>) -> wasmtime::Result<()> {
        let texture = self.table().get(&texture)?.texture_id;
        self.instance().texture_destroy(texture);
        Ok(())
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

    fn drop(&mut self, texture: Resource<webgpu::GpuTexture>) -> wasmtime::Result<()> {
        let texture = self.table().delete(texture)?;
        self.instance().texture_drop(texture.texture_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuTextureView for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, view: Resource<wgpu_core::id::TextureViewId>) -> wasmtime::Result<()> {
        let view_id = self.table().delete(view)?;
        self.instance().texture_view_drop(view_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandBuffer for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuCommandBuffer>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, command_buffer: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        let command_buffer_id = self.table().delete(command_buffer)?;
        self.instance().command_buffer_drop(command_buffer_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuShaderModule for WasiWebGpuImpl<T> {
    fn get_compilation_info(
        &mut self,
        _self_: Resource<webgpu::GpuShaderModule>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCompilationInfo>> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuShaderModule>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, shader: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        let shader_id = self.table().delete(shader)?;
        self.instance().shader_module_drop(shader_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuRenderPipeline for WasiWebGpuImpl<T> {
    fn label(&mut self, _self_: Resource<RenderPipeline>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<RenderPipeline>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_bind_group_layout(
        &mut self,
        pipeline: Resource<RenderPipeline>,
        index: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        let pipeline = self.table().get(&pipeline)?;
        let pipeline_id = pipeline.render_pipeline_id;
        let error_handler = Arc::clone(&pipeline.error_handler);
        let (layout, err) =
            self.instance()
                .render_pipeline_get_bind_group_layout(pipeline_id, index, None);
        error_handler.handle_possible_error(err);
        Ok(self.table().push(layout)?)
    }

    fn drop(&mut self, pipeline: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        let pipeline = self.table().delete(pipeline)?;
        self.instance()
            .render_pipeline_drop(pipeline.render_pipeline_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuAdapter for WasiWebGpuImpl<T> {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
        descriptor: Option<webgpu::GpuDeviceDescriptor>,
    ) -> wasmtime::Result<Result<Resource<webgpu::GpuDevice>, webgpu::RequestDeviceError>> {
        let adapter_id = *self.table().get(&adapter)?;

        let device_queue_result = self.instance().adapter_request_device(
            adapter_id,
            &descriptor
                .map(|d| d.to_core(self.table()))
                .unwrap_or(wgpu_types::DeviceDescriptor::default()),
            None,
            None,
        );

        Ok(match device_queue_result {
            Ok((device_id, queue_id)) => {
                let device = self.table().push(Device {
                    device: device_id,
                    queue: queue_id,
                    adapter: adapter_id,
                    error_handler: Arc::new(ErrorHandler::default()),
                })?;
                Ok(device)
            }

            Err(err) => {
                let message = err.to_string();
                // https://www.w3.org/TR/webgpu/#dom-gpuadapter-requestdevice
                match err {
                    wgpu_core::instance::RequestDeviceError::LimitsExceeded(_) => {
                        // From the spec:
                        // > 1. If any of the following requirements are unmet:
                        // >  - The set of values in descriptor.requiredFeatures must be a subset of those in adapter.[[features]].
                        // > Then issue the following steps on contentTimeline and return:
                        // >  1. Reject promise with a TypeError.
                        Err(webgpu::RequestDeviceError {
                            kind: webgpu::RequestDeviceErrorKind::TypeError,
                            message,
                        })
                    }
                    wgpu_core::instance::RequestDeviceError::UnsupportedFeature(_) => {
                        // From the spec:
                        // > 2. All of the requirements in the following steps must be met.
                        // >  2. For each [key, value] in descriptor.requiredLimits for which value is not undefined:
                        // >   1. key must be the name of a member of supported limits.
                        // >   2. value must be no better than adapter.[[limits]][key].
                        // >   3. If key’s class is alignment, value must be a power of 2 less than 232.
                        // > 3. If any are unmet, issue the following steps on contentTimeline and return:
                        // >  1. Reject promise with an OperationError.
                        Err(webgpu::RequestDeviceError {
                            kind: webgpu::RequestDeviceErrorKind::OperationError,
                            message,
                        })
                    }
                    err => todo!("unhandled request device error: {:#?}", err),
                }
            }
        })
    }

    fn features(
        &mut self,
        adapter: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<wasmtime::component::Resource<webgpu::GpuSupportedFeatures>> {
        let adapter = *self.table().get(&adapter)?;
        let features = self.instance().adapter_features(adapter);
        Ok(self.table().push(features)?)
    }

    fn limits(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedLimits>> {
        let adapter = *self.table().get(&adapter)?;
        let limits = self.instance().adapter_limits(adapter);
        Ok(self.table().push(limits)?)
    }

    fn is_fallback_adapter(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn info(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<Resource<webgpu::GpuAdapterInfo>> {
        let adapter_id = *self.table().get(&adapter)?;
        let info = self.instance().adapter_get_info(adapter_id);
        Ok(self.table().push(info)?)
    }

    fn drop(&mut self, adapter: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        let adapter_id = self.table().delete(adapter)?;
        self.instance().adapter_drop(adapter_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuQueue for WasiWebGpuImpl<T> {
    fn submit(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) -> wasmtime::Result<()> {
        let command_buffers = val
            .into_iter()
            .map(|buffer| *self.table().get(&buffer).unwrap())
            .collect::<Vec<_>>();
        let queue = self.table().get(&queue).copied()?;
        self.instance()
            .queue_submit(queue, &command_buffers)
            .unwrap();
        Ok(())
    }

    fn on_submitted_work_done(
        &mut self,
        _self_: Resource<wgpu_core::id::QueueId>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn write_buffer_with_copy(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        buffer: Resource<webgpu::GpuBuffer>,
        buffer_offset: webgpu::GpuSize64,
        data: Vec<u8>,
        data_offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<Result<(), webgpu::WriteBufferError>> {
        let queue = *self.table().get(&queue)?;
        let buffer_id = self.table().get(&buffer)?.buffer_id;
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
            .queue_write_buffer(queue, buffer_id, buffer_offset, data)?;
        Ok(Ok(()))
    }

    fn write_texture_with_copy(
        &mut self,
        queue: Resource<wgpu_core::id::QueueId>,
        destination: webgpu::GpuTexelCopyTextureInfo,
        data: Vec<u8>,
        data_layout: webgpu::GpuTexelCopyBufferLayout,
        size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        let queue = *self.table().get(&queue)?;
        self.instance().queue_write_texture(
            queue,
            &destination.to_core(self.table()),
            &data,
            &data_layout.to_core(self.table()),
            &size.to_core(self.table()),
        )?;
        Ok(())
    }

    fn label(&mut self, _self_: Resource<wgpu_core::id::QueueId>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<wgpu_core::id::QueueId>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, queue: Resource<wgpu_core::id::QueueId>) -> wasmtime::Result<()> {
        let queue_id = self.table().delete(queue)?;
        self.instance().queue_drop(queue_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuCommandEncoder for WasiWebGpuImpl<T> {
    fn begin_render_pass(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        descriptor: webgpu::GpuRenderPassDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPassEncoder>> {
        let command_encoder = self.table().get(&command_encoder)?;
        let command_encoder_id = command_encoder.command_encoder_id;
        let error_handler = Arc::clone(&command_encoder.error_handler);
        let timestamp_writes = descriptor
            .timestamp_writes
            .map(|tw| tw.to_core(self.table()));
        // can't use to_core because depth_stencil_attachment is Option<&x>.
        let depth_stencil_attachment = descriptor
            .depth_stencil_attachment
            .map(|d| d.to_core(self.table()));
        let descriptor = wgpu_core::command::RenderPassDescriptor {
            label: descriptor.label.map(|l| l.into()),
            color_attachments: descriptor
                .color_attachments
                .into_iter()
                .map(|c| c.map(|c| c.to_core(self.table())))
                .collect::<Vec<_>>()
                .into(),
            depth_stencil_attachment: depth_stencil_attachment.as_ref(),
            timestamp_writes: timestamp_writes.as_ref(),
            occlusion_query_set: descriptor
                .occlusion_query_set
                .map(|oqs| oqs.to_core(self.table())),
            // multiview_mask is not present in WebGPU
            multiview_mask: None,
            // TODO: self.max_draw_count not used
        };
        let (render_pass, err) = self
            .instance()
            .command_encoder_begin_render_pass(command_encoder_id, &descriptor);

        error_handler.handle_possible_error(err);

        Ok(self.table().push(RenderPassEncoder::new(render_pass))?)
    }

    fn finish(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        descriptor: Option<webgpu::GpuCommandBufferDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        let command_encoder = self.table().get(&command_encoder)?;
        let command_encoder_id = command_encoder.command_encoder_id;
        let error_handler = Arc::clone(&command_encoder.error_handler);
        let (command_buffer, err) = self.instance().command_encoder_finish(
            command_encoder_id,
            &descriptor
                .map(|d| d.to_core(self.table()))
                .unwrap_or(wgpu_types::CommandBufferDescriptor::default()),
            None,
        );
        // dropping the label.
        // TODO: reconsider when implementing real labels.
        let err = err.map(|(_label, err)| err);
        error_handler.handle_possible_error(err);
        Ok(self.table().push(command_buffer)?)
    }

    fn begin_compute_pass(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        descriptor: Option<webgpu::GpuComputePassDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePassEncoder>> {
        let command_encoder = self.table().get(&command_encoder)?;
        let command_encoder_id = command_encoder.command_encoder_id;
        let error_handler = Arc::clone(&command_encoder.error_handler);
        let (compute_pass, err) = self.instance().command_encoder_begin_compute_pass(
            command_encoder_id,
            // can't use to_core because timestamp_writes is Option<&x>.
            &wgpu_core::command::ComputePassDescriptor {
                // TODO: can we get rid of the clone here?
                label: descriptor
                    .as_ref()
                    .and_then(|d| d.label.clone().map(|l| l.into())),
                timestamp_writes: descriptor
                    .and_then(|d| d.timestamp_writes.map(|tw| tw.to_core(self.table()))),
            },
        );
        error_handler.handle_possible_error(err);
        Ok(self.table().push(ComputePassEncoder::new(compute_pass))?)
    }

    fn copy_buffer_to_buffer(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        source: Resource<webgpu::GpuBuffer>,
        source_offset: webgpu::GpuSize64,
        destination: Resource<webgpu::GpuBuffer>,
        destination_offset: webgpu::GpuSize64,
        size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        let source = self.table().get(&source)?.buffer_id;
        let destination = self.table().get(&destination)?.buffer_id;
        self.instance().command_encoder_copy_buffer_to_buffer(
            command_encoder,
            source,
            source_offset,
            destination,
            destination_offset,
            Some(size),
        )?;
        Ok(())
    }

    fn copy_buffer_to_texture(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        source: webgpu::GpuTexelCopyBufferInfo,
        destination: webgpu::GpuTexelCopyTextureInfo,
        copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance().command_encoder_copy_buffer_to_texture(
            command_encoder,
            &source.to_core(self.table()),
            &destination.to_core(self.table()),
            &copy_size.to_core(self.table()),
        )?;
        Ok(())
    }

    fn copy_texture_to_buffer(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        source: webgpu::GpuTexelCopyTextureInfo,
        destination: webgpu::GpuTexelCopyBufferInfo,
        copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance().command_encoder_copy_texture_to_buffer(
            command_encoder,
            &source.to_core(self.table()),
            &destination.to_core(self.table()),
            &copy_size.to_core(self.table()),
        )?;
        Ok(())
    }

    fn copy_texture_to_texture(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        source: webgpu::GpuTexelCopyTextureInfo,
        destination: webgpu::GpuTexelCopyTextureInfo,
        copy_size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance().command_encoder_copy_texture_to_texture(
            command_encoder,
            &source.to_core(self.table()),
            &destination.to_core(self.table()),
            &copy_size.to_core(self.table()),
        )?;
        Ok(())
    }

    fn clear_buffer(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        buffer: Resource<webgpu::GpuBuffer>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let buffer_id = self.table().get(&buffer)?.buffer_id;
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        // https://www.w3.org/TR/webgpu/#gpucommandencoder
        self.instance().command_encoder_clear_buffer(
            command_encoder,
            buffer_id,
            offset.unwrap_or(0),
            size,
        )?;
        Ok(())
    }

    fn resolve_query_set(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        query_set: Resource<webgpu::GpuQuerySet>,
        first_query: webgpu::GpuSize32,
        query_count: webgpu::GpuSize32,
        destination: Resource<webgpu::GpuBuffer>,
        destination_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let query_set_id = *self.table().get(&query_set)?;
        let destination = self.table().get(&destination)?.buffer_id;
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance().command_encoder_resolve_query_set(
            command_encoder,
            query_set_id,
            first_query,
            query_count,
            destination,
            destination_offset,
        )?;
        Ok(())
    }

    fn label(&mut self, command_encoder: Resource<CommandEncoder>) -> wasmtime::Result<String> {
        let _command_encoder = self.table().get(&command_encoder)?;
        // TODO: return real label
        Ok(String::new())
    }

    fn set_label(
        &mut self,
        _self_: Resource<CommandEncoder>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        group_label: String,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance()
            .command_encoder_push_debug_group(command_encoder, &group_label)?;
        Ok(())
    }

    fn pop_debug_group(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance()
            .command_encoder_pop_debug_group(command_encoder)?;
        Ok(())
    }

    fn insert_debug_marker(
        &mut self,
        command_encoder: Resource<CommandEncoder>,
        marker_label: String,
    ) -> wasmtime::Result<()> {
        let command_encoder = self.table().get(&command_encoder)?.command_encoder_id;
        self.instance()
            .command_encoder_insert_debug_marker(command_encoder, &marker_label)?;
        Ok(())
    }

    fn drop(&mut self, command_encoder: Resource<CommandEncoder>) -> wasmtime::Result<()> {
        let command_encoder = self.table().delete(command_encoder)?;
        self.instance()
            .command_encoder_drop(command_encoder.command_encoder_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuRenderPassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) -> wasmtime::Result<()> {
        let pipeline_id = self.table().get(&pipeline)?.render_pipeline_id;
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_pipeline(render_pass, pipeline_id)?;
        Ok(())
    }

    fn draw(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        vertex_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_vertex: Option<webgpu::GpuSize32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        instance.render_pass_draw(
            render_pass,
            vertex_count,
            instance_count.unwrap_or(1),
            first_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        )?;
        Ok(())
    }

    fn end(&mut self, render_pass: Resource<RenderPassEncoder>) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let mut render_pass = render_pass.take().unwrap();
        instance.render_pass_end(&mut render_pass)?;
        Ok(())
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
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_viewport(
            render_pass,
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        )?;
        Ok(())
    }

    fn set_scissor_rect(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        x: webgpu::GpuIntegerCoordinate,
        y: webgpu::GpuIntegerCoordinate,
        width: webgpu::GpuIntegerCoordinate,
        height: webgpu::GpuIntegerCoordinate,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_scissor_rect(render_pass, x, y, width, height)?;
        Ok(())
    }

    fn set_blend_constant(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        color: webgpu::GpuColor,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_blend_constant(render_pass, color.into())?;
        Ok(())
    }

    fn set_stencil_reference(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        reference: webgpu::GpuStencilValue,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_stencil_reference(render_pass, reference)?;
        Ok(())
    }

    fn begin_occlusion_query(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        query_index: webgpu::GpuSize32,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_begin_occlusion_query(render_pass, query_index)?;
        Ok(())
    }

    fn end_occlusion_query(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_end_occlusion_query(render_pass)?;
        Ok(())
    }

    fn execute_bundles(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        bundles: Vec<Resource<webgpu::GpuRenderBundle>>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let render_bundle_ids = bundles
            .iter()
            .map(|bundle| *self.table().get(bundle).unwrap())
            .collect::<Vec<_>>();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_execute_bundles(render_pass, &render_bundle_ids)?;
        Ok(())
    }

    fn label(&mut self, _self_: Resource<RenderPassEncoder>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<RenderPassEncoder>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn push_debug_group(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        group_label: String,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_push_debug_group(render_pass, &group_label, 0)?;
        Ok(())
    }

    fn pop_debug_group(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_pop_debug_group(render_pass)?;
        Ok(())
    }

    fn insert_debug_marker(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        marker_label: String,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_insert_debug_marker(render_pass, &marker_label, 0)?;
        Ok(())
    }

    fn set_bind_group(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets_data: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
        dynamic_offsets_data_start: Option<webgpu::GpuSize64>,
        dynamic_offsets_data_length: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<Result<(), webgpu::SetBindGroupError>> {
        let instance = self.instance();
        let bind_group = bind_group.map(|bind_group| *self.table().get(&bind_group).unwrap());
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#programmable-passes
        let dynamic_offsets = &dynamic_offsets_data.unwrap_or(vec![]);
        let mut dynamic_offsets = &dynamic_offsets[..];
        if let Some(dynamic_offsets_data_start) = dynamic_offsets_data_start {
            let dynamic_offsets_data_start = dynamic_offsets_data_start as usize;
            dynamic_offsets = &dynamic_offsets[dynamic_offsets_data_start..];
        }
        if let Some(dynamic_offsets_data_length) = dynamic_offsets_data_length {
            let dynamic_offsets_data_length = dynamic_offsets_data_length as usize;
            dynamic_offsets = &dynamic_offsets[..dynamic_offsets_data_length];
        }

        instance.render_pass_set_bind_group(render_pass, index, bind_group, dynamic_offsets)?;
        Ok(Ok(()))
    }

    fn set_index_buffer(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        buffer: Resource<webgpu::GpuBuffer>,
        index_format: webgpu::GpuIndexFormat,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let buffer_id = self.table().get(&buffer)?.buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_index_buffer(
            render_pass,
            buffer_id,
            index_format.into(),
            // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
            offset.unwrap_or(0),
            size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
        )?;
        Ok(())
    }

    fn set_vertex_buffer(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        slot: webgpu::GpuIndex32,
        buffer: Option<Resource<webgpu::GpuBuffer>>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let buffer_id = self
            .table()
            .get(&buffer.expect("TODO: deal null buffers"))?
            .buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_set_vertex_buffer(
            render_pass,
            slot,
            buffer_id,
            // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
            offset.unwrap_or(0),
            size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
        )?;
        Ok(())
    }

    fn draw_indexed(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        index_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_index: Option<webgpu::GpuSize32>,
        base_vertex: Option<webgpu::GpuSignedOffset32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_draw_indexed(
            render_pass,
            index_count,
            // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
            instance_count.unwrap_or(1),
            first_index.unwrap_or(0),
            base_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        )?;
        Ok(())
    }

    fn draw_indirect(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let indirect_buffer = self.table().get(&indirect_buffer)?.buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_draw_indirect(render_pass, indirect_buffer, indirect_offset)?;
        Ok(())
    }

    fn draw_indexed_indirect(
        &mut self,
        render_pass: Resource<RenderPassEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let indirect_buffer = self.table().get(&indirect_buffer)?.buffer_id;
        let mut render_pass = self.table().get_mut(&render_pass)?.lock();
        let render_pass = render_pass.as_mut().unwrap();
        instance.render_pass_draw_indexed_indirect(
            render_pass,
            indirect_buffer,
            indirect_offset,
        )?;
        Ok(())
    }

    fn drop(&mut self, render_pass: Resource<RenderPassEncoder>) -> wasmtime::Result<()> {
        self.table().delete(render_pass)?;
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuUncapturedErrorEvent for WasiWebGpuImpl<T> {
    // fn new(
    //     &mut self,
    //     _type_: String,
    //     _gpu_uncaptured_error_event_init_dict: webgpu::GpuUncapturedErrorEventInit,
    // ) -> Resource<webgpu::GpuUncapturedErrorEvent> {
    //     todo!()
    // }

    fn error(
        &mut self,
        _self_: Resource<webgpu::GpuUncapturedErrorEvent>,
    ) -> wasmtime::Result<Resource<webgpu::GpuError>> {
        todo!()
    }

    fn drop(&mut self, _error: Resource<webgpu::GpuUncapturedErrorEvent>) -> wasmtime::Result<()> {
        todo!()
    }
}
// impl<T: WasiWebGpuView> webgpu::HostGpuInternalError for WasiWebGpuImpl<T> {
//     fn new(&mut self, _message: String) -> Resource<webgpu::GpuInternalError> {
//         todo!()
//     }

//     fn message(&mut self, _self_: Resource<webgpu::GpuInternalError>) -> String {
//         todo!()
//     }

//     fn drop(&mut self, error: Resource<webgpu::GpuInternalError>) -> wasmtime::Result<()> {
//         self.table().delete(error)?;
//         Ok(())
//     }
// }
// impl<T: WasiWebGpuView> webgpu::HostGpuOutOfMemoryError for WasiWebGpuImpl<T> {
//     fn new(&mut self, _message: String) -> Resource<webgpu::GpuOutOfMemoryError> {
//         todo!()
//     }

//     fn message(&mut self, _self_: Resource<webgpu::GpuOutOfMemoryError>) -> String {
//         todo!()
//     }

//     fn drop(&mut self, error: Resource<webgpu::GpuOutOfMemoryError>) -> wasmtime::Result<()> {
//         self.table().delete(error)?;
//         Ok(())
//     }
// }
// impl<T: WasiWebGpuView> webgpu::HostGpuValidationError for WasiWebGpuImpl<T> {
//     fn new(&mut self, _message: String) -> Resource<webgpu::GpuValidationError> {
//         todo!()
//     }

//     fn message(&mut self, _self_: Resource<webgpu::GpuValidationError>) -> String {
//         todo!()
//     }

//     fn drop(&mut self, error: Resource<webgpu::GpuValidationError>) -> wasmtime::Result<()> {
//         self.table().delete(error)?;
//         Ok(())
//     }
// }
impl<T: WasiWebGpuView> webgpu::HostGpuError for WasiWebGpuImpl<T> {
    fn message(&mut self, error: Resource<webgpu::GpuError>) -> wasmtime::Result<String> {
        let error = self.table().get(&error)?;
        Ok(error.message.clone())
    }

    fn kind(
        &mut self,
        error: Resource<webgpu::GpuError>,
    ) -> wasmtime::Result<webgpu::GpuErrorKind> {
        let error = self.table().get(&error)?;
        Ok(error.kind)
    }

    fn drop(&mut self, _error: Resource<webgpu::GpuError>) -> wasmtime::Result<()> {
        self.table().delete(_error)?;
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuDeviceLostInfo for WasiWebGpuImpl<T> {
    fn reason(
        &mut self,
        _self_: Resource<webgpu::GpuDeviceLostInfo>,
    ) -> wasmtime::Result<webgpu::GpuDeviceLostReason> {
        todo!()
    }

    fn message(&mut self, _self_: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _info: Resource<webgpu::GpuDeviceLostInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCanvasContext for WasiWebGpuImpl<T> {
    fn configure(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
        _configuration: webgpu::GpuCanvasConfiguration,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_configuration(
        &mut self,
        _self_: Resource<webgpu::GpuCanvasContext>,
    ) -> wasmtime::Result<Option<webgpu::GpuCanvasConfigurationOwned>> {
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
impl<T: WasiWebGpuView> webgpu::HostGpuRenderBundle for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, _bundle: Resource<webgpu::GpuRenderBundle>) -> wasmtime::Result<()> {
        let bundle_id = self.table().delete(_bundle)?;
        self.instance().render_bundle_drop(bundle_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuComputePassEncoder for WasiWebGpuImpl<T> {
    fn set_pipeline(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        pipeline: Resource<webgpu::GpuComputePipeline>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let pipeline = self.table().get(&pipeline)?.compute_pipeline_id;
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_set_pipeline(compute_pass, pipeline)?;
        Ok(())
    }

    fn dispatch_workgroups(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        workgroup_count_x: webgpu::GpuSize32,
        workgroup_count_y: Option<webgpu::GpuSize32>,
        workgroup_count_z: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpucomputepassencoder
        instance.compute_pass_dispatch_workgroups(
            compute_pass,
            workgroup_count_x,
            workgroup_count_y.unwrap_or(1),
            workgroup_count_z.unwrap_or(1),
        )?;
        Ok(())
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let indirect_buffer = self.table().get(&indirect_buffer)?.buffer_id;
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_dispatch_workgroups_indirect(
            compute_pass,
            indirect_buffer,
            indirect_offset,
        )?;
        Ok(())
    }

    fn end(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let mut compute_pass = compute_pass.take().unwrap();
        instance.compute_pass_end(&mut compute_pass)?;
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
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        group_label: String,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_push_debug_group(compute_pass, &group_label, 0)?;
        Ok(())
    }

    fn pop_debug_group(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_pop_debug_group(compute_pass)?;
        Ok(())
    }

    fn insert_debug_marker(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        label: String,
    ) -> wasmtime::Result<()> {
        let instance = self.instance();
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        instance.compute_pass_insert_debug_marker(compute_pass, &label, 0)?;
        Ok(())
    }

    fn set_bind_group(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets_data: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
        dynamic_offsets_data_start: Option<webgpu::GpuSize64>,
        dynamic_offsets_data_length: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<Result<(), webgpu::SetBindGroupError>> {
        let instance = self.instance();
        let bind_group = bind_group.map(|bind_group| *self.table().get(&bind_group).unwrap());
        let mut compute_pass = self.table().get_mut(&compute_pass)?.lock();
        let compute_pass = compute_pass.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#programmable-passes

        let dynamic_offsets = &dynamic_offsets_data.unwrap_or(vec![]);
        let mut dynamic_offsets = &dynamic_offsets[..];
        if let Some(dynamic_offsets_data_start) = dynamic_offsets_data_start {
            let dynamic_offsets_data_start = dynamic_offsets_data_start as usize;
            dynamic_offsets = &dynamic_offsets[dynamic_offsets_data_start..];
        }
        if let Some(dynamic_offsets_data_length) = dynamic_offsets_data_length {
            let dynamic_offsets_data_length = dynamic_offsets_data_length as usize;
            dynamic_offsets = &dynamic_offsets[..dynamic_offsets_data_length];
        }

        instance.compute_pass_set_bind_group(compute_pass, index, bind_group, dynamic_offsets)?;
        Ok(Ok(()))
    }

    fn drop(
        &mut self,
        compute_pass: Resource<webgpu::GpuComputePassEncoder>,
    ) -> wasmtime::Result<()> {
        self.table().delete(compute_pass)?;
        Ok(())
    }
}
// impl<T: WasiWebGpuView> webgpu::HostGpuPipelineError for WasiWebGpuImpl<T> {
//     fn new(
//         &mut self,
//         _message: Option<String>,
//         _options: webgpu::GpuPipelineErrorInit,
//     ) -> Resource<webgpu::GpuPipelineError> {
//         todo!()
//     }

//     fn reason(
//         &mut self,
//         _self_: Resource<webgpu::GpuPipelineError>,
//     ) -> webgpu::GpuPipelineErrorReason {
//         todo!()
//     }

//     fn drop(&mut self, error: Resource<webgpu::GpuPipelineError>) -> wasmtime::Result<()> {
//         self.table().delete(error)?;
//         Ok(())
//     }
// }
impl<T: WasiWebGpuView> webgpu::HostGpuCompilationMessage for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, _cm: Resource<webgpu::GpuCompilationMessage>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuCompilationInfo for WasiWebGpuImpl<T> {
    fn messages(
        &mut self,
        _self_: Resource<webgpu::GpuCompilationInfo>,
    ) -> wasmtime::Result<Vec<Resource<webgpu::GpuCompilationMessage>>> {
        todo!()
    }

    fn drop(&mut self, _info: Resource<webgpu::GpuCompilationInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuQuerySet for WasiWebGpuImpl<T> {
    fn destroy(&mut self, _self_: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        // https://github.com/gfx-rs/wgpu/issues/6495
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

    fn drop(&mut self, _query_set: Resource<webgpu::GpuQuerySet>) -> wasmtime::Result<()> {
        let query_set_id = self.table().delete(_query_set)?;
        self.instance().query_set_drop(query_set_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuRenderBundleEncoder for WasiWebGpuImpl<T> {
    fn finish(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        descriptor: Option<webgpu::GpuRenderBundleDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderBundle>> {
        let instance = self.instance();
        let descriptor = descriptor
            .map(|d| d.to_core(self.table()))
            .unwrap_or(wgpu_types::RenderBundleDescriptor::default());
        let mut bundle_encoder_lock = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder_lock.take().unwrap();
        drop(bundle_encoder_lock);
        let (render_bundle, err) = instance.render_bundle_encoder_finish(
            bundle_encoder.render_bundle_encoder,
            &descriptor,
            None,
        );
        bundle_encoder.error_handler.handle_possible_error(err);
        Ok(self.table().push(render_bundle)?)
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
        _bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        _group_label: String,
    ) -> wasmtime::Result<()> {
        todo!("Debug markers for RenderBundleEncoder not yet implemented in wgpu")
    }

    fn pop_debug_group(
        &mut self,
        _bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
    ) -> wasmtime::Result<()> {
        todo!("Debug markers for RenderBundleEncoder not yet implemented in wgpu")
    }

    fn insert_debug_marker(
        &mut self,
        _bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        _marker_label: String,
    ) -> wasmtime::Result<()> {
        todo!("Debug markers for RenderBundleEncoder not yet implemented in wgpu")
    }

    fn set_bind_group(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        index: webgpu::GpuIndex32,
        bind_group: Option<Resource<webgpu::GpuBindGroup>>,
        dynamic_offsets_data: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
        dynamic_offsets_data_start: Option<webgpu::GpuSize64>,
        dynamic_offsets_data_length: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<Result<(), webgpu::SetBindGroupError>> {
        let bind_group_id = bind_group.map(|bind_group| *self.table().get(&bind_group).unwrap());
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();

        // https://www.w3.org/TR/webgpu/#programmable-passes
        let dynamic_offsets = &dynamic_offsets_data.unwrap_or(vec![]);
        let mut dynamic_offsets = &dynamic_offsets[..];
        if let Some(dynamic_offsets_data_start) = dynamic_offsets_data_start {
            let dynamic_offsets_data_start = dynamic_offsets_data_start as usize;
            dynamic_offsets = &dynamic_offsets[dynamic_offsets_data_start..];
        }
        if let Some(dynamic_offsets_data_length) = dynamic_offsets_data_length {
            let dynamic_offsets_data_length = dynamic_offsets_data_length as usize;
            dynamic_offsets = &dynamic_offsets[..dynamic_offsets_data_length];
        }

        unsafe {
            wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_bind_group(
                &mut bundle_encoder.render_bundle_encoder,
                index,
                bind_group_id,
                dynamic_offsets.as_ptr(),
                dynamic_offsets.len(),
            )
        };
        Ok(Ok(()))
    }

    fn set_pipeline(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        pipeline: Resource<RenderPipeline>,
    ) -> wasmtime::Result<()> {
        let pipeline = self.table().get(&pipeline)?;
        let pipeline_id = pipeline.render_pipeline_id;
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_pipeline(
            &mut bundle_encoder.render_bundle_encoder,
            pipeline_id,
        );
        Ok(())
    }

    fn set_index_buffer(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        buffer: Resource<webgpu::GpuBuffer>,
        index_format: webgpu::GpuIndexFormat,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let buffer_id = self.table().get(&buffer)?.buffer_id;
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_index_buffer(
            &mut bundle_encoder.render_bundle_encoder,
            buffer_id,
            index_format.into(),
            offset.unwrap_or(0),
            size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
        );
        Ok(())
    }

    fn set_vertex_buffer(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        slot: webgpu::GpuIndex32,
        buffer: Option<Resource<webgpu::GpuBuffer>>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        let buffer = buffer.expect("TODO: Null buffers not yet supported in wgpu");
        let buffer_id = self.table().get(&buffer)?.buffer_id;
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_set_vertex_buffer(
            &mut bundle_encoder.render_bundle_encoder,
            slot,
            buffer_id,
            offset.unwrap_or(0),
            size.map(|s| NonZeroU64::new(s).expect("Size can't be zero")),
        );
        Ok(())
    }

    fn draw(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        vertex_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_vertex: Option<webgpu::GpuSize32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw(
            &mut bundle_encoder.render_bundle_encoder,
            vertex_count,
            instance_count.unwrap_or(1),
            first_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        );
        Ok(())
    }

    fn draw_indexed(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        index_count: webgpu::GpuSize32,
        instance_count: Option<webgpu::GpuSize32>,
        first_index: Option<webgpu::GpuSize32>,
        base_vertex: Option<webgpu::GpuSignedOffset32>,
        first_instance: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        // https://www.w3.org/TR/webgpu/#gpurendercommandsmixin
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indexed(
            &mut bundle_encoder.render_bundle_encoder,
            index_count,
            instance_count.unwrap_or(1),
            first_index.unwrap_or(0),
            base_vertex.unwrap_or(0),
            first_instance.unwrap_or(0),
        );
        Ok(())
    }

    fn draw_indirect(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let indirect_buffer = self.table().get(&indirect_buffer)?.buffer_id;
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indirect(
            &mut bundle_encoder.render_bundle_encoder,
            indirect_buffer,
            indirect_offset,
        );
        Ok(())
    }

    fn draw_indexed_indirect(
        &mut self,
        bundle_encoder: Resource<webgpu::GpuRenderBundleEncoder>,
        indirect_buffer: Resource<webgpu::GpuBuffer>,
        indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        let indirect_buffer = self.table().get(&indirect_buffer)?.buffer_id;
        let mut bundle_encoder = self.table().get_mut(&bundle_encoder)?.lock();
        let bundle_encoder = bundle_encoder.as_mut().unwrap();
        wgpu_core::command::bundle_ffi::wgpu_render_bundle_draw_indexed_indirect(
            &mut bundle_encoder.render_bundle_encoder,
            indirect_buffer,
            indirect_offset,
        );
        Ok(())
    }

    fn drop(&mut self, encoder: Resource<webgpu::GpuRenderBundleEncoder>) -> wasmtime::Result<()> {
        self.table().delete(encoder)?;
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuComputePipeline for WasiWebGpuImpl<T> {
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
        let pipeline = self.table().get(&compute_pipeline)?;
        let pipeline_id = pipeline.compute_pipeline_id;
        let error_handler = Arc::clone(&pipeline.error_handler);
        let (bind_group_layout, err) =
            self.instance()
                .compute_pipeline_get_bind_group_layout(pipeline_id, index, None);
        error_handler.handle_possible_error(err);
        Ok(self.table().push(bind_group_layout)?)
    }

    fn drop(&mut self, pipeline: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<()> {
        let pipeline = self.table().delete(pipeline)?;
        self.instance()
            .compute_pipeline_drop(pipeline.compute_pipeline_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBindGroup for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, bind_group: Resource<webgpu::GpuBindGroup>) -> wasmtime::Result<()> {
        let bind_group_id = self.table().delete(bind_group)?;
        self.instance().bind_group_drop(bind_group_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuPipelineLayout for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, layout: Resource<webgpu::GpuPipelineLayout>) -> wasmtime::Result<()> {
        let layout_id = self.table().delete(layout)?;
        self.instance().pipeline_layout_drop(layout_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuBindGroupLayout for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, layout: Resource<webgpu::GpuBindGroupLayout>) -> wasmtime::Result<()> {
        let layout_id = self.table().delete(layout)?;
        self.instance().bind_group_layout_drop(layout_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuSampler for WasiWebGpuImpl<T> {
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

    fn drop(&mut self, sampler: Resource<webgpu::GpuSampler>) -> wasmtime::Result<()> {
        let sampler_id = self.table().delete(sampler)?;
        self.instance().sampler_drop(sampler_id);
        Ok(())
    }
}

impl<T: WasiWebGpuView> webgpu::HostGpuBuffer for WasiWebGpuImpl<T> {
    fn size(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuSize64Out> {
        let buffer = self.table().get(&buffer)?;
        Ok(buffer.size)
    }

    fn usage(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuFlagsConstant> {
        let buffer = self.table().get(&buffer)?;
        Ok(buffer.usage.bits())
    }

    fn map_state(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<webgpu::GpuBufferMapState> {
        let buffer = self.table().get(&buffer)?;
        Ok(buffer.map_state)
    }

    async fn map_async(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        mode: webgpu::GpuMapModeFlags,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> Result<Result<(), webgpu::MapAsyncError>, wasmtime::Error> {
        let instance = self.instance();
        let buffer = self.table().get_mut(&buffer)?;
        let buffer_id = buffer.buffer_id;
        buffer.map_state = webgpu::GpuBufferMapState::Pending;
        type Callback =
            Box<dyn FnOnce(Box<Result<(), wgpu_core::resource::BufferAccessError>>) + Send>;
        CallbackFuture::new(Box::new(move |resolve: Callback| {
            // TODO: move to convertion function
            // source: https://www.w3.org/TR/webgpu/#typedefdef-gpumapmodeflags
            let host = match mode {
                1 => wgpu_core::device::HostMap::Read,
                2 => wgpu_core::device::HostMap::Write,
                _ => panic!(),
            };
            let op = wgpu_core::resource::BufferMapOperation {
                host,
                callback: Some(Box::new(move |result| {
                    resolve(Box::new(result));
                })),
            };

            // https://www.w3.org/TR/webgpu/#gpubuffer
            let offset = offset.unwrap_or(0);
            instance
                .buffer_map_async(buffer_id, offset, size, op)
                .unwrap();
            // TODO: only poll this device.
            instance.poll_all_devices(true).unwrap();
        }))
        .await
        .unwrap();
        buffer.map_state = webgpu::GpuBufferMapState::Mapped;
        Ok(Ok(()))
    }

    fn get_mapped_range_get_with_copy(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<Result<Vec<u8>, webgpu::GetMappedRangeError>> {
        let buffer = self.table().get(&buffer)?;
        if buffer.map_state != webgpu::GpuBufferMapState::Mapped {
            todo!("Throw buffer not mapped error");
        }
        let buffer_id = buffer.buffer_id;
        let (ptr, len) = self
            .instance()
            // https://www.w3.org/TR/webgpu/#gpubuffer
            .buffer_get_mapped_range(buffer_id, offset.unwrap_or(0), size)?;
        let data = unsafe { slice::from_raw_parts(ptr.as_ptr(), len as usize) };
        let mut output = vec![0; len as usize];
        output.copy_from_slice(data);
        Ok(Ok(output))
    }

    fn get_mapped_range_set_with_copy(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
        data: Vec<u8>,
        offset: Option<webgpu::GpuSize64>,
        size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<Result<(), webgpu::GetMappedRangeError>> {
        let buffer = self.table().get(&buffer)?;
        if buffer.map_state != webgpu::GpuBufferMapState::Mapped {
            todo!("Throw buffer not mapped error");
        }
        let buffer_id = buffer.buffer_id;
        let (ptr, len) = self
            .instance()
            // https://www.w3.org/TR/webgpu/#gpubuffer
            .buffer_get_mapped_range(buffer_id, offset.unwrap_or(0), size)?;
        let buffer = unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), len as usize) };
        buffer.copy_from_slice(&data);
        Ok(Ok(()))
    }

    fn unmap(
        &mut self,
        buffer: Resource<webgpu::GpuBuffer>,
    ) -> wasmtime::Result<Result<(), webgpu::UnmapError>> {
        let instance = self.instance();
        let buffer = self.table().get_mut(&buffer)?;
        instance.buffer_unmap(buffer.buffer_id)?;
        buffer.map_state = webgpu::GpuBufferMapState::Unmapped;
        Ok(Ok(()))
    }

    fn destroy(&mut self, buffer: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        let buffer_id = self.table().get_mut(&buffer)?.buffer_id;
        self.instance().buffer_destroy(buffer_id);
        Ok(())
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

    fn drop(&mut self, buffer: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        let buffer = self.table().delete(buffer)?;
        self.instance().buffer_drop(buffer.buffer_id);
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpu for WasiWebGpuImpl<T> {
    fn request_adapter(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
        options: Option<webgpu::GpuRequestAdapterOptions>,
    ) -> wasmtime::Result<Option<Resource<wgpu_core::id::AdapterId>>> {
        let adapter = self.instance().request_adapter(
            &options
                .map(|o| o.to_core(self.table()))
                .unwrap_or(wgpu_types::RequestAdapterOptions::default()),
            wgpu_types::Backends::all(),
            None,
        );
        if let Err(wgpu_types::RequestAdapterError::NotFound { .. }) = &adapter {
            return Ok(None);
        }
        Ok(adapter.ok().map(|a| self.table().push(a).unwrap()))
    }

    fn get_preferred_canvas_format(
        &mut self,
        _gpu: Resource<webgpu::Gpu>,
    ) -> wasmtime::Result<webgpu::GpuTextureFormat> {
        Ok(PREFERRED_CANVAS_FORMAT)
    }

    fn wgsl_language_features(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
    ) -> wasmtime::Result<Resource<webgpu::WgslLanguageFeatures>> {
        todo!()
    }

    fn drop(&mut self, _gpu: Resource<webgpu::Gpu>) -> wasmtime::Result<()> {
        // not actually a resource in the table
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuAdapterInfo for WasiWebGpuImpl<T> {
    // TODO: more real values here
    // take ideas from https://bugzilla.mozilla.org/show_bug.cgi?id=1831994
    // keep an eye on https://github.com/gfx-rs/wgpu/issues/8649
    fn vendor(
        &mut self,
        adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        let adapter_info = self.table().get(&adapter_info)?;
        Ok(adapter_info.vendor.to_string())
    }

    fn architecture(
        &mut self,
        _adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        // TODO: implement real architecture
        Ok(String::new())
    }

    fn device(
        &mut self,
        adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        let adapter_info = self.table().get(&adapter_info)?;
        Ok(adapter_info.device.to_string())
    }

    fn description(
        &mut self,
        _adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<String> {
        // TODO: implement real description
        Ok(String::new())
    }

    fn subgroup_min_size(
        &mut self,
        adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<u32> {
        let adapter_info = self.table().get(&adapter_info)?;
        Ok(adapter_info.subgroup_min_size)
    }

    fn subgroup_max_size(
        &mut self,
        adapter_info: Resource<webgpu::GpuAdapterInfo>,
    ) -> wasmtime::Result<u32> {
        let adapter_info = self.table().get(&adapter_info)?;
        Ok(adapter_info.subgroup_max_size)
    }

    fn drop(&mut self, info: Resource<webgpu::GpuAdapterInfo>) -> wasmtime::Result<()> {
        self.table().delete(info)?;
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostWgslLanguageFeatures for WasiWebGpuImpl<T> {
    fn has(
        &mut self,
        _self_: Resource<webgpu::WgslLanguageFeatures>,
        _key: String,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn drop(&mut self, _features: Resource<webgpu::WgslLanguageFeatures>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedFeatures for WasiWebGpuImpl<T> {
    fn has(
        &mut self,
        features: Resource<webgpu::GpuSupportedFeatures>,
        query: String,
    ) -> wasmtime::Result<bool> {
        let features = self.table().get(&features)?;
        // TODO: disable the ones not present in the wgpu yet.
        Ok(match query.as_str() {
            // "core-features-and-limits" => {
            //     features.contains(wgpu_types::Features::CORE_FEATURES_AND_LIMITS)
            // }
            "depth-clip-control" => features.contains(wgpu_types::Features::DEPTH_CLIP_CONTROL),
            "depth32float-stencil8" => {
                features.contains(wgpu_types::Features::DEPTH32FLOAT_STENCIL8)
            }
            "texture-compression-bc" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_BC)
            }
            "texture-compression-bc-sliced-3d" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_BC_SLICED_3D)
            }
            "texture-compression-etc2" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_ETC2)
            }
            "texture-compression-astc" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_ASTC)
            }
            "texture-compression-astc-sliced-3d" => {
                features.contains(wgpu_types::Features::TEXTURE_COMPRESSION_ASTC_SLICED_3D)
            }
            "timestamp-query" => features.contains(wgpu_types::Features::TIMESTAMP_QUERY),
            "indirect-first-instance" => {
                features.contains(wgpu_types::Features::INDIRECT_FIRST_INSTANCE)
            }
            "shader-f16" => features.contains(wgpu_types::Features::SHADER_F16),
            "rg11b10ufloat-renderable" => {
                features.contains(wgpu_types::Features::RG11B10UFLOAT_RENDERABLE)
            }
            "bgra8unorm-storage" => features.contains(wgpu_types::Features::BGRA8UNORM_STORAGE),
            "float32-filterable" => features.contains(wgpu_types::Features::FLOAT32_FILTERABLE),
            // "float32-blendable" => {
            //     features.contains(wgpu_types::Features::FLOAT32_BLENDABLE)
            // }
            "clip-distances" => features.contains(wgpu_types::Features::CLIP_DISTANCES),
            "dual-source-blending" => features.contains(wgpu_types::Features::DUAL_SOURCE_BLENDING),
            // "subgroups" => {
            //     features.contains(wgpu_types::Features::SUBGROUPS)
            // }
            // "texture-formats-tier1" => {
            //     features.contains(wgpu_types::Features::TEXTURE_FORMATS_TIER1)
            // }
            // "texture-formats-tier2" => {
            //     features.contains(wgpu_types::Features::TEXTURE_FORMATS_TIER2)
            // }
            // "primitive-index" => {
            //     features.contains(wgpu_types::Features::PRIMITIVE_INDEX)
            // }
            // "texture-component-swizzle" => {
            //     features.contains(wgpu_types::Features::TEXTURE_COMPONENT_SWIZZLE)
            // }
            _ => false,
        })
    }

    fn drop(&mut self, features: Resource<webgpu::GpuSupportedFeatures>) -> wasmtime::Result<()> {
        self.table().delete(features)?;
        Ok(())
    }
}
impl<T: WasiWebGpuView> webgpu::HostGpuSupportedLimits for WasiWebGpuImpl<T> {
    fn max_texture_dimension1_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_texture_dimension_1d)
    }

    fn max_texture_dimension2_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_texture_dimension_2d)
    }

    fn max_texture_dimension3_d(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_texture_dimension_3d)
    }

    fn max_texture_array_layers(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_texture_array_layers)
    }

    fn max_bind_groups(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
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
        let limits = self.table().get(&limits)?;
        Ok(limits.max_bindings_per_bind_group)
    }

    fn max_dynamic_uniform_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_dynamic_uniform_buffers_per_pipeline_layout)
    }

    fn max_dynamic_storage_buffers_per_pipeline_layout(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_dynamic_storage_buffers_per_pipeline_layout)
    }

    fn max_sampled_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_sampled_textures_per_shader_stage)
    }

    fn max_samplers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_samplers_per_shader_stage)
    }

    fn max_storage_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_storage_buffers_per_shader_stage)
    }

    fn max_storage_textures_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_storage_textures_per_shader_stage)
    }

    fn max_uniform_buffers_per_shader_stage(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_uniform_buffers_per_shader_stage)
    }

    fn max_uniform_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_uniform_buffer_binding_size as u64)
    }

    fn max_storage_buffer_binding_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_storage_buffer_binding_size as u64)
    }

    fn min_uniform_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.min_uniform_buffer_offset_alignment)
    }

    fn min_storage_buffer_offset_alignment(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.min_storage_buffer_offset_alignment)
    }

    fn max_vertex_buffers(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_vertex_buffers)
    }

    fn max_buffer_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_buffer_size)
    }

    fn max_vertex_attributes(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_vertex_attributes)
    }

    fn max_vertex_buffer_array_stride(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_vertex_buffer_array_stride)
    }

    fn max_inter_stage_shader_variables(
        &mut self,
        _limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_color_attachments(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_color_attachments)
    }

    fn max_color_attachment_bytes_per_sample(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_color_attachment_bytes_per_sample)
    }

    fn max_compute_workgroup_storage_size(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_workgroup_storage_size)
    }

    fn max_compute_invocations_per_workgroup(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_invocations_per_workgroup)
    }

    fn max_compute_workgroup_size_x(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_workgroup_size_x)
    }

    fn max_compute_workgroup_size_y(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_workgroup_size_y)
    }

    fn max_compute_workgroup_size_z(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_workgroup_size_z)
    }

    fn max_compute_workgroups_per_dimension(
        &mut self,
        limits: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        let limits = self.table().get(&limits)?;
        Ok(limits.max_compute_workgroups_per_dimension)
    }

    fn drop(&mut self, limits: Resource<webgpu::GpuSupportedLimits>) -> wasmtime::Result<()> {
        self.table().delete(limits)?;
        Ok(())
    }
}
