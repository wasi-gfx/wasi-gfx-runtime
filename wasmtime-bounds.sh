#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

# Checks that the workspace compiles against the oldest and the newest wasmtime release the requirement in Cargo.toml allows.
# Cargo.toml and Cargo.lock are restored afterwards, even if a check fails.

cp Cargo.toml Cargo.toml.backup
cp Cargo.lock Cargo.lock.backup
trap 'mv Cargo.toml.backup Cargo.toml; mv Cargo.lock.backup Cargo.lock' EXIT

check() {
    cargo check
    # The CTS host is a `test = false` target, which a plain `cargo check` skips.
    cargo check -p cts --test cts
}

# wasmtime-wasi requires the exactly matching wasmtime, and `cargo update --precise` can only pin one package.
# So pin both requirements to the low end and let cargo resolve.
low=$(sed -n 's/^wasmtime = "[>=^]*\([0-9]*\).*/\1/p' Cargo.toml)
sed -i.sedbak -E "s/^(wasmtime|wasmtime-wasi) = .*/\1 = \"=$low.0.0\"/" Cargo.toml
rm Cargo.toml.sedbak
check

cp Cargo.toml.backup Cargo.toml
cp Cargo.lock.backup Cargo.lock
cargo update wasmtime wasmtime-wasi
check
