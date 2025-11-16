use anyhow::Context;
use std::path::Path;
use toml_edit::{Document, Item, Table, value};

pub fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document
    // 注意：我们将 doc 声明为 mut，以便直接在上面修改 TOML 结构。
    let mut doc: Document<String> = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 步骤 2：直接操作 Document 的根 Table
    // Document 实现了 DerefMut 到 Table，因此可以直接使用 entry() 等 Table 方法。
    // let mut table: Table = doc.into(); // <--- 移除：这是导致第一个错误的代码

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

    // 直接操作 doc。
    // 使用 Table::new() 替代 table!() 宏。
    let deps = doc
        .entry("dependencies")
        .or_insert(Item::Table(Table::new())); // <--- 修正第二个错误

    if let Item::Table(deps_table) = deps {
        deps_table[&full_key] = value(version.clone());

        // 美化格式
        // 如果需要为整个 [dependencies] 表添加前缀/后缀，应该操作 deps_table 的 decor
        let decor = deps_table.decor_mut();
        decor.set_prefix("\n");
        decor.set_suffix("\n");


        // 每个依赖换行 + 缩进
        for (_, item) in deps_table.iter_mut() {
            if let Item::Value(val) = item {
                let d = val.decor_mut();
                d.set_prefix("\n    "); // 4 空格缩进
                d.set_suffix("");
            }
        }
    }

    // 步骤 3：将 Document 转换回字符串并写入文件
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