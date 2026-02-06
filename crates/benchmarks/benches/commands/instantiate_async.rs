use std::hint;
use std::path::PathBuf;

use anyhow::{Context, Result};
use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use engine::v41::component::{Component as ComponentV41, Linker as LinkerV41};
use engine::v41::wasi::p2::bindings::CommandPre as CommandPreV41;
use engine::v41::{self, Config as ConfigV41, Engine as EngineV41, Store as StoreV41};

/// Setup v41 engine and component for instantiation benchmark
fn setup(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark instantiation performance for v41 engine
fn benchmark_v41(c: &mut Criterion, name: &str) {
    let wasm_path = golden::cmd_path(name);
    let (engine, component) = setup(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    v41::wasi::p2::add_to_linker_sync(&mut linker).expect("link wasip2");

    let pre_instance =
        CommandPreV41::new(linker.instantiate_pre(&component).expect("instantiate-pre")).expect("new command-pre");

    let id = format!("instantiate_async_{name}_v41");
    c.bench_function(&id, |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let mut store = StoreV41::new(&engine, engine::v41::WasiP2State::default());
            let _command = hint::black_box(
                pre_instance
                    .instantiate_async(&mut store)
                    .await
                    .expect("Instantiation failed"),
            );
        })
    });
}

/// Benchmark argon2.wasm instantiation with v41
fn benchmark_instantiate_argon2_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::ARGON2);
}

/// Benchmark pulldown-cmark.wasm instantiation with v41
fn benchmark_instantiate_pulldown_cmark_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::PULLDOWN_CMARK);
}

/// Benchmark sevenz-7z.wasm instantiation with v41
fn benchmark_instantiate_rust_python_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::RUST_PYTHON);
}

/// Benchmark sevenz-7z.wasm instantiation with v41
fn benchmark_instantiate_sevenz_7z_v41(c: &mut Criterion) {
    benchmark_v41(c, golden::SEVENZ_7Z);
}

criterion_group!(
    benches,
    benchmark_instantiate_argon2_v41,
    benchmark_instantiate_pulldown_cmark_v41,
    benchmark_instantiate_rust_python_v41,
    benchmark_instantiate_sevenz_7z_v41
);
criterion_main!(benches);
