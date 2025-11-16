// src/add.rs
use anyhow::Context;
use std::path::Path;
use toml_edit::{Document, Item, Table};

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    let mut doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    let (artifact, scala_ver, version) = parse_dep_spec(dep_spec)?;

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

    // 关键修复：使用 as_table_mut() 获取 &mut Table
    let table = doc.as_table_mut();

    let dep_table = table
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()));

    if let Item::Table(dep_table) = dep_table {
        dep_table.insert(&full_key, toml_edit::value(version.clone()));

        // 美化格式
        let decor = dep_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");
    }

    std::fs::write(manifest_path, doc.to_string())?;
    println!("Added dependency: {} = {}", full_key, version);
    Ok(())
}
fn parse_dep_spec(spec: &str) -> anyhow::Result<(String, String, String)> {
    let (dep_part, version) = spec
        .split_once(':')
        .map(|(d, v)| (d, v.to_string()))
        .unwrap_or((spec, "latest".to_string()));

    let (group, artifact, scala_ver) = if dep_part.contains("::") {
        let (g, rest) = dep_part.split_once("::").unwrap();
        let (a, sv) = rest.split_once('@').unwrap_or((rest, "2.13"));
        (g.to_string(), a.to_string(), sv.to_string())
    } else {
        let (a, sv) = dep_part.split_once('@').unwrap_or((dep_part, "2.13"));
        (String::new(), a.to_string(), sv.to_string())
    };

    let artifact_full = if !group.is_empty() {
        format!("{}::{}", group, artifact)
    } else {
        artifact
    };

    Ok((artifact_full, scala_ver, version))
}