use std::any::Any;

use crate::wasi::webgpu::graphics_context::ConfigureContextDesc;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::WasiView;

pub struct GraphicsContext {
    draw_api: Option<Box<dyn DrawApi + Send + Sync>>,
    display_api: Option<Box<dyn DisplayApi + Send + Sync>>,
    // has_display_handle: Option<Box<dyn HasDisplayHandle + Send + Sync>>,
    // has_window_handle: Option<Box<dyn HasWindowHandle + Send + Sync>>,
}

impl GraphicsContext {
    pub fn new() -> Self {
        Self {
            display_api: None,
            draw_api: None,
            // has_display_handle: None,
            // has_window_handle: None,
            // height: None,
            // width: None,
        }
    }

    pub fn configure(&mut self, _desc: ConfigureContextDesc) -> wasmtime::Result<()> {
        Ok(())
    }

    pub fn connect_display_api(
        &mut self,
        display_api: Box<dyn DisplayApi + Send + Sync>,
        // has_display_handle: Box<dyn HasRawDisplayHandle + Send + Sync>,
        // has_window_handle: Box<dyn HasRawWindowHandle + Send + Sync>,
        // height: u32,
        // width: u32,
    ) {
        // self.has_display_handle = Some(has_display_handle);
        // self.has_window_handle = Some(has_window_handle);
        // self.height = Some(height);
        // self.width = Some(width);

        if let Some(draw_api) = &mut self.draw_api {
            draw_api.display_api_ready(&display_api)
        }
        self.display_api = Some(display_api);
        // self.draw_api = Some(draw_api);
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

// TODO: rename to FrameProvider? since this isn't neceraly implemented on the whole api?
pub trait DrawApi {
    fn get_current_buffer(&mut self) -> wasmtime::Result<GraphicsContextBuffer>;
    fn present(&mut self) -> wasmtime::Result<()>;
    fn display_api_ready(&mut self, display_api: &Box<dyn DisplayApi + Send + Sync>);
}

pub trait DisplayApi: HasRawDisplayHandle + HasRawWindowHandle {
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
    // impl<T> From<T> for GraphicsContextBuffer
    // where
    //     T: Any + Send + Sync,
    // {
    // fn new<T>(value: Sized + Any + Send + Sync) -> Self
    // where
    //     T: Any + Send + Sync,
    // {
    //     Self { buffer: Box::new(value) }
    // }
    // }

    pub fn inner_type<T>(self) -> T
    where
        T: 'static,
    {
        // double box?
        *self.buffer.downcast::<T>().unwrap()
    }
}

// wasmtime
impl<T: WasiView> crate::wasi::webgpu::graphics_context::Host for T {}

impl<T: WasiView> crate::wasi::webgpu::graphics_context::HostGraphicsContext for T {
    fn new(&mut self) -> wasmtime::Result<Resource<GraphicsContext>> {
        Ok(self.table_mut().push(GraphicsContext::new()).unwrap())
    }

    fn configure(
        &mut self,
        context: Resource<GraphicsContext>,
        desc: ConfigureContextDesc,
    ) -> wasmtime::Result<()> {
        let graphics_context = self.table_mut().get_mut(&context).unwrap();
        graphics_context.configure(desc).unwrap();
        Ok(())
    }

    fn get_current_buffer(
        &mut self,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<Resource<GraphicsContextBuffer>> {
        let context_kind = self.table_mut().get_mut(&context).unwrap();
        let next_frame = context_kind
            .draw_api
            .as_mut()
            .unwrap()
            .get_current_buffer()
            .unwrap();
        let next_frame = self.table_mut().push(next_frame).unwrap();
        Ok(next_frame)
    }

    fn present(&mut self, context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        let context = self.table_mut().get_mut(&context).unwrap();
        // context.display_api.as_mut().unwrap().present().unwrap();
        context.draw_api.as_mut().unwrap().present().unwrap();
        Ok(())
    }

    fn drop(&mut self, _graphics_context: Resource<GraphicsContext>) -> wasmtime::Result<()> {
        // todo!()
        Ok(())
    }
}

impl<T: WasiView> crate::wasi::webgpu::graphics_context::HostGraphicsContextBuffer for T {
    fn drop(&mut self, _rep: Resource<GraphicsContextBuffer>) -> wasmtime::Result<()> {
        todo!()
    }
}
