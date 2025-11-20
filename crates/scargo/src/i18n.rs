use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Zh,
}

impl Language {
    pub fn from_env() -> Self {
        match env::var("SCARGO_LANG").as_deref() {
            Ok("zh") | Ok("zh-CN") | Ok("zh-TW") => Language::Zh,
            _ => Language::En,
        }
    }
}

pub struct I18n {
    messages: HashMap<&'static str, HashMap<Language, &'static str>>,
}

impl I18n {
    fn load_translations() -> HashMap<&'static str, HashMap<Language, &'static str>> {
        let mut messages: HashMap<&'static str, HashMap<Language, &'static str>> = HashMap::new();

        // Mapping from description to key
        let description_to_key = [
            ("应用主描述", "main_about"),
            ("Application main description", "main_about"),
            ("new (name=项目名称)", "new_about"),
            ("New command description", "new_about"),
            ("init", "init_about"),
            ("Init command description", "init_about"),
            ("build", "build_about"),
            ("Build command description", "build_about"),
            ("run (file=文件, lib=库标志)", "run_about"),
            ("Run command description", "run_about"),
            ("add (dep=依赖)", "add_about"),
            ("Add command description", "add_about"),
            ("workspace", "workspace_about"),
            ("Workspace command description", "workspace_about"),
            ("workspace add (path=路径)", "workspace_add_about"),
            ("Workspace add subcommand description", "workspace_add_about"),
            ("new name", "new_name_help"),
            ("New command name parameter help", "new_name_help"),
            ("run file", "run_file_help"),
            ("Run command file parameter help", "run_file_help"),
            ("run lib", "run_lib_help"),
            ("Run command lib flag help", "run_lib_help"),
            ("add dep", "add_dep_help"),
            ("Add command dependency parameter help", "add_dep_help"),
            ("workspace add path", "workspace_add_path_help"),
            ("Workspace add path parameter help", "workspace_add_path_help"),
            ("test (file=文件)", "test_about"),
            ("Test command description", "test_about"),
            ("test file", "test_file_help"),
            ("Test command file parameter help", "test_file_help"),
            ("项目已存在错误", "project_already_exists"),
            ("Project already exists error", "project_already_exists"),
            ("创建新项目消息", "created_project"),
            ("Created project message", "created_project"),
            ("Initialized empty workspace message", "initialized_empty_workspace"),
            ("Added workspace member message", "added_member_to_workspace"),
            ("Member already exists error", "member_already_exists"),
            ("Added dependency message", "added_dependency"),
            ("Built member message", "built_member"),
            ("Build succeeded with dependencies message", "build_succeeded_with_deps"),
            ("Library compile mode message", "lib_compiled_only"),
            ("Main file not found error", "main_file_not_found"),
            ("No command provided error", "no_command_provided"),
            ("Config file already exists error", "config_file_already_exists"),
            ("Not in workspace error", "not_in_workspace"),
        ];

        let description_map: HashMap<&str, &str> = description_to_key.iter().cloned().collect();

        // Load English translations
        let en_content = include_str!("templates/i18n.en.template");
        for line in en_content.lines() {
            let line = line.trim();
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            if let Some((description, value)) = line.split_once(':') {
                let description = description.trim();
                let value = value.trim();
                if let Some(&key) = description_map.get(description) {
                    let key_static = Box::leak(key.to_string().into_boxed_str());
                    let value_static = Box::leak(value.to_string().into_boxed_str());
                    messages.entry(key_static).or_insert_with(HashMap::new).insert(Language::En, value_static);
                }
            }
        }

        // Load Chinese translations
        let zh_content = include_str!("templates/i18n.zh.template");
        for line in zh_content.lines() {
            let line = line.trim();
            // Skip empty lines
            if line.is_empty() {
                continue;
            }
            // Try Chinese colon first, then English colon
            let (description, value) = if let Some((d, v)) = line.split_once('：') {
                (d.trim(), v.trim())
            } else if let Some((d, v)) = line.split_once(':') {
                (d.trim(), v.trim())
            } else {
                continue;
            };
            if let Some(&key) = description_map.get(description) {
                let key_static = Box::leak(key.to_string().into_boxed_str());
                let value_static = Box::leak(value.to_string().into_boxed_str());
                messages.entry(key_static).or_insert_with(HashMap::new).insert(Language::Zh, value_static);
            }
        }

        messages
    }

    pub fn new() -> Self {
        let messages = Self::load_translations();
        Self { messages }
    }

    pub fn get(&self, key: &str) -> String {
        let lang = Language::from_env();
        self.messages
            .get(key)
            .and_then(|lang_map| lang_map.get(&lang))
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string()) // 如果找不到翻译，返回key本身
    }

    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        let mut result = template.clone();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        result
    }
}

lazy_static::lazy_static! {
    pub static ref I18N: I18n = I18n::new();
}

pub fn t(key: &str) -> String {
    I18N.get(key)
}

pub fn tf(key: &str, args: &[&str]) -> String {
    I18N.format(key, args)
}