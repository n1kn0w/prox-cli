use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::FirewallCommand;
use crate::output;

/// Collect optional rule parameters into a Vec for POST requests.
fn build_rule_params(
    action: String,
    rule_type: String,
    iface: Option<String>,
    source: Option<String>,
    dest: Option<String>,
    dport: Option<String>,
    sport: Option<String>,
    proto: Option<String>,
    comment: Option<String>,
    enable: Option<bool>,
    pos: Option<u32>,
) -> Vec<(&'static str, String)> {
    let mut params: Vec<(&str, String)> = vec![
        ("action", action),
        ("type", rule_type),
    ];
    if let Some(v) = iface {
        params.push(("iface", v));
    }
    if let Some(v) = source {
        params.push(("source", v));
    }
    if let Some(v) = dest {
        params.push(("dest", v));
    }
    if let Some(v) = dport {
        params.push(("dport", v));
    }
    if let Some(v) = sport {
        params.push(("sport", v));
    }
    if let Some(v) = proto {
        params.push(("proto", v));
    }
    if let Some(v) = comment {
        params.push(("comment", v));
    }
    if let Some(v) = enable {
        params.push(("enable", if v { "1".to_string() } else { "0".to_string() }));
    }
    if let Some(v) = pos {
        params.push(("pos", v.to_string()));
    }
    params
}

const RULE_COLUMNS: &[(&str, &str)] = &[
    ("pos", "POS"),
    ("type", "TYPE"),
    ("action", "ACTION"),
    ("proto", "PROTO"),
    ("source", "SOURCE"),
    ("dest", "DEST"),
    ("dport", "DPORT"),
    ("sport", "SPORT"),
    ("iface", "IFACE"),
    ("enable", "ENABLE"),
    ("comment", "COMMENT"),
];

