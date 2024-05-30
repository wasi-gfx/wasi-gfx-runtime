use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use raw_window_handle::{DisplayHandle, WindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::wasi::webgpu::frame_buffer;
use wasi_graphics_context_wasmtime::{DisplayApi, DrawApi, GraphicsContext, GraphicsContextBuffer};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: {
        only_imports: [],
    },
    with: {
        "wasi:webgpu/frame-buffer/surface": FBSurfaceArc,
        "wasi:webgpu/frame-buffer/frame-buffer": FBBuffer,
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime,
    },
});

pub struct FBSurface {
    pub(crate) surface: Option<softbuffer::Surface<DisplayHandle<'static>, WindowHandle<'static>>>,
}
// TODO: actually ensure safety
unsafe impl Send for FBSurface {}
unsafe impl Sync for FBSurface {}
impl FBSurface {
    pub fn new() -> Self {
        Self { surface: None }
    }
}

// TODO: can we avoid the Mutex here?
pub struct FBSurfaceArc(pub Arc<Mutex<FBSurface>>);
impl FBSurfaceArc {
    pub fn new() -> Self {
        FBSurfaceArc(Arc::new(Mutex::new(FBSurface::new())))
    }
}
impl DrawApi for FBSurfaceArc {
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

impl DrawApi for FBSurface {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer> {
        let surface = self.surface.as_mut().unwrap();
        let buff = surface.buffer_mut().unwrap();
        // TODO: use ouroboros?
        let buff: softbuffer::Buffer<'static, GraphicsContext, GraphicsContext> =
            unsafe { mem::transmute(buff) };
        let buff: FBBuffer = buff.into();
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
        let context = softbuffer::Context::new(display.display_handle().unwrap()).unwrap();
        let surface = softbuffer::Surface::new(&context, display.window_handle().unwrap()).unwrap();

        // TODO: use ouroboros?
        let mut surface: softbuffer::Surface<DisplayHandle<'static>, WindowHandle<'static>> =
            unsafe { mem::transmute(surface) };

        // softbuffer requires setting the size before presenting.
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

pub struct FBBuffer {
    // Never none
    buffer: Arc<Mutex<Option<softbuffer::Buffer<'static, GraphicsContext, GraphicsContext>>>>,
}
// TODO: ensure safety
unsafe impl Send for FBBuffer {}
unsafe impl Sync for FBBuffer {}
impl From<softbuffer::Buffer<'static, GraphicsContext, GraphicsContext>> for FBBuffer {
    fn from(buffer: softbuffer::Buffer<'static, GraphicsContext, GraphicsContext>) -> Self {
        FBBuffer {
            buffer: Arc::new(Mutex::new(Some(buffer))),
        }
    }
}

// wasmtime
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiFrameBufferView,
{
    fn type_annotate<T, F>(val: F) -> F
    where
        F: Fn(&mut T) -> &mut dyn WasiFrameBufferView,
    {
        val
    }
    let closure = type_annotate::<T, _>(|t| t);
    wasi::webgpu::frame_buffer::add_to_linker_get_host(l, closure)?;
    Ok(())
}

pub trait WasiFrameBufferView: WasiView {}

impl frame_buffer::Host for dyn WasiFrameBufferView + '_ {}

impl frame_buffer::HostSurface for dyn WasiFrameBufferView + '_ {
    fn new(&mut self) -> Resource<crate::wasi::webgpu::frame_buffer::Surface> {
        self.table().push(FBSurfaceArc::new()).unwrap()
    }

    fn connect_graphics_context(
        &mut self,
        surface: Resource<FBSurfaceArc>,
        graphics_context: Resource<GraphicsContext>,
    ) {
        let surface = FBSurfaceArc(Arc::clone(&self.table().get(&surface).unwrap().0));
        let graphics_context = self.table().get_mut(&graphics_context).unwrap();
        graphics_context.connect_draw_api(Box::new(surface));
    }

    fn drop(&mut self, _rep: Resource<FBSurfaceArc>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl frame_buffer::HostFrameBuffer for dyn WasiFrameBufferView + '_ {
    fn from_graphics_buffer(
        &mut self,
        buffer: Resource<GraphicsContextBuffer>,
    ) -> Resource<FBBuffer> {
        let host_buffer: GraphicsContextBuffer = self.table().delete(buffer).unwrap();
        let host_buffer: FBBuffer = host_buffer.inner_type();
        self.table().push(host_buffer).unwrap()
    }

    fn length(&mut self, buffer: Resource<FBBuffer>) -> u32 {
        let buffer = self.table().get(&buffer).unwrap();
        let len = buffer.buffer.lock().unwrap().as_ref().unwrap().len();
        len as u32
    }

    fn get(&mut self, buffer: Resource<FBBuffer>, i: u32) -> u32 {
        let buffer = self.table().get(&buffer).unwrap();
        *buffer
            .buffer
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .get(i as usize)
            .unwrap()
    }

    fn set(&mut self, buffer: Resource<FBBuffer>, i: u32, val: u32) {
        let buffer = self.table().get_mut(&buffer).unwrap();
        buffer.buffer.lock().unwrap().as_mut().unwrap()[i as usize] = val as u32;
    }

    fn drop(&mut self, frame_buffer: Resource<FBBuffer>) -> wasmtime::Result<()> {
        let frame_buffer = self.table().delete(frame_buffer).unwrap();
        frame_buffer.buffer.lock().unwrap().take();
        Ok(())
    }
}
