use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use config::Config;
use toml_edit::{value, DocumentMut, Item, Table};

use crate::deps::deps::Dependency;

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    pub package: Package,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub workspace: Option<Workspace>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed(DependencyDetail),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DependencyDetail {
    pub version: Option<String>,
    #[serde(default)]
    pub workspace: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Workspace {
    pub members: Vec<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
}

#[derive(Deserialize, Serialize, Debug)]
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

pub fn load_project(dir: &Path) -> anyhow::Result<Project> {
    let manifest_path = dir.join("Scargo.toml");
    let settings = Config::builder()
        .add_source(config::File::from(manifest_path))
        .build()?;
    let proj: Project = settings.try_deserialize()?;
    Ok(proj)
}

pub fn load_project_async(dir: &Path) -> anyhow::Result<Project> {
    // For async operations if needed
    load_project(dir)
}

pub fn get_dependencies(project: &Project) -> Vec<Dependency> {
    project.dependencies
        .iter()
        .filter_map(|(k, spec)| match spec {
            crate::config::DependencySpec::Simple(version) => {
                Some(Dependency::from_toml_key(k, &version))
            }
            crate::config::DependencySpec::Detailed(detail) => {
                detail.version.as_ref().map(|v| Dependency::from_toml_key(k, v))
            }
        })
        .collect()
}

pub fn get_dependencies_with_workspace(project: &Project, workspace_root: Option<&Project>) -> Vec<Dependency> {
    let mut deps = Vec::new();

    // 先添加workspace依赖
    if let Some(ws) = workspace_root {
        if let Some(ws_config) = &ws.workspace {
            for (k, spec) in &ws_config.dependencies {
                match spec {
                    crate::config::DependencySpec::Simple(version) => {
                        deps.push(Dependency::from_toml_key(k, &version));
                    }
                    crate::config::DependencySpec::Detailed(detail) => {
                        if let Some(version) = &detail.version {
                            deps.push(Dependency::from_toml_key(k, version));
                        }
                    }
                }
            }
        }
    }

    // 然后添加项目特定依赖
    for (k, spec) in &project.dependencies {
        match spec {
            crate::config::DependencySpec::Simple(version) => {
                deps.push(Dependency::from_toml_key(k, &version));
            }
            crate::config::DependencySpec::Detailed(detail) => {
                if detail.workspace {
                    // workspace依赖，从workspace根获取版本
                    if let Some(ws) = workspace_root {
                        if let Some(ws_config) = &ws.workspace {
                            if let Some(ws_spec) = ws_config.dependencies.get(k) {
                                match ws_spec {
                                    crate::config::DependencySpec::Simple(version) => {
                                        deps.push(Dependency::from_toml_key(k, &version));
                                    }
                                    crate::config::DependencySpec::Detailed(ws_detail) => {
                                        if let Some(version) = &ws_detail.version {
                                            deps.push(Dependency::from_toml_key(k, version));
                                        }
                                    }
                                }
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

pub fn get_main_file_path(project: &Project) -> std::path::PathBuf {
    let main_class = project.package.main.as_deref().unwrap_or("Main");
    std::path::PathBuf::from(&project.package.source_dir).join(format!("{}.scala", main_class))
}

pub fn find_workspace_root(start_dir: &Path) -> Option<std::path::PathBuf> {
    let mut current = start_dir;
    loop {
        let manifest = current.join("Scargo.toml");
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

pub fn load_workspace(dir: &Path) -> anyhow::Result<Option<(Project, Vec<Project>)>> {
    let manifest_path = dir.join("Scargo.toml");
    if !manifest_path.exists() {
        return Ok(None);
    }
    let settings = Config::builder()
        .add_source(config::File::from(manifest_path))
        .build()?;
    let root_project: Project = settings.try_deserialize()?;
    if let Some(workspace) = &root_project.workspace {
        let mut members = Vec::new();
        for member_path in &workspace.members {
            let member_dir = dir.join(member_path);
            let member_project = load_project(&member_dir)?;
            members.push(member_project);
        }
        Ok(Some((root_project, members)))
    } else {
        Ok(None)
    }
}


pub fn add_dependency_to_manifest(manifest_path: &Path, key: &str, version: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 确保 dependencies 表存在
    let deps_key = "dependencies";
    if !doc.contains_key(deps_key) {
        doc.insert(deps_key, Item::Table(Table::new()));
    }

    let deps_item = doc.get_mut(deps_key).unwrap();
    if let Some(deps_table) = deps_item.as_table_mut() {
        deps_table[key] = value(version.to_string());

        // 美化格式
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");
    }

    std::fs::write(manifest_path, doc.to_string())?;
    Ok(())
}

pub fn add_workspace_member(manifest_path: &Path, member_path: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(manifest_path)?;
    let mut doc: DocumentMut = content.parse().context("Failed to parse Scargo.toml")?;

    // Ensure workspace table exists
    let ws_key = "workspace";
    if !doc.contains_key(ws_key) {
        doc.insert(ws_key, Item::Table(Table::new()));
    }

    let ws_item = doc.get_mut(ws_key).unwrap();
    if let Some(ws_table) = ws_item.as_table_mut() {
        // Ensure members array exists
        if !ws_table.contains_key("members") {
            ws_table.insert("members", Item::Value(toml_edit::Value::Array(Default::default())));
        }

        if let Some(members_item) = ws_table.get_mut("members") {
            if let Some(members_array) = members_item.as_array_mut() {
                // Check if member already exists
                let exists = members_array.iter().any(|v| v.as_str() == Some(member_path));
                if !exists {
                    members_array.push(member_path);
                    std::fs::write(manifest_path, doc.to_string())?;
                }
            }
        }
    }

    Ok(())
}