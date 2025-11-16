// src/cli.rs
use clap::{Arg, ArgMatches, Command};
use std::path::PathBuf;

pub struct Cli {
    pub command: Option<Commands>,
}

pub enum Commands {
    New {
        name: String,
    },
    Build,
    Check,
    Run {
        file: Option<PathBuf>,
        lib: bool,
    },
    Add {
        dep: String,
    },
    Remove {
        dep: String,
    },
    Update {
        dep: Option<String>,
    },
    Plugin {
        name: String,
        args: Vec<String>,
    },
    Plugins,
    Test {
        test: Option<String>,
    },
    Clean,
    Info,
    Dev,
}

impl Cli {
    pub fn build_command(i18n: &crate::i18n::I18n) -> Command {
        Command::new("scargo")
            .about(i18n.get("cli.about"))
            .subcommand_required(false)
            .arg_required_else_help(false)
            .subcommand(
                Command::new("new")
                    .about(i18n.get("cmd.new.about"))
                    .arg(
                        Arg::new("name")
                            .help(i18n.get("arg.name"))
                            .required(true)
                            .value_name("NAME"),
                    ),
            )
            .subcommand(
                Command::new("build")
                    .about(i18n.get("cmd.build.about")),
            )
            .subcommand(
                Command::new("check")
                    .about(i18n.get("cmd.check.about")),
            )
            .subcommand(
                Command::new("run")
                    .about(i18n.get("cmd.run.about"))
                    .arg(
                        Arg::new("file")
                            .help(i18n.get("arg.file"))
                            .value_name("FILE"),
                    )
                    .arg(
                        Arg::new("lib")
                            .long("lib")
                            .help(i18n.get("arg.lib"))
                            .action(clap::ArgAction::SetTrue),
                    ),
            )
            .subcommand(
                Command::new("add")
                    .about(i18n.get("cmd.add.about"))
                    .arg(
                        Arg::new("dep")
                            .help(i18n.get("arg.dep"))
                            .required(true)
                            .value_name("DEP"),
                    ),
            )
            .subcommand(
                Command::new("remove")
                    .about(i18n.get("cmd.remove.about"))
                    .arg(
                        Arg::new("dep")
                            .help(i18n.get("arg.dep"))
                            .required(true)
                            .value_name("DEP"),
                    ),
            )
            .subcommand(
                Command::new("update")
                    .about(i18n.get("cmd.update.about"))
                    .arg(
                        Arg::new("dep")
                            .help(i18n.get("arg.dep"))
                            .value_name("DEP"),
                    ),
            )
            .subcommand(
                Command::new("plugin")
                    .about(i18n.get("cmd.plugin.about"))
                    .arg(
                        Arg::new("name")
                            .help(i18n.get("arg.plugin_name"))
                            .required(true)
                            .value_name("NAME"),
                    )
                    .arg(
                        Arg::new("args")
                            .help(i18n.get("arg.plugin_args"))
                            .action(clap::ArgAction::Set)
                            .num_args(0..)
                            .value_name("ARGS"),
                    ),
            )
            .subcommand(
                Command::new("plugins")
                    .about(i18n.get("cmd.plugins.about")),
            )
            .subcommand(
                Command::new("test")
                    .about(i18n.get("cmd.test.about"))
                    .arg(
                        Arg::new("test")
                            .help(i18n.get("arg.test"))
                            .value_name("TEST"),
                    ),
            )
            .subcommand(
                Command::new("clean")
                    .about(i18n.get("cmd.clean.about")),
            )
            .subcommand(
                Command::new("info")
                    .about(i18n.get("cmd.info.about")),
            )
            .subcommand(
                Command::new("dev")
                    .about(i18n.get("cmd.dev.about")),
            )
    }

    pub fn parse_from(i18n: &crate::i18n::I18n, args: &[String]) -> Self {
        let command = Self::build_command(i18n);
        let matches = command.get_matches_from(args);

        let command = match matches.subcommand() {
            Some(("new", sub_matches)) => Some(Commands::New {
                name: sub_matches.get_one::<String>("name").unwrap().clone(),
            }),
            Some(("build", _)) => Some(Commands::Build),
            Some(("check", _)) => Some(Commands::Check),
            Some(("run", sub_matches)) => Some(Commands::Run {
                file: sub_matches.get_one::<String>("file").map(PathBuf::from),
                lib: sub_matches.get_flag("lib"),
            }),
            Some(("add", sub_matches)) => Some(Commands::Add {
                dep: sub_matches.get_one::<String>("dep").unwrap().clone(),
            }),
            Some(("remove", sub_matches)) => Some(Commands::Remove {
                dep: sub_matches.get_one::<String>("dep").unwrap().clone(),
            }),
            Some(("update", sub_matches)) => Some(Commands::Update {
                dep: sub_matches.get_one::<String>("dep").cloned(),
            }),
            Some(("plugin", sub_matches)) => Some(Commands::Plugin {
                name: sub_matches.get_one::<String>("name").unwrap().clone(),
                args: sub_matches.get_many::<String>("args").unwrap_or_default().map(|s| s.clone()).collect(),
            }),
            Some(("plugins", _)) => Some(Commands::Plugins),
            Some(("test", sub_matches)) => Some(Commands::Test {
                test: sub_matches.get_one::<String>("test").cloned(),
            }),
            Some(("clean", _)) => Some(Commands::Clean),
            Some(("info", _)) => Some(Commands::Info),
            Some(("dev", _)) => Some(Commands::Dev),
            _ => None,
        };

        Cli { command }
    }
}