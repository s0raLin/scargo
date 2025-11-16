use std::collections::HashMap;
use std::env;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

/// 国际化管理器
pub struct I18n {
    language: Language,
    messages: HashMap<&'static str, HashMap<Language, &'static str>>,
}

impl I18n {
    /// 创建新的国际化管理器
    pub fn new() -> Self {
        let mut messages = HashMap::new();

        // CLI相关消息
        let mut cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "A Cargo-like build tool for Scala");
        cli_messages.insert(Language::Chinese, "类似Cargo的Scala构建工具");
        messages.insert("cli.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Create a new Scala project");
        cli_messages.insert(Language::Chinese, "创建一个新的Scala项目");
        messages.insert("cmd.new.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Build the Scala project");
        cli_messages.insert(Language::Chinese, "构建Scala项目");
        messages.insert("cmd.build.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Check the Scala project for compilation errors");
        cli_messages.insert(Language::Chinese, "检查Scala项目的编译错误");
        messages.insert("cmd.check.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Run the Scala project or a specific file");
        cli_messages.insert(Language::Chinese, "运行Scala项目或指定文件");
        messages.insert("cmd.run.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Add a dependency to the project");
        cli_messages.insert(Language::Chinese, "为项目添加依赖");
        messages.insert("cmd.add.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Remove a dependency from the project");
        cli_messages.insert(Language::Chinese, "从项目中移除依赖");
        messages.insert("cmd.remove.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Update dependencies to their latest versions");
        cli_messages.insert(Language::Chinese, "将依赖更新到最新版本");
        messages.insert("cmd.update.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Execute a plugin command");
        cli_messages.insert(Language::Chinese, "执行插件命令");
        messages.insert("cmd.plugin.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "List available plugins");
        cli_messages.insert(Language::Chinese, "列出可用插件");
        messages.insert("cmd.plugins.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Run tests");
        cli_messages.insert(Language::Chinese, "运行测试");
        messages.insert("cmd.test.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Clean build artifacts");
        cli_messages.insert(Language::Chinese, "清理构建产物");
        messages.insert("cmd.clean.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Show project information");
        cli_messages.insert(Language::Chinese, "显示项目信息");
        messages.insert("cmd.info.about", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Start hot reload development mode");
        cli_messages.insert(Language::Chinese, "启动热重载开发模式");
        messages.insert("cmd.dev.about", cli_messages);

        // 参数相关
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Name of the new project");
        cli_messages.insert(Language::Chinese, "新项目的名称");
        messages.insert("arg.name", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Optional .scala file to run (relative to project root)");
        cli_messages.insert(Language::Chinese, "要运行的可选.scala文件（相对于项目根目录）");
        messages.insert("arg.file", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Force library mode (compile only)");
        cli_messages.insert(Language::Chinese, "强制库模式（仅编译）");
        messages.insert("arg.lib", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Dependency in format: group::artifact[@scala-version][:version]");
        cli_messages.insert(Language::Chinese, "依赖格式：group::artifact[@scala-version][:version]");
        messages.insert("arg.dep", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Plugin name");
        cli_messages.insert(Language::Chinese, "插件名称");
        messages.insert("arg.plugin_name", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Plugin command and arguments");
        cli_messages.insert(Language::Chinese, "插件命令和参数");
        messages.insert("arg.plugin_args", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Specific test to run");
        cli_messages.insert(Language::Chinese, "要运行的特定测试");
        messages.insert("arg.test", cli_messages);

        // 构建相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Build succeeded");
        cli_messages.insert(Language::Chinese, "构建成功");
        messages.insert("build.success", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Check succeeded");
        cli_messages.insert(Language::Chinese, "检查成功");
        messages.insert("check.success", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Build succeeded (from cache)");
        cli_messages.insert(Language::Chinese, "构建成功（来自缓存）");
        messages.insert("build.success_cached", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Build succeeded and cached");
        cli_messages.insert(Language::Chinese, "构建成功并已缓存");
        messages.insert("build.success_and_cached", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Build cache hit! Restoring from cache...");
        cli_messages.insert(Language::Chinese, "构建缓存命中！正在从缓存恢复...");
        messages.insert("build.cache_hit", cli_messages);

        // 项目相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Created project `{0}`");
        cli_messages.insert(Language::Chinese, "已创建项目 `{0}`");
        messages.insert("project.created", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Project: {0}");
        cli_messages.insert(Language::Chinese, "项目: {0}");
        messages.insert("project.name", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Version: {0}");
        cli_messages.insert(Language::Chinese, "版本: {0}");
        messages.insert("project.version", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Scala Version: {0}");
        cli_messages.insert(Language::Chinese, "Scala版本: {0}");
        messages.insert("project.scala_version", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Source Directory: {0}");
        cli_messages.insert(Language::Chinese, "源代码目录: {0}");
        messages.insert("project.source_dir", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Target Directory: {0}");
        cli_messages.insert(Language::Chinese, "目标目录: {0}");
        messages.insert("project.target_dir", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Main Class: {0}");
        cli_messages.insert(Language::Chinese, "主类: {0}");
        messages.insert("project.main_class", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Dependencies: {0}");
        cli_messages.insert(Language::Chinese, "依赖项: {0}");
        messages.insert("project.dependencies", cli_messages);

        // 插件相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "No plugins installed.");
        cli_messages.insert(Language::Chinese, "未安装插件。");
        messages.insert("plugins.none", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Available plugins:");
        cli_messages.insert(Language::Chinese, "可用插件:");
        messages.insert("plugins.available", cli_messages);

        // 热重载相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Starting hot reload mode. Watching for file changes...");
        cli_messages.insert(Language::Chinese, "启动热重载模式。正在监视文件变化...");
        messages.insert("hot_reload.start", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "File changed, rebuilding...");
        cli_messages.insert(Language::Chinese, "文件已更改，正在重新构建...");
        messages.insert("hot_reload.rebuild", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Output:");
        cli_messages.insert(Language::Chinese, "输出:");
        messages.insert("hot_reload.output", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Waiting for file changes...");
        cli_messages.insert(Language::Chinese, "等待文件变化...");
        messages.insert("hot_reload.waiting", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Hot reload stopped.");
        cli_messages.insert(Language::Chinese, "热重载已停止。");
        messages.insert("hot_reload.stopped", cli_messages);

        // 清理相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Cleaned build artifacts in {0}");
        cli_messages.insert(Language::Chinese, "已清理 {0} 中的构建产物");
        messages.insert("clean.success", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Cleaned {0} of build artifacts and {1} dependencies");
        cli_messages.insert(Language::Chinese, "已清理 {0} 构建产物和 {1} 个依赖项");
        messages.insert("clean.success_with_details", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "No build artifacts to clean.");
        cli_messages.insert(Language::Chinese, "没有构建产物需要清理。");
        messages.insert("clean.none", cli_messages);

        // 测试相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Tests passed!");
        cli_messages.insert(Language::Chinese, "测试通过！");
        messages.insert("test.passed", cli_messages);

        // 依赖相关消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Added dependency: {0} = {1}");
        cli_messages.insert(Language::Chinese, "已添加依赖: {0} = {1}");
        messages.insert("dep.added", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Removed dependency: {0}");
        cli_messages.insert(Language::Chinese, "已移除依赖: {0}");
        messages.insert("dep.removed", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Updated dependency: {0} from {1} to {2}");
        cli_messages.insert(Language::Chinese, "已更新依赖: {0} 从 {1} 到 {2}");
        messages.insert("dep.updated", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Dependency {0} is already up to date");
        cli_messages.insert(Language::Chinese, "依赖 {0} 已是最新版本");
        messages.insert("dep.up_to_date", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "All dependencies are up to date");
        cli_messages.insert(Language::Chinese, "所有依赖都是最新版本");
        messages.insert("dep.all_up_to_date", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "No dependencies found");
        cli_messages.insert(Language::Chinese, "未找到依赖项");
        messages.insert("dep.no_dependencies", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Dependency not found: {0}");
        cli_messages.insert(Language::Chinese, "未找到依赖: {0}");
        messages.insert("dep.not_found", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Ambiguous dependency specification: {0}. Possible matches: {1}");
        cli_messages.insert(Language::Chinese, "依赖规范不明确: {0}。可能的匹配项: {1}");
        messages.insert("dep.ambiguous", cli_messages);

        // 错误消息
        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "File not found: {0}");
        cli_messages.insert(Language::Chinese, "文件未找到: {0}");
        messages.insert("error.file_not_found", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Main file not found: {0}");
        cli_messages.insert(Language::Chinese, "主文件未找到: {0}");
        messages.insert("error.main_file_not_found", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "No command provided. Use --help for usage.");
        cli_messages.insert(Language::Chinese, "未提供命令。使用 --help 查看用法。");
        messages.insert("error.no_command", cli_messages);

        cli_messages = HashMap::new();
        cli_messages.insert(Language::English, "Plugin '{0}' not found");
        cli_messages.insert(Language::Chinese, "插件 '{0}' 未找到");
        messages.insert("error.plugin_not_found", cli_messages);

        Self {
            language: Self::detect_language(),
            messages,
        }
    }

    /// 检测系统语言
    fn detect_language() -> Language {
        // 检查SCARGO_LANG环境变量
        if let Ok(lang) = env::var("SCARGO_LANG") {
            match lang.to_lowercase().as_str() {
                "zh" | "zh-cn" | "zh_cn" | "chinese" => return Language::Chinese,
                _ => {}
            }
        }

        // 默认使用英文
        Language::English
    }

    /// 获取本地化消息
    pub fn get(&self, key: &str) -> String {
        self.messages
            .get(key)
            .and_then(|lang_map| lang_map.get(&self.language))
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                // 如果找不到翻译，返回英文默认值或key本身
                self.messages
                    .get(key)
                    .and_then(|lang_map| lang_map.get(&Language::English))
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| key.to_string())
            })
    }

    /// 格式化消息（支持参数）
    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        let mut result = template.to_string();

        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }

        result
    }

    /// 获取当前语言
    pub fn language(&self) -> Language {
        self.language
    }

    /// 设置语言
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}

/// 全局国际化实例
lazy_static::lazy_static! {
    pub static ref I18N: I18n = I18n::new();
}

/// 便捷宏用于获取本地化消息
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::i18n::I18N.get($key)
    };
    ($key:expr, $args:expr) => {
        $crate::i18n::I18N.format($key, $args)
    };
}