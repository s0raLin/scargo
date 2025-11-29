//! 项目服务
//!
//! 提供项目相关的业务逻辑服务

use crate::error::{Result, utils};
use crate::toolkit::file::ProjectCreator;
use crate::toolkit::template::Template;
use crate::toolkit::path::{PathManager, paths};

/// 项目服务接口
#[async_trait::async_trait]
pub trait ProjectService: Send + Sync {
    /// 创建新项目
    async fn create_project(&self, name: &str, cwd: &PathManager) -> Result<()>;

    /// 初始化工作区
    async fn init_workspace(&self, cwd: &PathManager) -> Result<()>;
}

/// 项目服务实现
pub struct ProjectServiceImpl;

impl ProjectServiceImpl {
    /// 创建新的项目服务实例
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn create_project(&self, name: &str, cwd: &PathManager) -> Result<()> {
        let proj_dir = cwd.join(name);
        if proj_dir.exists_sync() {
            println!("{}", crate::i18n::tf("project_already_exists", &[name]));
            return Ok(());
        }

        let creator = ProjectCreator::new(&proj_dir);
        creator.create_dirs(&["src/main/scala"]).await?;

        // project.toml
        let template_path = paths::project_template();
        let template_content = template_path.read_sync()?;
        let template = Template::new(&template_content);
        let manifest = template.replace("name", name).into_string();
        creator.write_file("project.toml", &manifest).await?;

        // Hello world
        let main_template_path = paths::main_template();
        let code = main_template_path.read_sync()?;
        creator.write_file("src/main/scala/Main.scala", &code).await?;

        // Auto-add to workspace if in one
        if let Some(workspace_root) = crate::config::loader::find_workspace_root(cwd) {
            let manifest_path = workspace_root.join("project.toml");
            let relative_path = proj_dir.strip_prefix(&workspace_root)
                .unwrap_or(&proj_dir)
                .to_string_lossy()
                .to_string();
            match crate::config::writer::add_workspace_member(&manifest_path, &relative_path) {
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

    async fn init_workspace(&self, cwd: &PathManager) -> Result<()> {
        // Check if project.toml already exists
        let manifest_path = cwd.join("project.toml");
        if manifest_path.exists_sync() {
            return Err(utils::single_validation_error(
                crate::i18n::t("config_file_already_exists").to_string()
            ));
        }

        // Create workspace project.toml
        let template_path = paths::workspace_template();
        let manifest = template_path.read_sync()?;
        manifest_path.write_sync(&manifest)?;

        println!("{}", crate::i18n::tf("initialized_empty_workspace", &[&cwd.to_path_buf().display().to_string()]));
        Ok(())
    }
}