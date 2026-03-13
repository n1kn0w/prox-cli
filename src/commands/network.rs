use anyhow::{bail, Result};

use crate::api::ProxmoxClient;
use crate::cli::NetworkCommand;
use crate::output;

const PROTECTED_IFACES: &[&str] = &["vmbr0", "lo"];

pub async fn handle(api: &ProxmoxClient, cmd: NetworkCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        NetworkCommand::List => {
            let data = api
                .get(&format!("/nodes/{}/network", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("iface", "IFACE"),
                    ("type", "TYPE"),
                    ("cidr", "CIDR"),
                    ("bridge_ports", "PORTS"),
                    ("active", "ACTIVE"),
                    ("autostart", "AUTO"),
                    ("comments", "COMMENT"),
                ],
            );
        }
        NetworkCommand::Create {
            iface,
            iface_type,
            bridge_ports,
            vlan_id,
            vlan_raw_device,
            cidr,
            gateway,
            vlan_aware,
            autostart,
            comment,
        } => {
            if PROTECTED_IFACES.contains(&iface.as_str()) {
                bail!("Cannot modify protected interface: {}", iface);
            }

            let mut params: Vec<(&str, String)> = vec![
                ("iface", iface.clone()),
                ("type", iface_type.clone()),
            ];
            if let Some(v) = bridge_ports {
                params.push(("bridge_ports", v));
            }
            if let Some(v) = vlan_id {
                params.push(("vlan-id", v.to_string()));
            }
            if let Some(v) = vlan_raw_device {
                params.push(("vlan-raw-device", v));
            }
            if let Some(v) = cidr {
                params.push(("cidr", v));
            }
            if let Some(v) = gateway {
                params.push(("gateway", v));
            }
            if vlan_aware {
                params.push(("bridge_vlan_aware", "1".to_string()));
            }
            if autostart {
                params.push(("autostart", "1".to_string()));
            }
            if let Some(v) = comment {
                params.push(("comments", v));
            }

            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post(&format!("/nodes/{}/network", api.node()), &refs)
                .await?;
            eprintln!(
                "Interface {} created. Run 'prox-cli network apply' to activate.",
                iface
            );
        }
        NetworkCommand::Delete { iface } => {
            if PROTECTED_IFACES.contains(&iface.as_str()) {
                bail!("Cannot delete protected interface: {}", iface);
            }
            if !yes && !output::confirm(&format!("Delete interface {}?", iface)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/nodes/{}/network/{}", api.node(), iface))
                .await?;
            eprintln!(
                "Interface {} deleted. Run 'prox-cli network apply' to activate.",
                iface
            );
        }
        NetworkCommand::Apply => {
            api.put(&format!("/nodes/{}/network", api.node()), &[])
                .await?;
            eprintln!("Network changes applied.");
        }
        NetworkCommand::Revert => {
            api.delete(&format!("/nodes/{}/network", api.node()))
                .await?;
            eprintln!("Pending network changes reverted.");
        }
    }
    Ok(())
}
