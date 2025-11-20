//! JSP 项目生成插件
//!
//! 这个文件展示了如何创建一个简单的插件。
//! 只需要实现 CommandHandler trait 的四个方法即可：
//! 1. name() - 返回命令名称
//! 2. about() - 返回命令描述
//! 3. configure() - 配置命令参数（可选，有默认实现）
//! 4. execute() - 执行命令逻辑
//!
//! 注意：只需要依赖 `scargo-plugin`，所有必要的类型都已重新导出

use scargo_plugin::*;

// 嵌入模板文件
static POM_XML_TEMPLATE: &str = include_str!("../templates/pom.xml.template");
static WEB_XML_TEMPLATE: &str = include_str!("../templates/web.xml.template");
static INDEX_JSP_TEMPLATE: &str = include_str!("../templates/index.jsp.template");

/// JSP 插件结构体
pub struct JspPlugin;

#[async_trait]
impl CommandHandler for JspPlugin {
    /// 命令名称：用户输入 `scargo jsp` 时会触发
    fn name(&self) -> &'static str {
        "jsp"
    }

    /// 命令描述：显示在帮助信息中
    fn about(&self) -> &'static str {
        "Generate a new JSP project"
    }

    /// 配置命令参数：定义 `scargo jsp <name>` 中的参数
    fn configure(&self, cmd: Command) -> Command {
        cmd.about(self.about()).arg(
            Arg::new("name")
                .help("Name of the JSP project")
                .required(true)
        )
    }

    /// 执行命令逻辑：当用户运行 `scargo jsp <name>` 时调用
    async fn execute(&self, matches: &ArgMatches, cwd: &PathBuf) -> AnyhowResult<()> {
        // 从命令行参数中获取项目名称
        let name = matches
            .get_one::<String>("name")
            .expect("name argument is required");

        // 创建项目目录
        let proj_dir = cwd.join(name);
        if proj_dir.exists() {
            println!("JSP project '{}' already exists", name);
            return Ok(());
        }

        // 创建目录结构
        fs::create_dir_all(proj_dir.join("src/main/webapp/WEB-INF")).await?;
        fs::create_dir_all(proj_dir.join("src/main/java")).await?;
        fs::create_dir_all(proj_dir.join("src/main/resources")).await?;

        // 生成 pom.xml
        let pom_xml = POM_XML_TEMPLATE.replace("{{name}}", name);
        fs::write(proj_dir.join("pom.xml"), pom_xml).await?;

        // 生成 web.xml
        fs::write(proj_dir.join("src/main/webapp/WEB-INF/web.xml"), WEB_XML_TEMPLATE).await?;

        // 生成 index.jsp
        fs::write(proj_dir.join("src/main/webapp/index.jsp"), INDEX_JSP_TEMPLATE).await?;

        println!("Created JSP project '{}'", name);
        println!("To build and run:");
        println!("  cd {}", name);
        println!("  mvn clean package");
        println!("  # Deploy the generated .war file to Tomcat");

        Ok(())
    }
}

// 导出插件实例创建函数，供注册使用
pub fn jsp_plugin() -> JspPlugin {
    JspPlugin
}

