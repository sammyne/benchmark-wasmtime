# 压测

## 快速开始

```bash
# 运行 benches/instantiate.rs 下名为 instantiate_argon2_v21 的测试
cargo bench --bench instantiate -- "instantiate_argon2_v21"

# 查看压测程序火焰图，确保目标代码没有被优化掉
cargo flamegraph --bench instantiate -- --bench instantiate_argon2_v21
```

## 温馨提示
在容器内借助 cargo-flamegraph + perf 生成火焰图需要为容器添加以下选项
```bash
--privileged \
--cap-add=ALL \
-v /lib/modules:/lib/modules \
-v /usr/src/kernels:/usr/src/kernels \
-v /sys/kernel/debug:/sys/kernel/debug \
```
