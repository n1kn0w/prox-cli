use anyhow::{bail, Result};
use colored::Colorize;
use futures::future::join_all;

use crate::api::ProxmoxClient;
use crate::output;

struct Guest {
    vmid: u32,
    name: String,
    kind: &'static str, // "qemu" or "lxc"
    status: String,
}

async fn list_guests(api: &ProxmoxClient) -> Result<Vec<Guest>> {
    let node = api.node().to_string();
    let qemu_path = format!("/nodes/{}/qemu", node);
    let lxc_path = format!("/nodes/{}/lxc", node);
    let (qemu, lxc) = tokio::try_join!(
        api.get(&qemu_path),
        api.get(&lxc_path),
    )?;

    let mut guests = Vec::new();
    if let Some(vms) = qemu.as_array() {
        for vm in vms {
            guests.push(Guest {
                vmid: vm["vmid"].as_u64().unwrap_or(0) as u32,
                name: vm["name"].as_str().unwrap_or("").to_string(),
                kind: "qemu",
                status: vm["status"].as_str().unwrap_or("").to_string(),
            });
        }
    }
    if let Some(cts) = lxc.as_array() {
        for ct in cts {
            guests.push(Guest {
                vmid: ct["vmid"].as_u64().unwrap_or(0) as u32,
                name: ct["name"].as_str().unwrap_or("").to_string(),
                kind: "lxc",
                status: ct["status"].as_str().unwrap_or("").to_string(),
            });
        }
    }
    guests.sort_by_key(|g| g.vmid);
    Ok(guests)
}

pub async fn handle_snap_all(
    api: &ProxmoxClient,
    name: &str,
    running_only: bool,
) -> Result<()> {
    let guests = list_guests(api).await?;
    let targets: Vec<&Guest> = if running_only {
        guests.iter().filter(|g| g.status == "running").collect()
    } else {
        guests.iter().collect()
    };

    if targets.is_empty() {
        println!("No guests to snapshot.");
        return Ok(());
    }

    eprintln!(
        "Snapshotting {} guests with name '{}'...",
        targets.len(),
        name
    );

    let futs: Vec<_> = targets
        .iter()
        .map(|g| {
            let path = format!(
                "/nodes/{}/{}/{}/snapshot",
                api.node(),
                g.kind,
                g.vmid
            );
            let vmid = g.vmid;
            let guest_name = g.name.clone();
            let snap_name = name.to_string();
            async move {
                let result = api
                    .post(&path, &[("snapname", snap_name.as_str())])
                    .await;
                (vmid, guest_name, result)
            }
        })
        .collect();

    let results = join_all(futs).await;

    let mut errors = 0;
    for (vmid, guest_name, result) in &results {
        match result {
            Ok(_) => eprintln!(
                "  {} {} ({}) — {}",
                "✓".green(),
                vmid,
                guest_name,
                "snapshot created".green()
            ),
            Err(e) => {
                errors += 1;
                eprintln!(
                    "  {} {} ({}) — {}",
                    "✗".red(),
                    vmid,
                    guest_name,
                    e.to_string().red()
                );
            }
        }
    }

    if errors > 0 {
        bail!("{} snapshot(s) failed", errors);
    }

    eprintln!("All snapshots created successfully.");
    Ok(())
}

pub async fn handle_rollback_all(
    api: &ProxmoxClient,
    name: &str,
    yes: bool,
) -> Result<()> {
    let guests = list_guests(api).await?;

    if guests.is_empty() {
        println!("No guests found.");
        return Ok(());
    }

    eprintln!(
        "Will rollback {} guests to snapshot '{}'.",
        guests.len(),
        name
    );

    if !yes
        && !output::confirm(&format!(
            "Rollback ALL {} guests to snapshot '{}'? This will stop running guests.",
            guests.len(),
            name
        ))
    {
        bail!("Aborted.");
    }

    let futs: Vec<_> = guests
        .iter()
        .map(|g| {
            let path = format!(
                "/nodes/{}/{}/{}/snapshot/{}/rollback",
                api.node(),
                g.kind,
                g.vmid,
                name
            );
            let vmid = g.vmid;
            let guest_name = g.name.clone();
            async move {
                let result = api.post(&path, &[]).await;
                (vmid, guest_name, result)
            }
        })
        .collect();

    let results = join_all(futs).await;

    let mut errors = 0;
    for (vmid, guest_name, result) in &results {
        match result {
            Ok(_) => eprintln!(
                "  {} {} ({}) — {}",
                "✓".green(),
                vmid,
                guest_name,
                "rolled back".green()
            ),
            Err(e) => {
                errors += 1;
                eprintln!(
                    "  {} {} ({}) — {}",
                    "✗".red(),
                    vmid,
                    guest_name,
                    e.to_string().red()
                );
            }
        }
    }

    if errors > 0 {
        bail!("{} rollback(s) failed", errors);
    }

    eprintln!("All rollbacks completed successfully.");
    Ok(())
}
