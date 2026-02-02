use anyhow::{Context, Result};
use criterion::{Criterion, criterion_group, criterion_main};
use std::path::PathBuf;

use engine::v21::{
    Config as ConfigV21, Engine as EngineV21, Store as StoreV21,
    component::Component as ComponentV21, component::Linker as LinkerV21,
};
use engine::v41::{
    Config as ConfigV41, Engine as EngineV41, Store as StoreV41,
    component::Component as ComponentV41, component::Linker as LinkerV41,
};

use v21ahost::Host as HostV21;
use v41ahost::Host as HostV41;

fn get_golden_wasm_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../golden/out");
    path.push(filename);
    path
}

/// Setup v21 engine and component with host interface
fn setup_engine_v21_with_host(path: &PathBuf) -> Result<(EngineV21, ComponentV21)> {
    let mut config = ConfigV21::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = EngineV21::new(&config).context("Failed to create v21 engine")?;
    let component = ComponentV21::from_file(&engine, path)
        .with_context(|| format!("Failed to load v21 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Setup v41 engine and component with host interface
fn setup_engine_v41_with_host(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark call performance for v21 engine with host interface
fn benchmark_v21(c: &mut Criterion, wasm_file: &str) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v21_with_host(&wasm_path).expect("Setup v21 failed");
    let mut linker = LinkerV21::new(&engine);

    // v21::wasi::add_to_linker_async(&mut linker).expect("link wasip1");
    HostV21::link(&mut linker).expect("link host");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .expect("build tokio");

    let group_name = format!(
        "instantiate_async_with_host_{}_v21",
        wasm_file.replace(".wasm", ""),
    );
    c.bench_function(&group_name, move |b| {
        b.to_async(&rt).iter(|| async {
            let mut store = StoreV21::new(&engine, HostV21::default());
            let _instance = std::hint::black_box(
                pre_instance
                    .instantiate_async(&mut store)
                    .await
                    .expect("Instantiation failed"),
            );
        })
    });
}

/// Benchmark call performance for v41 engine with host interface
fn benchmark_v41(c: &mut Criterion, wasm_file: &str) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v41_with_host(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    HostV41::link(&mut linker).expect("link host");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let group_name = format!(
        "instantiate_async_with_host_{}_v41",
        wasm_file.replace(".wasm", ""),
    );

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .expect("build tokio");

    c.bench_function(&group_name, move |b| {
        b.to_async(&rt).iter(|| async {
            let mut store = StoreV41::new(&engine, HostV41::default());
            let _instance = std::hint::black_box(
                pre_instance
                    .instantiate_async(&mut store)
                    .await
                    .expect("Instantiation failed"),
            );
        })
    });
}

/// Benchmark host-caller.wasm echo function with v21
fn benchmark_instantiate_host_caller_v21(c: &mut Criterion) {
    benchmark_v21(c, "host-caller.wasm");
}

/// Benchmark host-caller.wasm echo function with v41
fn benchmark_instantiate_host_caller_v41(c: &mut Criterion) {
    benchmark_v41(c, "host-caller.wasm");
}

criterion_group!(
    benches,
    benchmark_instantiate_host_caller_v21,
    benchmark_instantiate_host_caller_v41,
);
criterion_main!(benches);
