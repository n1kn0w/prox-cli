mod api;
mod cli;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Shell completions don't need API connection
    if let Commands::Completions { shell } = &cli.command {
        let mut cmd = Cli::command();
        generate(*shell, &mut cmd, "prox-cli", &mut std::io::stdout());
        return Ok(());
    }

    let config = config::Config::load(&cli.config)?;
    let api = api::ProxmoxClient::connect(&config.proxmox).await?;

    match cli.command {
        Commands::Vm { command } => commands::vm::handle(&api, command, cli.json, cli.yes).await,
        Commands::Ct { command } => commands::ct::handle(&api, command, cli.json, cli.yes).await,
        Commands::Storage { command } => commands::storage::handle(&api, command, cli.json).await,
        Commands::Network { command } => {
            commands::network::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::User { command } => {
            commands::user::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Template { command } => {
            commands::template::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Completions { .. } => unreachable!(),
    }
}
