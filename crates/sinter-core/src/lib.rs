//! Sinter - Scala 项目构建工具
//!
//! 这是一个类似 Cargo 的 Scala 项目管理和构建工具。

// 核心模块
pub mod core;

// 运行时模块
pub mod runtime;

// CLI 和配置
pub mod cli;
pub mod config;

// 功能模块
pub mod build;
pub mod deps;

// 国际化支持（构建时生成）
pub mod i18n;

// extern crate paste;

// 公共 API
pub use core::{CommandHandler, Sinter};
pub use cli::{Cli, Commands, WorkspaceCommands};