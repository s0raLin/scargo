//! 自定义错误类型
//!
//! 提供结构化的错误处理，替代anyhow的字符串错误

use std::fmt;

/// Sinter 错误类型
#[derive(Debug)]
pub enum SinterError {
    /// 配置相关错误
    Config(ConfigError),
    /// 项目验证错误
    Validation(Vec<String>),
    /// 构建错误
    Build(BuildError),
    /// 依赖解析错误
    Dependency(DependencyError),
    /// IO错误
    Io(std::io::Error),
    /// TOML解析错误
    Toml(toml::de::Error),
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound { path: std::path::PathBuf },
    ParseError { source: toml::de::Error },
    InvalidFormat(String),
}

#[derive(Debug)]
pub enum BuildError {
    BackendNotSupported(String),
    CommandFailed { command: String, exit_code: Option<i32> },
    ScalaVersionMismatch { required: String, found: String },
}

#[derive(Debug)]
pub enum DependencyError {
    InvalidCoordinate(String),
    VersionConflict { name: String, versions: Vec<String> },
    NotFound(String),
}

impl fmt::Display for SinterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SinterError::Config(e) => write!(f, "配置错误: {}", e),
            SinterError::Validation(errors) => {
                write!(f, "验证失败:\n{}", errors.join("\n"))
            }
            SinterError::Build(e) => write!(f, "构建错误: {}", e),
            SinterError::Dependency(e) => write!(f, "依赖错误: {}", e),
            SinterError::Io(e) => write!(f, "IO错误: {}", e),
            SinterError::Toml(e) => write!(f, "TOML解析错误: {}", e),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::FileNotFound { path } => {
                write!(f, "配置文件未找到: {}", path.display())
            }
            ConfigError::ParseError { source } => {
                write!(f, "配置文件解析失败: {}", source)
            }
            ConfigError::InvalidFormat(msg) => write!(f, "配置格式无效: {}", msg),
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::BackendNotSupported(backend) => {
                write!(f, "不支持的构建后端: {}", backend)
            }
            BuildError::CommandFailed { command, exit_code } => {
                write!(f, "命令执行失败: {} (退出码: {:?})", command, exit_code)
            }
            BuildError::ScalaVersionMismatch { required, found } => {
                write!(f, "Scala版本不匹配，需要: {}, 发现: {}", required, found)
            }
        }
    }
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DependencyError::InvalidCoordinate(coord) => {
                write!(f, "无效的依赖坐标: {}", coord)
            }
            DependencyError::VersionConflict { name, versions } => {
                write!(f, "依赖版本冲突 {}: {:?}", name, versions)
            }
            DependencyError::NotFound(name) => {
                write!(f, "依赖未找到: {}", name)
            }
        }
    }
}

impl std::error::Error for SinterError {}
impl std::error::Error for ConfigError {}
impl std::error::Error for BuildError {}
impl std::error::Error for DependencyError {}

impl From<std::io::Error> for SinterError {
    fn from(err: std::io::Error) -> Self {
        SinterError::Io(err)
    }
}

impl From<toml::de::Error> for SinterError {
    fn from(err: toml::de::Error) -> Self {
        SinterError::Toml(err)
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, SinterError>;