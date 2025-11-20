//! Sinter 插件集合
//!
//! 这个 crate 包含 Sinter 的扩展插件。
//! 所有插件都实现 `sinter::CommandHandler` trait。

pub mod jsp;

// 导出所有插件
pub use jsp::jsp_plugin;

