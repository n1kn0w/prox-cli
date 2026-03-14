use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::AgentCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: AgentCommand, json: bool) -> Result<()> {
    match cmd {
        AgentCommand::Ping { vmid } => {
            api.post(
                &format!("/nodes/{}/qemu/{}/agent/ping", api.node(), vmid),
                &[],
            )
            .await?;
            eprintln!("Guest agent is responding on VM {}.", vmid);
        }
        AgentCommand::Info { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/agent/get-osinfo",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_raw(&data, json);
        }
        AgentCommand::Network { vmid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/agent/network-get-interfaces",
                    api.node(),
                    vmid
                ))
                .await?;
            output::print_raw(&data, json);
        }
        AgentCommand::Exec {
            vmid,
            command,
            input_data,
        } => {
            let mut params: Vec<(&str, &str)> = vec![("command", command.as_str())];
            if let Some(ref input) = input_data {
                params.push(("input-data", input.as_str()));
            }
            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/agent/exec", api.node(), vmid),
                    &params,
                )
                .await?;
            if json {
                output::print_raw(&data, json);
            } else {
                let pid = data["pid"].as_u64().unwrap_or(0);
                eprintln!(
                    "Command started (PID: {}). Use 'prox-cli agent exec-status {} --pid {}' to check results.",
                    pid, vmid, pid
                );
            }
        }
        AgentCommand::ExecStatus { vmid, pid } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/agent/exec-status?pid={}",
                    api.node(),
                    vmid,
                    pid
                ))
                .await?;
            if json {
                output::print_raw(&data, true);
            } else {
                let exited = data["exited"].as_u64().unwrap_or(0);
                if exited == 1 {
                    let exitcode = data["exitcode"].as_i64().unwrap_or(-1);
                    eprintln!("Status: exited (code {})", exitcode);
                    if let Some(out) = data["out-data"].as_str() {
                        if !out.is_empty() {
                            println!("{}", out);
                        }
                    }
                    if let Some(err) = data["err-data"].as_str() {
                        if !err.is_empty() {
                            eprintln!("STDERR:\n{}", err);
                        }
                    }
                } else {
                    eprintln!("Status: still running");
                }
            }
        }
        AgentCommand::FileRead { vmid, file } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/qemu/{}/agent/file-read?file={}",
                    api.node(),
                    vmid,
                    file
                ))
                .await?;
            if json {
                output::print_raw(&data, true);
            } else if let Some(content) = data["content"].as_str() {
                println!("{}", content);
            }
        }
        AgentCommand::FileWrite {
            vmid,
            file,
            content,
        } => {
            api.post(
                &format!(
                    "/nodes/{}/qemu/{}/agent/file-write",
                    api.node(),
                    vmid
                ),
                &[("file", file.as_str()), ("content", content.as_str())],
            )
            .await?;
            eprintln!("File written: {}", file);
        }
        AgentCommand::SetPassword {
            vmid,
            username,
            password,
        } => {
            api.post(
                &format!(
                    "/nodes/{}/qemu/{}/agent/set-user-password",
                    api.node(),
                    vmid
                ),
                &[
                    ("username", username.as_str()),
                    ("password", password.as_str()),
                ],
            )
            .await?;
            eprintln!("Password set for {} on VM {}.", username, vmid);
        }
        AgentCommand::Shutdown { vmid } => {
            api.post(
                &format!(
                    "/nodes/{}/qemu/{}/agent/shutdown",
                    api.node(),
                    vmid
                ),
                &[],
            )
            .await?;
            eprintln!("Shutdown signal sent to VM {} via guest agent.", vmid);
        }
        AgentCommand::Fsfreeze { vmid } => {
            api.post(
                &format!(
                    "/nodes/{}/qemu/{}/agent/fsfreeze-freeze",
                    api.node(),
                    vmid
                ),
                &[],
            )
            .await?;
            eprintln!("Filesystems frozen on VM {}.", vmid);
        }
        AgentCommand::Fsthaw { vmid } => {
            api.post(
                &format!(
                    "/nodes/{}/qemu/{}/agent/fsfreeze-thaw",
                    api.node(),
                    vmid
                ),
                &[],
            )
            .await?;
            eprintln!("Filesystems thawed on VM {}.", vmid);
        }
    }
    Ok(())
}
