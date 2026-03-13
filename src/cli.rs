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
    /// Firewall management (cluster, VM, CT, IP sets, aliases)
    Firewall {
        #[command(subcommand)]
        command: FirewallCommand,
    },
    /// Backup and restore management (vzdump)
    Backup {
        #[command(subcommand)]
        command: BackupCommand,
    },
    /// Task management (list, status, log, cancel)
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
    /// Node information and diagnostics
    Node {
        #[command(subcommand)]
        command: NodeCommand,
    },
    /// Resource pool management
    Pool {
        #[command(subcommand)]
        command: PoolCommand,
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

// ── Backup ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum BackupCommand {
    /// Create a backup (vzdump)
    Create {
        /// VM/CT ID to backup
        #[arg(long)]
        vmid: u32,
        /// Target storage
        #[arg(long, default_value = "bulk-backup")]
        storage: String,
        /// Backup mode (snapshot, suspend, stop)
        #[arg(long, default_value = "snapshot")]
        mode: String,
        /// Compression algorithm (zstd, lzo, gzip)
        #[arg(long, default_value = "zstd")]
        compress: String,
        /// Backup notes
        #[arg(long)]
        notes: Option<String>,
    },
    /// List backups on a storage
    List {
        /// Storage name
        #[arg(long, default_value = "bulk-backup")]
        storage: String,
    },
    /// Restore a backup
    Restore {
        /// Backup volume ID (e.g. bulk-backup:backup/vzdump-qemu-200-2024_01_01.vma.zst)
        #[arg(long)]
        archive: String,
        /// Target VM/CT ID
        #[arg(long)]
        vmid: u32,
        /// Target storage for restored disks
        #[arg(long, default_value = "fast-vms")]
        storage: String,
        /// Overwrite existing VM/CT
        #[arg(long)]
        force: bool,
    },
    /// Delete a backup file
    Delete {
        /// Volume ID to delete
        #[arg(long)]
        volid: String,
    },
    /// List scheduled backup jobs
    Jobs,
    /// Create a scheduled backup job
    #[command(name = "job-create")]
    JobCreate {
        /// VM/CT ID or "all"
        #[arg(long)]
        vmid: String,
        /// Target storage
        #[arg(long, default_value = "bulk-backup")]
        storage: String,
        /// Schedule in cron format (e.g. "0 2 * * *")
        #[arg(long)]
        schedule: String,
        /// Backup mode (snapshot, suspend, stop)
        #[arg(long, default_value = "snapshot")]
        mode: String,
        /// Compression algorithm (zstd, lzo, gzip)
        #[arg(long, default_value = "zstd")]
        compress: String,
        /// Mail notification (always, failure)
        #[arg(long)]
        mailnotification: Option<String>,
        /// Enable the job
        #[arg(long)]
        enabled: bool,
    },
}

// ── Tasks ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum TaskCommand {
    /// List recent tasks
    List {
        /// Filter by VM/CT ID
        #[arg(long)]
        vmid: Option<u32>,
        /// Task source (all, active, archive)
        #[arg(long, default_value = "all")]
        source: String,
        /// Maximum number of tasks to return
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Show only failed tasks
        #[arg(long)]
        errors_only: bool,
    },
    /// Show task status
    Status {
        /// Task UPID
        upid: String,
    },
    /// Show task log
    Log {
        /// Task UPID
        upid: String,
        /// Maximum number of log lines
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Start offset (line number)
        #[arg(long)]
        start: Option<u32>,
    },
    /// Cancel a running task
    Cancel {
        /// Task UPID
        upid: String,
    },
}

// ── Node ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum NodeCommand {
    /// Show node status (CPU, memory, uptime)
    Status,
    /// Show node time and timezone
    Time,
    /// Show DNS configuration
    Dns,
    /// Show PVE version
    Version,
    /// List services
    Services,
    /// Show syslog entries
    Syslog {
        /// Maximum number of lines
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Start offset (line number)
        #[arg(long)]
        start: Option<u32>,
        /// Filter by service name
        #[arg(long)]
        service: Option<String>,
    },
}

