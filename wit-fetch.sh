#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

wkg wit fetch --config ./wkg-config.toml -d crates/wasi-webgpu-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d crates/frame-buffer-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d crates/surface-wasmtime/wit/
wkg wit fetch --config ./wkg-config.toml -d examples/wit/
# wkg prunes wkg.lock down to the deps of the dir being fetched, so the CTS gets its own lock.
(cd tests/webgpu-spec && wkg wit fetch --config ../../wkg-config.toml -d wit/)
