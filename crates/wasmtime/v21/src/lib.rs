use anyhow::{Context, Result};
use engine::{EngineApi, EngineError};
use wasmtime::{Engine, Instance, Module, Store};

/// Wasmtime v21.0 engine implementation
pub struct WasmtimeV21 {
    engine: Engine,
}

impl Default for WasmtimeV21 {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmtimeV21 {
    /// Create a new Wasmtime v21.0 engine
    pub fn new() -> Self {
        let engine = Engine::default();
        WasmtimeV21 { engine }
    }
}

impl EngineApi for WasmtimeV21 {
    type ModuleType = Module;

    fn load_module(&self, wasm_bytes: &[u8]) -> Result<Self::ModuleType, EngineError> {
        Module::from_binary(&self.engine, wasm_bytes).context("无法解析 WebAssembly 模块")
    }

    fn execute(&self, module: &Self::ModuleType, function_name: &str, _args: &[u8]) -> Result<Vec<u8>, EngineError> {
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, module, &[]).context("无法实例化模块")?;

        let func = instance
            .get_typed_func::<(), i32>(&mut store, function_name)
            .with_context(|| format!("无法找到 '{}' 函数或签名不匹配", function_name))?;

        let result = func
            .call(&mut store, ())
            .with_context(|| format!("函数 '{}' 调用失败", function_name))?;

        Ok(result.to_le_bytes().to_vec())
    }

    fn version(&self) -> &str {
        "wasmtime-21.0"
    }
}

#[cfg(test)]
mod tests;
