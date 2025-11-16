// src/build.rs
use crate::deps::Dependency;
use tokio::process::Command;
use std::path::Path;

pub async fn build_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let mut cmd = Command::new("scala-cli");
    cmd.arg("compile").arg(&source_path).current_dir(proj_dir);

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Build failed with dependencies");
    }
    Ok(())
}

pub async fn run_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str) -> anyhow::Result<String> {
    let source_path = proj_dir.join(source_dir);
    let mut cmd = Command::new("scala-cli");
    cmd.arg("run").arg(&source_path).current_dir(proj_dir);

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let output = cmd.output().await?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Run failed: {}", err);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}