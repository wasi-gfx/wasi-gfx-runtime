use std::sync::Mutex;

use crate::{
    component::webgpu::animation_frame::{HostFrameListener, FrameEvent, Pollable},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl crate::component::webgpu::animation_frame::Host for HostState {
    async fn listener(&mut self) -> wasmtime::Result<wasmtime::component::Resource<AnimationFrameListener>> {
        let receiver = self.sender.subscribe();

        Ok(self
            .table_mut()
            .push(AnimationFrameListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

#[async_trait::async_trait]
impl HostFrameListener for HostState {
    async fn subscribe(
        &mut self,
        frame_listener: Resource<AnimationFrameListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        preview2::subscribe(self.table_mut(), frame_listener)
    }
    async fn get(&mut self, frame_listener: Resource<AnimationFrameListener>) -> wasmtime::Result<Option<FrameEvent>> {
        let frame_listener = self.table.get(&frame_listener).unwrap();
        Ok(frame_listener.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<AnimationFrameListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

pub struct AnimationFrameListener {
    receiver: Receiver<HostEvent>,
    data: Mutex<Option<FrameEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for AnimationFrameListener {
    async fn ready(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            if let HostEvent::Frame = event {
                *self.data.lock().unwrap() = Some(FrameEvent { nothing: false });
                return;
            }
        }
    }
}
