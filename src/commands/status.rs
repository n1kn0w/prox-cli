use anyhow::Result;
use colored::Colorize;

use crate::api::ProxmoxClient;
use crate::output;

pub async fn handle(api: &ProxmoxClient, json: bool) -> Result<()> {
    let node = api.node().to_string();
    let status_path = format!("/nodes/{}/status", node);
    let qemu_path = format!("/nodes/{}/qemu", node);
    let lxc_path = format!("/nodes/{}/lxc", node);
    let (node_status, qemu_list, lxc_list) = tokio::try_join!(
        api.get(&status_path),
        api.get(&qemu_path),
        api.get(&lxc_path),
    )?;

    if json {
        let combined = serde_json::json!({
            "node": node_status,
            "qemu": qemu_list,
            "lxc": lxc_list,
        });
        println!("{}", serde_json::to_string_pretty(&combined).unwrap_or_default());
        return Ok(());
    }

    // Node info
    println!("{}", format!("── Node: {} ──", node).bold());
    let cpu = node_status["cpu"].as_f64().unwrap_or(0.0);
    let cpus = node_status["cpuinfo"]["cpus"].as_u64().unwrap_or(1);
    let mem_used = node_status["memory"]["used"].as_u64().unwrap_or(0);
    let mem_total = node_status["memory"]["total"].as_u64().unwrap_or(1);
    let root_used = node_status["rootfs"]["used"].as_u64().unwrap_or(0);
    let root_total = node_status["rootfs"]["total"].as_u64().unwrap_or(1);

    println!(
        "  CPU: {:.1}% ({} cores)",
        cpu * 100.0,
        cpus
    );
    println!(
        "  RAM: {} / {} ({:.0}%)",
        format_bytes(mem_used),
        format_bytes(mem_total),
        mem_used as f64 / mem_total as f64 * 100.0
    );
    println!(
        " Disk: {} / {} ({:.0}%)",
        format_bytes(root_used),
        format_bytes(root_total),
        root_used as f64 / root_total as f64 * 100.0
    );

    // VM summary
    let (vm_running, vm_stopped) = count_status(&qemu_list);
    let (ct_running, ct_stopped) = count_status(&lxc_list);

    println!();
    println!("{}", "── Guests ──".bold());
    println!(
        "   VMs: {} {} / {} {}",
        vm_running,
        "running".green(),
        vm_stopped,
        "stopped".red()
    );
    println!(
        "   CTs: {} {} / {} {}",
        ct_running,
        "running".green(),
        ct_stopped,
        "stopped".red()
    );

    // Detail table
    let mut all_guests: Vec<serde_json::Value> = Vec::new();
    if let Some(vms) = qemu_list.as_array() {
        for vm in vms {
            let mut v = vm.clone();
            v["_type"] = serde_json::Value::String("qemu".to_string());
            all_guests.push(v);
        }
    }
    if let Some(cts) = lxc_list.as_array() {
        for ct in cts {
            let mut c = ct.clone();
            c["_type"] = serde_json::Value::String("lxc".to_string());
            all_guests.push(c);
        }
    }

    if !all_guests.is_empty() {
        all_guests.sort_by_key(|g| g["vmid"].as_u64().unwrap_or(0));
        println!();
        let list = serde_json::Value::Array(all_guests);
        output::print_list(
            &list,
            false,
            &[
                ("vmid", "VMID"),
                ("_type", "TYPE"),
                ("name", "NAME"),
                ("status", "STATUS"),
                ("cpus", "CPUS"),
                ("maxmem", "MAXMEM"),
            ],
        );
    }

    Ok(())
}

fn count_status(data: &serde_json::Value) -> (usize, usize) {
    let items = match data.as_array() {
        Some(arr) => arr,
        None => return (0, 0),
    };
    let running = items
        .iter()
        .filter(|i| i["status"].as_str() == Some("running"))
        .count();
    let stopped = items.len() - running;
    (running, stopped)
}

fn format_bytes(bytes: u64) -> String {
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.1} GB", b / GB)
    } else {
        format!("{:.0} MB", b / MB)
    }
}
