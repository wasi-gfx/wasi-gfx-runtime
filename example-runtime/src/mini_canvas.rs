use std::{ops::Deref, sync::{Arc, Mutex, Weak}};

use crate::{
    graphics_context::DisplayApi, wasi::webgpu::mini_canvas::{CreateDesc, GraphicsContext, Pollable, ResizeEvent}, HostEvent, HostState
};
use futures::executor::block_on;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::{event_loop::{self, EventLoop}, platform::x11::EventLoopBuilderExtX11, window::Window};

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


// TODO: do we need weak refs?
#[derive(Clone)]
pub struct MiniCanvasArc(pub Arc<MiniCanvas>);
// impl AsRef<MiniCanvas> for MiniCanvasArc {
//     fn as_ref(&self) -> &MiniCanvas {
//         self.inner.as_ref()
//     }
// }
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


// pub struct MiniCanvasWeakRef {
//     inner: Weak<MiniCanvas>,
// }
// impl MiniCanvasWeakRef {
//     pub fn upgrade(&self) -> Option<MiniCanvasRef> {
//         self.inner.upgrade().map(|r| MiniCanvasRef(r))
//     }
// }

impl crate::wasi::webgpu::mini_canvas::Host for HostState {}



// struct TempEventLoop (pub EventLoop<()>);
// unsafe impl Send for TempEventLoop {}
// unsafe impl Sync for TempEventLoop {}

impl crate::wasi::webgpu::mini_canvas::HostMiniCanvas for HostState {
    fn new(&mut self, desc: CreateDesc) -> wasmtime::Result<Resource<MiniCanvasArc>> {
        // let event_loop = winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
        let window = block_on(self.message_sender.create_window());
        let res = Ok(self
            .table
            .push(MiniCanvasArc(Arc::new(MiniCanvas {
                // height: desc.height,
                // width: desc.width,
                offscreen: desc.offscreen,

                // TODO: remove any thread
                // window: Window::new(&winit::event_loop::EventLoopBuilder::new().build()).unwrap(),
                // window: Window::new(&event_loop).unwrap(),
                window,
            })))
            .unwrap());

        // let event_loop = TempEventLoop(event_loop);

        // tokio::spawn(async move {
        //     // &event_loop.0;
        //     // event_loop.0.run(|a, b, c| {});
        //     fn g(el: TempEventLoop) {
        //         el.0.run(|a, b, c| {});
        //     }
        //     g(event_loop);
        // });

        res

        // listen_to_events(event, sender);
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
        _mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<ResizeListener>> {
        let receiver = self.sender.subscribe();
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

#[derive(Debug)]
pub struct ResizeListener {
    receiver: Receiver<HostEvent>,
    data: Mutex<Option<ResizeEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for ResizeListener {
    async fn ready(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            if let HostEvent::CanvasResizeEvent(event) = event {
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
