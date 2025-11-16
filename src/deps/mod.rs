pub mod deps;
pub mod add;
pub mod remove;
pub mod update;
pub mod version_resolver;

pub use add::add_dependency;
pub use remove::remove_dependency;
pub use update::update_dependency;
pub use version_resolver::{VersionResolver, parse_dependency_spec};
