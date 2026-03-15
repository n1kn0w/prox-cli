use clap::Subcommand;

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
