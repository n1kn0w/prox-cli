use clap::Subcommand;

#[derive(Subcommand)]
pub enum TfaCommand {
    /// List all TFA entries
    List,
    /// List TFA entries for a user
    #[command(name = "user-list")]
    UserList {
        /// User ID (e.g. user1@pve)
        userid: String,
    },
    /// Add a TFA entry for a user
    Add {
        /// User ID (e.g. user1@pve)
        #[arg(long)]
        userid: String,
        /// TFA type (totp, u2f, webauthn, recovery)
        #[arg(long, name = "type")]
        tfa_type: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// TOTP URI (for type=totp)
        #[arg(long)]
        totp: Option<String>,
        /// Value (for recovery keys, etc.)
        #[arg(long)]
        value: Option<String>,
        /// Current user password (for verification)
        #[arg(long)]
        password: Option<String>,
    },
    /// Show a specific TFA entry
    Show {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
    },
    /// Update a TFA entry
    Update {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// Enable or disable
        #[arg(long)]
        enable: Option<bool>,
    },
    /// Delete a TFA entry
    Delete {
        /// User ID
        #[arg(long)]
        userid: String,
        /// TFA entry ID
        #[arg(long)]
        id: String,
    },
}
