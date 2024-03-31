use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use wasmtime::component::Resource;
use wasmtime_wasi::preview2::WasiView;

use crate::graphics_context::{DisplayApi, DrawApi, GraphicsContext, GraphicsContextBuffer};
use crate::HostState;

// TODO: rename to FBBuffer and FBSurface?

// pub type Surface = Option<Surface>;

// #[derive(Clone)]
pub struct Surface {
    // pub(super) surface: Arc<Mutex<Option<softbuffer::Surface>>>,
    pub(super) surface: Option<softbuffer::Surface>,
}
// TODO: ensure safety
unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}
impl Surface {
    pub fn new() -> Self {
        Self { surface: None }
    }
}

// TODO: can we avoid the Mutex here?
pub struct SurfaceArc(pub Arc<Mutex<Surface>>);
impl DrawApi for SurfaceArc {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer> {
        self.0.lock().unwrap().get_current_buffer()
    }

    fn present(&mut self) -> wasmtime::Result<()> {
        self.0.lock().unwrap().present()
    }

    fn display_api_ready(&mut self, display_api: &Box<dyn DisplayApi + Send + Sync>) {
        self.0.lock().unwrap().display_api_ready(display_api)
    }
}

// impl Surface {
//     pub fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
//         self.surface.lock().unwrap().resize(width, height).unwrap();
//     }
// }

impl DrawApi for Surface {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer> {
        let surface = self.surface.as_mut().unwrap();
        let buff = surface.buffer_mut().unwrap();
        // TODO: use ouroboros?
        let buff: softbuffer::Buffer<'static> = unsafe { mem::transmute(buff) };
        let buff: FrameBuffer = buff.into();
        let buff = Box::new(buff);
        let buff: GraphicsContextBuffer = buff.into();
        Ok(buff)
    }

    fn present(&mut self) -> wasmtime::Result<()> {
        self.surface
            .as_mut()
            .unwrap()
            .buffer_mut()
            .unwrap()
            .present()
            .unwrap();
        Ok(())
    }

    fn display_api_ready(&mut self, display: &Box<dyn DisplayApi + Send + Sync>) {
        let context =
            unsafe { softbuffer::Context::from_raw(display.raw_display_handle()) }.unwrap();
        let mut surface =
            unsafe { softbuffer::Surface::from_raw(&context, display.raw_window_handle()) }
                .unwrap();

        // TODO: needed?
        let _ = surface.resize(
            display
                .width()
                .try_into()
                .unwrap_or(NonZeroU32::new(1).unwrap()),
            display
                .height()
                .try_into()
                .unwrap_or(NonZeroU32::new(1).unwrap()),
        );
        self.surface = Some(surface);
    }
}

pub struct FrameBuffer {
    // Never none
    buffer: Arc<Mutex<Option<softbuffer::Buffer<'static>>>>,
}
// TODO: ensure safety
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
impl crate::wasi::webgpu::frame_buffer::Host for HostState {}

impl crate::wasi::webgpu::frame_buffer::HostSurface for HostState {
    fn new(&mut self) -> wasmtime::Result<Resource<crate::wasi::webgpu::frame_buffer::Surface>> {
        // let surface = None;
        Ok(self
            .table_mut()
            .push(SurfaceArc(Arc::new(Mutex::new(Surface::new()))))
            .unwrap())
    }

    fn connect_graphics_context(
        &mut self,
        surface: Resource<SurfaceArc>,
        graphics_context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let surface = SurfaceArc(Arc::clone(&self.table.get(&surface).unwrap().0));
        let graphics_context = self.table.get_mut(&graphics_context).unwrap();
        graphics_context.connect_draw_api(Box::new(surface));
        Ok(())
    }

    fn drop(&mut self, _rep: Resource<SurfaceArc>) -> wasmtime::Result<()> {
        todo!()
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
