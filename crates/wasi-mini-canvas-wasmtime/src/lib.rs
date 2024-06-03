use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use wasi_graphics_context_wasmtime::DisplayApi;

use crate::wasi::webgpu::mini_canvas::{self, GraphicsContext, Pollable};
use async_broadcast::{Receiver, TrySendError};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;

mod animation_frame;
mod key_events;
mod pointer_events;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use winit::{create_wasi_winit_event_loop, WasiWinitEventLoop, WasiWinitEventLoopProxy};

pub trait HasDisplayAndWindowHandle: HasDisplayHandle + HasWindowHandle {}

impl<T: HasDisplayHandle + HasWindowHandle> HasDisplayAndWindowHandle for T {}

pub use crate::wasi::webgpu::{
    key_events::KeyEvent,
    mini_canvas::{CreateDesc as MiniCanvasDesc, ResizeEvent},
    pointer_events::PointerEvent,
};

pub trait WasiMiniCanvasView: WasiView {
    fn create_canvas(&self, desc: MiniCanvasDesc) -> MiniCanvas;
}

pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiMiniCanvasView,
{
    fn type_annotate<T, F>(val: F) -> F
    where
        F: Fn(&mut T) -> &mut dyn WasiMiniCanvasView,
    {
        val
    }
    let closure = type_annotate::<T, _>(|t| t);
    wasi::webgpu::mini_canvas::add_to_linker_get_host(l, closure)?;
    wasi::webgpu::animation_frame::add_to_linker_get_host(l, closure)?;
    wasi::webgpu::pointer_events::add_to_linker_get_host(l, closure)?;
    wasi::webgpu::key_events::add_to_linker_get_host(l, closure)?;
    wasmtime_wasi::bindings::io::poll::add_to_linker_get_host(l, closure)?;
    wasmtime_wasi::bindings::io::streams::add_to_linker_get_host(l, closure)?;
    Ok(())
}

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: {
        only_imports: [
            "poll",
            "up-listener",
            "down-listener",
            "move-listener",
            "listener",
            // "resize-listener",
        ],
    },
    with: {
        "wasi:io": wasmtime_wasi::bindings::io,
        "wasi:webgpu/pointer-events/pointer-up-listener": pointer_events::PointerUpListener,
        "wasi:webgpu/pointer-events/pointer-down-listener": pointer_events::PointerDownListener,
        "wasi:webgpu/pointer-events/pointer-move-listener": pointer_events::PointerMoveListener,
        "wasi:webgpu/key-events/key-up-listener": key_events::KeyUpListener,
        "wasi:webgpu/key-events/key-down-listener": key_events::KeyDownListener,
        "wasi:webgpu/animation-frame/frame-listener": animation_frame::AnimationFrameListener,
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime,
        "wasi:webgpu/mini-canvas/mini-canvas": MiniCanvasArc,
        "wasi:webgpu/mini-canvas/resize-listener": ResizeListener,
    },
});

pub struct MiniCanvas {
    pub window: Box<dyn DisplayApi + Send + Sync + 'static>,

    // Keeping inactive receivers to keep channels open.
    // See https://docs.rs/async-broadcast/0.7.1/async_broadcast/struct.InactiveReceiver.html
    pointer_up_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_up_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    pointer_down_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_down_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    pointer_move_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_move_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    key_up_sender: async_broadcast::Sender<KeyEvent>,
    _key_up_receiver: async_broadcast::InactiveReceiver<KeyEvent>,
    key_down_sender: async_broadcast::Sender<KeyEvent>,
    _key_down_receiver: async_broadcast::InactiveReceiver<KeyEvent>,
    canvas_resize_sender: async_broadcast::Sender<ResizeEvent>,
    _canvas_resize_receiver: async_broadcast::InactiveReceiver<ResizeEvent>,
    frame_sender: async_broadcast::Sender<()>,
    _frame_receiver: async_broadcast::InactiveReceiver<()>,
}
impl Debug for MiniCanvas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiniCanvas")
            .field("window", &"<Boxed window>")
            .field("pointer_up_sender", &self.pointer_up_sender)
            .field("_pointer_up_receiver", &self._pointer_up_receiver)
            .field("pointer_down_sender", &self.pointer_down_sender)
            .field("_pointer_down_receiver", &self._pointer_down_receiver)
            .field("pointer_move_sender", &self.pointer_move_sender)
            .field("_pointer_move_receiver", &self._pointer_move_receiver)
            .field("key_up_sender", &self.key_up_sender)
            .field("_key_up_receiver", &self._key_up_receiver)
            .field("key_down_sender", &self.key_down_sender)
            .field("_key_down_receiver", &self._key_down_receiver)
            .field("canvas_resize_sender", &self.canvas_resize_sender)
            .field("_canvas_resize_receiver", &self._canvas_resize_receiver)
            .field("frame_sender", &self.frame_sender)
            .field("_frame_receiver", &self._frame_receiver)
            .finish()
    }
}

