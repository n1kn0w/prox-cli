use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::DomainCommand;
use crate::output;

pub async fn handle(
    api: &ProxmoxClient,
    cmd: DomainCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    match cmd {
        DomainCommand::List => {
            let data = api.get("/access/domains").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("realm", "REALM"),
                    ("type", "TYPE"),
                    ("comment", "COMMENT"),
                    ("tfa", "TFA"),
                    ("default", "DEFAULT"),
                ],
            );
        }
        DomainCommand::Show { realm } => {
            let data = api
                .get(&format!("/access/domains/{}", realm))
                .await?;
            output::print_raw(&data, json);
        }
        DomainCommand::Create {
            realm,
            realm_type,
            comment,
            server1,
            server2,
            port,
            base_dn,
            user_attr,
            bind_dn,
            default,
            tfa,
        } => {
            let mut params: Vec<(&str, String)> = vec![
                ("realm", realm.clone()),
                ("type", realm_type),
            ];
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            if let Some(v) = server1 {
                params.push(("server1", v));
            }
            if let Some(v) = server2 {
                params.push(("server2", v));
            }
            if let Some(v) = port {
                params.push(("port", v.to_string()));
            }
            if let Some(v) = base_dn {
                params.push(("base_dn", v));
            }
            if let Some(v) = user_attr {
                params.push(("user_attr", v));
            }
            if let Some(v) = bind_dn {
                params.push(("bind_dn", v));
            }
            if default {
                params.push(("default", "1".to_string()));
            }
            if let Some(v) = tfa {
                params.push(("tfa", v));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/access/domains", &refs).await?;
            eprintln!("Realm '{}' created.", realm);
        }
        DomainCommand::Update {
            realm,
            comment,
            server1,
            server2,
            port,
            base_dn,
            user_attr,
            bind_dn,
            default,
            tfa,
        } => {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = comment {
                params.push(("comment", v));
            }
            if let Some(v) = server1 {
                params.push(("server1", v));
            }
            if let Some(v) = server2 {
                params.push(("server2", v));
            }
            if let Some(v) = port {
                params.push(("port", v.to_string()));
            }
            if let Some(v) = base_dn {
                params.push(("base_dn", v));
            }
            if let Some(v) = user_attr {
                params.push(("user_attr", v));
            }
            if let Some(v) = bind_dn {
                params.push(("bind_dn", v));
            }
            if let Some(v) = default {
                params.push(("default", if v { "1".to_string() } else { "0".to_string() }));
            }
            if let Some(v) = tfa {
                params.push(("tfa", v));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.put(&format!("/access/domains/{}", realm), &refs)
                .await?;
            eprintln!("Realm '{}' updated.", realm);
        }
        DomainCommand::Delete { realm } => {
            if !yes && !output::confirm(&format!("Delete realm '{}'?", realm)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/access/domains/{}", realm)).await?;
            eprintln!("Realm '{}' deleted.", realm);
        }
        DomainCommand::Sync { realm } => {
            let data = api
                .post(
                    &format!("/access/domains/{}/sync", realm),
                    &[],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Realm '{}' synced.", realm);
        }
    }
    Ok(())
}
