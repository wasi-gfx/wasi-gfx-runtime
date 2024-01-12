use crate::{component::webgpu::graphics_context::ConfigureContextDesc, HostState};
use wasmtime::component::Resource;

// should context be an enum? like: Context::Webgpu2Canvas, Context::Buffer2Canvas.

pub struct GraphicsContext {
    pub kind: Option<GraphicsContextKind>,
}

pub enum GraphicsContextKind {
    Webgpu(wgpu::Surface),
    SimpleBuffer(crate::simple_buffer::Surface),
}

#[non_exhaustive]
pub enum GraphicsBuffer {
    Webgpu(wgpu::SurfaceTexture),
    SimpleBuffer(crate::simple_buffer::SimpleBuffer),
}

impl crate::component::webgpu::graphics_context::Host for HostState {}

#[async_trait::async_trait]
impl crate::component::webgpu::graphics_context::HostGraphicsContext for HostState {
    async fn new(&mut self) -> wasmtime::Result<Resource<GraphicsContext>> {
        // let surface = unsafe { self.instance.create_surface(&self.window) }.unwrap();
        Ok(self.table.push(GraphicsContext { kind: None }).unwrap())
    }

    async fn configure(
        &mut self,
        context: Resource<GraphicsContext>,
        _desc: ConfigureContextDesc,
    ) -> wasmtime::Result<()> {
        let _context = self.table.get(&context).unwrap();
        Ok(())
    }

    async fn get_current_buffer(
        &mut self,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<Resource<GraphicsBuffer>> {
        let context_kind = self.table.get_mut(&context).unwrap().kind.as_mut().unwrap();
        let next_frame = match context_kind {
            GraphicsContextKind::Webgpu(surface) => {
                GraphicsBuffer::Webgpu(surface.get_current_texture().unwrap())
            }
            GraphicsContextKind::SimpleBuffer(surface) => {
                GraphicsBuffer::SimpleBuffer(surface.buffer_mut())
            }
        };
        Ok(self.table.push_child(next_frame, &context).unwrap())
    }

    fn drop(&mut self, _context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl crate::component::webgpu::graphics_context::HostBuffer for HostState {
    fn drop(&mut self, _rep: Resource<GraphicsBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
