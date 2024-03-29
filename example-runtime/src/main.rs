use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use async_broadcast::{InactiveReceiver, Sender};
use clap::Parser;
use wasi::webgpu::{key_events::KeyEvent, mini_canvas::ResizeEvent, pointer_events::PointerEvent};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use webgpu::GpuInstance;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{EventLoop, EventLoopProxy},
    window::Window,
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
        "wasi:webgpu/frame-buffer/frame-buffer": frame_buffer::FrameBuffer,
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
    // pub sender: Sender<HostEvent>,
    pub instance: Arc<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>,
    // pub window: Window,
    // pub event_loop_proxy: EventLoopProxy<()>,
    pub message_sender: MyMessageSender,
}

// new event loop should return (event_loop, message_sender)
// call event_loop.run on main thread
// message sender should be clonable
// message sender should have methods for each event type
// event_loop should be able to reply when done (window, after window creation)

// TODO: move to canvas
pub struct MyEventLoop {
    event_loop: EventLoop<MainThreadAction>,
    // proxy: EventLoopProxy<()>,
    senders: MainThreadMessageSenders,
}

#[derive(Debug)]
enum MainThreadAction {
    // TODO: maybe add a oneshot channel for response.
    CreateWindow,
}

// Using seperate event for channel so that not everynoe has to wake up for each event
struct MainThreadMessageSenders {
    // TODO: find better ids than u32
    // new window in an Arc<Mutex<Option<Window>>> becuase broadcast needs everything to be clone. Only the matching should take the window with `Option::take`
    new_window: Sender<(u32, Arc<Mutex<Option<Window>>>)>,
    pointer_up_event: Sender<(u32, PointerEvent)>,
    pointer_down_event: Sender<(u32, PointerEvent)>,
    pointer_move_event: Sender<(u32, PointerEvent)>,
    key_up_event: Sender<(u32, KeyEvent)>,
    key_down_event: Sender<(u32, KeyEvent)>,
    canvas_resize_event: Sender<(u32, ResizeEvent)>,
    frame: Sender<(u32, ())>,
}

#[derive(Clone)]
struct MainThreadMessageReceivers {
    // TODO: can probably work around this Arc<Mutex<_>>, have cloneable receivers
    new_window: InactiveReceiver<(u32, Arc<Mutex<Option<Window>>>)>,
    pointer_up_event: InactiveReceiver<(u32, PointerEvent)>,
    pointer_down_event: InactiveReceiver<(u32, PointerEvent)>,
    pointer_move_event: InactiveReceiver<(u32, PointerEvent)>,
    key_up_event: InactiveReceiver<(u32, KeyEvent)>,
    key_down_event: InactiveReceiver<(u32, KeyEvent)>,
    canvas_resize_event: InactiveReceiver<(u32, ResizeEvent)>,
    frame: InactiveReceiver<(u32, ())>,
}
impl MainThreadMessageReceivers {
    // TODO: enable
    // fn new_window(&self, id: u32) -> Stream<Window> {
    //     self.new_window.subscribe()
    // }
}
#[derive(Clone)]
pub struct MyMessageSender {
    proxy: EventLoopProxy<MainThreadAction>,
    receivers: MainThreadMessageReceivers,
}
impl MyMessageSender {
    // TODO: add message id TO ALL METHODS HERE since more then one guest can request a new window at a time
    pub async fn create_window(&self) -> Window {
        // self.sender.send(HostEvent::CreateWindow).unwrap();
        self.proxy
            .send_event(MainThreadAction::CreateWindow)
            .unwrap();
        let mut receiver = self.receivers.new_window.activate_cloned();
        let (id, window) = receiver.recv().await.unwrap();

        // TODO: validate id

        let window = window.lock().unwrap().take().unwrap();

        window
    }
}

