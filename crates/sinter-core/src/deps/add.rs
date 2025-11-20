use std::path::Path;
use reqwest;
use serde_json;

pub async fn add_dependency(project_dir: &Path, dep_spec: &str) -> anyhow::Result<()> {
    let project = crate::config::load_project(project_dir)?;
    let manifest_path = project_dir.join("project.toml");

    let (artifact, scala_ver, version) = parse_dep_spec(dep_spec, &project.package.scala_version).await?;

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

    crate::config::add_dependency_to_manifest(&manifest_path, &full_key, &version)?;
    println!("{}", crate::i18n::tf("added_dependency", &[&full_key, &version]));
    Ok(())
}

async fn parse_dep_spec(spec: &str, default_scala_version: &str) -> anyhow::Result<(String, String, String)> {
    let parts: Vec<&str> = spec.split(':').collect();
    let (artifact_part, version) = if parts.len() == 2 {
        (parts[0], parts[1].to_string())
    } else {
        (parts[0], "latest".to_string())
    };

    let artifact_parts: Vec<&str> = artifact_part.split('@').collect();
    let (mut artifact, scala_ver) = if artifact_parts.len() == 2 {
        (artifact_parts[0].to_string(), artifact_parts[1])
    } else {
        (artifact_parts[0].to_string(), default_scala_version)
    };

    // 映射常见库到正确的 artifact id
    if artifact == "cats" {
        artifact = "cats-core".to_string();
    }
    // 可以添加更多映射

    let version = if version == "latest" {
        // 查询 Maven Central 获取最新版本
        let key = if artifact.contains("::") {
            artifact.to_string()
        } else {
            format!("org.typelevel::{}", artifact)
        };
        let group_artifact: Vec<&str> = key.split("::").collect();
        if group_artifact.len() != 2 {
            anyhow::bail!("Invalid artifact format: {}", key);
        }
        let group_id = group_artifact[0];
        let artifact_id = if !scala_ver.is_empty() && scala_ver != "latest" {
            format!("{}_{}", group_artifact[1], scala_ver)
        } else {
            group_artifact[1].to_string()
        };
        get_latest_version(group_id, &artifact_id).await?
    } else {
        version
    };

    Ok((artifact.to_string(), scala_ver.to_string(), version))
}

async fn get_latest_version(group_id: &str, artifact_id: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let json: serde_json::Value = client
        .get("https://search.maven.org/solrsearch/select")
        .header("User-Agent", "sinter/0.1.0")
        .query(&[
            ("q", format!("g:{} AND a:{}", group_id, artifact_id)),
            ("rows", "1".to_string()),
            ("wt", "json".to_string()),
        ])
        .send()
        .await?
        .json()
        .await?;
    if let Some(docs) = json["response"]["docs"].as_array() {
        if let Some(first) = docs.first() {
            if let Some(v) = first["latestVersion"].as_str() {
                return Ok(v.to_string());
            }
        }
    }
    anyhow::bail!("No version found for {}:{}", group_id, artifact_id);
}