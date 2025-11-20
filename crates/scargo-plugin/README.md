# Scargo 插件开发 API

这个 crate 提供了开发 Scargo 插件所需的所有依赖和类型。**插件开发者只需要依赖这一个 crate**，就可以获得所有必要的工具。

## 特性

- ✅ **单一依赖**：只需要添加 `scargo-plugin` 到你的 `Cargo.toml`
- ✅ **类型重新导出**：所有常用类型都已重新导出，无需额外导入
- ✅ **简化开发**：不需要关心底层依赖版本

## 使用

### 1. 添加依赖

在你的插件项目的 `Cargo.toml` 中：

```toml
[dependencies]
scargo-plugin = { path = "../scargo-plugin" }
# 或者如果发布到 crates.io：
# scargo-plugin = "0.1.0"
```

### 2. 创建插件

```rust
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

## 重新导出的类型

通过 `use scargo_plugin::*;` 你可以直接使用：

- `CommandHandler` - 插件 trait
- `async_trait` - async trait 宏
- `Command`, `Arg`, `ArgMatches` - clap 类型
- `PathBuf` - 路径类型
- `AnyhowResult<T>` - 错误处理类型（等同于 `anyhow::Result<T>`）
- `fs` - tokio 文件系统模块

## 完整示例

查看 `plugins/src/jsp.rs` 查看完整的插件示例。

