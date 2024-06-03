use std::sync::Mutex;

use crate::{
    wasi::webgpu::key_events::{self, KeyEvent, Pollable},
    MiniCanvasArc, WasiMiniCanvasView,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;

#[async_trait::async_trait]
impl key_events::Host for dyn WasiMiniCanvasView + '_ {
    async fn up_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<KeyUpListener> {
        let canvas = &self.table().get(&mini_canvas).unwrap().0;
        let receiver = canvas.key_up_sender.new_receiver();
        self.table()
            .push(KeyUpListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }

    async fn down_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<KeyDownListener> {
        let canvas = &self.table().get(&mini_canvas).unwrap().0;
        let receiver = canvas.key_down_sender.new_receiver();
        self.table()
            .push(KeyDownListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }
}

impl key_events::HostKeyUpListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, key_up: Resource<KeyUpListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), key_up).unwrap()
    }
    fn get(&mut self, key_up: Resource<KeyUpListener>) -> Option<KeyEvent> {
        let key_up = self.table().get(&key_up).unwrap();
        key_up.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<KeyUpListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyUpListener {
    receiver: Receiver<KeyEvent>,
    data: Mutex<Option<KeyEvent>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for KeyUpListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}
impl key_events::HostKeyDownListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, key_down: Resource<KeyDownListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), key_down).unwrap()
    }
    fn get(&mut self, key_down: Resource<KeyDownListener>) -> Option<KeyEvent> {
        let key_down = self.table().get(&key_down).unwrap();
        key_down.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<KeyDownListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyDownListener {
    receiver: Receiver<KeyEvent>,
    data: Mutex<Option<KeyEvent>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for KeyDownListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}
