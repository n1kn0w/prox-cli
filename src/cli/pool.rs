use clap::Subcommand;

#[derive(Subcommand)]
pub enum PoolCommand {
    /// List resource pools
    List,
    /// Show pool details
    Show {
        /// Pool ID
        poolid: String,
    },
    /// Create a resource pool
    Create {
        /// Pool ID
        #[arg(long)]
        poolid: String,
        /// Comment / description
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a resource pool
    Delete {
        /// Pool ID
        poolid: String,
    },
    /// Add resources to a pool
    Add {
        /// Pool ID
        poolid: String,
        /// VM/CT IDs to add
        #[arg(long)]
        vmid: Vec<u32>,
        /// Storage names to add
        #[arg(long)]
        storage: Vec<String>,
    },
    /// Remove resources from a pool
    Remove {
        /// Pool ID
        poolid: String,
        /// VM/CT IDs to remove
        #[arg(long)]
        vmid: Vec<u32>,
        /// Storage names to remove
        #[arg(long)]
        storage: Vec<String>,
    },
}
