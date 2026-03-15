use clap::Subcommand;

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
