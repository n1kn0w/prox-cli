mod api;
mod cli;
mod commands;
mod config;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use colored::Colorize;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {:?}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    // Commands that don't need API connection
    if let Commands::Completions { shell } = &cli.command {
        let mut cmd = Cli::command();
        generate(*shell, &mut cmd, "prox-cli", &mut std::io::stdout());
        return Ok(());
    }
    if let Commands::Conf { command } = cli.command {
        return match command {
            cli::ConfCommand::List => config::profile_list(),
            cli::ConfCommand::Use { name } => config::profile_use(&name),
            cli::ConfCommand::Show => config::profile_show(),
            cli::ConfCommand::Add { name, from } => config::profile_add(&name, &from),
            cli::ConfCommand::Remove { name } => config::profile_remove(&name),
        };
    }

    let config = config::Config::load(cli.config.as_deref())?;
    let api = api::ProxmoxClient::connect(&config.proxmox, cli.verbose).await?;

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
        Commands::Task { command } => {
            commands::task::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Backup { command } => {
            commands::backup::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Node { command } => commands::node::handle(&api, command, cli.json).await,
        Commands::Pool { command } => {
            commands::pool::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Firewall { command } => {
            commands::firewall::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Apt { command } => commands::apt::handle(&api, command, cli.json).await,
        Commands::Agent { command } => commands::agent::handle(&api, command, cli.json).await,
        Commands::Disk { command } => {
            commands::disk::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Group { command } => {
            commands::group::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Tfa { command } => {
            commands::tfa::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Domain { command } => {
            commands::domain::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::NodeFirewall { command } => {
            commands::node_firewall::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Console { command } => {
            commands::console::handle(&api, command, cli.json).await
        }
        Commands::Bulk { command } => {
            commands::bulk::handle(&api, command, cli.json, cli.yes).await
        }
        Commands::Hardware { command } => {
            commands::hardware::handle(&api, command, cli.json).await
        }
        Commands::Syslog { command } => {
            let proxy = config.ssh.as_ref().and_then(|s| s.proxy.clone());
            commands::syslog::handle(
                &api,
                command,
                cli.json,
                cli.yes,
                &config.proxmox.host,
                proxy.as_deref(),
            )
            .await
        }
        Commands::Scan { command } => {
            commands::scan::handle(&api, command, cli.json).await
        }
        Commands::Status => commands::status::handle(&api, cli.json).await,
        Commands::Ssh {
            vmid,
            user,
            interface,
            proxy,
        } => {
            let proxy = proxy.or_else(|| {
                config.ssh.as_ref().and_then(|s| s.proxy.clone())
            });
            commands::ssh::handle(&api, vmid, &user, interface.as_deref(), proxy.as_deref()).await
        }
        Commands::SnapAll {
            name,
            running_only,
        } => commands::snap_all::handle_snap_all(&api, &name, running_only).await,
        Commands::RollbackAll { name } => {
            commands::snap_all::handle_rollback_all(&api, &name, cli.yes).await
        }
        Commands::Conf { .. } => unreachable!(),
        Commands::Completions { .. } => unreachable!(),
    }
}
