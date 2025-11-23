//! 内置命令实现
//!
//! 包含所有内置命令的执行逻辑

use crate::cli::{Commands, commands::{cmd_new, cmd_init, cmd_test, cmd_workspace}};
use crate::build::{run_scala_file, run_single_file_with_deps};
use crate::ide::setup_bsp;
use crate::deps::add_dependency;
use std::path::PathBuf;
use crate::config::loader;

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
        Commands::Add { deps } => {
            execute_add(cwd, &deps).await?;
        }
        Commands::Test { file } => {
            cmd_test(cwd, file).await?;
        }
        Commands::Jsp { name } => {
            // JSP 命令应该由插件系统处理
            anyhow::bail!("JSP command '{}' requires the JSP plugin to be loaded", name);
        }
    }
    Ok(())
}

/// 执行默认行为（无命令时）
pub async fn execute_default(cwd: &PathBuf) -> anyhow::Result<()> {
    if cwd.join("project.toml").exists() {
        let project = loader::load_project(cwd)?;
        let target = project.get_main_file_path();
        if cwd.join(&target).exists() {
            let deps = crate::dependency::get_dependencies(&project);
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
    if let Ok(project) = loader::load_project(cwd) {
        if project.workspace.is_some() {
            // Workspace build - build all members
            let (root_project, members) = loader::load_workspace(cwd)?.ok_or_else(|| anyhow::anyhow!("Failed to load workspace configuration"))?;
            let mut all_deps = Vec::new();
            let mut source_dirs = Vec::new();
            let mut backend = None;
            for member in members.iter() {
                let member_dir = cwd.join(member.get_name());
                let transitive_deps = crate::dependency::get_transitive_dependencies_with_workspace(&member, Some(&root_project), &member_dir).await?;
                all_deps.extend(transitive_deps.clone());
                source_dirs.push((member.get_name().to_string(), member.get_source_dir().to_string()));
                if backend.is_none() {
                    backend = Some(member.get_backend().to_string());
                }
                // For workspace builds, use target directory relative to workspace root
                let workspace_target_dir = format!(
                    "{}/{}",
                    root_project.get_target_dir(), member.get_name()
                );
                crate::build::build_with_deps(
                    &member_dir,
                    &transitive_deps,
                    member.get_source_dir(),
                    &workspace_target_dir,
                    member.get_backend(),
                    Some(cwd),
                    false, // Do not setup BSP for each member
                    true, // is_workspace_build
                )
                .await?;
                println!("{}", crate::i18n::tf("built_member", &[member.get_name()]));
            }
            // Setup BSP for the entire workspace
            if let Some(bk) = backend {
                setup_bsp(cwd, &all_deps, &source_dirs, &bk).await?;
            }
            println!("{}", crate::i18n::t("workspace_build_succeeded"));
        } else {
            // Single project or member in workspace
            if let Some(workspace_root) = crate::config::loader::find_workspace_root(cwd) {
                // Build single member in workspace
                if let Some((root_project, members)) = loader::load_workspace(&workspace_root)? {
                    let relative_path = cwd.strip_prefix(&workspace_root).map_err(|_| anyhow::anyhow!("Invalid workspace structure"))?;
                    let member_name = relative_path.components().next()
                        .and_then(|c| c.as_os_str().to_str())
                        .ok_or_else(|| anyhow::anyhow!("Cannot determine member name from path"))?;
                    if let Some(member) = members.into_iter().find(|m| m.get_name() == member_name) {
                        let transitive_deps = crate::dependency::get_transitive_dependencies_with_workspace(&member, Some(&root_project), cwd).await?;
                        crate::build::build_with_deps(
                            cwd,
                            &transitive_deps,
                            member.get_source_dir(),
                            member.get_target_dir(),
                            member.get_backend(),
                            Some(&workspace_root),
                            true, // Setup BSP for this member
                            false, // not workspace build
                        )
                        .await?;
                        println!(
                            "{}",
                            crate::i18n::tf("build_succeeded_with_deps", &[&transitive_deps.len().to_string()])
                        );
                    } else {
                        anyhow::bail!("Member {} not found in workspace", member_name);
                    }
                } else {
                    // Not in a workspace, treat as single project
                    let transitive_deps = crate::dependency::get_transitive_dependencies_with_workspace(&project, None, cwd).await?;
                    crate::build::build_with_deps(
                        cwd,
                        &transitive_deps,
                        project.get_source_dir(),
                        project.get_target_dir(),
                        project.get_backend(),
                        None,
                        true, // Setup BSP for single project
                        false, // not workspace build
                    )
                    .await?;
                    println!(
                        "{}",
                        crate::i18n::tf("build_succeeded_with_deps", &[&transitive_deps.len().to_string()])
                    );
                }
            } else {
                // Single project build
                let transitive_deps = crate::dependency::get_transitive_dependencies_with_workspace(&project, None, cwd).await?;
                crate::build::build_with_deps(
                    cwd,
                    &transitive_deps,
                    project.get_source_dir(),
                    project.get_target_dir(),
                    project.get_backend(),
                    None,
                    true, // Setup BSP for single project
                    false, // not workspace build
                )
                .await?;
                println!(
                    "{}",
                    crate::i18n::tf("build_succeeded_with_deps", &[&transitive_deps.len().to_string()])
                );
            }
        }
    } else {
        anyhow::bail!("No project.toml found in {}", cwd.display());
    }
    Ok(())
}

/// 执行运行命令
async fn execute_run(cwd: &PathBuf, file: Option<PathBuf>, lib: bool) -> anyhow::Result<()> {
    let workspace_root = crate::config::loader::find_workspace_root(cwd);
    let workspace_root_ref = workspace_root.as_ref();

    // 确定项目配置和目录
    let (project, project_dir) = if let Some(ws_root) = workspace_root_ref {
        // 在 workspace 中，查找成员项目
        if let Some((_ws_proj, members)) = crate::config::loader::load_workspace(ws_root)? {
            let relative_path = cwd.strip_prefix(ws_root).map_err(|_| anyhow::anyhow!("Invalid workspace structure"))?;
            let member_name = relative_path.components().next()
                .and_then(|c| c.as_os_str().to_str())
                .ok_or_else(|| anyhow::anyhow!("Cannot determine member name from path"))?;
            if let Some(member) = members
                .into_iter()
                .find(|m| m.package.name == member_name)
            {
                (member, ws_root.clone().join(member_name))
            } else {
                // 不是成员，作为单个项目处理
                let proj = crate::config::loader::load_project(cwd)?;
                (proj, cwd.clone())
            }
        } else {
            // 实际上不是 workspace，作为单个项目处理
            let proj = crate::config::loader::load_project(cwd)?;
            (proj, cwd.clone())
        }
    } else {
        let proj = crate::config::loader::load_project(cwd)?;
        (proj, cwd.clone())
    };

    // 获取依赖
    let deps = if let Some(ws_root) = workspace_root_ref {
        let ws_proj = crate::config::loader::load_project(ws_root)?;
        crate::dependency::get_dependencies_with_workspace(&project, Some(&ws_proj))
    } else {
        crate::dependency::get_dependencies(&project)
    };

    // 设置 BSP 以支持 IDE
    let bsp_dir = workspace_root_ref.unwrap_or(&project_dir);
    let source_dirs = if let Some(ws_root) = workspace_root_ref {
        let member_name = project_dir.strip_prefix(ws_root)
            .map_err(|_| anyhow::anyhow!("Invalid project directory structure"))?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in path"))?;
        vec![(member_name.to_string(), project.get_source_dir().to_string())]
    } else {
        vec![("".to_string(), project.get_source_dir().to_string())]
    };
    setup_bsp(bsp_dir, &deps, &source_dirs, project.get_backend()).await?;

    let target = file.unwrap_or_else(|| project.get_main_file_path());

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
async fn execute_add(cwd: &PathBuf, deps: &[String]) -> anyhow::Result<()> {
    let workspace_root = loader::find_workspace_root(cwd);
    let project_dir = workspace_root.unwrap_or_else(|| cwd.clone());
    for dep in deps {
        add_dependency(&project_dir, dep).await?;
    }
    Ok(())
}