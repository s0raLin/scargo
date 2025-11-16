use anyhow::Context;
use std::path::Path;
use toml_edit::{value, Document, Item, Table}; 

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document<String>
    // Document 结构保留了原始文件的所有注释和格式信息 (decorations)
    let mut doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 步骤 2：解析依赖规格
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

    // **重要修复点：直接操作 Document 的根 Table 的 entry，保留 doc 的完整性**
    let deps = doc
        .as_table_mut() // 获取 Document 根 Table 的可变引用
        .entry("dependencies")
        .or_insert(Item::Table(Table::new())); 

    if let Item::Table(deps_table) = deps {
        // 插入新的依赖
        deps_table[&full_key] = value(version.clone());

        // 美化格式: 设置 [dependencies] 块的前后缀换行
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");

        // 美化格式: 设置每个依赖项的换行 + 缩进
        for (_, item) in deps_table.iter_mut() {
            if let Item::Value(val) = item {
                let d = val.decor_mut();
                d.set_prefix("\n    ");
                d.set_suffix("");
            }
        }
    }

    // **重要修复点：将修改后的 `doc` (Document) 转换回字符串并写入**
    // 这样才能保留原始 TOML 文件的所有内容 (如 [project] 等)
    std::fs::write(manifest_path, doc.to_string())?;
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