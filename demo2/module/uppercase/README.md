cargo build --target=wasm32-unknown-unknown
wasm-tools print --skeleton target/wasm32-unknown-unknown/debug/uppercase.wasm|less
