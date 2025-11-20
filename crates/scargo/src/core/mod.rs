//! 核心模块
//!
//! 包含插件系统的核心 trait 和应用结构

pub mod handler;
pub mod app;

pub use handler::CommandHandler;
pub use app::Scargo;

