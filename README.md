# wasmtime 基准测试

## 测试用例简介

测试用的 WASM 源码集中在 crates/golden 目录，分别如下

用例 | 大小 | 类型 | 备注
----|------|----------
argon2 | 168K | 计算密集型 | 密码哈希库
host-caller | 15K | 系统调用密集型 | echo 接口调用 1000 次系统调用
pulldown-cmark | 346K | 计算密集型 | Markdown 解析库
rust-python | 46M | | Python 解析器
sevenz-7z | 470K | 计算密集型 | 压缩算法库

## 测试场景
- **实例化**：基于预实例化的句柄，调用 `instantiate/instantiate_async` 接口实例化为可用的组件实例；
- **函数调用**：
    1. 基于预实例化的句柄，调用 `instantiate/instantiate_async` 接口实例化为可用的组件实例；
    2. 从组件实例找到待调用的函数（时耗可忽略不计）；
    3. 使用 `call/call_async` 接口调用函数；

## 系统信息
- **CPU 型号**: AMD EPYC 7K62 48-Core Processor
- **CPU 核心数**: 32
- **内存大小**: 62.4 GB
- **操作系统**: Linux 4.18.0-193.6.3.el8_2.x86_64

## 结果
时间单位：微妙

### 总览
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| call_async_argon2_hash_v21 | 45442.2568 | 45476.7513 | 45514.4905 | µs |
| call_async_argon2_hash_v41 | 44561.9294 | 44597.2820 | 44635.6225 | µs |
| call_async_pulldown-cmark_parse_v21 | 10.3849 | 10.3935 | 10.4023 | µs |
| call_async_pulldown-cmark_parse_v41 | 4.4671 | 4.4843 | 4.5025 | µs |
| call_async_sevenz-7z_zip_v21 | 12160.9000 | 12176.2773 | 12193.1500 | µs |
| call_async_sevenz-7z_zip_v41 | 12178.8173 | 12203.7594 | 12233.5154 | µs |
| call_async_with_host_host-caller_echo_v21 | 131.9644 | 132.1302 | 132.3071 | µs |
| call_async_with_host_host-caller_echo_v41 | 126.7450 | 127.5717 | 128.4781 | µs |
| call_async_with_pooling_alloc_host_host-caller_echo_v21 | 105.4124 | 105.5411 | 105.6990 | µs |
| call_async_with_pooling_alloc_host_host-caller_echo_v41 | 111.3520 | 111.4221 | 111.5018 | µs |
| call_with_pooling_alloc_host_host-caller_echo_v21 | 1.2245 | 1.2279 | 1.2340 | ns |
| call_with_pooling_alloc_host_host-caller_echo_v41 | 1.2240 | 1.2244 | 1.2248 | ns |
| instantiate_argon2_v21 | 9.4837 | 9.5289 | 9.5968 | µs |
| instantiate_argon2_v41 | 10.3037 | 10.3114 | 10.3193 | µs |
| instantiate_async_argon2_v21 | 17.4831 | 17.5104 | 17.5449 | µs |
| instantiate_async_argon2_v41 | 12.3469 | 12.3578 | 12.3698 | µs |
| instantiate_async_pulldown-cmark_v21 | 17.5042 | 17.6664 | 17.8367 | µs |
| instantiate_async_pulldown-cmark_v41 | 12.5935 | 12.6179 | 12.6464 | µs |
| instantiate_async_rust-python_v21 | 33.0584 | 33.1011 | 33.1441 | µs |
| instantiate_async_rust-python_v41 | 35.1839 | 35.2218 | 35.2631 | µs |
| instantiate_async_sevenz-7z_v21 | 15.7708 | 15.7981 | 15.8278 | µs |
| instantiate_async_sevenz-7z_v41 | 12.4779 | 12.4963 | 12.5150 | µs |
| instantiate_async_with_host_host-caller_v21 | 57.9211 | 57.9493 | 57.9805 | µs |
| instantiate_async_with_host_host-caller_v41 | 8.8539 | 8.8622 | 8.8706 | µs |
| instantiate_async_with_pooling_alloc_host_host-caller_v21 | 6.2016 | 6.2115 | 6.2241 | µs |
| instantiate_async_with_pooling_alloc_host_host-caller_v41 | 6.7560 | 6.7622 | 6.7679 | µs |
| instantiate_pulldown-cmark_v21 | 9.9066 | 9.9166 | 9.9277 | µs |
| instantiate_pulldown-cmark_v41 | 10.5895 | 10.5987 | 10.6087 | µs |
| instantiate_sevenz-7z_v21 | 9.8499 | 9.8613 | 9.8737 | µs |
| instantiate_sevenz-7z_v41 | 10.5057 | 10.5141 | 10.5226 | µs |

