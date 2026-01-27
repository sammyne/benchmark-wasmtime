use anyhow::Result;

/// Error type for engine operations
pub type EngineError = anyhow::Error;

/// Core abstraction API for WASM engines
/// This trait defines the common interface that all engine implementations must provide
pub trait EngineApi {
    /// The configuration type associated with this engine
    type Config;

    /// The module type associated with this engine
    type Module;

    /// Create a new engine instance with the given configuration
    ///
    /// # Arguments
    /// * `c` - The configuration for the engine
    ///
    /// # Returns
    /// A new engine instance or an error if creation fails
    fn new(c: &Self::Config) -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Load a WASM module from bytes
    ///
    /// # Arguments
    /// * `wasm_bytes` - The raw WASM module bytes
    ///
    /// # Returns
    /// The loaded WASM module
    fn load_module(&self, wasm_bytes: &[u8]) -> Result<Self::Module, EngineError>;

    /// Execute a function from the loaded WASM module
    ///
    /// # Arguments
    /// * `module` - The loaded WASM module
    /// * `function_name` - The name of the function to execute
    /// * `args` - Arguments to pass to the function
    ///
    /// # Returns
    /// The result of the function execution
    fn execute(
        &self,
        module: &Self::Module,
        function_name: &str,
        args: &[u8],
    ) -> Result<Vec<u8>, EngineError>;

    /// Get the version information of the engine
    fn version(&self) -> &str;
}

pub mod v21;
pub mod v41;

#[cfg(test)]
mod tests;
