use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use wasi_graphics_context_wasmtime::DisplayApi;

use crate::wasi::webgpu::surface::{self, Context as GraphicsContext, Pollable};
use async_broadcast::{Receiver, TrySendError};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::{IoImpl, IoView};

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use winit::{create_wasi_winit_event_loop, WasiWinitEventLoop, WasiWinitEventLoopProxy};

pub trait HasDisplayAndWindowHandle: HasDisplayHandle + HasWindowHandle {}

impl<T: HasDisplayHandle + HasWindowHandle> HasDisplayAndWindowHandle for T {}

pub use crate::wasi::webgpu::surface::{
    FrameEvent, KeyEvent, PointerEvent, {CreateDesc as MiniCanvasDesc, ResizeEvent},
};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: {
        only_imports: [],
    },
    with: {
        "wasi:io": wasmtime_wasi::bindings::io,
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime::wasi::webgpu::graphics_context,
        "wasi:webgpu/surface/surface": MiniCanvasArc,
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

    // TODO: remove once we get rid of pollable
    pointer_up_data: Mutex<Option<PointerEvent>>,
    pointer_down_data: Mutex<Option<PointerEvent>>,
    pointer_move_data: Mutex<Option<PointerEvent>>,
    key_up_data: Mutex<Option<KeyEvent>>,
    key_down_data: Mutex<Option<KeyEvent>>,
    canvas_resize_data: Mutex<Option<ResizeEvent>>,
    frame_data: Mutex<Option<FrameEvent>>,
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
            pointer_up_data: Default::default(),
            pointer_down_data: Default::default(),
            pointer_move_data: Default::default(),
            key_up_data: Default::default(),
            key_down_data: Default::default(),
            canvas_resize_data: Default::default(),
            frame_data: Default::default(),
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

    fn request_set_size(&self, width: Option<u32>, height: Option<u32>) {
        self.window.request_set_size(width, height);
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

    fn request_set_size(&self, width: Option<u32>, height: Option<u32>) {
        self.0.request_set_size(width, height);
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

// wasmtime
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiMiniCanvasView + IoView,
{
    fn type_annotate_io<T, F>(val: F) -> F
    where
        T: IoView,
        F: Fn(&mut T) -> IoImpl<&mut T>,
    {
        val
    }
    let closure_io = type_annotate_io::<T, _>(|t| IoImpl(t));
    wasmtime_wasi::bindings::io::poll::add_to_linker_get_host(l, closure_io)?;
    wasmtime_wasi::bindings::io::streams::add_to_linker_get_host(l, closure_io)?;
    add_only_mini_canvas_to_linker(l)?;
    Ok(())
}

pub fn add_only_mini_canvas_to_linker<T>(
    l: &mut wasmtime::component::Linker<T>,
) -> wasmtime::Result<()>
where
    T: WasiMiniCanvasView,
{
    fn type_annotate<T, F>(val: F) -> F
    where
        T: WasiMiniCanvasView,
        F: Fn(&mut T) -> WasiMiniCanvasImpl<&mut T>,
    {
        val
    }
    let closure = type_annotate::<T, _>(|t| WasiMiniCanvasImpl(t));
    wasi::webgpu::surface::add_to_linker_get_host(l, closure)?;
    Ok(())
}

pub trait WasiMiniCanvasView: IoView {
    fn create_canvas(&self, desc: MiniCanvasDesc) -> MiniCanvas;
}

#[repr(transparent)]
pub struct WasiMiniCanvasImpl<T: WasiMiniCanvasView>(pub T);
impl<T: WasiMiniCanvasView> IoView for WasiMiniCanvasImpl<T> {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        T::table(&mut self.0)
    }
}

impl<T: ?Sized + WasiMiniCanvasView> WasiMiniCanvasView for &mut T {
    fn create_canvas(&self, desc: MiniCanvasDesc) -> MiniCanvas {
        T::create_canvas(self, desc)
    }
}
impl<T: ?Sized + WasiMiniCanvasView> WasiMiniCanvasView for Box<T> {
    fn create_canvas(&self, desc: MiniCanvasDesc) -> MiniCanvas {
        T::create_canvas(self, desc)
    }
}
impl<T: WasiMiniCanvasView> WasiMiniCanvasView for WasiMiniCanvasImpl<T> {
    fn create_canvas(&self, desc: MiniCanvasDesc) -> MiniCanvas {
        T::create_canvas(&self.0, desc)
    }
}

impl<T: WasiMiniCanvasView> surface::Host for WasiMiniCanvasImpl<T> {}

impl<T: WasiMiniCanvasView> surface::HostSurface for WasiMiniCanvasImpl<T> {
    fn new(&mut self, desc: MiniCanvasDesc) -> Resource<MiniCanvasArc> {
        let canvas = self.create_canvas(desc);
        let mini_canvas = MiniCanvasArc(Arc::new(canvas));
        self.table().push(mini_canvas).unwrap()
    }

    fn connect_graphics_context(
        &mut self,
        surface: Resource<MiniCanvasArc>,
        context: Resource<GraphicsContext>,
    ) {
        let surface = self.table().get(&surface).unwrap().clone();
        let graphics_context = self.table().get_mut(&context).unwrap();

        graphics_context.connect_display_api(Box::new(surface));
    }

    fn height(&mut self, surface: Resource<MiniCanvasArc>) -> u32 {
        let surface = self.table().get(&surface).unwrap();
        surface.height()
    }

    fn width(&mut self, surface: Resource<MiniCanvasArc>) -> u32 {
        let surface = self.table().get(&surface).unwrap();
        surface.width()
    }

    fn request_set_size(
        &mut self,
        surface: Resource<MiniCanvasArc>,
        width: Option<u32>,
        height: Option<u32>,
    ) {
        let surface = self.table().get(&surface).unwrap();
        surface.request_set_size(width, height);
    }

    fn subscribe_resize(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.canvas_resize_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.canvas_resize_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_resize(&mut self, surface: Resource<MiniCanvasArc>) -> Option<ResizeEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.canvas_resize_data.lock().unwrap().take()
    }

    fn subscribe_frame(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.frame_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |_d| {
                canvas
                    .frame_data
                    .lock()
                    .unwrap()
                    .replace(FrameEvent { nothing: false });
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_frame(&mut self, surface: Resource<MiniCanvasArc>) -> Option<FrameEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.frame_data.lock().unwrap().take()
    }

    fn subscribe_pointer_up(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.pointer_up_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.pointer_up_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_pointer_up(&mut self, surface: Resource<MiniCanvasArc>) -> Option<PointerEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.pointer_up_data.lock().unwrap().take()
    }

    fn subscribe_pointer_down(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.pointer_down_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.pointer_down_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_pointer_down(&mut self, surface: Resource<MiniCanvasArc>) -> Option<PointerEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.pointer_down_data.lock().unwrap().take()
    }

    fn subscribe_pointer_move(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.pointer_move_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.pointer_move_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_pointer_move(&mut self, surface: Resource<MiniCanvasArc>) -> Option<PointerEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.pointer_move_data.lock().unwrap().take()
    }

    fn subscribe_key_up(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.key_up_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.key_up_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_key_up(&mut self, surface: Resource<MiniCanvasArc>) -> Option<KeyEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.key_up_data.lock().unwrap().take()
    }

    fn subscribe_key_down(&mut self, surface: Resource<MiniCanvasArc>) -> Resource<Pollable> {
        let canvas = Arc::clone(&self.table().get(&surface).unwrap().0);
        let receiver = canvas.key_down_sender.new_receiver();
        let listener = self
            .table()
            .push(Listener::new(receiver, move |data| {
                canvas.key_down_data.lock().unwrap().replace(data);
            }))
            .unwrap();
        wasmtime_wasi::subscribe(self.table(), listener).unwrap()
    }

    fn get_key_down(&mut self, surface: Resource<MiniCanvasArc>) -> Option<KeyEvent> {
        let canvas = &self.table().get(&surface).unwrap().0;
        canvas.key_down_data.lock().unwrap().take()
    }

    fn drop(&mut self, _self_: Resource<MiniCanvasArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    receiver: Receiver<T>,
    on_data: F,
}

impl<T, F> Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    pub fn new(receiver: Receiver<T>, on_data: F) -> Self {
        Self { receiver, on_data }
    }
}

#[async_trait::async_trait] // TODO: remove async_trait crate once wasmtime drops it
impl<T, F> wasmtime_wasi::Pollable for Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        (self.on_data)(event);
    }
}
