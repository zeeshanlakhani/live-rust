# To run w/ wasmtime

- [Install wasmtime](https://wasmtime.dev/)
- `cargo build --release --target wasm32-wasi`
- `wasmtime target/wasm32-wasi/release/fib.wasm`
