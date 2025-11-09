use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};
use wasi_graphics_context_wasmtime::DisplayApi;

use async_broadcast::{Receiver, TrySendError};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wasmtime_wasi::p2::Pollable;
use wasmtime_wasi::{ResourceTable, WasiView};

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "winit")]
pub use winit::{create_wasi_winit_event_loop, WasiWinitEventLoop, WasiWinitEventLoopProxy};

pub trait HasDisplayAndWindowHandle: HasDisplayHandle + HasWindowHandle {}

impl<T: HasDisplayHandle + HasWindowHandle> HasDisplayAndWindowHandle for T {}

pub use crate::wasi::surface::surface::{
    FrameEvent, KeyEvent, PointerEvent, {CreateDesc as SurfaceDesc, ResizeEvent},
};

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    with: {
        "wasi:graphics-context/graphics-context": wasi_graphics_context_wasmtime::wasi::graphics_context::graphics_context,
        "wasi:surface/surface/surface": SurfaceArc,
        "wasi:io/poll/pollable": wasmtime_wasi::p2::DynPollable,
    },
});

pub struct Surface {
    pub window: Box<dyn DisplayApi + Send + Sync + 'static>,

    // Keeping inactive receivers to keep channels open.
    // See https://docs.rs/async-broadcast/0.7.1/async_broadcast/struct.InactiveReceiver.html
    pub pointer_up_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_up_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    pub pointer_down_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_down_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    pub pointer_move_sender: async_broadcast::Sender<PointerEvent>,
    _pointer_move_receiver: async_broadcast::InactiveReceiver<PointerEvent>,
    pub key_up_sender: async_broadcast::Sender<KeyEvent>,
    _key_up_receiver: async_broadcast::InactiveReceiver<KeyEvent>,
    pub key_down_sender: async_broadcast::Sender<KeyEvent>,
    _key_down_receiver: async_broadcast::InactiveReceiver<KeyEvent>,
    pub canvas_resize_sender: async_broadcast::Sender<ResizeEvent>,
    _canvas_resize_receiver: async_broadcast::InactiveReceiver<ResizeEvent>,
    pub frame_sender: async_broadcast::Sender<()>,
    _frame_receiver: async_broadcast::InactiveReceiver<()>,

    // TODO: remove once we get rid of pollable
    pub pointer_up_data: Mutex<Option<PointerEvent>>,
    pub pointer_down_data: Mutex<Option<PointerEvent>>,
    pub pointer_move_data: Mutex<Option<PointerEvent>>,
    pub key_up_data: Mutex<Option<KeyEvent>>,
    pub key_down_data: Mutex<Option<KeyEvent>>,
    pub canvas_resize_data: Mutex<Option<ResizeEvent>>,
    pub frame_data: Mutex<Option<FrameEvent>>,
}

impl Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Surface")
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

impl Surface {
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

    pub fn proxy(&self) -> SurfaceProxy {
        SurfaceProxy {
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
pub struct SurfaceProxy {
    pointer_up_sender: async_broadcast::Sender<PointerEvent>,
    pointer_down_sender: async_broadcast::Sender<PointerEvent>,
    pointer_move_sender: async_broadcast::Sender<PointerEvent>,
    key_up_sender: async_broadcast::Sender<KeyEvent>,
    key_down_sender: async_broadcast::Sender<KeyEvent>,
    canvas_resize_sender: async_broadcast::Sender<ResizeEvent>,
    frame_sender: async_broadcast::Sender<()>,
}

impl SurfaceProxy {
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

impl HasDisplayHandle for Surface {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.window.display_handle()
    }
}
impl HasWindowHandle for Surface {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.window.window_handle()
    }
}

impl DisplayApi for Surface {
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
pub struct SurfaceArc(pub Arc<Surface>);

impl HasDisplayHandle for SurfaceArc {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.0.display_handle()
    }
}
impl HasWindowHandle for SurfaceArc {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.0.window_handle()
    }
}

impl DisplayApi for SurfaceArc {
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
pub trait WasiSurfaceView: Send {
    fn table(&mut self) -> &mut ResourceTable;
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_>;
    fn create_canvas(&self, desc: SurfaceDesc) -> Surface;
}

#[repr(transparent)]
pub struct WasiSurfaceImpl<T: WasiSurfaceView>(pub T);

impl<T: WasiSurfaceView + 'static> wasmtime::component::HasData for WasiSurfaceImpl<T> {
    type Data<'a> = WasiSurfaceImpl<&'a mut T>;
}

impl<T: WasiSurfaceView> WasiView for WasiSurfaceImpl<T> {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(&mut self.0)
    }
}

