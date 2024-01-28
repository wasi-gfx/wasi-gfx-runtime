use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::borrow::Cow;
use wasmtime::component::Resource;

use crate::component::webgpu::webgpu;
use crate::graphics_context::{GraphicsBuffer, GraphicsContext, GraphicsContextKind};
use crate::HostState;

pub struct Device {
    pub device: wgpu_core::id::DeviceId,
    // only needed when calling surface.get_capabilities in connect_graphics_context. If table would have a way to get parent from child, we could get it from device.
    pub adapter: Resource<wgpu_core::id::AdapterId>,
}

impl webgpu::Host for HostState {
    fn get_gpu(&mut self) -> wasmtime::Result<Resource<webgpu::Gpu>> {
        Ok(Resource::new_own(0))
    }
}

impl webgpu::HostGpuDevice for HostState {
    fn connect_graphics_context(
        &mut self,
        device: Resource<Device>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let surface = self.instance.instance_create_surface(
            self.window.raw_display_handle(),
            self.window.raw_window_handle(),
            (),
        );

        let host_daq = self.table.get(&device).unwrap();

        // think the table should have a way to get parent so that we can get adapter from device.
        let adapter = self.table.get(&host_daq.adapter).unwrap();

        let mut size = self.window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let swapchain_capabilities = self
            .instance
            .surface_get_capabilities::<crate::Backend>(surface, *adapter)
            .unwrap();
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu_types::SurfaceConfiguration {
            usage: wgpu_types::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu_types::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        self.instance
            .surface_configure::<crate::Backend>(surface, host_daq.device, &config);

        let context = self.table.get_mut(&context).unwrap();

        context.kind = Some(GraphicsContextKind::Webgpu(surface));

        Ok(())
    }

    fn create_command_encoder(
        &mut self,
        device: Resource<Device>,
        _descriptor: Option<webgpu::GpuCommandEncoderDescriptor>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::CommandEncoderId>> {
        let host_daq = self.table.get(&device).unwrap();

        let command_encoder = core_result(
            self.instance
                .device_create_command_encoder::<crate::Backend>(
                    host_daq.device,
                    &wgpu_types::CommandEncoderDescriptor { label: None },
                    (),
                ),
        )
        .unwrap();

        Ok(self.table.push_child(command_encoder, &device).unwrap())
    }

    fn create_shader_module(
        &mut self,
        device: Resource<Device>,
        desc: webgpu::GpuShaderModuleDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuShaderModule>> {
        let device = self.table.get(&device).unwrap();

        let shader = core_result(self.instance.device_create_shader_module::<crate::Backend>(
            device.device,
            &wgpu_core::pipeline::ShaderModuleDescriptor {
                label: desc.label.map(|label| label.into()),
                shader_bound_checks: Default::default(),
            },
            wgpu_core::pipeline::ShaderModuleSource::Wgsl(Cow::Owned(desc.code)),
            (),
        ))
        .unwrap();

        Ok(self.table.push(shader).unwrap())
    }

    fn create_render_pipeline(
        &mut self,
        device: Resource<Device>,
        props: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Resource<wgpu_core::id::RenderPipelineId>> {
        let vertex_module = self.table.get(&props.vertex.module).unwrap();
        let vertex = wgpu_core::pipeline::VertexState {
            stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                module: *vertex_module,
                entry_point: props.vertex.entry_point.into(),
            },
            buffers: Cow::Borrowed(&[]),
        };

        let fragment = props.fragment.map(|fragment| {
            let fragment_module = self.table.get(&fragment.module).unwrap();
            wgpu_core::pipeline::FragmentState {
                stage: wgpu_core::pipeline::ProgrammableStageDescriptor {
                    module: *fragment_module,
                    entry_point: fragment.entry_point.into(),
                },
                // targets: fragment
                //     .targets
                //     .iter()
                //     .map(|target| {
                //         Some(wgpu_types::ColorTargetState {
                //             format: (&target.format).into(),
                //             blend: None,
                //             write_mask: Default::default(),
                //         })
                //     })
                //     .collect::<Vec<_>>()
                //     .into(),
                targets: vec![Some(wgpu_types::ColorTargetState {
                    format: wgpu_types::TextureFormat::Bgra8UnormSrgb,
                    blend: None,
                    write_mask: Default::default(),
                })]
                .into(),
            }
        });

        let host_daq = self.table.get(&device).unwrap();

        let desc = &wgpu_core::pipeline::RenderPipelineDescriptor {
            vertex,
            fragment,
            primitive: wgpu_types::PrimitiveState::default(),
            depth_stencil: Default::default(),
            multisample: Default::default(),
            multiview: Default::default(),
            label: Default::default(),
            layout: Default::default(),
        };

        let implicit_pipeline_ids = match desc.layout {
            Some(_) => None,
            None => Some(wgpu_core::device::ImplicitPipelineIds {
                root_id: (),
                group_ids: &[(); wgpu_core::MAX_BIND_GROUPS],
            }),
        };
        let render_pipeline = core_result(
            self.instance
                .device_create_render_pipeline::<crate::Backend>(
                    host_daq.device,
                    desc,
                    (),
                    implicit_pipeline_ids,
                ),
        )
        .unwrap();

        Ok(self.table.push_child(render_pipeline, &device).unwrap())
    }

    fn queue(&mut self, device: Resource<Device>) -> wasmtime::Result<Resource<Device>> {
        Ok(Resource::new_own(device.rep()))
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn features(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedFeatures>> {
        todo!()
    }

    fn limits(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSupportedLimits>> {
        todo!()
    }

    fn destroy(&mut self, _self_: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        todo!()
    }

    fn create_buffer(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuBufferDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBuffer>> {
        todo!()
    }

    fn create_texture(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuTextureDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuTexture>> {
        todo!()
    }

    fn create_sampler(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: Option<webgpu::GpuSamplerDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuSampler>> {
        todo!()
    }

    fn import_external_texture(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuExternalTextureDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuExternalTexture>> {
        todo!()
    }

    fn create_bind_group_layout(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuBindGroupLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        todo!()
    }

    fn create_pipeline_layout(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuPipelineLayoutDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuPipelineLayout>> {
        todo!()
    }

    fn create_bind_group(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuBindGroupDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroup>> {
        todo!()
    }

    fn create_compute_pipeline(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuComputePipelineDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePipeline>> {
        todo!()
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
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuRenderBundleEncoderDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderBundleEncoder>> {
        todo!()
    }

    fn create_query_set(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _descriptor: webgpu::GpuQuerySetDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuQuerySet>> {
        todo!()
    }

    fn label(&mut self, _self_: Resource<webgpu::GpuDevice>) -> wasmtime::Result<String> {
        todo!()
    }

    fn set_label(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn lost(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDeviceLostInfo>> {
        todo!()
    }

    fn push_error_scope(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
        _filter: webgpu::GpuErrorFilter,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn pop_error_scope(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuError>> {
        todo!()
    }

    fn onuncapturederror(
        &mut self,
        _self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::EventHandler>> {
        todo!()
    }
}

impl webgpu::HostGpuTexture for HostState {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<GraphicsBuffer>,
    ) -> wasmtime::Result<Resource<crate::graphics_context::WebgpuTexture>> {
        let host_buffer = self.table.delete(buffer).unwrap();
        if let GraphicsBuffer::Webgpu(host_buffer) = host_buffer {
            Ok(self.table.push(host_buffer).unwrap())
        } else {
            panic!("Context not connected to webgpu");
        }
    }

    fn create_view(
        &mut self,
        texture: Resource<crate::graphics_context::WebgpuTexture>,
        _descriptor: Option<webgpu::GpuTextureViewDescriptor>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::TextureViewId>> {
        let texture_id = self.table.get(&texture).unwrap();
        let texture_view = core_result(self.instance.texture_create_view::<crate::Backend>(
            texture_id.texture,
            &Default::default(),
            (),
        ))
        .unwrap();
        Ok(self.table.push(texture_view).unwrap())
    }

    fn non_standard_present(
        &mut self,
        texture: Resource<crate::graphics_context::WebgpuTexture>,
    ) -> wasmtime::Result<()> {
        let texture = self.table.delete(texture).unwrap();

        self.instance
            .surface_present::<crate::Backend>(texture.surface)
            .unwrap();
        Ok(())
    }

    fn drop(
        &mut self,
        _rep: Resource<crate::graphics_context::WebgpuTexture>,
    ) -> wasmtime::Result<()> {
        todo!();
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

impl webgpu::HostGpuTextureView for HostState {
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

impl webgpu::HostGpuCommandBuffer for HostState {
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

impl webgpu::HostGpuShaderModule for HostState {
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

impl webgpu::HostGpuRenderPipeline for HostState {
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

impl webgpu::HostGpuAdapter for HostState {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
        _descriptor: Option<webgpu::GpuDeviceDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDevice>> {
        let adapter_id = self.table.get(&adapter).unwrap();

        let device_id = core_result(self.instance.adapter_request_device::<crate::Backend>(
            *adapter_id,
            &Default::default(),
            None,
            Default::default(),
        ))
        .unwrap();

        let daq = self
            .table
            .push_child(
                Device {
                    device: device_id,
                    adapter: Resource::new_own(adapter.rep()),
                },
                &adapter,
            )
            .unwrap();

        Ok(daq)
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
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<wasmtime::component::Resource<webgpu::GpuSupportedLimits>> {
        todo!()
    }

    fn is_fallback_adapter(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn request_adapter_info(
        &mut self,
        _self_: wasmtime::component::Resource<wgpu_core::id::AdapterId>,
    ) -> wasmtime::Result<wasmtime::component::Resource<webgpu::GpuAdapterInfo>> {
        todo!()
    }
}

impl webgpu::HostGpuQueue for HostState {
    fn submit(
        &mut self,
        daq: Resource<Device>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) -> wasmtime::Result<()> {
        let command_buffers = val
            .into_iter()
            .map(|buffer| self.table.delete(buffer).unwrap())
            .collect::<Vec<_>>();

        let daq = self.table.get(&daq).unwrap();
        self.instance
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
        _self_: Resource<Device>,
        _buffer: Resource<webgpu::GpuBuffer>,
        _buffer_offset: webgpu::GpuSize64,
        _data_offset: Option<webgpu::GpuSize64>,
        _data: Resource<webgpu::AllowSharedBufferSource>,
        _size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn write_texture(
        &mut self,
        _self_: Resource<Device>,
        _destination: webgpu::GpuImageCopyTexture,
        _data: Resource<webgpu::AllowSharedBufferSource>,
        _data_layout: webgpu::GpuImageDataLayout,
        _size: webgpu::GpuExtent3D,
    ) -> wasmtime::Result<()> {
        todo!()
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

impl webgpu::HostGpuCommandEncoder for HostState {
    fn begin_render_pass(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        descriptor: webgpu::GpuRenderPassDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPassEncoder>> {
        let command_encoder = self.table.get(&command_encoder).unwrap();
        let views = descriptor
            .color_attachments
            .iter()
            .map(|color_attachment| *self.table.get(&color_attachment.view).unwrap())
            .collect::<Vec<_>>();

        let mut color_attachments = vec![];
        for (i, _color_attachment) in descriptor.color_attachments.iter().enumerate() {
            color_attachments.push(Some(wgpu_core::command::RenderPassColorAttachment {
                view: views[i],
                // TODO: take from descriptor
                resolve_target: None,
                channel: wgpu_core::command::PassChannel {
                    load_op: wgpu_core::command::LoadOp::Clear,
                    store_op: wgpu_core::command::StoreOp::Store,
                    clear_value: wgpu_types::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.1,
                        a: 0.0,
                    },
                    read_only: false,
                },
            }));
        }

        let render_pass = wgpu_core::command::RenderPass::new(
            *command_encoder,
            &wgpu_core::command::RenderPassDescriptor {
                color_attachments: color_attachments.into(),
                label: None,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            },
        );

        Ok(self.table.push(render_pass).unwrap())
    }

    fn finish(
        &mut self,
        command_encoder: Resource<wgpu_core::id::CommandEncoderId>,
        _descriptor: Option<webgpu::GpuCommandBufferDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        let command_encoder = self.table.delete(command_encoder).unwrap();
        let command_buffer = core_result(
            self.instance
                .command_encoder_finish::<crate::Backend>(command_encoder, &Default::default()),
        )
        .unwrap();
        Ok(self.table.push(command_buffer).unwrap())
    }

    fn drop(&mut self, _rep: Resource<wgpu_core::id::CommandEncoderId>) -> wasmtime::Result<()> {
        Ok(())
    }

    fn begin_compute_pass(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _descriptor: Option<webgpu::GpuComputePassDescriptor>,
    ) -> wasmtime::Result<Resource<webgpu::GpuComputePassEncoder>> {
        todo!()
    }

    fn copy_buffer_to_buffer(
        &mut self,
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
        _source: Resource<webgpu::GpuBuffer>,
        _source_offset: webgpu::GpuSize64,
        _destination: Resource<webgpu::GpuBuffer>,
        _destination_offset: webgpu::GpuSize64,
        _size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
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
        _self_: Resource<wgpu_core::id::CommandEncoderId>,
    ) -> wasmtime::Result<String> {
        todo!()
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

impl webgpu::HostGpuRenderPassEncoder for HostState {
    fn set_pipeline(
        &mut self,
        render_pass: Resource<wgpu_core::command::RenderPass>,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) -> wasmtime::Result<()> {
        let pipeline = *self.table.get(&pipeline).unwrap();
        let cwr = self.table.get_mut(&render_pass).unwrap();
        wgpu_core::command::render_ffi::wgpu_render_pass_set_pipeline(cwr, pipeline);
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
        let cwr = self.table.get_mut(&cwr).unwrap();

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
        // use this instead of non_standard_present? Ask on ...

        let rpass = self.table.delete(rpass).unwrap();
        let encoder = self.table.get(&non_standard_encoder).unwrap();
        self.instance
            .command_encoder_run_render_pass::<crate::Backend>(*encoder, &rpass)
            .unwrap();
        Ok(())
    }

    fn drop(&mut self, cwr: Resource<wgpu_core::command::RenderPass>) -> wasmtime::Result<()> {
        self.table.delete(cwr).unwrap();
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
        _self_: Resource<wgpu_core::command::RenderPass>,
        _index: webgpu::GpuIndex32,
        _bind_group: Resource<webgpu::GpuBindGroup>,
        _dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) -> wasmtime::Result<()> {
        todo!()
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
        _self_: Resource<wgpu_core::command::RenderPass>,
        _slot: webgpu::GpuIndex32,
        _buffer: Resource<webgpu::GpuBuffer>,
        _offset: webgpu::GpuSize64,
        _size: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
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

impl webgpu::HostGpuUncapturedErrorEvent for HostState {
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
impl webgpu::HostGpuInternalError for HostState {
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
impl webgpu::HostGpuOutOfMemoryError for HostState {
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
impl webgpu::HostGpuValidationError for HostState {
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
impl webgpu::HostGpuError for HostState {
    fn message(&mut self, _self_: Resource<webgpu::GpuError>) -> wasmtime::Result<String> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuError>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostGpuDeviceLostInfo for HostState {
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
impl webgpu::HostGpuCanvasContext for HostState {
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
impl webgpu::HostGpuRenderBundle for HostState {
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
impl webgpu::HostGpuComputePassEncoder for HostState {
    fn set_pipeline(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _pipeline: Resource<webgpu::GpuComputePipeline>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn dispatch_workgroups(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _workgroup_count_x: webgpu::GpuSize32,
        _workgroup_count_y: Option<webgpu::GpuSize32>,
        _workgroup_count_z: Option<webgpu::GpuSize32>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn dispatch_workgroups_indirect(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _indirect_buffer: Resource<webgpu::GpuBuffer>,
        _indirect_offset: webgpu::GpuSize64,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn end(&mut self, _self_: Resource<webgpu::GpuComputePassEncoder>) -> wasmtime::Result<()> {
        todo!()
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
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _marker_label: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn set_bind_group(
        &mut self,
        _self_: Resource<webgpu::GpuComputePassEncoder>,
        _index: webgpu::GpuIndex32,
        _bind_group: Resource<webgpu::GpuBindGroup>,
        _dynamic_offsets: Option<Vec<webgpu::GpuBufferDynamicOffset>>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePassEncoder>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostGpuPipelineError for HostState {
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
impl webgpu::HostGpuCompilationMessage for HostState {
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
impl webgpu::HostGpuCompilationInfo for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuCompilationInfo>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostGpuQuerySet for HostState {
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
impl webgpu::HostGpuRenderBundleEncoder for HostState {
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
impl webgpu::HostGpuComputePipeline for HostState {
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
        _self_: Resource<webgpu::GpuComputePipeline>,
        _index: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuBindGroupLayout>> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuComputePipeline>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostGpuBindGroup for HostState {
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
        todo!()
    }
}
impl webgpu::HostGpuPipelineLayout for HostState {
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
        todo!()
    }
}
impl webgpu::HostGpuBindGroupLayout for HostState {
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
        todo!()
    }
}
impl webgpu::HostGpuExternalTexture for HostState {
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
impl webgpu::HostGpuSampler for HostState {
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
        todo!()
    }
}
impl webgpu::HostGpuBuffer for HostState {
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

    fn map_async(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
        _mode: webgpu::GpuMapModeFlags,
        _offset: Option<webgpu::GpuSize64>,
        _size: Option<webgpu::GpuSize64>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn get_mapped_range(
        &mut self,
        _self_: Resource<webgpu::GpuBuffer>,
        _offset: webgpu::GpuSize64,
        _size: webgpu::GpuSize64,
    ) -> wasmtime::Result<Resource<webgpu::ArrayBuffer>> {
        todo!()
    }

    fn unmap(&mut self, _self_: Resource<webgpu::GpuBuffer>) -> wasmtime::Result<()> {
        todo!()
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
        todo!()
    }
}
impl webgpu::HostGpu for HostState {
    fn request_adapter(
        &mut self,
        _self_: Resource<webgpu::Gpu>,
        _options: Option<webgpu::GpuRequestAdapterOptions>,
    ) -> wasmtime::Result<Resource<wgpu_core::id::AdapterId>> {
        let adapter = self
            .instance
            .request_adapter(
                &Default::default(),
                wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| ()),
            )
            .unwrap();
        Ok(self.table.push(adapter).unwrap())
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
        todo!()
    }
}
impl webgpu::HostGpuAdapterInfo for HostState {
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
        todo!()
    }
}
impl webgpu::HostWgslLanguageFeatures for HostState {
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
impl webgpu::HostGpuSupportedFeatures for HostState {
    fn has(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedFeatures>,
        _key: String,
    ) -> wasmtime::Result<bool> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedFeatures>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostGpuSupportedLimits for HostState {
    fn max_texture_dimension1_d(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_texture_dimension2_d(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_texture_dimension3_d(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_texture_array_layers(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_bind_groups(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_bind_groups_plus_vertex_buffers(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_bindings_per_bind_group(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_dynamic_uniform_buffers_per_pipeline_layout(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_dynamic_storage_buffers_per_pipeline_layout(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_sampled_textures_per_shader_stage(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_samplers_per_shader_stage(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_storage_buffers_per_shader_stage(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_storage_textures_per_shader_stage(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_uniform_buffers_per_shader_stage(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_uniform_buffer_binding_size(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        todo!()
    }

    fn max_storage_buffer_binding_size(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        todo!()
    }

    fn min_uniform_buffer_offset_alignment(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn min_storage_buffer_offset_alignment(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_vertex_buffers(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_buffer_size(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u64> {
        todo!()
    }

    fn max_vertex_attributes(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_vertex_buffer_array_stride(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_inter_stage_shader_components(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_inter_stage_shader_variables(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_color_attachments(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_color_attachment_bytes_per_sample(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroup_storage_size(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_invocations_per_workgroup(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroup_size_x(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroup_size_y(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroup_size_z(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn max_compute_workgroups_per_dimension(
        &mut self,
        _self_: Resource<webgpu::GpuSupportedLimits>,
    ) -> wasmtime::Result<u32> {
        todo!()
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuSupportedLimits>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostAllowSharedBufferSource for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::AllowSharedBufferSource>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostPredefinedColorSpace for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::PredefinedColorSpace>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostEventHandler for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::EventHandler>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostOffscreenCanvas for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::OffscreenCanvas>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostHtmlCanvasElement for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlCanvasElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostVideoFrame for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::VideoFrame>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostHtmlVideoElement for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlVideoElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostHtmlImageElement for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::HtmlImageElement>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostImageData for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::ImageData>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostImageBitmap for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::ImageBitmap>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostArrayBuffer for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::ArrayBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
impl webgpu::HostUint32Array for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::Uint32Array>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl From<&wgpu_types::TextureFormat> for webgpu::GpuTextureFormat {
    fn from(value: &wgpu_types::TextureFormat) -> Self {
        match value {
            wgpu_types::TextureFormat::Bgra8UnormSrgb => webgpu::GpuTextureFormat::Bgra8unormSrgb,
            _ => todo!(),
        }
    }
}
impl From<&webgpu::GpuTextureFormat> for wgpu_types::TextureFormat {
    fn from(value: &webgpu::GpuTextureFormat) -> Self {
        match value {
            webgpu::GpuTextureFormat::Bgra8unormSrgb => wgpu_types::TextureFormat::Bgra8UnormSrgb,
            webgpu::GpuTextureFormat::R8unorm => wgpu_types::TextureFormat::R8Unorm,
            webgpu::GpuTextureFormat::R8snorm => wgpu_types::TextureFormat::R8Snorm,
            webgpu::GpuTextureFormat::R8uint => wgpu_types::TextureFormat::R8Uint,
            webgpu::GpuTextureFormat::R8sint => wgpu_types::TextureFormat::R8Sint,
            webgpu::GpuTextureFormat::R16uint => wgpu_types::TextureFormat::R16Uint,
            webgpu::GpuTextureFormat::R16sint => wgpu_types::TextureFormat::R16Sint,
            webgpu::GpuTextureFormat::R16float => wgpu_types::TextureFormat::R16Float,
            webgpu::GpuTextureFormat::Rg8unorm => wgpu_types::TextureFormat::Rg8Unorm,
            webgpu::GpuTextureFormat::Rg8snorm => wgpu_types::TextureFormat::Rg8Snorm,
            webgpu::GpuTextureFormat::Rg8uint => wgpu_types::TextureFormat::Rg8Uint,
            webgpu::GpuTextureFormat::Rg8sint => wgpu_types::TextureFormat::Rg8Sint,
            webgpu::GpuTextureFormat::R32uint => wgpu_types::TextureFormat::R32Uint,
            webgpu::GpuTextureFormat::R32sint => wgpu_types::TextureFormat::R32Sint,
            webgpu::GpuTextureFormat::R32float => wgpu_types::TextureFormat::R32Float,
            webgpu::GpuTextureFormat::Rg16uint => wgpu_types::TextureFormat::Rg16Uint,
            webgpu::GpuTextureFormat::Rg16sint => wgpu_types::TextureFormat::Rg16Sint,
            webgpu::GpuTextureFormat::Rg16float => wgpu_types::TextureFormat::Rg16Float,
            webgpu::GpuTextureFormat::Rgba8unorm => wgpu_types::TextureFormat::Rgba8Unorm,
            webgpu::GpuTextureFormat::Rgba8unormSrgb => wgpu_types::TextureFormat::Rgba8UnormSrgb,
            webgpu::GpuTextureFormat::Rgba8snorm => wgpu_types::TextureFormat::Rgba8Snorm,
            webgpu::GpuTextureFormat::Rgba8uint => wgpu_types::TextureFormat::Rgba8Uint,
            webgpu::GpuTextureFormat::Rgba8sint => wgpu_types::TextureFormat::Rgba8Sint,
            webgpu::GpuTextureFormat::Bgra8unorm => wgpu_types::TextureFormat::Bgra8Unorm,
            webgpu::GpuTextureFormat::Rgb9e5ufloat => wgpu_types::TextureFormat::Rgb9e5Ufloat,
            webgpu::GpuTextureFormat::Rgb10a2uint => wgpu_types::TextureFormat::Rgb10a2Uint,
            webgpu::GpuTextureFormat::Rgb10a2unorm => wgpu_types::TextureFormat::Rgb10a2Unorm,
            webgpu::GpuTextureFormat::Rg11b10ufloat => wgpu_types::TextureFormat::Rg11b10Float,
            webgpu::GpuTextureFormat::Rg32uint => wgpu_types::TextureFormat::Rg32Uint,
            webgpu::GpuTextureFormat::Rg32sint => wgpu_types::TextureFormat::Rg32Sint,
            webgpu::GpuTextureFormat::Rg32float => wgpu_types::TextureFormat::Rg32Float,
            webgpu::GpuTextureFormat::Rgba16uint => wgpu_types::TextureFormat::Rgba16Uint,
            webgpu::GpuTextureFormat::Rgba16sint => wgpu_types::TextureFormat::Rgba16Sint,
            webgpu::GpuTextureFormat::Rgba16float => wgpu_types::TextureFormat::Rgba16Float,
            webgpu::GpuTextureFormat::Rgba32uint => wgpu_types::TextureFormat::Rgba32Uint,
            webgpu::GpuTextureFormat::Rgba32sint => wgpu_types::TextureFormat::Rgba32Sint,
            webgpu::GpuTextureFormat::Rgba32float => wgpu_types::TextureFormat::Rgba32Float,
            webgpu::GpuTextureFormat::Stencil8 => wgpu_types::TextureFormat::Stencil8,
            webgpu::GpuTextureFormat::Depth16unorm => wgpu_types::TextureFormat::Depth16Unorm,
            webgpu::GpuTextureFormat::Depth24plus => wgpu_types::TextureFormat::Depth24Plus,
            webgpu::GpuTextureFormat::Depth24plusStencil8 => {
                wgpu_types::TextureFormat::Depth24PlusStencil8
            }
            webgpu::GpuTextureFormat::Depth32float => wgpu_types::TextureFormat::Depth32Float,
            webgpu::GpuTextureFormat::Depth32floatStencil8 => {
                wgpu_types::TextureFormat::Depth32FloatStencil8
            }
            webgpu::GpuTextureFormat::Bc1RgbaUnorm => wgpu_types::TextureFormat::Bc1RgbaUnorm,
            webgpu::GpuTextureFormat::Bc1RgbaUnormSrgb => {
                wgpu_types::TextureFormat::Bc1RgbaUnormSrgb
            }
            webgpu::GpuTextureFormat::Bc2RgbaUnorm => wgpu_types::TextureFormat::Bc2RgbaUnorm,
            webgpu::GpuTextureFormat::Bc2RgbaUnormSrgb => {
                wgpu_types::TextureFormat::Bc2RgbaUnormSrgb
            }
            webgpu::GpuTextureFormat::Bc3RgbaUnorm => wgpu_types::TextureFormat::Bc3RgbaUnorm,
            webgpu::GpuTextureFormat::Bc3RgbaUnormSrgb => {
                wgpu_types::TextureFormat::Bc3RgbaUnormSrgb
            }
            webgpu::GpuTextureFormat::Bc4RUnorm => wgpu_types::TextureFormat::Bc4RUnorm,
            webgpu::GpuTextureFormat::Bc4RSnorm => wgpu_types::TextureFormat::Bc4RSnorm,
            webgpu::GpuTextureFormat::Bc5RgUnorm => wgpu_types::TextureFormat::Bc5RgUnorm,
            webgpu::GpuTextureFormat::Bc5RgSnorm => wgpu_types::TextureFormat::Bc5RgSnorm,
            webgpu::GpuTextureFormat::Bc6hRgbUfloat => wgpu_types::TextureFormat::Bc6hRgbUfloat,
            webgpu::GpuTextureFormat::Bc6hRgbFloat => wgpu_types::TextureFormat::Bc6hRgbFloat,
            webgpu::GpuTextureFormat::Bc7RgbaUnorm => wgpu_types::TextureFormat::Bc7RgbaUnorm,
            webgpu::GpuTextureFormat::Bc7RgbaUnormSrgb => {
                wgpu_types::TextureFormat::Bc7RgbaUnormSrgb
            }
            webgpu::GpuTextureFormat::Etc2Rgb8unorm => wgpu_types::TextureFormat::Etc2Rgb8Unorm,
            webgpu::GpuTextureFormat::Etc2Rgb8unormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgb8UnormSrgb
            }
            webgpu::GpuTextureFormat::Etc2Rgb8a1unorm => wgpu_types::TextureFormat::Etc2Rgb8A1Unorm,
            webgpu::GpuTextureFormat::Etc2Rgb8a1unormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgb8A1UnormSrgb
            }
            webgpu::GpuTextureFormat::Etc2Rgba8unorm => wgpu_types::TextureFormat::Etc2Rgba8Unorm,
            webgpu::GpuTextureFormat::Etc2Rgba8unormSrgb => {
                wgpu_types::TextureFormat::Etc2Rgba8UnormSrgb
            }
            webgpu::GpuTextureFormat::EacR11unorm => wgpu_types::TextureFormat::EacR11Unorm,
            webgpu::GpuTextureFormat::EacR11snorm => wgpu_types::TextureFormat::EacR11Snorm,
            webgpu::GpuTextureFormat::EacRg11unorm => wgpu_types::TextureFormat::EacRg11Unorm,
            webgpu::GpuTextureFormat::EacRg11snorm => wgpu_types::TextureFormat::EacRg11Snorm,
            webgpu::GpuTextureFormat::Astc4x4Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc4x4UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc5x4Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc5x4UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc5x5Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc5x5UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc6x5Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc6x5UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc6x6Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc6x6UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc8x5Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc8x5UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc8x6Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc8x6UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc8x8Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc8x8UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc10x5Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc10x5UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc10x6Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc10x6UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc10x8Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc10x8UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc10x10Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc10x10UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc12x10Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc12x10UnormSrgb => todo!(),
            webgpu::GpuTextureFormat::Astc12x12Unorm => todo!(),
            webgpu::GpuTextureFormat::Astc12x12UnormSrgb => todo!(),
        }
    }
}

impl From<&webgpu::GpuPrimitiveTopology> for wgpu_types::PrimitiveTopology {
    fn from(value: &webgpu::GpuPrimitiveTopology) -> Self {
        match value {
            webgpu::GpuPrimitiveTopology::PointList => wgpu_types::PrimitiveTopology::PointList,
            webgpu::GpuPrimitiveTopology::LineList => wgpu_types::PrimitiveTopology::LineList,
            webgpu::GpuPrimitiveTopology::LineStrip => wgpu_types::PrimitiveTopology::LineStrip,
            webgpu::GpuPrimitiveTopology::TriangleList => {
                wgpu_types::PrimitiveTopology::TriangleList
            }
            webgpu::GpuPrimitiveTopology::TriangleStrip => {
                wgpu_types::PrimitiveTopology::TriangleStrip
            }
        }
    }
}
impl From<&wgpu_types::PrimitiveTopology> for webgpu::GpuPrimitiveTopology {
    fn from(value: &wgpu_types::PrimitiveTopology) -> Self {
        match value {
            wgpu_types::PrimitiveTopology::PointList => webgpu::GpuPrimitiveTopology::PointList,
            wgpu_types::PrimitiveTopology::LineList => webgpu::GpuPrimitiveTopology::LineList,
            wgpu_types::PrimitiveTopology::LineStrip => webgpu::GpuPrimitiveTopology::LineStrip,
            wgpu_types::PrimitiveTopology::TriangleList => {
                webgpu::GpuPrimitiveTopology::TriangleList
            }
            wgpu_types::PrimitiveTopology::TriangleStrip => {
                webgpu::GpuPrimitiveTopology::TriangleStrip
            }
        }
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
