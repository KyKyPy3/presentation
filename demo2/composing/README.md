```
cargo component build --release
wasm-tools compose target/wasm32-wasip1/debug/hello.wasm -d ../cities/target/wasm32-wasip1/debug/cities.wasm -o composed.wasm
```
