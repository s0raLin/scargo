use std::path::PathBuf;

pub async fn cmd_init(cwd: &PathBuf) -> anyhow::Result<()> {
    // Check if Scargo.toml already exists
    let manifest_path = cwd.join("Scargo.toml");
    if manifest_path.exists() {
        anyhow::bail!("{}", crate::i18n::t("scargo_toml_already_exists"));
    }

    // Create workspace Scargo.toml
    let manifest = include_str!("../templates/workspace.scargo.toml.template");
    tokio::fs::write(manifest_path, manifest).await?;

    println!("{}", crate::i18n::tf("initialized_empty_workspace", &[&cwd.display().to_string()]));
    Ok(())
}