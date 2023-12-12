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

        let g = self.table_mut().push(FrameThis { receiver }).unwrap();
        Ok(Resource::new_own(g.rep()))
    }
}

#[async_trait::async_trait]
impl HostFrame for HostState {
    async fn subscribe(&mut self, self_: Resource<Frame>) -> wasmtime::Result<Resource<Pollable>> {
        let g: Resource<FrameThis> = Resource::new_own(self_.rep());
        preview2::subscribe(self.table_mut(), g)
    }
    async fn get(&mut self, _self_: Resource<Frame>) -> wasmtime::Result<Option<bool>> {
        println!("in get");
        Ok(Some(false))
    }
    fn drop(&mut self, _self_: Resource<Frame>) -> wasmtime::Result<()> {
        println!("in drop");
        Ok(())
    }
}

struct FrameThis {
    receiver: Receiver<HostEvent>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for FrameThis {
    async fn ready(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            if let HostEvent::Frame { .. } = event {
                println!("frame ready");
                return;
            }
        }
    }
}
