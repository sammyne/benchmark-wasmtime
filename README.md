# WASM benchmark

## 温馨提示
1. wasmtime v21 没有模块专用的 `Linker`；

## TODO
- 排查为什么以下配置对 golden 下的 crate 无效。`cargo build -r -v --target=wasm32-wasip2` 显示日志级别是 `3`
    ```toml
    [profile.bench]
    opt-level = "s"
    ```