use std::sync::{Arc, Mutex};

use crate::{
    graphics_context::DisplayApi,
    wasi::webgpu::mini_canvas::{self, CreateDesc, GraphicsContext, Pollable, ResizeEvent},
    HostState,
};
use async_broadcast::Receiver;
use futures::executor::block_on;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::window::Window;

#[derive(Debug)]
pub struct MiniCanvas {
    pub offscreen: bool,
    pub window: Window,
}

unsafe impl HasRawDisplayHandle for MiniCanvas {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}
unsafe impl HasRawWindowHandle for MiniCanvas {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl DisplayApi for MiniCanvas {
    fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }
}

// TODO: instead of Arc, maybe have a global list of windows and ids? That ways it's same as webgpu, but might be harder to handle? Would likely also require a Mutex.
#[derive(Clone)]
pub struct MiniCanvasArc(pub Arc<MiniCanvas>);

unsafe impl HasRawDisplayHandle for MiniCanvasArc {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.0.raw_display_handle()
    }
}
unsafe impl HasRawWindowHandle for MiniCanvasArc {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.raw_window_handle()
    }
}

impl DisplayApi for MiniCanvasArc {
    fn height(&self) -> u32 {
        self.0.height()
    }

    fn width(&self) -> u32 {
        self.0.width()
    }
}

#[derive(Debug)]
pub struct ResizeListener {
    receiver: Receiver<ResizeEvent>,
    data: Mutex<Option<ResizeEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for ResizeListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

// wasmtime
impl mini_canvas::Host for HostState {}

#[async_trait::async_trait]
impl mini_canvas::HostMiniCanvas for HostState {
    fn new(&mut self, desc: CreateDesc) -> wasmtime::Result<Resource<MiniCanvasArc>> {
        let window = block_on(self.main_thread_proxy.create_window());
        let mini_canvas = MiniCanvasArc(Arc::new(MiniCanvas {
            offscreen: desc.offscreen,
            window,
        }));
        Ok(self.table.push(mini_canvas).unwrap())
    }

    fn connect_graphics_context(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let mini_canvas = self.table.get(&mini_canvas).unwrap().clone();
        let graphics_context = self.table.get_mut(&context).unwrap();

        graphics_context.connect_display_api(Box::new(mini_canvas));
        Ok(())
    }

    fn resize_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<ResizeListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        // TODO: await instead of block_on
        let receiver = block_on(
            self.main_thread_proxy
                .create_canvas_resize_listener(window_id),
        );
        Ok(self
            .table_mut()
            .push(ResizeListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    fn height(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> wasmtime::Result<u32> {
        let mini_canvas = self.table.get(&mini_canvas).unwrap();
        Ok(mini_canvas.height())
    }

    fn width(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> wasmtime::Result<u32> {
        let mini_canvas = self.table.get(&mini_canvas).unwrap();
        Ok(mini_canvas.width())
    }

    fn drop(&mut self, _self_: Resource<MiniCanvasArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl mini_canvas::HostResizeListener for HostState {
    fn subscribe(
        &mut self,
        pointer_down: Resource<ResizeListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_down).unwrap())
    }
    fn get(
        &mut self,
        pointer_down: Resource<ResizeListener>,
    ) -> wasmtime::Result<Option<ResizeEvent>> {
        let pointer_down = self.table.get(&pointer_down).unwrap();
        Ok(pointer_down.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<ResizeListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}
