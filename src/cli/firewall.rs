use clap::Subcommand;

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
