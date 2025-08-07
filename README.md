# Free-Ollama

一个用于发现和扫描 Ollama 服务的简单工具

## 功能

- 扫描网络中的 Ollama 服务（截至2025-08-05 00:17:16，互联网所有 ollama 数据保存至 `ollama资产数据.csv` 中）
- 检测服务是否活跃
- 获取服务中的模型信息
- 将活跃的服务信息保存到 JSON 文件中
- **高性能扫描**: 能够在短时间内扫描大量目标（美国洛杉矶节点下默认配置下 ~60 秒内扫描 385,590 个目标并找到 2,786 个活跃服务）
![alt text](image.png)
## 使用方法

```bash
# 基本用法
cargo run -- -i ollama资产数据.csv

# 指定超时时间（秒）
cargo run -- -i ollama资产数据.csv -t 5
```

### 参数说明

- `-i, --input <FILE>`: 指定包含 Ollama 资产信息的 CSV 文件路径（必需）
- `-t, --timeout <SECS>`: 设置请求超时时间（秒），默认为 3 秒

## 输出

扫描结果将保存在 `results/` 目录中，文件名格式为 `ollama_scan_YYYYMMDD_HHMMSS.json`，仅包含活跃的服务。

## 依赖

- Rust 2021 edition
- 主要的 crates 包括:
  - reqwest: HTTP 客户端
  - tokio: 异步运行时
  - serde: 序列化/反序列化
  - clap: 命令行参数解析
  - csv: CSV 文件解析

## 注意

扫描结果与超时有关，仅用于评估目的，请勿用于非法用途

## 许可证

MIT License
