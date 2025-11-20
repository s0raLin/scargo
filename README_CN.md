# Sinter

一个类似 Cargo 的 Scala 项目构建工具。

📖 **文档**: [English](README.md) | [中文](README_CN.md)

## 功能特性

- 使用 `sinter new <name>` 初始化项目
- 使用 `sinter build` 构建 Scala 项目
- 使用 `sinter run` 运行 Scala 应用程序
- 使用 `sinter add <dep>` 添加依赖
- 通过 `project.toml` 配置项目设置

## 快速开始

```bash
# 创建新项目
sinter new hello-scala
cd hello-scala

# 添加依赖
sinter add cats

# 构建并运行
sinter build
sinter run
```

## 安装

### 从源码安装

```bash
git clone https://github.com/yourusername/sinter.git
cd sinter
cargo build --release
# 将 target/release/sinter 添加到 PATH
```

### 前置要求

- Rust（最新稳定版）
- Scala CLI（用于 Scala 编译和执行）

## 使用方法

### 获取帮助

```bash
sinter --help          # 显示所有命令
sinter [command] --help # 显示特定命令的帮助
```

### 创建新项目

```bash
sinter new my-scala-project
cd my-scala-project
```

这将创建一个具有以下结构的 Scala 项目：
```
my-scala-project/
├── project.toml          # 项目配置
└── src/main/scala/
    └── Main.scala       # 主应用程序文件
```

### 构建项目

```bash
sinter build
```

编译 `src/main/scala` 中的所有 Scala 源代码，并将编译后的类放在 `project.toml` 中指定的 `target_dir`。

### 运行项目

```bash
sinter run
sinter run path/to/MyFile.scala
sinter run --lib
```

- 无参数：运行 `project.toml` 中指定的主文件
- 指定文件路径：运行指定的 Scala 文件
- `--lib`：强制库模式（仅编译，不执行）

### 添加依赖

```bash
sinter add cats
sinter add org.typelevel::cats-core_2.13:2.10.0
sinter add cats@2.13:2.10.0
```

依赖格式：`group::artifact[@scala-version][:version]`

- `cats`：为项目 Scala 版本添加最新版本的 cats-core
- `org.typelevel::cats-core_2.13:2.10.0`：完整规范，包括组、制品、Scala 版本和版本
- `cats@2.13:2.10.0`：简短形式，包含 Scala 版本和版本

## 配置

项目配置存储在 `project.toml` 中：

```toml
[package]
name = "my-project"
version = "0.1.0"
main = "Main"
scala_version = "2.13"
source_dir = "src/main/scala"
target_dir = "target"

[dependencies]
"org.typelevel::cats-core_2.13" = "2.10.0"
```

## 故障排除

### 常见问题

- **找不到 Scala CLI**：确保 Scala CLI 已安装并在 PATH 中可用
- **构建失败**：检查 `project.toml` 中的所有依赖是否正确指定
- **运行失败**：确保主文件具有正确的入口点（extends App 或具有 main 方法）

### 获取更多帮助

- 运行 `sinter --help` 获取命令概览
- 查看 [Scala CLI 文档](https://scala-cli.virtuslab.org/) 了解 Scala 相关问题
- 在 [GitHub 仓库](https://github.com/yourusername/sinter) 上报告问题

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。