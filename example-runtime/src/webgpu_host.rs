use futures::executor::block_on;
use std::borrow::Cow;
use std::collections::HashMap;
use wasmtime::component::Resource;

use crate::component::webgpu::webgpu;
use crate::HostState;

pub struct DeviceAndQueue {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

// Don't think this should exist, at least not in this form.
pub struct DisplayableEntityView {
    texture_view: wgpu::TextureView,
    surface_texture: wgpu::SurfaceTexture,
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

pub struct WebGpuHost<'a> {
    encoders: HashMap<
        u32,
        (
            wgpu::CommandEncoder,
            Option<Vec<Option<wgpu::RenderPassColorAttachment<'a>>>>,
        ),
    >,
}

#[async_trait::async_trait]
impl<'a> webgpu::Host for HostState {
    async fn request_adapter(&mut self) -> wasmtime::Result<Resource<wgpu::Adapter>> {
        let adapter = block_on(self.instance.request_adapter(&Default::default())).unwrap();
        Ok(self.table.push(adapter).unwrap())
    }
    async fn get_displayable_entity(
        &mut self,
        adapter: Resource<wgpu::Adapter>,
        daq: Resource<DeviceAndQueue>,
    ) -> wasmtime::Result<Resource<webgpu::DisplayableEntity>> {
        let host_daq = self.table.get(&daq).unwrap();
        let adapter = self.table.get(&adapter).unwrap();

        let mut size = self.window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let surface = unsafe { self.instance.create_surface(&self.window) }.unwrap();

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

        Ok(self.table.push_child(surface, &daq).unwrap())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuDevice for HostState {
    async fn create_command_encoder(
        &mut self,
        daq: Resource<DeviceAndQueue>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let daq = self.table.get(&daq).unwrap();
        let command_encoder = daq
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let id = rand::random();
        self.web_gpu_host
            .encoders
            .insert(id, (command_encoder, None));

        Ok(Resource::new_own(id))
    }
    async fn do_all(
        &mut self,
        daq: Resource<DeviceAndQueue>,
        desc: webgpu::GpuRenderPassDescriptor,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
        _count: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let daq = self.table.get(&daq).unwrap();

        let render_pipeline = self.table.get(&pipeline).unwrap();

        let mut encoder = daq
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let color_attachments = desc
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let view = self
                    .table
                    .get(&color_attachment.view)
                    .unwrap();

                Some(wgpu::RenderPassColorAttachment {
                    view: &view.texture_view,
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
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &color_attachments,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.draw(0..3, 0..1);
        drop(render_pass);

        // Ok(())
        let id = rand::random();
        self.web_gpu_host.encoders.insert(id, (encoder, None));

        Ok(Resource::new_own(id))
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
            module: &self
                .table
                .get(&props.vertex.module)
                .unwrap(),
            entry_point: &props.vertex.entry_point,
            buffers: &[],
        };

        let fragment = wgpu::FragmentState {
            module: &self
                .table
                .get(&props.fragment.module)
                .unwrap(),
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

        let render_pipeline = host_daq
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
impl<'a> webgpu::HostDisplayableEntity for HostState {
    async fn create_view(
        &mut self,
        displayable_entity: Resource<wgpu::Surface>,
    ) -> wasmtime::Result<Resource<webgpu::DisplayableEntityView>> {
        let displayable_entity = self.table.get(&displayable_entity).unwrap();
        let surface_texture = displayable_entity.get_current_texture().unwrap();
        let texture_view = surface_texture.texture.create_view(&Default::default());

        let displayable_entity_view = self.table.push(DisplayableEntityView {
            texture_view,
            surface_texture,
        }).unwrap();

        Ok(displayable_entity_view)
    }

    fn drop(&mut self, _rep: Resource<webgpu::DisplayableEntity>) -> wasmtime::Result<()> {
        // self.web_gpu_host.displayable_entities.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostDisplayableEntityView for HostState {
    async fn non_standard_present(&mut self, displayable_entity_view: Resource<webgpu::DisplayableEntityView>) -> wasmtime::Result<()> {
        let displayable_entity_view = self.table.delete(displayable_entity_view).unwrap();

        displayable_entity_view.surface_texture.present();
        Ok(())
    }
    fn drop(&mut self, _rep: Resource<webgpu::DisplayableEntityView>) -> wasmtime::Result<()> {
        // self.web_gpu_host.displayable_entities.remove(&rep.rep());
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

        let daq = self.table.push_child(DeviceAndQueue { device, queue }, &adapter).unwrap();

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
            .map(|buffer| {
                self.table
                    .delete(buffer)
                    .unwrap()
            })
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
        self_: Resource<webgpu::GpuCommandEncoder>,
        desc: webgpu::GpuRenderPassDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPass>> {
        let encoder = self.web_gpu_host.encoders.get_mut(&self_.rep()).unwrap();

        let color_attachments = desc
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let view = self
                    .table
                    .get(&color_attachment.view)
                    .unwrap();

                Some(wgpu::RenderPassColorAttachment {
                    view: &view.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })
            })
            .collect::<Vec<_>>();

        let _render_pass = encoder.0.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &color_attachments,
            label: None,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        Ok(Resource::new_own(self_.rep()))
    }

    async fn finish(
        &mut self,
        command_encoder: Resource<webgpu::GpuCommandEncoder>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        let encoder = self.web_gpu_host.encoders.remove(&command_encoder.rep()).unwrap().0;
        let command_buffer = encoder.finish();
        Ok(self.table.push(command_buffer).unwrap())
    }

    fn drop(&mut self, rep: Resource<webgpu::GpuCommandEncoder>) -> wasmtime::Result<()> {
        self.web_gpu_host.encoders.remove(&rep.rep());
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

impl<'a> WebGpuHost<'a> {
    pub fn new() -> Self {
        Self {
            encoders: HashMap::new(),
        }
    }
}
