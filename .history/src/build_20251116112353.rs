// src/build.rs
use anyhow::Context;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn scala_cli_build(proj_dir: &PathBuf) -> anyhow::Result<()> {
    let status = Command::new("scala-cli")
        .arg("compile")
        .arg(proj_dir)
        .status()
        .await
        .context("failed to spawn scala-cli")?;

    if !status.success() {
        anyhow::bail!("scala-cli compile failed");
    }
    Ok(())
}

pub async fn scala_cli_run(proj_dir: &PathBuf) -> anyhow::Result<()> {
    let status = Command::new("scala-cli")
        .arg("run")
        .arg(proj_dir)
        .status()
        .await
        .context("failed to spawn scala-cli")?;

    if !status.success() {
        anyhow::bail!("scala-cli run failed");
    }
    Ok(())
}