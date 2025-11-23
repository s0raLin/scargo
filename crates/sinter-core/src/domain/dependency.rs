//! 依赖模型

use serde::{Deserialize, Serialize};

/// 依赖规范
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed(DependencyDetail),
}

/// 依赖详情
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DependencyDetail {
    pub version: Option<String>,
    #[serde(default)]
    pub workspace: bool,
}