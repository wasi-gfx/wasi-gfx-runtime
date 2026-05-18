use colored::Colorize;
use core::time::Duration;
use futures::executor::block_on;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    sync::{Arc, Mutex},
};
use wasi_frame_buffer_wasmtime::WasiFrameBufferView;
use wasi_graphics_context_wasmtime::WasiGraphicsContextView;
use wasi_surface_wasmtime::{Surface, SurfaceDesc, WasiSurfaceView};
use wasi_webgpu_wasmtime::WasiWebGpuView;
use wasmtime::{
    component::{Component, Linker},
    error::Context,
    Config, Engine, Store,
};

use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_io::IoView;

wasmtime::component::bindgen!({
    path: "wit",
    world: "imports",
    exports: { default: async },
    additional_derives: [serde::Serialize],
    with: {
        "wasi:io": wasmtime_wasi_io::bindings::wasi::io,
        "wasi:webgpu": wasi_webgpu_wasmtime::wasi::webgpu,
        "wasi:graphics-context": wasi_graphics_context_wasmtime::wasi::graphics_context,
        "wasi:surface": wasi_surface_wasmtime::wasi::surface,
        "wasi:frame-buffer": wasi_frame_buffer_wasmtime::wasi::frame_buffer,
    },
});

use exports::wasi_gfx::webgpu_cts::cts_tests;

const TEST_BINARY: &str = "js-guest/out/tests.wasm";
const HISTORICAL_RESULTS: &str = "results/historical.json";
const DETAILED_RESULTS: &str = "results/detailed.json";

#[derive(Serialize)]
struct SpecResult {
    name: String,
    status: SpecResultStatus,
    logs: Vec<LogItem>,
    duration: Duration,
}

type LogItem = cts_tests::CaseLog;

#[derive(Serialize)]
enum SpecResultStatus {
    /// All the tests in the spec succeeded.
    Pass,
    /// At lease one case in the spec failed.
    Fail,
    /// At least case item in the spec was skipped.
    /// Likely because the adapter didn't support the optional feature being tested.
    Skip,
    /// The runtime panicked while executing the spec.
    /// WARN: This is an unacceptable error to hit. The guest should never be able to make the host panic!
    RuntimeTrap,
    /// The guest trapped.
    GuestTrap,
}

#[derive(Serialize, Deserialize, Default)]
struct HighLevelResults {
    passing: Vec<String>,
    skipped: Vec<String>,
    failing: Vec<String>,
}

struct HostState {
    pub table: ResourceTable,
    pub ctx: WasiCtx,
    pub http_ctx: wasmtime_wasi_http::WasiHttpCtx,
    pub instance: Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global>,
    pub main_thread_proxy: wasi_surface_wasmtime::WasiWinitEventLoopProxy,
}

impl HostState {
    fn new(
        instance: Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global>,
        main_thread_proxy: wasi_surface_wasmtime::WasiWinitEventLoopProxy,
    ) -> Self {
        Self {
            table: ResourceTable::new(),
            // review: not implementing stdio so that we swallow guest logs
            ctx: WasiCtxBuilder::new()
                // .inherit_stdio()
                .build(),
            http_ctx: wasmtime_wasi_http::WasiHttpCtx::new(),
            instance,
            main_thread_proxy,
        }
    }
}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        wasmtime_wasi::WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl wasmtime_wasi_http::p2::WasiHttpView for HostState {
    fn http(&mut self) -> wasmtime_wasi_http::p2::WasiHttpCtxView<'_> {
        wasmtime_wasi_http::p2::WasiHttpCtxView {
            ctx: &mut self.http_ctx,
            table: &mut self.table,
            hooks: Default::default(),
        }
    }
}

impl WasiGraphicsContextView for HostState {}
impl WasiFrameBufferView for HostState {}

struct UiThreadSpawner(wasi_surface_wasmtime::WasiWinitEventLoopProxy);

