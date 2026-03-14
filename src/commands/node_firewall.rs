use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::NodeFirewallCommand;
use crate::output;

/// Collect optional rule parameters into a Vec for POST/PUT requests.
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

/// Collect optional update parameters (all fields optional).
fn build_update_params(
    action: Option<String>,
    rule_type: Option<String>,
    iface: Option<String>,
    source: Option<String>,
    dest: Option<String>,
    dport: Option<String>,
    sport: Option<String>,
    proto: Option<String>,
    comment: Option<String>,
    enable: Option<bool>,
) -> Vec<(&'static str, String)> {
    let mut params: Vec<(&str, String)> = Vec::new();
    if let Some(v) = action {
        params.push(("action", v));
    }
    if let Some(v) = rule_type {
        params.push(("type", v));
    }
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
    cmd: NodeFirewallCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    let base = format!("/nodes/{}/firewall", api.node());

    match cmd {
        // ── List rules ────────────────────────────────────────────────
        NodeFirewallCommand::List => {
            let data = api.get(&format!("{}/rules", base)).await?;
            output::print_list(&data, json, RULE_COLUMNS);
        }

        // ── Add rule ──────────────────────────────────────────────────
        NodeFirewallCommand::Add {
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
            api.post(&format!("{}/rules", base), &refs).await?;
            eprintln!("Node firewall rule added.");
        }

        // ── Show rule at position ─────────────────────────────────────
        NodeFirewallCommand::Show { pos } => {
            let data = api.get(&format!("{}/rules/{}", base, pos)).await?;
            output::print_raw(&data, json);
        }

        // ── Update rule at position ───────────────────────────────────
        NodeFirewallCommand::Update {
            pos,
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
        } => {
            let params = build_update_params(
                action, rule_type, iface, source, dest, dport, sport, proto, comment, enable,
            );
            if params.is_empty() {
                eprintln!("No parameters specified, nothing to update.");
                return Ok(());
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(&format!("{}/rules/{}", base, pos), &refs).await?;
            eprintln!("Node firewall rule at position {} updated.", pos);
        }

        // ── Delete rule at position ───────────────────────────────────
        NodeFirewallCommand::Delete { pos } => {
            if !yes
                && !output::confirm(&format!(
                    "Delete node firewall rule at position {}?",
                    pos
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("{}/rules/{}", base, pos)).await?;
            eprintln!("Node firewall rule at position {} deleted.", pos);
        }

        // ── Show firewall options ─────────────────────────────────────
        NodeFirewallCommand::Options => {
            let data = api.get(&format!("{}/options", base)).await?;
            output::print_raw(&data, json);
        }

        // ── Set firewall options ──────────────────────────────────────
        NodeFirewallCommand::SetOptions {
            enable,
            policy_in,
            policy_out,
            log_level_in,
            log_level_out,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = enable {
                params.push(("enable", if v { "1".to_string() } else { "0".to_string() }));
            }
            if let Some(v) = policy_in {
                params.push(("policy_in", v));
            }
            if let Some(v) = policy_out {
                params.push(("policy_out", v));
            }
            if let Some(v) = log_level_in {
                params.push(("log_level_in", v));
            }
            if let Some(v) = log_level_out {
                params.push(("log_level_out", v));
            }
            if params.is_empty() {
                eprintln!("No options specified, nothing to update.");
                return Ok(());
            }
            let refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(&format!("{}/options", base), &refs).await?;
            eprintln!("Node firewall options updated.");
        }

        // ── Firewall log ──────────────────────────────────────────────
        NodeFirewallCommand::Log { limit, start } => {
            let mut query = format!("{}/log", base);
            let mut sep = '?';
            if let Some(v) = limit {
                query.push_str(&format!("{}limit={}", sep, v));
                sep = '&';
            }
            if let Some(v) = start {
                query.push_str(&format!("{}start={}", sep, v));
            }
            let data = api.get(&query).await?;
            output::print_list(
                &data,
                json,
                &[
                    ("n", "N"),
                    ("t", "LINE"),
                ],
            );
        }

        // ── List refs (IPSet/alias references) ────────────────────────
        NodeFirewallCommand::Refs => {
            let data = api.get(&format!("{}/refs", base)).await?;
            output::print_list(
                &data,
                json,
                &[
                    ("type", "TYPE"),
                    ("name", "NAME"),
                    ("comment", "COMMENT"),
                ],
            );
        }
    }
    Ok(())
}
