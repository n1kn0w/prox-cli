use clap::Subcommand;

#[derive(Subcommand)]
pub enum NetworkCommand {
    /// List network interfaces
    List,
    /// Create a network interface (bridge, VLAN)
    Create {
        /// Interface name (e.g. vmbr2)
        #[arg(long)]
        iface: String,
        /// Interface type (bridge, vlan, bond, etc.)
        #[arg(long, name = "type")]
        iface_type: String,
        /// Bridge ports
        #[arg(long)]
        bridge_ports: Option<String>,
        /// VLAN ID
        #[arg(long)]
        vlan_id: Option<u32>,
        /// Parent device for VLAN
        #[arg(long)]
        vlan_raw_device: Option<String>,
        /// IP address (CIDR)
        #[arg(long)]
        cidr: Option<String>,
        /// Gateway
        #[arg(long)]
        gateway: Option<String>,
        /// Enable VLAN awareness (for bridges)
        #[arg(long)]
        vlan_aware: bool,
        /// Autostart on boot
        #[arg(long)]
        autostart: bool,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a network interface
    Delete {
        /// Interface name
        iface: String,
    },
    /// Apply pending network changes
    Apply,
    /// Revert pending network changes
    Revert,
}
