use anyhow::{bail, Result};

use crate::api::ProxmoxClient;
use crate::cli::VmCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: VmCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        VmCommand::List => {
            let data = api.get(&format!("/nodes/{}/qemu", api.node())).await?;
            output::print_list(
                &data,
                json,
                &[
                    ("vmid", "VMID"),
                    ("name", "NAME"),
                    ("status", "STATUS"),
                    ("cpus", "CPUS"),
                    ("maxmem", "MAXMEM"),
                    ("maxdisk", "MAXDISK"),
                ],
            );
        }
        VmCommand::Status { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/status/current",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("vmid", "VMID"),
                    ("name", "Name"),
                    ("status", "Status"),
                    ("cpus", "CPUs"),
                    ("mem", "Memory (used)"),
                    ("maxmem", "Memory (max)"),
                    ("disk", "Disk (used)"),
                    ("maxdisk", "Disk (max)"),
                    ("uptime", "Uptime"),
                    ("pid", "PID"),
                    ("qmpstatus", "QMP Status"),
                ],
            );
        }
        VmCommand::Start { vmid } => {
            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/status/start", api.node(), vmid),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} started.", vmid);
        }
        VmCommand::Stop { vmid } => {
            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/status/stop", api.node(), vmid),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} stopped.", vmid);
        }
        VmCommand::Shutdown { vmid } => {
            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/status/shutdown", api.node(), vmid),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} shutdown signal sent.", vmid);
        }
        VmCommand::Config { vmid } => {
            let data = api
                .get(&format!("/nodes/{}/qemu/{}/config", api.node(), vmid))
                .await?;
            output::print_raw(&data, json);
        }
        VmCommand::Set {
            vmid,
            name,
            memory,
            cores,
            sockets,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = name {
                params.push(("name", v));
            }
            if let Some(v) = memory {
                params.push(("memory", v.to_string()));
            }
            if let Some(v) = cores {
                params.push(("cores", v.to_string()));
            }
            if let Some(v) = sockets {
                params.push(("sockets", v.to_string()));
            }
            if params.is_empty() {
                bail!("No parameters specified. Use --name, --memory, --cores, or --sockets.");
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(
                &format!("/nodes/{}/qemu/{}/config", api.node(), vmid),
                &refs,
            )
            .await?;
            eprintln!("VM {} configuration updated.", vmid);
        }
        VmCommand::Create {
            vmid,
            name,
            memory,
            cores,
            storage,
            disk,
            iso,
            bridge,
            vlan,
            ostype,
            start,
        } => {
            let vmid_s = vmid.to_string();
            let memory_s = memory.to_string();
            let cores_s = cores.to_string();
            let scsi0 = format!("{}:{}", storage, disk);
            let mut net0 = format!("virtio,bridge={}", bridge);
            if let Some(tag) = vlan {
                net0.push_str(&format!(",tag={}", tag));
            }

            let mut params: Vec<(&str, &str)> = vec![
                ("vmid", &vmid_s),
                ("memory", &memory_s),
                ("cores", &cores_s),
                ("ostype", &ostype),
                ("scsi0", &scsi0),
                ("net0", &net0),
                ("scsihw", "virtio-scsi-pci"),
            ];
            if let Some(ref n) = name {
                params.push(("name", n));
            }
            if let Some(ref i) = iso {
                params.push(("ide2", i));
            }

            let data = api
                .post(&format!("/nodes/{}/qemu", api.node()), &params)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} created.", vmid);

            if start {
                let data = api
                    .post(
                        &format!("/nodes/{}/qemu/{}/status/start", api.node(), vmid),
                        &[],
                    )
                    .await?;
                if let Some(upid) = data.as_str() {
                    api.wait_task(upid).await?;
                }
                eprintln!("VM {} started.", vmid);
            }
        }
        VmCommand::Delete { vmid, purge } => {
            if !yes && !output::confirm(&format!("Delete VM {}?", vmid)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let path = if purge {
                format!("/nodes/{}/qemu/{}?purge=1", api.node(), vmid)
            } else {
                format!("/nodes/{}/qemu/{}", api.node(), vmid)
            };
            let data = api.delete(&path).await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} deleted.", vmid);
        }
        VmCommand::Clone {
            vmid,
            newid,
            name,
            full,
            storage,
        } => {
            let newid_s = newid.to_string();
            let mut params: Vec<(&str, &str)> = vec![("newid", &newid_s)];
            if let Some(ref n) = name {
                params.push(("name", n));
            }
            if full {
                params.push(("full", "1"));
            }
            if let Some(ref s) = storage {
                params.push(("storage", s));
            }

            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/clone", api.node(), vmid),
                    &params,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} cloned to {}.", vmid, newid);
        }
        VmCommand::Snapshot { vmid, name } => {
            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/snapshot", api.node(), vmid),
                    &[("snapname", name.as_str())],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Snapshot '{}' created for VM {}.", name, vmid);
        }
        VmCommand::Rollback { vmid, name } => {
            if !yes
                && !output::confirm(&format!("Rollback VM {} to snapshot '{}'?", vmid, name))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .post(
                    &format!(
                        "/nodes/{}/qemu/{}/snapshot/{}/rollback",
                        api.node(),
                        vmid,
                        name
                    ),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("VM {} rolled back to '{}'.", vmid, name);
        }
        VmCommand::Snapshots { vmid } => {
            let data = api
                .get(&format!("/nodes/{}/qemu/{}/snapshot", api.node(), vmid))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("snaptime", "TIME"),
                    ("description", "DESCRIPTION"),
                ],
            );
        }
    }
    Ok(())
}
