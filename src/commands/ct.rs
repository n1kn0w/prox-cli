use anyhow::{bail, Result};
use serde_json::Value;

use crate::api::ProxmoxClient;
use crate::cli::CtCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: CtCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        CtCommand::Templates => {
            // List all storages that support vztmpl content
            let storages = api.get("/storage").await?;
            let mut all_templates: Vec<Value> = Vec::new();

            if let Some(stores) = storages.as_array() {
                for store in stores {
                    let content = store["content"].as_str().unwrap_or("");
                    if !content.contains("vztmpl") {
                        continue;
                    }
                    let name = store["storage"].as_str().unwrap_or("");
                    let items = api
                        .get(&format!("/nodes/{}/storage/{}/content", api.node(), name))
                        .await
                        .unwrap_or(Value::Null);
                    if let Some(arr) = items.as_array() {
                        for item in arr {
                            if item["content"].as_str() == Some("vztmpl") {
                                let volid = item["volid"].as_str().unwrap_or("-");
                                let size = item["size"].as_u64().unwrap_or(0);
                                let size_mb = format!("{:.0} MB", size as f64 / 1_048_576.0);
                                all_templates.push(serde_json::json!({
                                    "ostemplate": volid,
                                    "size": size_mb,
                                    "storage": name,
                                }));
                            }
                        }
                    }
                }
            }

            let data = Value::Array(all_templates);
            output::print_list(
                &data,
                json,
                &[
                    ("ostemplate", "OSTEMPLATE (use with --ostemplate)"),
                    ("size", "SIZE"),
                    ("storage", "STORAGE"),
                ],
            );
        }
        CtCommand::List => {
            let data = api.get(&format!("/nodes/{}/lxc", api.node())).await?;
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
        CtCommand::Status { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/lxc/{}/status/current",
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
                ],
            );
        }
        CtCommand::Start { vmid } => {
            let data = api
                .post(
                    &format!("/nodes/{}/lxc/{}/status/start", api.node(), vmid),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Container {} started.", vmid);
        }
        CtCommand::Stop { vmid } => {
            let data = api
                .post(
                    &format!("/nodes/{}/lxc/{}/status/stop", api.node(), vmid),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Container {} stopped.", vmid);
        }
        CtCommand::Config { vmid } => {
            let data = api
                .get(&format!("/nodes/{}/lxc/{}/config", api.node(), vmid))
                .await?;
            output::print_raw(&data, json);
        }
        CtCommand::Set {
            vmid,
            hostname,
            memory,
            cores,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = hostname {
                params.push(("hostname", v));
            }
            if let Some(v) = memory {
                params.push(("memory", v.to_string()));
            }
            if let Some(v) = cores {
                params.push(("cores", v.to_string()));
            }
            if params.is_empty() {
                bail!("No parameters specified. Use --hostname, --memory, or --cores.");
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(
                &format!("/nodes/{}/lxc/{}/config", api.node(), vmid),
                &refs,
            )
            .await?;
            eprintln!("Container {} configuration updated.", vmid);
        }
        CtCommand::Create {
            vmid,
            ostemplate,
            hostname,
            memory,
            cores,
            storage,
            rootfs,
            bridge,
            ip,
            gw,
            vlan,
            password,
            start,
        } => {
            let vmid_s = vmid.to_string();
            let memory_s = memory.to_string();
            let cores_s = cores.to_string();
            let rootfs_s = format!("{}:{}", storage, rootfs);
            let mut net0 = format!("name=eth0,bridge={}", bridge);
            if let Some(ref addr) = ip {
                net0.push_str(&format!(",ip={}", addr));
            }
            if let Some(ref g) = gw {
                net0.push_str(&format!(",gw={}", g));
            }
            if let Some(tag) = vlan {
                net0.push_str(&format!(",tag={}", tag));
            }

            let mut params: Vec<(&str, &str)> = vec![
                ("vmid", &vmid_s),
                ("ostemplate", &ostemplate),
                ("memory", &memory_s),
                ("cores", &cores_s),
                ("rootfs", &rootfs_s),
                ("net0", &net0),
            ];
            if let Some(ref h) = hostname {
                params.push(("hostname", h));
            }
            if let Some(ref p) = password {
                params.push(("password", p));
            }

            let data = api
                .post(&format!("/nodes/{}/lxc", api.node()), &params)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Container {} created.", vmid);

            if start {
                let data = api
                    .post(
                        &format!("/nodes/{}/lxc/{}/status/start", api.node(), vmid),
                        &[],
                    )
                    .await?;
                if let Some(upid) = data.as_str() {
                    api.wait_task(upid).await?;
                }
                eprintln!("Container {} started.", vmid);
            }
        }
        CtCommand::Delete { vmid, force } => {
            if !yes && !output::confirm(&format!("Delete container {}?", vmid)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            if force {
                let _ = api
                    .post(
                        &format!("/nodes/{}/lxc/{}/status/stop", api.node(), vmid),
                        &[],
                    )
                    .await;
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
            let data = api
                .delete(&format!("/nodes/{}/lxc/{}", api.node(), vmid))
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Container {} deleted.", vmid);
        }
        CtCommand::Pull {
            reference,
            storage,
        } => {
            let parts: Vec<&str> = reference.split('/').collect();
            let last = parts.last().unwrap_or(&"image");
            let (img, tag) = match last.find(':') {
                Some(pos) => (&last[..pos], &last[pos + 1..]),
                None => (*last, "latest"),
            };

            eprintln!("Pulling {}...", reference);
            let data = api
                .post(
                    &format!(
                        "/nodes/{}/storage/{}/oci-registry-pull",
                        api.node(),
                        storage
                    ),
                    &[("reference", reference.as_str())],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Image pulled: {}_{}.tar", img, tag);
        }
        CtCommand::Snapshot { vmid, name } => {
            let data = api
                .post(
                    &format!("/nodes/{}/lxc/{}/snapshot", api.node(), vmid),
                    &[("snapname", name.as_str())],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Snapshot '{}' created for container {}.", name, vmid);
        }
        CtCommand::Rollback { vmid, name } => {
            if !yes
                && !output::confirm(&format!(
                    "Rollback container {} to snapshot '{}'?",
                    vmid, name
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .post(
                    &format!(
                        "/nodes/{}/lxc/{}/snapshot/{}/rollback",
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
            eprintln!("Container {} rolled back to '{}'.", vmid, name);
        }
        CtCommand::Snapshots { vmid } => {
            let data = api
                .get(&format!("/nodes/{}/lxc/{}/snapshot", api.node(), vmid))
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
