pub mod build;
pub mod run;
pub mod cache;
pub mod watch;

pub use build::build_with_deps;
pub use build::check_with_deps;
pub use build::run_tests;
pub use run::run_scala_file;
pub use run::run_single_file_with_deps;
pub use cache::BuildCache;
pub use watch::start_hot_reload;