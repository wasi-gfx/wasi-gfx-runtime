use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use component::webgpu::{
    key_events::KeyEvent, mini_canvas::ResizeEvent, pointer_events::PointerEvent,
};
use tokio::sync::broadcast::Sender;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use winit::{event::ElementState, event_loop::EventLoop, window::Window};

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
        "component:webgpu/webgpu/gpu-adapter": wgpu_core::id::AdapterId,
        "component:webgpu/webgpu/gpu-device": webgpu::Device,
        // queue is same as device
        "component:webgpu/webgpu/gpu-queue": webgpu::Device,
        "component:webgpu/webgpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "component:webgpu/webgpu/gpu-render-pass-encoder": wgpu_core::command::RenderPass,
        "component:webgpu/webgpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "component:webgpu/webgpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "component:webgpu/webgpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        "component:webgpu/webgpu/gpu-texture": wgpu_core::id::TextureId,
        "component:webgpu/webgpu/gpu-texture": graphics_context::WebgpuTexture,
        "component:webgpu/webgpu/gpu-texture-view": wgpu_core::id::TextureViewId,
        "component:webgpu/frame-buffer/frame-buffer": frame_buffer::FrameBuffer,
        "component:webgpu/pointer-events/pointer-up-listener": pointer_events::PointerUpListener,
        "component:webgpu/pointer-events/pointer-down-listener": pointer_events::PointerDownListener,
        "component:webgpu/pointer-events/pointer-move-listener": pointer_events::PointerMoveListener,
        "component:webgpu/key-events/key-up-listener": key_events::KeyUpListener,
        "component:webgpu/key-events/key-down-listener": key_events::KeyDownListener,
        "component:webgpu/animation-frame/frame-listener": animation_frame::AnimationFrameListener,
        "component:webgpu/graphics-context/graphics-context": graphics_context::GraphicsContext,
        "component:webgpu/graphics-context/graphics-context-buffer": graphics_context::GraphicsContextBuffer,
        "component:webgpu/mini-canvas/mini-canvas": mini_canvas::MiniCanvas,
        "component:webgpu/mini-canvas/resize-listener": mini_canvas::ResizeListener,
    },
});

struct HostState {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub sender: Sender<HostEvent>,
    pub instance: wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>,
    pub window: Window,
}

pub fn listen_to_events(event_loop: EventLoop<()>, sender: Sender<HostEvent>) {
    use winit::event::{Event, MouseButton, WindowEvent};

    let sender_2 = sender.clone();
    tokio::spawn(async move {
        loop {
            // winit doesn't provide frame callbacks.
            sender_2.send(HostEvent::Frame).unwrap();
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });

    let mut pointer_x: f64 = 0.0;
    let mut pointer_y: f64 = 0.0;

    event_loop.run(move |event, _target, _control_flow| {
        // *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {}
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                sender
                    .send(HostEvent::CanvasResizeEvent(ResizeEvent {
                        height: new_size.height,
                        width: new_size.width,
                    }))
                    .unwrap();
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
                let event = match state {
                    ElementState::Pressed => HostEvent::PointerDownEvent(PointerEvent {
                        x: pointer_x,
                        y: pointer_y,
                    }),
                    ElementState::Released => HostEvent::PointerUpEvent(PointerEvent {
                        x: pointer_x,
                        y: pointer_y,
                    }),
                };
                sender.send(event).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                #[allow(deprecated)]
                let event = match input.state {
                    ElementState::Pressed => HostEvent::KeyDownEvent(KeyEvent {
                        code: input
                            .virtual_keycode
                            .map(|k| format!("{k:?}"))
                            .unwrap_or_default(),
                        key: input.scancode.to_string(),
                        alt_key: input.modifiers.shift(),
                        ctrl_key: input.modifiers.ctrl(),
                        meta_key: input.modifiers.logo(),
                        shift_key: input.modifiers.shift(),
                    }),
                    ElementState::Released => HostEvent::KeyUpEvent(KeyEvent {
                        code: input
                            .virtual_keycode
                            .map(|k| format!("{k:?}"))
                            .unwrap_or_default(),
                        key: input.scancode.to_string(),
                        alt_key: input.modifiers.shift(),
                        ctrl_key: input.modifiers.ctrl(),
                        meta_key: input.modifiers.logo(),
                        shift_key: input.modifiers.shift(),
                    }),
                };
                sender.send(event).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                pointer_x = position.x;
                pointer_y = position.y;
                let event = HostEvent::PointerMoveEvent(PointerEvent {
                    x: pointer_x,
                    y: pointer_y,
                });
                sender.send(event).unwrap();
            }
            _ => (),
        }
    });
}

impl HostState {
    fn new(event_loop: &EventLoop<()>, sender: Sender<HostEvent>) -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            sender,
            instance: wgpu_core::global::Global::new(
                "webgpu",
                wgpu_core::identity::IdentityManagerFactory,
                wgpu_types::InstanceDescriptor {
                    backends: wgpu_types::Backends::all(),
                    flags: wgpu_types::InstanceFlags::from_build_config(),
                    dx12_shader_compiler: wgpu_types::Dx12Compiler::Fxc,
                    gles_minor_version: wgpu_types::Gles3MinorVersion::default(),
                },
            ),
            window: Window::new(event_loop).unwrap(),
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

impl ExampleImports for HostState {
    fn print(&mut self, s: String) -> wasmtime::Result<()> {
        println!("{s}");
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum HostEvent {
    PointerUpEvent(PointerEvent),
    PointerDownEvent(PointerEvent),
    PointerMoveEvent(PointerEvent),
    KeyUpEvent(KeyEvent),
    KeyDownEvent(KeyEvent),
    CanvasResizeEvent(ResizeEvent),
    Frame,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // can't drop receiver right away, that'll cause panics. No idea why.
    let (sender, _receiver) = tokio::sync::broadcast::channel::<HostEvent>(10);

    let args = RuntimeArgs::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    component::webgpu::webgpu::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::frame_buffer::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::animation_frame::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::pointer_events::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::key_events::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::graphics_context::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    component::webgpu::mini_canvas::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    preview2::bindings::io::poll::add_to_linker(&mut linker, |state| state)?;
    preview2::bindings::io::streams::add_to_linker(&mut linker, |state| state)?;

    Example::add_root_to_linker(&mut linker, |state: &mut HostState| state)?;

    let event = winit::event_loop::EventLoopBuilder::new().build();

    let host_state = HostState::new(&event, sender.clone());

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

    listen_to_events(event, sender);

    Ok(())
}
