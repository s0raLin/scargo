// // src/main.rs
// mod cli;
// mod project;
// mod build;
// mod run;
// mod deps;

// use clap::Parser;
// use cli::{Cli, Commands};
// use std::env;
// use std::path::PathBuf;

// use crate::project::Project;
// use crate::run::{RunMode, run_scala_file, run_single_file_with_deps};


// src/main.rs
mod cli;
mod project;
mod build;
mod run;
mod deps;
mod add;

use clap::Parser;
use cli::{Cli, Commands};
use std::env;
use std::path::PathBuf;
use crate::project::Project;
use crate::run::{
    RunMode,
    run_scala_file,
    run_single_file_with_deps,
    run_with_deps,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = env::current_dir()?;

    match cli.command {
        Some(Commands::New { name }) => {
            cmd_new(&cwd, &name).await?;
        }
        Some(Commands::Build) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();
            build::build_with_deps(&cwd, &deps).await?;
            println!("Build succeeded with {} dependencies", deps.len());
        }
        Some(Commands::Run { file, lib }) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();

            let target = file.unwrap_or_else(|| PathBuf::from("src/main/scala/Main.scala"));

            if !cwd.join(&target).exists() {
                anyhow::bail!("File not found: {}", target.display());
            }

            if lib {
                let result = run_scala_file(&cwd, &target, true).await?;
                println!("lib: {} (compiled only)", target.display());
            } else {
                let output = run_single_file_with_deps(&cwd, &target, &deps).await?;
                println!("{}", output);
            }
        },
        // 在 match 中
        Some(Commands::Add { dep }) => {
            add::add_dependency(&cwd, &dep)?;
        }
        None => {
            let target = PathBuf::from("src/main/scala/Main.scala");
            if cwd.join("Scargo.toml").exists() && cwd.join(&target).exists() {
                let project = Project::load(&cwd)?;
                let deps = project.get_dependencies();
                let output = run_single_file_with_deps(&cwd, &target, &deps).await?;
                println!("{}", output);
            } else {
                println!("No command provided. Use --help for usage.");
            }
        }
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

// 在 cmd_build 和 cmd_run 中使用
async fn cmd_build(cwd: &PathBuf) -> anyhow::Result<()> {
    let project = Project::load(cwd)?;
    let deps = project.get_dependencies();
    build::build_with_deps(cwd, &deps).await?;
    println!("Build succeeded with {} dependencies", deps.len());
    Ok(())
}

// async fn cmd_run(cwd: &PathBuf) -> anyhow::Result<()> {
//     let project = Project::load(cwd)?;
//     let deps = project.get_dependencies();

//     // 如果是 run 单个文件，也带上依赖
//     let result = if let Some(Commands::Run { file, .. }) = &cli.command {
//         // 单文件运行 + 依赖
//         let file_path = file.as_ref().unwrap_or(&PathBuf::from("src/main/scala/Main.scala"));
//         run_single_file_with_deps(cwd, file_path, &deps).await?
//     } else {
//         run::run_with_deps(cwd, &deps).await?
//     };

//     println!("{}", result);
//     Ok(())
// }