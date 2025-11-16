// src/project.rs
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Project {
    pub package: Package,
    pub dependencies: Option<toml::value::Table>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub main: Option<String>,
}

impl Project {
    pub fn load(dir: &PathBuf) -> anyhow::Result<Self> {
        let manifest = dir.join("Scargo.toml");
        let content = std::fs::read_to_string(manifest)?;
        let proj: Project = toml::from_str(&content)?;
        Ok(proj)
    }
}