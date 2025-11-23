//! IDE支持模块
//!
//! 提供IDE集成功能

pub mod classpath_generator;
pub mod bsp_setup;

// Re-export for convenience
pub use classpath_generator::*;
pub use bsp_setup::*;