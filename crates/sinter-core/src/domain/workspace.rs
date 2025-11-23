//! 工作空间模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::dependency::DependencySpec;

/// 工作空间配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Workspace {
    pub members: Vec<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}