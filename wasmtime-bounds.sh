#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

# Checks that the workspace compiles against the oldest and the newest wasmtime release the requirement in Cargo.toml allows.
# Cargo.lock is restored afterwards.

cp Cargo.lock Cargo.lock.backup

low=$(sed -n 's/^wasmtime = "[>=^]*\([0-9]*\).*/\1/p' Cargo.toml)
cargo update wasmtime --precise "$low.0.0"
cargo check
cargo update wasmtime
cargo check

mv Cargo.lock.backup Cargo.lock 