impl wasi_webgpu_wasmtime::MainThreadSpawner for UiThreadSpawner {
    async fn spawn<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.0.spawn(f).await
    }
}

impl WasiWebGpuView for HostState {
    fn instance(&self) -> Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global> {
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

fn main() {
    create_js_guest();

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let mut linker: Linker<HostState> = Linker::new(&engine);

    wasi_webgpu_wasmtime::add_to_linker(&mut linker).unwrap();
    wasi_frame_buffer_wasmtime::add_to_linker(&mut linker).unwrap();
    wasi_graphics_context_wasmtime::add_to_linker(&mut linker).unwrap();
    wasi_surface_wasmtime::add_only_surface_to_linker(&mut linker).unwrap();
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).unwrap();
    wasmtime_wasi_http::p2::add_only_http_to_linker_sync(&mut linker).unwrap();

    // not doing main_thread_loop.run() for now since we're not creating real windows
    let (_main_thread_loop, main_thread_proxy) =
        wasi_surface_wasmtime::create_wasi_winit_event_loop();

    let component = Component::from_file(&engine, TEST_BINARY)
        .context("Test component file not found")
        .unwrap();

    // std::thread::spawn(move || {
    let records = block_on(run_all_tests(
        &engine,
        &linker,
        &component,
        main_thread_proxy,
    ));
    std::fs::write(
        DETAILED_RESULTS,
        serde_json::to_vec_pretty(&records).unwrap(),
    )
    .unwrap();

    check_results_against_historical(&records);

    print_serve_command();

    //     std::process::exit(1);
    // });

    // main_thread_loop.run();
}

fn new_wgpu_instance() -> Arc<wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global> {
    Arc::new(
        wasi_webgpu_wasmtime::reexports::wgpu_core::global::Global::new(
            "webgpu",
            wasi_webgpu_wasmtime::reexports::wgpu_types::InstanceDescriptor {
                backends: wasi_webgpu_wasmtime::reexports::wgpu_types::Backends::all(),
                flags:
                    wasi_webgpu_wasmtime::reexports::wgpu_types::InstanceFlags::from_build_config(),
                backend_options:
                    wasi_webgpu_wasmtime::reexports::wgpu_types::BackendOptions::default(),
                memory_budget_thresholds:
                    wasi_webgpu_wasmtime::reexports::wgpu_types::MemoryBudgetThresholds {
                        for_resource_creation: Some(90),
                        for_device_loss: Some(90),
                    },
                display: None,
            },
            None,
        ),
    )
}

async fn run_all_tests(
    engine: &Engine,
    linker: &Linker<HostState>,
    component: &Component,
    main_thread_proxy: wasi_surface_wasmtime::WasiWinitEventLoopProxy,
) -> Vec<SpecResult> {
    let tests: Vec<String> = {
        let mut store = Store::new(
            engine,
            HostState::new(new_wgpu_instance(), main_thread_proxy.clone()),
        );
        let imports = Imports::instantiate_async(&mut store, component, linker)
            .await
            .unwrap();
        imports
            .wasi_gfx_webgpu_cts_cts_tests()
            .call_list_specs(&mut store)
            .await
            .unwrap()
    };

    let total_tests = tests.len();

    let mut records: Vec<SpecResult> = Vec::with_capacity(total_tests);

    println!("Running the {total_tests} tests");

    let bar = ProgressBar::new(total_tests as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{pos}/{len} {pct}%] [{bar:40.cyan/blue}] {wide_msg}",
        )
        .unwrap()
        .with_key("pct", |s: &ProgressState, w: &mut dyn std::fmt::Write| {
            let pct = if s.len().unwrap_or(0) > 0 {
                100.0 * s.pos() as f64 / s.len().unwrap() as f64
            } else {
                0.0
            };
            write!(w, "{pct:>5.2}").unwrap();
        })
        .progress_chars("##-"),
    );
    bar.enable_steady_tick(std::time::Duration::from_secs(1));

    let original_hook = std::panic::take_hook();
    let last_panic_message = Arc::new(Mutex::new(None));
    let last_panic_message_clone = Arc::clone(&last_panic_message);
    std::panic::set_hook(Box::new(move |info| {
        *last_panic_message_clone.lock().unwrap() = Some(LogItem {
            message: info.payload_as_str().unwrap_or_default().to_owned(),
            stack: info.location().map(|l| l.to_string()),
        });
    }));

    // TODO: find a way to remove these
    let skip_list: Vec<&str> = vec![
        // super slow
        "shader_execution_expression_binary_f16_matrix_matrix_multiplication",
        "shader_execution_expression_binary_f32_matrix_vector_multiplication",
        "shader_execution_expression_binary_f32_matrix_matrix_multiplication",
        "shader_execution_expression_call_builtin_mix",
        "shader_execution_expression_call_builtin_fwidth",
        "shader_execution_expression_call_builtin_fwidthCoarse",
        "shader_execution_expression_call_builtin_faceForward",
        "shader_execution_expression_call_builtin_fwidthFine",
        "shader_execution_expression_call_builtin_refract",
        "shader_execution_expression_call_builtin_distance",
        // segfaults
        "shader_execution_limits",
    ];

    for spec_name in tests.into_iter() {
        bar.set_message(spec_name.clone());

        if skip_list.iter().any(|s| s == &spec_name) {
            bar.inc(1);
            continue;
        }

        let mut store = Store::new(
            engine,
            HostState::new(new_wgpu_instance(), main_thread_proxy.clone()),
        );
        let test_start_time = std::time::Instant::now();

        let guest = Imports::instantiate_async(&mut store, component, linker)
            .await
            .unwrap();
        let spec_name_cloned = spec_name.clone();

        *last_panic_message.lock().unwrap() = None;
        let join_result = std::thread::spawn(move || {
            block_on(async {
                guest
                    .wasi_gfx_webgpu_cts_cts_tests()
                    .call_run_spec_tests(&mut store, &spec_name_cloned)
                    .await
            })
        })
        .join();

        let duration = test_start_time.elapsed();

        let wasmtime_result: Result<_, wasmtime::Error> = match join_result {
            Ok(res) => res,
            Err(_) => {
                let log_payload = match last_panic_message.lock().unwrap().take() {
                    Some(test_log) => test_log,
                    None => LogItem {
                        message: String::from("Panic message not saved"),
                        stack: None,
                    },
                };
                records.push(SpecResult {
                    name: spec_name.clone(),
                    status: SpecResultStatus::RuntimeTrap,
                    logs: vec![log_payload],
                    duration,
                });
                bar.inc(1);
                continue;
            }
        };

        let guest_result: Result<_, String> = match wasmtime_result {
            Ok(res) => res,
            Err(err) => {
                records.push(SpecResult {
                    name: spec_name.clone(),
                    status: SpecResultStatus::GuestTrap,
                    logs: vec![LogItem {
                        message: err.to_string(),
                        stack: None,
                    }],
                    duration,
                });
                bar.inc(1);
                continue;
            }
        };

        let spec_result_tests = match guest_result {
            Ok(res) => res,
            Err(err) => {
                records.push(SpecResult {
                    name: spec_name.clone(),
                    status: SpecResultStatus::GuestTrap,
                    logs: vec![LogItem {
                        message: err.to_string(),
                        stack: None,
                    }],
                    duration,
                });
                bar.inc(1);
                continue;
            }
        };

        let mut status = SpecResultStatus::Pass;
        for test in &spec_result_tests {
            for case in &test.cases {
                match case.status {
                    cts_tests::CaseStatus::Fail => {
                        status = SpecResultStatus::Fail;
                        break;
                    }
                    cts_tests::CaseStatus::Skip => {
                        status = SpecResultStatus::Skip;
                    }
                    cts_tests::CaseStatus::Pass => {}
                };
            }
        }

        let logs = spec_result_tests
            .into_iter()
            .map(|test| test.cases)
            .flatten()
            .map(|case| case.logs)
            .flatten()
            .collect();

        records.push(SpecResult {
            name: spec_name,
            status,
            logs,
            duration,
        });

        bar.inc(1);
    }

    std::panic::set_hook(original_hook);

    // clear current test message
    bar.set_message(String::new());
    bar.finish();

    records
}

