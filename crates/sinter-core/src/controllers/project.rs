//! 项目控制器
//!
//! 处理项目相关的命令，如创建新项目、初始化工作区等

use crate::cli::Commands;
use crate::controllers::Controller;
use crate::di::DIContext;
use crate::error::{Result, utils};
use crate::services::project::{ProjectService, ProjectServiceImpl};
use crate::toolkit::path::PathManager;

/// 项目控制器
pub struct ProjectController {
    di_context: DIContext,
}

impl ProjectController {
    /// 创建新的项目控制器
    pub fn new(di_context: DIContext) -> Self {
        Self { di_context }
    }
}

#[async_trait::async_trait]
impl Controller for ProjectController {
    async fn handle(&self, command: &Commands, cwd: &PathManager) -> Result<()> {
        let project_service = self.di_context.resolve::<ProjectServiceImpl>()
            .map_err(|e| utils::from_anyhow(anyhow::anyhow!(e)))?;

        match command {
            Commands::New { name } => project_service.create_project(name, cwd).await,
            Commands::Init => project_service.init_workspace(cwd).await,
            _ => Err(utils::single_validation_error(
                "ProjectController does not handle this command".to_string()
            )),
        }
    }
}