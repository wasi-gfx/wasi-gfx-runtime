use std::any::Any;

use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wasmtime_wasi::ResourceTable;

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    with: {
        "wasi:graphics-context/graphics-context@0.0.1/context": Context,
        "wasi:graphics-context/graphics-context@0.0.1/abstract-buffer": AbstractBuffer,
    },
});

pub struct Context {
    pub draw_api: Option<Box<dyn DrawApi + Send + Sync>>,
    pub display_api: Option<Box<dyn DisplayApi + Send + Sync>>,
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
            draw_api.display_api_ready(display_api)
        }
        self.draw_api = Some(draw_api);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
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

// Helper trait for implementing graphics context functionality
pub trait WasiGraphicsContextView: Send {
    fn table(&mut self) -> &mut ResourceTable;
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_>;
}

#[repr(transparent)]
pub struct WasiGraphicsContextImpl<T: WasiGraphicsContextView>(pub T);

impl<T: WasiGraphicsContextView + 'static> wasmtime::component::HasData
    for WasiGraphicsContextImpl<T>
{
    type Data<'a> = WasiGraphicsContextImpl<&'a mut T>;
}

impl<T: WasiGraphicsContextView> WasiGraphicsContextView for WasiGraphicsContextImpl<T> {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(&mut self.0)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(&mut self.0)
    }
}

// Blanket impl for &mut T - required for the get_impl function in add_to_linker
impl<T: ?Sized + WasiGraphicsContextView> WasiGraphicsContextView for &mut T {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(self)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(self)
    }
}

// Forward Host trait implementations to the wrapped type
impl<T: WasiGraphicsContextView> wasi::graphics_context::graphics_context::Host
    for WasiGraphicsContextImpl<T>
{
}

impl<T: WasiGraphicsContextView> wasi::graphics_context::graphics_context::HostContext
    for WasiGraphicsContextImpl<T>
{
    fn new(&mut self) -> wasmtime::component::Resource<Context> {
        WasiGraphicsContextView::table(&mut self.0)
            .push(Context::new())
            .expect("failed to push graphics context to resource table")
    }

    fn get_current_buffer(
        &mut self,
        context: wasmtime::component::Resource<Context>,
    ) -> wasmtime::component::Resource<AbstractBuffer> {
        let context_kind = WasiGraphicsContextView::table(&mut self.0)
            .get_mut(&context)
            .expect("invalid graphics context resource");
        let next_frame = context_kind
            .draw_api
            .as_mut()
            .expect("draw_api not set - must call connect_draw_api first")
            .get_current_buffer()
            .expect("failed to get current buffer from draw API");
        WasiGraphicsContextView::table(&mut self.0)
            .push(next_frame)
            .expect("failed to push abstract buffer to resource table")
    }

    fn present(&mut self, context: wasmtime::component::Resource<Context>) {
        let context = WasiGraphicsContextView::table(&mut self.0)
            .get_mut(&context)
            .expect("invalid graphics context resource");
        context
            .draw_api
            .as_mut()
            .expect("draw_api not set - must call connect_draw_api first")
            .present()
            .expect("failed to present frame");
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<Context>) -> wasmtime::Result<()> {
        WasiGraphicsContextView::table(&mut self.0)
            .delete(rep)
            .map_err(|e| {
                wasmtime::Error::msg(format!("failed to delete graphics context: {}", e))
            })?;
        Ok(())
    }
}

impl<T: WasiGraphicsContextView> wasi::graphics_context::graphics_context::HostAbstractBuffer
    for WasiGraphicsContextImpl<T>
{
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<AbstractBuffer>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

// Add to linker helper
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiGraphicsContextView + 'static,
{
    fn get_impl<T: WasiGraphicsContextView>(t: &mut T) -> WasiGraphicsContextImpl<&mut T> {
        WasiGraphicsContextImpl(t)
    }
    wasi::graphics_context::graphics_context::add_to_linker::<T, WasiGraphicsContextImpl<T>>(
        l, get_impl,
    )?;
    Ok(())
}
