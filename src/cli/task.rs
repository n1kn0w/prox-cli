use clap::Subcommand;

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