impl<T: WasiSurfaceView> WasiSurfaceView for WasiSurfaceImpl<T> {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(&mut self.0)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(&mut self.0)
    }
    fn create_canvas(&self, desc: SurfaceDesc) -> Surface {
        T::create_canvas(&self.0, desc)
    }
}

impl<T: ?Sized + WasiSurfaceView> WasiSurfaceView for &mut T {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(self)
    }
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        T::ctx(self)
    }
    fn create_canvas(&self, desc: SurfaceDesc) -> Surface {
        T::create_canvas(self, desc)
    }
}

// Implement Host trait for the wrapped type
impl<T: WasiSurfaceView> wasi::surface::surface::Host for WasiSurfaceImpl<T> {}

impl<T: WasiSurfaceView> wasi::surface::surface::HostSurface for WasiSurfaceImpl<T> {
    fn new(&mut self, desc: SurfaceDesc) -> wasmtime::component::Resource<SurfaceArc> {
        let surface = self.0.create_canvas(desc);
        let surface = SurfaceArc(std::sync::Arc::new(surface));
        WasiSurfaceView::table(&mut self.0)
            .push(surface)
            .expect("failed to push surface to resource table")
    }
    fn connect_graphics_context(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
        context: wasmtime::component::Resource<wasi_graphics_context_wasmtime::Context>,
    ) {
        let surface = WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .clone();
        let graphics_context = WasiSurfaceView::table(&mut self.0)
            .get_mut(&context)
            .expect("invalid graphics context resource");
        graphics_context.connect_display_api(Box::new(surface));
    }

    fn height(&mut self, surface: wasmtime::component::Resource<SurfaceArc>) -> u32 {
        use wasi_graphics_context_wasmtime::DisplayApi;
        let surface = WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource");
        surface.height()
    }

    fn width(&mut self, surface: wasmtime::component::Resource<SurfaceArc>) -> u32 {
        use wasi_graphics_context_wasmtime::DisplayApi;
        let surface = WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource");
        surface.width()
    }

    fn request_set_size(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
        width: Option<u32>,
        height: Option<u32>,
    ) {
        use wasi_graphics_context_wasmtime::DisplayApi;
        let surface = WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource");
        surface.request_set_size(width, height);
    }

    fn subscribe_resize(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.canvas_resize_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .canvas_resize_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_resize(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<ResizeEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .canvas_resize_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_frame(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.frame_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |_d| {
                canvas
                    .frame_data
                    .lock()
                    .unwrap()
                    .replace(FrameEvent { nothing: false });
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_frame(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<FrameEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .frame_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_pointer_up(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.pointer_up_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .pointer_up_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_pointer_up(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<PointerEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .pointer_up_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_pointer_down(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.pointer_down_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .pointer_down_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_pointer_down(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<PointerEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .pointer_down_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_pointer_move(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.pointer_move_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .pointer_move_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_pointer_move(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<PointerEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .pointer_move_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_key_up(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.key_up_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .key_up_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_key_up(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<KeyEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .key_up_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn subscribe_key_down(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> wasmtime::component::Resource<wasi::surface::surface::Pollable> {
        let canvas = std::sync::Arc::clone(
            &WasiSurfaceView::table(&mut self.0)
                .get(&surface)
                .expect("invalid surface resource")
                .0,
        );
        let receiver = canvas.key_down_sender.new_receiver();
        let listener = WasiSurfaceView::table(&mut self.0)
            .push(Listener::new(receiver, move |data| {
                canvas
                    .key_down_data
                    .lock()
                    .expect("failed to acquire lock")
                    .replace(data);
            }))
            .unwrap();
        wasmtime_wasi::p2::subscribe(WasiSurfaceView::table(&mut self.0), listener)
            .expect("failed to subscribe to pollable")
    }

    fn get_key_down(
        &mut self,
        surface: wasmtime::component::Resource<SurfaceArc>,
    ) -> Option<KeyEvent> {
        let canvas = &WasiSurfaceView::table(&mut self.0)
            .get(&surface)
            .expect("invalid surface resource")
            .0;
        canvas
            .key_down_data
            .lock()
            .expect("failed to acquire lock")
            .take()
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<SurfaceArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

// Add to linker helper
pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> wasmtime::Result<()>
where
    T: WasiSurfaceView + 'static,
{
    fn get_impl<T: WasiSurfaceView>(t: &mut T) -> WasiSurfaceImpl<&mut T> {
        WasiSurfaceImpl(t)
    }
    wasi::surface::surface::add_to_linker::<T, WasiSurfaceImpl<T>>(l, get_impl)?;
    Ok(())
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
impl<T, F> Pollable for Listener<T, F>
where
    T: Debug + Clone + Send + Sync + 'static,
    F: Fn(T) + Send + Sync + 'static,
{
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        (self.on_data)(event);
    }
}
