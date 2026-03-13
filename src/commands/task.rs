use anyhow::{bail, Result};

use crate::api::ProxmoxClient;
use crate::cli::TaskCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: TaskCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        TaskCommand::List {
            vmid,
            source,
            limit,
            errors_only,
        } => {
            let mut path = format!(
                "/nodes/{}/tasks?limit={}&source={}",
                api.node(),
                limit,
                source
            );
            if let Some(vmid) = vmid {
                path.push_str(&format!("&vmid={}", vmid));
            }
            if errors_only {
                path.push_str("&errors=1");
            }
            let data = api.get(&path).await?;
            output::print_list(
                &data,
                json,
                &[
                    ("upid", "UPID"),
                    ("type", "TYPE"),
                    ("status", "STATUS"),
                    ("starttime", "START"),
                    ("endtime", "END"),
                    ("user", "USER"),
                ],
            );
        }
        TaskCommand::Status { upid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/tasks/{}/status",
                    api.node(),
                    upid
                ))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("upid", "UPID"),
                    ("type", "Type"),
                    ("status", "Status"),
                    ("exitstatus", "Exit Status"),
                    ("pid", "PID"),
                    ("starttime", "Start Time"),
                    ("node", "Node"),
                    ("user", "User"),
                ],
            );
        }
        TaskCommand::Log { upid, limit, start } => {
            let mut path = format!(
                "/nodes/{}/tasks/{}/log?limit={}",
                api.node(),
                upid,
                limit
            );
            if let Some(start) = start {
                path.push_str(&format!("&start={}", start));
            }
            let data = api.get(&path).await?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            } else {
                if let Some(lines) = data.as_array() {
                    for line in lines {
                        let text = line["t"].as_str().unwrap_or("");
                        println!("{}", text);
                    }
                } else {
                    println!("No log data.");
                }
            }
        }
        TaskCommand::Cancel { upid } => {
            if !yes && !output::confirm(&format!("Cancel task {}?", upid)) {
                bail!("Aborted.");
            }
            api.delete(&format!("/nodes/{}/tasks/{}", api.node(), upid))
                .await?;
            println!("Task {} cancelled.", upid);
        }
    }
    Ok(())
}
