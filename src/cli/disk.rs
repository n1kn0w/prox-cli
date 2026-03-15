use clap::Subcommand;

#[derive(Subcommand)]
pub enum DiskCommand {
    /// Show SMART data for a disk
    Smart {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// Initialize disk with GPT partition table
    #[command(name = "init-gpt")]
    InitGpt {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// Wipe a disk (remove all partitions and data)
    Wipe {
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        disk: String,
    },
    /// List LVM volume groups
    #[command(name = "lvm-list")]
    LvmList,
    /// Create an LVM volume group
    #[command(name = "lvm-create")]
    LvmCreate {
        /// Volume group name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        device: String,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Delete an LVM volume group
    #[command(name = "lvm-delete")]
    LvmDelete {
        /// Volume group name
        name: String,
    },
    /// List LVM thin pools
    #[command(name = "lvmthin-list")]
    LvmThinList,
    /// Create an LVM thin pool
    #[command(name = "lvmthin-create")]
    LvmThinCreate {
        /// Thin pool name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb)
        #[arg(long)]
        device: String,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Delete an LVM thin pool
    #[command(name = "lvmthin-delete")]
    LvmThinDelete {
        /// Thin pool name
        name: String,
    },
    /// List directory storages
    #[command(name = "dir-list")]
    DirectoryList,
    /// Create a directory storage on a disk
    #[command(name = "dir-create")]
    DirectoryCreate {
        /// Directory name
        #[arg(long)]
        name: String,
        /// Disk device path (e.g. /dev/sdb1)
        #[arg(long)]
        device: String,
        /// Filesystem type (ext4, xfs)
        #[arg(long)]
        filesystem: Option<String>,
        /// Add as Proxmox storage
        #[arg(long)]
        add_storage: bool,
    },
    /// Show ZFS pool details
    #[command(name = "zfs-detail")]
    ZfsDetail {
        /// ZFS pool name
        name: String,
    },
}
