use anyhow::{Context, Result};
use testdata::{load_fixture, validate_wasm};

use engine::v21::{Config, Engine as EngineV21, Module as ModuleV21};
use engine::v21::{execute as execute_v21, version as version_v21};
use engine::v41::{Config as ConfigV41, Engine as EngineV41, Module as ModuleV41};
use engine::v41::{execute as execute_v41, version as version_v41};

#[test]
fn test_wasmtime_v21_execute_simple() -> Result<()> {
    let wasm_bytes = load_fixture("simple");
    validate_wasm(&wasm_bytes)?;

    let config = Config::new();
    let engine = EngineV21::new(&config).context("创建 v21 引擎失败")?;
    let module = ModuleV21::from_binary(&engine, &wasm_bytes).context("无法加载 WASM 模块")?;

    let result = execute_v21(&engine, &module, "add", &[]).context("执行函数失败")?;

    // The add function returns 100 (42 + 58)
    assert_eq!(result, 100i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v41_execute_simple() -> Result<()> {
    let wasm_bytes = load_fixture("simple");
    validate_wasm(&wasm_bytes)?;

    let config = ConfigV41::new();
    let engine = EngineV41::new(&config).context("创建 v41 引擎失败")?;
    let module = ModuleV41::from_binary(&engine, &wasm_bytes).context("无法加载 WASM 模块")?;

    let result = execute_v41(&engine, &module, "add", &[]).context("执行函数失败")?;

    // The add function returns 100 (42 + 58)
    assert_eq!(result, 100i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v21_execute_complex() -> Result<()> {
    let wasm_bytes = load_fixture("complex");
    validate_wasm(&wasm_bytes)?;

    let config = Config::new();
    let engine = EngineV21::new(&config).context("创建 v21 引擎失败")?;
    let module = ModuleV21::from_binary(&engine, &wasm_bytes).context("无法加载 WASM 模块")?;

    let result = execute_v21(&engine, &module, "fibonacci", &[]).context("执行函数失败")?;

    // The fibonacci function returns fib(11) = 89
    assert_eq!(result, 89i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_wasmtime_v41_execute_complex() -> Result<()> {
    let wasm_bytes = load_fixture("complex");
    validate_wasm(&wasm_bytes)?;

    let config = ConfigV41::new();
    let engine = EngineV41::new(&config).context("创建 v41 引擎失败")?;
    let module = ModuleV41::from_binary(&engine, &wasm_bytes).context("无法加载 WASM 模块")?;

    let result = execute_v41(&engine, &module, "fibonacci", &[]).context("执行函数失败")?;

    // The fibonacci function returns fib(11) = 89
    assert_eq!(result, 89i32.to_le_bytes().to_vec());
    Ok(())
}

#[test]
fn test_version_consistency() -> Result<()> {
    assert_eq!(version_v21(), "wasmtime-21.0");
    assert_eq!(version_v41(), "wasmtime-41.0");

    assert_ne!(version_v21(), version_v41());
    Ok(())
}
