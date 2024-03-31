use std::sync::Mutex;

use crate::{
    mini_canvas::MiniCanvasArc,
    wasi::webgpu::key_events::{self, KeyEvent, Pollable},
    HostState,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::window::WindowId;

impl key_events::Host for HostState {
    fn up_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<KeyUpListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy
            .receivers
            .key_up_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(KeyUpListener {
                window_id,
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    fn down_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<KeyDownListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy
            .receivers
            .key_down_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(KeyDownListener {
                window_id,
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

impl key_events::HostKeyUpListener for HostState {
    fn subscribe(
        &mut self,
        key_up: Resource<KeyUpListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), key_up).unwrap())
    }
    fn get(&mut self, key_up: Resource<KeyUpListener>) -> wasmtime::Result<Option<KeyEvent>> {
        let key_up = self.table.get(&key_up).unwrap();
        Ok(key_up.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<KeyUpListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyUpListener {
    window_id: WindowId,
    receiver: Receiver<(WindowId, KeyEvent)>,
    data: Mutex<Option<KeyEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for KeyUpListener {
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

impl key_events::HostKeyDownListener for HostState {
    fn subscribe(
        &mut self,
        key_down: Resource<KeyDownListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), key_down).unwrap())
    }
    fn get(&mut self, key_down: Resource<KeyDownListener>) -> wasmtime::Result<Option<KeyEvent>> {
        let key_down = self.table.get(&key_down).unwrap();
        Ok(key_down.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<KeyDownListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyDownListener {
    window_id: WindowId,
    receiver: Receiver<(WindowId, KeyEvent)>,
    data: Mutex<Option<KeyEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for KeyDownListener {
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
