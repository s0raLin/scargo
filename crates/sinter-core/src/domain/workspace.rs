//! 工作空间模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use super::dependency::DependencySpec;
use super::directory::Directory;
use super::library::Library;

/// 工作空间配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Workspace {
    /// 工作区根目录路径 - 映射到文件系统中的实际目录
    #[serde(skip)]
    pub root_path: PathBuf,
    pub members: Vec<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}

impl Workspace {
    /// 创建带有根路径的工作空间实例
    pub fn with_root_path(mut self, root_path: PathBuf) -> Self {
        self.root_path = root_path;
        self
    }

    /// 获取工作区根目录的绝对路径
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }

    /// 获取指定成员的绝对路径
    pub fn get_member_path(&self, member_path: &str) -> PathBuf {
        self.root_path.join(member_path)
    }

    /// 获取所有成员的绝对路径
    pub fn get_member_paths(&self) -> Vec<PathBuf> {
        self.members.iter()
            .map(|member| self.root_path.join(member))
            .collect()
    }

    /// 检查成员路径是否存在
    pub fn member_exists(&self, member_path: &str) -> bool {
        self.get_member_path(member_path).exists()
    }

    /// 验证工作空间配置
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // 验证成员列表
        if self.members.is_empty() {
            errors.push("工作空间至少需要一个成员项目".to_string());
        }

        for member in &self.members {
            if member.trim().is_empty() {
                errors.push("工作空间成员路径不能为空".to_string());
            }
            // 检查路径格式（基本检查）
            if member.contains("..") {
                errors.push(format!("工作空间成员路径不能包含 '..': {}", member));
            }
        }

        // 检查重复成员
        let mut seen = std::collections::HashSet::new();
        for member in &self.members {
            if !seen.insert(member) {
                errors.push(format!("工作空间成员重复: {}", member));
            }
        }

        // 验证工作空间级依赖
        for (name, spec) in &self.dependencies {
            if name.trim().is_empty() {
                errors.push("工作空间依赖名称不能为空".to_string());
            }
            if let Err(dep_errors) = spec.validate() {
                for error in dep_errors {
                    errors.push(format!("工作空间依赖 '{}' 验证失败: {}", name, error));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取所有成员路径
    pub fn get_members(&self) -> &[String] {
        &self.members
    }

    /// 获取工作空间级依赖
    pub fn get_dependencies(&self) -> &HashMap<String, DependencySpec> {
        &self.dependencies
    }

    /// 检查是否包含指定成员
    pub fn contains_member(&self, member_path: &str) -> bool {
        self.members.iter().any(|m| m == member_path)
    }

    /// 添加成员（返回新的Workspace实例）
    pub fn add_member(mut self, member_path: String) -> Self {
        if !self.members.contains(&member_path) {
            self.members.push(member_path);
        }
        self
    }

    /// 移除成员（返回新的Workspace实例）
    pub fn remove_member(mut self, member_path: &str) -> Self {
        self.members.retain(|m| m != member_path);
        self
    }

    /// 获取工作区目录实体
    pub fn get_directory(&self) -> Directory {
        Directory::from_path(&self.root_path)
    }

    /// 获取所有成员目录实体
    pub fn get_member_directories(&self) -> Vec<Directory> {
        self.members.iter()
            .map(|member| Directory::from_path(self.get_member_path(member)))
            .collect()
    }

    /// 获取工作空间级依赖库实体
    pub fn get_libraries(&self) -> Vec<Library> {
        self.dependencies.iter()
            .map(|(name, spec)| Library::from_dependency_spec(name.clone(), spec.clone()))
            .collect()
    }
}