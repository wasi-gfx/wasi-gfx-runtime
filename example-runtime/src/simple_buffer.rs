use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use wasmtime::component::Resource;

use crate::graphics_context::{GraphicsContext, GraphicsContextBuffer, GraphicsContextKind};
use crate::{HostEvent, HostState};

#[derive(Clone)]
pub struct Surface {
    surface: Arc<Mutex<softbuffer::Surface>>,
}
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}
impl From<softbuffer::Surface> for Surface {
    fn from(surface: softbuffer::Surface) -> Self {
        Surface {
            surface: Arc::new(Mutex::new(surface)),
        }
    }
}
impl Surface {
    pub fn buffer_mut<'a>(&'a mut self) -> SimpleBuffer {
        let mut surface = self.surface.lock().unwrap();
        let buff = surface.buffer_mut().unwrap();
        let buff: softbuffer::Buffer<'static> = unsafe { mem::transmute(buff) };
        buff.into()
    }

    pub fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.surface.lock().unwrap().resize(width, height).unwrap();
    }
}

pub struct SimpleBuffer {
    // Never none
    buffer: Arc<Mutex<Option<softbuffer::Buffer<'static>>>>,
}
unsafe impl Send for SimpleBuffer {}
unsafe impl Sync for SimpleBuffer {}
impl From<softbuffer::Buffer<'static>> for SimpleBuffer {
    fn from(buffer: softbuffer::Buffer<'static>) -> Self {
        SimpleBuffer {
            buffer: Arc::new(Mutex::new(Some(buffer))),
        }
    }
}

impl crate::component::webgpu::simple_buffer::Host for HostState {
    fn connect_graphics_context(
        &mut self,
        graphics_context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let context = unsafe { softbuffer::Context::new(&self.window) }.unwrap();
        let mut surface = unsafe { softbuffer::Surface::new(&context, &self.window) }.unwrap();

        let size = self.window.inner_size();

        let _ = surface.resize(
            size.width.try_into().unwrap(),
            size.height.try_into().unwrap(),
        );

        let surface: Surface = surface.into();

        let mut receiver = self.sender.subscribe();
        let mut surface_clone = surface.clone();

        tokio::spawn(async move {
            loop {
                let event = receiver.recv().await.unwrap();
                if let HostEvent::CanvasResizeEvent(event) = event {
                    surface_clone.resize(
                        event.width.try_into().unwrap(),
                        event.height.try_into().unwrap(),
                    );
                }
            }
        });

        let graphics_context = self.table.get_mut(&graphics_context).unwrap();
        graphics_context.kind = Some(GraphicsContextKind::SimpleBuffer(surface));
        Ok(())
    }
}

impl crate::component::webgpu::simple_buffer::HostSimpleBuffer for HostState {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<crate::graphics_context::GraphicsContextBuffer>,
    ) -> wasmtime::Result<Resource<SimpleBuffer>> {
        let host_buffer: GraphicsContextBuffer = self.table.delete(buffer).unwrap();
        if let GraphicsContextBuffer::SimpleBuffer(host_buffer) = host_buffer {
            Ok(self.table.push(host_buffer).unwrap())
        } else {
            panic!("Context not connected to webgpu");
        }
    }

    fn present(&mut self, buffer: Resource<SimpleBuffer>) -> wasmtime::Result<()> {
        let buffer = self.table.delete(buffer).unwrap();
        buffer
            .buffer
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .present()
            .unwrap();
        Ok(())
    }

    fn length(&mut self, buffer: Resource<SimpleBuffer>) -> wasmtime::Result<u32> {
        let buffer = self.table.get(&buffer).unwrap();
        let len = buffer.buffer.lock().unwrap().as_ref().unwrap().len();
        Ok(len as u32)
    }

    fn get(&mut self, buffer: Resource<SimpleBuffer>, i: u32) -> wasmtime::Result<u32> {
        let buffer = self.table.get(&buffer).unwrap();
        let val = *buffer
            .buffer
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(i as usize)
            .unwrap();
        Ok(val)
    }

    fn set(&mut self, buffer: Resource<SimpleBuffer>, i: u32, val: u32) -> wasmtime::Result<()> {
        let buffer = self.table.get_mut(&buffer).unwrap();
        buffer.buffer.lock().unwrap().as_mut().unwrap()[i as usize] = val as u32;
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<SimpleBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
