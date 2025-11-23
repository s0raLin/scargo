//! 项目配置模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::dependency::DependencySpec;
use super::workspace::Workspace;

/// 项目配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub workspace: Option<Workspace>,
}

impl Project {
    /// 获取主文件路径
    pub fn get_main_file_path(&self) -> std::path::PathBuf {
        let main_class = self.package.main.as_deref().unwrap_or("Main");
        std::path::PathBuf::from(&self.package.source_dir)
            .join(format!("{}.scala", main_class))
    }
}

/// 包信息
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub main: Option<String>,
    #[serde(default = "default_scala_version")]
    pub scala_version: String,
    #[serde(default = "default_source_dir")]
    pub source_dir: String,
    #[serde(default = "default_target_dir")]
    pub target_dir: String,
    #[serde(default = "default_test_dir")]
    pub test_dir: String,
    #[serde(default = "default_backend")]
    pub backend: String,
}

fn default_scala_version() -> String {
    "2.13".to_string()
}

fn default_source_dir() -> String {
    "src/main/scala".to_string()
}

fn default_target_dir() -> String {
    "target".to_string()
}

fn default_test_dir() -> String {
    "src/test/scala".to_string()
}

fn default_backend() -> String {
    "scala-cli".to_string()
}