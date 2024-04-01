use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
use wasi_graphics_context_wasmtime::DisplayApi;

use crate::wasi::webgpu::{
    key_events::KeyEvent,
    mini_canvas::{self, CreateDesc, GraphicsContext, Pollable, ResizeEvent},
    pointer_events::PointerEvent,
};
use async_broadcast::{Receiver, TrySendError};
use futures::executor::block_on;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::preview2::{self, WasiView};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

mod animation_frame;
mod key_events;
mod pointer_events;

// pub use wasi::webgpu::mini_canvas::add_to_linker;
pub fn add_to_linker<T, U>(
    linker: &mut wasmtime::component::Linker<T>,
    get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
) -> wasmtime::Result<()>
where
    U: wasi::webgpu::mini_canvas::Host
        + wasi::webgpu::animation_frame::Host
        + wasi::webgpu::pointer_events::Host
        + wasi::webgpu::key_events::Host
        + Send,
    T: Send,
{
    wasi::webgpu::mini_canvas::add_to_linker(linker, get)?;
    wasi::webgpu::animation_frame::add_to_linker(linker, get)?;
    wasi::webgpu::pointer_events::add_to_linker(linker, get)?;
    wasi::webgpu::key_events::add_to_linker(linker, get)?;
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
        "wasi:io/poll": preview2::bindings::io::poll,
        "wasi:io/streams": preview2::bindings::io::stream,
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

#[derive(Debug)]
pub struct MiniCanvas {
    pub offscreen: bool,
    pub window: Window,
}

impl MiniCanvas {
    pub fn create_event_loop() -> (MainThreadLoop, MainThreadProxy) {
        let event_loop = MainThreadLoop {
            event_loop: winit::event_loop::EventLoopBuilder::<MainThreadAction>::with_user_event()
                .build(),
        };
        let message_sender = MainThreadProxy {
            proxy: event_loop.event_loop.create_proxy(),
        };
        (event_loop, message_sender)
    }
}

unsafe impl HasRawDisplayHandle for MiniCanvas {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.window.raw_display_handle()
    }
}
unsafe impl HasRawWindowHandle for MiniCanvas {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.window.raw_window_handle()
    }
}

impl DisplayApi for MiniCanvas {
    fn height(&self) -> u32 {
        self.window.inner_size().height
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
    }
}

// TODO: instead of Arc, maybe have a global list of windows and ids? That ways it's same as webgpu, but might be harder to handle? Would likely also require a Mutex.
#[derive(Clone)]
pub struct MiniCanvasArc(pub Arc<MiniCanvas>);

