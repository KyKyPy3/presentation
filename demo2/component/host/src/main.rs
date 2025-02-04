use anyhow::Result;
use chrono::Local;
use wasmtime::{
    component::{Component, Linker, Val},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
wasmtime::component::bindgen!("extensibility");

// Step 1: Load components from files
fn load_components(engine: &Engine) -> Result<Vec<Component>> {
    let mut plugins = std::fs::read_dir("./plugins")?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == "wasm") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    plugins.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    println!(
        "[{}] Loading components",
        Local::now().format("%Y-%m-%d %H:%M:%S.%f")
    );

    let components = plugins
        .iter()
        .filter_map(|plugin| {
            println!(
                "[{}] Loading component from: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
                plugin.to_string_lossy()
            );
            match Component::from_file(engine, plugin) {
                Ok(component) => Some(component),
                Err(e) => {
                    println!("Error loading component from {:?}: {:?}", plugin, e);
                    None
                }
            }
        })
        .collect();

    Ok(components)
}

// Step 2: Process string through components
fn process_string(
    engine: &Engine,
    components: &[Component],
    initial_message: String,
) -> Result<String> {
    let mut linker = Linker::<MyState>::new(engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let mut message = initial_message;

    println!(
        "[{}] Processing through components",
        Local::now().format("%Y-%m-%d %H:%M:%S.%f")
    );

    for component in components {
        let mut store = Store::new(
            engine,
            MyState {
                ctx: WasiCtxBuilder::new().build(),
                table: ResourceTable::new(),
            },
        );

        // Instantiate the Component
        let instance = linker.instantiate(&mut store, component)?;

        // Look for the transform function
        let Some(f) = instance.get_func(&mut store, "transform") else {
            println!("Component does not export transform function! Skipping...");
            continue;
        };

        let params = vec![Val::String(message.clone())];
        let mut results = vec![Val::String("".into())];

        // invoke the transform function
        match f.call(store, &params, &mut results) {
            Ok(_) => {
                message = match &results[0] {
                    Val::String(s) => String::from(s),
                    _ => unreachable!(),
                };
            }
            Err(e) => {
                println!("Plugin failed upon invocation: {:?}", e);
                break;
            }
        };
    }

    println!(
        "[{}] Processing finished",
        Local::now().format("%Y-%m-%d %H:%M:%S.%f")
    );

    Ok(message)
}

fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_multi_memory(true);
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;

    // Step 1: Load all components
    let components = load_components(&engine)?;

    let initial_message = "hello world!".to_owned();
    println!(
        "[{}] Initial message: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
        initial_message
    );

    // Step 2: Process string through all components
    let result = process_string(&engine, &components, initial_message)?;

    println!(
        "[{}] Result message: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
        result
    );

    Ok(())
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
