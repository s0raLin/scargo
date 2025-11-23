//! Sinter - Scala 项目构建工具
//!
//! 这是一个类似 Cargo 的 Scala 项目管理和构建工具。

// 领域模型
pub mod domain;

// 配置管理
pub mod config;

// 依赖管理
pub mod dependency;

// 构建系统
pub mod build;

// 工作空间管理
pub mod workspace;

// IDE支持
pub mod ide;

// 命令行接口
pub mod cli;

// 运行时
pub mod runtime;

// 核心模块
pub mod core;

// 国际化支持（构建时生成）
pub mod i18n;

// 兼容性层（已废弃）
mod config_compat;

// 功能模块（已迁移）
pub mod deps;

// 公共 API
pub use core::{CommandHandler, Sinter};
pub use cli::{Cli, Commands, WorkspaceCommands};