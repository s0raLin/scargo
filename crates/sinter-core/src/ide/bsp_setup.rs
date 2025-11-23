use crate::deps::deps::Dependency;
use std::path::Path;
use crate::toolkit::os::{PathWrapper, remove_all, make_dir_all, write};

pub async fn setup_bsp(bsp_dir: &Path, deps: &[Dependency], source_dirs: &[(String, String)], backend: &str) -> anyhow::Result<()> {
    // Remove any existing .bsp and .scala-build in the bsp_dir.
    let _ = remove_all(&PathWrapper::new(bsp_dir.join(".bsp"))).await;
    let _ = remove_all(&PathWrapper::new(bsp_dir.join(".scala-build"))).await;

    // Clean source trees
    for (member_name, source_dir) in source_dirs {
        let source_path = if member_name.is_empty() {
            bsp_dir.join(source_dir)
        } else {
            bsp_dir.join(member_name).join(source_dir)
        };
        let _ = remove_all(&PathWrapper::new(source_path.join(".bsp"))).await;
        let _ = remove_all(&PathWrapper::new(source_path.join(".scala-build"))).await;
    }

    match backend {
        "scala-cli" => {
            let mut args: Vec<String> = vec!["setup-ide".to_string(), ".".to_string()];
            for dep in deps {
                args.push("--dependency".to_string());
                args.push(dep.coord());
            }
            let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let output = crate::build::run_scala_cli(&args_str, Some(bsp_dir)).await?;
            if !output.status.success() {
                anyhow::bail!("BSP setup failed");
            }
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

    // Manually set ide-options-v2.json
    let options_path = bsp_dir.join(".scala-build/ide-options-v2.json");
    make_dir_all(&PathWrapper::new(options_path.parent().unwrap())).await?;
    let dependencies: Vec<String> = deps.iter().map(|d| d.coord()).collect();
    let scalac_options: Vec<String> = source_dirs.iter().map(|(member_name, source_dir)| {
        let source_dir_rel = if member_name.is_empty() {
            source_dir.clone()
        } else {
            format!("{}/{}", member_name, source_dir)
        };
        format!("{}/**/*.scala", source_dir_rel)
    }).collect();
    let template_path = crate::toolkit::path::paths::template_file("ide-options-v2.json.template");
    let template = template_path.read_sync()?;
    let json_str = template.replace("{scalac_option}", &scalac_options.join("\",\""));
    let mut options: serde_json::Value = serde_json::from_str(&json_str)?;
    options["dependencies"]["dependency"] = serde_json::Value::Array(dependencies.into_iter().map(serde_json::Value::String).collect());
    let content = options.to_string();
    write(&PathWrapper::new(&options_path), &content).await?;
    Ok(())
}