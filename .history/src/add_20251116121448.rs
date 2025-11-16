// src/add.rs
use anyhow::Context;
use std::path::Path;
use toml_edit::{Document, Item, Table};
there is a method `as_table` with a similar name: `as_table`rustcE0599
add.rs(29, 10): original diagnostic
no method named `as_table_mut` found for struct `Document<std::string::String>` in the current scoperustcClick for full compiler diagnostic
add.rs(28, 21):
add.rs(29, 10): there is a method `as_table` with a similar name: `as_table`

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