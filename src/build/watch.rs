use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use crate::deps::deps::Dependency;
use crate::build::{build_with_deps, run_single_file_with_deps};
use crate::core::project::Project;

/// 热重载监视器
pub struct HotReload {
    project_dir: PathBuf,
    source_dir: String,
    target_dir: String,
    main_file: PathBuf,
    deps: Vec<Dependency>,
}

struct HotReloadInstance {
    project_dir: PathBuf,
    source_dir: String,
    target_dir: String,
    main_file: PathBuf,
    deps: Vec<Dependency>,
    watcher: Option<RecommendedWatcher>,
}

impl HotReload {
    pub fn new(project: &Project, project_dir: &Path, deps: Vec<Dependency>) -> Self {
        let main_file = project.get_main_file_path();
        Self {
            project_dir: project_dir.to_path_buf(),
            source_dir: project.package.source_dir.clone(),
            target_dir: project.package.target_dir.clone(),
            main_file,
            deps,
        }
    }

    /// 开始热重载监视
    pub async fn start(self) -> anyhow::Result<()> {
        println!("{}", crate::t!("hot_reload.start"));

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Ok(event) = res {
                        let _ = tx.send(event).await;
                    }
                });
            },
            Config::default(),
        )?;

        let source_path = self.project_dir.join(&self.source_dir);
        watcher.watch(&source_path, RecursiveMode::Recursive)?;

        let hot_reload = Arc::new(Mutex::new(HotReloadInstance {
            project_dir: self.project_dir,
            source_dir: self.source_dir,
            target_dir: self.target_dir,
            main_file: self.main_file,
            deps: self.deps,
            watcher: Some(watcher),
        }));

        // 初始构建和运行
        {
            let mut instance = hot_reload.lock().await;
            instance.rebuild_and_run().await?;
        }

        // 监听文件变化
        let debounce_duration = Duration::from_millis(500);
        let mut debounce_timer: Option<tokio::task::JoinHandle<()>> = None;

        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    if Self::should_trigger_rebuild_static(&event) {
                        // 防抖处理
                        if let Some(timer) = debounce_timer.take() {
                            timer.abort();
                        }

                        let hot_reload_clone = Arc::clone(&hot_reload);
                        let timer = tokio::spawn(async move {
                            time::sleep(debounce_duration).await;
                            let mut instance = hot_reload_clone.lock().await;
                            if let Err(e) = instance.rebuild_and_run().await {
                                eprintln!("Hot reload error: {}", e);
                            }
                        });

                        debounce_timer = Some(timer);
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("{}", crate::t!("hot_reload.stopped"));
                    break;
                }
            }
        }

        Ok(())
    }

    /// 判断是否应该触发重新构建（静态方法）
    fn should_trigger_rebuild_static(event: &Event) -> bool {
        // 只关心Scala源文件的变化
        event.paths.iter().any(|path| {
            path.extension().and_then(|ext| ext.to_str()) == Some("scala")
        }) && matches!(event.kind, notify::EventKind::Modify(_) | notify::EventKind::Create(_) | notify::EventKind::Remove(_))
    }

    /// 重新构建并运行
    async fn rebuild_and_run(&mut self) -> anyhow::Result<()> {
        println!("{}", crate::t!("hot_reload.rebuild"));

        // 构建
        build_with_deps(
            &self.project_dir,
            &self.deps,
            &self.source_dir,
            &self.target_dir,
        ).await?;

        // 运行
        let output = run_single_file_with_deps(
            &self.project_dir,
            &self.main_file,
            &self.deps,
        ).await?;

        println!("{}\n{}", crate::t!("hot_reload.output"), output);
        println!("{}", crate::t!("hot_reload.waiting"));

        Ok(())
    }
}

impl HotReloadInstance {
    /// 重新构建并运行
    async fn rebuild_and_run(&mut self) -> anyhow::Result<()> {
        println!("File changed, rebuilding...");

        // 构建
        build_with_deps(
            &self.project_dir,
            &self.deps,
            &self.source_dir,
            &self.target_dir,
        ).await?;

        // 运行
        let output = run_single_file_with_deps(
            &self.project_dir,
            &self.main_file,
            &self.deps,
        ).await?;

        println!("Output:\n{}", output);
        println!("Waiting for file changes...");

        Ok(())
    }
}

/// 启动热重载模式
pub async fn start_hot_reload(project_dir: &Path) -> anyhow::Result<()> {
    let project = Project::load(project_dir)?;
    let deps = project.get_dependencies();

    let hot_reload = HotReload::new(&project, project_dir, deps);
    hot_reload.start().await
}