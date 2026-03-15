use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConsoleCommand {
    /// Create terminal proxy for a VM
    Vm {
        /// VM ID
        vmid: u32,
        /// Serial port number
        #[arg(long)]
        serial: Option<u32>,
    },
    /// Create terminal proxy for a container
    Ct {
        /// Container ID
        vmid: u32,
    },
    /// Create terminal proxy for the node shell
    Node,
}
