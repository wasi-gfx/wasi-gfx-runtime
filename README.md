## Implementation of the [wasi-gfx](https://github.com/WebAssembly/wasi-gfx) Proposal

See [runtime example](/examples/runtime) for an example runtime.

See [app examples](/examples/apps) for a example apps.

### Prerequisites

Install the following:
```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-tools
cargo install wit-deps-cli
```

### Setup

Fetch the WIT dependencies:
```shell
wit-deps
```

### Running examples

List available examples:
```shell
cargo xtask run-demo
```

Run an example:
```shell
cargo xtask run-demo --name <example>
```
