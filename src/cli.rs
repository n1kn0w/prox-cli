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
    /// APT package management
    Apt {
        #[command(subcommand)]
        command: AptCommand,
    },
    /// Guest agent commands (execute in VM/CT)
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
    /// Advanced disk management (SMART, LVM, wipedisk, GPT, directories)
    Disk {
        #[command(subcommand)]
        command: DiskCommand,
    },
    /// User group management
    Group {
        #[command(subcommand)]
        command: GroupCommand,
    },
    /// Two-factor authentication management
    Tfa {
        #[command(subcommand)]
        command: TfaCommand,
    },
    /// Authentication realm management (PAM, PVE, LDAP, AD, OpenID)
    Domain {
        #[command(subcommand)]
        command: DomainCommand,
    },
    /// Node-level firewall rules and options
    NodeFirewall {
        #[command(subcommand)]
        command: NodeFirewallCommand,
    },
    /// Console/terminal proxy access
    Console {
        #[command(subcommand)]
        command: ConsoleCommand,
    },
    /// Bulk start/stop/migrate/suspend operations
    Bulk {
        #[command(subcommand)]
        command: BulkCommand,
    },
    /// Hardware information (PCI/USB passthrough)
    Hardware {
        #[command(subcommand)]
        command: HardwareCommand,
    },
    /// Scan for available storage targets
    Scan {
        #[command(subcommand)]
        command: ScanCommand,
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

// ── APT ───────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum AptCommand {
    /// List APT repositories
    Repos,
    /// Update package index
    Update,
    /// List available upgrades
    Upgrade,
    /// Show installed package versions
    Versions,
    /// Show changelog for a package
    Changelog {
        /// Package name
        #[arg(long)]
        name: String,
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

// ── Agent ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum AgentCommand {
    /// Ping guest agent
    Ping {
        /// VM ID
        vmid: u32,
    },
    /// Get guest OS info
    Info {
        /// VM ID
        vmid: u32,
    },
    /// Get guest network interfaces
    Network {
        /// VM ID
        vmid: u32,
    },
    /// Execute a command in the guest
    Exec {
        /// VM ID
        vmid: u32,
        /// Command to run
        #[arg(long)]
        command: String,
        /// Stdin data
        #[arg(long)]
        input_data: Option<String>,
    },
    /// Get status of a previous exec command
    #[command(name = "exec-status")]
    ExecStatus {
        /// VM ID
        vmid: u32,
        /// Process ID returned by exec
        #[arg(long)]
        pid: u32,
    },
    /// Read a file from the guest
    #[command(name = "file-read")]
    FileRead {
        /// VM ID
        vmid: u32,
        /// File path inside the guest
        #[arg(long)]
        file: String,
    },
    /// Write content to a file in the guest
    #[command(name = "file-write")]
    FileWrite {
        /// VM ID
        vmid: u32,
        /// File path inside the guest
        #[arg(long)]
        file: String,
        /// Content to write
        #[arg(long)]
        content: String,
    },
    /// Set user password in the guest
    #[command(name = "set-password")]
    SetPassword {
        /// VM ID
        vmid: u32,
        /// Username
        #[arg(long)]
        username: String,
        /// New password
        #[arg(long)]
        password: String,
    },
    /// Shutdown guest via agent
    Shutdown {
        /// VM ID
        vmid: u32,
    },
    /// Freeze guest filesystems for snapshot
    Fsfreeze {
        /// VM ID
        vmid: u32,
    },
    /// Thaw guest filesystems
    Fsthaw {
        /// VM ID
        vmid: u32,
    },
}

// ── Disk ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum DiskCommand {
    /// Show SMART data for a disk
    Smart {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// Initialize disk with GPT partition table
    #[command(name = "init-gpt")]
    InitGpt {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// Wipe a disk (remove all partitions and data)
    Wipe {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// List LVM volume groups
    #[command(name = "lvm-list")]
    LvmList,
    /// Create an LVM volume group
    #[command(name = "lvm-create")]
    LvmCreate {
        /// Volume group name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        device: String,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Delete an LVM volume group
    #[command(name = "lvm-delete")]
    LvmDelete {
        /// Volume group name
        name: String,
    },
    /// List LVM thin pools
    #[command(name = "lvmthin-list")]
    LvmThinList,
    /// Create an LVM thin pool
    #[command(name = "lvmthin-create")]
    LvmThinCreate {
        /// Thin pool name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        device: String,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Delete an LVM thin pool
    #[command(name = "lvmthin-delete")]
    LvmThinDelete {
        /// Thin pool name
        name: String,
    },
    /// List directory storages
    #[command(name = "dir-list")]
    DirectoryList,
    /// Create a directory storage on a disk
    #[command(name = "dir-create")]
    DirectoryCreate {
        /// Directory name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb1)
        #[arg(long)]
        device: String,
        /// Filesystem type (ext4, xfs)
        #[arg(long)]
        filesystem: Option<String>,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Show ZFS pool details
    #[command(name = "zfs-detail")]
    ZfsDetail {
        /// ZFS pool name
        name: String,
    },
}

// ── Group ─────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum GroupCommand {
    /// List user groups
    List,
    /// Show group details
    Show {
        /// Group ID
        groupid: String,
    },
    /// Create a group
    Create {
        /// Group ID
        #[arg(long)]
        groupid: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Update a group
    Update {
        /// Group ID
        groupid: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a group
    Delete {
        /// Group ID
        groupid: String,
    },
}

// ── TFA ───────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum TfaCommand {
    /// List all TFA entries
    List,
    /// List TFA entries for a user
    #[command(name = "user-list")]
    UserList {
        /// User ID (e.g. user1@pve)
        userid: String,
    },
    /// Add a TFA entry for a user
    Add {
        /// User ID (e.g. user1@pve)
        #[arg(long)]
        userid: String,
        /// TFA type (totp, u2f, webauthn, recovery)
        #[arg(long, name = "type")]
        tfa_type: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// TOTP URI (for type=totp)
        #[arg(long)]
        totp: Option<String>,
        /// Value (for recovery keys, etc.)
        #[arg(long)]
        value: Option<String>,
        /// Current user password (for verification)
        #[arg(long)]
        password: Option<String>,
    },
    /// Show a specific TFA entry
    Show {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
    },
    /// Update a TFA entry
    Update {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// Enable or disable
        #[arg(long)]
        enable: Option<bool>,
    },
    /// Delete a TFA entry
    Delete {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
    },
}

// ── Domain ────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum DomainCommand {
    /// List authentication realms
    List,
    /// Show realm details
    Show {
        /// Realm name (e.g. pam, pve, my-ldap)
        realm: String,
    },
    /// Create an authentication realm
    Create {
        /// Realm name
        #[arg(long)]
        realm: String,
        /// Realm type (pam, pve, ldap, ad, openid)
        #[arg(long, name = "type")]
        realm_type: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Primary server (for LDAP/AD)
        #[arg(long)]
        server1: Option<String>,
        /// Fallback server (for LDAP/AD)
        #[arg(long)]
        server2: Option<String>,
        /// Server port
        #[arg(long)]
        port: Option<u32>,
        /// Base DN (for LDAP/AD)
        #[arg(long)]
        base_dn: Option<String>,
        /// User attribute name (for LDAP/AD)
        #[arg(long)]
        user_attr: Option<String>,
        /// Bind DN (for LDAP/AD)
        #[arg(long)]
        bind_dn: Option<String>,
        /// Set as default realm
        #[arg(long)]
        default: bool,
        /// TFA requirement (e.g. "type=totp")
        #[arg(long)]
        tfa: Option<String>,
    },
    /// Update an authentication realm
    Update {
        /// Realm name
        realm: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Primary server (for LDAP/AD)
        #[arg(long)]
        server1: Option<String>,
        /// Fallback server (for LDAP/AD)
        #[arg(long)]
        server2: Option<String>,
        /// Server port
        #[arg(long)]
        port: Option<u32>,
        /// Base DN (for LDAP/AD)
        #[arg(long)]
        base_dn: Option<String>,
        /// User attribute name (for LDAP/AD)
        #[arg(long)]
        user_attr: Option<String>,
        /// Bind DN (for LDAP/AD)
        #[arg(long)]
        bind_dn: Option<String>,
        /// Set as default realm
        #[arg(long)]
        default: Option<bool>,
        /// TFA requirement (e.g. "type=totp")
        #[arg(long)]
        tfa: Option<String>,
    },
    /// Delete an authentication realm
    Delete {
        /// Realm name
        realm: String,
    },
    /// Sync a realm (LDAP/AD)
    Sync {
        /// Realm name
        realm: String,
    },
}

// ── Node Firewall ─────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum NodeFirewallCommand {
    /// List node firewall rules
    List,
    /// Add a node firewall rule
    Add {
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
        /// Protocol (tcp, udp, icmp)
        #[arg(long)]
        proto: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Enable rule
        #[arg(long)]
        enable: Option<bool>,
        /// Position in rule list
        #[arg(long)]
        pos: Option<u32>,
    },
    /// Show a rule at position
    Show {
        /// Rule position
        pos: u32,
    },
    /// Update a rule at position
    Update {
        /// Rule position
        pos: u32,
        /// Action (ACCEPT, DROP, REJECT)
        #[arg(long)]
        action: Option<String>,
        /// Direction (in, out, group)
        #[arg(long, name = "type")]
        rule_type: Option<String>,
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
        /// Protocol (tcp, udp, icmp)
        #[arg(long)]
        proto: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Enable rule
        #[arg(long)]
        enable: Option<bool>,
    },
    /// Delete a rule at position
    Delete {
        /// Rule position
        pos: u32,
    },
    /// Show node firewall options
    Options,
    /// Set node firewall options
    #[command(name = "set-options")]
    SetOptions {
        /// Enable/disable firewall
        #[arg(long)]
        enable: Option<bool>,
        /// Input policy (ACCEPT, DROP, REJECT)
        #[arg(long)]
        policy_in: Option<String>,
        /// Output policy (ACCEPT, DROP, REJECT)
        #[arg(long)]
        policy_out: Option<String>,
        /// Log level for incoming (emerg, alert, crit, err, warning, notice, info, debug, nolog)
        #[arg(long)]
        log_level_in: Option<String>,
        /// Log level for outgoing
        #[arg(long)]
        log_level_out: Option<String>,
    },
    /// Show firewall log
    Log {
        /// Maximum number of lines
        #[arg(long)]
        limit: Option<u32>,
        /// Start offset
        #[arg(long)]
        start: Option<u32>,
    },
    /// List available IPSet/alias references
    Refs,
}

// ── Console ───────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ConsoleCommand {
    /// Create terminal proxy for a VM
    Vm {
        /// VM ID
        vmid: u32,
        /// Serial port number
        #[arg(long)]
        serial: Option<u32>,
    },
    /// Create terminal proxy for a container
    Ct {
        /// Container ID
        vmid: u32,
    },
    /// Create terminal proxy for the node shell
    Node,
}

// ── Bulk ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum BulkCommand {
    /// Start all VMs/CTs
    #[command(name = "start-all")]
    StartAll {
        /// Comma-separated VM/CT IDs to start (default: all)
        #[arg(long)]
        vms: Option<String>,
        /// Force start even if HA managed
        #[arg(long)]
        force: bool,
    },
    /// Stop all VMs/CTs
    #[command(name = "stop-all")]
    StopAll {
        /// Comma-separated VM/CT IDs to stop (default: all)
        #[arg(long)]
        vms: Option<String>,
        /// Force stop (don't wait for graceful shutdown)
        #[arg(long)]
        force_stop: bool,
    },
    /// Migrate all VMs/CTs to another node
    #[command(name = "migrate-all")]
    MigrateAll {
        /// Target node name
        #[arg(long)]
        target: String,
        /// Comma-separated VM/CT IDs to migrate (default: all)
        #[arg(long)]
        vms: Option<String>,
        /// Allow migration with local disks
        #[arg(long)]
        with_local_disks: bool,
    },
    /// Suspend all VMs/CTs
    #[command(name = "suspend-all")]
    SuspendAll {
        /// Comma-separated VM/CT IDs to suspend (default: all)
        #[arg(long)]
        vms: Option<String>,
    },
}

// ── Hardware ──────────────────────────────────────────────────────────

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

// ── Scan ──────────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ScanCommand {
    /// Scan NFS exports
    Nfs {
        /// NFS server address
        #[arg(long)]
        server: String,
    },
    /// Scan CIFS/SMB shares
    Cifs {
        /// CIFS server address
        #[arg(long)]
        server: String,
        /// Username
        #[arg(long)]
        username: Option<String>,
        /// Password
        #[arg(long)]
        password: Option<String>,
        /// Domain
        #[arg(long)]
        domain: Option<String>,
    },
    /// Scan iSCSI targets
    Iscsi {
        /// iSCSI portal address
        #[arg(long)]
        portal: String,
    },
    /// Scan LVM volume groups
    Lvm,
    /// Scan LVM thin pools
    Lvmthin {
        /// Volume group name
        #[arg(long)]
        vg: String,
    },
    /// Scan ZFS pools
    Zfs,
    /// Scan PBS (Proxmox Backup Server) datastores
    Pbs {
        /// PBS server address
        #[arg(long)]
        server: String,
        /// Username
        #[arg(long)]
        username: String,
        /// Password
        #[arg(long)]
        password: String,
        /// TLS fingerprint
        #[arg(long)]
        fingerprint: Option<String>,
        /// Server port
        #[arg(long)]
        port: Option<u32>,
    },
    /// Scan GlusterFS volumes
    Glusterfs {
        /// GlusterFS server address
        #[arg(long)]
        server: String,
    },
}
