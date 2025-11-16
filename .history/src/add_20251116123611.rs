// src/add.rs
use anyhow::Context;
use std::path::Path;
use toml_edit::{table, value, Document, Item, Table}; // 必须有 table！

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 1. 解析为 Document
    let doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml")?;

    // 2. 克隆为可变 Table
    let mut table: Table = doc.as_table().clone();

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

    // 3. 创建或获取 [dependencies]
    let deps = table
        .entry("dependencies")
        .or_insert(Item::Table(table!())); // 使用 table!() 宏

    if let Item::Table(deps_table) = deps {
        deps_table[&full_key] = value(version.clone());

        // 美化格式
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");

        for (_, item) in deps_table.iter_mut() {
            if let Item::Value(val) = item {
                let d = val.decor_mut();
                d.set_prefix("\n    ");
                d.set_suffix("");
            }
        }
    }

    std::fs::write(manifest_path, table.to_string())?;
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