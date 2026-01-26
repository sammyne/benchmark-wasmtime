# AGENTS.md - Wasmtime Rust 压测指南

## 概述

本指南提供了使用 Wasmtime WebAssembly 运行时开发 Rust 应用程序的全面说明。Wasmtime 是一个独立的 JIT 风格 WebAssembly 运行时，能够在 Rust 应用程序中高效执行 WASM 模块。

**注意**：本指南明确仅支持 Wasmtime 的以下两个版本：
- **v41.0.0**（最新稳定版）
- **v21.0.1**（早期稳定版）

## 目录

1. [入门指南](#入门指南)
2. [错误处理](#错误处理)
3. [其他资源](#其他资源)
4. [贡献指南](#贡献指南)
5. [许可证](#许可证)

## 入门指南

### 前置要求

- Rust 1.92.0
- Cargo（随 Rust 一起安装）
- 基本的 WebAssembly 概念理解
- criterion >= v0.8.1（用于性能基准测试）

### 安装

将 Wasmtime 添加到您的 `Cargo.toml`（根据您需要的版本选择）：

**使用 v41.0.0（推荐）：**

```toml
[dependencies]
wasmtime = "41.0"
wasmtime-wasi = "41.0"
anyhow = "1.0"
```

**使用 v21.0.1：**

```toml
[dependencies]
wasmtime = "21.0"
wasmtime-wasi = "21.0"
anyhow = "1.0"
```

## 错误处理

本指南推荐使用 `anyhow` 库进行错误处理，它提供了简单而强大的错误处理机制。

### anyhow 基础用法

`anyhow` 是一个通用的错误处理库，专门用于应用代码而非库代码。它提供了以下特性：

- 简单的错误类型转换：`anyhow::Result<T>`
- 上下文信息附加：`context()` 和 `with_context()` 方法
- 错误链追踪：自动保留底层错误
- 转换为 `Box<dyn Error>`：通过 `?` 运算符

### 基本示例

```rust
use anyhow::{Context, Result, bail, anyhow};
use wasmtime::{Engine, Module, Store};

fn load_wasm_module(engine: &Engine, wasm_bytes: &[u8]) -> Result<Module> {
    Module::from_binary(engine, wasm_bytes)
        .context("无法解析 WebAssembly 模块")
}

fn run_wasm(wasm_bytes: &[u8]) -> Result<i32> {
    let engine = Engine::default();
    
    let module = load_wasm_module(&engine, wasm_bytes)?;
    
    let mut store = Store::new(&engine, ());
    let instance = module.instantiate(&mut store, [])
        .with_context(|| format!("无法实例化模块：{}", module.name()))?;
    
    // 获取并调用函数
    let func = instance.get_typed_func::<(), i32>(&mut store, "main")
        .context("无法找到 'main' 函数")?;
    
    let result = func.call(&mut store, ())
        .context("函数调用失败")?;
    
    Ok(result)
}
```

### 错误类型转换

```rust
use anyhow::Result;

fn parse_config(config_str: &str) -> Result<serde_json::Value> {
    let config: serde_json::Value = serde_json::from_str(config_str)
        .map_err(|e| anyhow!("JSON 解析失败：{}", e))?;
    
    Ok(config)
}
```

### 使用 bail! 宏快速返回错误

```rust
use anyhow::{Result, bail};

fn validate_wasm(wasm_bytes: &[u8]) -> Result<()> {
    if wasm_bytes.is_empty() {
        bail!("WASM 模块为空");
    }
    
    if !wasm_bytes.starts_with(&[0x00, 0x61, 0x73, 0x6D]) {
        bail!("无效的 WASM 魔术数字");
    }
    
    Ok(())
}
```

### 错误链与上下文信息

```rust
use anyhow::{Context, Result};

fn complex_operation(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("无法读取文件：{}", path))
        .and_then(|content| {
            // 处理内容
            if content.is_empty() {
                anyhow::bail!("文件内容为空：{}", path);
            }
            Ok(content)
        })
}
```

### Wasmtime 特定错误处理

```rust
use anyhow::{Context, Result};
use wasmtime::{Engine, Store, Instance};

fn safe_instantiate(module: &Module, store: &mut Store<()>) -> Result<Instance> {
    module.instantiate(store, [])
        .context("实例化失败")
        .map_err(|e| {
            // 转换 Wasmtime 特定错误并添加更多上下文
            anyhow::anyhow!("WASM 实例化错误: {}", e)
        })
}
```

### 错误打印和调试

```rust
fn main() -> anyhow::Result<()> {
    if let Err(e) = run_wasm(&[0x00, 0x61, 0x73, 0x6D]) {
        // 打印错误链
        eprintln!("错误：{:?}", e);
        
        // 打印错误链的每一层
        for cause in e.chain() {
            eprintln!("原因：{}", cause);
        }
        
        // 返回错误
        return Err(e);
    }
    
    Ok(())
}
```

## 测试实践

### 单元测试

测试代码和源代码应分离在不同模块。对于模块 `hello`，目录结构如下：

```
|-hello
  |-mod.rs    // 源代码
  |-tests.rs  // 测试代码
```

**源代码文件 `hello/mod.rs`：**

```rust
// hello/mod.rs

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn calculate_score(value: u32) -> Result<u32, String> {
    if value == 0 {
        Err("value cannot be zero".to_string())
    } else {
        Ok(value * 2)
    }
}

// 在源文件末尾引入测试模块
#[cfg(test)]
mod tests;  // 引入同目录下的 tests.rs
```

**测试代码文件 `hello/tests.rs`：**

```rust
// hello/tests.rs

use super::*;

#[test]
fn greet() {
    assert_eq!(greet("World"), "Hello, World!");
    assert_eq!(greet("Rust"), "Hello, Rust!");
}

#[test]
fn calculate_score_success() {
    assert_eq!(calculate_score(5).unwrap(), 10);
    assert_eq!(calculate_score(100).unwrap(), 200);
}

#[test]
fn calculate_score_error() {
    assert!(calculate_score(0).is_err());
    assert_eq!(calculate_score(0).unwrap_err(), "value cannot be zero");
}
```

**模块导出文件 `src/modules/mod.rs`：**

```rust
pub mod hello;
```

### 集成测试

在 `tests/` 目录中创建文件：

```rust
// tests/integration_test.rs
use my_library::process_data;

#[test]
fn public_api() {
    assert_eq!(process_data("test").unwrap(), "TEST");
}
```

### 测试组织最佳实践

1. **测试命名**: 使用描述性名称，直接描述被测试的功能或场景，不需要 `test_` 前缀
   - 示例: `greet()`、`calculate_score_success()`、`empty_input_handling()`
2. **模块分离**: 测试代码应与源代码分离到独立的 `tests.rs` 文件
3. **目录结构**: 使用子目录组织模块，源代码和测试代码在同一目录下
4. **Arrange-Act-Assert 模式**: 清晰组织测试结构
5. **测试边界情况**: 包括空输入、边界值和错误条件
6. **使用 `assert!` 和 `assert_eq!`**: 选择合适的断言
7. **测试错误变体**: 验证所有错误分支正确工作

**完整的示例目录结构：**

```
src/
├── lib.rs
├── error.rs
└── modules/
    ├── mod.rs           # 模块导出
    ├── hello/
    │   ├── mod.rs       # hello 模块源代码
    │   └── tests.rs     # hello 模块测试代码
    └── calculator/
        ├── mod.rs       # calculator 模块源代码
        └── tests.rs     # calculator 模块测试代码
```

**测试代码示例 `hello/tests.rs`：**

```rust
use super::*;

#[test]
fn calculate_score_handles_zero_input() {
    // Arrange
    let zero_input = 0;

    // Act
    let result = calculate_score(zero_input);

    // Assert
    assert!(matches!(result, Err(ref msg) if msg.contains("zero")));
}
```

**这种结构的优势：**
- 源代码和测试代码完全分离，便于维护
- 测试文件专注于测试逻辑，不干扰源代码阅读
- 保持模块的组织清晰
- 遵循 Rust 项目的标准组织方式

### 最佳实践

1. **在应用代码中使用 anyhow**，在库代码中定义自定义错误类型
2. **使用 `context()` 添加上下文信息**，帮助调试
3. **使用 `bail!` 宏快速返回错误**，简化代码
4. **保留错误链**，不要吞掉底层错误
5. **为用户友好的错误消息**，提供清晰的错误描述

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
