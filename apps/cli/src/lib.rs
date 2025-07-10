use clap::{Parser, Subcommand};

mod cmds;
mod utils;

#[derive(Parser)]
#[command(name = "booqer", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Upload { path: String },
}

pub async fn run() -> anyhow::Result<()> {
    let ctx = core::AppContext::new(core::AppConfig::load()?)
        .await?;
    let cli = Cli::parse();

    match cli.command {
        Command::Upload { path } => cmds::upload::handle_upload(&ctx, path).await,
    }
}
