// src/build.rs
use crate::deps::deps::Dependency;
use tokio::process::Command;
use std::path::Path;

pub async fn build_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str, target_dir: &str) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let target_path = proj_dir.join(target_dir);
    let mut cmd = Command::new("scala-cli");
    cmd.arg("compile").arg("-d").arg(&target_path).arg(&source_path).current_dir(proj_dir);

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Build failed with dependencies");
    }
    Ok(())
}
