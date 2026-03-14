use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::ScanCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: ScanCommand, json: bool) -> Result<()> {
    match cmd {
        ScanCommand::Nfs { server } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/scan/nfs?server={}",
                    api.node(),
                    server
                ))
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
            let mut url = format!(
                "/nodes/{}/scan/cifs?server={}",
                api.node(),
                server
            );
            if let Some(u) = &username {
                url.push_str(&format!("&username={}", u));
            }
            if let Some(p) = &password {
                url.push_str(&format!("&password={}", p));
            }
            if let Some(d) = &domain {
                url.push_str(&format!("&domain={}", d));
            }
            let data = api.get(&url).await?;
            output::print_list(
                &data,
                json,
                &[("share", "SHARE"), ("description", "DESCRIPTION")],
            );
        }
        ScanCommand::Iscsi { portal } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/scan/iscsi?portal={}",
                    api.node(),
                    portal
                ))
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
                .get(&format!(
                    "/nodes/{}/scan/lvmthin?vg={}",
                    api.node(),
                    vg
                ))
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
            let mut url = format!(
                "/nodes/{}/scan/pbs?server={}&username={}&password={}",
                api.node(),
                server,
                username,
                password
            );
            if let Some(fp) = &fingerprint {
                url.push_str(&format!("&fingerprint={}", fp));
            }
            if let Some(p) = port {
                url.push_str(&format!("&port={}", p));
            }
            let data = api.get(&url).await?;
            output::print_list(
                &data,
                json,
                &[("store", "STORE"), ("comment", "COMMENT")],
            );
        }
        ScanCommand::Glusterfs { server } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/scan/glusterfs?server={}",
                    api.node(),
                    server
                ))
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
