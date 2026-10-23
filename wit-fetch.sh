#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

wkg fetch --config ./wkg-config.toml crates/wasi-webgpu-wasmtime/wit/
wkg fetch --config ./wkg-config.toml crates/frame-buffer-wasmtime/wit/
wkg fetch --config ./wkg-config.toml crates/surface-wasmtime/wit/
wkg fetch --config ./wkg-config.toml examples/wit/
