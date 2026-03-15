use clap::Subcommand;

#[derive(Subcommand)]
pub enum StorageCommand {
    /// List Proxmox storages
    List,
    /// Show ZFS pools
    Pools,
    /// List physical disks
    Disks,
    /// Storage usage status
    Status,
}
