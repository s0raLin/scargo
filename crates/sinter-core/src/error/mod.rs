//! 错误处理模块
//!
//! 提供统一的错误类型和处理机制，支持结构化的错误信息和错误转换

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
    /// 服务相关错误
    Service(ServiceError),
    /// 依赖注入错误
    DI(DIError),
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

#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    RegistrationFailed(String),
    ResolutionFailed(String),
}

#[derive(Debug)]
pub enum DIError {
    ServiceNotRegistered(String),
    TypeMismatch(String),
    ContainerNotInitialized,
    RegistrationFailed(String),
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
            SinterError::Service(e) => write!(f, "服务错误: {}", e),
            SinterError::DI(e) => write!(f, "依赖注入错误: {}", e),
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

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound(name) => write!(f, "服务未找到: {}", name),
            ServiceError::RegistrationFailed(msg) => write!(f, "服务注册失败: {}", msg),
            ServiceError::ResolutionFailed(msg) => write!(f, "服务解析失败: {}", msg),
        }
    }
}

impl fmt::Display for DIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DIError::ServiceNotRegistered(type_name) => {
                write!(f, "服务未注册: {}", type_name)
            }
            DIError::TypeMismatch(msg) => write!(f, "类型不匹配: {}", msg),
            DIError::ContainerNotInitialized => write!(f, "服务容器未初始化"),
            DIError::RegistrationFailed(msg) => write!(f, "服务注册失败: {}", msg),
        }
    }
}

impl std::error::Error for SinterError {}
impl std::error::Error for ConfigError {}
impl std::error::Error for BuildError {}
impl std::error::Error for DependencyError {}
impl std::error::Error for ServiceError {}
impl std::error::Error for DIError {}

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

impl From<ServiceError> for SinterError {
    fn from(err: ServiceError) -> Self {
        SinterError::Service(err)
    }
}

impl From<DIError> for SinterError {
    fn from(err: DIError) -> Self {
        SinterError::DI(err)
    }
}

impl From<anyhow::Error> for SinterError {
    fn from(err: anyhow::Error) -> Self {
        utils::from_anyhow(err)
    }
}

impl From<String> for DIError {
    fn from(msg: String) -> Self {
        DIError::RegistrationFailed(msg)
    }
}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, SinterError>;

/// 错误处理工具
pub mod utils {
    use super::{Result, SinterError};

    /// 将anyhow错误转换为SinterError
    pub fn from_anyhow(err: anyhow::Error) -> SinterError {
        SinterError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }

    /// 将字符串转换为验证错误
    pub fn validation_error(messages: Vec<String>) -> SinterError {
        SinterError::Validation(messages)
    }

    /// 创建单个验证错误
    pub fn single_validation_error(message: String) -> SinterError {
        SinterError::Validation(vec![message])
    }
}