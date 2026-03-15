use clap::Subcommand;

#[derive(Subcommand)]
pub enum TemplateCommand {
    /// List templates (VMs marked as template)
    List,
    /// Convert a VM to a template
    Create {
        /// VM ID to convert
        vmid: u32,
    },
    /// Clone from a template
    Clone {
        /// Template VM ID
        vmid: u32,
        /// New VM ID
        #[arg(long)]
        newid: u32,
        /// Name for the new VM
        #[arg(long)]
        name: Option<String>,
        /// Target storage
        #[arg(long)]
        storage: Option<String>,
        /// Full clone instead of linked
        #[arg(long)]
        full: bool,
    },
}
