pub mod cli;
pub mod core;
pub mod build;
pub mod deps;
pub mod cmd;

pub use cli::{Cli, Commands};
use core::project::Project;
use build::{
    run_scala_file,
    run_single_file_with_deps,
};
use deps::add_dependency;

use cmd::cmd_new;

pub async fn run(cli: Cli, cwd: std::path::PathBuf) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::New { name }) => {
            cmd_new(&cwd, &name).await?;
        }
        Some(Commands::Build) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();
            build::build::build_with_deps(&cwd, &deps, &project.package.source_dir, &project.package.target_dir).await?;
            println!("Build succeeded with {} dependencies", deps.len());
        }
        Some(Commands::Run { file, lib }) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();

            let target = file.unwrap_or_else(|| project.get_main_file_path());

            if !cwd.join(&target).exists() {
                anyhow::bail!("File not found: {}", target.display());
            }

            if lib {
                let _ = run_scala_file(&cwd, &target, true).await?;
                println!("lib: {} (compiled only)", target.display());
            } else {
                let output = run_single_file_with_deps(&cwd, &target, &deps).await?;
                println!("{}", output);
            }
        },
        Some(Commands::Add { dep }) => {
            add_dependency(&cwd, &dep).await?;
        }
        None => {
            if cwd.join("Scargo.toml").exists() {
                let project = Project::load(&cwd)?;
                let target = project.get_main_file_path();
                if cwd.join(&target).exists() {
                    let deps = project.get_dependencies();
                    let output = run_single_file_with_deps(&cwd, &target, &deps).await?;
                    println!("{}", output);
                } else {
                    println!("Main file not found: {}", target.display());
                }
            } else {
                println!("No command provided. Use --help for usage.");
            }
        }
    }

    Ok(())
}