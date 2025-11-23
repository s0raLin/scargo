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

impl DependencySpec {
    /// 验证依赖规范
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        match self {
            DependencySpec::Simple(dep_str) => {
                if dep_str.trim().is_empty() {
                    errors.push("依赖字符串不能为空".to_string());
                } else {
                    // 简单的格式验证：group:artifact:version
                    let parts: Vec<&str> = dep_str.split(':').collect();
                    if parts.len() < 3 {
                        errors.push(format!("依赖格式无效 '{}'，应为 'group:artifact:version'", dep_str));
                    }
                }
            }
            DependencySpec::Detailed(detail) => {
                if let Some(version) = &detail.version {
                    if version.trim().is_empty() {
                        errors.push("依赖版本不能为空".to_string());
                    }
                }
                // workspace依赖不需要版本
                if detail.workspace && detail.version.is_some() {
                    errors.push("工作空间依赖不应指定版本".to_string());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// 获取依赖坐标字符串
    pub fn to_coordinate(&self) -> Option<String> {
        match self {
            DependencySpec::Simple(coord) => Some(coord.clone()),
            DependencySpec::Detailed(detail) => {
                // 对于详细配置，我们需要外部提供group和artifact信息
                // 这里返回None，表示需要更多上下文
                None
            }
        }
    }

    /// 检查是否为工作空间依赖
    pub fn is_workspace_dependency(&self) -> bool {
        match self {
            DependencySpec::Simple(_) => false,
            DependencySpec::Detailed(detail) => detail.workspace,
        }
    }
}