use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use futures::executor::block_on;
use wasi_frame_buffer_wasmtime::WasiFrameBufferView;
use wasi_graphics_context_wasmtime::WasiGraphicsContextView;
use wasi_surface_wasmtime::{Surface, SurfaceDesc, WasiSurfaceView};
use wasi_webgpu_wasmtime::WasiWebGpuView;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use wasmtime_wasi::{IoView, ResourceTable};

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
        "wasi:graphics-context/graphics-context": wasi_graphics_context_wasmtime::wasi::graphics_context::graphics_context,
        "wasi:surface/surface": wasi_surface_wasmtime::wasi::surface::surface,
        "wasi:frame-buffer/frame-buffer": wasi_frame_buffer_wasmtime::wasi::frame_buffer::frame_buffer,
        "wasi:webgpu/webgpu": wasi_webgpu_wasmtime::wasi::webgpu::webgpu,
    },
});

struct HostState {
    pub table: ResourceTable,
    pub instance: Arc<wgpu_core::global::Global>,
    pub main_thread_proxy: wasi_surface_wasmtime::WasiWinitEventLoopProxy,
}

impl HostState {
    fn new(main_thread_proxy: wasi_surface_wasmtime::WasiWinitEventLoopProxy) -> Self {
        Self {
            table: ResourceTable::new(),
            instance: Arc::new(wgpu_core::global::Global::new(
                "webgpu",
                &wgpu_types::InstanceDescriptor {
                    backends: wgpu_types::Backends::all(),
                    flags: wgpu_types::InstanceFlags::from_build_config(),
                    backend_options: Default::default(),
                },
            )),
            main_thread_proxy,
        }
    }
}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiGraphicsContextView for HostState {}
impl WasiFrameBufferView for HostState {}

struct UiThreadSpawner(wasi_surface_wasmtime::WasiWinitEventLoopProxy);

impl wasi_webgpu_wasmtime::MainThreadSpawner for UiThreadSpawner {
    async fn spawn<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        self.0.spawn(f).await
    }
}

impl WasiWebGpuView for HostState {
    fn instance(&self) -> Arc<wgpu_core::global::Global> {
        Arc::clone(&self.instance)
    }

    fn ui_thread_spawner(&self) -> Box<impl wasi_webgpu_wasmtime::MainThreadSpawner + 'static> {
        Box::new(UiThreadSpawner(self.main_thread_proxy.clone()))
    }
}

impl WasiSurfaceView for HostState {
    fn create_canvas(&self, desc: SurfaceDesc) -> Surface {
        block_on(self.main_thread_proxy.create_window(desc))
    }
}

impl ExampleImports for HostState {
    fn print(&mut self, s: String) {
        println!("{s}");
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
    let mut linker: Linker<HostState> = Linker::new(&engine);

    wasi_webgpu_wasmtime::add_to_linker(&mut linker)?;
    wasi_frame_buffer_wasmtime::add_to_linker(&mut linker)?;
    wasi_graphics_context_wasmtime::add_to_linker(&mut linker)?;
    wasi_surface_wasmtime::add_to_linker(&mut linker)?;

    fn type_annotate<F>(val: F) -> F
    where
        F: Fn(&mut HostState) -> &mut dyn ExampleImports,
    {
        val
    }
    let closure = type_annotate::<_>(|t| t);
    Example::add_to_linker_imports_get_host(&mut linker, closure)?;

    let (main_thread_loop, main_thread_proxy) =
        wasi_surface_wasmtime::create_wasi_winit_event_loop();
    let host_state = HostState::new(main_thread_proxy);

    let mut store = Store::new(&engine, host_state);

    let wasm_path = format!("./target/example-{}.wasm", args.example);

    let component =
        Component::from_file(&engine, &wasm_path).context("Component file not found")?;

    let instance = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    tokio::spawn(async move {
        instance.call_start(&mut store).await.unwrap();
    });

    main_thread_loop.run();

    Ok(())
}