unsafe impl HasRawDisplayHandle for MiniCanvasArc {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.0.raw_display_handle()
    }
}
unsafe impl HasRawWindowHandle for MiniCanvasArc {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.0.raw_window_handle()
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

pub struct MainThreadLoop {
    event_loop: EventLoop<MainThreadAction>,
}

impl MainThreadLoop {
    /// This has to be run on the main thread.
    /// This call will block the tread.
    pub fn run(self) {
        let mut pointer_pos: HashMap<WindowId, (f64, f64)> = HashMap::new();
        let mut pointer_up_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>> =
            HashMap::new();
        let mut pointer_down_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>> =
            HashMap::new();
        let mut pointer_move_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>> =
            HashMap::new();
        let mut key_up_senders: HashMap<WindowId, async_broadcast::Sender<KeyEvent>> =
            HashMap::new();
        let mut key_down_senders: HashMap<WindowId, async_broadcast::Sender<KeyEvent>> =
            HashMap::new();
        let mut canvas_resize_senders: HashMap<WindowId, async_broadcast::Sender<ResizeEvent>> =
            HashMap::new();
        let frame_senders: Arc<Mutex<HashMap<WindowId, async_broadcast::Sender<()>>>> =
            Default::default();

        {
            let frame_senders = Arc::clone(&frame_senders);
            thread::spawn(move || {
                loop {
                    for (_, sender) in frame_senders.lock().unwrap().iter() {
                        if let Err(e) = sender.try_broadcast(()) {
                            match e {
                                TrySendError::Full(_) => {
                                    println!("skipping a pointer move event");
                                }
                                TrySendError::Inactive(_) => {
                                    // don't care
                                }
                                TrySendError::Closed(_) => {
                                    panic!("Channel closed")
                                }
                            }
                        }
                    }
                    sleep(Duration::from_millis(16));
                }
            });
        }

        self.event_loop
            .run(move |event, event_loop, _control_flow| {
                match event {
                    Event::UserEvent(event) => match event {
                        MainThreadAction::CreateWindow(response_channel) => {
                            let window = winit::window::Window::new(event_loop).unwrap();
                            // TODO: remove when window is drooped.
                            pointer_pos.insert(window.id(), (0.0, 0.0));
                            response_channel.send(window).unwrap();
                        }
                        MainThreadAction::CreatePointerUpListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            pointer_up_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreatePointerDownListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            pointer_down_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreatePointerMoveListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            pointer_move_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreateKeyUpListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            key_up_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreateKeyDownListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            key_down_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreateCanvasResizeListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            canvas_resize_senders.insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                        MainThreadAction::CreateFrameListener(window_id, res) => {
                            let (sender, receiver) = async_broadcast::broadcast(5);
                            frame_senders.lock().unwrap().insert(window_id, sender);
                            res.send(receiver).unwrap();
                        }
                    },
                    Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            pointer_pos
                                .insert(window_id, (position.x, position.y))
                                .unwrap();
                            let event = PointerEvent {
                                x: position.x,
                                y: position.y,
                            };

                            if let Some(sender) = pointer_move_senders.get(&window_id) {
                                if let Err(e) = sender.try_broadcast(event) {
                                    match e {
                                        TrySendError::Full(_) => {
                                            println!("skipping a pointer move event");
                                        }
                                        TrySendError::Inactive(_) => {
                                            // don't care
                                        }
                                        TrySendError::Closed(_) => {
                                            panic!("Channel closed")
                                        }
                                    }
                                }
                            }
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            #[allow(deprecated)]
                            let event = KeyEvent {
                                code: input
                                    .virtual_keycode
                                    .map(|k| format!("{k:?}"))
                                    .unwrap_or_default(),
                                key: input.scancode.to_string(),
                                alt_key: input.modifiers.shift(),
                                ctrl_key: input.modifiers.ctrl(),
                                meta_key: input.modifiers.logo(),
                                shift_key: input.modifiers.shift(),
                            };
                            match input.state {
                                ElementState::Pressed => {
                                    if let Some(sender) = key_down_senders.get(&window_id) {
                                        unwrap_unless_inactive(sender.try_broadcast(event));
                                    }
                                }
                                ElementState::Released => {
                                    if let Some(sender) = key_up_senders.get(&window_id) {
                                        unwrap_unless_inactive(sender.try_broadcast(event));
                                    }
                                }
                            };
                        }
                        WindowEvent::MouseInput { state, .. } => {
                            let (pointer_x, pointer_y) = pointer_pos.get(&window_id).unwrap();
                            let event = PointerEvent {
                                x: *pointer_x,
                                y: *pointer_y,
                            };
                            match state {
                                ElementState::Pressed => {
                                    if let Some(sender) = pointer_down_senders.get(&window_id) {
                                        unwrap_unless_inactive(sender.try_broadcast(event));
                                    }
                                }
                                ElementState::Released => {
                                    if let Some(sender) = pointer_up_senders.get(&window_id) {
                                        unwrap_unless_inactive(sender.try_broadcast(event));
                                    }
                                }
                            };
                        }
                        WindowEvent::Resized(new_size) => {
                            if let Some(sender) = canvas_resize_senders.get(&window_id) {
                                unwrap_unless_inactive(sender.try_broadcast(ResizeEvent {
                                    height: new_size.height,
                                    width: new_size.width,
                                }));
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            });
    }
}

#[derive(Clone)]
pub struct MainThreadProxy {
    proxy: EventLoopProxy<MainThreadAction>,
}

impl MainThreadProxy {
    pub async fn create_window(&self) -> Window {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateWindow(sender))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_pointer_up_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<PointerEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreatePointerUpListener(window_id, sender))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_pointer_down_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<PointerEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreatePointerDownListener(
                window_id, sender,
            ))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_pointer_move_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<PointerEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreatePointerMoveListener(
                window_id, sender,
            ))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_key_up_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<KeyEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateKeyUpListener(window_id, sender))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_key_down_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<KeyEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateKeyDownListener(window_id, sender))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_canvas_resize_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<ResizeEvent> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateCanvasResizeListener(
                window_id, sender,
            ))
            .unwrap();
        receiver.await.unwrap()
    }
    pub async fn create_frame_listener(
        &self,
        window_id: WindowId,
    ) -> async_broadcast::Receiver<()> {
        let (sender, receiver) = oneshot::channel();
        self.proxy
            .send_event(MainThreadAction::CreateFrameListener(window_id, sender))
            .unwrap();
        receiver.await.unwrap()
    }
}

