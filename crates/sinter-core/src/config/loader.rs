//! 配置加载器
//!
//! 负责从文件系统加载和解析配置文件

use std::path::{Path, PathBuf};
use anyhow::Context;
use config::Config;

use crate::domain::*;

/// 加载项目配置
pub fn load_project(dir: &Path) -> anyhow::Result<Project> {
    let manifest_path = dir.join("project.toml");
    let settings = Config::builder()
        .add_source(config::File::from(manifest_path))
        .build()
        .context("Failed to load project configuration")?;
    let proj: Project = settings.try_deserialize()
        .context("Failed to parse project configuration")?;

    // 验证配置
    if let Err(errors) = proj.validate() {
        return Err(anyhow::anyhow!("项目配置验证失败:\n{}", errors.join("\n")));
    }

    Ok(proj)
}

/// 异步版本的项目配置加载
pub async fn load_project_async(dir: &Path) -> anyhow::Result<Project> {
    // 对于异步操作，如果需要可以扩展
    load_project(dir)
}

/// 查找工作空间根目录
pub fn find_workspace_root(start_dir: &Path) -> Option<PathBuf> {
    let mut current = start_dir;
    loop {
        let manifest = current.join("project.toml");
        if manifest.exists() {
            if let Ok(settings) = Config::builder()
                .add_source(config::File::from(manifest.clone()))
                .build()
            {
                if let Ok(project) = settings.try_deserialize::<Project>() {
                    if project.workspace.is_some() {
                        return Some(current.to_path_buf());
                    }
                }
            }
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break;
        }
    }
    None
}

/// 加载工作空间配置
pub fn load_workspace(dir: &Path) -> anyhow::Result<Option<(Project, Vec<Project>)>> {
    let manifest_path = dir.join("project.toml");
    if !manifest_path.exists() {
        return Ok(None);
    }

    let settings = Config::builder()
        .add_source(config::File::from(manifest_path))
        .build()
        .context("Failed to load workspace configuration")?;

    let root_project: Project = settings.try_deserialize()
        .context("Failed to parse workspace configuration")?;

    if let Some(workspace) = &root_project.workspace {
        let mut members = Vec::new();
        for member_path in &workspace.members {
            let member_dir = dir.join(member_path);
            let member_project = load_project(&member_dir)
                .with_context(|| format!("Failed to load workspace member: {}", member_path))?;
            members.push(member_project);
        }
        Ok(Some((root_project, members)))
    } else {
        Ok(None)
    }
}