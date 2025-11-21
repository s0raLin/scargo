// src/cli.rs
use clap::{Arg, Command};
use std::path::PathBuf;

pub struct Cli {
    pub command: Option<crate::cmd::Commands>,
    pub raw_matches: clap::ArgMatches,
}

impl Cli {

    // 辅助函数：安全提取必需的字符串参数
    fn extract_required_string(matches: &clap::ArgMatches, key: &str) -> String {
        matches.get_one::<String>(key).unwrap().clone()
    }

    // 辅助函数：安全提取可选的字符串参数并转换为PathBuf
    fn extract_optional_path(matches: &clap::ArgMatches, key: &str) -> Option<PathBuf> {
        matches.get_one::<String>(key).map(|s| PathBuf::from(s))
    }

    pub fn parse() -> Self {
        Self::parse_with_plugins(&[])
    }

    fn parse_command_from_matches(matches: &clap::ArgMatches) -> Option<crate::cmd::Commands> {
        match matches.subcommand() {
            Some(("new", sub_m)) => Some(crate::cmd::Commands::New {
                name: Self::extract_required_string(sub_m, "name"),
            }),
            Some(("init", _)) => Some(crate::cmd::Commands::Init),
            Some(("build", _)) => Some(crate::cmd::Commands::Build),
            Some(("run", sub_m)) => Some(crate::cmd::Commands::Run {
                file: Self::extract_optional_path(sub_m, "file"),
                lib: sub_m.get_flag("lib"),
            }),
            Some(("add", sub_m)) => Some(crate::cmd::Commands::Add {
                deps: sub_m.get_many::<String>("dep").unwrap_or_default().map(|s| s.to_string()).collect(),
            }),
            Some(("test", sub_m)) => Some(crate::cmd::Commands::Test {
                file: Self::extract_optional_path(sub_m, "file"),
            }),
            Some(("workspace", ws_m)) => match ws_m.subcommand() {
                Some(("add", sub_m)) => Some(crate::cmd::Commands::Workspace {
                    subcommand: crate::cmd::WorkspaceCommands::Add {
                        paths: sub_m.get_many::<String>("path").unwrap_or_default().map(|s| s.to_string()).collect(),
                    }
                }),
                _ => None,
            },
            Some(("jsp", sub_m)) => Some(crate::cmd::Commands::Jsp {
                name: Self::extract_required_string(sub_m, "name"),
            }),
            _ => None,
        }
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

        let command = Self::parse_command_from_matches(&matches);

        Cli { command, raw_matches: matches }
    }
}