use clap::Subcommand;

#[derive(Subcommand)]
pub enum GroupCommand {
    /// List user groups
    List,
    /// Show group details
    Show {
        /// Group ID
        groupid: String,
    },
    /// Create a group
    Create {
        /// Group ID
        #[arg(long)]
        groupid: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Update a group
    Update {
        /// Group ID
        groupid: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a group
    Delete {
        /// Group ID
        groupid: String,
    },
}
