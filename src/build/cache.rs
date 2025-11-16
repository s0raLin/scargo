use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::fs;
use anyhow::Result;

/// 构建缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    hash: String,
    timestamp: u64,
    outputs: Vec<PathBuf>,
}

/// 构建缓存管理器
pub struct BuildCache {
    cache_dir: PathBuf,
    cache_index: HashMap<String, CacheEntry>,
}

impl BuildCache {
    pub fn new(project_dir: &Path) -> Self {
        let cache_dir = project_dir.join("target").join(".scargo-cache");
        Self {
            cache_dir: cache_dir.clone(),
            cache_index: HashMap::new(),
        }
    }

    /// 初始化缓存
    pub async fn init(&mut self) -> Result<()> {
        fs::create_dir_all(&self.cache_dir).await?;

        let index_path = self.cache_dir.join("index.json");
        if index_path.exists() {
            let content = fs::read_to_string(&index_path).await?;
            self.cache_index = serde_json::from_str(&content)?;
        }

        Ok(())
    }

    /// 计算源文件的哈希
    pub async fn calculate_source_hash(&self, source_dir: &Path, dependencies: &[String]) -> Result<String> {
        let mut hasher = Sha256::new();

        // 哈希所有源文件
        let mut entries = vec![];
        if source_dir.exists() {
            collect_files(source_dir, &mut entries)?;
        }

        entries.sort(); // 确保一致的顺序

        for entry in entries {
            if let Ok(content) = fs::read(&entry).await {
                hasher.update(&content);
            }
        }

        // 哈希依赖列表
        for dep in dependencies {
            hasher.update(dep.as_bytes());
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// 检查缓存是否命中
    pub fn is_cache_hit(&self, hash: &str) -> bool {
        self.cache_index.contains_key(hash)
    }

    /// 从缓存恢复输出
    pub async fn restore_from_cache(&self, hash: &str, target_dir: &Path) -> Result<()> {
        if let Some(entry) = self.cache_index.get(hash) {
            for output in &entry.outputs {
                let source = self.cache_dir.join(hash).join(output);
                let dest = target_dir.join(output);

                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).await?;
                }

                if source.exists() {
                    fs::copy(&source, &dest).await?;
                }
            }
        }
        Ok(())
    }

    /// 保存构建结果到缓存
    pub async fn save_to_cache(&mut self, hash: &str, target_dir: &Path) -> Result<()> {
        let cache_entry_dir = self.cache_dir.join(hash);
        fs::create_dir_all(&cache_entry_dir).await?;

        let mut outputs = vec![];

        // 复制所有输出文件到缓存
        if target_dir.exists() {
            let mut entries = vec![];
            collect_files(target_dir, &mut entries)?;

            for entry in entries {
                if let Ok(relative) = entry.strip_prefix(target_dir) {
                    let cache_path = cache_entry_dir.join(relative);
                    if let Some(parent) = cache_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::copy(&entry, &cache_path)?;
                    outputs.push(relative.to_path_buf());
                }
            }
        }

        // 更新索引
        let entry = CacheEntry {
            hash: hash.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            outputs,
        };

        self.cache_index.insert(hash.to_string(), entry);

        // 保存索引
        let index_content = serde_json::to_string_pretty(&self.cache_index)?;
        fs::write(self.cache_dir.join("index.json"), index_content).await?;

        Ok(())
    }

    /// 清理过期缓存
    pub async fn clean_expired(&mut self, max_age_days: u64) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let max_age_secs = max_age_days * 24 * 60 * 60;
        let mut to_remove = vec![];

        for (hash, entry) in &self.cache_index {
            if now - entry.timestamp > max_age_secs {
                to_remove.push(hash.clone());
            }
        }

        for hash in to_remove {
            if let Some(entry) = self.cache_index.remove(&hash) {
                let cache_entry_dir = self.cache_dir.join(&entry.hash);
                if cache_entry_dir.exists() {
                    fs::remove_dir_all(&cache_entry_dir).await?;
                }
            }
        }

        // 保存更新后的索引
        let index_content = serde_json::to_string_pretty(&self.cache_index)?;
        fs::write(self.cache_dir.join("index.json"), index_content).await?;

        Ok(())
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> CacheStats {
        let total_entries = self.cache_index.len();
        let total_size = self.cache_index.values()
            .map(|entry| entry.outputs.len())
            .sum();

        CacheStats {
            total_entries,
            total_outputs: total_size,
        }
    }
}

/// 缓存统计信息
#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_outputs: usize,
}

/// 递归收集目录中的所有文件
fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            collect_files(&path, files)?;
        }
    }
    Ok(())
}