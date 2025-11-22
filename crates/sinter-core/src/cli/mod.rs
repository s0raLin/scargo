// src/cli/mod.rs
use clap::{Arg, Command};

pub mod commands;
pub mod builtin;
pub mod parser;



#[derive(Debug)]
pub struct Cli {
    pub command: Option<Commands>,
    pub raw_matches: clap::ArgMatches,
}

impl std::ops::Deref for Cli {
    type Target = Option<Commands>;

    fn deref(&self) -> &Self::Target {
        &self.command
    }
}


// 命令枚举定义
#[derive(Debug, Clone)]
pub enum Commands {
    New {name: String},
    Init,
    Build,
    Run {file: Option<std::path::PathBuf>,lib: bool,},
    Add {deps: Vec<String>,},
    Test {file: Option<std::path::PathBuf>,},
    Workspace {subcommand: WorkspaceCommands,},
    Jsp {name: String,},
}


#[derive(clap::Subcommand, Debug, Clone)]
pub enum WorkspaceCommands {
    Add {paths: Vec<String>,},
}

impl Cli {
    pub fn parse() -> Self {
        Self::parse_with_plugins(&[])
    }

    pub fn parse_with_plugins(plugins: &[Box<dyn crate::core::CommandHandler>]) -> Self {
        let mut cmd = Command::new("sinter")
            .about(crate::i18n::t("main_about"))
            .subcommand(
                Command::new("new")
                    .about(crate::i18n::t("new_about"))
                    .arg(
                        Arg::new("name")
                            .help(crate::i18n::t("new_name_help"))
                            .required(true)
                    )
            )
            .subcommand(
                Command::new("init")
                    .about(crate::i18n::t("init_about"))
            )
            .subcommand(
                Command::new("build")
                    .about(crate::i18n::t("build_about"))
            )
            .subcommand(
                Command::new("run")
                    .about(crate::i18n::t("run_about"))
                    .arg(
                        Arg::new("file")
                            .help(crate::i18n::t("run_file_help"))
                            .value_name("FILE")
                    )
                    .arg(
                        Arg::new("lib")
                            .long("lib")
                            .help(crate::i18n::t("run_lib_help"))
                            .action(clap::ArgAction::SetTrue)
                    )
            )
            .subcommand(
                Command::new("add")
                    .about(crate::i18n::t("add_about"))
                    .arg(
                        Arg::new("dep")
                            .help(crate::i18n::t("add_dep_help"))
                            .value_name("DEP")
                            .required(true)
                            .num_args(1..)
                    )
            )
            .subcommand(
                Command::new("test")
                    .about(crate::i18n::t("test_about"))
                    .arg(
                        Arg::new("file")
                            .help(crate::i18n::t("test_file_help"))
                            .value_name("FILE")
                    )
            )
            .subcommand(
                Command::new("workspace")
                    .about(crate::i18n::t("workspace_about"))
                    .subcommand(
                        Command::new("add")
                            .about(crate::i18n::t("workspace_add_about"))
                            .arg(
                                Arg::new("path")
                                    .help(crate::i18n::t("workspace_add_path_help"))
                                    .value_name("PATH")
                                    .required(true)
                                    .num_args(1..)
                            )
                    )
            );

        // 自动添加所有插件命令
        for handler in plugins {
            cmd = cmd.subcommand(handler.configure(Command::new(handler.name())));
        }

        let matches = cmd.get_matches();

        let command = parser::parse_command_from_matches(&matches);

        Cli { command, raw_matches: matches }
    }
}