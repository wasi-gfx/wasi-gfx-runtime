#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

wkg wit fetch --config ./wkg-config.toml -d crates/wasi-webgpu-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d crates/frame-buffer-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d crates/surface-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d examples/wit/
