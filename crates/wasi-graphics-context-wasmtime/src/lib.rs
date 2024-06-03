use std::any::Any;

use crate::wasi::webgpu::graphics_context::{self, ConfigureContextDesc};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: false,
    with: {
        "wasi:webgpu/graphics-context/graphics-context": GraphicsContext,
        "wasi:webgpu/graphics-context/graphics-context-buffer": GraphicsContextBuffer,
    },
});

pub struct GraphicsContext {
    draw_api: Option<Box<dyn DrawApi + Send + Sync>>,
    display_api: Option<Box<dyn DisplayApi + Send + Sync>>,
}

impl GraphicsContext {
    pub fn new() -> Self {
        Self {
            display_api: None,
            draw_api: None,
        }
    }

    pub fn configure(&mut self, _desc: ConfigureContextDesc) -> wasmtime::Result<()> {
        Ok(())
    }

    pub fn connect_display_api(&mut self, display_api: Box<dyn DisplayApi + Send + Sync>) {
        if let Some(draw_api) = &mut self.draw_api {
            draw_api.display_api_ready(&display_api)
        }
        self.display_api = Some(display_api);
    }

    // pub fn resize(&mut self, height: u32, width: u32) {
    //     self.height = Some(height);
    //     self.width = Some(width);
    // }

    pub fn connect_draw_api(&mut self, mut draw_api: Box<dyn DrawApi + Send + Sync>) {
        if let Some(display_api) = &self.display_api {
            draw_api.display_api_ready(&*display_api)
        }
        self.draw_api = Some(draw_api);
    }
}

impl HasDisplayHandle for GraphicsContext {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match &self.display_api {
            Some(display_api) => display_api.display_handle(),
            None => Err(HandleError::Unavailable),
        }
    }
}

impl HasWindowHandle for GraphicsContext {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match &self.display_api {
            Some(display_api) => display_api.window_handle(),
            None => Err(HandleError::Unavailable),
        }
    }
}

// TODO: rename to FrameProvider? since this isn't necessarily implemented on the whole api?
pub trait DrawApi {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer>;
    fn present(&mut self) -> wasmtime::Result<()>;
    fn display_api_ready(&mut self, display_api: &Box<dyn DisplayApi + Send + Sync>);
}

pub trait DisplayApi: HasDisplayHandle + HasWindowHandle {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

pub struct GraphicsContextBuffer {
    buffer: Box<dyn Any + Send + Sync>,
}
impl<T> From<Box<T>> for GraphicsContextBuffer
where
    T: Any + Send + Sync + 'static,
{
    fn from(value: Box<T>) -> Self {
        Self {
            buffer: Box::new(value),
        }
    }
}

impl GraphicsContextBuffer {
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
        F: Fn(&mut T) -> &mut dyn WasiGraphicsContextView,
    {
        val
    }
    let closure = type_annotate::<T, _>(|t| t);
    wasi::webgpu::graphics_context::add_to_linker_get_host(l, closure)?;
    Ok(())
}

pub trait WasiGraphicsContextView: WasiView {}

impl graphics_context::Host for dyn WasiGraphicsContextView + '_ {}

impl graphics_context::HostGraphicsContext for dyn WasiGraphicsContextView + '_ {
    fn new(&mut self) -> Resource<GraphicsContext> {
        self.table().push(GraphicsContext::new()).unwrap()
    }

    fn configure(&mut self, context: Resource<GraphicsContext>, desc: ConfigureContextDesc) {
        let graphics_context = self.table().get_mut(&context).unwrap();
        graphics_context.configure(desc).unwrap();
    }

    fn get_current_buffer(
        &mut self,
        context: Resource<GraphicsContext>,
    ) -> Resource<GraphicsContextBuffer> {
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

    fn present(&mut self, context: Resource<GraphicsContext>) {
        let context = self.table().get_mut(&context).unwrap();
        // context.display_api.as_mut().unwrap().present().unwrap();
        context.draw_api.as_mut().unwrap().present().unwrap();
    }

    fn drop(&mut self, _graphics_context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
    }
}

impl graphics_context::HostGraphicsContextBuffer for dyn WasiGraphicsContextView + '_ {
    fn drop(&mut self, _rep: Resource<GraphicsContextBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
