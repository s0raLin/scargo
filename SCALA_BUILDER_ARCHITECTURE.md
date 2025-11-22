# Scala 构建器架构设计

## 概述

Scala 构建器是 Sinter 工具的一个扩展模块，用于生成模块化 Scala 项目结构。该构建器将集成到现有的 `new` 命令中，通过 `--modular` 选项启用模块化项目生成。

## 模块化 Scala 项目结构

以下是生成的模块化 Scala 项目的完整目录树：

```
my-scala-project/
├── project.toml                 # 工作区根配置
├── modules/                     # 模块目录
│   ├── core/                    # 核心模块
│   │   ├── project.toml         # 模块配置
│   │   ├── src/
│   │   │   └── main/
│   │   │       └── scala/
│   │   │           └── Core.scala
│   │   └── src/
│   │       └── test/
│   │           └── scala/
│   │               └── CoreSpec.scala
│   ├── api/                     # API 模块
│   │   ├── project.toml         # 模块配置
│   │   ├── src/
│   │   │   └── main/
│   │   │       └── scala/
│   │   │           └── Api.scala
│   │   └── src/
│   │       └── test/
│   │           └── scala/
│   │               └── ApiSpec.scala
│   └── app/                     # 应用模块
│       ├── project.toml         # 模块配置
│       ├── src/
│       │   └── main/
│       │       └── scala/
│       │           └── Main.scala
│       └── src/
│           └── test/
│               └── scala/
│                   └── AppSpec.scala
├── .gitignore                   # Git 忽略文件
├── README.md                    # 项目说明
└── build.sbt                    # SBT 构建文件（可选）
```

### 文件说明

- **project.toml (根)**: 工作区配置，定义成员模块和全局设置
- **modules/**: 存放所有子模块的目录
- **core/**: 核心业务逻辑模块
- **api/**: API 接口定义模块
- **app/**: 主应用入口模块
- **project.toml (模块)**: 每个模块的独立配置
- **src/main/scala/**: 主源码目录
- **src/test/scala/**: 测试源码目录

## Rust 模块划分

Scala 构建器将在 `crates/sinter-core/src/build/` 下新增 `scala_builder` 模块：

```
crates/sinter-core/src/build/
├── mod.rs
├── manager.rs
├── runner.rs
└── scala_builder/
    ├── mod.rs
    ├── project_generator.rs    # 项目生成器
    ├── module_generator.rs     # 模块生成器
    ├── template_manager.rs     # 模板管理器
    └── config_generator.rs     # 配置生成器
```

### 模块作用说明

#### project_generator.rs
- **职责**: 负责生成完整的模块化项目结构
- **功能**:
  - 创建根目录和子目录
  - 调用模块生成器创建各个子模块
  - 生成工作区配置文件
  - 生成根级别的辅助文件（如 .gitignore, README.md）

#### module_generator.rs
- **职责**: 生成单个模块的结构和文件
- **功能**:
  - 创建模块目录结构
  - 生成模块的 project.toml 配置
  - 创建源码和测试目录
  - 生成示例代码文件

#### template_manager.rs
- **职责**: 管理代码和配置模板
- **功能**:
  - 加载和渲染模板文件
  - 支持变量替换
  - 提供不同类型的模板（核心模块、API 模块、应用模块等）

#### config_generator.rs
- **职责**: 生成各种配置文件
- **功能**:
  - 生成工作区 project.toml
  - 生成模块级 project.toml
  - 生成 build.sbt（如果需要）
  - 处理依赖关系配置

## 集成方式

### CLI 集成
扩展现有的 `new` 命令，添加 `--modular` 选项：

```bash
sinter new --modular my-project
```

### 代码集成
在 `crates/sinter-core/src/cli/commands/new.rs` 中添加逻辑：

```rust
pub async fn cmd_new(cwd: &PathBuf, name: &str, modular: bool) -> anyhow::Result<()> {
    if modular {
        // 使用 Scala 构建器生成模块化项目
        crate::build::scala_builder::generate_modular_project(cwd, name).await?;
    } else {
        // 原有逻辑
        // ...
    }
    Ok(())
}
```

## 设计原则

1. **模块化**: 每个功能独立模块，便于维护和测试
2. **可扩展**: 易于添加新的模块类型和模板
3. **一致性**: 与现有 Sinter 架构保持一致
4. **灵活性**: 支持自定义模块结构和配置

## 依赖关系

```
sinter-core
├── build/
│   ├── scala_builder/
│   │   ├── project_generator     -> template_manager, config_generator
│   │   ├── module_generator      -> template_manager, config_generator
│   │   ├── template_manager      -> (独立)
│   │   └── config_generator      -> (独立)
│   └── ...
└── cli/commands/new.rs           -> scala_builder
```

## 模板文件

新增模板文件位于 `crates/sinter-core/templates/`：

- `modular_workspace.toml.template`: 工作区配置模板
- `modular_module.toml.template`: 模块配置模板
- `Core.scala.template`: 核心模块示例代码
- `Api.scala.template`: API 模块示例代码
- `MainModular.scala.template`: 应用模块主类
- `*.scala.template`: 各种测试模板

## 验证兼容性

该设计与现有 Sinter 架构完全兼容：

- 使用现有的模板系统
- 遵循现有的配置格式
- 集成到现有的 CLI 命令结构
- 不影响现有功能
- 支持现有的构建和依赖管理