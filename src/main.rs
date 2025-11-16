use scargo::{run, Cli};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化国际化系统
    let i18n = &scargo::i18n::I18N;

    let args: Vec<String> = env::args().collect();
    let cli = Cli::parse_from(i18n, &args);
    let cwd = env::current_dir()?;
    run(cli, cwd).await?;
    Ok(())
}
