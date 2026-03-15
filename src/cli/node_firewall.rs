use clap::Subcommand;

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