fn check_results_against_historical(records: &Vec<SpecResult>) {
    let new_high_level_results =
        records
            .iter()
            .fold(HighLevelResults::default(), |mut acc, record| {
                match record.status {
                    SpecResultStatus::Pass => acc.passing.push(record.name.clone()),
                    SpecResultStatus::Skip => acc.skipped.push(record.name.clone()),
                    SpecResultStatus::Fail
                    | SpecResultStatus::RuntimeTrap
                    | SpecResultStatus::GuestTrap => acc.failing.push(record.name.clone()),
                }
                acc
            });

    let historical = fs::read_to_string(HISTORICAL_RESULTS).unwrap();
    let historical: HighLevelResults = serde_json::from_str(&historical).unwrap();

    let historical_passing_hash: HashSet<String> = HashSet::from_iter(historical.passing);
    let newly_passing: Vec<&String> = new_high_level_results
        .passing
        .iter()
        .filter(|r| !historical_passing_hash.contains(r.as_str()))
        .collect();
    let historical_skipped_hash: HashSet<String> = HashSet::from_iter(historical.skipped);
    let newly_skipped: Vec<&String> = new_high_level_results
        .skipped
        .iter()
        .filter(|r| !historical_skipped_hash.contains(r.as_str()))
        .collect();
    let historical_failing_hash: HashSet<String> = HashSet::from_iter(historical.failing);
    let newly_failing: Vec<&String> = new_high_level_results
        .failing
        .iter()
        .filter(|r| !historical_failing_hash.contains(r.as_str()))
        .collect();

    if newly_passing.is_empty() && newly_skipped.is_empty() && newly_failing.is_empty() {
        println!("{}", "No changes since last run.".dimmed());
    } else {
        for t in &newly_passing {
            println!("🎉 {} {}", "[NEW PASS]".green().bold(), t);
        }
        for t in &newly_skipped {
            println!("{} {}", "[NEW SKIP]".yellow().bold(), t);
        }
        for t in &newly_failing {
            println!("{} {}", "[NEW FAIL]".red().bold(), t);
        }
    }

    if !newly_failing.is_empty() || !newly_skipped.is_empty() {
        panic!(
            "Regression: {} newly failing, {} newly skipped",
            newly_failing.len(),
            newly_skipped.len()
        );
    }

    fs::write(
        HISTORICAL_RESULTS,
        serde_json::to_string_pretty(&new_high_level_results).unwrap(),
    )
    .unwrap();
}

fn create_js_guest() {
    let status = std::process::Command::new("npm")
        .arg("install")
        .current_dir("js-guest")
        .status()
        .context("Failed to spawn `npm install`. Is npm installed and on PATH?")
        .unwrap();
    if !status.success() {
        panic!("`npm install` failed with status: {status}");
    }

    let status = std::process::Command::new("npm")
        .args(["run", "create-guest"])
        .current_dir("js-guest")
        .status()
        .context("Failed to spawn `npm run create-guest`. Is npm installed and on PATH?")
        .unwrap();
    if !status.success() {
        panic!("`npm run create-guest` failed with status: {status}");
    }
}

fn print_serve_command() {
    println!(
        "To view the detailed results of the tests run, serve the tests/webgpu-spec/results dir"
    );
    println!("E.g.");
    println!("");
    println!("    {}", "python -m webbrowser 'http://localhost:8000' && python -m http.server -d tests/webgpu-spec/results".bright_cyan().bold());
    println!("");
}