### v21 实例化
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| argon2_v21 | 9.4837 | 9.5289 | 9.5968 | µs |
| argon2_v21-async | 17.4831 | 17.5104 | 17.5449 | µs |
| pulldown-cmark_v21 | 9.9066 | 9.9166 | 9.9277 | µs |
| pulldown-cmark_v21-async | 17.5042 | 17.6664 | 17.8367 | µs |
| rust-python_v21-async | 33.0584 | 33.1011 | 33.1441 | µs |
| sevenz-7z_v21 | 9.8499 | 9.8613 | 9.8737 | µs |
| sevenz-7z_v21-async | 15.7708 | 15.7981 | 15.8278 | µs |
| with_host_host-caller_v21-async | 57.9211 | 57.9493 | 57.9805 | µs |
| with_pooling_alloc_host_host-caller_v21-async | 6.2016 | 6.2115 | 6.2241 | µs |

### v21 函数调用
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| argon2_hash_v21-async | 45442.2568 | 45476.7513 | 45514.4905 | µs |
| pulldown-cmark_parse_v21-async | 10.3849 | 10.3935 | 10.4023 | µs |
| sevenz-7z_zip_v21-async | 12160.9000 | 12176.2773 | 12193.1500 | µs |
| with_host_host-caller_echo_v21-async | 131.9644 | 132.1302 | 132.3071 | µs |
| with_pooling_alloc_host_host-caller_echo_v21 | 1.2245 | 1.2279 | 1.2340 | ns |
| with_pooling_alloc_host_host-caller_echo_v21-async | 105.4124 | 105.5411 | 105.6990 | µs |

### v41 实例化
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| argon2_v41 | 10.3037 | 10.3114 | 10.3193 | µs |
| argon2_v41-async | 12.3469 | 12.3578 | 12.3698 | µs |
| pulldown-cmark_v41 | 10.5895 | 10.5987 | 10.6087 | µs |
| pulldown-cmark_v41-async | 12.5935 | 12.6179 | 12.6464 | µs |
| rust-python_v41-async | 35.1839 | 35.2218 | 35.2631 | µs |
| sevenz-7z_v41 | 10.5057 | 10.5141 | 10.5226 | µs |
| sevenz-7z_v41-async | 12.4779 | 12.4963 | 12.5150 | µs |
| with_host_host-caller_v41-async | 8.8539 | 8.8622 | 8.8706 | µs |
| with_pooling_alloc_host_host-caller_v41-async | 6.7560 | 6.7622 | 6.7679 | µs |

### v41 函数调用
| 基准测试名称 | 下限 | 平均值 | 上限 | 单位 |
|--------------|------|--------|------|------|
| argon2_hash_v41-async | 44561.9294 | 44597.2820 | 44635.6225 | µs |
| pulldown-cmark_parse_v41-async | 4.4671 | 4.4843 | 4.5025 | µs |
| sevenz-7z_zip_v41-async | 12178.8173 | 12203.7594 | 12233.5154 | µs |
| with_host_host-caller_echo_v41-async | 126.7450 | 127.5717 | 128.4781 | µs |
| with_pooling_alloc_host_host-caller_echo_v41 | 1.2240 | 1.2244 | 1.2248 | ns |
| with_pooling_alloc_host_host-caller_echo_v41-async | 111.3520 | 111.4221 | 111.5018 | µs |

### v21 vs v41 对比

#### 实例化对比
| 基准测试名称 | v21 平均值 | v41 平均值 |
|--------------|-----------|----------|
| argon2 | 9.5289 | 10.3114 |
| argon2-async | 17.5104 | 12.3578 |
| pulldown-cmark | 9.9166 | 10.5987 |
| pulldown-cmark-async | 17.6664 | 12.6179 |
| rust-python-async | 33.1011 | 35.2218 |
| sevenz-7z | 9.8613 | 10.5141 |
| sevenz-7z-async | 15.7981 | 12.4963 |
| with_host_host-caller-async | 57.9493 | 8.8622 |
| with_pooling_alloc_host_host-caller-async | 6.2115 | 6.7622 |

#### 函数调用对比
| 基准测试名称 | v21 平均值 | v41 平均值 |
|--------------|-----------|----------|
| argon2_hash-async | 45476.7513 | 44597.2820 |
| pulldown-cmark_parse-async | 10.3935 | 4.4843 |
| sevenz-7z_zip-async | 12176.2773 | 12203.7594 |
| with_host_host-caller_echo-async | 132.1302 | 127.5717 |
| with_pooling_alloc_host_host-caller_echo | 0.0012 | 0.0012 |
| with_pooling_alloc_host_host-caller_echo-async | 105.5411 | 111.4221 |


## 温馨提示
1. wasmtime v21 没有模块专用的 `Linker`；


## TODO
- 排查为什么以下配置对 golden 下的 crate 无效。`cargo build -r -v --target=wasm32-wasip2` 显示日志级别是 `3`
    ```toml
    [profile.bench]
    opt-level = "s"
    ```