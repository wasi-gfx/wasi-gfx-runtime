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
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wasmtime::component::Resource;
use wasmtime_wasi::WasiView;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::ModifiersState,
    window::{Window, WindowAttributes, WindowId},
};

mod animation_frame;
mod key_events;
mod pointer_events;

pub trait WasiMiniCanvasView: WasiView {
    fn main_thread_proxy(&self) -> &MainThreadProxy;
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

#[derive(Debug)]
pub struct MiniCanvas {
    pub offscreen: bool,
    pub window: Window,
}

impl MiniCanvas {
    pub fn create_event_loop() -> (MainThreadLoop, MainThreadProxy) {
        let event_loop = MainThreadLoop {
            event_loop: winit::event_loop::EventLoop::<MainThreadAction>::with_user_event()
                .build()
                .unwrap(),
        };
        let message_sender = MainThreadProxy {
            proxy: event_loop.event_loop.create_proxy(),
        };
        (event_loop, message_sender)
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
        self.window.inner_size().height
    }

    fn width(&self) -> u32 {
        self.window.inner_size().width
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

pub struct MainThreadLoop {
    event_loop: EventLoop<MainThreadAction>,
}

impl MainThreadLoop {
    /// This has to be run on the main thread.
    /// This call will block the tread.
    pub fn run(self) {
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

        #[derive(Default)]
        struct App {
            pointer_pos: HashMap<WindowId, (f64, f64)>,
            modifiers: HashMap<WindowId, ModifiersState>,
            pointer_up_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>>,
            pointer_down_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>>,
            pointer_move_senders: HashMap<WindowId, async_broadcast::Sender<PointerEvent>>,
            key_up_senders: HashMap<WindowId, async_broadcast::Sender<KeyEvent>>,
            key_down_senders: HashMap<WindowId, async_broadcast::Sender<KeyEvent>>,
            canvas_resize_senders: HashMap<WindowId, async_broadcast::Sender<ResizeEvent>>,
            frame_senders: Arc<Mutex<HashMap<WindowId, async_broadcast::Sender<()>>>>,
        }

        impl ApplicationHandler<MainThreadAction> for App {
            fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
                // TODO:
            }

            fn user_event(&mut self, event_loop: &ActiveEventLoop, event: MainThreadAction) {
                match event {
                    MainThreadAction::CreateWindow(response_channel) => {
                        let window = event_loop
                            .create_window(WindowAttributes::default())
                            .unwrap();
                        // TODO: remove when window is drooped.
                        self.pointer_pos.insert(window.id(), (0.0, 0.0));
                        response_channel.send(window).unwrap();
                    }
                    MainThreadAction::CreatePointerUpListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.pointer_up_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreatePointerDownListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.pointer_down_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreatePointerMoveListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.pointer_move_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreateKeyUpListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.key_up_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreateKeyDownListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.key_down_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreateCanvasResizeListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.canvas_resize_senders.insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                    MainThreadAction::CreateFrameListener(window_id, res) => {
                        let (sender, receiver) = async_broadcast::broadcast(5);
                        self.frame_senders.lock().unwrap().insert(window_id, sender);
                        res.send(receiver).unwrap();
                    }
                }
            }

            fn window_event(
                &mut self,
                _event_loop: &ActiveEventLoop,
                window_id: WindowId,
                event: WindowEvent,
            ) {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        self.pointer_pos
                            .insert(window_id, (position.x, position.y))
                            .unwrap();
                        let event = PointerEvent {
                            x: position.x,
                            y: position.y,
                        };

                        if let Some(sender) = self.pointer_move_senders.get(&window_id) {
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
                    WindowEvent::ModifiersChanged(modifiers) => {
                        self.modifiers.insert(window_id, modifiers.state());
                    }
                    WindowEvent::KeyboardInput { event: input, .. } => {
                        let modifiers = self.modifiers.get(&window_id).unwrap();
                        let event = KeyEvent {
                            code: match input.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => format!("{code:?}"),
                                winit::keyboard::PhysicalKey::Unidentified(_) => todo!(),
                            },
                            key: match input.logical_key {
                                winit::keyboard::Key::Character(char) => char.to_string(),
                                _ => todo!(),
                            },
                            alt_key: modifiers.alt_key(),
                            ctrl_key: modifiers.control_key(),
                            meta_key: modifiers.super_key(),
                            shift_key: modifiers.shift_key(),
                        };
                        match input.state {
                            ElementState::Pressed => {
                                if let Some(sender) = self.key_down_senders.get(&window_id) {
                                    unwrap_unless_inactive(sender.try_broadcast(event));
                                }
                            }
                            ElementState::Released => {
                                if let Some(sender) = self.key_up_senders.get(&window_id) {
                                    unwrap_unless_inactive(sender.try_broadcast(event));
                                }
                            }
                        };
                    }
                    WindowEvent::MouseInput { state, .. } => {
                        let (pointer_x, pointer_y) = self.pointer_pos.get(&window_id).unwrap();
                        let event = PointerEvent {
                            x: *pointer_x,
                            y: *pointer_y,
                        };
                        match state {
                            ElementState::Pressed => {
                                if let Some(sender) = self.pointer_down_senders.get(&window_id) {
                                    unwrap_unless_inactive(sender.try_broadcast(event));
                                }
                            }
                            ElementState::Released => {
                                if let Some(sender) = self.pointer_up_senders.get(&window_id) {
                                    unwrap_unless_inactive(sender.try_broadcast(event));
                                }
                            }
                        };
                    }
                    WindowEvent::Resized(new_size) => {
                        if let Some(sender) = self.canvas_resize_senders.get(&window_id) {
                            unwrap_unless_inactive(sender.try_broadcast(ResizeEvent {
                                height: new_size.height,
                                width: new_size.width,
                            }));
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut app = App::default();
        app.frame_senders = Arc::clone(&frame_senders);
        self.event_loop.run_app(&mut app).unwrap();
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
    fn new(&mut self, desc: CreateDesc) -> Resource<MiniCanvasArc> {
        let window = block_on(self.main_thread_proxy().create_window());
        let mini_canvas = MiniCanvasArc(Arc::new(MiniCanvas {
            offscreen: desc.offscreen,
            window,
        }));
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
        let window_id = self.table().get(&mini_canvas).unwrap().0.window.id();
        // TODO: await instead of block_on
        let receiver = block_on(
            self.main_thread_proxy()
                .create_canvas_resize_listener(window_id),
        );
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
