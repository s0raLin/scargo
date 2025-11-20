# Scargo 插件集合

这个 crate 包含 Scargo 的扩展插件。

## 结构

```
plugins/
├── Cargo.toml
├── src/
│   ├── lib.rs      # 导出所有插件
│   └── jsp.rs      # JSP 项目生成插件
└── README.md
```

## 使用

在主项目的 `main.rs` 中：

```rust
use scargo::Scargo;
use plugins::jsp_plugin;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Scargo::new()
        .plugin(jsp_plugin())
        .run()
        .await
}
```

## 添加新插件

### 1. 添加依赖

在 `Cargo.toml` 中，只需要依赖 `scargo-plugin`：

```toml
[dependencies]
scargo-plugin = { path = "../scargo-plugin" }
```

### 2. 创建插件

在 `src/` 目录下创建新的插件文件（如 `my_plugin.rs`）：

```rust
// src/my_plugin.rs
use scargo_plugin::*;

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

    async fn execute(&self, matches: &ArgMatches, cwd: &PathBuf) -> AnyhowResult<()> {
        let input = matches.get_one::<String>("input").unwrap();
        println!("Processing: {}", input);
        
        // 使用重新导出的 fs 模块
        fs::write(cwd.join("output.txt"), input).await?;
        
        Ok(())
    }
}

pub fn my_plugin() -> MyPlugin {
    MyPlugin
}
```

### 3. 导出插件

在 `src/lib.rs` 中：

```rust
pub mod my_plugin;
pub use my_plugin::my_plugin;
```

## 优势

使用 `scargo-plugin` 的好处：

- ✅ **单一依赖**：不需要关心底层依赖版本
- ✅ **简化导入**：所有类型都通过 `use scargo_plugin::*;` 导入
- ✅ **统一接口**：所有插件使用相同的 API
- ✅ **易于维护**：依赖更新只需更新 API crate

