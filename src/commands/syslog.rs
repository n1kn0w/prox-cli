use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::SyslogCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: SyslogCommand, json: bool, yes: bool) -> Result<()> {
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
    }
    Ok(())
}
