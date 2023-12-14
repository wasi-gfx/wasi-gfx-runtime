use std::time::Duration;

use anyhow::Context;
use clap::Parser;
use tokio::sync::broadcast::Sender;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use winit::{event::ElementState, event_loop::EventLoop, window::Window};

use wasmtime_wasi::preview2::{self, Table, WasiCtx, WasiCtxBuilder, WasiView};
mod pointer_events;
mod request_animation_frame;
mod webgpu_host;

#[derive(clap::Parser, Debug)]
struct RuntimeArgs {
    /// The example name
    #[arg(long)]
    example: String,
}

// needed for wasmtime::component::bindgen! as it only looks in the current crate.
pub(crate) use wgpu;

wasmtime::component::bindgen!({
    path: "../wit/",
    world: "example",
    async: {
        // only_imports: [],
        except_imports: [],
    },
    with: {
        "wasi:io/poll": preview2::bindings::io::poll,
        "wasi:io/streams": preview2::bindings::io::streams,

        "component:webgpu/webgpu/gpu-adapter": wgpu::Adapter,
        "component:webgpu/webgpu/gpu-device": webgpu_host::DeviceAndQueue,
        "component:webgpu/webgpu/gpu-device-queue": webgpu_host::DeviceAndQueue,
        "component:webgpu/webgpu/displayable-entity": wgpu::Surface,
        "component:webgpu/webgpu/gpu-command-encoder": wgpu::CommandEncoder,
        "component:webgpu/webgpu/gpu-shader-module": wgpu::ShaderModule,
        // "component:webgpu/webgpu/gpu-render-pass": wgpu::RenderPass,
        "component:webgpu/webgpu/gpu-render-pipeline": wgpu::RenderPipeline,
        "component:webgpu/webgpu/gpu-command-buffer": wgpu::CommandBuffer,
        "component:webgpu/webgpu/displayable-entity-view": webgpu_host::DisplayableEntityView,
        "component:webgpu/pointer-events/pointer-up": pointer_events::HostPointerEvent,
        "component:webgpu/request-animation-frame/frame": request_animation_frame::FrameThis,
    },
});

struct HostState {
    pub table: Table,
    pub ctx: WasiCtx,
    pub sender: Sender<HostEvent>,
    pub instance: wgpu::Instance,
    pub window: Window,
}

pub fn listen_to_events(event_loop: EventLoop<()>, sender: Sender<HostEvent>) {
    use winit::event::{Event, MouseButton, WindowEvent};

    let sender_2 = sender.clone();
    tokio::spawn(async move {
        loop {
            sender_2.send(HostEvent::Frame).unwrap();
            tokio::time::sleep(Duration::from_millis(16)).await;
        }
    });

    event_loop.run(move |event, _target, _control_flow| {
        // *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {}

            Event::WindowEvent {
                event: WindowEvent::Resized(_new_size),
                ..
            } => {
                // println!("Resized to {:?}", new_size);
            }

            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => {
                let event = HostEvent::PointerEvent { x: 0, y: 0 };
                let sender = sender.clone();
                sender.send(event).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: _, .. },
                ..
            } => {
                // // println!("mouse CursorMoved {position:?}");
                // // *self.pointer_up.lock().unwrap() = Some((position.x as i32, position.y as i32));
                // let event = HostEvent::PointerEvent {
                //     x: position.x as i32,
                //     y: position.y as i32,
                // };
                // println!("herer 1");
                // let sender = sender.clone();
                // tokio::spawn(async move {
                //     println!("herer 2");
                //     sender.send(event).unwrap();
                // });
            }
            // Event::RedrawRequested(_) => {
            //     window.request_redraw();
            //     println!("animation frame {:?}", t.elapsed());
            // },
            _ => (),
        }
    });
}

impl HostState {
    fn new(event_loop: &EventLoop<()>, sender: Sender<HostEvent>) -> Self {
        Self {
            table: Table::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            sender,
            instance: Default::default(),
            window: Window::new(event_loop).unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl WasiView for HostState {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.ctx
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

#[async_trait::async_trait]
impl ExampleImports for HostState {
    async fn print(&mut self, s: String) -> wasmtime::Result<()> {
        println!("{s}");
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum HostEvent {
    PointerEvent { x: i32, y: i32 },
    Frame,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // can't drop receiver right away, that'll cause panics. No idea why.
    let (sender, _receiver) = tokio::sync::broadcast::channel::<HostEvent>(10);

    let args = RuntimeArgs::parse();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    component::webgpu::webgpu::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    component::webgpu::request_animation_frame::add_to_linker(
        &mut linker,
        |state: &mut HostState| state,
    )?;

    component::webgpu::pointer_events::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    // preview2::bindings::io::poll::add_to_linker(&mut linker, |state| &mut state.frame_host)?;
    // preview2::bindings::io::streams::add_to_linker(&mut linker, |state| &mut state.frame_host)?;
    // preview2::bindings::io::poll::add_to_linker(&mut linker, |state| &mut state.pointer_events_host)?;
    // preview2::bindings::io::streams::add_to_linker(&mut linker, |state| &mut state.pointer_events_host)?;
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