// ── Pool ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum PoolCommand {
    /// List resource pools
    List,
    /// Show pool details
    Show {
        /// Pool ID
        poolid: String,
    },
    /// Create a resource pool
    Create {
        /// Pool ID
        #[arg(long)]
        poolid: String,
        /// Comment / description
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a resource pool
    Delete {
        /// Pool ID
        poolid: String,
    },
    /// Add resources to a pool
    Add {
        /// Pool ID
        poolid: String,
        /// VM/CT IDs to add
        #[arg(long)]
        vmid: Vec<u32>,
        /// Storage names to add
        #[arg(long)]
        storage: Vec<String>,
    },
    /// Remove resources from a pool
    Remove {
        /// Pool ID
        poolid: String,
        /// VM/CT IDs to remove
        #[arg(long)]
        vmid: Vec<u32>,
        /// Storage names to remove
        #[arg(long)]
        storage: Vec<String>,
    },
}

// ── Firewall ────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum FirewallCommand {
    // ── Cluster-level rules ─────────────────────────────────────────
    /// List cluster firewall rules
    #[command(name = "cluster-rules")]
    ClusterRules,
    /// Add a cluster firewall rule
    #[command(name = "cluster-add")]
    ClusterAdd {
        /// Action (ACCEPT, DROP, REJECT)
        #[arg(long)]
        action: String,
        /// Direction (in, out, group)
        #[arg(long, name = "type")]
        rule_type: String,
        /// Network interface
        #[arg(long)]
        iface: Option<String>,
        /// Source address/CIDR
        #[arg(long)]
        source: Option<String>,
        /// Destination address/CIDR
        #[arg(long)]
        dest: Option<String>,
        /// Destination port
        #[arg(long)]
        dport: Option<String>,
        /// Source port
        #[arg(long)]
        sport: Option<String>,
        /// Protocol (tcp, udp, icmp, etc.)
        #[arg(long)]
        proto: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Enable rule (default true)
        #[arg(long)]
        enable: Option<bool>,
        /// Position in rule list
        #[arg(long)]
        pos: Option<u32>,
    },
    /// Delete a cluster firewall rule by position
    #[command(name = "cluster-delete")]
    ClusterDelete {
        /// Rule position
        pos: u32,
    },
    /// Show cluster firewall options
    #[command(name = "cluster-options")]
    ClusterOptions,
    /// Enable or disable the cluster firewall
    #[command(name = "cluster-enable")]
    ClusterEnable {
        /// 1 to enable, 0 to disable
        enable: u8,
    },

    // ── VM-level rules ──────────────────────────────────────────────
    /// List VM firewall rules
    #[command(name = "vm-rules")]
    VmRules {
        /// VM ID
        #[arg(long)]
        vmid: u32,
    },
    /// Add a VM firewall rule
    #[command(name = "vm-add")]
    VmAdd {
        /// VM ID
        #[arg(long)]
        vmid: u32,
        /// Action (ACCEPT, DROP, REJECT)
        #[arg(long)]
        action: String,
        /// Direction (in, out, group)
        #[arg(long, name = "type")]
        rule_type: String,
        /// Network interface
        #[arg(long)]
        iface: Option<String>,
        /// Source address/CIDR
        #[arg(long)]
        source: Option<String>,
        /// Destination address/CIDR
        #[arg(long)]
        dest: Option<String>,
        /// Destination port
        #[arg(long)]
        dport: Option<String>,
        /// Source port
        #[arg(long)]
        sport: Option<String>,
        /// Protocol (tcp, udp, icmp, etc.)
        #[arg(long)]
        proto: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Enable rule (default true)
        #[arg(long)]
        enable: Option<bool>,
        /// Position in rule list
        #[arg(long)]
        pos: Option<u32>,
    },
    /// Delete a VM firewall rule by position
    #[command(name = "vm-delete")]
    VmDelete {
        /// VM ID
        #[arg(long)]
        vmid: u32,
        /// Rule position
        #[arg(long)]
        pos: u32,
    },
    /// Show VM firewall options
    #[command(name = "vm-options")]
    VmOptions {
        /// VM ID
        #[arg(long)]
        vmid: u32,
    },
    /// Enable or disable VM firewall
    #[command(name = "vm-enable")]
    VmEnable {
        /// VM ID
        #[arg(long)]
        vmid: u32,
        /// 1 to enable, 0 to disable
        #[arg(long)]
        enable: u8,
    },

    // ── CT-level rules ──────────────────────────────────────────────
    /// List container firewall rules
    #[command(name = "ct-rules")]
    CtRules {
        /// Container ID
        #[arg(long)]
        vmid: u32,
    },
    /// Add a container firewall rule
    #[command(name = "ct-add")]
    CtAdd {
        /// Container ID
        #[arg(long)]
        vmid: u32,
        /// Action (ACCEPT, DROP, REJECT)
        #[arg(long)]
        action: String,
        /// Direction (in, out, group)
        #[arg(long, name = "type")]
        rule_type: String,
        /// Network interface
        #[arg(long)]
        iface: Option<String>,
        /// Source address/CIDR
        #[arg(long)]
        source: Option<String>,
        /// Destination address/CIDR
        #[arg(long)]
        dest: Option<String>,
        /// Destination port
        #[arg(long)]
        dport: Option<String>,
        /// Source port
        #[arg(long)]
        sport: Option<String>,
        /// Protocol (tcp, udp, icmp, etc.)
        #[arg(long)]
        proto: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Enable rule (default true)
        #[arg(long)]
        enable: Option<bool>,
        /// Position in rule list
        #[arg(long)]
        pos: Option<u32>,
    },
    /// Delete a container firewall rule by position
    #[command(name = "ct-delete")]
    CtDelete {
        /// Container ID
        #[arg(long)]
        vmid: u32,
        /// Rule position
        #[arg(long)]
        pos: u32,
    },
    /// Show container firewall options
    #[command(name = "ct-options")]
    CtOptions {
        /// Container ID
        #[arg(long)]
        vmid: u32,
    },
    /// Enable or disable container firewall
    #[command(name = "ct-enable")]
    CtEnable {
        /// Container ID
        #[arg(long)]
        vmid: u32,
        /// 1 to enable, 0 to disable
        #[arg(long)]
        enable: u8,
    },

    // ── IP Sets ─────────────────────────────────────────────────────
    /// List IP sets
    #[command(name = "ipset-list")]
    IpsetList,
    /// Create an IP set
    #[command(name = "ipset-create")]
    IpsetCreate {
        /// IP set name
        #[arg(long)]
        name: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete an IP set
    #[command(name = "ipset-delete")]
    IpsetDelete {
        /// IP set name
        #[arg(long)]
        name: String,
    },
    /// List entries in an IP set
    #[command(name = "ipset-entries")]
    IpsetEntries {
        /// IP set name
        #[arg(long)]
        name: String,
    },
    /// Add an entry to an IP set
    #[command(name = "ipset-add")]
    IpsetAdd {
        /// IP set name
        #[arg(long)]
        name: String,
        /// CIDR address (e.g. 10.0.1.0/24)
        #[arg(long)]
        cidr: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Remove an entry from an IP set
    #[command(name = "ipset-remove")]
    IpsetRemove {
        /// IP set name
        #[arg(long)]
        name: String,
        /// CIDR address to remove
        #[arg(long)]
        cidr: String,
    },

    // ── Aliases ─────────────────────────────────────────────────────
    /// List firewall aliases
    #[command(name = "alias-list")]
    AliasList,
    /// Create a firewall alias
    #[command(name = "alias-create")]
    AliasCreate {
        /// Alias name
        #[arg(long)]
        name: String,
        /// CIDR address
        #[arg(long)]
        cidr: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a firewall alias
    #[command(name = "alias-delete")]
    AliasDelete {
        /// Alias name
        #[arg(long)]
        name: String,
    },
}
