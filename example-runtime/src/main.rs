use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use clap::Parser;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use winit::{event::ElementState, event_loop::EventLoop};

use crate::webgpu_host::WebGpuHost;
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

wasmtime::component::bindgen!({
    path: "../wit/",
    world: "example",
    async: {
        only_imports: [],
    },
    with: {
        "wasi:io/poll": preview2::bindings::io::poll,
        "wasi:io/streams": preview2::bindings::io::streams,
     },
});

struct HostState {
    pub web_gpu_host: WebGpuHost<'static>,
    pub table: Table,
    pub ctx: WasiCtx,
    pub events: Arc<HostStateEvents>,
}

struct HostStateEvents {
    pub pointer_up: Mutex<Option<(i32, i32)>>,
    pub frame: Mutex<Option<()>>,
}

impl HostStateEvents {
    pub fn new() -> Self {
        Self {
            pointer_up: Mutex::new(None),
            frame: Mutex::new(None),
        }
    }

    pub fn listen_to_events(self: Arc<Self>, event_loop: EventLoop<()>) {
        use winit::event::{Event, MouseButton, WindowEvent};

        let state = Arc::clone(&self);
        std::thread::spawn(move || loop {
            let mut frame = state.frame.lock().unwrap();
            *frame = Some(());
            drop(frame);
            std::thread::sleep(Duration::from_millis(16));
        });

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
                    println!("Resized to {:?}", new_size);
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
                    println!("mouse click");
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    println!("mouse CursorMoved {position:?}");
                    *self.pointer_up.lock().unwrap() = Some((position.x as i32, position.y as i32));
                }
                // Event::RedrawRequested(_) => {
                //     window.request_redraw();
                //     println!("animation frame {:?}", t.elapsed());
                // },
                _ => (),
            }
        });
    }
}

impl HostState {
    fn new(event_loop: &EventLoop<()>) -> Self {
        Self {
            web_gpu_host: WebGpuHost::new(event_loop),
            table: Table::new(),
            ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            events: Arc::new(HostStateEvents::new()),
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
    fn print(&mut self, s: String) -> wasmtime::Result<()> {
        println!("{s}");
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
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

    let host_state = HostState::new(&event);

    let events_state = Arc::clone(&host_state.events);

    let mut store = Store::new(&engine, host_state);

    let wasm_path = format!("../example-apps/{}/out.wasm", args.example);

    let component =
        Component::from_file(&engine, &wasm_path).context("Component file not found")?;

    let (instance, _) = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    async_std::task::spawn(async move {
        instance.call_start(&mut store).await.unwrap();
    });

    events_state.listen_to_events(event);

    Ok(())
}
