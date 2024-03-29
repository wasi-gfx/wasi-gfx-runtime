use std::sync::{Arc, Mutex};

use crate::{
    wasi::webgpu::pointer_events::{PointerEvent, Pollable},
    HostState,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

impl crate::wasi::webgpu::pointer_events::Host for HostState {
    fn up_listener(&mut self) -> wasmtime::Result<Resource<PointerUpListener>> {
        let receiver = self
            .message_sender
            .receivers
            .pointer_up_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(PointerUpListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    fn down_listener(&mut self) -> wasmtime::Result<Resource<PointerDownListener>> {
        let receiver = self
            .message_sender
            .receivers
            .pointer_down_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(PointerDownListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    fn move_listener(&mut self) -> wasmtime::Result<Resource<PointerMoveListener>> {
        let receiver = self
            .message_sender
            .receivers
            .pointer_move_event
            .activate_cloned();
        Ok(self
            .table_mut()
            .push(PointerMoveListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

impl crate::wasi::webgpu::pointer_events::HostPointerUpListener for HostState {
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
        let pointer_up = self.table.get(&pointer_up).unwrap();
        Ok(pointer_up.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<PointerUpListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerUpListener {
    receiver: Receiver<(u32, PointerEvent)>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for PointerUpListener {
    async fn ready(&mut self) {
        let (id, event) = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

impl crate::wasi::webgpu::pointer_events::HostPointerDownListener for HostState {
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
        let pointer_down = self.table.get(&pointer_down).unwrap();
        Ok(pointer_down.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<PointerDownListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerDownListener {
    receiver: Receiver<(u32, PointerEvent)>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for PointerDownListener {
    async fn ready(&mut self) {
        // loop {
        let (id, event) = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
        // }
    }
}

impl crate::wasi::webgpu::pointer_events::HostPointerMoveListener for HostState {
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
        let pointer_move = self.table.get(&pointer_move).unwrap();
        Ok(pointer_move.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<PointerMoveListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PointerMoveListener {
    receiver: Receiver<(u32, PointerEvent)>,
    data: Arc<Mutex<Option<PointerEvent>>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for PointerMoveListener {
    async fn ready(&mut self) {
        let (id, event) = self.receiver.recv().await.unwrap();
        // let (id, event) = receiver.await.unwrap();
        *self.data.lock().unwrap() = Some(event);
        println!("recv");
    }
}
