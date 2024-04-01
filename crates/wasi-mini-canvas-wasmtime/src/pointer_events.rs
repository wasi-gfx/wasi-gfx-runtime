use std::sync::{Arc, Mutex};

use crate::{
    wasi::webgpu::pointer_events::{self, PointerEvent, Pollable},
    HasMainThreadProxy, MiniCanvasArc,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl<T: WasiView + HasMainThreadProxy> pointer_events::Host for T {
    async fn up_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<PointerUpListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_up_listener(window_id)
            .await;
        Ok(self
            .table_mut()
            .push(PointerUpListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    async fn down_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<PointerDownListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_down_listener(window_id)
            .await;
        Ok(self
            .table_mut()
            .push(PointerDownListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    async fn move_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<PointerMoveListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_pointer_move_listener(window_id)
            .await;
        Ok(self
            .table_mut()
            .push(PointerMoveListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

impl<T: WasiView + HasMainThreadProxy> pointer_events::HostPointerUpListener for T {
    fn subscribe(
        &mut self,
        pointer_up: Resource<PointerUpListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_up).unwrap())
    }
    fn get(
        &mut self,
        pointer_up: Resource<PointerUpListener>,
    ) -> wasmtime::Result<Option<PointerEvent>> {
        let pointer_up = self.table().get(&pointer_up).unwrap();
        Ok(pointer_up.data.lock().unwrap().take())
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
impl preview2::Subscribe for PointerUpListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

impl<T: WasiView + HasMainThreadProxy> pointer_events::HostPointerDownListener for T {
    fn subscribe(
        &mut self,
        pointer_down: Resource<PointerDownListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_down).unwrap())
    }
    fn get(
        &mut self,
        pointer_down: Resource<PointerDownListener>,
    ) -> wasmtime::Result<Option<PointerEvent>> {
        let pointer_down = self.table().get(&pointer_down).unwrap();
        Ok(pointer_down.data.lock().unwrap().take())
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
impl preview2::Subscribe for PointerDownListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

impl<T: WasiView + HasMainThreadProxy> pointer_events::HostPointerMoveListener for T {
    fn subscribe(
        &mut self,
        pointer_move: Resource<PointerMoveListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_move).unwrap())
    }
    fn get(
        &mut self,
        pointer_move: Resource<PointerMoveListener>,
    ) -> wasmtime::Result<Option<PointerEvent>> {
        let pointer_move = self.table().get(&pointer_move).unwrap();
        Ok(pointer_move.data.lock().unwrap().take())
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
impl preview2::Subscribe for PointerMoveListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}
