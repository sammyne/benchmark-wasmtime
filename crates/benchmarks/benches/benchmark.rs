use anyhow::{Context, Result};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use engine::EngineApi;
use std::hint::black_box;
use testdata::{load_fixture, validate_wasm};
use wasmtime_v21::WasmtimeV21;
use wasmtime_v41::WasmtimeV41;

fn setup_engine_v21(wasm_bytes: &[u8]) -> Result<(WasmtimeV21, <WasmtimeV21 as EngineApi>::ModuleType)> {
    validate_wasm(wasm_bytes).context("验证 WASM 格式失败")?;

    let engine = WasmtimeV21::new();
    let module = engine.load_module(wasm_bytes).context("加载 WASM 模块失败")?;

    Ok((engine, module))
}

fn setup_engine_v41(wasm_bytes: &[u8]) -> Result<(WasmtimeV41, <WasmtimeV41 as EngineApi>::ModuleType)> {
    validate_wasm(wasm_bytes).context("验证 WASM 格式失败")?;

    let engine = WasmtimeV41::new();
    let module = engine.load_module(wasm_bytes).context("加载 WASM 模块失败")?;

    Ok((engine, module))
}

fn benchmark_simple_arithmetic(c: &mut Criterion) {
    let wasm_bytes = load_fixture("simple");
    let function_name = "add";

    let mut group = c.benchmark_group("simple_arithmetic");

    // Benchmark Wasmtime v21
    let (engine_v21, module_v21) = setup_engine_v21(&wasm_bytes).expect("设置 WasmtimeV21 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v21", function_name),
        &module_v21,
        |b, module| b.iter(|| black_box(engine_v21.execute(module, function_name, &[]).expect("执行失败"))),
    );

    // Benchmark Wasmtime v41
    let (engine_v41, module_v41) = setup_engine_v41(&wasm_bytes).expect("设置 WasmtimeV41 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v41", function_name),
        &module_v41,
        |b, module| b.iter(|| black_box(engine_v41.execute(module, function_name, &[]).expect("执行失败"))),
    );

    group.finish();
}

fn benchmark_complex_calculation(c: &mut Criterion) {
    let wasm_bytes = load_fixture("complex");
    let function_name = "fibonacci";

    let mut group = c.benchmark_group("complex_calculation");

    // Benchmark Wasmtime v21
    let (engine_v21, module_v21) = setup_engine_v21(&wasm_bytes).expect("设置 WasmtimeV21 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v21", function_name),
        &module_v21,
        |b, module| b.iter(|| black_box(engine_v21.execute(module, function_name, &[]).expect("执行失败"))),
    );

    // Benchmark Wasmtime v41
    let (engine_v41, module_v41) = setup_engine_v41(&wasm_bytes).expect("设置 WasmtimeV41 引擎失败");
    group.bench_with_input(
        BenchmarkId::new("wasmtime-v41", function_name),
        &module_v41,
        |b, module| b.iter(|| black_box(engine_v41.execute(module, function_name, &[]).expect("执行失败"))),
    );

    group.finish();
}

criterion_group!(
    benches,
    benchmark_simple_arithmetic,
    benchmark_complex_calculation
);
criterion_main!(benches);
