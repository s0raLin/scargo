// src/build.rs
use crate::deps::deps::Dependency;
use tokio::process::Command;
use tokio::fs;
use std::path::Path;

pub async fn build_with_deps(proj_dir: &Path, deps: &[Dependency], source_dir: &str, target_dir: &str, backend: &str, workspace_root: Option<&Path>, setup_bsp_flag: bool, is_workspace_build: bool) -> anyhow::Result<()> {
    let source_path = proj_dir.join(source_dir);
    let target_path = if let Some(ws_root) = workspace_root {
        ws_root.join(target_dir)
    } else {
        proj_dir.join(target_dir)
    };
    let workspace_dir = workspace_root.unwrap_or(proj_dir);

    // Ensure target directory exists
    fs::create_dir_all(&target_path).await?;

    // Setup BSP for IDE support if requested
    if setup_bsp_flag {
        let bsp_dir = workspace_root.unwrap_or(proj_dir);
        let source_dirs = if let Some(ws_root) = workspace_root {
            let member_name = proj_dir.strip_prefix(ws_root).unwrap().to_str().unwrap();
            vec![(member_name.to_string(), source_dir.to_string())]
        } else {
            vec![("".to_string(), source_dir.to_string())]
        };
        setup_bsp(bsp_dir, deps, &source_dirs, backend).await?;
    }

    let mut cmd = match backend {
        "scala-cli" => {
            let mut cmd = Command::new("scala-cli");
            cmd.arg("compile");
            if is_workspace_build {
                cmd.arg("--workspace").arg(workspace_dir);
            }
            cmd.arg("-d").arg(&target_path).arg(&source_path);
            for dep in deps {
                cmd.arg("--dependency").arg(dep.coord());
            }
            cmd
        }
        "sbt" => {
            // For sbt, we assume build.sbt exists and contains dependencies
            // 如果有外部依赖（比如从其他项目导入的），可能需要特殊处理
            let mut cmd = Command::new("sbt");
            cmd.arg("compile");

            // 对于传递的依赖，如果是 Maven 依赖，我们可以尝试通过系统属性传递
            // 但 sbt 通常期望依赖在 build.sbt 中定义
            // 这里我们主要依赖 sbt 自己的依赖管理
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

pub async fn setup_bsp(bsp_dir: &Path, deps: &[Dependency], source_dirs: &[(String, String)], backend: &str) -> anyhow::Result<()> {
    // Remove any existing .bsp and .scala-build in the bsp_dir.
    let _ = fs::remove_dir_all(bsp_dir.join(".bsp")).await;
    let _ = fs::remove_dir_all(bsp_dir.join(".scala-build")).await;

    // Clean source trees
    for (member_name, source_dir) in source_dirs {
        let source_path = if member_name.is_empty() {
            bsp_dir.join(source_dir)
        } else {
            bsp_dir.join(member_name).join(source_dir)
        };
        let _ = fs::remove_dir_all(source_path.join(".bsp")).await;
        let _ = fs::remove_dir_all(source_path.join(".scala-build")).await;
    }

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
    let scalac_options: Vec<String> = source_dirs.iter().map(|(member_name, source_dir)| {
        let source_dir_rel = if member_name.is_empty() {
            source_dir.clone()
        } else {
            format!("{}/{}", member_name, source_dir)
        };
        format!("{}/**/*.scala", source_dir_rel)
    }).collect();
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/ide-options-v2.json.template"));
    let json_str = template.replace("{scalac_option}", &scalac_options.join("\",\""));
    let mut options: serde_json::Value = serde_json::from_str(&json_str)?;
    options["dependencies"]["dependency"] = serde_json::Value::Array(dependencies.into_iter().map(serde_json::Value::String).collect());
    let content = options.to_string();
    fs::write(&options_path, content).await?;
    Ok(())
}


