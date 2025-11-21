// src/build.rs
use crate::deps::deps::Dependency;
use tokio::process::Command;
use tokio::fs;
use std::path::Path;

pub async fn build_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str, target_dir: &str, backend: &str, workspace_root: Option<&Path>) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let target_path = if let Some(ws_root) = workspace_root {
        ws_root.join(target_dir)
    } else {
        proj_dir.join(target_dir)
    };
    let workspace_dir = workspace_root.unwrap_or(proj_dir);

    // Ensure target directory exists
    fs::create_dir_all(&target_path).await?;

    // Setup BSP for IDE support
    setup_bsp(proj_dir, deps, source_dir, backend, workspace_root).await?;

    let mut cmd = match backend {
        "scala-cli" => {
            let mut cmd = Command::new("scala-cli");
            cmd.arg("compile");
            cmd.arg("--workspace").arg(workspace_dir);
            cmd.arg("-d").arg(&target_path).arg(&source_path);
            for dep in deps {
                cmd.arg("--dependency").arg(dep.coord());
            }
            cmd
        }
        "sbt" => {
            // For sbt, we assume build.sbt exists and contains dependencies
            let mut cmd = Command::new("sbt");
            cmd.arg("compile");
            cmd
        }
        "gradle" => {
            // For gradle, we assume build.gradle exists and contains dependencies
            let mut cmd = Command::new("gradle");
            cmd.arg("compile");
            cmd
        }
        "maven" => {
            // For maven, we assume pom.xml exists and contains dependencies
            let mut cmd = Command::new("mvn");
            cmd.arg("compile");
            cmd
        }
        _ => {
            anyhow::bail!("Unsupported backend: {}", backend);
        }
    };
    cmd.current_dir(proj_dir);

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("Build failed with dependencies");
    }

    // Move build files from source to target
    // move_build_files(&source_path, &target_path).await?;

    // Always clean up build artifacts that scala-cli drops inside the source tree.
    let _ = fs::remove_dir_all(source_path.join(".bsp")).await;
    let _ = fs::remove_dir_all(source_path.join(".scala-build")).await;

    Ok(())
}

pub async fn setup_bsp(proj_dir: &Path, deps: &[Dependency], source_dir: &str, backend: &str, workspace_root: Option<&std::path::Path>) -> anyhow::Result<()> {
    let bsp_dir = workspace_root.unwrap_or(proj_dir);
    let source_dir_rel = if let Some(ws_root) = workspace_root {
        let member_name = proj_dir.strip_prefix(ws_root).unwrap().to_str().unwrap();
        format!("{}/{}", member_name, source_dir)
    } else {
        source_dir.to_string()
    };

    // If in workspace, remove any existing .bsp and .scala-build in the project dir.
    if workspace_root.is_some() {
        let _ = fs::remove_dir_all(proj_dir.join(".bsp")).await;
        let _ = fs::remove_dir_all(proj_dir.join(".scala-build")).await;
    }
    // Regardless of workspace mode, ensure the source tree stays clean.
    let source_path = proj_dir.join(source_dir);
    let _ = fs::remove_dir_all(source_path.join(".bsp")).await;
    let _ = fs::remove_dir_all(source_path.join(".scala-build")).await;

    let mut cmd = match backend {
        "scala-cli" => {
            let mut cmd = Command::new("scala-cli");
            cmd.arg("setup-ide").arg(".");
            for dep in deps {
                cmd.arg("--dependency").arg(dep.coord());
            }
            cmd
        }
        "sbt" | "gradle" | "maven" => {
            // For other backends, BSP setup might be different or not needed
            // For now, skip BSP setup for non-scala-cli backends
            return Ok(());
        }
        _ => {
            anyhow::bail!("Unsupported backend: {}", backend);
        }
    };
    cmd.current_dir(bsp_dir);

    let status = cmd.status().await?;
    if !status.success() {
        anyhow::bail!("BSP setup failed");
    }

    // Manually set ide-options-v2.json
    let options_path = bsp_dir.join(".scala-build/ide-options-v2.json");
    fs::create_dir_all(options_path.parent().unwrap()).await?;
    let dependencies: Vec<String> = deps.iter().map(|d| d.coord()).collect();
    let scalac_option = format!("{}/**/*.scala", source_dir_rel);
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/ide-options-v2.json.template"));
    let json_str = template.replace("{scalac_option}", &scalac_option);
    let mut options: serde_json::Value = serde_json::from_str(&json_str)?;
    options["dependencies"]["dependency"] = serde_json::Value::Array(dependencies.into_iter().map(serde_json::Value::String).collect());
    let content = options.to_string();
    fs::write(&options_path, content).await?;
    Ok(())
}


