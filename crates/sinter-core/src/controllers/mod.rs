//! 控制器模块
//!
//! 实现各种控制器，处理具体的业务逻辑

pub mod project;

use crate::cli::Commands;
use crate::error::Result;
use crate::toolkit::path::PathManager;

/// 控制器 trait
#[async_trait::async_trait]
pub trait Controller: Send + Sync {
    /// 处理命令
    async fn handle(&self, command: &Commands, cwd: &PathManager) -> Result<()>;
}