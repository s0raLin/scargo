//! Scargo 插件集合
//!
//! 这个 crate 包含 Scargo 的扩展插件。
//! 所有插件都实现 `scargo::CommandHandler` trait。

pub mod jsp;

// 导出所有插件
pub use jsp::jsp_plugin;

