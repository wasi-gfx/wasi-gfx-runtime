//! Building the JS guest and running specs against it

use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasi_webgpu_wasmtime::{
    reexports::{wgpu_core, wgpu_types},
    spawn_poll_devices_thread, MpscPollNotifier, WasiWebGpuCtx, WasiWebGpuCtxView,
    WasiWebGpuOptions,
};
use wasmtime::{
    component::{Component, Linker, ResourceTable},
    error::Context,
    Engine, Store,
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::exports::wasi_gfx::webgpu_cts::cts_tests::{
    CaseResultStatus, SpecResult as RunSpecResult,
};
use crate::Imports;
use crate::{ErrorInfo, SpecResult, SpecResultStatus};

pub(crate) struct WorkloadState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    instance: Arc<wgpu_core::global::Global>,
    poll_notifier: MpscPollNotifier,
    webgpu_options: WasiWebGpuOptions,
}

impl WorkloadState {
    /// Every spec gets a fresh wgpu instance so that state can't leak between specs.
    pub(crate) fn new() -> Self {
        let instance = new_wgpu_instance();
        // The poll thread exits once the store (and with it the notifier) is dropped.
        let (poll_notifier, _poll_thread) =
            spawn_poll_devices_thread(Arc::clone(&instance), Default::default())
                .expect("failed to spawn the poll thread");
        Self {
            table: ResourceTable::new(),
            wasi_ctx: WasiCtxBuilder::new().build(),
            instance,
            poll_notifier,
            webgpu_options: WasiWebGpuOptions::default(),
        }
    }
}

impl WasiView for WorkloadState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.table,
        }
    }
}

impl WasiWebGpuCtxView for WorkloadState {
    fn webgpu_ctx(&mut self) -> WasiWebGpuCtx<'_> {
        WasiWebGpuCtx {
            instance: &self.instance,
            poll_notifier: &self.poll_notifier,
            table: &mut self.table,
            options: &self.webgpu_options,
        }
    }
}

fn new_wgpu_instance() -> Arc<wgpu_core::global::Global> {
    Arc::new(wgpu_core::global::Global::new(
        "webgpu",
        wgpu_types::InstanceDescriptor {
            backends: wgpu_types::Backends::all(),
            flags: wgpu_types::InstanceFlags::from_build_config(),
            backend_options: wgpu_types::BackendOptions::default(),
            memory_budget_thresholds: wgpu_types::MemoryBudgetThresholds {
                for_resource_creation: Some(90),
                for_device_loss: Some(90),
            },
            display: None,
        },
        None,
    ))
}

pub(crate) const SPEC_TIMEOUT: Duration = Duration::from_mins(5);
const EPOCH_TICK: Duration = Duration::from_millis(100);
/// Long enough for a host call that's still running when the timeout hits (e.g. lavapipe compiling a large pipeline)
/// to finish, since the abort only lands once the guest is back in wasm.
const ABORT_GRACE: Duration = Duration::from_secs(60);

/// Advances the engine's epoch every `EPOCH_TICK`. Each store yields to tokio on every tick,
/// so a spec stuck in a busy loop still reaches an await point where its task can be aborted.
pub(crate) fn spawn_epoch_ticker(engine: &Engine) {
    let engine = engine.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(EPOCH_TICK);
        engine.increment_epoch();
    });
}

fn new_store(engine: &Engine) -> Store<WorkloadState> {
    let mut store = Store::new(engine, WorkloadState::new());
    store.epoch_deadline_async_yield_and_update(1);
    store
}

pub(crate) struct RunOutcome {
    pub(crate) records: Vec<SpecResult>,
    /// A timed-out spec couldn't be stopped, so the specs after it weren't run.
    pub(crate) stopped_early: bool,
}

