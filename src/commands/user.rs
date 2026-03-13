use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::UserCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: UserCommand, json: bool, yes: bool) -> Result<()> {
    match cmd {
        UserCommand::List => {
            let data = api.get("/access/users").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("userid", "USER"),
                    ("firstname", "FIRST"),
                    ("lastname", "LAST"),
                    ("email", "EMAIL"),
                    ("enable", "ENABLED"),
                ],
            );
        }
        UserCommand::Show { userid } => {
            let data = api.get(&format!("/access/users/{}", userid)).await?;
            output::print_item(
                &data,
                json,
                &[
                    ("userid", "User ID"),
                    ("firstname", "First Name"),
                    ("lastname", "Last Name"),
                    ("email", "Email"),
                    ("enable", "Enabled"),
                    ("groups", "Groups"),
                    ("comment", "Comment"),
                ],
            );
        }
        UserCommand::Create {
            userid,
            password,
            firstname,
            lastname,
            email,
            groups,
            comment,
        } => {
            let mut params: Vec<(&str, String)> = vec![("userid", userid.clone())];
            if let Some(v) = password {
                params.push(("password", v));
            }
            if let Some(v) = firstname {
                params.push(("firstname", v));
            }
            if let Some(v) = lastname {
                params.push(("lastname", v));
            }
            if let Some(v) = email {
                params.push(("email", v));
            }
            if let Some(v) = groups {
                params.push(("groups", v));
            }
            if let Some(v) = comment {
                params.push(("comment", v));
            }

            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            api.post("/access/users", &refs).await?;
            eprintln!("User {} created.", userid);
        }
        UserCommand::Delete { userid } => {
            if !yes && !output::confirm(&format!("Delete user {}?", userid)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            api.delete(&format!("/access/users/{}", userid)).await?;
            eprintln!("User {} deleted.", userid);
        }
        UserCommand::SetPassword { userid, password } => {
            api.put(
                "/access/password",
                &[("userid", userid.as_str()), ("password", password.as_str())],
            )
            .await?;
            eprintln!("Password updated for {}.", userid);
        }
        UserCommand::Acl {
            userid,
            path,
            role,
            propagate,
        } => {
            let propagate_s = if propagate { "1" } else { "0" };
            api.put(
                "/access/acl",
                &[
                    ("users", userid.as_str()),
                    ("path", path.as_str()),
                    ("roles", role.as_str()),
                    ("propagate", propagate_s),
                ],
            )
            .await?;
            eprintln!("ACL set: {} -> {} on {}", userid, role, path);
        }
        UserCommand::Roles => {
            let data = api.get("/access/roles").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("roleid", "ROLE"),
                    ("privs", "PRIVILEGES"),
                    ("special", "SPECIAL"),
                ],
            );
        }
        UserCommand::Acls => {
            let data = api.get("/access/acl").await?;
            output::print_list(
                &data,
                json,
                &[
                    ("path", "PATH"),
                    ("ugid", "USER/GROUP"),
                    ("roleid", "ROLE"),
                    ("type", "TYPE"),
                    ("propagate", "PROPAGATE"),
                ],
            );
        }
    }
    Ok(())
}
