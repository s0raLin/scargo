//! 数据模型和DTO
//!
//! 定义 Sinter 的数据传输对象（DTO）和领域模型

pub mod project;
pub mod dependency;
pub mod workspace;
pub mod directory;
pub mod library;

// Re-export for convenience
pub use project::{Project, Package, ProjectDto};
pub use dependency::{DependencySpec, DependencyDetail, DependencyDto};
pub use workspace::{Workspace, WorkspaceDto};
pub use directory::Directory;
pub use library::{Library, LibraryType};