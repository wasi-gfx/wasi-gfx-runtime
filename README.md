#### To run the examples:

In `example-apps/*/`
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/triangle.wasm -o out.wasm
```


In `example-runtime/`
```bash
cargo run -- --example triangle
```

Wayland on an Nvidia GPU is [not working well](https://github.com/gfx-rs/wgpu/issues/2519), use XWayland instead:

```bash
export GRUB_CMDLINE_LINUX="nvidia-drm.modeset=1" && cargo run -- --example triangle
```


#### View wit

In `example-apps/*/`
```bash
wasm-tools component wit ./target/wasm32-unknown-unknown/release/webgpu.wasm
```
