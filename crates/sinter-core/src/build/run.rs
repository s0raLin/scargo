// src/run.rs
use std::path::Path;

use tokio::process::Command;

/// 检测Scala文件是否包含main方法
fn has_main_method(content: &str) -> bool {
    content.contains("def main(") || content.contains("extends App")
}

// src/run.rs
use anyhow::Context;


#[derive(Debug, PartialEq)]
pub enum RunMode {
    App,   // 有 main 或 extends App
    Lib,   // 无入口 -> 只编译
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
    let has_main = has_main_method(&content);
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
use crate::deps::deps::Dependency;

pub async fn run_single_file_with_deps(
    proj_dir: &Path,
    file_path: &Path,
    deps: &[Dependency],
) -> anyhow::Result<String> {
    let abs_file = proj_dir.join(file_path);
    let content = tokio::fs::read_to_string(&abs_file).await?;
    let has_main = has_main_method(&content);

    // 使用抽象的依赖管理器
    let dep_manager = crate::deps::default_dependency_manager().await;
    dep_manager.prepare_dependencies(deps, &proj_dir.join("target")).await?;

    let mut cmd = Command::new("scala-cli");
    cmd.current_dir(proj_dir);

    if has_main {
        cmd.arg("run").arg(&abs_file);
    } else {
        cmd.arg("compile").arg(&abs_file);
    }

    // 添加依赖参数
    let dep_args = dep_manager.get_run_args(deps);
    cmd.args(&dep_args);

    let output = cmd.output().await?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed: {}", err);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}


