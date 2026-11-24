use colored::Colorize;
use core::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use wasmtime::{
    component::{Component, Linker},
    error::Context,
    Config, Engine,
};

mod expectations;
mod runner;

wasmtime::component::bindgen!({
    path: "wit",
    world: "imports",
    exports: { default: async },
    additional_derives: [serde::Serialize, serde::Deserialize],
    with: {
        "wasi:webgpu": wasi_webgpu_wasmtime::wasi::webgpu,
        "wasi:clocks": wasmtime_wasi::p3::bindings::clocks,
    },
    require_store_data_send: true,
});

const TEST_BINARY: &str = "js-guest/out/tests.wasm";
const RESULTS_PATH: &str = "results/results.json";
const EXPECTATIONS_PATH: &str = "results/expectations.json";

/// The `results.json` model: every case of every spec from one run.
#[derive(Serialize, Deserialize)]
struct SpecResult {
    name: String,
    status: SpecResultStatus,
    duration: Duration,
}

#[derive(Serialize, Deserialize)]
enum SpecResultStatus {
    /// The tests have run successfully.
    Ran(exports::wasi_gfx::webgpu_cts::cts_tests::SpecResult),
    // Tests that we're skipping for now.
    // Includes reason as string.
    Skipping(String),
    /// The runtime panicked while executing the spec.
    /// WARN: This is an unacceptable error to hit. The guest should never be able to make the host panic!
    RuntimeTrapped(ErrorInfo),
    /// The guest trapped.
    GuestTrapped(ErrorInfo),
    /// Didn't finish within `runner::SPEC_TIMEOUT`.
    TimedOut,
}

#[derive(Serialize, Deserialize)]
struct ErrorInfo {
    message: String,
    stack: Option<String>,
}

#[tokio::main]
async fn main() {
    runner::create_js_guest();

    let mut config = Config::default();
    config.wasm_component_model(true);
    config.wasm_component_model_async(true);
    config.epoch_interruption(true);
    let engine = Engine::new(&config).unwrap();
    runner::spawn_epoch_ticker(&engine);
    let mut linker: Linker<runner::WorkloadState> = Linker::new(&engine);

    wasi_webgpu_wasmtime::add_to_linker(&mut linker).unwrap();
    // The CTS polyfills timers with wasi:clocks@0.3.0, while the QuickJS runtime itself still imports wasi 0.2.
    wasmtime_wasi::p3::add_to_linker(&mut linker).unwrap();
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();

    let component = Component::from_file(&engine, TEST_BINARY)
        .context("Test component file not found")
        .unwrap();

    // Like cargo test's built-in harness: `cargo test --test cts -- [filter]... [--skip <skip>]...`
    // Only runs the specs whose name contains any `filter` (all if none) and none of the `skip`s.
    let mut filters = Vec::new();
    let mut skips = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--skip" {
            skips.push(args.next().expect("`--skip` needs a value"));
        } else if arg.starts_with('-') {
            panic!("unknown flag `{arg}`; usage: cargo test --test cts -- [filter]... [--skip <skip>]...");
        } else {
            filters.push(arg);
        }
    }
    let is_selected = |name: &str| {
        (filters.is_empty() || filters.iter().any(|f| name.contains(f.as_str())))
            && !skips.iter().any(|s| name.contains(s.as_str()))
    };

    let previous_records: Vec<SpecResult> = fs::read_to_string(RESULTS_PATH)
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<SpecResult>>(&s).ok())
        .unwrap_or_default();

    let expected: expectations::Expectations = serde_json::from_str(
        &fs::read_to_string(EXPECTATIONS_PATH)
            .context(
                "expectations file not found; create it containing `{}` to start a fresh baseline",
            )
            .unwrap(),
    )
    .unwrap();

    let runner::RunOutcome {
        records,
        stopped_early,
    } = runner::run_all_tests(&engine, &linker, &component, &is_selected).await;

    // Specs that didn't run this time (not selected, or after a run that stopped early) keep their previous results.
    let ran: HashSet<&str> = records.iter().map(|r| r.name.as_str()).collect();
    let kept_records = previous_records
        .iter()
        .filter(|r| !ran.contains(r.name.as_str()));
    fs::write(
        RESULTS_PATH,
        serde_json::to_vec_pretty(&kept_records.chain(&records).collect::<Vec<_>>()).unwrap(),
    )
    .unwrap();

    let blessed = expectations::bless_expectations(expected.clone(), &records);
    if blessed != expected {
        let json = serde_json::to_vec_pretty(&blessed).unwrap();
        fs::write(EXPECTATIONS_PATH, json).unwrap();
    }

    let regressed_from_expectations = expectations::check_against_expectations(&expected, &records);

    print_serve_command();

    if stopped_early {
        panic!("the run stopped early because a timed-out spec couldn't be stopped");
    }
    if regressed_from_expectations {
        panic!("test(s) regressed from {EXPECTATIONS_PATH}; see the lines above. Committing the rewritten file accepts the new baseline.");
    }
}

fn print_serve_command() {
    println!(
        "To view the detailed results of the tests run, serve the tests/webgpu-spec/results dir"
    );
    println!("E.g.");
    println!();
    println!("    {}", "python -m webbrowser 'http://localhost:8000' && python -m http.server -d tests/webgpu-spec/results".bright_cyan().bold());
    println!();
}
