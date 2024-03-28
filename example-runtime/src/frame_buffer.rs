use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use wasmtime::component::Resource;
use wasmtime_wasi::preview2::WasiView;

use crate::graphics_context::{DisplayApi, DrawApi, GraphicsContext, GraphicsContextBuffer};
use crate::{HostEvent, HostState};


// TODO: rename to FBBuffer and FBSurface?

#[derive(Clone)]
pub struct Surface {
    pub(super) surface: Arc<Mutex<softbuffer::Surface>>,
}
impl Surface {
    // pub fn buffer_mut<'a>(&'a mut self) -> FrameBuffer {
    //     let mut surface = self.surface.lock().unwrap();
    //     let buff = surface.buffer_mut().unwrap();
    //     // TODO: use ouroboros?
    //     let buff: softbuffer::Buffer<'static> = unsafe { mem::transmute(buff) };
    //     buff.into()
    // }

    pub fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.surface.lock().unwrap().resize(width, height).unwrap();
    }
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
impl DrawApi for Surface {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer> {
        let mut surface = self.surface.lock().unwrap();
        let buff = surface.buffer_mut().unwrap();
        // TODO: use ouroboros?
        let buff: softbuffer::Buffer<'static> = unsafe { mem::transmute(buff) };
        let buff: FrameBuffer = buff.into();
        let buff = Box::new(buff);
        let buff: GraphicsContextBuffer = buff.into();
        Ok(buff)
    }
    
    fn present(&mut self) -> wasmtime::Result<()> {
        // TODO: should present be on the actual buffer? That's would track better with both softbuffer and wgpu
        self
            .surface
            .lock()
            .unwrap()
            .buffer_mut()
            .unwrap()
            .present()
            .unwrap();
        Ok(())
    }
    
    fn display_api_ready(&mut self, display: &Box<dyn DisplayApi + Send + Sync>) {
        todo!()
    }
}

pub struct FrameBuffer {
    // Never none
    buffer: Arc<Mutex<Option<softbuffer::Buffer<'static>>>>,
}
unsafe impl Send for FrameBuffer {}
unsafe impl Sync for FrameBuffer {}
impl From<softbuffer::Buffer<'static>> for FrameBuffer {
    fn from(buffer: softbuffer::Buffer<'static>) -> Self {
        FrameBuffer {
            buffer: Arc::new(Mutex::new(Some(buffer))),
        }
    }
}


// wasmtime
impl crate::wasi::webgpu::frame_buffer::Host for HostState {
    fn connect_graphics_context(
        &mut self,
        graphics_context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        todo!()
        // let context = unsafe { softbuffer::Context::new(&self.window) }.unwrap();
        // let mut surface = unsafe { softbuffer::Surface::new(&context, &self.window) }.unwrap();

        // let size = self.window.inner_size();

        // let _ = surface.resize(
        //     size.width.try_into().unwrap(),
        //     size.height.try_into().unwrap(),
        // );

        // let surface: Surface = surface.into();

        // let mut receiver = self.sender.subscribe();
        // let mut surface_clone = surface.clone();

        // tokio::spawn(async move {
        //     loop {
        //         let event = receiver.recv().await.unwrap();
        //         if let HostEvent::CanvasResizeEvent(event) = event {
        //             surface_clone.resize(
        //                 event.width.try_into().unwrap(),
        //                 event.height.try_into().unwrap(),
        //             );
        //         }
        //     }
        // });

        // let graphics_context = self.table.get_mut(&graphics_context).unwrap();
        // // graphics_context.kind = Some(GraphicsContextKind::FrameBuffer(surface));
        // graphics_context.connect_draw_api(Box::new(surface));
        // Ok(())
    }
}

impl<T: WasiView> crate::wasi::webgpu::frame_buffer::HostFrameBuffer for T {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<crate::graphics_context::GraphicsContextBuffer>,
    ) -> wasmtime::Result<Resource<FrameBuffer>> {
        let host_buffer: GraphicsContextBuffer = self.table_mut().delete(buffer).unwrap();
        let host_buffer: FrameBuffer = host_buffer.inner_type();
        Ok(self.table_mut().push(host_buffer).unwrap())
    }

    fn length(&mut self, buffer: Resource<FrameBuffer>) -> wasmtime::Result<u32> {
        let buffer = self.table().get(&buffer).unwrap();
        let len = buffer.buffer.lock().unwrap().as_ref().unwrap().len();
        Ok(len as u32)
    }

    fn get(&mut self, buffer: Resource<FrameBuffer>, i: u32) -> wasmtime::Result<u32> {
        let buffer = self.table().get(&buffer).unwrap();
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

    fn set(&mut self, buffer: Resource<FrameBuffer>, i: u32, val: u32) -> wasmtime::Result<()> {
        let buffer = self.table_mut().get_mut(&buffer).unwrap();
        buffer.buffer.lock().unwrap().as_mut().unwrap()[i as usize] = val as u32;
        Ok(())
    }

    fn drop(&mut self, frame_buffer: Resource<FrameBuffer>) -> wasmtime::Result<()> {
        let frame_buffer = self.table_mut().delete(frame_buffer).unwrap();
        frame_buffer.buffer.lock().unwrap().take();
        Ok(())
    }
}
