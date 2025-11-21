pub mod deps;
pub mod add;
pub mod manager;

pub use add::add_dependency;
pub use manager::{DependencyManager, ScalaCliDependencyManager, CoursierDependencyManager, default_dependency_manager, default_dependency_manager_sync};
