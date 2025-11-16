pub mod cli;
pub mod core;
pub mod build;
pub mod deps;
pub mod cmd;
pub mod plugins;
pub mod i18n;
pub mod utils;

pub use cli::{Cli, Commands};
use core::project::Project;
use build::{
    run_scala_file,
    run_single_file_with_deps,
};

use deps::{add_dependency, remove_dependency, update_dependency};

use cmd::cmd_new;
use plugins::PluginManager;
use utils::{calculate_dir_size, format_size};

pub async fn run(cli: Cli, cwd: std::path::PathBuf) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::New { name }) => {
            cmd_new(&cwd, &name).await?;
        }
        Some(Commands::Build) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();
            build::build::build_with_deps(&cwd, &deps, &project.package.source_dir, &project.package.target_dir).await?;
            println!("{}", crate::t!("build.success"));
        }
        Some(Commands::Check) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();
            build::check_with_deps(&cwd, &deps, &project.package.source_dir).await?;
        }
        Some(Commands::Run { file, lib }) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();

            let target = file.unwrap_or_else(|| project.get_main_file_path());

            if !cwd.join(&target).exists() {
                anyhow::bail!("{}", crate::t!("error.file_not_found", &[&target.display().to_string()]));
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
        Some(Commands::Remove { dep }) => {
            remove_dependency(&cwd, &dep).await?;
        }
        Some(Commands::Update { dep }) => {
            update_dependency(&cwd, dep.as_deref()).await?;
        }
        Some(Commands::Plugin { name, args }) => {
            let mut plugin_manager = PluginManager::new()?;
            plugin_manager.load_plugins()?;
            let result = plugin_manager.execute_plugin(&name, &args)?;
            println!("{}", result);
        }
        Some(Commands::Plugins) => {
            let mut plugin_manager = PluginManager::new()?;
            plugin_manager.load_plugins()?;
            let plugins = plugin_manager.list_plugins();
            if plugins.is_empty() {
                println!("{}", crate::t!("plugins.none"));
            } else {
                println!("{}", crate::t!("plugins.available"));
                for plugin in plugins {
                    println!("  {} v{} - {}", plugin.name, plugin.version, plugin.description);
                    println!("    Commands: {}", plugin.commands.join(", "));
                }
            }
        }
        Some(Commands::Test { test }) => {
            let project = Project::load(&cwd)?;
            let deps = project.get_dependencies();
            build::run_tests(&cwd, &deps, &project.package.source_dir, test.as_deref()).await?;
        }
        Some(Commands::Clean) => {
            let project = Project::load(&cwd)?;
            let target_dir = cwd.join(&project.package.target_dir);
            if target_dir.exists() {
                let size = calculate_dir_size(&target_dir).await?;
                let dep_count = project.dependencies.len();
                tokio::fs::remove_dir_all(&target_dir).await?;
                println!("{}", crate::t!("clean.success_with_details", &[&format_size(size), &dep_count.to_string()]));
            } else {
                println!("{}", crate::t!("clean.none"));
            }
        }
        Some(Commands::Info) => {
            let project = Project::load(&cwd)?;
            println!("{}", crate::t!("project.name", &[&project.package.name]));
            println!("{}", crate::t!("project.version", &[&project.package.version]));
            println!("{}", crate::t!("project.scala_version", &[&project.package.scala_version]));
            println!("{}", crate::t!("project.source_dir", &[&project.package.source_dir]));
            println!("{}", crate::t!("project.target_dir", &[&project.package.target_dir]));
            println!("{}", crate::t!("project.main_class", &[project.package.main.as_deref().unwrap_or("Main")]));
            println!("{}", crate::t!("project.dependencies", &[&project.dependencies.len().to_string()]));
            for (dep, version) in &project.dependencies {
                println!("  {}: {}", dep, version);
            }
        }
        Some(Commands::Dev) => {
            build::start_hot_reload(&cwd).await?;
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
                    println!("{}", crate::t!("error.main_file_not_found", &[&target.display().to_string()]));
                }
            } else {
                println!("{}", crate::t!("error.no_command"));
            }
        }
    }

    Ok(())
}