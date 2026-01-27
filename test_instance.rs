use wasmtime_v41::{component::*, Config, Engine};

fn main() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    
    // Create a minimal linker
    let linker = Linker::new(&engine);
    
    // Now let's see what methods are available on linker.instantiate return type
    let _: Linker<()> = linker;
    
    // Check if Instance has the methods we're trying to use
    // This will help us understand the API
}
