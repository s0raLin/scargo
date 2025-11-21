use std::path::PathBuf;

pub async fn cmd_init(cwd: &PathBuf) -> anyhow::Result<()> {
    // Check if project.toml already exists
    let manifest_path = cwd.join("project.toml");
    if manifest_path.exists() {
        anyhow::bail!("{}", crate::i18n::t("config_file_already_exists"));
    }

    // Create workspace project.toml
    let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/workspace.project.toml.template"));
    tokio::fs::write(manifest_path, manifest).await?;

    println!("{}", crate::i18n::tf("initialized_empty_workspace", &[&cwd.display().to_string()]));
    Ok(())
}