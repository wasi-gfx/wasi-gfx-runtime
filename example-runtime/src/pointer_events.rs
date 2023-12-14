use std::sync::Mutex;

use crate::{
    component::webgpu::pointer_events::{HostPointerUp, PointerEvent, Pollable},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl crate::component::webgpu::pointer_events::Host for HostState {
    async fn up(&mut self) -> wasmtime::Result<wasmtime::component::Resource<HostPointerEvent>> {
        let receiver = self.sender.subscribe();
        Ok(self
            .table_mut()
            .push(HostPointerEvent {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

#[async_trait::async_trait]
impl HostPointerUp for HostState {
    async fn subscribe(
        &mut self,
        pointer_up: Resource<HostPointerEvent>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_up).unwrap())
    }
    async fn get(
        &mut self,
        pointer_up: Resource<HostPointerEvent>,
    ) -> wasmtime::Result<Option<PointerEvent>> {
        let ddd = self.table.get(&pointer_up).unwrap();
        let res = ddd.data.lock().unwrap().take();
        Ok(res)
    }
    fn drop(&mut self, _self_: Resource<HostPointerEvent>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct HostPointerEvent {
    receiver: Receiver<HostEvent>,
    data: Mutex<Option<PointerEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for HostPointerEvent {
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
