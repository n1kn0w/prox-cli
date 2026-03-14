use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::BulkCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: BulkCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        BulkCommand::StartAll { vms, force } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(ref vms) = vms {
                params.push(("vms", vms.clone()));
            }
            if force {
                params.push(("force", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            eprintln!("Starting all VMs/CTs...");
            let data = api
                .post(
                    &format!("/nodes/{}/startall", api.node()),
                    &refs,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("All VMs/CTs started.");
        }
        BulkCommand::StopAll { vms, force_stop } => {
            let label = match &vms {
                Some(ids) => format!("VMs/CTs {}", ids),
                None => "ALL VMs/CTs".to_string(),
            };
            if !yes && !output::confirm(&format!("Stop {}?", label)) {
                eprintln!("Cancelled.");
                return Ok(());
            }

            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(ref vms) = vms {
                params.push(("vms", vms.clone()));
            }
            if force_stop {
                params.push(("force-stop", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            eprintln!("Stopping {}...", label);
            let data = api
                .post(
                    &format!("/nodes/{}/stopall", api.node()),
                    &refs,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("{} stopped.", label);
        }
        BulkCommand::MigrateAll {
            target,
            vms,
            with_local_disks,
        } => {
            let mut params: Vec<(&str, String)> = vec![("target", target.clone())];
            if let Some(ref vms) = vms {
                params.push(("vms", vms.clone()));
            }
            if with_local_disks {
                params.push(("with-local-disks", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            eprintln!("Migrating all VMs/CTs to node '{}'...", target);
            let data = api
                .post(
                    &format!("/nodes/{}/migrateall", api.node()),
                    &refs,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("All VMs/CTs migrated to '{}'.", target);
        }
        BulkCommand::SuspendAll { vms } => {
            let label = match &vms {
                Some(ids) => format!("VMs/CTs {}", ids),
                None => "ALL VMs/CTs".to_string(),
            };
            if !yes && !output::confirm(&format!("Suspend {}?", label)) {
                eprintln!("Cancelled.");
                return Ok(());
            }

            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(ref vms) = vms {
                params.push(("vms", vms.clone()));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            eprintln!("Suspending {}...", label);
            let data = api
                .post(
                    &format!("/nodes/{}/suspendall", api.node()),
                    &refs,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("{} suspended.", label);
        }
    }

    if json {
        println!("{{\"status\": \"ok\"}}");
    }

    Ok(())
}
