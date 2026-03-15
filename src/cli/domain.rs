use clap::Subcommand;

#[derive(Subcommand)]
pub enum DomainCommand {
    /// List authentication realms
    List,
    /// Show realm details
    Show {
        /// Realm name (e.g. pam, pve, my-ldap)
        realm: String,
    },
    /// Create an authentication realm
    Create {
        /// Realm name
        #[arg(long)]
        realm: String,
        /// Realm type (pam, pve, ldap, ad, openid)
        #[arg(long, name = "type")]
        realm_type: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Primary server (for LDAP/AD)
        #[arg(long)]
        server1: Option<String>,
        /// Fallback server (for LDAP/AD)
        #[arg(long)]
        server2: Option<String>,
        /// Server port
        #[arg(long)]
        port: Option<u32>,
        /// Base DN (for LDAP/AD)
        #[arg(long)]
        base_dn: Option<String>,
        /// User attribute name (for LDAP/AD)
        #[arg(long)]
        user_attr: Option<String>,
        /// Bind DN (for LDAP/AD)
        #[arg(long)]
        bind_dn: Option<String>,
        /// Set as default realm
        #[arg(long)]
        default: bool,
        /// TFA requirement (e.g. "type=totp")
        #[arg(long)]
        tfa: Option<String>,
    },
    /// Update an authentication realm
    Update {
        /// Realm name
        realm: String,
        /// Comment
        #[arg(long)]
        comment: Option<String>,
        /// Primary server (for LDAP/AD)
        #[arg(long)]
        server1: Option<String>,
        /// Fallback server (for LDAP/AD)
        #[arg(long)]
        server2: Option<String>,
        /// Server port
        #[arg(long)]
        port: Option<u32>,
        /// Base DN (for LDAP/AD)
        #[arg(long)]
        base_dn: Option<String>,
        /// User attribute name (for LDAP/AD)
        #[arg(long)]
        user_attr: Option<String>,
        /// Bind DN (for LDAP/AD)
        #[arg(long)]
        bind_dn: Option<String>,
        /// Set as default realm
        #[arg(long)]
        default: Option<bool>,
        /// TFA requirement (e.g. "type=totp")
        #[arg(long)]
        tfa: Option<String>,
    },
    /// Delete an authentication realm
    Delete {
        /// Realm name
        realm: String,
    },
    /// Sync a realm (LDAP/AD)
    Sync {
        /// Realm name
        realm: String,
    },
}
