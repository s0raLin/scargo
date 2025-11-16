// src/add.rs
use anyhow::Context;
use std::path::Path;
use toml_edit::{Document, Item, Table};

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;
    let mut doc = content.parse().context("Failed to parse Scargo.toml")?;

    // 解析 dep_spec: "cats-core@2.13" 或 "upickle@2.13:4.4.1"
    let (artifact, scala_ver, version) = parse_dep_spec(dep_spec)?;

    let key = format!("{}::{}", artifact, scala_ver);
    let dep_table = doc
        .as_table_mut()
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()));

    if let Item::Table(table) = dep_table {
        table.insert(&key, toml_edit::value(version));
        table.decorate("", ""); // 格式化
    }

    std::fs::write(manifest_path, doc.to_string())?;
    println!("Added dependency: {} = {}", key, version);
    Ok(())
}

fn parse_dep_spec(spec: &str) -> anyhow::Result<(String, String, String)> {
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
        // TODO: 查询 Maven Central
        "latest".to_string()
    } else {
        version
    };

    Ok((artifact.to_string(), scala_ver.to_string(), version))
}