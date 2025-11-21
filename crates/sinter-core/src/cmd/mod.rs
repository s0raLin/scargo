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

// 命令枚举定义
#[derive(Debug)]
pub enum Commands {
    New {
        name: String
    },
    Init,
    Build,
    Run {
        file: Option<std::path::PathBuf>,
        lib: bool,
    },
    Add {
        deps: Vec<String>,
    },
    Test {
        file: Option<std::path::PathBuf>,
    },
    Workspace {
        subcommand: WorkspaceCommands,
    },
    Jsp {
        name: String,
    },
}

#[derive(Debug)]
pub enum WorkspaceCommands {
    Add {
        paths: Vec<String>,
    },
}