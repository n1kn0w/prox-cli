use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::ScanCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: ScanCommand, json: bool) -> Result<()> {
    match cmd {
        ScanCommand::Nfs { server } => {
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/nfs", api.node()),
                    &[("server", server.as_str())],
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("path", "PATH"), ("options", "OPTIONS")],
            );
        }
        ScanCommand::Cifs {
            server,
            username,
            password,
            domain,
        } => {
            let mut params: Vec<(&str, &str)> = vec![("server", server.as_str())];
            if let Some(ref u) = username {
                params.push(("username", u.as_str()));
            }
            if let Some(ref p) = password {
                params.push(("password", p.as_str()));
            }
            if let Some(ref d) = domain {
                params.push(("domain", d.as_str()));
            }
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/cifs", api.node()),
                    &params,
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("share", "SHARE"), ("description", "DESCRIPTION")],
            );
        }
        ScanCommand::Iscsi { portal } => {
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/iscsi", api.node()),
                    &[("portal", portal.as_str())],
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("target", "TARGET"), ("portal", "PORTAL")],
            );
        }
        ScanCommand::Lvm => {
            let data = api
                .get(&format!("/nodes/{}/scan/lvm", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[("vg", "VG"), ("size", "SIZE"), ("free", "FREE")],
            );
        }
        ScanCommand::Lvmthin { vg } => {
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/lvmthin", api.node()),
                    &[("vg", vg.as_str())],
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("lv", "LV"), ("size", "SIZE")],
            );
        }
        ScanCommand::Zfs => {
            let data = api
                .get(&format!("/nodes/{}/scan/zfs", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[("pool", "POOL"), ("size", "SIZE")],
            );
        }
        ScanCommand::Pbs {
            server,
            username,
            password,
            fingerprint,
            port,
        } => {
            let port_str = port.map(|p| p.to_string());
            let mut params: Vec<(&str, &str)> = vec![
                ("server", server.as_str()),
                ("username", username.as_str()),
                ("password", password.as_str()),
            ];
            if let Some(ref fp) = fingerprint {
                params.push(("fingerprint", fp.as_str()));
            }
            if let Some(ref p) = port_str {
                params.push(("port", p.as_str()));
            }
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/pbs", api.node()),
                    &params,
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("store", "STORE"), ("comment", "COMMENT")],
            );
        }
        ScanCommand::Glusterfs { server } => {
            let data = api
                .get_with_query(
                    &format!("/nodes/{}/scan/glusterfs", api.node()),
                    &[("server", server.as_str())],
                )
                .await?;
            output::print_list(
                &data,
                json,
                &[("volname", "VOLNAME")],
            );
        }
    }
    Ok(())
}
