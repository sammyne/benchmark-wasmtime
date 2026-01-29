use anyhow::{Context, Result};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use engine::v21;
use std::hint::black_box;
use std::path::PathBuf;

use engine::v21::{
    Config as ConfigV21, Engine as EngineV21, Store as StoreV21,
    component::Component as ComponentV21, component::Linker as LinkerV21,
};
use engine::v41::wasi::p2::add_to_linker_sync as add_to_linker_sync_v41;
use engine::v41::{
    Config as ConfigV41, Engine as EngineV41, Store as StoreV41,
    component::Component as ComponentV41, component::Linker as LinkerV41,
};

/// Load a WASM component file path from the golden/out directory
fn get_golden_wasm_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../golden/out");
    path.push(filename);
    path
}

/// Setup v21 engine and component for instantiation benchmark
fn setup_engine_v21(path: &PathBuf) -> Result<(EngineV21, ComponentV21)> {
    let mut config = ConfigV21::new();
    config.wasm_component_model(true);

    let engine = EngineV21::new(&config).context("Failed to create v21 engine")?;
    let component = ComponentV21::from_file(&engine, path)
        .with_context(|| format!("Failed to load v21 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Setup v41 engine and component for instantiation benchmark
fn setup_engine_v41(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark instantiation performance for v21 engine
fn benchmark_instantiate_v21(c: &mut Criterion, wasm_file: &str) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v21(&wasm_path).expect("Setup v21 failed");
    let mut linker = LinkerV21::new(&engine);

    v21::wasi::add_to_linker_sync(&mut linker).expect("link wasip1");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let mut group = c.benchmark_group(format!(
        "instantiate_{}_v21",
        wasm_file.replace(".wasm", "")
    ));
    group.bench_function(BenchmarkId::new("wasmtime-v21", wasm_file), |b| {
        b.iter(|| {
            let mut store = StoreV21::new(&engine, v21::WasiP2State::default());
            black_box(
                pre_instance
                    .instantiate(&mut store)
                    .expect("Instantiation failed"),
            );
        })
    });
    group.finish();
}

/// Benchmark instantiation performance for v41 engine
fn benchmark_instantiate_v41(c: &mut Criterion, wasm_file: &str) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v41(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    add_to_linker_sync_v41(&mut linker).expect("link wasip2");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let mut group = c.benchmark_group(format!(
        "instantiate_{}_v41",
        wasm_file.replace(".wasm", "")
    ));
    group.bench_function(BenchmarkId::new("wasmtime-v41", wasm_file), |b| {
        b.iter(|| {
            let mut store = StoreV41::new(&engine, engine::v41::WasiP2State::default());
            black_box(
                pre_instance
                    .instantiate(&mut store)
                    .expect("Instantiation failed"),
            );
        })
    });
    group.finish();
}

/// Benchmark argon2.wasm instantiation with v21
fn benchmark_instantiate_argon2_v21(c: &mut Criterion) {
    benchmark_instantiate_v21(c, "argon2.wasm");
}

/// Benchmark argon2.wasm instantiation with v41
fn benchmark_instantiate_argon2_v41(c: &mut Criterion) {
    benchmark_instantiate_v41(c, "argon2.wasm");
}

/// Benchmark pulldown-cmark.wasm instantiation with v21
fn benchmark_instantiate_pulldown_cmark_v21(c: &mut Criterion) {
    benchmark_instantiate_v21(c, "pulldown-cmark.wasm");
}

/// Benchmark pulldown-cmark.wasm instantiation with v41
fn benchmark_instantiate_pulldown_cmark_v41(c: &mut Criterion) {
    benchmark_instantiate_v41(c, "pulldown-cmark.wasm");
}

/// Benchmark sevenz-7z.wasm instantiation with v21
fn benchmark_instantiate_sevenz_7z_v21(c: &mut Criterion) {
    benchmark_instantiate_v21(c, "sevenz-7z.wasm");
}

/// Benchmark sevenz-7z.wasm instantiation with v41
fn benchmark_instantiate_sevenz_7z_v41(c: &mut Criterion) {
    benchmark_instantiate_v41(c, "sevenz-7z.wasm");
}

criterion_group!(
    benches,
    benchmark_instantiate_argon2_v21,
    benchmark_instantiate_argon2_v41,
    benchmark_instantiate_pulldown_cmark_v21,
    benchmark_instantiate_pulldown_cmark_v41,
    benchmark_instantiate_sevenz_7z_v21,
    benchmark_instantiate_sevenz_7z_v41
);
criterion_main!(benches);