pub fn create_event_loop() -> (MyEventLoop, MyMessageSender) {
    // TODO: remove Arc only here to make Clone since `broadcast: Clone`
    let (new_window_sender, new_window_receiver) = async_broadcast::broadcast(10);
    let (pointer_up_event_sender, pointer_up_event_receiver) = async_broadcast::broadcast(10);
    let (pointer_down_event_sender, pointer_down_event_receiver) = async_broadcast::broadcast(10);
    let (pointer_move_event_sender, pointer_move_event_receiver) = async_broadcast::broadcast(10);
    let (key_up_event_sender, key_up_event_receiver) = async_broadcast::broadcast(10);
    let (key_down_event_sender, key_down_event_receiver) = async_broadcast::broadcast(10);
    let (canvas_resize_event_sender, canvas_resize_event_receiver) = async_broadcast::broadcast(10);
    let (frame_sender, frame_receiver) = async_broadcast::broadcast(1);
    let senders = MainThreadMessageSenders {
        new_window: new_window_sender,
        pointer_up_event: pointer_up_event_sender,
        pointer_down_event: pointer_down_event_sender,
        pointer_move_event: pointer_move_event_sender,
        key_up_event: key_up_event_sender,
        key_down_event: key_down_event_sender,
        canvas_resize_event: canvas_resize_event_sender,
        frame: frame_sender,
    };
    let receivers = MainThreadMessageReceivers {
        new_window: new_window_receiver.deactivate(),
        pointer_up_event: pointer_up_event_receiver.deactivate(),
        pointer_down_event: pointer_down_event_receiver.deactivate(),
        pointer_move_event: pointer_move_event_receiver.deactivate(),
        key_up_event: key_up_event_receiver.deactivate(),
        key_down_event: key_down_event_receiver.deactivate(),
        canvas_resize_event: canvas_resize_event_receiver.deactivate(),
        frame: frame_receiver.deactivate(),
    };
    let event_loop = MyEventLoop {
        event_loop: winit::event_loop::EventLoopBuilder::<MainThreadAction>::with_user_event()
            .build(),
        senders,
    };
    let message_sender = MyMessageSender {
        proxy: event_loop.event_loop.create_proxy(),
        receivers,
    };
    (event_loop, message_sender)
}

impl MyEventLoop {
    pub fn run(self) {
        tokio::spawn(async move {
            loop {
                if let Err(_) = self.senders.frame.try_broadcast((0, ())) {
                    println!("skipping a frame")
                }
                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        });

        let mut pointer_x: f64 = 0.0;
        let mut pointer_y: f64 = 0.0;

        self.event_loop
            .run(move |event, event_loop, _control_flow| {
                match event {
                    Event::UserEvent(event) => {
                        // TODO: look at
                        // https://github.com/rust-windowing/winit/issues/413

                        match event {
                            MainThreadAction::CreateWindow => {
                                let window = winit::window::Window::new(event_loop).unwrap();
                                self.senders
                                    .new_window
                                    .try_broadcast((0, Arc::new(Mutex::new(Some(window)))))
                                    .unwrap();
                            }
                        }
                    }
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => {}
                    Event::WindowEvent {
                        event: WindowEvent::Resized(new_size),
                        ..
                    } => {
                        self.senders.canvas_resize_event.try_broadcast((
                            0,
                            ResizeEvent {
                                height: new_size.height,
                                width: new_size.width,
                            },
                        ));
                    }
                    Event::WindowEvent {
                        event:
                            WindowEvent::MouseInput {
                                button: MouseButton::Left,
                                state,
                                ..
                            },
                        ..
                    } => {
                        let event = PointerEvent {
                            x: pointer_x,
                            y: pointer_y,
                        };
                        let event = match state {
                            ElementState::Pressed => {
                                self.senders.pointer_down_event.try_broadcast((0, event));
                            }
                            ElementState::Released => {
                                self.senders
                                    .pointer_up_event
                                    .try_broadcast((0, event))
                                    .unwrap();
                            }
                        };
                    }
                    Event::WindowEvent {
                        event: WindowEvent::KeyboardInput { input, .. },
                        ..
                    } => {
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
                                self.senders
                                    .key_down_event
                                    .try_broadcast((0, event))
                                    .unwrap();
                            }
                            ElementState::Released => {
                                self.senders.key_up_event.try_broadcast((0, event)).unwrap();
                            }
                        };
                    }
                    Event::WindowEvent {
                        event: WindowEvent::CursorMoved { position, .. },
                        ..
                    } => {
                        pointer_x = position.x;
                        pointer_y = position.y;
                        let event = PointerEvent {
                            x: pointer_x,
                            y: pointer_y,
                        };
                        if let Err(_) = self.senders.pointer_move_event.try_broadcast((0, event)) {
                            println!("skipping a pointer move event");
                        }
                    }
                    _ => (),
                }
            });
    }
}

impl HostState {
    fn new(message_sender: MyMessageSender) -> Self {
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
            message_sender,
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

impl GpuInstance for HostState {
    fn instance(&self) -> &wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory> {
        &self.instance
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

    let (event_loop, message_sender) = create_event_loop();
    let host_state = HostState::new(message_sender);

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

    event_loop.run();

    Ok(())
}
