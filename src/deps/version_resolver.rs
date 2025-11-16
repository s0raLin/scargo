use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use semver::Version;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

/// Maven Central API响应
#[derive(Deserialize)]
struct MavenResponse {
    response: ResponseData,
}

#[derive(Deserialize)]
struct ResponseData {
    docs: Vec<MavenDoc>,
}

#[derive(Deserialize)]
struct MavenDoc {
    g: String, // groupId
    a: String, // artifactId
    v: String, // version
    timestamp: i64,
}

/// 版本解析器
pub struct VersionResolver {
    cache: HashMap<String, Vec<String>>,
    client: reqwest::Client,
}

impl VersionResolver {
    pub fn new() -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("scargo/0.1.0 (https://github.com/cangli/scargo)"),
        );

        Self {
            cache: HashMap::new(),
            client: reqwest::Client::builder()
                .default_headers(headers)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// 解析版本，支持latest、stable等关键字
    pub async fn resolve_version(&mut self, group: &str, artifact: &str, version_spec: &str) -> Result<String> {
        match version_spec {
            "latest" => self.get_latest_version(group, artifact).await,
            "stable" => self.get_stable_version(group, artifact).await,
            v if v.starts_with('[') && v.ends_with(']') => {
                // 版本范围，如 [1.0,2.0)
                self.resolve_version_range(group, artifact, v).await
            }
            v if v.contains(',') => {
                // 多个版本选项，用逗号分隔
                self.resolve_from_options(group, artifact, v).await
            }
            _ => Ok(version_spec.to_string()), // 直接使用指定版本
        }
    }

    /// 获取最新版本
    pub async fn get_latest_version(&mut self, group: &str, artifact: &str) -> Result<String> {
        let versions = self.get_available_versions(group, artifact).await?;
        versions.into_iter().max_by(|a, b| {
            Version::parse(a).unwrap_or(Version::new(0, 0, 0))
                .cmp(&Version::parse(b).unwrap_or(Version::new(0, 0, 0)))
        })
        .context("No versions found")
    }

    /// 获取稳定版本（非快照、非预发布）
    async fn get_stable_version(&mut self, group: &str, artifact: &str) -> Result<String> {
        let versions = self.get_available_versions(group, artifact).await?;
        let stable_versions: Vec<String> = versions.into_iter()
            .filter(|v| {
                !v.contains("SNAPSHOT") && !v.contains("-alpha") &&
                !v.contains("-beta") && !v.contains("-rc") &&
                Version::parse(v).is_ok()
            })
            .collect();

        stable_versions.into_iter().max_by(|a, b| {
            Version::parse(a).unwrap().cmp(&Version::parse(b).unwrap())
        })
        .context("No stable versions found")
    }

    /// 解析版本范围
    async fn resolve_version_range(&mut self, group: &str, artifact: &str, range_spec: &str) -> Result<String> {
        let range_str = &range_spec[1..range_spec.len()-1]; // 移除方括号
        let versions = self.get_available_versions(group, artifact).await?;

        // 简单的范围解析，实际应该使用semver range
        let parts: Vec<&str> = range_str.split(',').collect();
        if parts.len() == 2 {
            let min = parts[0].trim();
            let max = parts[1].trim();

            for version in versions {
                if let Ok(v) = Version::parse(&version) {
                    let min_ok = min.is_empty() || v >= Version::parse(min)?;
                    let max_ok = max.is_empty() || v < Version::parse(max)?;
                    if min_ok && max_ok {
                        return Ok(version);
                    }
                }
            }
        }

        anyhow::bail!("No version found in range {}", range_spec);
    }

    /// 从选项中选择版本
    async fn resolve_from_options(&mut self, group: &str, artifact: &str, options: &str) -> Result<String> {
        let available_versions = self.get_available_versions(group, artifact).await?;
        let requested_versions: Vec<&str> = options.split(',').map(|s| s.trim()).collect();

        for requested in requested_versions {
            if available_versions.contains(&requested.to_string()) {
                return Ok(requested.to_string());
            }
        }

        // 如果没有找到精确匹配，返回最新版本
        self.get_latest_version(group, artifact).await
    }

    /// 获取可用的版本列表
    async fn get_available_versions(&mut self, group: &str, artifact: &str) -> Result<Vec<String>> {
        let cache_key = format!("{}:{}", group, artifact);

        if let Some(versions) = self.cache.get(&cache_key) {
            return Ok(versions.clone());
        }

        let url = format!(
            "https://search.maven.org/solrsearch/select?q=g:{}+AND+a:{}&core=gav&rows=50&wt=json&start=0",
            group, artifact
        );

        let mut response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            if response.status() == reqwest::StatusCode::FORBIDDEN {
                // 重试一次，403 可能是临时的
                sleep(Duration::from_millis(1000)).await;
                response = self.client.get(&url).send().await?;
                if !response.status().is_success() {
                    anyhow::bail!("Failed to fetch versions from Maven Central: HTTP {}", response.status());
                }
            } else {
                anyhow::bail!("Failed to fetch versions from Maven Central: HTTP {}", response.status());
            }
        }
        let maven_response: MavenResponse = response.json().await
            .context("Failed to parse Maven Central response as JSON")?;

        let versions: Vec<String> = maven_response.response.docs
            .into_iter()
            .map(|doc| doc.v)
            .collect();

        if versions.is_empty() {
            anyhow::bail!("No versions found for {}:{}", group, artifact);
        }

        self.cache.insert(cache_key, versions.clone());
        Ok(versions)
    }
}

/// 增强的依赖解析
pub fn parse_dependency_spec(spec: &str) -> Result<(String, String, String)> {
    // 支持格式：
    // group:artifact:version
    // group::artifact:version
    // group:artifact@scala_version:version
    // group::artifact@scala_version:version
    // group::artifact (无版本，用于更新)

    let parts: Vec<&str> = spec.split(':').collect();
    match parts.len() {
        2 => {
            // group:artifact
            let group = parts[0].to_string();
            let artifact = parts[1].to_string();
            let version = String::new();
            Ok((group, artifact, version))
        }
        3 => {
            let group = parts[0].to_string();
            if parts[1].is_empty() {
                // group::artifact
                let artifact = parts[2].to_string();
                let version = String::new();
                Ok((group, artifact, version))
            } else {
                // group:artifact:version
                let artifact = parts[1].to_string();
                let version = parts[2].to_string();
                Ok((group, artifact, version))
            }
        }
        4 => {
            let group = parts[0].to_string();
            let artifact = format!("{}_{}", parts[1], parts[2]);
            let version = parts[3].to_string();
            Ok((group, artifact, version))
        }
        _ => anyhow::bail!("Invalid dependency format: {}", spec),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency_spec() {
        assert_eq!(
            parse_dependency_spec("org.scalatest:scalatest_2.13:3.2.15").unwrap(),
            ("org.scalatest".to_string(), "scalatest_2.13".to_string(), "3.2.15".to_string())
        );

        assert_eq!(
            parse_dependency_spec("com.typesafe:config:1.4.2").unwrap(),
            ("com.typesafe".to_string(), "config".to_string(), "1.4.2".to_string())
        );

        // 测试 group::artifact 格式（无版本）
        assert_eq!(
            parse_dependency_spec("ch.qos.logback::_logback-classic").unwrap(),
            ("ch.qos.logback".to_string(), "_logback-classic".to_string(), "".to_string())
        );

        // 测试 group:artifact 格式（无版本）
        assert_eq!(
            parse_dependency_spec("com.example:library").unwrap(),
            ("com.example".to_string(), "library".to_string(), "".to_string())
        );
    }
}