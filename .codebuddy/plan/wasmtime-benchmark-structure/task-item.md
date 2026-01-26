# 实施计划

- [ ] 1. 配置工作空间和创建基础目录结构
   - 修改根 Cargo.toml，配置 workspace 成员和共享依赖
   - 创建 crates/ 目录及其子目录结构（engine、testdata、wasmtime/v21、wasmtime/v41、benchmarks）
   - _需求：1.1、1.2、1.3、1.4、6.1、6.2、6.3、6.4、7.1_

- [ ] 2. 实现核心抽象层 Engine crate
   - 创建 engine/Cargo.toml，配置基础依赖（anyhow）
   - 实现 EngineApi trait，包含 new、load_wasm、execute、version 方法
   - 实现统一的错误类型 EngineError
   - _需求：2.1、2.2、2.3、2.4、2.5、2.6、2.7、7.1_

- [ ] 3. 实现测试数据模块 Testdata crate
   - 创建 testdata/Cargo.toml，配置基础依赖
   - 实现 load_fixture 函数用于加载 fixtures 目录下的 WASM 模块
   - 创建 testdata/src/fixtures/ 目录
   - _需求：3.1、3.2、3.3、3.4、7.2_

- [ ] 4. 实现 Wasmtime v21 版本
   - 创建 crates/wasmtime/v21/Cargo.toml，配置 wasmtime 21.0 依赖
   - 实现 WasmtimeV21 结构体并实现 EngineApi trait
   - 实现初始化引擎、加载 WASM 模块、执行函数和获取版本信息的具体逻辑
   - _需求：4.1、4.2、4.3、7.3_

- [ ] 5. 实现 Wasmtime v41 版本
   - 创建 crates/wasmtime/v41/Cargo.toml，配置 wasmtime 41.0 依赖
   - 实现 WasmtimeV41 结构体并实现 EngineApi trait
   - 实现初始化引擎、加载 WASM 模块、执行函数和获取版本信息的具体逻辑
   - _需求：4.4、4.5、4.6、7.3_

- [ ] 6. 实现基准测试模块 Benchmarks crate
   - 创建 benchmarks/Cargo.toml，配置 Criterion 框架和所有必要依赖
   - 实现 benchmark 函数，加载所有版本实现并执行性能测试
   - 配置 benchmark 输出格式，生成可读的性能对比报告
   - _需求：5.1、5.2、5.3、5.4、5.5、5.6、7.4_

- [ ] 7. 添加示例 WASM 测试数据
   - 编译或获取 simple.wasm 测试模块（基础算术运算）
   - 编译或获取 complex.wasm 测试模块（复杂计算场景）
   - 将 WASM 文件放置到 testdata/src/fixtures/ 目录
   - _需求：3.2、3.4、5.4_

- [ ] 8. 编写单元测试验证接口一致性
   - 为 engine crate 编写错误类型测试
   - 为 testdata crate 编写加载函数测试
   - 为 wasmtime-v21 和 wasmtime-v41 编写 EngineApi 实现测试
   - 编写集成测试验证所有版本能够正确加载和执行相同的测试数据
   - _需求：2.7、3.3、4.3、4.6、5.3、5.4、7.5_