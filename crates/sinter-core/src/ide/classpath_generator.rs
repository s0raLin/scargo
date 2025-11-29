//! Classpath生成器

use crate::models::Project;
use crate::deps::Dependency;

/// Classpath生成器
pub struct ClasspathGenerator;

impl ClasspathGenerator {
    /// 生成项目classpath
    pub fn generate_classpath(&self, project: &Project, dependencies: &[Dependency]) -> anyhow::Result<String> {
        // TODO: 实现classpath生成逻辑
        Ok(String::new())
    }

    /// 生成工作空间classpath
    pub fn generate_workspace_classpath(&self, projects: &[Project], dependencies: &[Dependency]) -> anyhow::Result<String> {
        // TODO: 实现工作空间classpath生成逻辑
        Ok(String::new())
    }
}