//! 配置兼容性层
//!
//! 提供向后兼容的配置函数，这些函数委托给新的模块化实现

use crate::domain::*;
use crate::deps::Dependency;

/// 获取项目依赖（兼容性函数）
#[deprecated(note = "Use dependency resolver instead")]
pub fn get_dependencies(project: &Project) -> Vec<Dependency> {
    project.dependencies
        .iter()
        .filter_map(|(k, spec)| match spec {
            DependencySpec::Simple(version) => {
                Some(Dependency::from_toml_key(k, &version))
            }
            DependencySpec::Detailed(detail) => {
                detail.version.as_ref().map(|v| Dependency::from_toml_key(k, v))
            }
        })
        .collect()
}

/// 获取包含工作空间的依赖（兼容性函数）
#[deprecated(note = "Use dependency resolver instead")]
pub fn get_dependencies_with_workspace(project: &Project, workspace_root: Option<&Project>) -> Vec<Dependency> {
    let mut deps = Vec::new();

    for (k, spec) in &project.dependencies {
        match spec {
            DependencySpec::Simple(version) => {
                deps.push(Dependency::from_toml_key(k, &version));
            }
            DependencySpec::Detailed(detail) => {
                if detail.workspace {
                    if let Some(ws) = workspace_root {
                        if let Some(ws_config) = &ws.workspace {
                            if let Some(ws_spec) = ws_config.dependencies.get(k) {
                                match ws_spec {
                                    DependencySpec::Simple(version) => {
                                        deps.push(Dependency::from_toml_key(k, &version));
                                    }
                                    DependencySpec::Detailed(ws_detail) => {
                                        if let Some(version) = &ws_detail.version {
                                            deps.push(Dependency::from_toml_key(k, version));
                                        }
                                    }
                                }
                            } else {
                                eprintln!("Warning: dependency '{}' marked as workspace but not found in workspace root", k);
                            }
                        }
                    }
                } else if let Some(version) = &detail.version {
                    deps.push(Dependency::from_toml_key(k, version));
                }
            }
        }
    }

    deps
}

/// 获取传递依赖（兼容性函数）
#[deprecated(note = "Use dependency resolver instead")]
pub async fn get_transitive_dependencies_with_workspace(
    project: &Project,
    workspace_root: Option<&Project>,
    project_dir: &std::path::Path,
) -> anyhow::Result<Vec<Dependency>> {
    let direct_deps = get_dependencies_with_workspace(project, workspace_root);
    let mut dep_manager = crate::deps::default_dependency_manager().await;
    dep_manager.set_project_dir(project_dir);
    dep_manager.get_transitive_dependencies(&direct_deps).await
}

/// 获取主文件路径（兼容性函数）
#[deprecated(note = "Use Project::get_main_file_path() instead")]
pub fn get_main_file_path(project: &Project) -> std::path::PathBuf {
    project.get_main_file_path()
}