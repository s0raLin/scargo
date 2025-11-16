use anyhow::Context;
use std::path::Path;
// 导入 Deref 和 DerefMut，确保所有解引用方法都在作用域内

use toml_edit::{Document, Item, Table, value};

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 步骤 2：消耗 doc，转换为 Table（可变）
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

    // 直接操作 table
    let deps = table
        .entry("dependencies")
        .or_insert(Item::Table(table!())); // 导入 table 宏

    if let Item::Table(deps_table) = deps {
        deps_table[&full_key] = value(version.clone());

        // 美化格式
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");

        // 每个依赖换行 + 缩进
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