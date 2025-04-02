use std::any::Any;

use crate::wasi::graphics_context::graphics_context;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wasmtime::component::Resource;
use wasmtime_wasi::{IoView, ResourceTable};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: false,
    with: {
        "wasi:graphics-context/graphics-context/context": Context,
        "wasi:graphics-context/graphics-context/abstract-buffer": AbstractBuffer,
    },
});

pub struct Context {
    draw_api: Option<Box<dyn DrawApi + Send + Sync>>,
    display_api: Option<Box<dyn DisplayApi + Send + Sync>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            display_api: None,
            draw_api: None,
        }
    }

    pub fn connect_display_api(&mut self, display_api: Box<dyn DisplayApi + Send + Sync>) {
        if let Some(draw_api) = &mut self.draw_api {
            draw_api.display_api_ready(&display_api)
        }
        self.display_api = Some(display_api);
    }

    pub fn connect_draw_api(&mut self, mut draw_api: Box<dyn DrawApi + Send + Sync>) {
        if let Some(display_api) = &self.display_api {
            draw_api.display_api_ready(&*display_api)
        }
        self.draw_api = Some(draw_api);
    }
}

impl HasDisplayHandle for Context {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match &self.display_api {
            Some(display_api) => display_api.display_handle(),
            None => Err(HandleError::Unavailable),
        }
    }
}

impl HasWindowHandle for Context {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match &self.display_api {
            Some(display_api) => display_api.window_handle(),
            None => Err(HandleError::Unavailable),
        }
    }
}

// TODO: rename to FrameProvider? since this isn't necessarily implemented on the whole api?
pub trait DrawApi {
    fn get_current_buffer(&mut self) -> wasmtime::Result<AbstractBuffer>;
    fn present(&mut self) -> wasmtime::Result<()>;
    fn display_api_ready(&mut self, display_api: &Box<dyn DisplayApi + Send + Sync>);
}

pub trait DisplayApi: HasDisplayHandle + HasWindowHandle {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn request_set_size(&self, width: Option<u32>, height: Option<u32>);
}

pub struct AbstractBuffer {
    buffer: Box<dyn Any + Send + Sync>,
}
impl<T> From<Box<T>> for AbstractBuffer
where
    T: Any + Send + Sync + 'static,
{
    fn from(value: Box<T>) -> Self {
        Self {
            buffer: Box::new(value),
        }
    }
}

impl AbstractBuffer {
    pub fn inner_type<T>(self) -> T
    where
        T: 'static,
    {
        **self.buffer.downcast::<Box<T>>().unwrap()
    }
}

// wasmtime
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiGraphicsContextView,
{
    fn type_annotate<T, F>(val: F) -> F
    where
        T: WasiGraphicsContextView,
        F: Fn(&mut T) -> WasiGraphicsContextImpl<&mut T>,
    {
        val
    }
    let closure = type_annotate::<T, _>(|t| WasiGraphicsContextImpl(t));
    wasi::graphics_context::graphics_context::add_to_linker_get_host(l, closure)?;
    Ok(())
}

pub trait WasiGraphicsContextView: IoView {}

#[repr(transparent)]
pub struct WasiGraphicsContextImpl<T: WasiGraphicsContextView>(pub T);
impl<T: WasiGraphicsContextView> IoView for WasiGraphicsContextImpl<T> {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(&mut self.0)
    }
}

impl<T: ?Sized + WasiGraphicsContextView> WasiGraphicsContextView for &mut T {}
impl<T: ?Sized + WasiGraphicsContextView> WasiGraphicsContextView for Box<T> {}
impl<T: WasiGraphicsContextView> WasiGraphicsContextView for WasiGraphicsContextImpl<T> {}

impl<T: WasiGraphicsContextView> graphics_context::Host for WasiGraphicsContextImpl<T> {}

impl<T: WasiGraphicsContextView> graphics_context::HostContext for WasiGraphicsContextImpl<T> {
    fn new(&mut self) -> Resource<Context> {
        self.table().push(Context::new()).unwrap()
    }

    fn get_current_buffer(&mut self, context: Resource<Context>) -> Resource<AbstractBuffer> {
        let context_kind = self.table().get_mut(&context).unwrap();
        let next_frame = context_kind
            .draw_api
            .as_mut()
            .expect("draw_api not set")
            .get_current_buffer()
            .unwrap();
        let next_frame = self.table().push(next_frame).unwrap();
        next_frame
    }

    fn present(&mut self, context: Resource<Context>) {
        let context = self.table().get_mut(&context).unwrap();
        // context.display_api.as_mut().unwrap().present().unwrap();
        context.draw_api.as_mut().unwrap().present().unwrap();
    }

    fn drop(&mut self, _graphics_context: Resource<Context>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
    }
}

impl<T: WasiGraphicsContextView> graphics_context::HostAbstractBuffer
    for WasiGraphicsContextImpl<T>
{
    fn drop(&mut self, _rep: Resource<AbstractBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
