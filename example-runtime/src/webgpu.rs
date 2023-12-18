use futures::executor::block_on;
use std::borrow::Cow;
use std::collections::HashMap;
use wasmtime::component::Resource;

use crate::component::webgpu::webgpu;
use crate::graphics_context::{GraphicsBuffer, GraphicsContext, GraphicsContextKind};
use crate::HostState;

pub struct DeviceAndQueue {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,

    // only needed when calling surface.get_capabilities in connect_graphics_context. If table would have a way to get parent from child, we could get it from device.
    pub _adapter: Resource<wgpu::Adapter>,
}

#[async_trait::async_trait]
impl<'a> webgpu::Host for HostState {
    async fn request_adapter(&mut self) -> wasmtime::Result<Resource<wgpu::Adapter>> {
        let adapter = block_on(self.instance.request_adapter(&Default::default())).unwrap();
        Ok(self.table.push(adapter).unwrap())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuDevice for HostState {
    async fn connect_graphics_context(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let surface = unsafe { self.instance.create_surface(&self.window) }.unwrap();

        let host_daq = self.table.get(&daq).unwrap();

        // think the table should have a way to get parent so that we can get adapter from device.
        let adapter = self.table.get(&host_daq._adapter).unwrap();

        let mut size = self.window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&host_daq.device, &config);

        let context = self.table.get_mut(&context).unwrap();

        context.kind = Some(GraphicsContextKind::Webgpu(surface));

        Ok(())
    }

    async fn create_command_encoder(
        &mut self,
        daq: Resource<DeviceAndQueue>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let host_daq = self.table.get(&daq).unwrap();
        let command_encoder = host_daq
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Ok(self.table.push_child(command_encoder, &daq).unwrap())
    }
    async fn do_all(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        desc: webgpu::GpuRenderPassDescriptor,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
        _count: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let host_daq = self.table.get(&daq).unwrap();

        let render_pipeline = self.table.get(&pipeline).unwrap();

        let mut command_encoder = host_daq
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let color_attachments = desc
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let view = self.table.get(&color_attachment.view).unwrap();

                Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.1,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect::<Vec<_>>();
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

        Ok(self.table.push_child(command_encoder, &daq).unwrap())
    }

    async fn create_shader_module(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        desc: webgpu::GpuShaderModuleDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuShaderModule>> {
        let daq = self.table.get(&daq).unwrap();
        let shader = daq
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: desc.label.as_deref(),
                source: wgpu::ShaderSource::Wgsl(Cow::Owned(desc.code)),
            });

        Ok(self.table.push(shader).unwrap())
    }

    async fn create_render_pipeline(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        props: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPipeline>> {
        let vertex = wgpu::VertexState {
            module: &self.table.get(&props.vertex.module).unwrap(),
            entry_point: &props.vertex.entry_point,
            buffers: &[],
        };

        let fragment = wgpu::FragmentState {
            module: &self.table.get(&props.fragment.module).unwrap(),
            entry_point: &props.fragment.entry_point,
            targets: &props
                .fragment
                .targets
                .iter()
                .map(|target| {
                    Some(wgpu::ColorTargetState {
                        format: target.into(),
                        blend: None,
                        write_mask: Default::default(),
                    })
                })
                .collect::<Vec<_>>(),
        };

        // let primitive = wgpu::PrimitiveState {
        //     topology: (&props.primitive.topology).into(),
        //     ..Default::default()
        // };

        let host_daq = self.table.get(&daq).unwrap();

        let render_pipeline =
            host_daq
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    vertex,
                    fragment: Some(fragment),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: Default::default(),
                    multisample: Default::default(),
                    multiview: Default::default(),
                    label: Default::default(),
                    layout: Default::default(),
                });

        Ok(self.table.push_child(render_pipeline, &daq).unwrap())
    }

    async fn queue(
        &mut self,
        daq: Resource<DeviceAndQueue>,
    ) -> wasmtime::Result<Resource<DeviceAndQueue>> {
        Ok(Resource::new_own(daq.rep()))
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        // self.web_gpu_host.devices.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuTexture for HostState {
    async fn from_graphics_buffer(
        &mut self,
        buffer: Resource<GraphicsBuffer>,
    ) -> wasmtime::Result<Resource<wgpu::SurfaceTexture>> {
        let host_buffer = self.table.delete(buffer).unwrap();
        if let GraphicsBuffer::Webgpu(host_buffer) = host_buffer {
            Ok(self.table.push(host_buffer).unwrap())
        } else {
            panic!("Context not connected to webgpu");
        }
    }

    async fn create_view(
        &mut self,
        texture: Resource<wgpu::SurfaceTexture>,
    ) -> wasmtime::Result<Resource<wgpu::TextureView>> {
        let host_texture = self.table.get(&texture).unwrap();
        let texture_view = host_texture.texture.create_view(&Default::default());

        Ok(self.table.push(texture_view).unwrap())
    }

    async fn non_standard_present(
        &mut self,
        texture: Resource<wgpu::SurfaceTexture>,
    ) -> wasmtime::Result<()> {
        let texture = self.table.delete(texture).unwrap();
        texture.present();
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<wgpu::SurfaceTexture>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuTextureView for HostState {
    fn drop(&mut self, _rep: Resource<wgpu::TextureView>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuCommandBuffer for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        // self.web_gpu_host.command_buffers.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuShaderModule for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        // self.web_gpu_host.shaders.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuRenderPipeline for HostState {
    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderPipeline>) -> wasmtime::Result<()> {
        // TODO:
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuAdapter for HostState {
    async fn request_device(
        &mut self,
        adapter: Resource<wgpu::Adapter>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDevice>> {
        let host_adapter = self.table.get(&adapter).unwrap();

        let (device, queue) =
            block_on(host_adapter.request_device(&Default::default(), Default::default())).unwrap();

        let daq = self
            .table
            .push_child(
                DeviceAndQueue {
                    device,
                    queue,
                    _adapter: Resource::new_own(adapter.rep()),
                },
                &adapter,
            )
            .unwrap();

        Ok(daq)
    }

    fn drop(&mut self, adapter: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        self.table.delete(adapter).unwrap();
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuDeviceQueue for HostState {
    async fn submit(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) -> wasmtime::Result<()> {
        let command_buffers = val
            .into_iter()
            .map(|buffer| self.table.delete(buffer).unwrap())
            .collect::<Vec<_>>();

        let daq = self.table.get(&daq).unwrap();
        daq.queue.submit(command_buffers);

        Ok(())
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuDeviceQueue>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuCommandEncoder for HostState {
    async fn begin_render_pass(
        &mut self,
        command_encoder: Resource<webgpu::GpuCommandEncoder>,
        desc: webgpu::GpuRenderPassDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPass>> {
        let mut command_encoder_and_views = self.table.iter_entries({
            let mut m = HashMap::new();
            m.insert(command_encoder.rep(), command_encoder.rep());
            for color_attachment in &desc.color_attachments {
                m.insert(color_attachment.view.rep(), color_attachment.view.rep());
            }
            m
        });
        let host_command_encoder: &mut wgpu::CommandEncoder = command_encoder_and_views
            .next()
            .unwrap()
            .0
            .unwrap()
            .downcast_mut()
            .unwrap();

        let mut color_attachments = vec![];
        for _color_attachment in desc.color_attachments {
            let view: &wgpu::TextureView = command_encoder_and_views
                .next()
                .unwrap()
                .0
                .unwrap()
                .downcast_ref()
                .unwrap();

            color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                    store: wgpu::StoreOp::Store,
                },
            }));
        }

        let _render_pass = host_command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &color_attachments,
            label: None,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        Ok(Resource::new_own(command_encoder.rep()))
    }

    async fn finish(
        &mut self,
        command_encoder: Resource<webgpu::GpuCommandEncoder>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        // let encoder = self.web_gpu_host.encoders.remove(&command_encoder.rep()).unwrap().0;
        let command_encoder = self.table.delete(command_encoder).unwrap();
        let command_buffer = command_encoder.finish();
        Ok(self.table.push(command_buffer).unwrap())
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuCommandEncoder>) -> wasmtime::Result<()> {
        // self.web_gpu_host.encoders.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuRenderPass for HostState {
    async fn set_pipeline(
        &mut self,
        _self_: Resource<webgpu::GpuRenderPass>,
        _pipeline: Resource<webgpu::GpuRenderPipeline>,
    ) -> wasmtime::Result<()> {
        anyhow::bail!("")
    }

    async fn draw(
        &mut self,
        _self_: Resource<webgpu::GpuRenderPass>,
        _count: u32,
    ) -> wasmtime::Result<()> {
        anyhow::bail!("")
    }

    async fn end(&mut self, _self_: Resource<webgpu::GpuRenderPass>) -> wasmtime::Result<()> {
        anyhow::bail!("")
    }

    fn drop(&mut self, _rep: Resource<webgpu::GpuRenderPass>) -> wasmtime::Result<()> {
        anyhow::bail!("")
    }
}

impl From<&wgpu::TextureFormat> for webgpu::GpuTextureFormat {
    fn from(value: &wgpu::TextureFormat) -> Self {
        match value {
            wgpu::TextureFormat::Bgra8UnormSrgb => webgpu::GpuTextureFormat::Bgra8UnormSrgb,
            _ => todo!(),
        }
    }
}
impl From<&webgpu::GpuTextureFormat> for wgpu::TextureFormat {
    fn from(value: &webgpu::GpuTextureFormat) -> Self {
        match value {
            webgpu::GpuTextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
}

impl From<&webgpu::GpuPrimitiveTopology> for wgpu::PrimitiveTopology {
    fn from(value: &webgpu::GpuPrimitiveTopology) -> Self {
        match value {
            webgpu::GpuPrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            webgpu::GpuPrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            webgpu::GpuPrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            webgpu::GpuPrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            webgpu::GpuPrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
}
impl From<&wgpu::PrimitiveTopology> for webgpu::GpuPrimitiveTopology {
    fn from(value: &wgpu::PrimitiveTopology) -> Self {
        match value {
            wgpu::PrimitiveTopology::PointList => webgpu::GpuPrimitiveTopology::PointList,
            wgpu::PrimitiveTopology::LineList => webgpu::GpuPrimitiveTopology::LineList,
            wgpu::PrimitiveTopology::LineStrip => webgpu::GpuPrimitiveTopology::LineStrip,
            wgpu::PrimitiveTopology::TriangleList => webgpu::GpuPrimitiveTopology::TriangleList,
            wgpu::PrimitiveTopology::TriangleStrip => webgpu::GpuPrimitiveTopology::TriangleStrip,
        }
    }
}
