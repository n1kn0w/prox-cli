use clap::Subcommand;

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
