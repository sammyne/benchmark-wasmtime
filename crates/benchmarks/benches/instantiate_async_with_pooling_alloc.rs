use std::hint::black_box;
use std::path::PathBuf;

use anyhow::{Context, Result};
use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use engine::v21;
use engine::v21::component::{Component as ComponentV21, Linker as LinkerV21};
use engine::v21::{Config as ConfigV21, Engine as EngineV21, Store as StoreV21};
use engine::v41::component::{Component as ComponentV41, Linker as LinkerV41};
use engine::v41::{self, Config as ConfigV41, Engine as EngineV41, Store as StoreV41};

/// Setup v21 engine and component for instantiation benchmark
fn setup_v21(path: &PathBuf) -> Result<(EngineV21, ComponentV21)> {
    let mut config = ConfigV21::new();
    config.wasm_component_model(true);
    config.async_support(true);

    // rust-python 依赖 558 个内存页
    let mut alloc_config = v21::PoolingAllocationConfig::default();
    alloc_config.memory_pages(2000);
    config.allocation_strategy(v21::InstanceAllocationStrategy::Pooling(alloc_config));

    let engine = EngineV21::new(&config).context("Failed to create v21 engine")?;
    let component = ComponentV21::from_file(&engine, path)
        .with_context(|| format!("Failed to load v21 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Setup v41 engine and component for instantiation benchmark
fn setup_v41(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config.allocation_strategy(v41::PoolingAllocationConfig::default());

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark instantiation performance for v21 engine
fn benchmark_v21(c: &mut Criterion, name: &str) {
    let wasm_path = golden::reactor_path(name);
    let (engine, component) = setup_v21(&wasm_path).expect("Setup v21 failed");
    let mut linker = LinkerV21::new(&engine);

    v21::wasi::add_to_linker_async(&mut linker).expect("link wasip1");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let id = format!("instantiate_async_with_pooling_alloc_{name}_v21");
    c.bench_function(&id, move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let mut store = StoreV21::new(&engine, v21::WasiP2State::default());
            black_box(
                pre_instance
                    .instantiate_async(&mut store)
                    .await
                    .expect("Instantiation failed"),
            );
        })
    });
}

/// Benchmark instantiation performance for v41 engine
fn benchmark_v41(c: &mut Criterion, name: &str) {
    let wasm_path = golden::reactor_path(name);
    let (engine, component) = setup_v41(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    v41::wasi::p2::add_to_linker_sync(&mut linker).expect("link wasip2");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let id = format!("instantiate_async_with_pooling_alloc_{name}_v41");
    c.bench_function(&id, |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let mut store = StoreV41::new(&engine, engine::v41::WasiP2State::default());
            let _ii = black_box(
                pre_instance
                    .instantiate_async(&mut store)
                    .await
                    .expect("Instantiation failed"),
            );
        })
    });
}

/// Benchmark argon2.wasm instantiation with v21
fn benchmark_instantiate_argon2_v21(c: &mut Criterion) {
    benchmark_v21(c, golden::ARGON2);
}

/// Benchmark argon2.wasm instantiation with v41
fn benchmark_instantiate_argon2_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::ARGON2);
}

/// Benchmark pulldown-cmark.wasm instantiation with v21
fn benchmark_instantiate_pulldown_cmark_v21(c: &mut Criterion) {
    benchmark_v21(c, golden::PULLDOWN_CMARK);
}

/// Benchmark pulldown-cmark.wasm instantiation with v41
fn benchmark_instantiate_pulldown_cmark_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::PULLDOWN_CMARK);
}

/// Benchmark sevenz-7z.wasm instantiation with v21
fn benchmark_instantiate_rust_python_v21(c: &mut Criterion) {
    benchmark_v21(c, golden::RUST_PYTHON);
}

/// Benchmark sevenz-7z.wasm instantiation with v41
fn benchmark_instantiate_rust_python_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::RUST_PYTHON);
}

/// Benchmark sevenz-7z.wasm instantiation with v21
fn benchmark_instantiate_sevenz_7z_v21(c: &mut Criterion) {
    benchmark_v21(c, golden::SEVENZ_7Z);
}

/// Benchmark sevenz-7z.wasm instantiation with v41
fn benchmark_instantiate_sevenz_7z_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::SEVENZ_7Z);
}

criterion_group!(
    benches,
    benchmark_instantiate_argon2_v21,
    benchmark_instantiate_argon2_v41,
    benchmark_instantiate_pulldown_cmark_v21,
    benchmark_instantiate_pulldown_cmark_v41,
    benchmark_instantiate_rust_python_v21,
    benchmark_instantiate_rust_python_v41,
    benchmark_instantiate_sevenz_7z_v21,
    benchmark_instantiate_sevenz_7z_v41
);
criterion_main!(benches);
