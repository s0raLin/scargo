//! 项目配置模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use super::dependency::DependencySpec;
use super::workspace::Workspace;
use super::directory::Directory;
use super::library::Library;

/// 项目配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// 项目根目录路径 - 映射到文件系统中的实际目录
    #[serde(skip)]
    pub root_path: PathBuf,
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub workspace: Option<Workspace>,
}

impl Project {
    /// 创建带有根路径的项目实例
    pub fn with_root_path(mut self, root_path: PathBuf) -> Self {
        self.root_path = root_path;
        self
    }

    /// 获取项目根目录的绝对路径
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }

    /// 获取主文件路径（绝对路径）
    pub fn get_main_file_path(&self) -> PathBuf {
        let main_class = self.package.main.as_deref().unwrap_or("Main");
        self.root_path
            .join(&self.package.source_dir)
            .join(format!("{}.scala", main_class))
    }

    /// 获取源代码目录路径（相对路径）
    pub fn get_source_dir(&self) -> &str {
        &self.package.source_dir
    }

    /// 获取源代码目录绝对路径
    pub fn get_source_dir_abs(&self) -> PathBuf {
        self.root_path.join(&self.package.source_dir)
    }

    /// 获取目标目录路径（相对路径）
    pub fn get_target_dir(&self) -> &str {
        &self.package.target_dir
    }

    /// 获取目标目录绝对路径
    pub fn get_target_dir_abs(&self) -> PathBuf {
        self.root_path.join(&self.package.target_dir)
    }

    /// 获取测试目录路径（相对路径）
    pub fn get_test_dir(&self) -> &str {
        &self.package.test_dir
    }

    /// 获取测试目录绝对路径
    pub fn get_test_dir_abs(&self) -> PathBuf {
        self.root_path.join(&self.package.test_dir)
    }

    /// 获取构建后端
    pub fn get_backend(&self) -> &str {
        &self.package.backend
    }

    /// 获取项目名称
    pub fn get_name(&self) -> &str {
        &self.package.name
    }

    /// 获取项目版本
    pub fn get_version(&self) -> &str {
        &self.package.version
    }

    /// 获取Scala版本
    pub fn get_scala_version(&self) -> &str {
        &self.package.scala_version
    }

    /// 获取所有依赖（包括工作空间级别的）
    pub fn get_all_dependencies(&self) -> std::collections::HashMap<String, &DependencySpec> {
        let mut deps = std::collections::HashMap::new();

        // 添加项目级依赖
        for (name, spec) in &self.dependencies {
            deps.insert(name.clone(), spec);
        }

        // 添加工作空间级依赖（如果存在）
        if let Some(workspace) = &self.workspace {
            for (name, spec) in &workspace.dependencies {
                deps.insert(name.clone(), spec);
            }
        }

        deps
    }

    /// 检查是否为工作空间根项目
    pub fn is_workspace_root(&self) -> bool {
        self.workspace.is_some()
    }

    /// 获取工作空间配置（如果存在）
    pub fn get_workspace(&self) -> Option<&Workspace> {
        self.workspace.as_ref()
    }

    /// 获取项目目录实体
    pub fn get_directory(&self) -> Directory {
        Directory::from_path(&self.root_path)
    }

    /// 获取源代码目录实体
    pub fn get_source_directory(&self) -> Directory {
        Directory::from_path(self.get_source_dir_abs())
    }

    /// 获取目标目录实体
    pub fn get_target_directory(&self) -> Directory {
        Directory::from_path(self.get_target_dir_abs())
    }

    /// 获取测试目录实体
    pub fn get_test_directory(&self) -> Directory {
        Directory::from_path(self.get_test_dir_abs())
    }

    /// 获取所有依赖库实体
    pub fn get_libraries(&self) -> Vec<Library> {
        self.get_all_dependencies()
            .into_iter()
            .map(|(name, spec)| Library::from_dependency_spec(name, spec.clone()))
            .collect()
    }

    /// 验证项目配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证包信息
        if self.package.name.trim().is_empty() {
            errors.push("项目名称不能为空".to_string());
        }

        if self.package.version.trim().is_empty() {
            errors.push("项目版本不能为空".to_string());
        }

        // 验证Scala版本格式
        if !self.package.scala_version.starts_with("2.") && !self.package.scala_version.starts_with("3.") {
            errors.push("Scala版本格式无效，应为 2.x 或 3.x".to_string());
        }

        // 验证目录路径
        if self.package.source_dir.trim().is_empty() {
            errors.push("源代码目录不能为空".to_string());
        }

        if self.package.target_dir.trim().is_empty() {
            errors.push("目标目录不能为空".to_string());
        }

        // 验证后端
        let valid_backends = ["scala-cli", "sbt", "gradle", "maven"];
        if !valid_backends.contains(&self.package.backend.as_str()) {
            errors.push(format!("不支持的后端: {}，支持的后端: {}", self.package.backend, valid_backends.join(", ")));
        }

        // 验证依赖
        for (name, spec) in &self.dependencies {
            if name.trim().is_empty() {
                errors.push("依赖名称不能为空".to_string());
            }
            if let Err(dep_errors) = spec.validate() {
                for error in dep_errors {
                    errors.push(format!("依赖 '{}' 验证失败: {}", name, error));
                }
            }
        }

        // 验证工作空间
        if let Some(workspace) = &self.workspace {
            if let Err(ws_errors) = workspace.validate() {
                errors.extend(ws_errors);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
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