# AGENTS.md - Wasmtime Rust 开发指南

## 概述

本指南提供了使用 Wasmtime WebAssembly 运行时开发 Rust 应用程序的全面说明。Wasmtime 是一个独立的 JIT 风格 WebAssembly 运行时，能够在 Rust 应用程序中高效执行 WASM 模块。

## 目录

1. [入门指南](#getting-started)
2. [项目设置](#project-setup)
3. [常见模式](#common-patterns)
4. [性能优化](#performance-optimization)
5. [测试](#testing)
6. [错误处理](#error-handling)
7. [安全考虑](#security-considerations)

## 入门指南

### 前置要求

- Rust 1.70 或更高版本
- Cargo（随 Rust 一起安装）
- 基本的 WebAssembly 概念理解

### 安装

将 Wasmtime 添加到您的 `Cargo.toml`：

```toml
[dependencies]
wasmtime = "20.0"
wasmtime-wasi = "20.0"
anyhow = "1.0"
```

## 项目设置

### 基本项目结构

```
project/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── wasm/
│   │   └── example.wat
│   └── modules/
└── tests/
```

### Cargo.toml 示例

```toml
[package]
name = "wasmtime-app"
version = "0.1.0"
edition = "2021"

[dependencies]
wasmtime = { version = "20.0", features = ["component-model"] }
wasmtime-wasi = "20.0"
wasmtime-wasi-http = "20.0"
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "benchmark"
harness = false
```

## 常见模式

### 1. 插件系统

```rust
use wasmtime::*;
use std::collections::HashMap;
use std::path::Path;

struct PluginManager {
    engine: Engine,
    linker: Linker<()>,
    plugins: HashMap<String, Instance>,
}

impl PluginManager {
    fn new() -> Result<Self> {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        
        Ok(Self {
            engine,
            linker,
            plugins: HashMap::new(),
        })
    }
    
    fn load_plugin(&mut self, name: &str, path: &Path) -> Result<()> {
        let module = Module::from_file(&self.engine, path)?;
        let mut store = Store::new(&self.engine, ());
        let instance = self.linker.instantiate(&mut store, &module)?;
        
        self.plugins.insert(name.to_string(), instance);
        Ok(())
    }
    
    fn call_plugin(&mut self, plugin_name: &str, func_name: &str) -> Result<()> {
        let instance = self.plugins.get(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found"))?;
        
        let mut store = Store::new(&self.engine, ());
        let func = instance.get_func(&mut store, func_name)
            .ok_or_else(|| anyhow::anyhow!("Function not found"))?;
        
        func.call(&mut store, &[], &mut [])?;
        Ok(())
    }
}
```

### 2. 流式编译

```rust
use wasmtime::*;
use std::io::Read;

fn compile_from_stream<R: Read>(mut reader: R) -> Result<Module> {
    let engine = Engine::default();
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    
    let module = Module::new(&engine, &bytes)?;
    Ok(module)
}
```

### 3. 异步执行

```rust
use wasmtime::*;
use tokio::runtime::Runtime;

async fn async_wasm_execution(wasm_bytes: &[u8]) -> Result<i32> {
    let mut config = Config::new();
    config.async_support(true);
    config.epoch_interruption(true);
    
    let engine = Engine::new(&config)?;
    let module = Module::new(&engine, wasm_bytes)?;
    
    let mut store = Store::new(&engine, ());
    store.set_epoch_deadline(1);
    
    let instance = Instance::new(&mut store, &module, &[])?;
    let func = instance.get_typed_func::<(), i32>(&mut store, "async_func")?;
    
    let result = func.call_async(&mut store, ()).await?;
    Ok(result)
}
```

### 4. 多线程

```rust
use wasmtime::*;
use std::sync::Arc;
use std::thread;

fn parallel_execution(wasm_bytes: Vec<u8>, num_threads: usize) -> Result<Vec<i32>> {
    let engine = Arc::new(Engine::default());
    let module = Arc::new(Module::new(&engine, &wasm_bytes)?);
    
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            let engine = Arc::clone(&engine);
            let module = Arc::clone(&module);
            
            thread::spawn(move || {
                let mut store = Store::new(&engine, ());
                let instance = Instance::new(&mut store, &module, &[])?;
                let func = instance.get_typed_func::<i32, i32>(&mut store, "compute")?;
                func.call(&mut store, 42)
            })
        })
        .collect();
    
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap()?);
    }
    
    Ok(results)
}
```

## 性能优化

### 1. 缓存

```rust
use wasmtime::*;

fn with_cache() -> Result<()> {
    let mut config = Config::new();
    
    // Configure cache directory
    config.cache_config_load_default()?;
    
    let engine = Engine::new(&config)?;
    
    // First compilation (slower)
    let module1 = Module::from_file(&engine, "example.wasm")?;
    
    // Second compilation (faster, uses cache)
    let module2 = Module::from_file(&engine, "example.wasm")?;
    
    Ok(())
}
```

### 2. 优化级别

```rust
use wasmtime::*;

fn optimized_config() -> Result<Engine> {
    let mut config = Config::new();
    
    // Set optimization strategy
    config.strategy(Strategy::Cranelift);
    
    // Set optimization level
    config.cranelift_opt_level(OptLevel::Speed);
    
    // Enable target-specific optimizations
    config.cranelift_debug_verifier(false);
    
    Engine::new(&config)
}
```

### 3. 内存池

```rust
use wasmtime::*;

struct MemoryPool {
    engine: Engine,
    memory_type: MemoryType,
    pool: Vec<Memory>,
}

impl MemoryPool {
    fn new(engine: Engine, min_pages: u64, max_pages: u64) -> Self {
        let memory_type = MemoryType::new(min_pages, Some(max_pages), false);
        Self {
            engine,
            memory_type,
            pool: Vec::new(),
        }
    }
    
    fn acquire(&mut self) -> Result<Memory> {
        if let Some(memory) = self.pool.pop() {
            Ok(memory)
        } else {
            Ok(Memory::new(&self.engine, self.memory_type)?)
        }
    }
    
    fn release(&mut self, memory: Memory) {
        self.pool.push(memory);
    }
}
```

## 测试

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::*;

    #[test]
    fn test_basic_execution() -> Result<()> {
        let wasm = wat::parse_str(r#"
            (module
                (func (export "add") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add)
            )
        "#)?;
        
        let engine = Engine::default();
        let module = Module::new(&engine, &wasm)?;
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;
        
        let add = instance.get_typed_func::<i32, i32, i32>(&mut store, "add")?;
        let result = add.call(&mut store, 5, 3)?;
        
        assert_eq!(result, 8);
        Ok(())
    }

    #[test]
    fn test_fuel_consumption() -> Result<()> {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config)?;
        
        let wasm = wat::parse_str(r#"
            (module
                (func (export "infinite") (result i32)
                    (loop $l
                        i32.const 1
                        drop
                        br $l)
                    i32.const 0)
            )
        "#)?;
        
        let module = Module::new(&engine, &wasm)?;
        let mut store = Store::new(&engine, ());
        store.add_fuel(1000)?;
        let instance = Instance::new(&mut store, &module, &[])?;
        
        let func = instance.get_typed_func::<(), i32>(&mut store, "infinite")?;
        let result = func.call(&mut store, ());
        
        assert!(result.is_err());
        Ok(())
    }
}
```

### 基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_execution(c: &mut Criterion) {
    let engine = Engine::default();
    let wasm = std::fs::read("example.wasm").unwrap();
    let module = Module::new(&engine, &wasm).unwrap();
    
    c.bench_function("wasm_execution", |b| {
        b.iter(|| {
            let mut store = Store::new(&engine, ());
            let instance = Instance::new(&mut store, &module, &[]).unwrap();
            let func = instance.get_typed_func::<i32, i32>(&mut store, "compute").unwrap();
            black_box(func.call(&mut store, black_box(42)).unwrap())
        })
    });
}

criterion_group!(benches, benchmark_execution);
criterion_main!(benches);
```

## 错误处理

### 全面的错误类型

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmRuntimeError {
    #[error("Module compilation failed: {0}")]
    Compilation(#[from] wasmtime::CompileError),
    
    #[error("Instantiation failed: {0}")]
    Instantiation(#[from] wasmtime::InstantiationError),
    
    #[error("Function call failed: {0}")]
    Call(#[from] wasmtime::Trap),
    
    #[error("Resource limit exceeded: fuel={0}")]
    FuelExhausted(u64),
    
    #[error("Memory access violation: offset={offset}, length={length}, size={size}")]
    MemoryAccess {
        offset: usize,
        length: usize,
        size: usize,
    },
    
    #[error("Invalid export: expected '{expected}', got '{found}'")]
    InvalidExport { expected: String, found: String },
}

pub type Result<T> = std::result::Result<T, WasmRuntimeError>;
```

### 错误恢复

```rust
fn resilient_execution(wasm_bytes: &[u8], max_retries: usize) -> Result<i32> {
    let engine = Engine::default();
    
    for attempt in 0..max_retries {
        match attempt_execution(&engine, wasm_bytes) {
            Ok(result) => return Ok(result),
            Err(WasmRuntimeError::FuelExhausted(_)) => {
                // Retry with more fuel
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(WasmRuntimeError::Call(wasmtime::Trap::new("Max retries exceeded")))
}

fn attempt_execution(engine: &Engine, wasm_bytes: &[u8]) -> Result<i32> {
    let module = Module::new(engine, wasm_bytes)?;
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let func = instance.get_typed_func::<(), i32>(&mut store, "main")?;
    func.call(&mut store, ())
}
```

## 安全考虑

### 1. 资源限制

```rust
use wasmtime::*;

fn secure_config() -> Result<Config> {
    let mut config = Config::new();
    
    // Enable fuel
    config.consume_fuel(true);
    
    // Limit stack size
    config.max_wasm_stack(512 * 1024); // 512KB
    
    // Limit memory growth
    config.limit_memory_growth(true);
    
    // Set epoch interruption for timeout handling
    config.epoch_interruption(true);
    
    Ok(config)
}
```

### 2. 输入验证

```rust
fn validate_wasm_module(wasm_bytes: &[u8]) -> Result<()> {
    // Check size limit
    const MAX_MODULE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    
    if wasm_bytes.len() > MAX_MODULE_SIZE {
        return Err(anyhow::anyhow!("Module too large"));
    }
    
    // Validate WASM header
    if !wasm_bytes.starts_with(&[0x00, 0x61, 0x73, 0x6D]) {
        return Err(anyhow::anyhow!("Invalid WASM magic number"));
    }
    
    // Additional validation...
    
    Ok(())
}
```

### 3. 沙箱

```rust
fn sandboxed_execution(wasm_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    
    // Disable potentially dangerous features
    config.wasm_simd(false);
    config.wasm_multi_memory(false);
    config.wasm_multi_value(false);
    
    // Use WASI preview1 with restricted capabilities
    let mut wasi = wasmtime_wasi::WasiCtxBuilder::new();
    // Only allow specific directories
    wasi.preopened_dir(
        std::path::PathBuf::from("/sandbox"),
        std::path::PathBuf::from("/sandbox"),
    )?;
    
    let engine = Engine::new(&config)?;
    let module = Module::new(&engine, wasm_bytes)?;
    
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |s| s)?;
    
    let mut store = Store::new(&engine, wasi.build());
    let _instance = linker.instantiate(&mut store, &module)?;
    
    Ok(())
}
```

## 其他资源

### 文档

- [Wasmtime 文档](https://docs.wasmtime.dev/)
- [WebAssembly 规范](https://webassembly.github.io/spec/)
- [WASI 规范](https://wasi.dev/)

### 社区

- [Wasmtime GitHub](https://github.com/bytecodealliance/wasmtime)
- [Bytecode Alliance Discord](https://bytecodealliance.org/discord)

### 相关项目

- `wasm-tools`: WebAssembly 工具包
- `wit-bindgen`: 为组件模型生成绑定
- `wasi-test-runner`: WASI 应用程序的测试运行器

## 贡献指南

在为本代码库做贡献时：

1. 遵循 Rust 命名约定和风格指南
2. 为新功能编写全面的测试
3. 记录所有公共 API
4. 确保所有代码通过 `cargo clippy`
5. 提交前运行 `cargo fmt`
6. 为复杂功能添加示例

## 许可证

[在此处添加您的许可证信息]

---

*最后更新：2026-01-23*
