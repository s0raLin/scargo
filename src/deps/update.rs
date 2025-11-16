use anyhow::Context;
use std::path::Path;
use toml_edit::{value, DocumentMut};
use crate::deps::version_resolver::{VersionResolver, parse_dependency_spec};

/// 解析依赖键，返回 (group, artifact) 用于 Maven Central 查询
fn parse_dependency_key(key: &str) -> anyhow::Result<(String, String)> {
    // 键格式: group::artifact_scala_version
    // 我们需要提取 group 和完整的 artifact（包括 scala 版本）
    let parts: Vec<&str> = key.split("::").collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid dependency key format: {}", key);
    }

    let group = parts[0].to_string();
    let artifact = parts[1].to_string(); // 保持完整的 artifact，包括 Scala 版本

    Ok((group, artifact))
}

pub async fn update_dependency(project_dir: &Path, dep_spec: Option<&str>) -> anyhow::Result<()> {
    let project = crate::core::project::Project::load(project_dir)?;
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 确保 dependencies 表存在
    let deps_key = "dependencies";
    if !doc.contains_key(deps_key) {
        println!("{}", crate::t!("dep.no_dependencies"));
        return Ok(());
    }

    let deps_item = doc.get_mut(deps_key).unwrap();
    if let Some(deps_table) = deps_item.as_table_mut() {
        let mut resolver = VersionResolver::new();
        let mut updated_count = 0;

        // 收集需要更新的依赖
        let mut updates = Vec::new();

        if let Some(dep_spec) = dep_spec {
            // 更新指定的依赖
            let mut found = false;
            for (key, item) in deps_table.iter() {
                let key_str = key.to_string();
                if key_str.contains(dep_spec) {
                    if let Some(version_item) = item.as_value() {
                        if let Some(current_version) = version_item.as_str() {
                            // 解析依赖键
                            let (group, artifact) = parse_dependency_key(&key_str)?;
                            let latest_version = resolver.get_latest_version(&group, &artifact).await?;

                            if latest_version != current_version {
                                updates.push((key_str.clone(), latest_version.clone(), current_version.to_string()));
                            } else {
                                println!("{}", crate::t!("dep.up_to_date", &[&key_str]));
                            }
                            found = true;
                            break;
                        }
                    }
                }
            }
            if !found {
                anyhow::bail!("{}", crate::t!("dep.not_found", &[dep_spec]));
            }
        } else {
            // 更新所有依赖
            for (key, item) in deps_table.iter() {
                if let Some(version_item) = item.as_value() {
                    if let Some(current_version) = version_item.as_str() {
                        // 解析依赖键
                        let key_str = key.to_string();
                        let (group, artifact) = parse_dependency_key(&key_str)?;
                        let latest_version = resolver.get_latest_version(&group, &artifact).await?;

                        if latest_version != current_version {
                            updates.push((key_str, latest_version, current_version.to_string()));
                        }
                    }
                }
            }
        }

        // 应用更新
        for (key_str, latest_version, current_version) in updates {
            deps_table[&key_str] = value(latest_version.clone());
            println!("{}", crate::t!("dep.updated", &[&key_str, &current_version, &latest_version]));
            updated_count += 1;
        }

        if updated_count == 0 {
            println!("{}", crate::t!("dep.all_up_to_date"));
        }
    }

    std::fs::write(manifest_path, doc.to_string())?;
    Ok(())
}