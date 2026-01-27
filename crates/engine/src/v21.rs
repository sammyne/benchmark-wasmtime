use crate::{EngineApi, EngineError};
use anyhow::{Context, Result};

// Re-export wasmtime v21 types for convenience
pub use wasmtime_v21::{Config, Engine, Module};

// Implement EngineApi for wasmtime v21 Engine
impl EngineApi for Engine {
    type Config = Config;
    type Module = Module;

    fn new(config: &Self::Config) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Engine::new(config).context("Failed to create Wasmtime v21.0 engine")
    }

    fn load_module(&self, wasm_bytes: &[u8]) -> Result<Self::Module, EngineError> {
        Module::from_binary(self, wasm_bytes).context("无法解析 WebAssembly 模块")
    }

    fn execute(
        &self,
        module: &Self::Module,
        function_name: &str,
        _args: &[u8],
    ) -> Result<Vec<u8>, EngineError> {
        use wasmtime_v21::{Instance, Store};
        let mut store = Store::new(self, ());
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
