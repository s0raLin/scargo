# Scargo 架构文档

## 项目结构

```
scargo/
├── Cargo.toml              # Workspace 配置
├── src/                     # 主项目库代码
│   ├── core/               # 核心模块
│   │   ├── handler.rs      # CommandHandler trait
│   │   ├── app.rs          # Scargo 应用构建器
│   │   └── mod.rs
│   ├── runtime/            # 运行时模块
│   │   ├── executor.rs     # 命令执行器
│   │   └── mod.rs
│   ├── cli.rs              # CLI 定义和解析
│   ├── config.rs           # 配置管理
│   ├── build/              # 构建相关
│   ├── deps/               # 依赖管理
│   ├── cmd/                # 内置命令实现
│   │   ├── builtin.rs      # 内置命令执行逻辑
│   │   └── ...
│   └── lib.rs              # 公共 API
├── cli/                    # 可执行文件（独立 crate）
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # 主入口，注册插件
├── scargo-plugin/          # 插件开发 API（独立 crate）
│   ├── Cargo.toml
│   ├── README.md
│   └── src/lib.rs          # 插件开发 API
└── plugins/                # 插件集合（独立 crate）
    ├── Cargo.toml
    ├── README.md
    └── src/
        ├── lib.rs          # 导出所有插件
        └── jsp.rs          # JSP 插件示例
```

## 模块说明

### 核心模块 (core)

核心模块包含插件系统的核心抽象：

- **CommandHandler trait**: 所有命令（内置和插件）都需要实现的 trait
- **Scargo 结构体**: 应用构建器，使用 Builder 模式

### 运行时模块 (runtime)

运行时模块处理命令的执行：

- **Executor**: 负责将解析后的命令分发到对应的处理器

### 插件系统

插件系统采用独立 crate 设计：

- **plugins**: 独立的插件 crate，包含所有扩展插件
- **cli**: 独立的可执行文件 crate，负责注册和运行插件
- **scargo-plugin**: 插件开发 API crate，提供插件开发所需的所有依赖和类型

这种设计的优势：
1. **避免循环依赖**：主项目库不依赖插件，插件依赖主项目
2. **清晰的分离**：核心功能和扩展功能完全分离
3. **易于维护**：插件可以独立开发和发布
4. **灵活配置**：可以选择性地包含插件

## 依赖关系

```
scargo (lib)
  └─ 不依赖任何插件

scargo-plugin
  └─ 依赖 scargo (lib)

plugins
  └─ 依赖 scargo-plugin

cli
  ├─ 依赖 scargo (lib)
  └─ 依赖 plugins
```

## 设计原则

1. **清晰的模块分层**：
   - core: 核心抽象
   - runtime: 运行时逻辑
   - cmd: 具体命令实现

2. **统一的命令接口**：
   - 所有命令（内置和插件）都实现 `CommandHandler` trait
   - 统一的执行流程

3. **易于扩展**：
   - 添加新插件只需在 `plugins` 中实现 `CommandHandler` trait
   - 无需修改核心代码

4. **避免循环依赖**：
   - 主项目库不依赖插件
   - 插件依赖主项目
   - 可执行文件同时依赖主项目和插件

## 使用示例

### 创建插件

在 `plugins/src/` 中创建新插件：

```rust
use scargo::CommandHandler;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

pub struct MyPlugin;

#[async_trait]
impl CommandHandler for MyPlugin {
    fn name(&self) -> &'static str {
        "mycommand"
    }

    fn about(&self) -> &'static str {
        "My awesome command"
    }

    fn configure(&self, cmd: Command) -> Command {
        cmd.arg(Arg::new("input").required(true))
    }

    async fn execute(&self, matches: &ArgMatches, cwd: &PathBuf) -> anyhow::Result<()> {
        let input = matches.get_one::<String>("input").unwrap();
        println!("Processing: {}", input);
        Ok(())
    }
}

pub fn my_plugin() -> MyPlugin {
    MyPlugin
}
```

### 注册插件

在 `cli/src/main.rs` 中：

```rust
use scargo::Scargo;
use plugins::{jsp_plugin, my_plugin};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Scargo::new()
        .plugin(jsp_plugin())
        .plugin(my_plugin())
        .run()
        .await
}
```

## 构建和运行

```bash
# 构建所有 crate
cargo build

# 构建特定 crate
cargo build --package scargo
cargo build --package scargo-plugin
cargo build --package plugins
cargo build --package cli

# 运行
cargo run --package cli
```
