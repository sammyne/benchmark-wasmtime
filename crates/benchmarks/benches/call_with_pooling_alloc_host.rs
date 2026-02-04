use std::path::PathBuf;

use anyhow::{Context, Result};
use criterion::{Criterion, criterion_group, criterion_main};
use engine::v21::component::{Component as ComponentV21, Linker as LinkerV21, Val as ValV21};
use engine::v21::{self, Config as ConfigV21, Engine as EngineV21, Store as StoreV21};
use engine::v41::component::{Component as ComponentV41, Linker as LinkerV41, Val as ValV41};
use engine::v41::{self, Config as ConfigV41, Engine as EngineV41, Store as StoreV41};
use v21host::Host as HostV21;
use v41host::Host as HostV41;

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
    config.allocation_strategy(v21::InstanceAllocationStrategy::Pooling(
        v21::PoolingAllocationConfig::default(),
    ));

    let engine = EngineV21::new(&config).context("Failed to create v21 engine")?;
    let component = ComponentV21::from_file(&engine, path)
        .with_context(|| format!("Failed to load v21 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Setup v41 engine and component with host interface
fn setup_engine_v41_with_host(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.allocation_strategy(v41::PoolingAllocationConfig::default());

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark call performance for v21 engine with host interface
fn benchmark_v21(c: &mut Criterion, wasm_file: &str, func_name: &str, params: &[ValV21]) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v21_with_host(&wasm_path).expect("Setup v21 failed");
    let mut linker = LinkerV21::new(&engine);

    // v21::wasi::add_to_linker_async(&mut linker).expect("link wasip1");
    HostV21::link(&mut linker).expect("link host");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let setup = || {
        let mut store = StoreV21::new(&engine, HostV21::default());
        let instance = pre_instance.instantiate(&mut store).expect("Instantiation failed");

        (store, instance)
    };

    let func_name_short = func_name
        .rsplit_once('#')
        .expect(&format!("get short func_name from {func_name}"))
        .1;

    let group_name = format!(
        "call_with_pooling_alloc_host_{}_{}_v21",
        wasm_file.replace(".wasm", ""),
        func_name_short
    );
    c.bench_function(&group_name, move |b| {
        b.iter(|| async move {
            let (mut store, instance) = setup();

            let func = v21::find_func(&instance, &mut store, func_name).expect("find func");

            let mut results = [ValV21::Bool(false); 1];
            func.call(&mut store, &params, &mut results).expect("Call failed");
            std::hint::black_box(func.post_return(&mut store).expect("unexpected error"));
        })
    });
}

/// Benchmark call performance for v41 engine with host interface
fn benchmark_v41(c: &mut Criterion, wasm_file: &str, func_name: &str, params: &[ValV41]) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v41_with_host(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    HostV41::link(&mut linker).expect("link host");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let setup = || {
        let mut store = StoreV41::new(&engine, HostV41::default());
        let instance = pre_instance.instantiate(&mut store).expect("Instantiation failed");

        (store, instance)
    };

    let func_name_short = func_name
        .rsplit_once('#')
        .expect(&format!("get short func_name from {func_name}"))
        .1;

    let group_name = format!(
        "call_with_pooling_alloc_host_{}_{}_v41",
        wasm_file.replace(".wasm", ""),
        func_name_short
    );

    c.bench_function(&group_name, move |b| {
        b.iter(|| async {
            let (mut store, instance) = setup();

            let func = v41::find_func(&instance, &mut store, func_name).expect("find func");

            let mut results = [ValV41::Bool(false); 1];
            func.call(&mut store, &params, &mut results).expect("Call failed");
            std::hint::black_box(func.post_return(&mut store).expect("unexpected error"));
        })
    });
}

/// Benchmark host-caller.wasm echo function with v21
fn benchmark_call_host_caller_echo_v21(c: &mut Criterion) {
    let times = ValV21::U64(1000);

    let params = [times];

    benchmark_v21(c, "host-caller.wasm", "sammyne:host-caller/api@1.0.0#echo", &params);
}

/// Benchmark host-caller.wasm echo function with v41
fn benchmark_call_host_caller_echo_v41(c: &mut Criterion) {
    let times = ValV41::U64(1000);

    let params = [times];

    benchmark_v41(c, "host-caller.wasm", "sammyne:host-caller/api@1.0.0#echo", &params);
}

criterion_group!(
    benches,
    benchmark_call_host_caller_echo_v21,
    benchmark_call_host_caller_echo_v41,
);
criterion_main!(benches);
