use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::StorageCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: StorageCommand, json: bool) -> Result<()> {
    match cmd {
        StorageCommand::List => {
            let data = api.get("/storage").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("storage", "STORAGE"),
                    ("type", "TYPE"),
                    ("content", "CONTENT"),
                    ("path", "PATH"),
                    ("pool", "POOL"),
                ],
            );
        }
        StorageCommand::Pools => {
            let data = api
                .get(&format!("/nodes/{}/disks/zfs", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("size", "SIZE"),
                    ("free", "FREE"),
                    ("alloc", "ALLOC"),
                    ("health", "HEALTH"),
                ],
            );
        }
        StorageCommand::Disks => {
            let data = api
                .get(&format!("/nodes/{}/disks/list", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("devpath", "DEVICE"),
                    ("model", "MODEL"),
                    ("serial", "SERIAL"),
                    ("size", "SIZE"),
                    ("type", "TYPE"),
                    ("health", "HEALTH"),
                ],
            );
        }
        StorageCommand::Status => {
            let data = api
                .get(&format!("/nodes/{}/storage", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("storage", "STORAGE"),
                    ("type", "TYPE"),
                    ("content", "CONTENT"),
                    ("total", "TOTAL"),
                    ("used", "USED"),
                    ("avail", "AVAILABLE"),
                    ("active", "ACTIVE"),
                ],
            );
        }
    }
    Ok(())
}
