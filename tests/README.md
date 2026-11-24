### Run the WebGPU [CTS](https://github.com/gpuweb/cts) tests

The CTS doesn't run as part of `cargo test`, since it takes a while to run:

```bash
cargo test --release --test cts
```
