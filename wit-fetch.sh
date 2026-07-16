#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

wkg wit fetch -d crates/wasi-webgpu-wasmtime/wit/
wkg wit fetch -d crates/frame-buffer-wasmtime/wit/
wkg wit fetch -d crates/surface-wasmtime/wit/
wkg wit fetch -d examples/wit/
