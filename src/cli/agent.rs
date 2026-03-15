use clap::Subcommand;

#[derive(Subcommand)]
pub enum AgentCommand {
    /// Ping guest agent
    Ping {
        /// VM ID
        vmid: u32,
    },
    /// Get guest OS info
    Info {
        /// VM ID
        vmid: u32,
    },
    /// Get guest network interfaces
    Network {
        /// VM ID
        vmid: u32,
    },
    /// Execute a command in the guest
    Exec {
        /// VM ID
        vmid: u32,
        /// Command to run
        #[arg(long)]
        command: String,
        /// Stdin data
        #[arg(long)]
        input_data: Option<String>,
    },
    /// Get status of a previous exec command
    #[command(name = "exec-status")]
    ExecStatus {
        /// VM ID
        vmid: u32,
        /// Process ID returned by exec
        #[arg(long)]
        pid: u32,
    },
    /// Read a file from the guest
    #[command(name = "file-read")]
    FileRead {
        /// VM ID
        vmid: u32,
        /// File path inside the guest
        #[arg(long)]
        file: String,
    },
    /// Write content to a file in the guest
    #[command(name = "file-write")]
    FileWrite {
        /// VM ID
        vmid: u32,
        /// File path inside the guest
        #[arg(long)]
        file: String,
        /// Content to write
        #[arg(long)]
        content: String,
    },
    /// Set user password in the guest
    #[command(name = "set-password")]
    SetPassword {
        /// VM ID
        vmid: u32,
        /// Username
        #[arg(long)]
        username: String,
        /// New password
        #[arg(long)]
        password: String,
    },
    /// Shutdown guest via agent
    Shutdown {
        /// VM ID
        vmid: u32,
    },
    /// Freeze guest filesystems for snapshot
    Fsfreeze {
        /// VM ID
        vmid: u32,
    },
    /// Thaw guest filesystems
    Fsthaw {
        /// VM ID
        vmid: u32,
    },
}
