// src/add.rs
use anyhow::Context;
use std::path::Path;
use toml_edit::{value, Document, Item, Table};

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml")?;

    // 关键修复：消耗 doc，获得完整 Table（包含所有装饰）
    let mut table: Table = doc.into();

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

    // 使用 Table::new() 创建新表
    let deps = table
        .entry("dependencies")
        .or_insert(Item::Table(Table::new()));

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