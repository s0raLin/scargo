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

Compiled project (Scala 3.7.3, JVM (21))
[hint]  "upickle_2.13 is outdated, update to 4.4.1"
     upickle_2.13 3.1.4 -> com.lihaoyi:upickle_2.13:4.4.1
[hint]  "cats-core_2.13 is outdated, update to 2.13.0"
     cats-core_2.13 2.10.0 -> org.typelevel:cats-core_2.13:2.13.0
[hint]  "zio_2.13 is outdated, update to 2.1.22"
     zio_2.13 2.0.22 -> dev.zio:zio_2.13:2.1.22
Build succeeded with 3 dependencies
}