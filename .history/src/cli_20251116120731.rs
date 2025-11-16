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
    New { name: String },
    Build,
    Run {
        /// Optional .scala file to run (relative to project root)
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Force library mode (compile only)
        #[arg(long)]
        lib: bool,
    },
    Add {
        /// Dependency in format: group::artifact[@scala-version][:version]
        #[arg(value_name = "DEP")]
        dep: String,
    }
}