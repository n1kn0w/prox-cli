use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::TemplateCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: TemplateCommand, json: bool, _yes: bool) -> Result<()> {
    match cmd {
        TemplateCommand::List => {
            let data = api.get(&format!("/nodes/{}/qemu", api.node())).await?;
            let templates: Vec<&serde_json::Value> = data
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter(|v| v["template"].as_i64() == Some(1))
                        .collect()
                })
                .unwrap_or_default();
            let filtered = serde_json::Value::Array(templates.into_iter().cloned().collect());
            output::print_list(
                &filtered,
                json,
                &[
                    ("vmid", "VMID"),
                    ("name", "NAME"),
                    ("maxmem", "MEMORY"),
                    ("maxdisk", "DISK"),
                ],
            );
        }
        TemplateCommand::Create { vmid } => {
            api.post(
                &format!("/nodes/{}/qemu/{}/template", api.node(), vmid),
                &[],
            )
            .await?;
            eprintln!("VM {} converted to template.", vmid);
        }
        TemplateCommand::Clone {
            vmid,
            newid,
            name,
            storage,
            full,
        } => {
            let newid_s = newid.to_string();
            let mut params: Vec<(&str, &str)> = vec![("newid", &newid_s)];
            if let Some(ref n) = name {
                params.push(("name", n));
            }
            if let Some(ref s) = storage {
                params.push(("storage", s));
            }
            if full {
                params.push(("full", "1"));
            }

            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/clone", api.node(), vmid),
                    &params,
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Template {} cloned to VM {}.", vmid, newid);
        }
    }
    Ok(())
}
