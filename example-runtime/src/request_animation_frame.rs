use std::sync::Mutex;

use crate::{
    component::webgpu::request_animation_frame::{Frame, HostFrame, Pollable},
    HostEvent, HostState,
};
use tokio::sync::broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl crate::component::webgpu::request_animation_frame::Host for HostState {
    async fn get_frame(&mut self) -> wasmtime::Result<wasmtime::component::Resource<Frame>> {
        println!("in get_frame");
        let receiver = self.sender.subscribe();

        let g = self
            .table_mut()
            .push(FrameThis {
                receiver,
                data: Default::default(),
            })
            .unwrap();
        Ok(Resource::new_own(g.rep()))
    }
}

#[async_trait::async_trait]
impl HostFrame for HostState {
    async fn subscribe(
        &mut self,
        self_: Resource<FrameThis>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        preview2::subscribe(self.table_mut(), self_)
    }
    async fn get(&mut self, self_: Resource<Frame>) -> wasmtime::Result<Option<bool>> {
        let ddd = self.table.get(&self_).unwrap();
        let res = ddd.data.lock().unwrap().take();
        Ok(res.map(|_| true))
    }
    fn drop(&mut self, _self_: Resource<Frame>) -> wasmtime::Result<()> {
        println!("in drop");
        Ok(())
    }
}

pub struct FrameThis {
    receiver: Receiver<HostEvent>,
    data: Mutex<Option<()>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for FrameThis {
    async fn ready(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            if let HostEvent::Frame = event {
                *self.data.lock().unwrap() = Some(());
                return;
            }
        }
    }
}
