// src/cli.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "scargo")]
#[command(about = "A Cargo-like build tool for Scala")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new Scala project")]
    New {
        /// Name of the new project
        name: String
    },
    #[command(about = "Build the Scala project")]
    Build,
    #[command(about = "Run the Scala project or a specific file")]
    Run {
        /// Optional .scala file to run (relative to project root)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Force library mode (compile only)
        #[arg(long)]
        lib: bool,
    },
    #[command(about = "Add a dependency to the project")]
    Add {
        /// Dependency in format: group::artifact[@scala-version][:version]
        #[arg(value_name = "DEP")]
        dep: String,
    },
}