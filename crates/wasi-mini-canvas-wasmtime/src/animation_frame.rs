use std::sync::Mutex;

use crate::{
    wasi::webgpu::animation_frame::{self, FrameEvent, Pollable},
    HasMainThreadProxy, MiniCanvasArc,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};

#[async_trait::async_trait]
impl<T: WasiView + HasMainThreadProxy> animation_frame::Host for T {
    async fn listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<AnimationFrameListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_frame_listener(window_id)
            .await;
        Ok(self
            .table_mut()
            .push(AnimationFrameListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }
}

impl<T: WasiView + HasMainThreadProxy> animation_frame::HostFrameListener for T {
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
        let frame_listener = self.table().get(&frame_listener).unwrap();
        Ok(frame_listener.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<AnimationFrameListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct AnimationFrameListener {
    receiver: Receiver<()>,
    data: Mutex<Option<FrameEvent>>,
}

#[async_trait::async_trait]
impl preview2::Subscribe for AnimationFrameListener {
    async fn ready(&mut self) {
        let _ = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(FrameEvent { nothing: false });
    }
}
