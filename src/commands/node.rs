use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::NodeCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: NodeCommand, json: bool) -> Result<()> {
    match cmd {
        NodeCommand::Status => {
            let data = api
                .get(&format!("/nodes/{}/status", api.node()))
                .await?;
            output::print_raw(&data, json);
        }
        NodeCommand::Time => {
            let data = api
                .get(&format!("/nodes/{}/time", api.node()))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("localtime", "Local time"),
                    ("timezone", "Timezone"),
                    ("time", "UTC"),
                ],
            );
        }
        NodeCommand::Dns => {
            let data = api
                .get(&format!("/nodes/{}/dns", api.node()))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("dns1", "DNS 1"),
                    ("dns2", "DNS 2"),
                    ("dns3", "DNS 3"),
                    ("search", "Search domain"),
                ],
            );
        }
        NodeCommand::Version => {
            let data = api.get("/version").await?;
            output::print_item(
                &data,
                json,
                &[
                    ("version", "Version"),
                    ("release", "Release"),
                    ("repoid", "Repo ID"),
                ],
            );
        }
        NodeCommand::Services => {
            let data = api
                .get(&format!("/nodes/{}/services", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("state", "STATE"),
                    ("desc", "DESCRIPTION"),
                ],
            );
        }
        NodeCommand::Syslog {
            limit,
            start,
            service,
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
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/syslog", api.node()),
                    &params,
                )
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
    }
    Ok(())
}
