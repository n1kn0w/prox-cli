use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::TfaCommand;
use crate::output;

pub async fn handle(
    api: &ProxmoxClient,
    cmd: TfaCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    match cmd {
        TfaCommand::List => {
            let data = api.get("/access/tfa").await?;
            if json {
                output::print_raw(&data, json);
            } else {
                // TFA list returns object keyed by userid
                if let Some(obj) = data.as_object() {
                    for (userid, entries) in obj {
                        if let Some(arr) = entries.as_array() {
                            for entry in arr {
                                println!(
                                    "{}\t{}\t{}\t{}",
                                    userid,
                                    entry["type"].as_str().unwrap_or("-"),
                                    entry["id"].as_str().unwrap_or("-"),
                                    entry["description"].as_str().unwrap_or("-"),
                                );
                            }
                        }
                    }
                } else if let Some(arr) = data.as_array() {
                    output::print_list(
                        &data,
                        json,
                        &[
                            ("userid", "USER"),
                            ("type", "TYPE"),
                            ("id", "ID"),
                            ("description", "DESCRIPTION"),
                            ("enable", "ENABLED"),
                        ],
                    );
                    let _ = arr;
                } else {
                    output::print_raw(&data, json);
                }
            }
        }
        TfaCommand::UserList { userid } => {
            let data = api
                .get(&format!("/access/tfa/{}", userid))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("type", "TYPE"),
                    ("id", "ID"),
                    ("description", "DESCRIPTION"),
                    ("enable", "ENABLED"),
                    ("created", "CREATED"),
                ],
            );
        }
        TfaCommand::Add {
            userid,
            tfa_type,
            description,
            totp,
            value,
            password,
        } => {
            let mut params: Vec<(&str, String)> = vec![("type", tfa_type.clone())];
            if let Some(v) = description {
                params.push(("description", v));
            }
            if let Some(v) = totp {
                params.push(("totp", v));
            }
            if let Some(v) = value {
                params.push(("value", v));
            }
            if let Some(v) = password {
                params.push(("password", v));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = api
                .post(&format!("/access/tfa/{}", userid), &refs)
                .await?;
            output::print_raw(&data, json);
            eprintln!("TFA entry added for {}.", userid);
        }
        TfaCommand::Show { userid, id } => {
            let data = api
                .get(&format!("/access/tfa/{}/{}", userid, id))
                .await?;
            output::print_raw(&data, json);
        }
        TfaCommand::Update {
            userid,
            id,
            description,
            enable,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = description {
                params.push(("description", v));
            }
            if let Some(v) = enable {
                params.push(("enable", if v { "1".to_string() } else { "0".to_string() }));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(&format!("/access/tfa/{}/{}", userid, id), &refs)
                .await?;
            eprintln!("TFA entry {} updated for {}.", id, userid);
        }
        TfaCommand::Delete { userid, id } => {
            if !yes
                && !output::confirm(&format!(
                    "Delete TFA entry {} for {}?",
                    id, userid
                ))
            {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/access/tfa/{}/{}", userid, id))
                .await?;
            eprintln!("TFA entry {} deleted for {}.", id, userid);
        }
    }
    Ok(())
}
