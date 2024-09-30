use anyhow::Context;
use std::collections::HashSet;

use pico_args::Arguments;
use xshell::Shell;

lazy_static::lazy_static! {
    static ref SUPPORTED_DEMOS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("skybox");
        s.insert("triangle");
        s.insert("rectangle_simple_buffer");
        s.insert("hello_compute");
        s
    };
}

pub(crate) fn run_demo(shell: Shell, mut args: Arguments) -> anyhow::Result<()> {
    let demo_name: String = args.opt_value_from_str("--name")?.ok_or_else(|| {
        eprintln!(
            "Supported demos:\n- {}",
            SUPPORTED_DEMOS.iter().cloned().collect::<Vec<_>>().join("\n- ")
        );
        anyhow::anyhow!("Demo name is required")
    })?;

    let demo_package =
        SUPPORTED_DEMOS.get(demo_name.as_str()).ok_or_else(|| {
            eprintln!(
                "Unknown demo: {}\nSupported demos: {}",
                demo_name,
                SUPPORTED_DEMOS.iter().cloned().collect::<Vec<_>>().join("\n- ")
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
        "wasm-tools component new ./target/wasm32-unknown-unknown/release/{demo_package}.wasm -o ./target/example-{demo_package}.wasm"
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
