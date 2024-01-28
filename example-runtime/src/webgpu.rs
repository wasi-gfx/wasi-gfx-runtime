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
    fn request_adapter(&mut self) -> wasmtime::Result<Resource<wgpu_core::id::AdapterId>> {
        let adapter = self
            .instance
            .request_adapter(
                &Default::default(),
                wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| ()),
            )
            .unwrap();
        Ok(self.table.push(adapter).unwrap())
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
                targets: fragment
                    .targets
                    .iter()
                    .map(|target| {
                        Some(wgpu_types::ColorTargetState {
                            format: target.into(),
                            blend: None,
                            write_mask: Default::default(),
                        })
                    })
                    .collect::<Vec<_>>()
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
}

impl webgpu::HostGpuTextureView for HostState {
    fn drop(&mut self, _rep: Resource<wgpu_core::id::TextureViewId>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl webgpu::HostGpuCommandBuffer for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        // self.web_gpu_host.command_buffers.remove(&rep.rep());
        Ok(())
    }
}

impl webgpu::HostGpuShaderModule for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        // self.web_gpu_host.shaders.remove(&rep.rep());
        Ok(())
    }
}

impl webgpu::HostGpuRenderPipeline for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

impl webgpu::HostGpuAdapter for HostState {
    fn request_device(
        &mut self,
        adapter: Resource<wgpu_core::id::AdapterId>,
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

    fn drop(&mut self, _rep: Resource<webgpu::GpuQueue>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
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
}

impl From<&wgpu_types::TextureFormat> for webgpu::GpuTextureFormat {
    fn from(value: &wgpu_types::TextureFormat) -> Self {
        match value {
            wgpu_types::TextureFormat::Bgra8UnormSrgb => webgpu::GpuTextureFormat::Bgra8UnormSrgb,
            _ => todo!(),
        }
    }
}
impl From<&webgpu::GpuTextureFormat> for wgpu_types::TextureFormat {
    fn from(value: &webgpu::GpuTextureFormat) -> Self {
        match value {
            webgpu::GpuTextureFormat::Bgra8UnormSrgb => wgpu_types::TextureFormat::Bgra8UnormSrgb,
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
