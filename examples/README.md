## Examples

#### Setup env:
Add the wasm Rust target 
```bash
rustup target add wasm32-unknown-unknown
```

Install wasm-tools
```bash
cargo install wasm-tools
```

Install wit-deps
```bash
cargo install wit-deps-cli
```


#### Install wit dependencies
```bash
wit-deps
```


#### List of available examples:
- triangle
- skybox
- rectangle_simple_buffer


#### To run the examples:

Build an example app
```bash
cargo build --package [example] --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/[example].wasm -o ./target/example-[example].wasm
```


Run with the runtime
```bash
cargo run -- --example [example]
```

Wayland on an Nvidia GPU is [not working well](https://github.com/gfx-rs/wgpu/issues/2519), use XWayland instead:

```bash
export WAYLAND_DISPLAY=wayland-1 vkcube && cargo run -- --example [example]
```


#### View wit
```bash
wasm-tools component wit ./target/wasm32-unknown-unknown/release/[example].wasm
```
