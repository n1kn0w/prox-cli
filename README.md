<h1 align="center">
  <br>
  prox-cli
  <br>
</h1>

<p align="center">
  <b>A fast, feature-rich CLI for managing Proxmox VE servers from your terminal.</b>
</p>

<p align="center">
  <a href="#installation">Installation</a> &bull;
  <a href="#quick-start">Quick Start</a> &bull;
  <a href="#features">Features</a> &bull;
  <a href="#usage">Usage</a> &bull;
  <a href="#contributing">Contributing</a>
</p>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/rust-stable-orange?logo=rust">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-blue">
  <img alt="Proxmox" src="https://img.shields.io/badge/proxmox-VE%208.x-E57000?logo=proxmox">
</p>

---

**prox-cli** wraps the entire Proxmox VE REST API into a single binary with **150+ commands**, colored output, JSON mode for scripting, and multi-profile support. Built in Rust for speed and reliability.

```bash
# Dashboard overview
prox-cli status

# SSH into a VM (IP resolved via guest agent)
prox-cli ssh 100 --user admin
prox-cli ssh 100 -J user@bastion:2222

# Snapshot everything before a lab exercise
prox-cli snap-all baseline --running-only

# Rollback the whole environment
prox-cli rollback-all baseline
```

## Installation

### From source (recommended)

```bash
git clone https://github.com/n1kn0w/prox-cli.git
cd prox-cli
cargo install --path .
```

### Build only

```bash
cargo build --release
cp target/release/prox-cli ~/.cargo/bin/
```

### Shell completions

```bash
# Zsh
mkdir -p ~/.zfunc
prox-cli completions zsh > ~/.zfunc/_prox-cli
# Add to ~/.zshrc:
#   fpath=(~/.zfunc $fpath)
#   autoload -Uz compinit && compinit

# Bash
prox-cli completions bash > /etc/bash_completion.d/prox-cli

# Fish
prox-cli completions fish > ~/.config/fish/completions/prox-cli.fish
```

## Quick Start

### 1. Create a config file

```toml
# config.toml
[proxmox]
host = "192.168.1.100"
port = 8006
user = "root@pam"
password = "your-password"
node = "pve"
verify_ssl = false
```

### 2. Save it as a profile

```bash
prox-cli conf add homelab config.toml
prox-cli conf use homelab
```

### 3. Start using it

```bash
prox-cli status          # Dashboard overview
prox-cli vm list         # List all VMs
prox-cli ct list         # List all containers
```

### Config resolution order

1. `--config <path>` (explicit override)
2. Active profile (`prox-cli conf use <name>`)
3. `./config.toml` (local fallback)
4. `~/.config/prox-cli/config.toml` (global fallback)

## Features

### Core Management

| Domain | Commands |
|--------|----------|
| **VMs (QEMU)** | list, status, start, stop, shutdown, create, delete, clone, config, set, snapshot, rollback |
| **Containers (LXC)** | list, status, start, stop, create, delete, pull (OCI/Docker), templates, snapshot, rollback |
| **Storage** | list storages, ZFS pools, disks, usage |
| **Network** | list, create/delete bridges, apply/revert config, vmbr0 protection |
| **Users** | list, create, delete, set-password, ACL permissions, roles |
| **Templates** | list, convert VM to template, clone from template |
| **Pools** | create, delete, show, add/remove resources |

### Security & Firewall

| Domain | Commands |
|--------|----------|
| **Firewall** | cluster/VM/CT rules, IP sets, aliases |
| **Node Firewall** | node-level rules, options, log, refs |
| **TFA** | TOTP, U2F, WebAuthn, recovery keys |
| **Domains** | PAM, PVE, LDAP, AD, OpenID realms |
| **Groups** | user group CRUD |

### Operations

| Domain | Commands |
|--------|----------|
| **Dashboard** | `prox-cli status` — CPU/RAM/disk + VM/CT overview |
| **SSH** | `prox-cli ssh <vmid>` — resolve IP via guest agent, exec SSH, proxy/jump host support (`-J`) |
| **Snap-all** | `prox-cli snap-all <name>` — parallel snapshot of all guests |
| **Rollback-all** | `prox-cli rollback-all <name>` — parallel rollback with confirmation |
| **Bulk** | start/stop/migrate/suspend all VMs/CTs |
| **Backup** | vzdump create/restore, list, delete, scheduled jobs |
| **Tasks** | list, status, log, cancel async operations |
| **Guest Agent** | exec, file-read/write, ping, network, set-password, fsfreeze |

### Infrastructure

| Domain | Commands |
|--------|----------|
| **Node** | status, time, dns, version, services, syslog |
| **Syslog** | view syslog/journal, manage rsyslog service, configure remote forwarding via SSH |
| **Disks** | SMART, init-gpt, wipe, LVM/LVMthin CRUD, directories, ZFS detail |
| **Hardware** | PCI/USB device listing for passthrough |
| **Scan** | NFS, CIFS, iSCSI, LVM, ZFS, PBS, GlusterFS target discovery |
| **APT** | repos, update, upgrade, versions, changelog |
| **Console** | terminal proxy for VM, CT, node shell |

### Global Flags

