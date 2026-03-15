use clap::Subcommand;

#[derive(Subcommand)]
pub enum AptCommand {
    /// List APT repositories
    Repos,
    /// Update package index
    Update,
    /// List available upgrades
    Upgrade,
    /// Show installed package versions
    Versions,
    /// Show changelog for a package
    Changelog {
        /// Package name
        #[arg(long)]
        name: String,
    },
}
