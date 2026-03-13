# prox-cli

CLI Rust pour piloter un serveur Proxmox VE via son API REST.

## Fonctionnalites

- **VMs (QEMU)** — list, status, start, stop, shutdown, create, delete, clone, snapshot, rollback
- **Containers (LXC)** — list, status, start, stop, create, delete, pull OCI/Docker, snapshot, rollback
- **Storage** — list storages, ZFS pools, disks, usage
- **Network** — list, create/delete bridges, apply/revert, protection vmbr0
- **Users** — list, create, delete, set-password, ACL permissions, roles
- **Templates** — list, convert VM to template, clone from template
- **Shell completions** — bash, zsh, fish, powershell

## Installation

```bash
cargo build --release
cp target/release/prox-cli ~/.cargo/bin/
```

### Autocompletion (zsh)

```bash
mkdir -p ~/.zfunc
prox-cli completions zsh > ~/.zfunc/_prox-cli
# Ajouter dans ~/.zshrc :
# fpath=(~/.zfunc $fpath)
# autoload -Uz compinit && compinit
```

## Configuration

Copier `config.example.toml` vers `config.toml` et renseigner les credentials Proxmox :

```toml
[proxmox]
host = "192.168.68.105"
port = 8886
user = "root@pam"
password = "changeme"
node = "cyber-range"
verify_ssl = false
```

Le fichier est recherche dans cet ordre :
1. `./config.toml` (repertoire courant)
2. `~/.config/prox-cli/config.toml`
3. Ou via `--config <path>`

## Utilisation

```bash
# VMs
prox-cli vm list
prox-cli vm create --vmid 300 --name kali --memory 4096 --cores 4 --iso local:iso/kali.iso
prox-cli vm start 300
prox-cli vm snapshot 300 --name clean-install
prox-cli vm clone 300 --newid 301 --name kali-user1 --full

# Containers
prox-cli ct list
prox-cli ct pull --reference docker.io/library/alpine:latest
prox-cli ct create --vmid 400 --ostemplate bulk-backup:vztmpl/alpine_latest.tar --hostname test --start
prox-cli ct stop 400
prox-cli ct delete 400

# Storage
prox-cli storage status
prox-cli storage pools
prox-cli storage disks

# Network
prox-cli network list
prox-cli network create --iface vmbr2 --type bridge --vlan-aware --autostart
prox-cli network apply

# Users
prox-cli user list
prox-cli user create --userid student1@pve --password s3cret --firstname John --lastname Doe
prox-cli user acl --userid student1@pve --path /vms/300 --role PVEVMUser --propagate
prox-cli user roles

# Templates
prox-cli template list
prox-cli template create 300
prox-cli template clone 300 --newid 301 --name new-vm --storage fast-vms
```

### Flags globaux

| Flag | Description |
|------|-------------|
| `--json` | Sortie JSON (pour scripting, pipes avec jq) |
| `-y` / `--yes` | Skip les confirmations interactives |
| `--config <path>` | Chemin vers le fichier de config |

## Protections

- `vmbr0` et `lo` ne peuvent pas etre modifies/supprimes (interface management)
- Confirmation interactive sur toutes les actions destructrices (delete, rollback)
- Credentials dans `config.toml` qui est gitignored

## Architecture

```
src/
├── main.rs             # Entry point + routing
├── cli.rs              # Definitions CLI (clap derive + clap_complete)
├── config.rs           # Chargement config TOML
├── api.rs              # Client Proxmox API (auth ticket/CSRF, HTTP, wait_task)
├── output.rs           # Formatage table + JSON + confirm
└── commands/
    ├── vm.rs           # Gestion VMs QEMU
    ├── ct.rs           # Gestion Containers LXC + OCI pull
    ├── storage.rs      # Info stockage
    ├── network.rs      # Gestion reseau
    ├── user.rs         # Gestion utilisateurs + ACL
    └── template.rs     # Gestion templates
```

## Dependances

- `clap` + `clap_complete` — CLI + autocompletion
- `reqwest` — Client HTTP (TLS, JSON)
- `tokio` — Runtime async
- `serde` + `serde_json` — Serialisation
- `toml` — Parsing config
- `anyhow` — Gestion d'erreurs

## Licence

MIT