pub trait HasMainThreadProxy {
    fn main_thread_proxy(&self) -> &MainThreadProxy;
}

#[derive(Debug)]
enum MainThreadAction {
    CreateWindow(oneshot::Sender<Window>),
    CreatePointerUpListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<PointerEvent>>,
    ),
    CreatePointerDownListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<PointerEvent>>,
    ),
    CreatePointerMoveListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<PointerEvent>>,
    ),
    CreateKeyUpListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<KeyEvent>>,
    ),
    CreateKeyDownListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<KeyEvent>>,
    ),
    CreateCanvasResizeListener(
        WindowId,
        oneshot::Sender<async_broadcast::Receiver<ResizeEvent>>,
    ),
    CreateFrameListener(WindowId, oneshot::Sender<async_broadcast::Receiver<()>>),
}

fn unwrap_unless_inactive<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(e) = &res {
        if let TrySendError::Inactive(_) = e {
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
impl preview2::Subscribe for ResizeListener {
    async fn ready(&mut self) {
        let event = self.receiver.recv().await.unwrap();
        *self.data.lock().unwrap() = Some(event);
    }
}

// wasmtime
impl<T: WasiView + HasMainThreadProxy> mini_canvas::Host for T {}

#[async_trait::async_trait]
impl<T: WasiView + HasMainThreadProxy> mini_canvas::HostMiniCanvas for T {
    fn new(&mut self, desc: CreateDesc) -> wasmtime::Result<Resource<MiniCanvasArc>> {
        let window = block_on(self.main_thread_proxy().create_window());
        let mini_canvas = MiniCanvasArc(Arc::new(MiniCanvas {
            offscreen: desc.offscreen,
            window,
        }));
        Ok(self.table_mut().push(mini_canvas).unwrap())
    }

    fn connect_graphics_context(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
        context: Resource<GraphicsContext>,
    ) -> wasmtime::Result<()> {
        let mini_canvas = self.table().get(&mini_canvas).unwrap().clone();
        let graphics_context = self.table_mut().get_mut(&context).unwrap();

        graphics_context.connect_display_api(Box::new(mini_canvas));
        Ok(())
    }

    fn resize_listener(
        &mut self,
        mini_canvas: Resource<MiniCanvasArc>,
    ) -> wasmtime::Result<Resource<ResizeListener>> {
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        // TODO: await instead of block_on
        let receiver = block_on(
            self.main_thread_proxy()
                .create_canvas_resize_listener(window_id),
        );
        Ok(self
            .table_mut()
            .push(ResizeListener {
                receiver,
                data: Default::default(),
            })
            .unwrap())
    }

    fn height(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> wasmtime::Result<u32> {
        let mini_canvas = self.table().get(&mini_canvas).unwrap();
        Ok(mini_canvas.height())
    }

    fn width(&mut self, mini_canvas: Resource<MiniCanvasArc>) -> wasmtime::Result<u32> {
        let mini_canvas = self.table().get(&mini_canvas).unwrap();
        Ok(mini_canvas.width())
    }

    fn drop(&mut self, _self_: Resource<MiniCanvasArc>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl<T: WasiView + HasMainThreadProxy> mini_canvas::HostResizeListener for T {
    fn subscribe(
        &mut self,
        pointer_down: Resource<ResizeListener>,
    ) -> wasmtime::Result<Resource<Pollable>> {
        Ok(preview2::subscribe(self.table_mut(), pointer_down).unwrap())
    }
    fn get(
        &mut self,
        pointer_down: Resource<ResizeListener>,
    ) -> wasmtime::Result<Option<ResizeEvent>> {
        let pointer_down = self.table().get(&pointer_down).unwrap();
        Ok(pointer_down.data.lock().unwrap().take())
    }
    fn drop(&mut self, _self_: Resource<ResizeListener>) -> wasmtime::Result<()> {
        Ok(())
    }
}
