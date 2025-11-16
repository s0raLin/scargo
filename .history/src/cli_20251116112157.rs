// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scargo")]
#[command(about = "A Cargo-like build tool for Scala")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new Scala project
    New { name: String },
    /// Build the project
    Build,
    /// Run the main class
    Run,
}