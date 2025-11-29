//! 路由模块
//!
//! 实现路由分发器，将CLI命令分发到对应的控制器

use crate::cli::Commands;
use crate::controllers::Controller;
use crate::error::{Result, utils};
use crate::toolkit::path::PathManager;
use std::collections::HashMap;

/// 路由处理器
pub struct RouteHandler {
    controller: Box<dyn Controller>,
}

impl RouteHandler {
    /// 创建新的路由处理器
    pub fn new<C: Controller + 'static>(controller: C) -> Self {
        Self {
            controller: Box::new(controller),
        }
    }
}

#[async_trait::async_trait]
impl Controller for RouteHandler {
    async fn handle(&self, command: &Commands, cwd: &PathManager) -> Result<()> {
        self.controller.handle(command, cwd).await
    }
}

/// 路由分发器
pub struct Router {
    routes: HashMap<String, RouteHandler>,
}

impl Router {
    /// 创建新的路由分发器
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    /// 注册路由
    pub fn register<C: Controller + 'static>(mut self, name: &str, controller: C) -> Self {
        self.routes.insert(name.to_string(), RouteHandler::new(controller));
        self
    }

    /// 分发命令
    pub async fn dispatch(&self, command: Commands, cwd: PathManager) -> Result<()> {
        let command_name = match &command {
            Commands::New { .. } => "new",
            Commands::Init => "init",
            Commands::Workspace { .. } => "workspace",
            Commands::Build => "build",
            Commands::Run { .. } => "run",
            Commands::Add { .. } => "add",
            Commands::Test { .. } => "test",
            Commands::Jsp { .. } => "jsp",
        };

        if let Some(handler) = self.routes.get(command_name) {
            handler.handle(&command, &cwd).await
        } else {
            Err(utils::single_validation_error(
                format!("No handler found for command: {}", command_name)
            ))
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}