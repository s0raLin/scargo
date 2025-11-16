// src/deps.rs
use serde::Deserialize;

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
