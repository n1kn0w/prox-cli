use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum ConfCommand {
    /// List available profiles (* = active)
    List,
    /// Switch to a profile
    Use {
        /// Profile name
        name: String,
    },
    /// Show active profile details
    Show,
    /// Import a config file as a named profile
    Add {
        /// Profile name
        name: String,
        /// Source config file path
        #[arg(long, default_value = "config.toml")]
        from: PathBuf,
    },
    /// Remove a profile
    Remove {
        /// Profile name
        name: String,
    },
}
