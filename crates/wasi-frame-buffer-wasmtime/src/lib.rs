use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use raw_window_handle::{DisplayHandle, WindowHandle};
use wasmtime::component::{HasData, Resource};
use wasmtime_wasi_io::IoView;

use crate::wasi::frame_buffer::frame_buffer;
use wasi_graphics_context_wasmtime::{AbstractBuffer, Context, DisplayApi, DrawApi};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    with: {
        "wasi:frame-buffer/frame-buffer/device": FBDeviceArc,
        "wasi:frame-buffer/frame-buffer/buffer": FBBuffer,
        "wasi:graphics-context/graphics-context": wasi_graphics_context_wasmtime::wasi::graphics_context::graphics_context,
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
struct WasiFrameBuffer<T: Send>(T);
impl<T: Send + 'static> HasData for WasiFrameBuffer<T> {
    type Data<'a> = WasiFrameBufferImpl<&'a mut T>;
}
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiFrameBufferView,
{
    wasi::frame_buffer::frame_buffer::add_to_linker::<_, WasiFrameBuffer<T>>(l, |x| {
        WasiFrameBufferImpl(x)
    })?;
    Ok(())
}

pub trait WasiFrameBufferView: IoView + Send {}

#[repr(transparent)]
pub struct WasiFrameBufferImpl<T>(pub T);
impl<T: WasiFrameBufferView> IoView for WasiFrameBufferImpl<T> {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        T::table(&mut self.0)
    }
}

impl<T: ?Sized + WasiFrameBufferView> WasiFrameBufferView for &mut T {}
impl<T: ?Sized + WasiFrameBufferView> WasiFrameBufferView for Box<T> {}
impl<T: WasiFrameBufferView> WasiFrameBufferView for WasiFrameBufferImpl<T> {}

impl<T: WasiFrameBufferView> frame_buffer::Host for WasiFrameBufferImpl<T> {}

impl<T: WasiFrameBufferView> frame_buffer::HostDevice for WasiFrameBufferImpl<T> {
    fn new(&mut self) -> Resource<crate::wasi::frame_buffer::frame_buffer::Device> {
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

impl<T: WasiFrameBufferView> frame_buffer::HostBuffer for WasiFrameBufferImpl<T> {
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
        let mut guard = buffer.buffer.lock().unwrap();
        if let Some(dest) = guard.as_mut() {
            if dest.len() == val.len() {
                dest.copy_from_slice(&val);
                println!(
                    "set: buffer size correct: dest={}, src={}",
                    dest.len(),
                    val.len()
                );
            } else {
                println!(
                    "set: buffer size mismatch: dest={}, src={}",
                    dest.len(),
                    val.len()
                );
            }
        }
        //buffer
        //    .buffer
        //    .lock()
        //    .unwrap()
        //    .as_mut()
        //    .unwrap()
        //    .copy_from_slice(&val);
    }

    fn drop(&mut self, frame_buffer: Resource<FBBuffer>) -> wasmtime::Result<()> {
        let frame_buffer = self.table().delete(frame_buffer).unwrap();
        frame_buffer.buffer.lock().unwrap().take();
        Ok(())
    }
}