impl MiniCanvas {
    pub fn new(window: Box<dyn DisplayApi + Send + Sync + 'static>) -> Self {
        let (pointer_up_sender, pointer_up_receiver) = async_broadcast::broadcast(5);
        let pointer_up_receiver = pointer_up_receiver.deactivate();
        let (pointer_down_sender, pointer_down_receiver) = async_broadcast::broadcast(5);
        let pointer_down_receiver = pointer_down_receiver.deactivate();
        let (pointer_move_sender, pointer_move_receiver) = async_broadcast::broadcast(5);
        let pointer_move_receiver = pointer_move_receiver.deactivate();
        let (key_up_sender, key_up_receiver) = async_broadcast::broadcast(5);
        let key_up_receiver = key_up_receiver.deactivate();
        let (key_down_sender, key_down_receiver) = async_broadcast::broadcast(5);
        let key_down_receiver = key_down_receiver.deactivate();
        let (canvas_resize_sender, canvas_resize_receiver) = async_broadcast::broadcast(5);
        let canvas_resize_receiver = canvas_resize_receiver.deactivate();
        let (frame_sender, frame_receiver) = async_broadcast::broadcast(1);
        let frame_receiver = frame_receiver.deactivate();
        Self {
            window,
            pointer_up_sender,
            _pointer_up_receiver: pointer_up_receiver,
            pointer_down_sender,
            _pointer_down_receiver: pointer_down_receiver,
            pointer_move_sender,
            _pointer_move_receiver: pointer_move_receiver,
            key_up_sender,
            _key_up_receiver: key_up_receiver,
            key_down_sender,
            _key_down_receiver: key_down_receiver,
            canvas_resize_sender,
            _canvas_resize_receiver: canvas_resize_receiver,
            frame_sender,
            _frame_receiver: frame_receiver,
        }
    }

    pub fn proxy(&self) -> MiniCanvasProxy {
        MiniCanvasProxy {
            pointer_up_sender: self.pointer_up_sender.clone(),
            pointer_down_sender: self.pointer_down_sender.clone(),
            pointer_move_sender: self.pointer_move_sender.clone(),
            key_up_sender: self.key_up_sender.clone(),
            key_down_sender: self.key_down_sender.clone(),
            canvas_resize_sender: self.canvas_resize_sender.clone(),
            frame_sender: self.frame_sender.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiniCanvasProxy {
    pointer_up_sender: async_broadcast::Sender<PointerEvent>,
    pointer_down_sender: async_broadcast::Sender<PointerEvent>,
    pointer_move_sender: async_broadcast::Sender<PointerEvent>,
    key_up_sender: async_broadcast::Sender<KeyEvent>,
    key_down_sender: async_broadcast::Sender<KeyEvent>,
    canvas_resize_sender: async_broadcast::Sender<ResizeEvent>,
    frame_sender: async_broadcast::Sender<()>,
}

impl MiniCanvasProxy {
    pub fn pointer_up(&self, event: PointerEvent) {
        unwrap_unless_inactive(self.pointer_up_sender.try_broadcast(event));
    }
    pub fn pointer_down(&self, event: PointerEvent) {
        unwrap_unless_inactive(self.pointer_down_sender.try_broadcast(event));
    }
    pub fn pointer_move(&self, event: PointerEvent) {
        unwrap_unless_inactive_or_full(self.pointer_move_sender.try_broadcast(event));
    }
    pub fn key_up(&self, event: KeyEvent) {
        unwrap_unless_inactive(self.key_up_sender.try_broadcast(event));
    }
    pub fn key_down(&self, event: KeyEvent) {
        unwrap_unless_inactive(self.key_down_sender.try_broadcast(event));
    }
    pub fn canvas_resize(&self, event: ResizeEvent) {
        unwrap_unless_inactive(self.canvas_resize_sender.try_broadcast(event));
    }
    pub fn animation_frame(&self) {
        unwrap_unless_inactive_or_full(self.frame_sender.try_broadcast(()));
    }
}

impl HasDisplayHandle for MiniCanvas {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle, raw_window_handle::HandleError> {
        self.window.display_handle()
    }
}
impl HasWindowHandle for MiniCanvas {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle, raw_window_handle::HandleError> {
        self.window.window_handle()
    }
}

impl DisplayApi for MiniCanvas {
    fn height(&self) -> u32 {
        self.window.height()
    }

    fn width(&self) -> u32 {
        self.window.width()
    }
}

// TODO: instead of Arc, maybe have a global list of windows and ids? That ways it's same as webgpu, but might be harder to handle? Would likely also require a Mutex.
#[derive(Clone)]
pub struct MiniCanvasArc(pub Arc<MiniCanvas>);

impl HasDisplayHandle for MiniCanvasArc {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.0.display_handle()
    }
}
impl HasWindowHandle for MiniCanvasArc {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.0.window_handle()
    }
}

impl DisplayApi for MiniCanvasArc {
    fn height(&self) -> u32 {
        self.0.height()
    }

