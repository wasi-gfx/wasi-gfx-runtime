use std::sync::Mutex;

use crate::{
    wasi::webgpu::animation_frame::{self, FrameEvent, Pollable},
    MiniCanvasArc, WasiMiniCanvasView,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;

#[async_trait::async_trait]
impl animation_frame::Host for dyn WasiMiniCanvasView + '_ {
    async fn listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<AnimationFrameListener> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self
            .main_thread_proxy()
            .create_frame_listener(window_id)
            .await;
        self.table()
            .push(AnimationFrameListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }
}

impl animation_frame::HostFrameListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(
        &mut self,
        frame_listener: Resource<AnimationFrameListener>,
    ) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), frame_listener).unwrap()
    }
    fn get(&mut self, frame_listener: Resource<AnimationFrameListener>) -> Option<FrameEvent> {
        let frame_listener = self.table().get(&frame_listener).unwrap();
        frame_listener.data.lock().unwrap().take()
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
impl wasmtime_wasi::Subscribe for AnimationFrameListener {
    async fn ready(&mut self) {
        let _ = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(FrameEvent { nothing: false });
    }
}
