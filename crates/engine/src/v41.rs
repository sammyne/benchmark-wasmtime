use crate::EngineError;
use anyhow::Context;

// Re-export wasmtime v41 types for convenience
pub use wasmtime_v41::*;

use wasmtime_wasi_v41::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
// Re-export wasmtime-wasi v41 as wasi for convenience
// Note: We re-export the module contents to allow easier access
pub use wasmtime_wasi_v41 as wasi;

/// Execute a function from the loaded WASM module
///
/// # Arguments
/// * `engine` - The engine instance
/// * `module` - The loaded WASM module
/// * `function_name` - The name of the function to execute
/// * `_args` - Arguments to pass to the function (currently unused)
///
/// # Returns
/// The result of the function execution
pub fn execute(
    engine: &Engine,
    module: &Module,
    function_name: &str,
    _args: &[u8],
) -> Result<Vec<u8>, EngineError> {
    use wasmtime_v41::{Instance, Store};
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, module, &[]).context("无法实例化模块")?;

    let func = instance
        .get_typed_func::<(), i32>(&mut store, function_name)
        .with_context(|| format!("无法找到 '{}' 函数或签名不匹配", function_name))?;

    let result = func
        .call(&mut store, ())
        .with_context(|| format!("函数 '{}' 调用失败", function_name))?;

    Ok(result.to_le_bytes().to_vec())
}

/// Get the version information of the engine
///
/// # Returns
/// The version string "wasmtime-41.0"
pub fn version() -> &'static str {
    "wasmtime-41.0"
}

#[derive(Default)]
pub struct WasiP2State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for WasiP2State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}
