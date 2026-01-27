use anyhow::{Context, Result};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use testdata::{load_fixture, validate_wasm};

use engine::v21::{Config as ConfigV21, Engine as EngineV21, Module as ModuleV21};
use engine::v21::{execute as execute_v21};
use engine::v41::{Config as ConfigV41, Engine as EngineV41, Module as ModuleV41};
use engine::v41::{execute as execute_v41};

fn setup_engine_v21(wasm_bytes: &[u8]) -> Result<(EngineV21, ModuleV21)> {
    validate_wasm(wasm_bytes).context("验证 WASM 格式失败")?;

    let config = ConfigV21::new();
    let engine = EngineV21::new(&config).context("创建 v21 引擎失败")?;
    let module = ModuleV21::from_binary(&engine, wasm_bytes).context("加载 WASM 模块失败")?;

    Ok((engine, module))
}

fn setup_engine_v41(wasm_bytes: &[u8]) -> Result<(EngineV41, ModuleV41)> {
    validate_wasm(wasm_bytes).context("验证 WASM 格式失败")?;

    let config = ConfigV41::new();
    let engine = EngineV41::new(&config).context("创建 v41 引擎失败")?;
    let module = ModuleV41::from_binary(&engine, wasm_bytes).context("加载 WASM 模块失败")?;

    Ok((engine, module))
}

fn benchmark_simple_arithmetic(c: &mut Criterion) {
    let wasm_bytes = load_fixture("simple");
    let function_name = "add";

    let mut group = c.benchmark_group("simple_arithmetic");

    // Benchmark Wasmtime v21
    let (engine_v21, module_v21) =
        setup_engine_v21(&wasm_bytes).expect("设置 WasmtimeV21 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v21", function_name),
        &module_v21,
        |b, module| {
            b.iter(|| {
                black_box(
                    execute_v21(&engine_v21, module, function_name, &[])
                        .expect("执行失败"),
                )
            })
        },
    );

    // Benchmark Wasmtime v41
    let (engine_v41, module_v41) =
        setup_engine_v41(&wasm_bytes).expect("设置 WasmtimeV41 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v41", function_name),
        &module_v41,
        |b, module| {
            b.iter(|| {
                black_box(
                    execute_v41(&engine_v41, module, function_name, &[])
                        .expect("执行失败"),
                )
            })
        },
    );

    group.finish();
}

fn benchmark_complex_calculation(c: &mut Criterion) {
    let wasm_bytes = load_fixture("complex");
    let function_name = "fibonacci";

    let mut group = c.benchmark_group("complex_calculation");

    // Benchmark Wasmtime v21
    let (engine_v21, module_v21) =
        setup_engine_v21(&wasm_bytes).expect("设置 WasmtimeV21 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v21", function_name),
        &module_v21,
        |b, module| {
            b.iter(|| {
                black_box(
                    execute_v21(&engine_v21, module, function_name, &[])
                        .expect("执行失败"),
                )
            })
        },
    );

    // Benchmark Wasmtime v41
    let (engine_v41, module_v41) =
        setup_engine_v41(&wasm_bytes).expect("设置 WasmtimeV41 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v41", function_name),
        &module_v41,
        |b, module| {
            b.iter(|| {
                black_box(
                    execute_v41(&engine_v41, module, function_name, &[])
                        .expect("执行失败"),
                )
            })
        },
    );

    group.finish();
}

criterion_group!(
    benches,
    benchmark_simple_arithmetic,
    benchmark_complex_calculation
);
criterion_main!(benches);
