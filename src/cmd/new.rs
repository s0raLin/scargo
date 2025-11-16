use std::path::PathBuf;

use crate::core::project::{Package, Project};


pub async fn cmd_new(cwd: &PathBuf, name: &str) -> anyhow::Result<()> {
    let proj_dir = cwd.join(name);
    tokio::fs::create_dir_all(proj_dir.join("src/main/scala")).await?;

    // Scargo.toml
    let project = Project {
        package: Package {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            main: Some("Main".to_string()),
            scala_version: "2.13".to_string(),
            source_dir: "src/main/scala".to_string(),
            target_dir: "build".to_string(),
        },
        dependencies: std::collections::HashMap::new(),
    };
    let manifest = toml::to_string_pretty(&project)?;
    tokio::fs::write(proj_dir.join("Scargo.toml"), manifest).await?;

    // Hello world
    let code = r#"object Main extends App {
        println("Hello from scargo!")
    }
    "#;
    tokio::fs::write(
        proj_dir.join("src/main/scala/Main.scala"),
        code,
    )
    .await?;

    println!("Created project `{}`", name);
    Ok(())
}
