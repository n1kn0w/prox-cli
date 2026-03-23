use anyhow::{bail, Result};
use std::process::Command;

use crate::api::ProxmoxClient;
use crate::cli::SyslogCommand;
use crate::output;

const REMOTE_CONF: &str = "/etc/rsyslog.d/remote.conf";

fn ssh_exec(host: &str, ssh_user: &str, proxy: Option<&str>, remote_cmd: &str) -> Result<String> {
    let target = format!("{}@{}", ssh_user, host);
    let mut cmd = Command::new("ssh");
    cmd.arg("-o").arg("StrictHostKeyChecking=accept-new");
    if let Some(p) = proxy {
        cmd.arg("-J").arg(p);
    }
    cmd.arg(&target).arg(remote_cmd);
    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("SSH command failed: {}", stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn handle(
    api: &ProxmoxClient,
    cmd: SyslogCommand,
    json: bool,
    yes: bool,
    node_host: &str,
    ssh_proxy: Option<&str>,
) -> Result<()> {
    match cmd {
        SyslogCommand::List {
            limit,
            start,
            service,
            since,
            until,
        } => {
            let limit_str = limit.to_string();
            let start_str = start.map(|s| s.to_string());
            let mut params: Vec<(&str, &str)> = vec![("limit", &limit_str)];
            if let Some(ref s) = start_str {
                params.push(("start", s));
            }
            if let Some(ref svc) = service {
                params.push(("service", svc));
            }
            if let Some(ref s) = since {
                params.push(("since", s));
            }
            if let Some(ref u) = until {
                params.push(("until", u));
            }
            let data = api
                .get_with_query(&format!("/nodes/{}/syslog", api.node()), &params)
                .await?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            } else if let Some(lines) = data.as_array() {
                for line in lines {
                    if let Some(t) = line["t"].as_str() {
                        println!("{}", t);
                    }
                }
            }
        }
        SyslogCommand::Journal {
            limit,
            start,
            since,
            until,
            lastentries,
        } => {
            let limit_str = limit.to_string();
            let start_str = start.map(|s| s.to_string());
            let lastentries_str = lastentries.map(|l| l.to_string());
            let mut params: Vec<(&str, &str)> = vec![("limit", &limit_str)];
            if let Some(ref s) = start_str {
                params.push(("start", s));
            }
            if let Some(ref s) = since {
                params.push(("since", s));
            }
            if let Some(ref u) = until {
                params.push(("until", u));
            }
            if let Some(ref l) = lastentries_str {
                params.push(("lastentries", l));
            }
            let data = api
                .get_with_query(&format!("/nodes/{}/journal", api.node()), &params)
                .await?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            } else if let Some(lines) = data.as_array() {
                for line in lines {
                    if let Some(t) = line["t"].as_str() {
                        println!("{}", t);
                    }
                }
            }
        }
        SyslogCommand::ServiceStatus => {
            let data = api
                .get(&format!("/nodes/{}/services/rsyslog", api.node()))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("name", "Service"),
                    ("state", "State"),
                    ("desc", "Description"),
                ],
            );
        }
        SyslogCommand::ServiceStart => {
            if !yes {
                if !output::confirm("Start rsyslog service?") {
                    return Ok(());
                }
            }
            let data = api
                .post(
                    &format!("/nodes/{}/services/rsyslog/start", api.node()),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
                println!("rsyslog service started.");
            }
        }
        SyslogCommand::ServiceStop => {
            if !yes {
                if !output::confirm("Stop rsyslog service?") {
                    return Ok(());
                }
            }
            let data = api
                .post(
                    &format!("/nodes/{}/services/rsyslog/stop", api.node()),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
                println!("rsyslog service stopped.");
            }
        }
        SyslogCommand::ServiceRestart => {
            if !yes {
                if !output::confirm("Restart rsyslog service?") {
                    return Ok(());
                }
            }
            let data = api
                .post(
                    &format!("/nodes/{}/services/rsyslog/restart", api.node()),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
                println!("rsyslog service restarted.");
            }
        }
        SyslogCommand::ServiceReload => {
            if !yes {
                if !output::confirm("Reload rsyslog service configuration?") {
                    return Ok(());
                }
            }
            let data = api
                .post(
                    &format!("/nodes/{}/services/rsyslog/reload", api.node()),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
                println!("rsyslog service reloaded.");
            }
        }
        SyslogCommand::ConfigShow { ssh_user, proxy } => {
            let proxy_ref = proxy.as_deref().or(ssh_proxy);
            let result = ssh_exec(
                node_host,
                &ssh_user,
                proxy_ref,
                &format!("cat {} 2>/dev/null || echo 'No remote syslog forwarding configured.'", REMOTE_CONF),
            )?;
            print!("{}", result);
        }
        SyslogCommand::ConfigSet {
            server,
            port,
            protocol,
            facility,
            ssh_user,
            proxy,
        } => {
            let proto = match protocol.to_lowercase().as_str() {
                "tcp" => "@@",
                "udp" => "@",
                other => bail!("Invalid protocol '{}': use 'tcp' or 'udp'", other),
            };
            if !yes {
                if !output::confirm(&format!(
                    "Set syslog forwarding to {}:{} ({})? This will overwrite {} and restart rsyslog.",
                    server, port, protocol, REMOTE_CONF
                )) {
                    return Ok(());
                }
            }
            let conf_content = format!(
                "# Managed by prox-cli\\n{} {}{}:{}",
                facility, proto, server, port
            );
            let proxy_ref = proxy.as_deref().or(ssh_proxy);
            let remote_cmd = format!(
                "echo -e '{}' > {} && systemctl restart rsyslog",
                conf_content, REMOTE_CONF
            );
            ssh_exec(node_host, &ssh_user, proxy_ref, &remote_cmd)?;
            println!(
                "Syslog forwarding set: {} {}{}:{} — rsyslog restarted.",
                facility, proto, server, port
            );
        }
        SyslogCommand::ConfigDelete { ssh_user, proxy } => {
            if !yes {
                if !output::confirm(&format!(
                    "Delete {} and restart rsyslog? This will remove remote syslog forwarding.",
                    REMOTE_CONF
                )) {
                    return Ok(());
                }
            }
            let proxy_ref = proxy.as_deref().or(ssh_proxy);
            let remote_cmd = format!(
                "rm -f {} && systemctl restart rsyslog",
                REMOTE_CONF
            );
            ssh_exec(node_host, &ssh_user, proxy_ref, &remote_cmd)?;
            println!("Remote syslog forwarding removed — rsyslog restarted.");
        }
    }
    Ok(())
}
