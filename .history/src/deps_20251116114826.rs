// src/deps.rs
use anyhow::Context;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Deserialize, Debug, Clone)]
pub struct Dependency {
    pub group: String,
    pub artifact: String,
    pub version: String,
}

impl Dependency {
    pub fn from_toml_key(key: &str, version: &str) -> Self {
        let parts: Vec<&str> = key.split("::").collect();
        let (group, artifact) = if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ( "".to_string(), key.to_string() )
        };
        Self {
            group,
            artifact,
            version: version.to_string(),
        }
    }

    // 生成 Maven 坐标：group:artifact:version
    pub fn coord(&self) -> String {
        format!("{}:{}:{}", self.group, self.artifact, self.version)
    }

    // 生成 coursier 风格参数
    pub fn coursier_arg(&self) -> String {
        format!("{}:{}", self.group.replace(".", "/"), self.coord())
    }
}

// 从 Scargo.toml 解析依赖
pub fn parse_dependencies(manifest: &toml::Value) -> anyhow::Result<Vec<Dependency>> {
    let deps_table = manifest
        .get("dependencies")
        .and_then(|v| v.as_table())
        .ok_or_else(|| anyhow::anyhow!("No [dependencies] section"))?;

    let mut deps = Vec::new();
    for (key, value) in deps_table {
        let version = value
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Dependency version must be string: {}", key))?;
        deps.push(Dependency::from_toml_key(key, version));
    }
    Ok(deps)
}

// 使用 scala-cli 自动解析 + 下载依赖
pub async fn resolve_with_scala_cli(
    proj_dir: &Path,
    deps: &[Dependency],
) -> anyhow::Result<()> {
    let mut cmd = Command::new("scala-cli");

    cmd.arg("compile")
        .arg(proj_dir)
        .current_dir(proj_dir);

    // 添加每个依赖
    for dep in deps {
        cmd.arg("--dependency").arg(&dep.coord());
    }

    let status = cmd
        .status()
        .await
        .context("Failed to run scala-cli with dependencies")?;

    if !status.success() {
        anyhow::bail!("Dependency resolution failed");
    }

    Ok(())
}