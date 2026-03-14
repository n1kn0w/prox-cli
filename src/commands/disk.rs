use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::DiskCommand;
use crate::output;

const PROTECTED_DISKS: &[&str] = &["/dev/sda"];

fn is_protected_disk(disk: &str) -> bool {
    PROTECTED_DISKS.iter().any(|d| disk.starts_with(d))
}

pub async fn handle(
    api: &ProxmoxClient,
    cmd: DiskCommand,
    json: bool,
    yes: bool,
) -> Result<()> {
    match cmd {
        DiskCommand::Smart { disk } => {
            let data = api
                .get(&format!(
                    "/nodes/{}/disks/smart?disk={}",
                    api.node(),
                    disk
                ))
                .await?;
            output::print_raw(&data, json);
        }
        DiskCommand::InitGpt { disk } => {
            if is_protected_disk(&disk) {
                eprintln!("Refused: {} is a protected disk.", disk);
                return Ok(());
            }
            if !yes && !output::confirm(&format!("Initialize GPT on {}? ALL DATA WILL BE LOST", disk)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .post(
                    &format!("/nodes/{}/disks/initgpt", api.node()),
                    &[("disk", disk.as_str())],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("GPT initialized on {}.", disk);
        }
        DiskCommand::Wipe { disk } => {
            if is_protected_disk(&disk) {
                eprintln!("Refused: {} is a protected disk.", disk);
                return Ok(());
            }
            if !yes && !output::confirm(&format!("Wipe disk {}? ALL DATA WILL BE LOST", disk)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .put(
                    &format!("/nodes/{}/disks/wipedisk", api.node()),
                    &[("disk", disk.as_str())],
                )
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Disk {} wiped.", disk);
        }
        DiskCommand::LvmList => {
            let data = api
                .get(&format!("/nodes/{}/disks/lvm", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("name", "NAME"),
                    ("size", "SIZE"),
                    ("free", "FREE"),
                    ("lvcount", "LV COUNT"),
                ],
            );
        }
        DiskCommand::LvmCreate { name, device, add_storage } => {
            if is_protected_disk(&device) {
                eprintln!("Refused: {} is a protected disk.", device);
                return Ok(());
            }
            let mut params: Vec<(&str, String)> = vec![
                ("name", name.clone()),
                ("device", device),
            ];
            if add_storage {
                params.push(("add_storage", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = api
                .post(&format!("/nodes/{}/disks/lvm", api.node()), &refs)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("LVM volume group '{}' created.", name);
        }
        DiskCommand::LvmDelete { name } => {
            if !yes && !output::confirm(&format!("Delete LVM volume group '{}'?", name)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .delete(&format!("/nodes/{}/disks/lvm/{}", api.node(), name))
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("LVM volume group '{}' deleted.", name);
        }
        DiskCommand::LvmThinList => {
            let data = api
                .get(&format!("/nodes/{}/disks/lvmthin", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("lv", "THIN POOL"),
                    ("vg", "VOLUME GROUP"),
                    ("lv_size", "SIZE"),
                    ("metadata_size", "META SIZE"),
                ],
            );
        }
        DiskCommand::LvmThinCreate { name, device, add_storage } => {
            if is_protected_disk(&device) {
                eprintln!("Refused: {} is a protected disk.", device);
                return Ok(());
            }
            let mut params: Vec<(&str, String)> = vec![
                ("name", name.clone()),
                ("device", device),
            ];
            if add_storage {
                params.push(("add_storage", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = api
                .post(&format!("/nodes/{}/disks/lvmthin", api.node()), &refs)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("LVM thin pool '{}' created.", name);
        }
        DiskCommand::LvmThinDelete { name } => {
            if !yes && !output::confirm(&format!("Delete LVM thin pool '{}'?", name)) {
                eprintln!("Cancelled.");
                return Ok(());
            }
            let data = api
                .delete(&format!("/nodes/{}/disks/lvmthin/{}", api.node(), name))
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("LVM thin pool '{}' deleted.", name);
        }
        DiskCommand::DirectoryList => {
            let data = api
                .get(&format!("/nodes/{}/disks/directory", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("path", "PATH"),
                    ("device", "DEVICE"),
                    ("type", "FS TYPE"),
                    ("unitfile", "UNIT"),
                ],
            );
        }
        DiskCommand::DirectoryCreate { name, device, filesystem, add_storage } => {
            if is_protected_disk(&device) {
                eprintln!("Refused: {} is a protected disk.", device);
                return Ok(());
            }
            let mut params: Vec<(&str, String)> = vec![
                ("name", name.clone()),
                ("device", device),
            ];
            if let Some(fs) = filesystem {
                params.push(("filesystem", fs));
            }
            if add_storage {
                params.push(("add_storage", "1".to_string()));
            }
            let refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = api
                .post(&format!("/nodes/{}/disks/directory", api.node()), &refs)
                .await?;
            if let Some(upid) = data.as_str() {
                api.wait_task(upid).await?;
            }
            eprintln!("Directory storage '{}' created.", name);
        }
        DiskCommand::ZfsDetail { name } => {
            let data = api
                .get(&format!("/nodes/{}/disks/zfs/{}", api.node(), name))
                .await?;
            output::print_raw(&data, json);
        }
    }
    Ok(())
}
