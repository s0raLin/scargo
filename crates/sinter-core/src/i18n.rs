use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Zh,
}

impl Language {
    pub fn from_env() -> Self {
        match env::var("SINTER_LANG").as_deref() {
            Ok("zh") | Ok("zh-CN") | Ok("zh-TW") => Language::Zh,
            _ => Language::En,
        }
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "zh" => Language::Zh,
            _ => Language::En,
        }
    }
}

pub struct I18n {
    messages: HashMap<Language, HashMap<String, String>>,
}

impl I18n {
    fn load_translations() -> HashMap<Language, HashMap<String, String>> {
        let json: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(include_str!("../templates/i18n.json")).unwrap();
        let mut messages = HashMap::new();
        for (lang_str, translations) in json {
            let lang = Language::from(lang_str.as_str());
            messages.insert(lang, translations);
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
            .get(&lang)
            .and_then(|m| m.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    pub fn format(&self, key: &str, args: &[&str]) -> String {
        let template = self.get(key);
        let mut result = template;
        for arg in args {
            if result.contains("{}") {
                result = result.replacen("{}", arg, 1);
            } else {
                break;
            }
        }
        result
    }

    /// 导出翻译到JSON文件
    pub fn export_to_json(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let json_data: std::collections::HashMap<String, std::collections::HashMap<String, String>> = self.messages
            .iter()
            .map(|(lang, translations)| {
                let lang_str = match lang {
                    Language::En => "en".to_string(),
                    Language::Zh => "zh".to_string(),
                };
                (lang_str, translations.clone())
            })
            .collect();

        let json = serde_json::to_string_pretty(&json_data)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// 从JSON文件加载翻译
    pub fn load_from_json(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let json: std::collections::HashMap<String, std::collections::HashMap<String, String>> =
            serde_json::from_str(&content)?;

        let mut messages = std::collections::HashMap::new();
        for (lang_str, translations) in json {
            let lang = Language::from(lang_str.as_str());
            messages.insert(lang, translations);
        }

        Ok(Self { messages })
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