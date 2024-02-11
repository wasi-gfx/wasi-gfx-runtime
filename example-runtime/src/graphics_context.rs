use crate::{component::webgpu::graphics_context::ConfigureContextDesc, HostState};
use wasmtime::component::Resource;

// should context be an enum? like: Context::Webgpu2Canvas, Context::Buffer2Canvas.

pub struct GraphicsContext {
    pub kind: Option<GraphicsContextKind>,
}

pub enum GraphicsContextKind {
    Webgpu(wgpu_core::id::SurfaceId),
    FrameBuffer(crate::frame_buffer::Surface),
}

#[non_exhaustive]
pub enum GraphicsContextBuffer {
    Webgpu(wgpu_core::id::TextureId),
    FrameBuffer(crate::frame_buffer::FrameBuffer),
}

impl crate::component::webgpu::graphics_context::Host for HostState {}

impl crate::component::webgpu::graphics_context::HostGraphicsContext for HostState {
    fn new(&mut self) -> wasmtime::Result<Resource<GraphicsContext>> {
        Ok(self.table.push(GraphicsContext { kind: None }).unwrap())
    }

    fn configure(
        &mut self,
        context: Resource<GraphicsContext>,
        _desc: ConfigureContextDesc,
    ) -> wasmtime::Result<()> {
        let _context = self.table.get(&context).unwrap();
        Ok(())
    }

    fn get_current_buffer(
        &mut self,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<Resource<GraphicsContextBuffer>> {
        let context_kind = self.table.get_mut(&context).unwrap().kind.as_mut().unwrap();
        let next_frame = match context_kind {
            GraphicsContextKind::Webgpu(surface) => {
                let texture = self
                    .instance
                    .surface_get_current_texture::<crate::Backend>(*surface, ())
                    .unwrap()
                    .texture_id
                    .unwrap();
                GraphicsContextBuffer::Webgpu(texture)
            }
            GraphicsContextKind::FrameBuffer(surface) => {
                GraphicsContextBuffer::FrameBuffer(surface.buffer_mut())
            }
        };
        Ok(self.table.push_child(next_frame, &context).unwrap())
    }

    fn present(&mut self, context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        let context = self.table.get(&context).unwrap();
        match &context.kind {
            Some(GraphicsContextKind::Webgpu(surface)) => {
                self.instance
                    .surface_present::<crate::Backend>(*surface)
                    .unwrap();
            }
            Some(GraphicsContextKind::FrameBuffer(surface)) => {
                surface
                    .surface
                    .lock()
                    .unwrap()
                    .buffer_mut()
                    .unwrap()
                    .present()
                    .unwrap();
            }
            None => {
                panic!("present called on an unconnected context.");
            }
        };
        Ok(())
    }

    fn drop(&mut self, _graphics_context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl crate::component::webgpu::graphics_context::HostGraphicsContextBuffer for HostState {
    fn drop(&mut self, _rep: Resource<GraphicsContextBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
