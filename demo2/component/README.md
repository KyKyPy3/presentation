```
cargo build --target wasm32-unknown-unknown
wasm-tools component new target/wasm32-unknown-unknown/debug/component.wasm -o component.wasm
wasm-tools print --skeleton component.wasm|less
```
