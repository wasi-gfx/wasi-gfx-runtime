use std::sync::Mutex;

use crate::{
    component::webgpu::pointer_events::{HostPointerUpListener, PointerEvent, Pollable},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl crate::component::webgpu::pointer_events::Host for HostState {
    async fn up_listener(&mut self) -> wasmtime::Result<wasmtime::component::Resource<PointerUpListener>> {
        let receiver = self.sender.subscribe();
        Ok(self
            .table_mut()
            .push(PointerUpListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

#[async_trait::async_trait]
impl HostPointerUpListener for HostState {
    async fn subscribe(
        &mut self,
        pointer_up: Resource<PointerUpListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_up).unwrap())
    }
    async fn get(
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
    receiver: Receiver<HostEvent>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for PointerUpListener {
    async fn ready(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            if let HostEvent::PointerEvent { x, y } = event {
                *self.data.lock().unwrap() = Some(PointerEvent { x, y });
                return;
            }
        }
    }
}
