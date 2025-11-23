//! 领域模型
//!
//! 定义 Sinter 的核心业务实体和值对象

pub mod project;
pub mod dependency;
pub mod workspace;
pub mod directory;
pub mod library;

// Re-export for convenience
pub use project::{Project, Package};
pub use dependency::{DependencySpec, DependencyDetail};
pub use workspace::Workspace;
pub use directory::Directory;
pub use library::Library;