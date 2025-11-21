use crate::deps::deps::Dependency;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

/// 抽象的依赖管理器trait
#[async_trait::async_trait]
pub trait DependencyManager {
    /// 准备依赖（下载、构建等）
    async fn prepare_dependencies(&self, deps: &[Dependency], target_dir: &Path) -> anyhow::Result<()>;

    /// 获取构建命令的参数
    fn get_build_args(&self, deps: &[Dependency]) -> Vec<String>;

    /// 获取运行命令的参数
    fn get_run_args(&self, deps: &[Dependency]) -> Vec<String>;

    /// 验证依赖是否可用（用于添加依赖时）
    async fn validate_dependency(&self, dep: &Dependency) -> anyhow::Result<()>;

    /// 获取传递依赖（包括直接依赖和所有传递依赖）
    async fn get_transitive_dependencies(&self, deps: &[Dependency]) -> anyhow::Result<Vec<Dependency>>;
}

/// 获取打包的coursier可执行文件路径
/// 优先使用打包的版本，如果不存在则返回None
fn get_bundled_coursier_path() -> Option<PathBuf> {
    // 尝试从多个可能的位置查找打包的coursier
    let exe_name = if cfg!(target_os = "windows") {
        "coursier.exe"
    } else {
        "coursier"
    };
    
    // 1. 尝试从bin目录（相对于可执行文件）
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            let bundled_path = exe_dir.join("bin").join(exe_name);
            if bundled_path.exists() {
                return Some(bundled_path);
            }
        }
    }
    
    // 2. 尝试从CARGO_MANIFEST_DIR/bin目录（开发时）
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let bundled_path = PathBuf::from(manifest_dir).join("bin").join(exe_name);
        if bundled_path.exists() {
            return Some(bundled_path);
        }
    }
    
    None
}

/// 获取coursier可执行文件路径
/// 优先使用打包的版本，如果不存在则使用系统命令
async fn get_coursier_path() -> Option<String> {
    // 首先尝试使用打包的coursier
    if let Some(bundled_path) = get_bundled_coursier_path() {
        // 确保文件有执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mut perms) = std::fs::metadata(&bundled_path).map(|m| m.permissions()) {
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&bundled_path, perms);
            }
        }
        
        if let Some(path_str) = bundled_path.to_str() {
            // 验证打包的coursier是否可用
            let mut cmd = Command::new(path_str);
            cmd.arg("--version");
            if cmd.output().await.map(|o| o.status.success()).unwrap_or(false) {
                return Some(path_str.to_string());
            }
        }
    }
    
    // 回退到系统命令
    if check_command_available("coursier").await {
        Some("coursier".to_string())
    } else {
        None
    }
}

/// 检查命令是否可用
async fn check_command_available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

static COURSIER_WARNING_PRINTED: AtomicBool = AtomicBool::new(false);

/// 检查coursier是否可用，如果不可用则打印安装提示（仅一次）
pub async fn check_coursier_available() -> bool {
    let available = get_coursier_path().await.is_some();
    if !available {
        // 只打印一次警告
        if !COURSIER_WARNING_PRINTED.swap(true, Ordering::Relaxed) {
            eprintln!("Warning: coursier is not available. For better dependency management:");
            eprintln!("  - Install coursier: curl -fL https://github.com/coursier/coursier/releases/latest/download/cs-x86_64-pc-linux.gz | gzip -d > cs && chmod +x cs && ./cs install coursier");
            eprintln!("  - Or visit: https://get-coursier.io/");
            eprintln!("  Falling back to scala-cli for dependency management.");
        }
    }
    available
}

/// Coursier 依赖管理器
pub struct CoursierDependencyManager;

#[async_trait::async_trait]
impl DependencyManager for CoursierDependencyManager {
    async fn prepare_dependencies(&self, deps: &[Dependency], _target_dir: &Path) -> anyhow::Result<()> {
        // 获取coursier路径（优先使用打包的版本）
        let coursier_path = get_coursier_path().await
            .ok_or_else(|| anyhow::anyhow!("coursier is not available"))?;
        
        // 使用coursier下载依赖到本地缓存
        // target_dir参数保留以保持trait接口一致性，但coursier使用自己的缓存目录
        for dep in deps {
            match dep {
                Dependency::Maven { group, artifact, version } => {
                    let coord = format!("{}:{}:{}", group, artifact, version);
                    
                    // 使用coursier fetch下载依赖（这会自动缓存）
                    let mut cmd = Command::new(&coursier_path);
                    cmd.arg("fetch")
                        .arg("--quiet")
                        .arg(&coord);
                    
                    let output = cmd.output().await?;
                    if !output.status.success() {
                        let err = String::from_utf8_lossy(&output.stderr);
                        anyhow::bail!("Failed to fetch dependency {}: {}", coord, err);
                    }
                    
                    // 可选：将依赖复制到target目录（如果需要）
                    // 通常coursier会管理缓存，scala-cli也会使用相同的缓存
                }
                Dependency::Sbt { path } => {
                    // 验证sbt项目存在
                    let sbt_project_path = Path::new(path);
                    if !sbt_project_path.exists() {
                        anyhow::bail!("sbt project path does not exist: {}", path);
                    }
                }
            }
        }
        
        Ok(())
    }

