use std::path::PathBuf;


pub async fn cmd_new(cwd: &PathBuf, name: &str) -> anyhow::Result<()> {
    let proj_dir = cwd.join(name);
    if proj_dir.exists() {
        println!("{}", crate::i18n::tf("project_already_exists", &[name]));
        return Ok(());
    }
    tokio::fs::create_dir_all(proj_dir.join("src/main/scala")).await?;

    // Scargo.toml
    let template = include_str!("../templates/scargo.toml.template");
    let manifest = template.replace("{name}", name);
    tokio::fs::write(proj_dir.join("Scargo.toml"), manifest).await?;

    // Hello world
    let code = include_str!("../templates/main.scala.template");
    tokio::fs::write(
        proj_dir.join("src/main/scala/Main.scala"),
        code,
    )
    .await?;

    println!("{}", crate::i18n::tf("created_project", &[name]));
    Ok(())
}
