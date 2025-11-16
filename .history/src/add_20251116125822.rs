use anyhow::Context;
use std::path::Path;
// 导入 Deref 和 DerefMut，确保所有解引用方法都在作用域内

// 注意：这里不再需要导入 table 宏
use toml_edit::{value, DocumentMut, Item, Table};
use reqwest;
use serde_json;

pub async fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    let (artifact, scala_ver, version) = parse_dep_spec(dep_spec).await?;

    let key = if artifact.contains("::") {
        artifact.clone()
    } else {
        format!("org.typelevel::{}", artifact)
    };

    let full_key = if !scala_ver.is_empty() && scala_ver != "latest" {
        format!("{}_{}", key, scala_ver)
    } else {
        key
    };

    // 确保 dependencies 表存在
    let deps_key = "dependencies";
    if !doc.contains_key(deps_key) {
        doc.insert(deps_key, Item::Table(Table::new()));
    }

    let deps_item = doc.get_mut(deps_key).unwrap();
    if let Some(deps_table) = deps_item.as_table_mut() {
        deps_table[&full_key] = value(version.clone());

        // 美化格式
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");
    }

    std::fs::write(manifest_path, doc.to_string())?;
    println!("Added dependency: {} = {}", full_key, version);
    Ok(())
}

async fn parse_dep_spec(spec: &str) -> anyhow::Result<(String, String, String)> {
    let parts: Vec<&str> = spec.split(':').collect();
    let (artifact_part, version) = if parts.len() == 2 {
        (parts[0], parts[1].to_string())
    } else {
        (parts[0], "latest".to_string())
    };

    let artifact_parts: Vec<&str> = artifact_part.split('@').collect();
    let (artifact, scala_ver) = if artifact_parts.len() == 2 {
        (artifact_parts[0], artifact_parts[1])
    } else {
        (artifact_parts[0], "2.13") // 默认
    };

    let version = if version == "latest" {
        // 查询 Maven Central 获取最新版本
        let key = if artifact.contains("::") {
            artifact.to_string()
        } else {
            format!("org.typelevel::{}", artifact)
        };
        let group_artifact: Vec<&str> = key.split("::").collect();
        if group_artifact.len() != 2 {
            anyhow::bail!("Invalid artifact format: {}", key);
        }
        let group_id = group_artifact[0];
        let artifact_id = if !scala_ver.is_empty() && scala_ver != "latest" {
            format!("{}_{}", group_artifact[1], scala_ver)
        } else {
            group_artifact[1].to_string()
        };
        get_latest_version(group_id, &artifact_id).await?
    } else {
        version
    };

    Ok((artifact.to_string(), scala_ver.to_string(), version))
}

async fn get_latest_version(group_id: &str, artifact_id: &str) -> anyhow::Result<String> {
    let url = format!(
        "https://search.maven.org/solrsearch/select?q=g:{}+AND+a:{}&rows=1&wt=json",
        group_id, artifact_id
    );
    let response: serde_json::Value = reqwest::get(&url)
        .await?
        .json()
        .await?;
    if let Some(docs) = response["response"]["docs"].as_array() {
        if let Some(first) = docs.first() {
            if let Some(v) = first["latestVersion"].as_str() {
                return Ok(v.to_string());
            }
        }
    }
    anyhow::bail!("No version found for {}:{}", group_id, artifact_id);
}