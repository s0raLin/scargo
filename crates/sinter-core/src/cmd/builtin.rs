//! 内置命令实现
//!
//! 包含所有内置命令的执行逻辑

use crate::cli::Commands;
use crate::cmd::{cmd_new, cmd_init, cmd_test, cmd_workspace};
use crate::build::{run_scala_file, run_single_file_with_deps, setup_bsp};
use crate::deps::add_dependency;
use std::path::PathBuf;

/// 执行内置命令
pub async fn execute_command(command: Commands, cwd: &PathBuf) -> anyhow::Result<()> {
    match command {
        Commands::New { name } => {
            cmd_new(cwd, &name).await?;
        }
        Commands::Init => {
            cmd_init(cwd).await?;
        }
        Commands::Workspace { subcommand } => {
            cmd_workspace(cwd, &subcommand).await?;
        }
        Commands::Build => {
            execute_build(cwd).await?;
        }
        Commands::Run { file, lib } => {
            execute_run(cwd, file, lib).await?;
        }
        Commands::Add { dep } => {
            execute_add(cwd, &dep).await?;
        }
        Commands::Test { file } => {
            cmd_test(cwd, file).await?;
        }
        Commands::Jsp { .. } => {
            // JSP 命令应该由插件系统处理
            unreachable!("JSP command should be handled by plugin system");
        }
    }
    Ok(())
}

/// 执行默认行为（无命令时）
pub async fn execute_default(cwd: &PathBuf) -> anyhow::Result<()> {
    if cwd.join("Sinter.toml").exists() {
        let project = crate::config::load_project(cwd)?;
        let target = crate::config::get_main_file_path(&project);
        if cwd.join(&target).exists() {
            let deps = crate::config::get_dependencies(&project);
            let output = run_single_file_with_deps(cwd, &target, &deps).await?;
            println!("{}", output);
        } else {
            println!(
                "{}",
                crate::i18n::tf("main_file_not_found", &[&target.display().to_string()])
            );
        }
    } else {
        println!("{}", crate::i18n::t("no_command_provided"));
    }
    Ok(())
}

/// 执行构建命令
async fn execute_build(cwd: &PathBuf) -> anyhow::Result<()> {
    if let Some(workspace_root) = crate::config::find_workspace_root(cwd) {
        // Workspace build
        let (root_project, members) = crate::config::load_workspace(&workspace_root)?.unwrap();
        for member in members {
            let member_dir = workspace_root.join(&member.package.name);
            let deps = crate::config::get_dependencies_with_workspace(&member, Some(&root_project));
            // For workspace builds, use target directory relative to workspace root
            let workspace_target_dir = format!(
                "{}/{}",
                root_project.package.target_dir, member.package.name
            );
            crate::build::build::build_with_deps(
                &member_dir,
                &deps,
                &member.package.source_dir,
                &workspace_target_dir,
                &member.package.backend,
                Some(&workspace_root),
            )
            .await?;
            println!("{}", crate::i18n::tf("built_member", &[&member.package.name]));
        }
        println!("{}", crate::i18n::t("workspace_build_succeeded"));
    } else {
        // Single project build
        let project = crate::config::load_project(cwd)?;
        let deps = crate::config::get_dependencies(&project);
        crate::build::build::build_with_deps(
            cwd,
            &deps,
            &project.package.source_dir,
            &project.package.target_dir,
            &project.package.backend,
            None,
        )
        .await?;
        println!(
            "{}",
            crate::i18n::tf("build_succeeded_with_deps", &[&deps.len().to_string()])
        );
    }
    Ok(())
}

/// 执行运行命令
async fn execute_run(cwd: &PathBuf, file: Option<PathBuf>, lib: bool) -> anyhow::Result<()> {
    let workspace_root = crate::config::find_workspace_root(cwd);
    let workspace_root_ref = workspace_root.as_ref();

    // 确定项目配置和目录
    let (project, project_dir) = if let Some(ws_root) = workspace_root_ref {
        // 在 workspace 中，查找成员项目
        if let Some((_ws_proj, members)) = crate::config::load_workspace(ws_root)? {
            let member_name = cwd
                .strip_prefix(ws_root)
                .unwrap()
                .components()
                .next()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap();
            if let Some(member) = members
                .into_iter()
                .find(|m| m.package.name == member_name)
            {
                (member, ws_root.clone().join(member_name))
            } else {
                // 不是成员，作为单个项目处理
                let proj = crate::config::load_project(cwd)?;
                (proj, cwd.clone())
            }
        } else {
            // 实际上不是 workspace，作为单个项目处理
            let proj = crate::config::load_project(cwd)?;
            (proj, cwd.clone())
        }
    } else {
        let proj = crate::config::load_project(cwd)?;
        (proj, cwd.clone())
    };

    // 获取依赖
    let deps = if let Some(ws_root) = workspace_root_ref {
        let ws_proj = crate::config::load_project(ws_root)?;
        crate::config::get_dependencies_with_workspace(&project, Some(&ws_proj))
    } else {
        crate::config::get_dependencies(&project)
    };

    // 设置 BSP 以支持 IDE
    setup_bsp(
        &project_dir,
        &deps,
        &project.package.source_dir,
        &project.package.backend,
        workspace_root_ref.map(|p| p.as_path()),
    )
    .await?;

    let target = file.unwrap_or_else(|| crate::config::get_main_file_path(&project));

    if !project_dir.join(&target).exists() {
        anyhow::bail!("File not found: {}", target.display());
    }

    if lib {
        let _ = run_scala_file(&project_dir, &target, true).await?;
        println!(
            "{}",
            crate::i18n::tf("lib_compiled_only", &[&target.display().to_string()])
        );
    } else {
        let output = run_single_file_with_deps(&project_dir, &target, &deps).await?;
        println!("{}", output);
    }

    Ok(())
}

/// 执行添加依赖命令
async fn execute_add(cwd: &PathBuf, dep: &str) -> anyhow::Result<()> {
    let workspace_root = crate::config::find_workspace_root(cwd);
    let project_dir = workspace_root.unwrap_or(cwd.clone());
    add_dependency(&project_dir, dep).await?;
    Ok(())
}