pub async fn handle(
    api: &ProxmoxClient,
    cmd: FirewallCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    match cmd {
        // ── Cluster-level rules ─────────────────────────────────────
        FirewallCommand::ClusterRules => {
            let data = api.get("/cluster/firewall/rules").await?;
            output::print_list(&data, json, RULE_COLUMNS);
        }
        FirewallCommand::ClusterAdd {
            action,
            rule_type,
            iface,
            source,
            dest,
            dport,
            sport,
            proto,
            comment,
            enable,
            pos,
        } => {
            let params = build_rule_params(
                action, rule_type, iface, source, dest, dport, sport, proto, comment, enable, pos,
            );
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/cluster/firewall/rules", &refs).await?;
            eprintln!("Cluster firewall rule added.");
        }
        FirewallCommand::ClusterDelete { pos } => {
            if !yes && !output::confirm(&format!("Delete cluster firewall rule at position {}?", pos)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/cluster/firewall/rules/{}", pos)).await?;
            eprintln!("Cluster firewall rule at position {} deleted.", pos);
        }
        FirewallCommand::ClusterOptions => {
            let data = api.get("/cluster/firewall/options").await?;
            output::print_raw(&data, json);
        }
        FirewallCommand::ClusterEnable { enable } => {
            let val = enable.to_string();
            api.put("/cluster/firewall/options", &[("enable", val.as_str())])
                .await?;
            let state = if enable == 1 { "enabled" } else { "disabled" };
            eprintln!("Cluster firewall {}.", state);
        }

        // ── VM-level rules ──────────────────────────────────────────
        FirewallCommand::VmRules { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/firewall/rules",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_list(&data, json, RULE_COLUMNS);
        }
        FirewallCommand::VmAdd {
            vmid,
            action,
            rule_type,
            iface,
            source,
            dest,
            dport,
            sport,
            proto,
            comment,
            enable,
            pos,
        } => {
            let params = build_rule_params(
                action, rule_type, iface, source, dest, dport, sport, proto, comment, enable, pos,
            );
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post(
                &format!("/nodes/{}/qemu/{}/firewall/rules", api.node(), vmid),
                &refs,
            )
            .await?;
            eprintln!("Firewall rule added to VM {}.", vmid);
        }
        FirewallCommand::VmDelete { vmid, pos } => {
            if !yes
                && !output::confirm(&format!(
                    "Delete firewall rule at position {} on VM {}?",
                    pos, vmid
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!(
                "/nodes/{}/qemu/{}/firewall/rules/{}",
                api.node(),
                vmid,
                pos
            ))
            .await?;
            eprintln!("Firewall rule at position {} deleted from VM {}.", pos, vmid);
        }
        FirewallCommand::VmOptions { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/firewall/options",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_raw(&data, json);
        }
        FirewallCommand::VmEnable { vmid, enable } => {
            let val = enable.to_string();
            api.put(
                &format!("/nodes/{}/qemu/{}/firewall/options", api.node(), vmid),
                &[("enable", val.as_str())],
            )
            .await?;
            let state = if enable == 1 { "enabled" } else { "disabled" };
            eprintln!("Firewall {} on VM {}.", state, vmid);
        }

        // ── CT-level rules ──────────────────────────────────────────
        FirewallCommand::CtRules { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/lxc/{}/firewall/rules",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_list(&data, json, RULE_COLUMNS);
        }
        FirewallCommand::CtAdd {
            vmid,
            action,
            rule_type,
            iface,
            source,
            dest,
            dport,
            sport,
            proto,
            comment,
            enable,
            pos,
        } => {
            let params = build_rule_params(
                action, rule_type, iface, source, dest, dport, sport, proto, comment, enable, pos,
            );
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post(
                &format!("/nodes/{}/lxc/{}/firewall/rules", api.node(), vmid),
                &refs,
            )
            .await?;
            eprintln!("Firewall rule added to CT {}.", vmid);
        }
        FirewallCommand::CtDelete { vmid, pos } => {
            if !yes
                && !output::confirm(&format!(
                    "Delete firewall rule at position {} on CT {}?",
                    pos, vmid
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!(
                "/nodes/{}/lxc/{}/firewall/rules/{}",
                api.node(),
                vmid,
                pos
            ))
            .await?;
            eprintln!("Firewall rule at position {} deleted from CT {}.", pos, vmid);
        }
        FirewallCommand::CtOptions { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/lxc/{}/firewall/options",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_raw(&data, json);
        }
        FirewallCommand::CtEnable { vmid, enable } => {
            let val = enable.to_string();
            api.put(
                &format!("/nodes/{}/lxc/{}/firewall/options", api.node(), vmid),
                &[("enable", val.as_str())],
            )
            .await?;
            let state = if enable == 1 { "enabled" } else { "disabled" };
            eprintln!("Firewall {} on CT {}.", state, vmid);
        }

        // ── IP Sets ─────────────────────────────────────────────────
        FirewallCommand::IpsetList => {
            let data = api.get("/cluster/firewall/ipset").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("comment", "COMMENT"),
                    ("digest", "DIGEST"),
                ],
            );
        }
        FirewallCommand::IpsetCreate { name, comment } => {
            let mut params: Vec<(&str, String)> = vec![("name", name.clone())];
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/cluster/firewall/ipset", &refs).await?;
            eprintln!("IP set '{}' created.", name);
        }
        FirewallCommand::IpsetDelete { name } => {
            if !yes && !output::confirm(&format!("Delete IP set '{}'?", name)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/cluster/firewall/ipset/{}", name))
                .await?;
            eprintln!("IP set '{}' deleted.", name);
        }
        FirewallCommand::IpsetEntries { name } => {
            let data = api
                .get(&format!("/cluster/firewall/ipset/{}", name))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("cidr", "CIDR"),
                    ("nomatch", "NOMATCH"),
                    ("comment", "COMMENT"),
                ],
            );
        }
        FirewallCommand::IpsetAdd {
            name,
            cidr,
            comment,
        } => {
            let mut params: Vec<(&str, String)> = vec![("cidr", cidr.clone())];
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post(&format!("/cluster/firewall/ipset/{}", name), &refs)
                .await?;
            eprintln!("Entry '{}' added to IP set '{}'.", cidr, name);
        }
        FirewallCommand::IpsetRemove { name, cidr } => {
            if !yes
                && !output::confirm(&format!(
                    "Remove '{}' from IP set '{}'?",
                    cidr, name
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/cluster/firewall/ipset/{}/{}", name, cidr))
                .await?;
            eprintln!("Entry '{}' removed from IP set '{}'.", cidr, name);
        }

        // ── Aliases ─────────────────────────────────────────────────
        FirewallCommand::AliasList => {
            let data = api.get("/cluster/firewall/aliases").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("cidr", "CIDR"),
                    ("comment", "COMMENT"),
                ],
            );
        }
        FirewallCommand::AliasCreate {
            name,
            cidr,
            comment,
        } => {
            let mut params: Vec<(&str, String)> = vec![
                ("name", name.clone()),
                ("cidr", cidr),
            ];
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/cluster/firewall/aliases", &refs).await?;
            eprintln!("Alias '{}' created.", name);
        }
        FirewallCommand::AliasDelete { name } => {
            if !yes && !output::confirm(&format!("Delete alias '{}'?", name)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/cluster/firewall/aliases/{}", name))
                .await?;
            eprintln!("Alias '{}' deleted.", name);
        }
    }
    Ok(())
}
