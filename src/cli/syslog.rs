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
    /// Show remote syslog forwarding configuration (via SSH)
    ConfigShow {
        /// SSH user for the Proxmox node
        #[arg(long, default_value = "root")]
        ssh_user: String,
        /// SSH proxy/jump host (overrides config)
        #[arg(long, short = 'J')]
        proxy: Option<String>,
    },
    /// Set remote syslog forwarding target (via SSH)
    ConfigSet {
        /// Remote syslog server address
        #[arg(long)]
        server: String,
        /// Remote syslog server port
        #[arg(long, default_value = "514")]
        port: u16,
        /// Protocol: tcp or udp
        #[arg(long, default_value = "udp")]
        protocol: String,
        /// Syslog facility filter
        #[arg(long, default_value = "*.*")]
        facility: String,
        /// SSH user for the Proxmox node
        #[arg(long, default_value = "root")]
        ssh_user: String,
        /// SSH proxy/jump host (overrides config)
        #[arg(long, short = 'J')]
        proxy: Option<String>,
    },
    /// Remove remote syslog forwarding configuration (via SSH)
    ConfigDelete {
        /// SSH user for the Proxmox node
        #[arg(long, default_value = "root")]
        ssh_user: String,
        /// SSH proxy/jump host (overrides config)
        #[arg(long, short = 'J')]
        proxy: Option<String>,
    },
}
