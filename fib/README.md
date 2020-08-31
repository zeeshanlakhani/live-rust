# To run w/ wasmtime

- [Install wasmtime](https://wasmtime.dev/)
- `rustup target add wasm32-wasi`
- `cargo build --release --target wasm32-wasi`
- `wasmtime target/wasm32-wasi/release/fib.wasm`
