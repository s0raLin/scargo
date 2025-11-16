use scargo::{run, Cli};
use clap::Parser;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cwd = env::current_dir()?;
    run(cli, cwd).await?;
    Ok(())
}
