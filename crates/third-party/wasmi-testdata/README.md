# Wasmi 测试数据

本目录包含从 [wasmi-labs/wasmi-benchmarks](https://github.com/wasmi-labs/wasmi-benchmarks) 仓库下载的测试用 wasm 文件。

## 下载信息

- **仓库**: wasmi-labs/wasmi-benchmarks
- **提交版本**: b9385cae9bfb8cf84dbb13996d0b948ca5826b53
- **下载脚本**: [get.sh](./get.sh)

## 文件列表

| 文件名 | 大小 | 说明 |
|--------|------|------|
| argon2.wasm | 60K | Argon2 密码哈希算法，用于密码哈希和密钥派生 |
| bz2.wasm | 148K | bzip2 压缩算法，高压缩率的文件压缩格式 |
| coremark-minimal.wasm | 7.6K | CoreMark 基准测试（精简版），评估嵌入式系统性能 |
| erc20.wasm | 7.0K | ERC20 代币标准，以太坊区块链上广泛使用的代币合约 |
| ffmpeg.wasm | 19M | FFmpeg 视频处理库，多媒体编解码和转码工具 |
| pulldown-cmark.wasm | 1.6M | Markdown 解析器，符合 CommonMark 规范的 Markdown 到 HTML 转换器 |
| spidermonkey.wasm | 4.1M | SpiderMonkey JavaScript 引擎，Mozilla 开发的 JS 引擎 |

## 使用说明

运行 [get.sh](./get.sh) 脚本可以重新下载或更新所有 wasm 文件：

```bash
./get.sh
```

这些文件涵盖了不同类型和规模的工作负载，从轻量级（几 KB）到大型应用（几十 MB），适合用于全面的基准测试。
