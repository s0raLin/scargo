use std::path::PathBuf;

pub async fn cmd_new(cwd: &PathBuf, name: &str) -> anyhow::Result<()> {
    let proj_dir = cwd.join(name);
    if proj_dir.exists() {
        println!("{}", crate::i18n::tf("project_already_exists", &[name]));
        return Ok(());
    }
    tokio::fs::create_dir_all(proj_dir.join("src/main/scala")).await?;

    // project.toml
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/project.toml.template"));
    let manifest = template.replace("{name}", name);
    tokio::fs::write(proj_dir.join("project.toml"), manifest).await?;

    // Hello world
    let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/main.scala.template"));
    tokio::fs::write(
        proj_dir.join("src/main/scala/Main.scala"),
        code,
    )
    .await?;

    // Auto-add to workspace if in one
    if let Some(workspace_root) = crate::config::find_workspace_root(cwd) {
        let manifest_path = workspace_root.join("project.toml");
        let relative_path = proj_dir.strip_prefix(&workspace_root)
            .unwrap_or(&proj_dir)
            .to_string_lossy()
            .to_string();
        match crate::config::add_workspace_member(&manifest_path, &relative_path) {
            Ok(_) => {
                println!("{}", crate::i18n::tf("added_project_to_workspace", &[name]));
            }
            Err(e) => {
                if !e.to_string().contains("already exists") {
                    eprintln!("Warning: Failed to add project to workspace: {}", e);
                }
            }
        }
    }

    println!("{}", crate::i18n::tf("created_project", &[name]));
    Ok(())
}
