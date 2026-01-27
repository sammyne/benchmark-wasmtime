use anyhow::{Context, Result};
use engine::EngineApi;
use testdata::{load_fixture, validate_wasm};

use engine::v21::{Config, Engine as WasmtimeV21};
use engine::v41::{Config as ConfigV41, Engine as WasmtimeV41};

#[test]
fn test_wasmtime_v21_execute_simple() -> Result<()> {
    let wasm_bytes = load_fixture("simple");
    validate_wasm(&wasm_bytes)?;

    let config = Config::new();
    let engine = WasmtimeV21::new(&config).context("创建 v21 引擎失败")?;
    let module = engine
        .load_module(&wasm_bytes)
        .context("无法加载 WASM 模块")?;

    let result = engine
        .execute(&module, "add", &[])
        .context("执行函数失败")?;

    // The add function returns 100 (42 + 58)
    assert_eq!(result, 100i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v41_execute_simple() -> Result<()> {
    let wasm_bytes = load_fixture("simple");
    validate_wasm(&wasm_bytes)?;

    let config = ConfigV41::new();
    let engine = WasmtimeV41::new(&config).context("创建 v41 引擎失败")?;
    let module = engine
        .load_module(&wasm_bytes)
        .context("无法加载 WASM 模块")?;

    let result = engine
        .execute(&module, "add", &[])
        .context("执行函数失败")?;

    // The add function returns 100 (42 + 58)
    assert_eq!(result, 100i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v21_execute_complex() -> Result<()> {
    let wasm_bytes = load_fixture("complex");
    validate_wasm(&wasm_bytes)?;

    let config = Config::new();
    let engine = WasmtimeV21::new(&config).context("创建 v21 引擎失败")?;
    let module = engine
        .load_module(&wasm_bytes)
        .context("无法加载 WASM 模块")?;

    let result = engine
        .execute(&module, "fibonacci", &[])
        .context("执行函数失败")?;

    // The fibonacci function returns fib(11) = 89
    assert_eq!(result, 89i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v41_execute_complex() -> Result<()> {
    let wasm_bytes = load_fixture("complex");
    validate_wasm(&wasm_bytes)?;

    let config = ConfigV41::new();
    let engine = WasmtimeV41::new(&config).context("创建 v41 引擎失败")?;
    let module = engine
        .load_module(&wasm_bytes)
        .context("无法加载 WASM 模块")?;

    let result = engine
        .execute(&module, "fibonacci", &[])
        .context("执行函数失败")?;

    // The fibonacci function returns fib(11) = 89
    assert_eq!(result, 89i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_version_consistency() -> Result<()> {
    let config = Config::new();
    let config_v41 = ConfigV41::new();
    let engine_v21 = WasmtimeV21::new(&config).context("创建 v21 引擎失败")?;
    let engine_v41 = WasmtimeV41::new(&config_v41).context("创建 v41 引擎失败")?;

    assert_eq!(engine_v21.version(), "wasmtime-21.0");
    assert_eq!(engine_v41.version(), "wasmtime-41.0");

    assert_ne!(engine_v21.version(), engine_v41.version());
    Ok(())
}
