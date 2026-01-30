# 需求文档

## 引言
本文档定义了为 benchmark-wasmtime 项目配置 GitHub Actions 自动化流水线的需求。该流水线将在每次推送到 main 分支时自动运行基准测试，收集测试结果，生成报告，并将报告作为版本制品保存，便于历史性能数据的追踪和对比。

## 需求

### 需求 1：触发机制

**用户故事：** 作为一名开发者，我希望每次推送到 main 分支时自动触发基准测试流水线，以便持续监控项目性能变化。

#### 验收标准

1. WHEN 代码被推送到 `main` 分支 THEN 系统 SHALL 自动触发 GitHub Actions 工作流
2. IF 推送到其他分支 THEN 系统 SHALL 不触发该工作流
3. IF 提交消息包含 `[skip-ci]` 关键字 THEN 系统 SHALL 跳过工作流的执行

### 需求 2：运行基准测试

**用户故事：** 作为一名开发者，我希望流水线能够自动运行 `crates/benchmarks` 下的所有基准测试，以便获得准确的性能数据。

#### 验收标准

1. WHEN 工作流被触发 THEN 系统 SHALL 在工作目录中执行 `cargo bench --workspace`
2. WHEN 执行基准测试时 THEN 系统 SHALL 确保使用 release 优化级别进行编译
3. WHEN 基准测试完成 THEN 系统 SHALL 生成 criterion 格式的性能数据到 `target/criterion` 目录

### 需求 3：收集并生成测试报告

**用户故事：** 作为一名开发者，我希望自动调用 `collect_report.py` 脚本整理压测数据并生成 Markdown 报告，以便查看格式化的测试结果。

#### 验收标准

1. WHEN 基准测试完成后 THEN 系统 SHALL 调用 `crates/benchmarks/collect_report.py` 脚本
2. WHEN 调用脚本时 THEN 系统 SHALL 传入 `-d` 参数指定项目根目录
3. WHEN 调用脚本时 THEN 系统 SHALL 传入 `-o` 参数指定输出文件路径为 `benchmark-{YYYYMMDD}-{rev}.md`
4. WHEN 生成报告文件 THEN 系统 SHALL 使用格式：`benchmark-{YYYYMMDD}-{rev}.md`，其中 `{YYYYMMDD}` 为当前日期，`{rev}` 为 commit 的短哈希

### 需求 4：创建版本标签

**用户故事：** 作为一名开发者，我希望自动创建格式为 `{YYYYMMDD}-{rev}` 的 Git 标签，以便为每次基准测试建立版本标记。

#### 验收标准

1. WHEN 报告文件生成成功 THEN 系统 SHALL 创建格式为 `{YYYYMMDD}-{rev}` 的 Git 标签
2. WHEN 创建标签时 THEN 系统 SHALL 使用与报告文件相同的日期和版本号
3. WHEN 创建标签时 THEN 系统 SHALL 将标签推送到远程仓库

### 需求 5：上传报告制品

**用户故事：** 作为一名开发者，我希望将生成的 Markdown 报告上传为标签的制品，以便在 GitHub 上方便地查看和下载历史报告。

#### 验收标准

1. WHEN 标签创建成功 THEN 系统 SHALL 将 `benchmark-{YYYYMMDD}-{rev}.md` 文件上传为 GitHub Release 制品
2. WHEN 上传制品时 THEN 系统 SHALL 将制品关联到对应的 `{YYYYMMDD}-{rev}` 标签
3. WHEN 上传制品时 THEN 系统 SHALL 设置制品名称为 `benchmark-{YYYYMMDD}-{rev}.md`

### 需求 6：环境配置

**用户故事：** 作为一名开发者，我希望流水线具备适当的权限和运行环境，以便能够执行所有必要的操作。

#### 验收标准

1. WHEN 工作流定义时 THEN 系统 SHALL 配置 `contents: write` 权限以允许推送标签和创建 Release
2. WHEN 运行基准测试时 THEN 系统 SHALL 使用支持 cargo 的运行环境（如 ubuntu-latest）
3. WHEN 安装依赖时 THEN 系统 SHALL 配置合理的 Rust 缓存以加快构建速度
