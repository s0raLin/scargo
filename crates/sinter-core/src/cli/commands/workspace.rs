use std::path::PathBuf;

use crate::cli::WorkspaceCommands;

pub async fn cmd_workspace(cwd: &PathBuf, subcommand: &WorkspaceCommands) -> anyhow::Result<()> {
    match subcommand {
        WorkspaceCommands::Add { paths } => {
            cmd_workspace_add(cwd, paths).await?;
        }
    }
    Ok(())
}

async fn cmd_workspace_add(cwd: &PathBuf, member_paths: &[String]) -> anyhow::Result<()> {
    // Find workspace root
    let workspace_root = crate::config::loader::find_workspace_root(cwd)
        .ok_or_else(|| anyhow::anyhow!("{}", crate::i18n::t("not_in_workspace")))?;

    let manifest_path = workspace_root.join("project.toml");

    for member_path in member_paths {
        // Check if member already exists by trying to add it
        match crate::config::writer::add_workspace_member(&manifest_path, member_path) {
            Ok(_) => {
                println!("{}", crate::i18n::tf("added_member_to_workspace", &[member_path]));
            }
            Err(_) => {
                println!("{}", crate::i18n::tf("member_already_exists", &[member_path]));
            }
        }
    }
    Ok(())
}