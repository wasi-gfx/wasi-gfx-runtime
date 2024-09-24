use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use raw_window_handle::{DisplayHandle, WindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

use crate::wasi::webgpu::frame_buffer;
use wasi_graphics_context_wasmtime::{AbstractBuffer, Context, DisplayApi, DrawApi};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: {
        only_imports: [],
    },
    with: {
        "wasi:webgpu/frame-buffer/device": FBDeviceArc,
        "wasi:webgpu/frame-buffer/buffer": FBBuffer,
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime,
    },
});

pub struct FBDevice {
    pub(crate) surface: Option<softbuffer::Surface<DisplayHandle<'static>, WindowHandle<'static>>>,
}
// TODO: actually ensure safety
unsafe impl Send for FBDevice {}
unsafe impl Sync for FBDevice {}
impl FBDevice {
    pub fn new() -> Self {
        Self { surface: None }
    }
}

// TODO: can we avoid the Mutex here?
pub struct FBDeviceArc(pub Arc<Mutex<FBDevice>>);
impl FBDeviceArc {
    pub fn new() -> Self {
        FBDeviceArc(Arc::new(Mutex::new(FBDevice::new())))
    }
}
impl DrawApi for FBDeviceArc {
    fn get_current_buffer(&mut self) -> wasmtime::Result<AbstractBuffer> {
        self.0.lock().unwrap().get_current_buffer()
    }

    fn present(&mut self) -> wasmtime::Result<()> {
        self.0.lock().unwrap().present()
    }

    fn display_api_ready(&mut self, display_api: &Box<dyn DisplayApi + Send + Sync>) {
        self.0.lock().unwrap().display_api_ready(display_api)
    }
}

impl DrawApi for FBDevice {
    fn get_current_buffer(&mut self) -> wasmtime::Result<AbstractBuffer> {
        let surface = self.surface.as_mut().unwrap();
        let buff = surface.buffer_mut().unwrap();
        // TODO: use ouroboros?
        let buff: softbuffer::Buffer<'static, Context, Context> = unsafe { mem::transmute(buff) };
        let buff: FBBuffer = buff.into();
        let buff = Box::new(buff);
        let buff: AbstractBuffer = buff.into();
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
    buffer: Arc<Mutex<Option<softbuffer::Buffer<'static, Context, Context>>>>,
}
// TODO: ensure safety
unsafe impl Send for FBBuffer {}
unsafe impl Sync for FBBuffer {}
impl From<softbuffer::Buffer<'static, Context, Context>> for FBBuffer {
    fn from(buffer: softbuffer::Buffer<'static, Context, Context>) -> Self {
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

impl frame_buffer::HostDevice for dyn WasiFrameBufferView + '_ {
    fn new(&mut self) -> Resource<crate::wasi::webgpu::frame_buffer::Device> {
        self.table().push(FBDeviceArc::new()).unwrap()
    }

    fn connect_graphics_context(
        &mut self,
        surface: Resource<FBDeviceArc>,
        graphics_context: Resource<Context>,
    ) {
        let surface = FBDeviceArc(Arc::clone(&self.table().get(&surface).unwrap().0));
        let graphics_context = self.table().get_mut(&graphics_context).unwrap();
        graphics_context.connect_draw_api(Box::new(surface));
    }

    fn drop(&mut self, _rep: Resource<FBDeviceArc>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl frame_buffer::HostBuffer for dyn WasiFrameBufferView + '_ {
    fn from_graphics_buffer(&mut self, buffer: Resource<AbstractBuffer>) -> Resource<FBBuffer> {
        let host_buffer: AbstractBuffer = self.table().delete(buffer).unwrap();
        let host_buffer: FBBuffer = host_buffer.inner_type();
        self.table().push(host_buffer).unwrap()
    }

    fn get(&mut self, buffer: Resource<FBBuffer>) -> Vec<u8> {
        let buffer = self.table().get(&buffer).unwrap();
        let buffer = buffer.buffer.lock().unwrap();
        let buffer = buffer.as_ref().unwrap();
        let buffer = bytemuck::try_cast_slice(buffer).unwrap();
        buffer.to_vec()
    }

    fn set(&mut self, buffer: Resource<FBBuffer>, val: Vec<u8>) {
        let buffer = self.table().get_mut(&buffer).unwrap();
        let val = bytemuck::try_cast_slice(&val).unwrap();
        buffer
            .buffer
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .copy_from_slice(&val);
    }

    fn drop(&mut self, frame_buffer: Resource<FBBuffer>) -> wasmtime::Result<()> {
        let frame_buffer = self.table().delete(frame_buffer).unwrap();
        frame_buffer.buffer.lock().unwrap().take();
        Ok(())
    }
}
