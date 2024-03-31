use std::sync::{Arc, Mutex};

use crate::{
    graphics_context::DisplayApi,
    wasi::webgpu::mini_canvas::{CreateDesc, GraphicsContext, Pollable, ResizeEvent},
    HostState,
};
use async_broadcast::Receiver;
use futures::executor::block_on;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::window::{Window, WindowId};

#[derive(Debug)]
pub struct MiniCanvas {
    // pub height: u32,
    // pub width: u32,
    pub offscreen: bool,
    // let event_loop: EventLoop<()>
    pub window: Window,
}
// impl AsRef<MiniCanvas> for Weak<MiniCanvas> {
//     fn as_ref(&self) -> &MiniCanvas {
//         self.upgrade().unwrap().as_ref()
//     }
// }

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

// TODO: instead of Arc, have a global list of windows and ids? That ways it's same as webgpu, but might be harder to handle? Would likely also require a Mutex.
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

impl crate::wasi::webgpu::mini_canvas::Host for HostState {}

impl crate::wasi::webgpu::mini_canvas::HostMiniCanvas for HostState {
    fn new(&mut self, desc: CreateDesc) -> wasmtime::Result<Resource<MiniCanvasArc>> {
        let window = block_on(self.message_sender.create_window());
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
        // noop for now, will do the surface creation once window is part min-canvas instead of a singleton.
        Ok(())
    }

    fn resize_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<ResizeListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .message_sender
            .receivers
            .canvas_resize_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(ResizeListener {
                window_id,
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

#[derive(Debug)]
pub struct ResizeListener {
    window_id: WindowId,
    receiver: Receiver<(WindowId, ResizeEvent)>,
    data: Mutex<Option<ResizeEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for ResizeListener {
    async fn ready(&mut self) {
        loop {
            let (window_id, event) = self.receiver.recv().await.unwrap();
            if window_id == self.window_id {
                *self.data.lock().unwrap() = Some(event);
                return;
            }
        }
    }
}

impl crate::wasi::webgpu::mini_canvas::HostResizeListener for HostState {
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
