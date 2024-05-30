use std::sync::{Arc, Mutex};

use crate::{
    wasi::webgpu::pointer_events::{self, PointerEvent, Pollable},
    MiniCanvasArc, WasiMiniCanvasView,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;

#[async_trait::async_trait]
impl pointer_events::Host for dyn WasiMiniCanvasView + '_ {
    async fn up_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<PointerUpListener> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_up_listener(window_id)
            .await;
        self.table()
            .push(PointerUpListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }

    async fn down_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<PointerDownListener> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_down_listener(window_id)
            .await;
        self.table()
            .push(PointerDownListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }

    async fn move_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<PointerMoveListener> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_move_listener(window_id)
            .await;
        self.table()
            .push(PointerMoveListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }
}

impl pointer_events::HostPointerUpListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, pointer_up: Resource<PointerUpListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), pointer_up).unwrap()
    }
    fn get(&mut self, pointer_up: Resource<PointerUpListener>) -> Option<PointerEvent> {
        let pointer_up = self.table().get(&pointer_up).unwrap();
        pointer_up.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<PointerUpListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerUpListener {
    receiver: Receiver<PointerEvent>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for PointerUpListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

impl pointer_events::HostPointerDownListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, pointer_down: Resource<PointerDownListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), pointer_down).unwrap()
    }
    fn get(&mut self, pointer_down: Resource<PointerDownListener>) -> Option<PointerEvent> {
        let pointer_down = self.table().get(&pointer_down).unwrap();
        pointer_down.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<PointerDownListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerDownListener {
    receiver: Receiver<PointerEvent>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for PointerDownListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

impl pointer_events::HostPointerMoveListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, pointer_move: Resource<PointerMoveListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), pointer_move).unwrap()
    }
    fn get(&mut self, pointer_move: Resource<PointerMoveListener>) -> Option<PointerEvent> {
        let pointer_move = self.table().get(&pointer_move).unwrap();
        pointer_move.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<PointerMoveListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerMoveListener {
    receiver: Receiver<PointerEvent>,
    data: Arc<Mutex<Option<PointerEvent>>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for PointerMoveListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}
