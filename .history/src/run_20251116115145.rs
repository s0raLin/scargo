// src/run.rs (新增函数)
use crate::deps::Dependency;

pub async fn run_single_file_with_deps(
    proj_dir: &Path,
    file_path: &Path,
    deps: &[Dependency],
) -> anyhow::Result<String> {
    let abs_file = proj_dir.join(file_path);
    let content = tokio::fs::read_to_string(&abs_file).await?;
    let has_main = content.contains("def main") || content.contains("extends App");

    let mut cmd = Command::new("scala-cli");
    cmd.current_dir(proj_dir);

    if has_main {
        cmd.arg("run").arg(&abs_file);
    } else {
        cmd.arg("compile").arg(&abs_file);
    }

    for dep in deps {
        cmd.arg("--dependency").arg(dep.coord());
    }

    let output = cmd.output().await?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed: {}", err);
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}