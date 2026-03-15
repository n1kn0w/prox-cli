use clap::Subcommand;

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
