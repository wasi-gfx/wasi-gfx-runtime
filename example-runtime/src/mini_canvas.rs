use std::sync::Mutex;

use crate::{
    component::webgpu::mini_canvas::{CreateDesc, GraphicsContext, Pollable, ResizeEvent},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[derive(Debug)]
pub struct MiniCanvas {
    pub height: u32,
    pub width: u32,
    pub offscreen: bool,
}

impl crate::component::webgpu::mini_canvas::Host for HostState {}

impl crate::component::webgpu::mini_canvas::HostMiniCanvas for HostState {
    fn new(&mut self, desc: CreateDesc) -> wasmtime::Result<Resource<MiniCanvas>> {
        Ok(self
            .table
            .push(MiniCanvas {
                height: desc.height,
                width: desc.width,
                offscreen: desc.offscreen,
            })
            .unwrap())
    }

    fn connect_graphics_context(
        &mut self,
        mini_canvas: Resource<MiniCanvas>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let _mini_canvas = self.table.get(&mini_canvas).unwrap();
        let _context = self.table.get(&context).unwrap();
        // noop for now, will do the surface creation once window is part min-canvas instead of a singleton.
        Ok(())
    }

    fn resize_listener(
        &mut self,
        _mini_canvas: Resource<MiniCanvas>,
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

    fn height(&mut self, mini_canvas: Resource<MiniCanvas>) -> wasmtime::Result<u32> {
        let _mini_canvas = self.table.get(&mini_canvas).unwrap();
        Ok(self.window.inner_size().height)
    }

    fn width(&mut self, mini_canvas: Resource<MiniCanvas>) -> wasmtime::Result<u32> {
        let _mini_canvas = self.table.get(&mini_canvas).unwrap();
        Ok(self.window.inner_size().width)
    }

    fn drop(&mut self, _self_: Resource<MiniCanvas>) -> wasmtime::Result<()> {
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

impl crate::component::webgpu::mini_canvas::HostResizeListener for HostState {
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
