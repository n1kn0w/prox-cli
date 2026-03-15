use clap::Subcommand;

#[derive(Subcommand)]
pub enum HardwareCommand {
    /// List PCI devices
    #[command(name = "pci-list")]
    PciList,
    /// Show PCI device details
    #[command(name = "pci-show")]
    PciShow {
        /// PCI device ID (e.g. 0000:01:00.0)
        pciid: String,
    },
    /// List mediated device types for a PCI device
    #[command(name = "pci-mdev")]
    PciMdev {
        /// PCI device ID
        pciid: String,
    },
    /// List USB devices
    #[command(name = "usb-list")]
    UsbList,
}
