// src/main.rs
mod cli;
mod project;
mod build;

use clap::Parser;
use cli::{Cli, Commands};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cwd = env::current_dir()?;
    match cli.command {
        Commands::New { name } => cmd_new(&cwd, &name).await?,
        Commands::Build => cmd_build(&cwd).await?,
        Commands::Run => cmd_run(&cwd).await?,
    }
    Ok(())
}