| Flag | Description |
|------|-------------|
| `--json` | JSON output for scripting and piping to `jq` |
| `-y` / `--yes` | Skip interactive confirmations |
| `-v` | Show API requests on stderr |
| `-vv` | Show API requests + full JSON responses |
| `--config <path>` | Override config file path |

## Usage

### Virtual Machines

```bash
prox-cli vm list
prox-cli vm create --vmid 300 --name kali --memory 4096 --cores 4 --iso local:iso/kali.iso
prox-cli vm start 300
prox-cli vm snapshot 300 --name clean-install
prox-cli vm clone 300 --newid 301 --name kali-student1 --full
prox-cli vm stop 300
prox-cli vm delete 300
```

### Containers

```bash
prox-cli ct list
prox-cli ct templates
prox-cli ct pull --reference docker.io/library/alpine:latest
prox-cli ct create --vmid 400 --ostemplate bulk-backup:vztmpl/alpine_latest.tar --hostname test --start
prox-cli ct stop 400
prox-cli ct delete 400
```

### Network & Firewall

```bash
prox-cli network list
prox-cli network create --iface vmbr2 --type bridge --vlan-aware --autostart
prox-cli network apply

prox-cli firewall cluster-rules
prox-cli firewall vm-add 300 --action ACCEPT --type in --dport 80 --proto tcp
prox-cli firewall ipset-list
```

### Users & Access

```bash
prox-cli user list
prox-cli user create --userid student1@pve --password s3cret --firstname John
prox-cli user acl --userid student1@pve --path /vms/300 --role PVEVMUser --propagate
prox-cli user roles
```

### Backup & Restore

```bash
prox-cli backup create --vmid 300 --storage bulk-backup --mode snapshot
prox-cli backup list
prox-cli backup restore --archive bulk-backup:backup/vzdump-qemu-300.vma.zst --vmid 310
```

### Storage Scanning

```bash
prox-cli scan nfs --server 192.168.1.1
prox-cli scan cifs --server 192.168.1.1 --username admin --password secret
prox-cli scan iscsi --portal 192.168.1.1
prox-cli scan pbs --server pbs.local --username root@pam --password secret
```

### SSH with jump host

```bash
# Direct SSH via guest agent IP
prox-cli ssh 100 --user admin

# SSH through a proxy/jump host
prox-cli ssh 100 -J user@bastion:2222

# Or configure it globally in config.toml
# [ssh]
# proxy = "user@bastion:2222"
```

### Syslog & Journal

```bash
prox-cli syslog list --limit 100 --service pvedaemon
prox-cli syslog list --since 2024-01-01 --until 2024-01-02
prox-cli syslog journal --lastentries 200
prox-cli syslog service-status
prox-cli syslog service-restart

# Remote syslog forwarding (via SSH to the Proxmox node)
prox-cli syslog config-show
prox-cli syslog config-set --server 10.0.0.5 --port 514 --protocol tcp
prox-cli syslog config-set --server siem.local --protocol udp --facility "auth.*"
prox-cli syslog config-delete
```

### Debugging with verbose mode

```bash
# Show API requests
prox-cli -v vm list

# Show requests + full JSON responses (like curl -v)
prox-cli -vv node status
```

### JSON mode for scripting

```bash
# Get all running VM IDs
prox-cli vm list --json | jq -r '.[] | select(.status=="running") | .vmid'

# Count containers
prox-cli ct list --json | jq length

# Export node status
prox-cli node status --json > node-report.json
```

## Config Profiles

Manage multiple Proxmox environments:

```bash
prox-cli conf add production config-prod.toml
prox-cli conf add lab config-lab.toml
prox-cli conf list        # Shows all profiles (* = active)
prox-cli conf use lab      # Switch to lab environment
prox-cli conf show         # Show current config (passwords masked)
prox-cli conf remove lab
```

## Safety

- `vmbr0` and `lo` interfaces are protected from modification/deletion
- Interactive confirmation on all destructive actions (delete, rollback, restore)
- Skip with `-y` for scripting
- Passwords masked in `prox-cli conf show` and `-vv` output
- Credentials stored in config files outside the repo

## Architecture

```
src/
├── main.rs              # Entry point, error handling, command routing
├── cli/                 # CLI definitions (clap derive)
│   ├── mod.rs           # Cli struct, Commands enum, re-exports
│   ├── vm.rs            # VM subcommands
│   ├── ct.rs            # Container subcommands
│   ├── firewall.rs      # Firewall subcommands (cluster/VM/CT/ipset/alias)
│   └── ...              # 20 more domain-specific modules
├── config.rs            # TOML config loading + profile management
├── api.rs               # Proxmox API client (auth, HTTP, verbose logging)
├── output.rs            # Colored tables, JSON output, confirmations
└── commands/
    ├── vm.rs            # VM operations
    ├── ct.rs            # Container operations
    ├── status.rs        # Dashboard (parallel API calls)
    ├── ssh.rs           # SSH via guest agent IP resolution
    ├── snap_all.rs      # Parallel snapshot/rollback all guests
    └── ...              # 20 more command handlers
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI parsing + shell completions |
| `reqwest` | HTTP client (TLS, JSON) |
| `tokio` | Async runtime |
| `serde` / `serde_json` | Serialization |
| `colored` | Terminal colors (auto-disabled in pipes) |
| `futures` | Parallel async operations |
| `toml` | Config file parsing |
| `anyhow` | Error handling |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT](LICENSE)
