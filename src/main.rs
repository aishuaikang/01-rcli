use clap::Parser;
use rcli::{cli::Opts, CmdExector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Opts::parse();
    args.command.execute().await?;
    Ok(())
}
