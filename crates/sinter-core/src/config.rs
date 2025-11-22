use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Context;
use config::Config;
use toml_edit::{value, DocumentMut, Item, Table};

use crate::deps::Dependency;

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
    let manifest_path = dir.join("project.toml");
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

    // 只添加项目中显式声明的依赖
    for (k, spec) in &project.dependencies {
        match spec {
            crate::config::DependencySpec::Simple(version) => {
                // 简单格式的依赖直接使用项目中指定的版本
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
                            } else {
                                // workspace中没有定义此依赖，跳过
                                eprintln!("Warning: dependency '{}' marked as workspace but not found in workspace root", k);
                            }
                        }
                    }
                } else if let Some(version) = &detail.version {
                    // 非workspace依赖，使用项目中指定的版本
                    deps.push(Dependency::from_toml_key(k, version));
                }
            }
        }
    }

    deps
}

pub async fn get_transitive_dependencies_with_workspace(project: &Project, workspace_root: Option<&Project>, project_dir: &Path) -> anyhow::Result<Vec<Dependency>> {
    let direct_deps = get_dependencies_with_workspace(project, workspace_root);
    let mut dep_manager = crate::deps::default_dependency_manager().await;
    dep_manager.set_project_dir(project_dir);
    dep_manager.get_transitive_dependencies(&direct_deps).await
}

pub async fn generate_ide_classpath(project: &Project, workspace_root: Option<&Project>, project_dir: &Path) -> anyhow::Result<()> {
    let transitive_deps = get_transitive_dependencies_with_workspace(project, workspace_root, project_dir).await?;

    let dep_manager = crate::deps::default_dependency_manager().await;
    let target_dir = project_dir.join(&project.package.target_dir);
    dep_manager.prepare_dependencies(&transitive_deps, &target_dir).await?;

    // 获取依赖JAR文件的路径
    let mut classpath_entries = String::new();
    for dep in &transitive_deps {
        if let Dependency::Maven { group, artifact, version, is_scala: _ } = dep {
            // coursier通常将JAR文件存储在~/.coursier/cache/v1/https/repo1.maven.org/maven2/...
            // 我们需要找到实际的JAR文件路径
            if let Some(jar_path) = find_jar_path(group, artifact, version) {
                classpath_entries.push_str(&format!("\t<classpathentry kind=\"lib\" path=\"{}\"/>\n", jar_path.display()));
            }
        }
    }

    // 读取模板
    let template_path = Path::new("templates/.classpath.template");
    let template_content = std::fs::read_to_string(template_path)?;

    // 替换模板变量
    let classpath_content = template_content
        .replace("{source_dir}", &project.package.source_dir)
        .replace("{target_dir}", &project.package.target_dir)
        .replace("{classpath_entries}", &classpath_entries);

    // 写入.classpath文件
    let classpath_path = project_dir.join(".classpath");
    std::fs::write(classpath_path, classpath_content)?;

    Ok(())
}

pub async fn generate_ide_options_v2(project: &Project, workspace_root: Option<&Project>, project_dir: &Path) -> anyhow::Result<()> {
    let transitive_deps = get_transitive_dependencies_with_workspace(project, workspace_root, project_dir).await?;

    let dep_manager = crate::deps::default_dependency_manager().await;
    let target_dir = project_dir.join(&project.package.target_dir);
    dep_manager.prepare_dependencies(&transitive_deps, &target_dir).await?;

    // 构建依赖列表
    let mut dependency_coords = Vec::new();
    for dep in &transitive_deps {
        dependency_coords.push(dep.coord());
    }

    // 读取模板（相对于crate根目录）
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/ide-options-v2.json.template");
    let template_content = std::fs::read_to_string(template_path)?;

    // 替换模板变量
    let dependencies_json = serde_json::to_string(&dependency_coords)?;
    let options_content = template_content
        .replace("{scalac_option}", "-deprecation")  // 默认scalac选项
        .replace("\"dependency\": []", &format!("\"dependency\": {}", dependencies_json));

    // 写入ide-options-v2.json文件
    let options_path = project_dir.join("ide-options-v2.json");
    std::fs::write(options_path, options_content)?;

    Ok(())
}

fn find_jar_path(group: &str, artifact: &str, version: &str) -> Option<PathBuf> {
    // 尝试常见的coursier缓存位置
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cache_base = PathBuf::from(home).join(".coursier").join("cache").join("v1");

    // Maven Central路径
    let maven_path = cache_base
        .join("https")
        .join("repo1.maven.org")
        .join("maven2")
        .join(group.replace(".", "/"))
        .join(artifact)
        .join(version)
        .join(format!("{}-{}.jar", artifact, version));

    if maven_path.exists() {
        Some(maven_path)
    } else {
        // 尝试其他仓库或返回None
        None
    }
}

pub fn get_main_file_path(project: &Project) -> std::path::PathBuf {
    let main_class = project.package.main.as_deref().unwrap_or("Main");
    std::path::PathBuf::from(&project.package.source_dir).join(format!("{}.scala", main_class))
}

pub fn find_workspace_root(start_dir: &Path) -> Option<std::path::PathBuf> {
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

pub fn load_workspace(dir: &Path) -> anyhow::Result<Option<(Project, Vec<Project>)>> {
    let manifest_path = dir.join("project.toml");
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
        .context("Failed to parse project.toml as TOML document")?;

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

pub fn add_workspace_dependency_to_manifest(manifest_path: &Path, key: &str, version: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse project.toml as TOML document")?;

    // 确保 workspace 表存在
    let ws_key = "workspace";
    if !doc.contains_key(ws_key) {
        doc.insert(ws_key, Item::Table(Table::new()));
    }

    let ws_item = doc.get_mut(ws_key).unwrap();
    if let Some(ws_table) = ws_item.as_table_mut() {
        // 确保 workspace.dependencies 表存在
        let deps_key = "dependencies";
        if !ws_table.contains_key(deps_key) {
            ws_table.insert(deps_key, Item::Table(Table::new()));
        }

        if let Some(deps_item) = ws_table.get_mut(deps_key) {
            if let Some(deps_table) = deps_item.as_table_mut() {
                deps_table[key] = value(version.to_string());

                // 美化格式
                let decor = deps_table.decor_mut();
                decor.set_prefix("\n");
                decor.set_suffix("\n");
            }
        }
    }

    std::fs::write(manifest_path, doc.to_string())?;
    Ok(())
}

pub fn add_workspace_member(manifest_path: &Path, member_path: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(manifest_path)?;
    let mut doc: DocumentMut = content.parse().context("Failed to parse project.toml")?;

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
                if exists {
                    anyhow::bail!("Member '{}' already exists in workspace", member_path);
                }
                members_array.push(member_path);
                std::fs::write(manifest_path, doc.to_string())?;
            }
        }
    }

    Ok(())
}