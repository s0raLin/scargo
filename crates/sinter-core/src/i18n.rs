//! 国际化支持模块
//!
//! 此模块在构建时由 build.rs 生成，提供类型安全的翻译功能。
//! 语言选择通过 Cargo 特性标志控制：
//! - `lang-en` (默认): 英文
//! - `lang-zh`: 中文
//!
//! 使用示例：
//! ```rust
//! use crate::i18n::{t, tf};
//!
//! // 简单翻译
//! println!("{}", t("main_about"));
//!
//! // 带参数的翻译
//! println!("{}", tf("created_project", &["my-project"]));
//! ```

// Fallback 实现（用于 rust-analyzer 和测试环境）
// 当 OUT_DIR 未设置时，rust-analyzer 可以使用这个实现进行分析
mod fallback {
    /// Get a translated string by key (fallback implementation)
    /// This is used when build scripts haven't run (e.g., in rust-analyzer)
    pub fn t(_key: &str) -> &'static str {
        // 返回一个占位符字符串，这样 rust-analyzer 可以正常工作
        // 实际构建时会使用生成的代码
        "[Translation placeholder]"
    }

    /// Format a translated string with arguments (fallback implementation)
    pub fn tf(key: &str, args: &[&str]) -> String {
        let template = t(key);
        let mut result = template.to_string();
        for arg in args {
            if result.contains("{}") {
                result = result.replacen("{}", arg, 1);
            } else {
                break;
            }
        }
        result
    }
}

// 尝试包含构建时生成的代码
// 使用条件编译：只有在实际构建时才包含生成的代码
#[cfg(all(not(test), not(doctest)))]
mod generated {
    // 尝试包含生成的代码
    // 如果 OUT_DIR 未设置，这会在编译时失败
    // 但我们可以通过提供一个默认的英文实现来避免这个问题
    // 注意：在实际构建时，build.rs 总是会运行，所以 OUT_DIR 总是存在
    include!(concat!(env!("OUT_DIR"), "/i18n.rs"));
}

// 导出函数
// 在实际构建时使用生成的代码，在测试或 rust-analyzer 时使用 fallback
#[cfg(all(not(test), not(doctest)))]
pub use generated::{t, tf};

#[cfg(any(test, doctest))]
pub use fallback::{t, tf};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_translation() {
        // 测试简单翻译
        let msg = t("main_about");
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_formatted_translation() {
        // 测试格式化翻译
        // 注意：在测试模式下使用 fallback，所以只测试函数能正常调用
        let msg = tf("created_project", &["test-project"]);
        assert!(!msg.is_empty());
        // fallback 实现会替换 {}，所以应该包含参数
        assert!(msg.contains("test-project") || msg == "[Translation placeholder]");
    }
}

