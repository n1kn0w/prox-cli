use anyhow::{bail, Result};

use crate::api::ProxmoxClient;
use crate::cli::PoolCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: PoolCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        PoolCommand::List => {
            let data = api.get("/pools").await?;
            output::print_list(
                &data,
                json,
                &[("poolid", "POOL"), ("comment", "COMMENT")],
            );
        }
        PoolCommand::Show { poolid } => {
            let data = api.get(&format!("/pools/{}", poolid)).await?;
            output::print_raw(&data, json);
        }
        PoolCommand::Create { poolid, comment } => {
            let mut params: Vec<(&str, &str)> = vec![("poolid", &poolid)];
            if let Some(ref c) = comment {
                params.push(("comment", c));
            }
            api.post("/pools", &params).await?;
            println!("Pool '{}' created.", poolid);
        }
        PoolCommand::Delete { poolid } => {
            if !yes && !output::confirm(&format!("Delete pool '{}'?", poolid)) {
                bail!("Aborted.");
            }
            api.delete(&format!("/pools/{}", poolid)).await?;
            println!("Pool '{}' deleted.", poolid);
        }
        PoolCommand::Add {
            poolid,
            vmid,
            storage,
        } => {
            let vms_str = vmid
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let storage_str = storage.join(",");
            let mut params: Vec<(&str, &str)> = Vec::new();
            if !vms_str.is_empty() {
                params.push(("vms", &vms_str));
            }
            if !storage_str.is_empty() {
                params.push(("storage", &storage_str));
            }
            if params.is_empty() {
                bail!("Specify at least one --vmid or --storage to add.");
            }
            api.put(&format!("/pools/{}", poolid), &params).await?;
            println!("Resources added to pool '{}'.", poolid);
        }
        PoolCommand::Remove {
            poolid,
            vmid,
            storage,
        } => {
            let vms_str = vmid
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            let storage_str = storage.join(",");
            let mut params: Vec<(&str, &str)> = vec![("delete", "1")];
            if !vms_str.is_empty() {
                params.push(("vms", &vms_str));
            }
            if !storage_str.is_empty() {
                params.push(("storage", &storage_str));
            }
            if params.len() == 1 {
                bail!("Specify at least one --vmid or --storage to remove.");
            }
            api.put(&format!("/pools/{}", poolid), &params).await?;
            println!("Resources removed from pool '{}'.", poolid);
        }
    }
    Ok(())
}
