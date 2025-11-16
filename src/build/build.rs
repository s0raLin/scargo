// src/build.rs
use crate::deps::deps::Dependency;
use crate::build::cache::BuildCache;
use tokio::process::Command;
use std::path::Path;

pub async fn build_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str, target_dir: &str) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let target_path = proj_dir.join(target_dir);

    // 初始化缓存
    let mut cache = BuildCache::new(proj_dir);
    cache.init().await?;

    // 计算依赖字符串用于缓存key
    let dep_strings: Vec<String> = deps.iter().map(|d| d.coord()).collect();

    // 计算源代码哈希
    let source_hash = cache.calculate_source_hash(&source_path, &dep_strings).await?;

    // 检查缓存
    if cache.is_cache_hit(&source_hash) {
        println!("{}", crate::t!("build.cache_hit"));
        cache.restore_from_cache(&source_hash, &target_path).await?;
        println!("{}", crate::t!("build.success_cached"));
        return Ok(());
    }

    // 执行构建
    let mut cmd = Command::new("scala-cli");
    cmd.arg("compile").arg("-d").arg(&target_path).arg(&source_path).current_dir(proj_dir);

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Build failed with dependencies");
    }

    // 保存到缓存
    cache.save_to_cache(&source_hash, &target_path).await?;
    println!("{}", crate::t!("build.success_and_cached"));

    Ok(())
}

pub async fn check_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);

    let mut cmd = Command::new("scala-cli");
    cmd.arg("compile").arg(&source_path).current_dir(proj_dir);

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Check failed");
    }

    println!("{}", crate::t!("check.success"));
    Ok(())
}

pub async fn run_tests(proj_dir: &Path, deps: &[Dependency], source_dir: &str, test_filter: Option<&str>) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let mut cmd = Command::new("scala-cli");
    cmd.arg("test").arg(&source_path);

    if let Some(filter) = test_filter {
        cmd.arg("--test-only").arg(format!("*{}*", filter));
    }

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Tests failed");
    }

    println!("{}", crate::t!("test.passed"));
    Ok(())
}
