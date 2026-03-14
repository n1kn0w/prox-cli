use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::AptCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: AptCommand, json: bool) -> Result<()> {
    match cmd {
        AptCommand::Repos => {
            let data = api
                .get(&format!("/nodes/{}/apt/repositories", api.node()))
                .await?;
            output::print_raw(&data, json);
        }
        AptCommand::Update => {
            let data = api
                .post(&format!("/nodes/{}/apt/update", api.node()), &[])
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Package index updated.");
        }
        AptCommand::Upgrade => {
            let data = api
                .get(&format!("/nodes/{}/apt/update", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("Package", "PACKAGE"),
                    ("Title", "TITLE"),
                    ("OldVersion", "OLD VERSION"),
                    ("NewVersion", "NEW VERSION"),
                    ("Priority", "PRIORITY"),
                ],
            );
        }
        AptCommand::Versions => {
            let data = api
                .get(&format!("/nodes/{}/apt/versions", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("Package", "PACKAGE"),
                    ("Title", "TITLE"),
                    ("OldVersion", "OLD VERSION"),
                    ("CurrentState", "STATE"),
                    ("RunningKernel", "RUNNING KERNEL"),
                ],
            );
        }
        AptCommand::Changelog { name } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/apt/changelog?name={}",
                    api.node(),
                    name
                ))
                .await?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            } else if let Some(text) = data.as_str() {
                println!("{}", text);
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&data).unwrap_or_default()
                );
            }
        }
    }
    Ok(())
}