    fn get_build_args(&self, deps: &[Dependency]) -> Vec<String> {
        let mut args = Vec::new();

        for dep in deps {
            match dep {
                Dependency::Maven { group, artifact, version } => {
                    args.push("--dependency".to_string());
                    args.push(format!("{}:{}:{}", group, artifact, version));
                }
                Dependency::Sbt { path } => {
                    if Path::new(path).is_relative() {
                        args.push("--dependency".to_string());
                        args.push(format!("file://{}", path));
                    } else {
                        println!("Warning: Absolute sbt path {} may not work with scala-cli", path);
                    }
                }
            }
        }

        args
    }

    fn get_run_args(&self, deps: &[Dependency]) -> Vec<String> {
        self.get_build_args(deps)
    }

    async fn validate_dependency(&self, dep: &Dependency) -> anyhow::Result<()> {
        // 获取coursier路径（优先使用打包的版本）
        let coursier_path = get_coursier_path().await
            .ok_or_else(|| anyhow::anyhow!("coursier is not available"))?;

        match dep {
            Dependency::Maven { group, artifact, version } => {
                let coord = format!("{}:{}:{}", group, artifact, version);

                // 使用coursier resolve验证依赖是否存在
                let mut cmd = Command::new(&coursier_path);
                cmd.arg("resolve")
                    .arg("--quiet")
                    .arg(&coord);

                let output = cmd.output().await?;
                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("Dependency {} is not available: {}", coord, err);
                }

                Ok(())
            }
            Dependency::Sbt { path } => {
                let sbt_project_path = Path::new(path);
                if !sbt_project_path.exists() {
                    anyhow::bail!("sbt project path does not exist: {}", path);
                }
                Ok(())
            }
        }
    }

    async fn get_transitive_dependencies(&self, deps: &[Dependency]) -> anyhow::Result<Vec<Dependency>> {
        let coursier_path = get_coursier_path().await
            .ok_or_else(|| anyhow::anyhow!("coursier is not available"))?;

        let mut all_deps = Vec::new();
        let mut processed_coords = std::collections::HashSet::new();

        for dep in deps {
            match dep {
                Dependency::Maven { group, artifact, version } => {
                    let coord = format!("{}:{}:{}", group, artifact, version);

                    // 使用coursier resolve获取传递依赖
                    let mut cmd = Command::new(&coursier_path);
                    cmd.arg("resolve")
                        .arg("--quiet")
                        .arg("--print-tree=false")
                        .arg("--intransitive")  // 获取所有传递依赖
                        .arg(&coord);

                    let output = cmd.output().await?;
                    if !output.status.success() {
                        let err = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Warning: Failed to resolve transitive dependencies for {}: {}", coord, err);
                        // 如果解析失败，至少包含原始依赖
                        if !processed_coords.contains(&coord) {
                            all_deps.push(dep.clone());
                            processed_coords.insert(coord);
                        }
                        continue;
                    }

                    // 解析输出，coursier resolve输出格式为每行一个坐标
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }

                        // 解析Maven坐标格式 group:artifact:version
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 3 {
                            let group = parts[0];
                            let artifact = parts[1];
                            let version = parts[2];
                            let coord = format!("{}:{}:{}", group, artifact, version);

                            if !processed_coords.contains(&coord) {
                                all_deps.push(Dependency::Maven {
                                    group: group.to_string(),
                                    artifact: artifact.to_string(),
                                    version: version.to_string(),
                                });
                                processed_coords.insert(coord);
                            }
                        }
                    }
                }
                Dependency::Sbt { path } => {
                    // 对于sbt依赖，我们无法解析传递依赖，所以只包含本身
                    all_deps.push(dep.clone());
                }
            }
        }

        Ok(all_deps)
    }
}

/// Scala CLI 依赖管理器
pub struct ScalaCliDependencyManager;

#[async_trait::async_trait]
impl DependencyManager for ScalaCliDependencyManager {
    async fn prepare_dependencies(&self, deps: &[Dependency], _target_dir: &Path) -> anyhow::Result<()> {
        // Scala CLI 会自动处理依赖下载，不需要预先准备
        // 但是我们可以验证依赖是否有效

        // 对于sbt依赖，我们需要特殊处理
        for dep in deps {
            if let Some(sbt_path) = dep.sbt_path() {
                // 验证sbt项目存在
                let sbt_project_path = Path::new(sbt_path);
                if !sbt_project_path.exists() {
                    anyhow::bail!("sbt project path does not exist: {}", sbt_path);
                }

                // 检查是否有project.toml或其他Scala CLI配置文件
                let has_scala_cli_config = sbt_project_path.join("project.scala").exists() ||
                    sbt_project_path.join("project.sc").exists();

                if !has_scala_cli_config {
                    println!("Warning: sbt project {} may not be compatible with scala-cli", sbt_path);
                    println!("Consider converting it to use scala-cli or providing Maven coordinates");
                }
            }
        }

        Ok(())
    }

