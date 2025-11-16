

use serde::Deserialize;

// src/project.rs
use crate::deps::Dependency;

#[derive(Deserialize, Debug)]
pub struct Project {
    pub package: Package,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub main: Option<String>,
}

impl Project {
    pub fn load(dir: &PathBuf) -> anyhow::Result<Self> {
        let manifest_path = dir.join("Scargo.toml");
        let content = std::fs::read_to_string(&manifest_path)?;
        let proj: Project = toml::from_str(&content)?;
        Ok(proj)
    }

    pub fn get_dependencies(&self) -> Vec<Dependency> {
        self.dependencies
            .iter()
            .map(|(k, v)| Dependency::from_toml_key(k, v))
            .collect()
    }
}