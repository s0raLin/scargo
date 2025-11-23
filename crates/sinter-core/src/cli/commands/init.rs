use crate::toolkit::path::{PathManager, paths};

pub async fn cmd_init(cwd: &PathManager) -> anyhow::Result<()> {
    // Check if project.toml already exists
    let manifest_path = cwd.join("project.toml");
    if manifest_path.exists_sync() {
        anyhow::bail!("{}", crate::i18n::t("config_file_already_exists"));
    }

    // Create workspace project.toml
    let template_path = paths::workspace_template();
    let manifest = template_path.read_sync()?;
    manifest_path.write_sync(&manifest)?;

    println!("{}", crate::i18n::tf("initialized_empty_workspace", &[&cwd.to_path_buf().display().to_string()]));
    Ok(())
}