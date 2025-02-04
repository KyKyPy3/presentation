cargo new --bin hello
cargo build --target wasm32-wasip1
wasmtime run <путь к wasm файлу>
