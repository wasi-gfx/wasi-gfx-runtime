#### To run the examples:

In example-apps/*/
```
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/triangle.wasm -o out.wasm
```


In example-runtime/
```
cargo run -- --example triangle
```


#### View wit

In example-apps/*/
```
wasm-tools component wit ./target/wasm32-unknown-unknown/release/webgpu.wasm
```
