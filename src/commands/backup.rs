use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::BackupCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: BackupCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        BackupCommand::Create {
            vmid,
            storage,
            mode,
            compress,
            notes,
        } => {
            let vmid_s = vmid.to_string();
            let mut params: Vec<(&str, &str)> = vec![
                ("vmid", &vmid_s),
                ("storage", &storage),
                ("mode", &mode),
                ("compress", &compress),
            ];
            if let Some(ref n) = notes {
                params.push(("notes-template", n));
            }

            let data = api
                .post(&format!("/nodes/{}/vzdump", api.node()), &params)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Backup of VM/CT {} completed.", vmid);
        }
        BackupCommand::List { storage } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/storage/{}/content?content=backup",
                    api.node(),
                    storage
                ))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("volid", "VOLID"),
                    ("size", "SIZE"),
                    ("ctime", "CREATED"),
                    ("notes", "NOTES"),
                ],
            );
        }
        BackupCommand::Restore {
            archive,
            vmid,
            storage,
            force,
        } => {
            if !yes
                && !output::confirm(&format!(
                    "Restore '{}' to VM/CT {}?",
                    archive, vmid
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }

            let vmid_s = vmid.to_string();
            let mut params: Vec<(&str, &str)> = vec![
                ("vmid", &vmid_s),
                ("archive", &archive),
                ("storage", &storage),
            ];
            if force {
                params.push(("force", "1"));
            }

            // Determine restore endpoint based on archive name
            let endpoint = if archive.contains("vzdump-lxc") {
                format!("/nodes/{}/lxc", api.node())
            } else {
                format!("/nodes/{}/qemu", api.node())
            };

            let data = api.post(&endpoint, &params).await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Restore to VM/CT {} completed.", vmid);
        }
        BackupCommand::Delete { volid } => {
            if !yes && !output::confirm(&format!("Delete backup '{}'?", volid)) {
                eprintln!("Cancelled.");
                return Ok(());
            }

            // Extract storage name from volid (format: storage:backup/filename)
            let storage = volid
                .split(':')
                .next()
                .unwrap_or("bulk-backup");

            api.delete(&format!(
                "/nodes/{}/storage/{}/content/{}",
                api.node(),
                storage,
                volid
            ))
            .await?;
            eprintln!("Backup '{}' deleted.", volid);
        }
        BackupCommand::Jobs => {
            let data = api.get("/cluster/backup").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("id", "ID"),
                    ("vmid", "VMID"),
                    ("storage", "STORAGE"),
                    ("schedule", "SCHEDULE"),
                    ("mode", "MODE"),
                    ("compress", "COMPRESS"),
                    ("enabled", "ENABLED"),
                ],
            );
        }
        BackupCommand::JobCreate {
            vmid,
            storage,
            schedule,
            mode,
            compress,
            mailnotification,
            enabled,
        } => {
            let enabled_s = if enabled { "1" } else { "0" };
            let mut params: Vec<(&str, &str)> = vec![
                ("vmid", &vmid),
                ("storage", &storage),
                ("schedule", &schedule),
                ("mode", &mode),
                ("compress", &compress),
                ("enabled", enabled_s),
            ];
            if let Some(ref m) = mailnotification {
                params.push(("mailnotification", m));
            }

            api.post("/cluster/backup", &params).await?;
            eprintln!("Backup job created.");
        }
    }
    Ok(())
}
