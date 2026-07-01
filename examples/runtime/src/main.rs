use std::sync::Arc;

use clap::Parser;
use wasi_frame_buffer_wasmtime::{FrameBufferCtx, FrameBufferCtxView};
use wasi_surface_wasmtime::{
    winit::WasiWinitEventLoopProxy, SurfaceCtxView, SurfaceFrameBufferCtx,
    SurfaceFrameBufferCtxView, SurfaceWebgpuCtx, SurfaceWebgpuCtxView,
};
use wasi_webgpu_wasmtime::{WasiWebGpuCtx, WasiWebGpuCtxView};
use wasmtime::{
    component::{Component, Linker},
    error::Context,
    Config, Engine, Store,
};

use wasmtime_wasi::ResourceTable;

#[derive(clap::Parser, Debug)]
struct RuntimeArgs {
    /// The example name
    #[arg(long)]
    example: String,
}

wasmtime::component::bindgen!({
    path: "../../wit/",
    world: "example",
    exports: {
        "start": async,
    },
    require_store_data_send: true,
});

struct HostState {
    instance: Arc<wgpu_core::global::Global>,
    main_thread_proxy: Arc<wasi_surface_wasmtime::winit::WasiWinitEventLoopProxy>,
}

impl HostState {
    fn new(main_thread_proxy: wasi_surface_wasmtime::winit::WasiWinitEventLoopProxy) -> Self {
        Self {
            instance: Arc::new(wgpu_core::global::Global::new(
                "webgpu",
                wgpu_types::InstanceDescriptor {
                    backends: wgpu_types::Backends::all(),
                    flags: wgpu_types::InstanceFlags::from_build_config(),
                    backend_options: Default::default(),
                    memory_budget_thresholds: Default::default(),
                    display: None,
                },
                None,
            )),
            main_thread_proxy: Arc::new(main_thread_proxy),
        }
    }
    pub fn add_workload(&self) -> WorkloadState {
        WorkloadState {
            table: ResourceTable::new(),
            instance: Arc::clone(&self.instance),
            main_thread_proxy: Arc::clone(&self.main_thread_proxy),
        }
    }
}

struct WorkloadState {
    table: ResourceTable,
    instance: Arc<wgpu_core::global::Global>,
    main_thread_proxy: Arc<WasiWinitEventLoopProxy>,
}

impl wasmtime::component::HasData for WorkloadState {
    type Data<'a> = &'a mut WorkloadState;
}

impl WasiWebGpuCtxView for WorkloadState {
    fn webgpu_ctx(&mut self) -> WasiWebGpuCtx<'_> {
        WasiWebGpuCtx {
            instance: &self.instance,
            table: &mut self.table,
        }
    }
}

impl FrameBufferCtxView for WorkloadState {
    fn frame_buffer_ctx<'a>(&'a mut self) -> FrameBufferCtx<'a> {
        FrameBufferCtx {
            table: &mut self.table,
        }
    }
}

impl SurfaceCtxView for WorkloadState {
    type Spawner = WasiWinitEventLoopProxy;
    fn surface_ctx(&mut self) -> wasi_surface_wasmtime::SurfaceCtx<'_, WasiWinitEventLoopProxy> {
        wasi_surface_wasmtime::SurfaceCtx {
            table: &mut self.table,
            main_thread_spawner: &self.main_thread_proxy,
        }
    }
}
impl SurfaceWebgpuCtxView for WorkloadState {
    type Spawner = WasiWinitEventLoopProxy;
    fn surface_webgpu_ctx(&mut self) -> SurfaceWebgpuCtx<'_, WasiWinitEventLoopProxy> {
        SurfaceWebgpuCtx {
            table: &mut self.table,
            instance: &self.instance,
            main_thread_spawner: &self.main_thread_proxy,
        }
    }
}
impl SurfaceFrameBufferCtxView for WorkloadState {
    type Spawner = WasiWinitEventLoopProxy;
    fn surface_frame_buffer_ctx(&mut self) -> SurfaceFrameBufferCtx<'_, WasiWinitEventLoopProxy> {
        SurfaceFrameBufferCtx {
            table: &mut self.table,
            instance: &self.instance,
            main_thread_spawner: &self.main_thread_proxy,
        }
    }
}

impl ExampleImports for WorkloadState {
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

    let (main_thread_loop, main_thread_proxy) =
        wasi_surface_wasmtime::winit::create_wasi_winit_event_loop();
    let host_state = HostState::new(main_thread_proxy);

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.wasm_component_model_async(true);
    let engine = Engine::new(&config)?;
    let mut linker: Linker<WorkloadState> = Linker::new(&engine);

    wasi_webgpu_wasmtime::add_to_linker(&mut linker)?;
    wasi_frame_buffer_wasmtime::add_to_linker(&mut linker)?;
    wasi_surface_wasmtime::add_all_to_linker(&mut linker)?;
    Example::add_to_linker_imports::<_, WorkloadState>(&mut linker, |x| x)?;

    let workload_state = host_state.add_workload();

    let mut store = Store::new(&engine, workload_state);

    let wasm_path = format!("./target/example-{}.wasm", args.example);

    let component =
        Component::from_file(&engine, &wasm_path).context("Component file not found")?;

    let instance = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();

    tokio::spawn(async move {
        instance.func_start().call_async(store, ()).await.unwrap();
    });

    main_thread_loop.run();

    Ok(())
}
