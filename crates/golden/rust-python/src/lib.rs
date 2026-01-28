#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

use anyhow::{anyhow, Result};
use rustpython::vm::{self, PyResult};

/// Run Python code and return the result
///
/// # Arguments
/// * `code` - The Python code to execute
///
/// # Returns
/// The result of the Python code execution as a string
///
/// # Errors
/// Returns an error if the Python code execution fails
pub fn run(code: &str) -> Result<String> {
    // 参考：https://github.com/RustPython/RustPython/blob/main/examples/hello_embed.rs
    let result: PyResult<String> = vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let code_obj = vm
            .compile(code, vm::compiler::Mode::Exec, "<embedded>".to_owned())
            .map_err(|err| vm.new_syntax_error(&err, Some(code)))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok("hello-world".to_string())
    });

    result.map_err(|err| anyhow!("Python execution failed: {:?}", err))
}
