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
            let limit_str = limit.to_string();
            let vmid_str = vmid.map(|v| v.to_string());
            let mut params: Vec<(&str, &str)> = vec![
                ("limit", &limit_str),
                ("source", &source),
            ];
            if let Some(ref v) = vmid_str {
                params.push(("vmid", v));
            }
            if errors_only {
                params.push(("errors", "1"));
            }
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/tasks", api.node()),
                    &params,
                )
                .await?;
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
            let limit_str = limit.to_string();
            let start_str = start.map(|s| s.to_string());
            let mut params: Vec<(&str, &str)> = vec![("limit", &limit_str)];
            if let Some(ref s) = start_str {
                params.push(("start", s));
            }
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/tasks/{}/log", api.node(), upid),
                    &params,
                )
                .await?;
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
