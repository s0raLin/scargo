pub mod new;
pub mod init;
pub mod test;
pub mod workspace;
pub mod builtin;

// 导出命令函数
pub use init::cmd_init;
pub use test::cmd_test;
pub use workspace::cmd_workspace;
pub use new::cmd_new;