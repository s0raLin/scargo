use anyhow::Context;
use std::path::Path;
use toml_edit::{value, DocumentMut, Item, Table};
use crate::deps::version_resolver::{VersionResolver, parse_dependency_spec};

pub async fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let project = crate::core::project::Project::load(project_dir)?;
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 步骤 1：解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    let (group, artifact, version_spec) = parse_dependency_spec(dep_spec)?;
    let mut resolver = VersionResolver::new();
    let version = resolver.resolve_version(&group, &artifact, &version_spec).await?;

    // 构建完整的依赖key
    let full_key = if artifact.contains('_') {
        // 已经包含Scala版本
        format!("{}::{}", group, artifact)
    } else {
        // 需要添加Scala版本
        let scala_suffix = if version_spec.contains("@") {
            // 从版本规范中提取Scala版本
            version_spec.split('@').nth(1).unwrap_or(&project.package.scala_version)
        } else {
            &project.package.scala_version
        };
        format!("{}::{}_{}", group, artifact, scala_suffix)
    };

    // 确保 dependencies 表存在
    let deps_key = "dependencies";
    if !doc.contains_key(deps_key) {
        doc.insert(deps_key, Item::Table(Table::new()));
    }

    let deps_item = doc.get_mut(deps_key).unwrap();
    if let Some(deps_table) = deps_item.as_table_mut() {
        deps_table[&full_key] = value(version.clone());
    }

    std::fs::write(manifest_path, doc.to_string())?;
    println!("{}", crate::t!("dep.added", &[&full_key, &version]));
    Ok(())
}
