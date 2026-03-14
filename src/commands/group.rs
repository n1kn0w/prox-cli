use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::GroupCommand;
use crate::output;

pub async fn handle(
    api: &ProxmoxClient,
    cmd: GroupCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    match cmd {
        GroupCommand::List => {
            let data = api.get("/access/groups").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("groupid", "GROUP"),
                    ("comment", "COMMENT"),
                    ("users", "MEMBERS"),
                ],
            );
        }
        GroupCommand::Show { groupid } => {
            let data = api
                .get(&format!("/access/groups/{}", groupid))
                .await?;
            output::print_item(
                &data,
                json,
                &[
                    ("groupid", "Group ID"),
                    ("comment", "Comment"),
                    ("members", "Members"),
                ],
            );
        }
        GroupCommand::Create { groupid, comment } => {
            let mut params: Vec<(&str, String)> = vec![("groupid", groupid.clone())];
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/access/groups", &refs).await?;
            eprintln!("Group '{}' created.", groupid);
        }
        GroupCommand::Update { groupid, comment } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(&format!("/access/groups/{}", groupid), &refs).await?;
            eprintln!("Group '{}' updated.", groupid);
        }
        GroupCommand::Delete { groupid } => {
            if !yes && !output::confirm(&format!("Delete group '{}'?", groupid)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/access/groups/{}", groupid)).await?;
            eprintln!("Group '{}' deleted.", groupid);
        }
    }
    Ok(())
}
