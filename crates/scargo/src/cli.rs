// src/cli.rs
use clap::{Arg, Command};
use std::path::PathBuf;

pub struct Cli {
    pub command: Option<Commands>,
    pub raw_matches: clap::ArgMatches,
}

pub enum Commands {
    New {
        name: String
    },
    Init,
    Build,
    Run {
        file: Option<PathBuf>,
        lib: bool,
    },
    Add {
        dep: String,
    },
    Test {
        file: Option<PathBuf>,
    },
    Workspace {
        subcommand: WorkspaceCommands,
    },
    Jsp {
        name: String,
    },
}

pub enum WorkspaceCommands {
    Add {
        path: String,
    },
}

impl Cli {

    pub fn parse() -> Self {
        Self::parse_with_plugins(&[])
    }

    pub fn parse_with_plugins(plugins: &[Box<dyn crate::core::CommandHandler>]) -> Self {
        let mut cmd = Command::new("scargo")
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
                                    .required(true)
                            )
                    )
            );

        // 自动添加所有插件命令
        for handler in plugins {
            cmd = cmd.subcommand(handler.configure(Command::new(handler.name())));
        }

        let matches = cmd.get_matches();

        let command = if let Some(sub_m) = matches.subcommand_matches("new") {
            Some(Commands::New {
                name: sub_m.get_one::<String>("name").unwrap().clone(),
            })
        } else if matches.subcommand_matches("init").is_some() {
            Some(Commands::Init)
        } else if matches.subcommand_matches("build").is_some() {
            Some(Commands::Build)
        } else if let Some(sub_m) = matches.subcommand_matches("run") {
            Some(Commands::Run {
                file: sub_m.get_one::<String>("file").map(|s| PathBuf::from(s)),
                lib: sub_m.get_flag("lib"),
            })
        } else if let Some(sub_m) = matches.subcommand_matches("add") {
            Some(Commands::Add {
                dep: sub_m.get_one::<String>("dep").unwrap().clone(),
            })
        } else if let Some(sub_m) = matches.subcommand_matches("test") {
            Some(Commands::Test {
                file: sub_m.get_one::<String>("file").map(|s| PathBuf::from(s)),
            })
        } else if let Some(ws_m) = matches.subcommand_matches("workspace") {
            if let Some(sub_m) = ws_m.subcommand_matches("add") {
                Some(Commands::Workspace {
                    subcommand: WorkspaceCommands::Add {
                        path: sub_m.get_one::<String>("path").unwrap().clone(),
                    }
                })
            } else {
                None
            }
        } else if let Some(sub_m) = matches.subcommand_matches("jsp") {
            Some(Commands::Jsp {
                name: sub_m.get_one::<String>("name").unwrap().clone(),
            })
        } else {
            None
        };

        Cli { command, raw_matches: matches }
    }
}