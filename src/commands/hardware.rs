use anyhow::Result;

use crate::api::ProxmoxClient;
use crate::cli::HardwareCommand;
use crate::output;

pub async fn handle(api: &ProxmoxClient, cmd: HardwareCommand, json: bool) -> Result<()> {
    match cmd {
        HardwareCommand::PciList => {
            eprintln!("Listing PCI devices...");
            let data = api
                .get(&format!("/nodes/{}/hardware/pci", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("id", "ID"),
                    ("vendor_name", "VENDOR"),
                    ("device_name", "DEVICE"),
                    ("class", "CLASS"),
                    ("iommugroup", "IOMMU GROUP"),
                    ("mdev", "MDEV"),
                ],
            );
        }
        HardwareCommand::PciShow { pciid } => {
            eprintln!("Showing PCI device {}...", pciid);
            let data = api
                .get(&format!(
                    "/nodes/{}/hardware/pci/{}",
                    api.node(),
                    pciid
                ))
                .await?;
            output::print_raw(&data, json);
        }
        HardwareCommand::PciMdev { pciid } => {
            eprintln!("Listing mediated device types for {}...", pciid);
            let data = api
                .get(&format!(
                    "/nodes/{}/hardware/pci/{}/mdev",
                    api.node(),
                    pciid
                ))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("type", "TYPE"),
                    ("name", "NAME"),
                    ("description", "DESCRIPTION"),
                    ("available", "AVAILABLE"),
                ],
            );
        }
        HardwareCommand::UsbList => {
            eprintln!("Listing USB devices...");
            let data = api
                .get(&format!("/nodes/{}/hardware/usb", api.node()))
                .await?;
            output::print_list(
                &data,
                json,
                &[
                    ("busnum", "BUS"),
                    ("devnum", "DEV"),
                    ("vendid", "VENDOR ID"),
                    ("prodid", "PRODUCT ID"),
                    ("manufacturer", "MANUFACTURER"),
                    ("product", "PRODUCT"),
                    ("speed", "SPEED"),
                    ("class", "CLASS"),
                ],
            );
        }
    }
    Ok(())
}
