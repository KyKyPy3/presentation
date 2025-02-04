cargo new --lib component

wit_bindgen::generate!();

pub struct MyGuest;

impl Guest for MyGuest {
    fn to_upper(input: String) -> String {
        input.to_uppercase()
    }
}

export!(MyGuest);

[lib]
crate-type = ["cdylib"]

package example:component;

world my-world {
  export to-upper: func(input: string) -> string;
}

cargo build --target wasm32-unknown-unknown
wasm-tools component new target/wasm32-unknown-unknown/debug/component.wasm -o component.wasm
wasm-tools print --skeleton component.wasm|less


use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Engine, Store};

fn main() {
    let component_bytes = std::fs::read("component.wasm").unwrap();

    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let component = Component::new(&engine, &component_bytes).unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    let mut result = [Val::String(Default::default())];
    instance
        .get_func(&mut store, "to-upper")
        .unwrap()
        .call(&mut store, &[Val::String("hello".into())], &mut result)
        .unwrap();

    println!("{result:?}");
}

cargo run
