# 实施计划

- [ ] 1. 创建 GitHub Actions 工作流文件和基本结构
   - 在 `.github/workflows/` 目录下创建 `benchmark.yml` 文件
   - 定义工作流名称和触发条件（push 到 main 分支）
   - _需求：1.1_

- [ ] 2. 配置工作流权限和运行环境
   - 在工作流文件中配置 `contents: write` 权限以支持标签推送和 Release 创建
   - 设置运行器为 `ubuntu-latest`
   - _需求：6.1、6.2_

- [ ] 3. 设置 Rust 环境和依赖缓存
   - 添加 checkout 步骤获取代码
   - 配置 Rust toolchain（stable）
   - 设置 cargo 缓存以加速构建过程
   - _需求：6.3_

- [ ] 4. 实现基准测试执行步骤
   - 添加 cargo bench 执行步骤，使用 release 模式
   - 确保生成 criterion 格式的性能数据到 `target/criterion` 目录
   - _需求：2.1、2.2、2.3_

- [ ] 5. 实现日期和版本号提取逻辑
   - 使用 GitHub Actions 环境变量获取当前日期（格式 YYYYMMDD）
   - 使用 git 命令获取 commit 短哈希（格式 rev）
   - 将这些值保存为环境变量供后续步骤使用
   - _需求：3.4、4.2_

- [ ] 6. 实现报告生成步骤
   - 调用 `crates/benchmarks/collect_report.py` 脚本
   - 传入 `-d` 参数指定项目根目录
   - 传入 `-o` 参数指定输出文件路径为 `benchmark-{YYYYMMDD}-{rev}.md`
   - _需求：3.1、3.2、3.3、3.4_

- [ ] 7. 实现标签创建和推送步骤
   - 使用 git 命令创建格式为 `{YYYYMMDD}-{rev}` 的标签
   - 配置 git 用户信息以便提交标签
   - 推送标签到远程仓库
   - _需求：4.1、4.2、4.3_

- [ ] 8. 实现报告制品上传步骤
   - 使用 `gh release create` 命令创建 Release
   - 将 `benchmark-{YYYYMMDD}-{rev}.md` 文件作为制品上传
   - 设置制品名称为 `benchmark-{YYYYMMDD}-{rev}.md` 并关联到对应标签
   - _需求：5.1、5.2、5.3_

- [ ] 9. 添加 skip-ci 过滤逻辑
   - 在工作流开始处添加条件检查步骤
   - 检查提交消息是否包含 `[skip-ci]` 关键字
   - 如果包含则跳过后续步骤并终止工作流
   - _需求：1.3_
