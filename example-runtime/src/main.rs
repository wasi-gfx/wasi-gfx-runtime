use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use async_broadcast::TrySendError;
use clap::Parser;
use wasi::webgpu::{key_events::KeyEvent, mini_canvas::ResizeEvent, pointer_events::PointerEvent};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use webgpu::HasGpuInstance;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

use wasmtime_wasi::preview2::{self, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
mod animation_frame;
mod frame_buffer;
mod graphics_context;
mod key_events;
mod mini_canvas;
mod pointer_events;
mod webgpu;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) type Backend = wgpu_core::api::Vulkan;

#[cfg(target_os = "windows")]
pub(crate) type Backend = wgpu_core::api::Dx12;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) type Backend = wgpu_core::api::Metal;

#[cfg(all(
    not(target_os = "linux"),
    not(target_os = "android"),
    not(target_os = "windows"),
    not(target_os = "macos"),
    not(target_os = "ios"),
))]
pub(crate) type Backend = wgpu_core::api::Gl;

#[derive(clap::Parser, Debug)]
struct RuntimeArgs {
    /// The example name
    #[arg(long)]
    example: String,
}

// needed for wasmtime::component::bindgen! as it only looks in the current crate.
pub(crate) use wgpu_core;
pub(crate) use wgpu_types;

wasmtime::component::bindgen!({
    path: "../wit/",
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
        "wasi:io/streams": preview2::bindings::io::streams,
        "wasi:webgpu/webgpu/gpu-adapter": wgpu_core::id::AdapterId,
        "wasi:webgpu/webgpu/gpu-device": webgpu::Device,
        // queue is same as device
        "wasi:webgpu/webgpu/gpu-queue": webgpu::Device,
        "wasi:webgpu/webgpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "wasi:webgpu/webgpu/gpu-render-pass-encoder": wgpu_core::command::RenderPass,
        "wasi:webgpu/webgpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "wasi:webgpu/webgpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "wasi:webgpu/webgpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        // "wasi:webgpu/webgpu/gpu-buffer": wgpu_core::id::BufferId,
        "wasi:webgpu/webgpu/gpu-buffer": webgpu::Buffer,
        "wasi:webgpu/webgpu/remote-buffer": webgpu::Buffer,
        "wasi:webgpu/webgpu/gpu-pipeline-layout": wgpu_core::id::PipelineLayoutId,
        "wasi:webgpu/webgpu/gpu-bind-group-layout": wgpu_core::id::BindGroupLayoutId,
        "wasi:webgpu/webgpu/gpu-sampler": wgpu_core::id::SamplerId,
        "wasi:webgpu/webgpu/gpu-supported-features": wgpu_types::Features,
        "wasi:webgpu/webgpu/gpu-texture": wgpu_core::id::TextureId,
        "wasi:webgpu/webgpu/gpu-bind-group": wgpu_core::id::BindGroupId,
        "wasi:webgpu/webgpu/gpu-texture-view": wgpu_core::id::TextureViewId,
        "wasi:webgpu/frame-buffer/surface": frame_buffer::FBSurfaceArc,
        "wasi:webgpu/frame-buffer/frame-buffer": frame_buffer::FBBuffer,
        "wasi:webgpu/pointer-events/pointer-up-listener": pointer_events::PointerUpListener,
        "wasi:webgpu/pointer-events/pointer-down-listener": pointer_events::PointerDownListener,
        "wasi:webgpu/pointer-events/pointer-move-listener": pointer_events::PointerMoveListener,
        "wasi:webgpu/key-events/key-up-listener": key_events::KeyUpListener,
        "wasi:webgpu/key-events/key-down-listener": key_events::KeyDownListener,
        "wasi:webgpu/animation-frame/frame-listener": animation_frame::AnimationFrameListener,
        "wasi:webgpu/graphics-context/graphics-context": graphics_context::GraphicsContext,
        "wasi:webgpu/graphics-context/graphics-context-buffer": graphics_context::GraphicsContextBuffer,
        "wasi:webgpu/mini-canvas/mini-canvas": mini_canvas::MiniCanvasArc,
        "wasi:webgpu/mini-canvas/resize-listener": mini_canvas::ResizeListener,
    },
});

struct HostState {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub instance: Arc<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>,
    pub main_thread_proxy: MainThreadProxy,
}

// new event loop should return (event_loop, message_sender)
// call event_loop.run on main thread
// message sender should be clonable
// message sender should have methods for each event type
// event_loop should be able to reply when done (window, after window creation)

// TODO: move to canvas
pub struct MainThreadLoop {
    event_loop: EventLoop<MainThreadAction>,
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

impl MainThreadLoop {
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
            tokio::spawn(async move {
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
                    tokio::time::sleep(Duration::from_millis(16)).await;
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

fn unwrap_unless_inactive<T>(res: Result<Option<T>, TrySendError<T>>) {
    if let Err(e) = &res {
        if let TrySendError::Inactive(_) = e {
            return;
        }
    }
    res.unwrap();
}

impl HostState {
    fn new(main_thread_proxy: MainThreadProxy) -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            instance: Arc::new(wgpu_core::global::Global::new(
                "webgpu",
                wgpu_core::identity::IdentityManagerFactory,
                wgpu_types::InstanceDescriptor {
                    backends: wgpu_types::Backends::all(),
                    flags: wgpu_types::InstanceFlags::from_build_config(),
                    dx12_shader_compiler: wgpu_types::Dx12Compiler::Fxc,
                    gles_minor_version: wgpu_types::Gles3MinorVersion::default(),
                },
            )),
            main_thread_proxy,
        }
    }
}

#[async_trait::async_trait]
impl WasiView for HostState {
    fn table(&self) -> &ResourceTable {
        &self.table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl HasGpuInstance for HostState {
    fn instance(
        &self,
    ) -> Arc<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>> {
        Arc::clone(&self.instance)
    }
}

impl ExampleImports for HostState {
    fn print(&mut self, s: String) -> wasmtime::Result<()> {
        println!("{s}");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args = RuntimeArgs::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    wasi::webgpu::webgpu::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::frame_buffer::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::animation_frame::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::pointer_events::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::key_events::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::graphics_context::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi::webgpu::mini_canvas::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    preview2::bindings::io::poll::add_to_linker(&mut linker, |state| state)?;
    preview2::bindings::io::streams::add_to_linker(&mut linker, |state| state)?;

    Example::add_root_to_linker(&mut linker, |state: &mut HostState| state)?;

    let (main_thread_loop, main_thread_proxy) = create_event_loop();
    let host_state = HostState::new(main_thread_proxy);

    let mut store = Store::new(&engine, host_state);

    let wasm_path = format!("../example-apps/{}/out.wasm", args.example);

    let component =
        Component::from_file(&engine, &wasm_path).context("Component file not found")?;

    let (instance, _) = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    tokio::spawn(async move {
        instance.call_start(&mut store).await.unwrap();
    });

    main_thread_loop.run();

    Ok(())
}
