cargo build --target wasm32-wasip1
wasmtime run target/wasm32-wasi/debug/filesystem.wasm
wasmtime run --dir . target/wasm32-wasi/debug/filesystem.wasm
