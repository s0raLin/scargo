// src/main.rs
mod cli;
mod project;
mod build;

use clap::Parser;
use cli::{Cli, Commands};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cwd = env::current_dir()?;
    match cli.command {
        Commands::New { name } => cmd_new(&cwd, &name).await?,
        Commands::Build => cmd_build(&cwd).await?,
        Commands::Run => cmd_run(&cwd).await?,
    }
    Ok(())
}

// inside main.rs or separate file
async fn cmd_new(cwd: &PathBuf, name: &str) -> anyhow::Result<()> {
    let proj_dir = cwd.join(name);
    tokio::fs::create_dir_all(proj_dir.join("src/main/scala")).await?;

    // Scargo.toml
    let manifest = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
main = "Main"

[dependencies]
"#,
        name
    );
    tokio::fs::write(proj_dir.join("Scargo.toml"), manifest).await?;

    // Hello world
    let code = r#"object Main extends App {
  println("Hello from scargo!")
}
"#;
    tokio::fs::write(
        proj_dir.join("src/main/scala/Main.scala"),
        code,
    )
    .await?;

    println!("Created project `{}`", name);
    Ok(())
}


use crate::build::{scala_cli_build, scala_cli_run};

async fn cmd_build(cwd: &PathBuf) -> anyhow::Result<()> {
    scala_cli_build(cwd).await?;
    println!("Build succeeded → target/");
    Ok(())
}

// src/main.rs (只展示 run 部分)
use crate::run::{run_scala_file, RunMode};
use std::path::PathBuf;

async fn cmd_run(cwd: &PathBuf) -> anyhow::Result<()> {
    let cli = Cli::parse();
    let Commands::Run { file, lib } = cli.command.unwrap() else {
        unreachable!()
    };

    let target_file = if let Some(f) = file {
        f
    } else {
        // 默认找 Main.scala
        PathBuf::from("src/main/scala/Main.scala")
    };

    // 检查文件是否存在
    if !cwd.join(&target_file).exists() {
        anyhow::bail!("File not found: {}", target_file.display());
    }

    let result = run_scala_file(cwd, &target_file, lib).await?;

    match result.mode {
        RunMode::App => {
            if !result.output.is_empty() {
                println!("{}", result.output);
            }
        }
        RunMode::Lib => {
            println!("lib: {} (compiled only)", target_file.display());
        }
    }

    Ok(())
}