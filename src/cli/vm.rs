use clap::Subcommand;

#[derive(Subcommand)]
pub enum VmCommand {
    /// List all VMs
    List,
    /// Show VM status
    Status {
        /// VM ID
        vmid: u32,
    },
    /// Start a VM
    Start {
        /// VM ID
        vmid: u32,
    },
    /// Stop a VM (immediate)
    Stop {
        /// VM ID
        vmid: u32,
    },
    /// Shutdown a VM (graceful ACPI)
    Shutdown {
        /// VM ID
        vmid: u32,
    },
    /// Show VM configuration
    Config {
        /// VM ID
        vmid: u32,
    },
    /// Modify VM configuration
    Set {
        /// VM ID
        vmid: u32,
        #[arg(long)]
        name: Option<String>,
        /// Memory in MB
        #[arg(long)]
        memory: Option<u64>,
        /// CPU cores
        #[arg(long)]
        cores: Option<u32>,
        /// CPU sockets
        #[arg(long)]
        sockets: Option<u32>,
    },
    /// Create a new VM
    Create {
        /// VM ID
        #[arg(long)]
        vmid: u32,
        /// VM name
        #[arg(long)]
        name: Option<String>,
        /// Memory in MB
        #[arg(long, default_value = "2048")]
        memory: u64,
        /// CPU cores
        #[arg(long, default_value = "2")]
        cores: u32,
        /// Storage for disk
        #[arg(long, default_value = "fast-vms")]
        storage: String,
        /// Disk size in GB
        #[arg(long, default_value = "32")]
        disk: u32,
        /// ISO image (e.g. local:iso/ubuntu.iso)
        #[arg(long)]
        iso: Option<String>,
        /// Network bridge
        #[arg(long, default_value = "vmbr1")]
        bridge: String,
        /// VLAN tag
        #[arg(long)]
        vlan: Option<u32>,
        /// OS type (l26, win11, etc.)
        #[arg(long, default_value = "l26")]
        ostype: String,
        /// Start VM after creation
        #[arg(long)]
        start: bool,
    },
    /// Delete a VM
    Delete {
        /// VM ID
        vmid: u32,
        /// Remove from all related configurations
        #[arg(long)]
        purge: bool,
    },
    /// Clone a VM
    Clone {
        /// Source VM ID
        vmid: u32,
        /// New VM ID
        #[arg(long)]
        newid: u32,
        /// Name for the clone
        #[arg(long)]
        name: Option<String>,
        /// Full clone (not linked)
        #[arg(long)]
        full: bool,
        /// Target storage
        #[arg(long)]
        storage: Option<String>,
    },
    /// Create a snapshot
    Snapshot {
        /// VM ID
        vmid: u32,
        /// Snapshot name
        #[arg(long)]
        name: String,
    },
    /// Rollback to a snapshot
    Rollback {
        /// VM ID
        vmid: u32,
        /// Snapshot name
        #[arg(long)]
        name: String,
    },
    /// List snapshots
    Snapshots {
        /// VM ID
        vmid: u32,
    },
}
