use anyhow::{bail, Context, Result};

use crate::api::ProxmoxClient;

pub async fn handle(
    api: &ProxmoxClient,
    vmid: u32,
    user: &str,
    interface: Option<&str>,
    proxy: Option<&str>,
) -> Result<()> {
    eprintln!("Resolving IP for VM {} via guest agent...", vmid);

    let data = api
        .get(&format!(
            "/nodes/{}/qemu/{}/agent/network-get-interfaces",
            api.node(),
            vmid
        ))
        .await
        .context("Failed to query guest agent. Is the VM running with qemu-guest-agent installed?")?;

    let interfaces = data
        .as_array()
        .context("Unexpected response from guest agent")?;

    let mut ip: Option<String> = None;

    for iface in interfaces {
        let name = iface["name"].as_str().unwrap_or("");

        // Skip loopback
        if name == "lo" {
            continue;
        }

        // If user specified an interface, only match that
        if let Some(target) = interface {
            if name != target {
                continue;
            }
        }

        if let Some(addrs) = iface["ip-addresses"].as_array() {
            for addr in addrs {
                if addr["ip-address-type"].as_str() == Some("ipv4") {
                    if let Some(a) = addr["ip-address"].as_str() {
                        ip = Some(a.to_string());
                        break;
                    }
                }
            }
        }
        if ip.is_some() {
            break;
        }
    }

    let ip = ip.context(
        "No IPv4 address found. Ensure the VM has a network interface with an IP and qemu-guest-agent is running.",
    )?;

    let destination = format!("{}@{}", user, ip);

    let mut cmd = std::process::Command::new("ssh");

    if let Some(jump) = proxy {
        eprintln!("Connecting to {} via proxy {}...", destination, jump);
        cmd.arg("-J").arg(jump);
    } else {
        eprintln!("Connecting to {}...", destination);
    }

    cmd.arg(&destination);

    let status = cmd
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("Failed to execute ssh")?;

    if !status.success() {
        bail!("SSH exited with code {}", status.code().unwrap_or(-1));
    }

    Ok(())
}
