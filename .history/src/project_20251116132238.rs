

use std::path::{Path, PathBuf};

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
    #[serde(default = "default_scala_version")]
    pub scala_version: String,
    #[serde(default = "default_source_dir")]
    pub source_dir: String,
    #[serde(default = "default_target_dir")]
    pub target_dir: String,
}

fn default_scala_version() -> String {
    "2.13".to_string()
}

fn default_source_dir() -> String {
    "src/main/scala".to_string()
}

fn default_target_dir() -> String {
    "build".to_string()
}

impl Project {
    pub fn load(dir: &Path) -> anyhow::Result<Self> {
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

    pub fn get_main_file_path(&self) -> PathBuf {
        let main_class = self.package.main.as_deref().unwrap_or("Main");
        PathBuf::from(&self.package.source_dir).join(format!("{}.scala", main_class))
    }
}