pub(crate) async fn run_all_tests(
    engine: &Engine,
    linker: &Linker<WorkloadState>,
    component: &Component,
    is_selected: &impl Fn(&str) -> bool,
) -> RunOutcome {
    let mut tests: Vec<String> = {
        let mut store = new_store(engine);
        let imports = Imports::instantiate_async(&mut store, component, linker)
            .await
            .unwrap();
        imports
            .wasi_gfx_webgpu_cts_cts_tests()
            .call_list_specs(&mut store)
            .await
            .unwrap()
    };
    tests.retain(|name| is_selected(name));

    let total_tests = tests.len();

    let mut records: Vec<SpecResult> = Vec::with_capacity(total_tests);

    println!("Running the {total_tests} tests");
    let run_start_time = std::time::Instant::now();

    let original_hook = std::panic::take_hook();
    let last_panic_info = Arc::new(Mutex::new(None));
    let last_panic_message_clone = Arc::clone(&last_panic_info);
    std::panic::set_hook(Box::new(move |info| {
        *last_panic_message_clone.lock().unwrap() = Some(ErrorInfo {
            message: info.payload_as_str().unwrap_or_default().to_owned(),
            stack: info.location().map(|l| l.to_string()),
        });
    }));

    let mut stopped_early = false;
    for (index, spec_name) in tests.into_iter().enumerate() {
        print!("[{}/{total_tests}] {spec_name} ... ", index + 1);
        std::io::stdout().flush().unwrap();

        let mut store = new_store(engine);
        let component = component.clone();
        let linker = linker.clone();
        let test_start_time = std::time::Instant::now();
        let spec_name_cloned = spec_name.clone();

        *last_panic_info.lock().unwrap() = None;
        // Run on its own task so that a host panic is caught instead of taking down the whole run.
        let mut task = tokio::spawn(async move {
            let guest = Imports::instantiate_async(&mut store, &component, &linker).await?;
            store
                .run_concurrent(async move |accessor| {
                    guest
                        .wasi_gfx_webgpu_cts_cts_tests()
                        .call_run_spec_tests(accessor, spec_name_cloned)
                        .await
                })
                .await
                .and_then(|result| result)
        });
        let join_result = tokio::time::timeout(SPEC_TIMEOUT, &mut task).await;

        // Unwraps the three levels where an error can occur:
        //   1. thread panic (host runtime trap)
        //   2. wasmtime::Error (guest trap)
        //   3. Err(String) returned by the guest
        fn compute_status(
            join_result: Result<
                Result<Result<RunSpecResult, String>, wasmtime::Error>,
                tokio::task::JoinError,
            >,
            last_panic_info: &Mutex<Option<ErrorInfo>>,
        ) -> SpecResultStatus {
            let wasmtime_result = match join_result {
                Ok(res) => res,
                Err(_) => {
                    let error_info =
                        last_panic_info
                            .lock()
                            .unwrap()
                            .take()
                            .unwrap_or_else(|| ErrorInfo {
                                message: String::from("Panic message not saved"),
                                stack: None,
                            });
                    return SpecResultStatus::RuntimeTrapped(error_info);
                }
            };

            let guest_result = match wasmtime_result {
                Ok(res) => res,
                Err(err) => {
                    return SpecResultStatus::GuestTrapped(ErrorInfo {
                        message: err.to_string(),
                        stack: None,
                    })
                }
            };

            match guest_result {
                Ok(ran) => SpecResultStatus::Ran(ran),
                Err(err) => SpecResultStatus::GuestTrapped(ErrorInfo {
                    message: err.to_string(),
                    stack: None,
                }),
            }
        }

        let mut still_running = false;
        let status = match join_result {
            Ok(join_result) => compute_status(join_result, &last_panic_info),
            Err(_) => {
                task.abort();
                // The abort only lands at the task's next await, so wait for it to actually end before the next spec starts.
                still_running = tokio::time::timeout(ABORT_GRACE, &mut task).await.is_err();
                SpecResultStatus::TimedOut
            }
        };
        let duration = test_start_time.elapsed();
        println!(
            "{} ({:.1}s)",
            describe_status(&status),
            duration.as_secs_f64()
        );
        records.push(SpecResult {
            name: spec_name.clone(),
            status,
            duration,
        });

        if still_running {
            eprintln!(
                "error: {spec_name} still hadn't stopped {}s after being aborted. \
                 Stopping the run, since it would interfere with the specs after it.",
                ABORT_GRACE.as_secs()
            );
            stopped_early = true;
            break;
        }
    }

    std::panic::set_hook(original_hook);

    println!(
        "Ran {} of {total_tests} tests in {:.0}s",
        records.len(),
        run_start_time.elapsed().as_secs_f64()
    );

    RunOutcome {
        records,
        stopped_early,
    }
}

/// One spec's outcome, e.g. `56 passed, 91 failed`.
fn describe_status(status: &SpecResultStatus) -> String {
    match status {
        SpecResultStatus::Ran(run) => {
            let (mut passed, mut warned, mut failed, mut skipped) = (0, 0, 0, 0);
            for case in run.tests.iter().flat_map(|test| &test.cases) {
                match case.status {
                    CaseResultStatus::Passed => passed += 1,
                    CaseResultStatus::Warned => warned += 1,
                    CaseResultStatus::Failed => failed += 1,
                    CaseResultStatus::Skipped => skipped += 1,
                }
            }
            let parts: Vec<String> = [
                (passed, "passed"),
                (warned, "warned"),
                (failed, "failed"),
                (skipped, "skipped"),
            ]
            .into_iter()
            .filter(|&(n, _)| n > 0)
            .map(|(n, label)| format!("{n} {label}"))
            .collect();
            if parts.is_empty() {
                "no cases".to_owned()
            } else {
                parts.join(", ")
            }
        }
        SpecResultStatus::Skipping(_) => "skipped".to_owned(),
        SpecResultStatus::GuestTrapped(_) => "guest trapped".to_owned(),
        SpecResultStatus::RuntimeTrapped(_) => "RUNTIME TRAPPED".to_owned(),
        SpecResultStatus::TimedOut => "TIMED OUT".to_owned(),
    }
}

pub(crate) fn create_js_guest() {
    let status = std::process::Command::new("npm")
        .arg("ci")
        .current_dir("js-guest")
        .status()
        .context("Failed to spawn `npm ci`. Is npm installed and on PATH?")
        .unwrap();
    if !status.success() {
        panic!("`npm ci` failed with status: {status}");
    }

    let status = std::process::Command::new("npm")
        .arg("ci")
        .current_dir("js-guest/cts")
        .status()
        .context("Failed to spawn `npm ci` in cts/. Is npm installed and on PATH?")
        .unwrap();
    if !status.success() {
        panic!("`npm ci` in cts/ failed with status: {status}");
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
