## Examples

#### Setup env:
Add the wasm Rust target 
```shell
rustup target add wasm32-unknown-unknown
```

Install wasm-tools
```shell
cargo install wasm-tools
```

Install wit-deps
```shell
cargo install wit-deps-cli
```


#### Install wit dependencies
```shell
wit-deps
```


#### List the available examples:
```shell
cargo xtask run-demo
```


#### To run the examples:

Run an example app
```shell
cargo xtask run-demo --name [example]
```

Wayland on an Nvidia GPU is [not working well](https://github.com/gfx-rs/wgpu/issues/2519), use XWayland instead:

```shell
export WAYLAND_DISPLAY=wayland-1 vkcube && cargo run -- --example [example]
```


#### View wit
```shell
wasm-tools component wit ./target/wasm32-unknown-unknown/release/[example].wasm
```
