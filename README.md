# Benchmark wasmtime

## 数据快照

### 系统信息
- **CPU 型号**: AMD EPYC 7K62 48-Core Processor
- **CPU 核心数**: 32
- **内存大小**: 62.4 GB
- **操作系统**: Linux 4.18.0-193.6.3.el8_2.x86_64

### 测试数据
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| call_async_argon2_hash_v21 | 44705.3056 | 44748.8270 | 44795.0446 | µs |
| call_async_argon2_hash_v41 | 44751.4757 | 44792.3973 | 44833.5687 | µs |
| call_async_pulldown-cmark_parse_v21 | 10.1951 | 10.2028 | 10.2101 | µs |
| call_async_pulldown-cmark_parse_v41 | 4.4357 | 4.4413 | 4.4477 | µs |
| call_async_sevenz-7z_zip_v21 | 12165.1547 | 12190.5279 | 12220.6083 | µs |
| call_async_sevenz-7z_zip_v41 | 12016.3611 | 12030.1360 | 12044.7741 | µs |
| instantiate_async_argon2_v21 | 16.6470 | 16.6732 | 16.7019 | µs |
| instantiate_async_argon2_v41 | 11.7473 | 11.7560 | 11.7660 | µs |
| instantiate_async_pulldown-cmark_v21 | 16.7213 | 16.7333 | 16.7454 | µs |
| instantiate_async_pulldown-cmark_v41 | 12.0052 | 12.0667 | 12.1367 | µs |
| instantiate_async_sevenz-7z_v21 | 14.9197 | 14.9371 | 14.9559 | µs |
| instantiate_async_sevenz-7z_v41 | 11.8551 | 11.8742 | 11.9023 | µs |

## 温馨提示
1. wasmtime v21 没有模块专用的 `Linker`；

## TODO
- 排查为什么以下配置对 golden 下的 crate 无效。`cargo build -r -v --target=wasm32-wasip2` 显示日志级别是 `3`
    ```toml
    [profile.bench]
    opt-level = "s"
    ```