    fn width(&self) -> u32 {
        self.0.width()
    }
}

fn unwrap_unless_inactive<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(e) = &res {
        if let TrySendError::Inactive(_) = e {
            return;
        }
    }
    res.unwrap();
}

fn unwrap_unless_inactive_or_full<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(e) = &res {
        if matches!(e, TrySendError::Inactive(_) | TrySendError::Full(_)) {
            return;
        }
    }
    res.unwrap();
}

#[derive(Debug)]
pub struct ResizeListener {
    receiver: Receiver<ResizeEvent>,
    data: Mutex<Option<ResizeEvent>>,
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for ResizeListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

// wasmtime
impl mini_canvas::Host for dyn WasiMiniCanvasView + '_ {}

#[async_trait::async_trait]
impl mini_canvas::HostMiniCanvas for dyn WasiMiniCanvasView + '_ {
    fn new(&mut self, desc: MiniCanvasDesc) -> Resource<MiniCanvasArc> {
        let canvas = self.create_canvas(desc);
        let mini_canvas = MiniCanvasArc(Arc::new(canvas));
        self.table().push(mini_canvas).unwrap()
    }

    fn connect_graphics_context(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
        context: Resource<GraphicsContext>,
    ) {
        let mini_canvas = self.table().get(&mini_canvas).unwrap().clone();
        let graphics_context = self.table().get_mut(&context).unwrap();

        graphics_context.connect_display_api(Box::new(mini_canvas));
    }

    fn resize_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> Resource<ResizeListener> {
        let canvas = &self.table().get(&mini_canvas).unwrap().0;
        let receiver = canvas.canvas_resize_sender.new_receiver();
        self.table()
            .push(ResizeListener {
                receiver,
                data: Default::default(),
            })
            .unwrap()
    }

    fn height(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> u32 {
        let mini_canvas = self.table().get(&mini_canvas).unwrap();
        mini_canvas.height()
    }

    fn width(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> u32 {
        let mini_canvas = self.table().get(&mini_canvas).unwrap();
        mini_canvas.width()
    }

    fn drop(&mut self, _self_: Resource<MiniCanvasArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl mini_canvas::HostResizeListener for dyn WasiMiniCanvasView + '_ {
    fn subscribe(&mut self, pointer_down: Resource<ResizeListener>) -> Resource<Pollable> {
        wasmtime_wasi::subscribe(self.table(), pointer_down).unwrap()
    }
    fn get(&mut self, pointer_down: Resource<ResizeListener>) -> Option<ResizeEvent> {
        let pointer_down = self.table().get(&pointer_down).unwrap();
        pointer_down.data.lock().unwrap().take()
    }
    fn drop(&mut self, _self_: Resource<ResizeListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}
