use std::sync::Arc;

use __with_name1::HasGpuInstance;
use anyhow::Context;
use clap::Parser;
use wasi_mini_canvas_wasmtime::{HasMainThreadProxy, MainThreadProxy};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use wasmtime_wasi::preview2::{self, ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

#[derive(clap::Parser, Debug)]
struct RuntimeArgs {
    /// The example name
    #[arg(long)]
    example: String,
}

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    async: {
        only_imports: [],
    },
    with: {
        "wasi:webgpu/graphics-context": wasi_graphics_context_wasmtime,
        "wasi:webgpu/mini-canvas": wasi_mini_canvas_wasmtime,
        "wasi:webgpu/frame-buffer": wasi_frame_buffer_wasmtime,
        "wasi:webgpu/webgpu": wasi_webgpu_wasmtime,
    },
});

struct HostState {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub instance: Arc<wgpu_core::global::Global<wgpu_core::identity::IdentityManagerFactory>>,
    pub main_thread_proxy: MainThreadProxy,
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

impl HasMainThreadProxy for HostState {
    fn main_thread_proxy(&self) -> &MainThreadProxy {
        &self.main_thread_proxy
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

    wasi_webgpu_wasmtime::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi_frame_buffer_wasmtime::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi_graphics_context_wasmtime::add_to_linker(&mut linker, |state: &mut HostState| state)?;
    wasi_mini_canvas_wasmtime::add_to_linker(&mut linker, |state: &mut HostState| state)?;

    preview2::bindings::io::poll::add_to_linker(&mut linker, |state| state)?;
    preview2::bindings::io::streams::add_to_linker(&mut linker, |state| state)?;

    Example::add_root_to_linker(&mut linker, |state: &mut HostState| state)?;

    let (main_thread_loop, main_thread_proxy) =
        wasi_mini_canvas_wasmtime::MiniCanvas::create_event_loop();
    let host_state = HostState::new(main_thread_proxy);

    let mut store = Store::new(&engine, host_state);

    let wasm_path = format!("./target/example-{}.wasm", args.example);

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
