use clap::Subcommand;

#[derive(Subcommand)]
pub enum SyslogCommand {
    /// View syslog entries with filtering
    List {
        /// Maximum number of lines
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Start offset (line number)
        #[arg(long)]
        start: Option<u32>,
        /// Filter by service name (e.g. pvedaemon, pveproxy)
        #[arg(long)]
        service: Option<String>,
        /// Display since this date-time (YYYY-MM-DD or epoch)
        #[arg(long)]
        since: Option<String>,
        /// Display until this date-time (YYYY-MM-DD or epoch)
        #[arg(long)]
        until: Option<String>,
    },
    /// View systemd journal entries
    Journal {
        /// Maximum number of lines
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Start offset (line number)
        #[arg(long)]
        start: Option<u32>,
        /// Display since this date-time (YYYY-MM-DD or epoch)
        #[arg(long)]
        since: Option<String>,
        /// Display until this date-time (YYYY-MM-DD or epoch)
        #[arg(long)]
        until: Option<String>,
        /// Limit to the last N entries
        #[arg(long)]
        lastentries: Option<u32>,
    },
    /// Show rsyslog service status
    ServiceStatus,
    /// Start rsyslog service
    ServiceStart,
    /// Stop rsyslog service
    ServiceStop,
    /// Restart rsyslog service
    ServiceRestart,
    /// Reload rsyslog service configuration
    ServiceReload,
}
