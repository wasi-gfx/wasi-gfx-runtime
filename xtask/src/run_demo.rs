use anyhow::Context;
use std::collections::HashMap;

use pico_args::Arguments;
use xshell::Shell;

lazy_static::lazy_static! {
    static ref SUPPORTED_DEMOS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        m.insert("skybox", ("skybox", "./target/example-skybox.wasm"));
        m.insert("triangle", ("triangle", "./target/example-triangle.wasm"));
        m.insert("fb-rectangle", ("rectangle_simple_buffer", "./target/example-rectangle_simple_buffer.wasm"));
        m
    };
}

pub(crate) fn run_demo(shell: Shell, mut args: Arguments) -> anyhow::Result<()> {
    let demo_name: String = args.opt_value_from_str("--name")?.ok_or_else(|| {
        eprintln!(
            "Supported demos: {:#?}",
            SUPPORTED_DEMOS.keys().cloned().collect::<Vec<_>>()
        );
        anyhow::anyhow!("Demo name is required")
    })?;

    let (demo_package, demo_binary_path) =
        SUPPORTED_DEMOS.get(demo_name.as_str()).ok_or_else(|| {
            eprintln!(
                "Unknown demo: {:#?}\nSupported demos: {:#?}",
                demo_name,
                SUPPORTED_DEMOS.keys().cloned().collect::<Vec<_>>()
            );
            anyhow::anyhow!("Unsupported demo name")
        })?;

    args.finish();

    xshell::cmd!(
        shell,
        "cargo build --package {demo_package} --release --target wasm32-unknown-unknown"
    )
    .quiet()
    .run()
    .context(format!("Failed to build wasm module for {}", demo_name))?;

    xshell::cmd!(
        shell,
        "wasm-tools component new ./target/wasm32-unknown-unknown/release/{demo_package}.wasm -o {demo_binary_path}"
    )
    .quiet()
    .run()
    .context(format!("Failed to build wasm component module for {}", demo_name))?;

    xshell::cmd!(shell, "cargo run -p runtime -- --example {demo_package}")
        .quiet()
        .run()
        .context(format!("Failed to run in runtime for {}", demo_name))?;

    Ok(())
}
