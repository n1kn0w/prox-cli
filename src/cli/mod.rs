mod agent;
mod apt;
mod backup;
mod bulk;
mod conf;
mod console;
mod ct;
mod disk;
mod domain;
mod firewall;
mod group;
mod hardware;
mod network;
mod node;
mod node_firewall;
mod pool;
mod scan;
mod storage;
mod task;
mod template;
mod tfa;
mod user;
mod vm;

pub use agent::AgentCommand;
pub use apt::AptCommand;
pub use backup::BackupCommand;
pub use bulk::BulkCommand;
pub use conf::ConfCommand;
pub use console::ConsoleCommand;
pub use ct::CtCommand;
pub use disk::DiskCommand;
pub use domain::DomainCommand;
pub use firewall::FirewallCommand;
pub use group::GroupCommand;
pub use hardware::HardwareCommand;
pub use network::NetworkCommand;
pub use node::NodeCommand;
pub use node_firewall::NodeFirewallCommand;
pub use pool::PoolCommand;
pub use scan::ScanCommand;
pub use storage::StorageCommand;
pub use task::TaskCommand;
pub use template::TemplateCommand;
pub use tfa::TfaCommand;
pub use user::UserCommand;
pub use vm::VmCommand;

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

    /// Config file path (overrides active profile)
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Verbose mode (-v: requests, -vv: requests + responses)
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
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
    /// Dashboard: node status + VM/CT overview
    Status,
    /// SSH into a VM via guest agent IP resolution
    Ssh {
        /// VM ID
        vmid: u32,
        /// SSH user (default: root)
        #[arg(long, default_value = "root")]
        user: String,
        /// Network interface to use for IP resolution
        #[arg(long)]
        interface: Option<String>,
        /// SSH proxy/jump host (e.g. user@bastion:2222), overrides config
        #[arg(long, short = 'J')]
        proxy: Option<String>,
    },
    /// Snapshot all VMs and CTs
    SnapAll {
        /// Snapshot name
        name: String,
        /// Only snapshot running guests
        #[arg(long)]
        running_only: bool,
    },
    /// Rollback all VMs and CTs to a snapshot
    RollbackAll {
        /// Snapshot name to rollback to
        name: String,
    },
    /// Manage config profiles
    Conf {
        #[command(subcommand)]
        command: ConfCommand,
    },
    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: Shell,
    },
}