    fn get_build_args(&self, deps: &[Dependency]) -> Vec<String> {
        let mut args = Vec::new();

        for dep in deps {
            match dep {
                Dependency::Maven { group, artifact, version } => {
                    args.push("--dependency".to_string());
                    args.push(format!("{}:{}:{}", group, artifact, version));
                }
                Dependency::Sbt { path } => {
                    // 对于sbt依赖，尝试将其作为scala-cli项目添加
                    // 如果是相对路径，假设它是一个scala-cli项目
                    if Path::new(path).is_relative() {
                        args.push("--dependency".to_string());
                        args.push(format!("file://{}", path));
                    } else {
                        println!("Warning: Absolute sbt path {} may not work with scala-cli", path);
                    }
                }
            }
        }

        args
    }

    fn get_run_args(&self, deps: &[Dependency]) -> Vec<String> {
        // 运行时参数与构建时相同
        self.get_build_args(deps)
    }

    async fn validate_dependency(&self, dep: &Dependency) -> anyhow::Result<()> {
        // Scala CLI 验证：通过尝试编译一个简单的程序来验证依赖是否可用
        match dep {
            Dependency::Maven { group, artifact, version } => {
                let coord = format!("{}:{}:{}", group, artifact, version);

                // 使用scala-cli来验证依赖（通过尝试编译一个简单的程序）
                // 这样可以确保依赖可以被正确解析和下载
                let mut cmd = Command::new("scala-cli");
                cmd.arg("--dependency")
                    .arg(&coord)
                    .arg("--quiet")
                    .arg("-e")
                    .arg("println(\"test\")");

                let output = cmd.output().await?;
                if !output.status.success() {
                    let err = String::from_utf8_lossy(&output.stderr);
                    anyhow::bail!("Dependency {} is not available: {}", coord, err);
                }

                Ok(())
            }
            Dependency::Sbt { path } => {
                let sbt_project_path = Path::new(path);
                if !sbt_project_path.exists() {
                    anyhow::bail!("sbt project path does not exist: {}", path);
                }
                Ok(())
            }
        }
    }

    async fn get_transitive_dependencies(&self, deps: &[Dependency]) -> anyhow::Result<Vec<Dependency>> {
        // Scala CLI没有直接的传递依赖解析功能
        // 我们尝试使用coursier作为后备，如果不可用则返回直接依赖

        if let Some(coursier_path) = get_coursier_path().await {
            let mut all_deps = Vec::new();
            let mut processed_coords = std::collections::HashSet::new();

            for dep in deps {
                match dep {
                    Dependency::Maven { group, artifact, version } => {
                        let coord = format!("{}:{}:{}", group, artifact, version);

                        // 使用coursier resolve获取传递依赖
                        let mut cmd = Command::new(&coursier_path);
                        cmd.arg("resolve")
                            .arg("--quiet")
                            .arg("--print-tree=false")
                            .arg("--intransitive")
                            .arg(&coord);

                        let output = cmd.output().await?;
                        if !output.status.success() {
                            // 如果coursier解析失败，至少包含原始依赖
                            if !processed_coords.contains(&coord) {
                                all_deps.push(dep.clone());
                                processed_coords.insert(coord);
                            }
                            continue;
                        }

                        // 解析输出
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        for line in stdout.lines() {
                            let line = line.trim();
                            if line.is_empty() || line.starts_with('#') {
                                continue;
                            }

                            let parts: Vec<&str> = line.split(':').collect();
                            if parts.len() >= 3 {
                                let group = parts[0];
                                let artifact = parts[1];
                                let version = parts[2];
                                let coord = format!("{}:{}:{}", group, artifact, version);

                                if !processed_coords.contains(&coord) {
                                    all_deps.push(Dependency::Maven {
                                        group: group.to_string(),
                                        artifact: artifact.to_string(),
                                        version: version.to_string(),
                                    });
                                    processed_coords.insert(coord);
                                }
                            }
                        }
                    }
                    Dependency::Sbt { .. } => {
                        // 对于sbt依赖，无法解析传递依赖
                        all_deps.push(dep.clone());
                    }
                }
            }

            Ok(all_deps)
        } else {
            // 如果coursier不可用，返回直接依赖
            Ok(deps.to_vec())
        }
    }
}

/// 获取默认的依赖管理器
/// 优先使用coursier（如果可用），否则回退到ScalaCliDependencyManager
pub async fn default_dependency_manager() -> Box<dyn DependencyManager + Send + Sync> {
    if check_coursier_available().await {
        Box::new(CoursierDependencyManager)
    } else {
        Box::new(ScalaCliDependencyManager)
    }
}

/// 同步版本的默认依赖管理器（用于不需要异步的场景）
/// 注意：这会检查coursier是否可用，但不会等待
pub fn default_dependency_manager_sync() -> Box<dyn DependencyManager + Send + Sync> {
    // 使用tokio runtime来检查命令
    let rt = tokio::runtime::Runtime::new().unwrap();
    if rt.block_on(check_command_available("coursier")) {
        Box::new(CoursierDependencyManager)
    } else {
        Box::new(ScalaCliDependencyManager)
    }
}
