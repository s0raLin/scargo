use anyhow::Context;
use std::path::Path;
use toml_edit::{DocumentMut};

pub async fn remove_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let manifest_path = project_dir.join("Scargo.toml");
    let content = std::fs::read_to_string(&manifest_path)?;

    // 解析为 Document<String>
    let mut doc: DocumentMut = content
        .parse()
        .context("Failed to parse Scargo.toml as TOML document")?;

    // 确保 dependencies 表存在
    let deps_key = "dependencies";
    if !doc.contains_key(deps_key) {
        anyhow::bail!("{}", crate::t!("dep.not_found", &[dep_spec]));
    }

    let deps_item = doc.get_mut(deps_key).unwrap();
    if let Some(deps_table) = deps_item.as_table_mut() {
        // 尝试多种可能的依赖key格式
        let possible_keys = vec![
            dep_spec.to_string(),
            format!("{}::{}", dep_spec, "cats-core"), // 示例，如果是简化格式
        ];

        let mut removed = false;
        for key in possible_keys {
            if deps_table.contains_key(&key) {
                deps_table.remove(&key);
                println!("{}", crate::t!("dep.removed", &[&key]));
                removed = true;
                break;
            }
        }

        // 如果没有找到精确匹配，尝试模糊匹配
        if !removed {
            let mut matching_keys = Vec::new();
            for (key, _) in deps_table.iter() {
                if key.contains(dep_spec) {
                    matching_keys.push(key.to_string());
                }
            }

            if matching_keys.len() == 1 {
                let key = &matching_keys[0];
                deps_table.remove(key);
                println!("{}", crate::t!("dep.removed", &[key]));
                removed = true;
            } else if matching_keys.len() > 1 {
                anyhow::bail!("{}", crate::t!("dep.ambiguous", &[dep_spec, &matching_keys.join(", ")]));
            }
        }

        if !removed {
            anyhow::bail!("{}", crate::t!("dep.not_found", &[dep_spec]));
        }
    }

    std::fs::write(manifest_path, doc.to_string())?;
    Ok(())
}