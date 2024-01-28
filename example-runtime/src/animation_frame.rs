use std::sync::Mutex;

use crate::{
    component::webgpu::animation_frame::{FrameEvent, HostFrameListener, Pollable},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

impl crate::component::webgpu::animation_frame::Host for HostState {
    fn listener(&mut self) -> wasmtime::Result<Resource<AnimationFrameListener>> {
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

impl HostFrameListener for HostState {
    fn subscribe(
        &mut self,
        frame_listener: Resource<AnimationFrameListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        preview2::subscribe(self.table_mut(), frame_listener)
    }
    fn get(
        &mut self,
        frame_listener: Resource<AnimationFrameListener>,
    ) -> wasmtime::Result<Option<FrameEvent>> {
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
            if let Ok(event) = self.receiver.recv().await {
                if let HostEvent::Frame = event {
                    *self.data.lock().unwrap() = Some(FrameEvent { nothing: false });
                    return;
                }
            }
        }
    }
}
