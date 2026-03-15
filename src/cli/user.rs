use clap::Subcommand;

#[derive(Subcommand)]
pub enum UserCommand {
    /// List users
    List,
    /// Show user details
    Show {
        /// User ID (e.g. user1@pve)
        userid: String,
    },
    /// Create a user
    Create {
        /// User ID (e.g. user1@pve)
        #[arg(long)]
        userid: String,
        /// Password
        #[arg(long)]
        password: Option<String>,
        /// First name
        #[arg(long)]
        firstname: Option<String>,
        /// Last name
        #[arg(long)]
        lastname: Option<String>,
        /// Email
        #[arg(long)]
        email: Option<String>,
        /// Groups (comma-separated)
        #[arg(long)]
        groups: Option<String>,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    /// Delete a user
    Delete {
        /// User ID
        userid: String,
    },
    /// Change user password
    #[command(name = "set-password")]
    SetPassword {
        /// User ID
        userid: String,
        /// New password
        #[arg(long)]
        password: String,
    },
    /// Set ACL permissions
    Acl {
        /// User ID
        #[arg(long)]
        userid: String,
        /// ACL path (e.g. /vms/200, /pool/mypool)
        #[arg(long)]
        path: String,
        /// Role (e.g. PVEVMAdmin, PVEAuditor)
        #[arg(long)]
        role: String,
        /// Propagate to child objects
        #[arg(long)]
        propagate: bool,
    },
    /// List available roles
    Roles,
    /// List ACL entries
    Acls,
}
