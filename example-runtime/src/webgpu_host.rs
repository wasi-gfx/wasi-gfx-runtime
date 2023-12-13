use futures::executor::block_on;
use std::borrow::Cow;
use std::collections::HashMap;
use wasmtime::component::Resource;
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::component::webgpu::webgpu;
use crate::HostState;

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
    instance: wgpu::Instance,
    adapters: HashMap<u32, wgpu::Adapter>,
    devices: HashMap<u32, (wgpu::Device, wgpu::Queue)>,
    displayable_entities: HashMap<u32, wgpu::Surface>,
    views: HashMap<u32, (wgpu::TextureView, wgpu::SurfaceTexture)>,
    shaders: HashMap<u32, wgpu::ShaderModule>,
    encoders: HashMap<
        u32,
        (
            wgpu::CommandEncoder,
            Option<Vec<Option<wgpu::RenderPassColorAttachment<'a>>>>,
        ),
    >,
    command_buffers: HashMap<u32, wgpu::CommandBuffer>,
    render_pipelines: HashMap<u32, wgpu::RenderPipeline>,
    window: Window,
}

#[async_trait::async_trait]
impl<'a> webgpu::Host for HostState {
    async fn request_adapter(&mut self) -> wasmtime::Result<Resource<webgpu::GpuAdapter>> {
        let adapter = block_on(
            self.web_gpu_host
                .instance
                .request_adapter(&Default::default()),
        )
        .unwrap();
        let id = rand::random();
        self.web_gpu_host.adapters.insert(id, adapter);
        Ok(Resource::new_own(id))
    }
    async fn get_displayable_entity(
        &mut self,
        _adapter: u32,
        _device: u32,
    ) -> wasmtime::Result<Resource<webgpu::DisplayableEntity>> {
        let device = self.web_gpu_host.devices.keys().into_iter().next().unwrap();
        let adapter = self
            .web_gpu_host
            .adapters
            .keys()
            .into_iter()
            .next()
            .unwrap();

        let (device, _) = self.web_gpu_host.devices.get(&device).unwrap();
        let adapter = self.web_gpu_host.adapters.get(&adapter).unwrap();

        let mut size = self.web_gpu_host.window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        let surface = unsafe {
            self.web_gpu_host
                .instance
                .create_surface(&self.web_gpu_host.window)
        }
        .unwrap();

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

        surface.configure(&device, &config);

        let id = rand::random();
        self.web_gpu_host.displayable_entities.insert(id, surface);

        Ok(Resource::new_own(id))
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuDevice for HostState {
    async fn create_command_encoder(
        &mut self,
        self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let (device, _) = self.web_gpu_host.devices.get(&self_.rep()).unwrap();
        let command_encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let id = rand::random();
        self.web_gpu_host
            .encoders
            .insert(id, (command_encoder, None));

        Ok(Resource::new_own(id))
    }
    async fn do_all(
        &mut self,
        self_: Resource<webgpu::GpuDevice>,
        desc: webgpu::GpuRenderPassDescriptor,
        pipeline: Resource<webgpu::GpuRenderPipeline>,
        _count: u32,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandEncoder>> {
        let (device, _) = self.web_gpu_host.devices.get(&self_.rep()).unwrap();
        let render_pipeline = self
            .web_gpu_host
            .render_pipelines
            .get(&pipeline.rep())
            .unwrap();

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let color_attachments = desc
            .color_attachments
            .iter()
            .map(|color_attachment| {
                let view = self
                    .web_gpu_host
                    .views
                    .get(&color_attachment.view.rep())
                    .unwrap();

                Some(wgpu::RenderPassColorAttachment {
                    view: &view.0,
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
        self_: Resource<webgpu::GpuDevice>,
        desc: webgpu::GpuShaderModuleDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuShaderModule>> {
        let (device, _) = self.web_gpu_host.devices.get(&self_.rep()).unwrap();
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: desc.label.as_deref(),
            source: wgpu::ShaderSource::Wgsl(Cow::Owned(desc.code)),
        });

        let id = rand::random();
        self.web_gpu_host.shaders.insert(id, shader);

        Ok(Resource::new_own(id))
    }

    async fn create_render_pipeline(
        &mut self,
        self_: Resource<webgpu::GpuDevice>,
        props: webgpu::GpuRenderPipelineDescriptor,
    ) -> wasmtime::Result<Resource<webgpu::GpuRenderPipeline>> {
        let vertex = wgpu::VertexState {
            module: &self
                .web_gpu_host
                .shaders
                .get(&props.vertex.module.rep())
                .unwrap(),
            entry_point: &props.vertex.entry_point,
            buffers: &[],
        };

        let fragment = wgpu::FragmentState {
            module: &self
                .web_gpu_host
                .shaders
                .get(&props.fragment.module.rep())
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

        let (device, _) = self.web_gpu_host.devices.get(&self_.rep()).unwrap();

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            vertex,
            fragment: Some(fragment),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Default::default(),
            multisample: Default::default(),
            multiview: Default::default(),
            label: Default::default(),
            layout: Default::default(),
        });

        let id = rand::random();
        self.web_gpu_host
            .render_pipelines
            .insert(id, render_pipeline);
        Ok(Resource::new_own(id))
    }

    async fn queue(
        &mut self,
        self_: Resource<webgpu::GpuDevice>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDeviceQueue>> {
        Ok(Resource::new_own(self_.rep()))
    }

    fn drop(&mut self, rep: Resource<webgpu::GpuDevice>) -> wasmtime::Result<()> {
        self.web_gpu_host.devices.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostDisplayableEntity for HostState {
    async fn create_view(
        &mut self,
        self_: Resource<webgpu::DisplayableEntity>,
    ) -> wasmtime::Result<Resource<webgpu::DisplayableEntityView>> {
        let displayable_entity = self
            .web_gpu_host
            .displayable_entities
            .get(&self_.rep())
            .unwrap();
        let surface = displayable_entity.get_current_texture().unwrap();
        let view = surface.texture.create_view(&Default::default());

        let id = rand::random();
        self.web_gpu_host.views.insert(id, (view, surface));
        Ok(Resource::new_own(id))
    }

    fn drop(&mut self, rep: Resource<webgpu::DisplayableEntity>) -> wasmtime::Result<()> {
        self.web_gpu_host.displayable_entities.remove(&rep.rep());
        Ok(())
    }
}

impl<'a> webgpu::HostDisplayableEntityView for HostState {
    fn drop(&mut self, rep: Resource<webgpu::DisplayableEntityView>) -> wasmtime::Result<()> {
        self.web_gpu_host.displayable_entities.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuCommandBuffer for HostState {
    fn drop(&mut self, rep: Resource<webgpu::GpuCommandBuffer>) -> wasmtime::Result<()> {
        self.web_gpu_host.command_buffers.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuShaderModule for HostState {
    fn drop(&mut self, rep: Resource<webgpu::GpuShaderModule>) -> wasmtime::Result<()> {
        self.web_gpu_host.shaders.remove(&rep.rep());
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
        self_: Resource<webgpu::GpuAdapter>,
    ) -> wasmtime::Result<Resource<webgpu::GpuDevice>> {
        let adapter = self.web_gpu_host.adapters.get(&self_.rep()).unwrap();

        let device =
            block_on(adapter.request_device(&Default::default(), Default::default())).unwrap();

        let id = rand::random();
        self.web_gpu_host.devices.insert(id, device);
        Ok(Resource::new_own(id))
    }

    fn drop(&mut self, rep: Resource<webgpu::GpuAdapter>) -> wasmtime::Result<()> {
        self.web_gpu_host.adapters.remove(&rep.rep());
        Ok(())
    }
}

#[async_trait::async_trait]
impl<'a> webgpu::HostGpuDeviceQueue for HostState {
    async fn submit(
        &mut self,
        self_: Resource<webgpu::GpuDeviceQueue>,
        val: Vec<Resource<webgpu::GpuCommandBuffer>>,
    ) -> wasmtime::Result<()> {
        let (_, queue) = self.web_gpu_host.devices.get(&self_.rep()).unwrap();
        let command_buffers = val
            .iter()
            .map(|buffer| {
                self.web_gpu_host
                    .command_buffers
                    .remove(&buffer.rep())
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let id = {
            let keys: Vec<_> = self.web_gpu_host.views.keys().collect();
            *keys[0]
        };

        let (_, surface_texture) = self.web_gpu_host.views.remove(&id).unwrap();
        queue.submit(command_buffers);

        surface_texture.present();

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
                    .web_gpu_host
                    .views
                    .get(&color_attachment.view.rep())
                    .unwrap();

                Some(wgpu::RenderPassColorAttachment {
                    view: &view.0,
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
        self_: Resource<webgpu::GpuCommandEncoder>,
    ) -> wasmtime::Result<Resource<webgpu::GpuCommandBuffer>> {
        let encoder = self.web_gpu_host.encoders.remove(&self_.rep()).unwrap().0;
        let command_buffer = encoder.finish();
        let id = rand::random();
        self.web_gpu_host.command_buffers.insert(id, command_buffer);
        Ok(Resource::new_own(id))
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
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        Self {
            instance: Default::default(),
            adapters: HashMap::new(),
            devices: HashMap::new(),
            displayable_entities: HashMap::new(),
            shaders: HashMap::new(),
            encoders: HashMap::new(),
            command_buffers: HashMap::new(),
            render_pipelines: HashMap::new(),
            views: HashMap::new(),
            window: winit::window::Window::new(event_loop).unwrap(),
        }
    }
}
