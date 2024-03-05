## Example Code

The code in the examples in the repo are not meant to be the best code out there, it's just to get simple points across.

If you think you can improve them, you're more than welcome to open a PR.

Make sure to resolve any warning reported by `cargo check` and `cargo fmt --check`.

Prefer `.unwrap()` over the `?` operator, as this is not production code, and just getting a clear point where things went wrong is preferred over error propagation.


#### Setup env:
```bash
rustup target add wasm32-unknown-unknown
```

```bash
cargo install wasm-tools
```

```bash
cargo install wit-deps-cli
```


#### Install wit dependencies
In `/`
```bash
wit-deps
```


#### List of available examples:
- triangle
- skybox
- rectangle_simple_buffer


#### To run the examples:

In `example-apps/*/`
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/[example].wasm -o out.wasm
```


In `example-runtime/`
```bash
cargo run -- --example [example]
```

Wayland on an Nvidia GPU is [not working well](https://github.com/gfx-rs/wgpu/issues/2519), use XWayland instead:

```bash
export WAYLAND_DISPLAY=wayland-1 vkcube && cargo run -- --example [example]
```


#### View wit

In `example-apps/*/`
```bash
wasm-tools component wit ./target/wasm32-unknown-unknown/release/[example].wasm
```
