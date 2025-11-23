//! 依赖库模型
//!
//! 映射外部依赖库实体

use std::path::PathBuf;
use super::dependency::DependencySpec;

/// 依赖库实体 - 映射到实际的外部库或本地库
#[derive(Debug, Clone)]
pub struct Library {
    /// 库名称
    pub name: String,
    /// 依赖规范
    pub spec: DependencySpec,
    /// 本地路径（如果已解析）
    pub local_path: Option<PathBuf>,
    /// 是否已下载/可用
    pub available: bool,
    /// 库类型
    pub library_type: LibraryType,
}

/// 库类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum LibraryType {
    /// 外部Maven/Ivy仓库库
    External,
    /// 本地文件系统库
    Local,
    /// 工作空间内部库
    Workspace,
}

impl Library {
    /// 从依赖规范创建库实例
    pub fn from_dependency_spec(name: String, spec: DependencySpec) -> Self {
        let library_type = match &spec {
            DependencySpec::Simple(_) => LibraryType::External,
            DependencySpec::Detailed(detail) => {
                if detail.workspace {
                    LibraryType::Workspace
                } else if detail.version.is_some() {
                    LibraryType::External
                } else {
                    LibraryType::Local
                }
            }
        };

        Self {
            name,
            spec,
            local_path: None,
            available: false,
            library_type,
        }
    }

    /// 设置本地路径
    pub fn with_local_path(mut self, path: PathBuf) -> Self {
        self.local_path = Some(path);
        self.available = true;
        self
    }

    /// 标记为可用
    pub fn mark_available(mut self) -> Self {
        self.available = true;
        self
    }

    /// 获取库坐标字符串
    pub fn get_coordinate(&self) -> Option<String> {
        self.spec.to_coordinate()
    }

    /// 检查是否为工作空间库
    pub fn is_workspace_library(&self) -> bool {
        self.library_type == LibraryType::Workspace
    }

    /// 检查是否为外部库
    pub fn is_external_library(&self) -> bool {
        self.library_type == LibraryType::External
    }

    /// 检查是否为本地库
    pub fn is_local_library(&self) -> bool {
        self.library_type == LibraryType::Local
    }

    /// 获取库的显示名称
    pub fn get_display_name(&self) -> String {
        if let Some(coord) = self.get_coordinate() {
            format!("{} ({})", self.name, coord)
        } else {
            self.name.clone()
        }
    }

    /// 获取库的版本（如果有）
    pub fn get_version(&self) -> Option<&str> {
        match &self.spec {
            DependencySpec::Simple(coord) => {
                // 从坐标字符串中提取版本
                coord.split(':').nth(2)
            }
            DependencySpec::Detailed(detail) => detail.version.as_deref(),
        }
    }
}

impl From<(String, DependencySpec)> for Library {
    fn from((name, spec): (String, DependencySpec)) -> Self {
        Self::from_dependency_spec(name, spec)
    }
}