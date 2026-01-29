use anyhow::{Context, Result};
use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, Criterion};
use engine::v21::{self, component::Val as ValV21};
use engine::v41::{self, component::Val as ValV41};
use std::path::PathBuf;
use std::time::Instant;

use engine::v21::{
    component::Component as ComponentV21, component::Linker as LinkerV21, Config as ConfigV21,
    Engine as EngineV21, Store as StoreV21,
};
use engine::v41::wasi::p2::add_to_linker_sync as add_to_linker_sync_v41;
use engine::v41::{
    component::Component as ComponentV41, component::Linker as LinkerV41, Config as ConfigV41,
    Engine as EngineV41, Store as StoreV41,
};
fn get_golden_wasm_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../golden/out");
    path.push(filename);
    path
}

/// Setup v21 engine and component for call benchmark
fn setup_engine_v21(path: &PathBuf) -> Result<(EngineV21, ComponentV21)> {
    let mut config = ConfigV21::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = EngineV21::new(&config).context("Failed to create v21 engine")?;
    let component = ComponentV21::from_file(&engine, path)
        .with_context(|| format!("Failed to load v21 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Setup v41 engine and component for call benchmark
fn setup_engine_v41(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark call performance for v21 engine
fn benchmark_call_v21(c: &mut Criterion, wasm_file: &str, func_name: &str, params: &[ValV21]) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v21(&wasm_path).expect("Setup v21 failed");
    let mut linker = LinkerV21::new(&engine);

    v21::wasi::add_to_linker_async(&mut linker).expect("link wasip1");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let setup = || async {
        let mut store = StoreV21::new(&engine, v21::WasiP2State::default());
        let instance = pre_instance
            .instantiate_async(&mut store)
            .await
            .expect("Instantiation failed");

        (store, instance)
    };

    let group_name = format!(
        "call_async_{}_{}_v21",
        wasm_file.replace(".wasm", ""),
        func_name.replace('/', "_")
    );
    c.bench_function(&group_name, move |b| {
        b.to_async(FuturesExecutor).iter_custom(|iters| async move {
            let (mut store, instance) = setup().await;

            let func = v21::find_func(&instance, &mut store, func_name).expect("find func");

            let start = Instant::now();

            for _ in 0..iters {
                let mut results = [ValV21::Bool(false); 1];
                func.call_async(&mut store, &params, &mut results)
                    .await
                    .expect("Call failed");
                std::hint::black_box(
                    func.post_return_async(&mut store)
                        .await
                        .expect("unexpected error"),
                );
            }
            start.elapsed()
        })
    });
}

/// Benchmark call performance for v41 engine
fn benchmark_call_v41(c: &mut Criterion, wasm_file: &str, func_name: &str, params: &[ValV41]) {
    let wasm_path = get_golden_wasm_path(wasm_file);
    let (engine, component) = setup_engine_v41(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    add_to_linker_sync_v41(&mut linker).expect("link wasip2");

    let pre_instance = linker.instantiate_pre(&component).expect("instantiate-pre");

    let setup = || async {
        let mut store = StoreV41::new(&engine, v41::WasiP2State::default());
        let instance = pre_instance
            .instantiate_async(&mut store)
            .await
            .expect("Instantiation failed");

        (store, instance)
    };

    let group_name = format!(
        "call_async_{}_{}_v41",
        wasm_file.replace(".wasm", ""),
        func_name.replace('/', "_")
    );

    c.bench_function(&group_name, move |b| {
        b.to_async(FuturesExecutor).iter_custom(|iters| async move {
            let (mut store, instance) = setup().await;

            let func = v41::find_func(&instance, &mut store, func_name).expect("find func");

            let start = Instant::now();

            for _ in 0..iters {
                let mut results = [ValV41::Bool(false); 1];
                func.call_async(&mut store, &params, &mut results)
                    .await
                    .expect("Call failed");
                std::hint::black_box(
                    func.post_return_async(&mut store)
                        .await
                        .expect("unexpected error"),
                );
            }

            start.elapsed()
        })
    });
}

/// Benchmark argon2.wasm hash function with v21
fn benchmark_call_argon2_v21(c: &mut Criterion) {
    let password = {
        let v = (0..=255).cycle().take(1024).map(ValV21::U8).collect();
        ValV21::List(v)
    };
    let salt = ValV21::String("hello-world-hello-world".to_owned());

    let params = [password, salt];

    benchmark_call_v21(c, "argon2.wasm", "sammyne:argon2/api@1.0.0#hash", &params);
}

/// Benchmark argon2.wasm hash function with v41
fn benchmark_call_argon2_v41(c: &mut Criterion) {
    let password = {
        let v = (0..=255).cycle().take(1024).map(ValV41::U8).collect();
        ValV41::List(v)
    };
    let salt = ValV41::String("hello-world-hello-world".to_owned());

    let params = [password, salt];

    benchmark_call_v41(c, "argon2.wasm", "sammyne:argon2/api@1.0.0#hash", &params);
}

/// Benchmark pulldown-cmark.wasm parse function with v21
fn benchmark_call_pulldown_cmark_v21(c: &mut Criterion) {
    let markdown =
        ValV21::String("Hello world, this is a ~~complicated~~ *very simple* example.".to_owned());

    let params = [markdown];

    benchmark_call_v21(
        c,
        "pulldown-cmark.wasm",
        "sammyne:pulldown-cmark/api@1.0.0#parse",
        &params,
    );
}

/// Benchmark pulldown-cmark.wasm parse function with v41
fn benchmark_call_pulldown_cmark_v41(c: &mut Criterion) {
    let markdown =
        ValV41::String("Hello world, this is a ~~complicated~~ *very simple* example.".to_owned());

    let params = [markdown];

    benchmark_call_v41(
        c,
        "pulldown-cmark.wasm",
        "sammyne:pulldown-cmark/api@1.0.0#parse",
        &params,
    );
}

/// Benchmark sevenz-7z.wasm zip function with v21
fn benchmark_call_sevenz_7z_zip_v21(c: &mut Criterion) {
    let req = {
        let v = (0..=255).cycle().take(1024).map(ValV21::U8).collect();
        ValV21::List(v)
    };

    let params = [req];

    benchmark_call_v21(c, "sevenz-7z.wasm", "sammyne:sevenz7z/api@1.0.0#zip", &params);
}

/// Benchmark sevenz-7z.wasm zip function with v41
fn benchmark_call_sevenz_7z_zip_v41(c: &mut Criterion) {
    let req = {
        let v = (0..=255).cycle().take(1024).map(ValV41::U8).collect();
        ValV41::List(v)
    };

    let params = [req];

    benchmark_call_v41(c, "sevenz-7z.wasm", "sammyne:sevenz7z/api@1.0.0#zip", &params);
}

// /// Benchmark sevenz-7z.wasm unzip function with v21
// fn benchmark_call_sevenz_7z_unzip_v21(c: &mut Criterion) {
//     benchmark_call_v21(c, "sevenz-7z.wasm", "sammyne:sevenz7z/api@1.0.0#unzip");
// }

// /// Benchmark sevenz-7z.wasm unzip function with v41
// fn benchmark_call_sevenz_7z_unzip_v41(c: &mut Criterion) {
//     benchmark_call_v41(c, "sevenz-7z.wasm", "sammyne:sevenz7z/api@1.0.0#unzip");
// }

criterion_group!(
    benches,
    benchmark_call_argon2_v21,
    benchmark_call_argon2_v41,
    benchmark_call_pulldown_cmark_v21,
    benchmark_call_pulldown_cmark_v41,
    benchmark_call_sevenz_7z_zip_v21,
    benchmark_call_sevenz_7z_zip_v41,
    // benchmark_call_sevenz_7z_unzip_v21,
    // benchmark_call_sevenz_7z_unzip_v41
);
criterion_main!(benches);
