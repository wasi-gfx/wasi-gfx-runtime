use std::mem;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use raw_window_handle::{DisplayHandle, WindowHandle};
use wasmtime_wasi::{ResourceTable, WasiView};

use wasi_graphics_context_wasmtime::{AbstractBuffer, Context, DisplayApi, DrawApi};

// Re-export for use in macros
pub use bytemuck;

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
impl Default for FBDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl FBDevice {
    pub fn new() -> Self {
        Self { surface: None }
    }
}

// TODO: can we avoid the Mutex here?
pub struct FBDeviceArc(pub Arc<Mutex<FBDevice>>);
impl Default for FBDeviceArc {
    fn default() -> Self {
        Self::new()
    }
}

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
    pub buffer: Arc<Mutex<Option<softbuffer::Buffer<'static, Context, Context>>>>,
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

// Helper trait for implementing frame buffer functionality
pub trait WasiFrameBufferView: Send {
    fn table(&mut self) -> &mut ResourceTable;
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_>;
}

#[repr(transparent)]
pub struct WasiFrameBufferImpl<T: WasiFrameBufferView>(pub T);

impl<T: WasiFrameBufferView + 'static> wasmtime::component::HasData for WasiFrameBufferImpl<T> {
    type Data<'a> = WasiFrameBufferImpl<&'a mut T>;
}

impl<T: WasiFrameBufferView> WasiView for WasiFrameBufferImpl<T> {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(&mut self.0)
    }
}

impl<T: WasiFrameBufferView> WasiFrameBufferView for WasiFrameBufferImpl<T> {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(&mut self.0)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(&mut self.0)
    }
}

impl<T: ?Sized + WasiFrameBufferView> WasiFrameBufferView for &mut T {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(self)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(self)
    }
}

// Implement Host trait for the wrapped type
impl<T: WasiFrameBufferView> wasi::frame_buffer::frame_buffer::Host for WasiFrameBufferImpl<T> {}

impl<T: WasiFrameBufferView> wasi::frame_buffer::frame_buffer::HostDevice
    for WasiFrameBufferImpl<T>
{
    fn new(&mut self) -> wasmtime::component::Resource<FBDeviceArc> {
        WasiFrameBufferView::table(&mut self.0)
            .push(FBDeviceArc::new())
            .expect("failed to push frame buffer device to resource table")
    }

    fn connect_graphics_context(
        &mut self,
        surface: wasmtime::component::Resource<FBDeviceArc>,
        graphics_context: wasmtime::component::Resource<wasi_graphics_context_wasmtime::Context>,
    ) {
        let surface = FBDeviceArc(std::sync::Arc::clone(
            &WasiFrameBufferView::table(&mut self.0)
                .get(&surface)
                .expect("invalid frame buffer device resource")
                .0,
        ));
        let graphics_context = WasiFrameBufferView::table(&mut self.0)
            .get_mut(&graphics_context)
            .expect("invalid graphics context resource");
        graphics_context.connect_draw_api(Box::new(surface));
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<FBDeviceArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiFrameBufferView> wasi::frame_buffer::frame_buffer::HostBuffer
    for WasiFrameBufferImpl<T>
{
    fn from_graphics_buffer(
        &mut self,
        buffer: wasmtime::component::Resource<wasi_graphics_context_wasmtime::AbstractBuffer>,
    ) -> wasmtime::component::Resource<FBBuffer> {
        let host_buffer: wasi_graphics_context_wasmtime::AbstractBuffer =
            WasiFrameBufferView::table(&mut self.0)
                .delete(buffer)
                .expect("failed to delete abstract buffer from resource table");
        let host_buffer: FBBuffer = host_buffer.inner_type();
        WasiFrameBufferView::table(&mut self.0)
            .push(host_buffer)
            .expect("failed to push frame buffer to resource table")
    }

    fn get(&mut self, buffer: wasmtime::component::Resource<FBBuffer>) -> Vec<u8> {
        let buffer = WasiFrameBufferView::table(&mut self.0)
            .get(&buffer)
            .expect("invalid frame buffer resource");
        let buffer = buffer.buffer.lock().expect("failed to acquire lock");
        let buffer = buffer.as_ref().expect("buffer is None");
        let buffer = bytemuck::try_cast_slice(buffer).expect("failed to cast buffer to u8 slice");
        buffer.to_vec()
    }

    fn set(&mut self, buffer: wasmtime::component::Resource<FBBuffer>, val: Vec<u8>) {
        let buffer = WasiFrameBufferView::table(&mut self.0)
            .get_mut(&buffer)
            .expect("invalid frame buffer resource");
        let val = bytemuck::try_cast_slice(&val).expect("failed to cast u8 slice to buffer format");
        buffer
            .buffer
            .lock()
            .expect("failed to acquire lock")
            .as_mut()
            .expect("buffer is None")
            .copy_from_slice(val);
    }

    fn drop(
        &mut self,
        frame_buffer: wasmtime::component::Resource<FBBuffer>,
    ) -> wasmtime::Result<()> {
        let frame_buffer = WasiFrameBufferView::table(&mut self.0)
            .delete(frame_buffer)
            .map_err(|e| wasmtime::Error::msg(format!("failed to delete frame buffer: {}", e)))?;
        frame_buffer
            .buffer
            .lock()
            .expect("failed to acquire lock")
            .take();
        Ok(())
    }
}

// Add to linker helper
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiFrameBufferView + 'static,
{
    fn get_impl<T: WasiFrameBufferView>(t: &mut T) -> WasiFrameBufferImpl<&mut T> {
        WasiFrameBufferImpl(t)
    }
    wasi::frame_buffer::frame_buffer::add_to_linker::<T, WasiFrameBufferImpl<T>>(l, get_impl)?;
    Ok(())
}
