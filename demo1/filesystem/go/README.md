tinygo build -o main.wasm -target wasi main.go
wasmtime run --dir . main.wasm
