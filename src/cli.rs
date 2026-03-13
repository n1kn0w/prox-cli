use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "prox-cli", about = "Proxmox Cyber Range CLI Manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,

    /// Skip confirmation prompts
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,

    /// Config file path
    #[arg(long, global = true, default_value = "config.toml")]
    pub config: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Virtual machine management (QEMU)
    Vm {
        #[command(subcommand)]
        command: VmCommand,
    },
    /// Container management (LXC)
    Ct {
        #[command(subcommand)]
        command: CtCommand,
    },
    /// Storage information
    Storage {
        #[command(subcommand)]
        command: StorageCommand,
    },
    /// Network management
    Network {
        #[command(subcommand)]
        command: NetworkCommand,
    },
    /// User management
    User {
        #[command(subcommand)]
        command: UserCommand,
    },
    /// Template management
    Template {
        #[command(subcommand)]
        command: TemplateCommand,
    },
    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: Shell,
    },
}

// ── VM ──────────────────────────────────────────────────────────────────

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

// ── Containers ──────────────────────────────────────────────────────────

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

// ── Storage ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum StorageCommand {
    /// List Proxmox storages
    List,
    /// Show ZFS pools
    Pools,
    /// List physical disks
    Disks,
    /// Storage usage status
    Status,
}

// ── Network ─────────────────────────────────────────────────────────────

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

// ── Users ───────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum UserCommand {
    /// List users
    List,
    /// Show user details
    Show {
        /// User ID (e.g. user1@pve)
        userid: String,
    },
    /// Create a user
    Create {
        /// User ID (e.g. user1@pve)
        #[arg(long)]
        userid: String,
        /// Password
        #[arg(long)]
        password: Option<String>,
        /// First name
        #[arg(long)]
        firstname: Option<String>,
        /// Last name
        #[arg(long)]
        lastname: Option<String>,
        /// Email
        #[arg(long)]
        email: Option<String>,
        /// Groups (comma-separated)
        #[arg(long)]
        groups: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a user
    Delete {
        /// User ID
        userid: String,
    },
    /// Change user password
    #[command(name = "set-password")]
    SetPassword {
        /// User ID
        userid: String,
        /// New password
        #[arg(long)]
        password: String,
    },
    /// Set ACL permissions
    Acl {
        /// User ID
        #[arg(long)]
        userid: String,
        /// ACL path (e.g. /vms/200, /pool/mypool)
        #[arg(long)]
        path: String,
        /// Role (e.g. PVEVMAdmin, PVEAuditor)
        #[arg(long)]
        role: String,
        /// Propagate to child objects
        #[arg(long)]
        propagate: bool,
    },
    /// List available roles
    Roles,
    /// List ACL entries
    Acls,
}

// ── Templates ───────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// List templates (VMs marked as template)
    List,
    /// Convert a VM to a template
    Create {
        /// VM ID to convert
        vmid: u32,
    },
    /// Clone from a template
    Clone {
        /// Template VM ID
        vmid: u32,
        /// New VM ID
        #[arg(long)]
        newid: u32,
        /// Name for the new VM
        #[arg(long)]
        name: Option<String>,
        /// Target storage
        #[arg(long)]
        storage: Option<String>,
        /// Full clone instead of linked
        #[arg(long)]
        full: bool,
    },
}
