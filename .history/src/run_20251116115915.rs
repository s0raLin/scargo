use std::path::Path;

use tokio::process::Command;

// src/run.rs
use anyhow::Context;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Debug, PartialEq)]
pub enum RunMode {
    App,   // 有 main 或 extends App
    Lib,   // 无入口 → 只编译
}

pub struct RunResult {
    pub mode: RunMode,
    pub output: String,
}

pub async fn run_scala_file(
    proj_dir: &Path,
    file_path: &Path,
    force_lib: bool,
) -> anyhow::Result<RunResult> {
    let abs_file = proj_dir.join(file_path);

    // 1. 读取文件内容，检测是否有 main
    let content = tokio::fs::read_to_string(&abs_file).await?;
    let has_main = content.contains("def main(") || content.contains("extends App");
    let mode = if force_lib || !has_main {
        RunMode::Lib
    } else {
        RunMode::App
    };

    // 2. 调用 scala-cli
    let mut cmd = Command::new("scala-cli");

    if mode == RunMode::Lib {
        cmd.arg("compile").arg(&abs_file);
    } else {
        cmd.arg("run").arg(&abs_file);
    }

    cmd.current_dir(proj_dir);

    let output = cmd
        .output()
        .await
        .context("failed to execute scala-cli")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let full_output = if !stderr.is_empty() {
        format!("{stdout}\n{stderr}")
    } else {
        stdout.to_string()
    };

    if !output.status.success() {
        anyhow::bail!("scala-cli failed: {}", full_output);
    }

    Ok(RunResult {
        mode,
        output: full_output.trim().to_string(),
    })
}


// src/run.rs (新增函数)
use crate::deps::Dependency;

pub async fn run_single_file_with_deps(
    proj_dir: &Path,
    file_path: &Path,
    deps: &[Dependency],
) -> anyhow::Result<String> {
    let abs_file = proj_dir.join(file_path);
    let content = tokio::fs::read_to_string(&abs_file).await?;
    let has_main = content.contains("def main") || content.contains("extends App");

    let mut cmd = Command::new("scala-cli");
    cmd.current_dir(proj_dir);

    if has_main {
        cmd.arg("run").arg(&abs_file);
    } else {
        cmd.arg("compile").arg(&abs_file);
    }

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let output = cmd.output().await?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed: {}", err);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}