use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::ConsoleCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: ConsoleCommand, json: bool) -> Result<()> {
    match cmd {
        ConsoleCommand::Vm { vmid, serial } => {
            eprintln!("Creating terminal proxy for VM {}...", vmid);

            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(s) = serial {
                params.push(("serial", s.to_string()));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();

            let data = api
                .post(
                    &format!("/nodes/{}/qemu/{}/termproxy", api.node(), vmid),
                    &refs,
                )
                .await?;
            output::print_raw(&data, json);
        }
        ConsoleCommand::Ct { vmid } => {
            eprintln!("Creating terminal proxy for CT {}...", vmid);

            let data = api
                .post(
                    &format!("/nodes/{}/lxc/{}/termproxy", api.node(), vmid),
                    &[],
                )
                .await?;
            output::print_raw(&data, json);
        }
        ConsoleCommand::Node => {
            eprintln!("Creating terminal proxy for node {}...", api.node());

            let data = api
                .post(
                    &format!("/nodes/{}/termproxy", api.node()),
                    &[],
                )
                .await?;
            output::print_raw(&data, json);
        }
    }
    Ok(())
}
