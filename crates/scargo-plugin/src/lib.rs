//! Scargo 插件开发 API
//!
//! 这个 crate 提供了开发 Scargo 插件所需的所有依赖和类型。
//! 插件开发者只需要依赖这个 crate，就可以获得所有必要的工具。
//!
//! ## 使用示例
//!
//! ```rust
//! use scargo_plugin::*;
//!
//! pub struct MyPlugin;
//!
//! #[async_trait]
//! impl CommandHandler for MyPlugin {
//!     fn name(&self) -> &'static str {
//!         "mycommand"
//!     }
//!
//!     fn about(&self) -> &'static str {
//!         "My awesome command"
//!     }
//!
//!     fn configure(&self, cmd: Command) -> Command {
//!         cmd.arg(Arg::new("input").required(true))
//!     }
//!
//!     async fn execute(&self, matches: &ArgMatches, cwd: &PathBuf) -> AnyhowResult<()> {
//!         let input = matches.get_one::<String>("input").unwrap();
//!         println!("Processing: {}", input);
//!         
//!         // 使用重新导出的 fs 模块
//!         fs::write(cwd.join("output.txt"), input).await?;
//!         
//!         Ok(())
//!     }
//! }
//! ```

// 重新导出核心类型
pub use scargo::CommandHandler;

// 重新导出常用类型，方便插件开发
pub use clap::{Arg, ArgMatches, Command};
pub use std::path::PathBuf;
pub use anyhow::Result as AnyhowResult;
pub use async_trait::async_trait;

// 重新导出 tokio 的常用功能
pub use tokio::fs;

