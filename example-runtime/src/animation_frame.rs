use std::sync::Mutex;

use crate::{
    mini_canvas::MiniCanvasArc,
    wasi::webgpu::animation_frame::{FrameEvent, HostFrameListener, Pollable},
    HostState,
};
use async_broadcast::Receiver;
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::window::WindowId;

impl crate::wasi::webgpu::animation_frame::Host for HostState {
    fn listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<AnimationFrameListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        let receiver = self.message_sender.receivers.frame.activate_cloned();
        Ok(self
            .table_mut()
            .push(AnimationFrameListener {
                _window_id: window_id,
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
    _window_id: WindowId,
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
