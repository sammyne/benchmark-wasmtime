use std::hint;
use std::path::PathBuf;

use anyhow::{Context, Result};
use criterion::async_executor::FuturesExecutor;
use criterion::{Criterion, criterion_group, criterion_main};
use engine::v41::component::{Component as ComponentV41, Linker as LinkerV41};
use engine::v41::wasi::WasiCtx;
use engine::v41::wasi::p2::bindings::CommandPre;
use engine::v41::{self, Config as ConfigV41, Engine as EngineV41, Store as StoreV41};

/// Setup v41 engine and component for call benchmark
fn setup_v41(path: &PathBuf) -> Result<(EngineV41, ComponentV41)> {
    let mut config = ConfigV41::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let alloc_config = v41::PoolingAllocationConfig::default();
    config.allocation_strategy(v41::InstanceAllocationStrategy::Pooling(alloc_config));

    let engine = EngineV41::new(&config).context("Failed to create v41 engine")?;
    let component = ComponentV41::from_file(&engine, path)
        .with_context(|| format!("Failed to load v41 component from: {}", path.display()))?;

    Ok((engine, component))
}

/// Benchmark call performance for v41 engine
fn benchmark_v41(c: &mut Criterion, name: &str, params: &[String]) {
    let wasm_path = golden::cmd_path(name);
    let (engine, component) = setup_v41(&wasm_path).expect("Setup v41 failed");
    let mut linker = LinkerV41::new(&engine);

    v41::wasi::p2::add_to_linker_async(&mut linker).expect("link wasip2");

    let pre_instance =
        CommandPre::new(linker.instantiate_pre(&component).expect("instantiate-pre")).expect("new command-pre");

    let args = {
        let mut p = params.to_vec();
        p.insert(0, "-".to_owned());
        p
    };

    let setup = || async {
        let mut builder = WasiCtx::builder();
        builder.args(&args);
        let data = v41::WasiP2State::from(builder);

        let mut store = StoreV41::new(&engine, data);
        let instance = pre_instance
            .instantiate_async(&mut store)
            .await
            .expect("Instantiation failed");

        (store, instance)
    };

    let id = format!("cmd_call_async_with_pooling_alloc_{name}_v41");

    c.bench_function(&id, move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let (mut store, cmd) = setup().await;
            hint::black_box(cmd.wasi_cli_run().call_run(&mut store).await)
                .expect("call run")
                .expect("main error out");
        });
    });
}

/// Benchmark argon2.wasm hash function with v41
fn benchmark_call_argon2_v41(c: &mut Criterion) {
    let password = "a".repeat(1024);
    let salt = "hello-world-hello-world".to_owned();

    let params = [password, salt];

    benchmark_v41(c, "argon2", &params);
}

// Benchmark pulldown-cmark.wasm parse function with v41
fn benchmark_call_pulldown_cmark_v41(c: &mut Criterion) {
    let markdown = "Hello world, this is a ~~complicated~~ *very simple* example.".to_owned();

    let params = [markdown];

    benchmark_v41(c, "pulldown-cmark", &params);
}

/// Benchmark sevenz-7z.wasm zip function with v41
fn benchmark_call_sevenz_7z_zip_v41(c: &mut Criterion) {
    let req = "b".repeat(1024);

    let params = [req];

    benchmark_v41(c, "sevenz-7z", &params);
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
    benchmark_call_argon2_v41,
    benchmark_call_pulldown_cmark_v41,
    benchmark_call_sevenz_7z_zip_v41,
    // benchmark_call_sevenz_7z_unzip_v41
);
criterion_main!(benches);
