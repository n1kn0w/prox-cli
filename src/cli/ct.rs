use clap::Subcommand;

#[derive(Subcommand)]
pub enum CtCommand {
    /// List all containers
    List,
    /// Show container status
    Status { vmid: u32 },
    /// Start a container
    Start { vmid: u32 },
    /// Stop a container
    Stop { vmid: u32 },
    /// Show container configuration
    Config { vmid: u32 },
    /// Modify container configuration
    Set {
        vmid: u32,
        #[arg(long)]
        hostname: Option<String>,
        /// Memory in MB
        #[arg(long)]
        memory: Option<u64>,
        /// CPU cores
        #[arg(long)]
        cores: Option<u32>,
    },
    /// Create a new container
    Create {
        /// Container ID
        #[arg(long)]
        vmid: u32,
        /// OS template (e.g. bulk-backup:vztmpl/ubuntu.tar.gz)
        #[arg(long)]
        ostemplate: String,
        #[arg(long)]
        hostname: Option<String>,
        /// Memory in MB
        #[arg(long, default_value = "512")]
        memory: u64,
        /// CPU cores
        #[arg(long, default_value = "1")]
        cores: u32,
        /// Storage for rootfs
        #[arg(long, default_value = "fast-vms")]
        storage: String,
        /// Root filesystem size in GB
        #[arg(long, default_value = "8")]
        rootfs: u32,
        /// Network bridge
        #[arg(long, default_value = "vmbr1")]
        bridge: String,
        /// IP address (CIDR, e.g. 10.0.1.10/24)
        #[arg(long)]
        ip: Option<String>,
        /// Gateway
        #[arg(long)]
        gw: Option<String>,
        /// VLAN tag
        #[arg(long)]
        vlan: Option<u32>,
        /// Root password
        #[arg(long)]
        password: Option<String>,
        /// Start after creation
        #[arg(long)]
        start: bool,
    },
    /// Delete a container
    Delete {
        vmid: u32,
        /// Force stop before deleting
        #[arg(long)]
        force: bool,
    },
    /// List available container templates
    Templates,
    /// Pull OCI/Docker image
    Pull {
        /// Image reference (e.g. docker.io/library/ubuntu:latest)
        #[arg(long)]
        reference: String,
        /// Target storage
        #[arg(long, default_value = "bulk-backup")]
        storage: String,
    },
    /// Create a snapshot
    Snapshot {
        vmid: u32,
        /// Snapshot name
        #[arg(long)]
        name: String,
    },
    /// Rollback to a snapshot
    Rollback {
        vmid: u32,
        /// Snapshot name
        #[arg(long)]
        name: String,
    },
    /// List snapshots
    Snapshots { vmid: u32 },
}
