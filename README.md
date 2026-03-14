# prox-cli

CLI Rust pour piloter un serveur Proxmox VE via son API REST. Concu pour gerer un Cyber Range (lab de cybersecurite) de maniere automatisee.

## Fonctionnalites

- **VMs (QEMU)** — list, status, start, stop, shutdown, create, delete, clone, snapshot, rollback
- **Containers (LXC)** — list, status, start, stop, create, delete, pull OCI/Docker, templates, snapshot, rollback
- **Storage** — list storages, ZFS pools, disks, usage
- **Network** — list, create/delete bridges, apply/revert, protection vmbr0
- **Users** — list, create, delete, set-password, ACL permissions, roles
- **Templates** — list, convert VM to template, clone from template
- **Firewall** — regles cluster/VM/CT, IP sets, aliases
- **Backup** — vzdump create/restore, list, delete, scheduled jobs
- **Tasks** — list, status, log, cancel des operations async
- **Node** — status serveur, time, dns, version, services, syslog
- **Pools** — resource pools CRUD, add/remove resources
- **APT** — repos, update, upgrade, versions, changelog
- **Guest Agent** — exec, file-read/write, ping, info, network, set-password, fsfreeze/thaw
- **Disks** — SMART, init-gpt, wipe, LVM/LVMthin CRUD, directory, ZFS detail
- **Groups** — user groups CRUD
- **TFA** — two-factor authentication management (TOTP, U2F, WebAuthn, recovery)
- **Domains** — authentication realms (PAM, PVE, LDAP, AD, OpenID)
- **Node Firewall** — node-level firewall rules, options, log, refs
- **Console** — terminal proxy for VM, CT, node shell
- **Bulk** — start/stop/migrate/suspend all VMs/CTs
- **Hardware** — PCI/USB device listing for passthrough
- **Scan** — scan NFS, CIFS, iSCSI, LVM, ZFS, PBS, GlusterFS targets
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
prox-cli ct templates                    # liste les templates avec le format --ostemplate
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

# Firewall
prox-cli firewall cluster-rules
prox-cli firewall cluster-add --action DROP --type in --source 10.0.1.0/24 --dport 22 --proto tcp
prox-cli firewall vm-rules 300
prox-cli firewall vm-add 300 --action ACCEPT --type in --dport 80 --proto tcp
prox-cli firewall ipset-list
prox-cli firewall alias-list

# Backup
prox-cli backup create --vmid 300 --storage bulk-backup --mode snapshot
prox-cli backup list
prox-cli backup restore --archive bulk-backup:backup/vzdump-qemu-300.vma.zst --vmid 310
prox-cli backup jobs

# Tasks
prox-cli task list
prox-cli task list --errors-only
prox-cli task status UPID:cyber-range:...
prox-cli task log UPID:cyber-range:...

# Node
prox-cli node status
prox-cli node version
prox-cli node dns
prox-cli node services
prox-cli node syslog --limit 100

# Pools
prox-cli pool list
prox-cli pool create --poolid students --comment "Student resources"
prox-cli pool add students --vmid 300,301,302
prox-cli pool show students

# Disks
prox-cli disk smart --disk /dev/sdb
prox-cli disk lvm-list
prox-cli disk lvm-create --name myvg --device /dev/sdb --add-storage
prox-cli disk lvmthin-list
prox-cli disk dir-list
prox-cli disk zfs-detail rpool
prox-cli disk init-gpt --disk /dev/sdb
prox-cli disk wipe --disk /dev/sdb

# Groups
prox-cli group list
prox-cli group create --groupid students --comment "Student group"
prox-cli group show students
prox-cli group delete students

# TFA
prox-cli tfa list
prox-cli tfa user-list root@pam
prox-cli tfa add --userid user1@pve --type totp --description "Phone"
prox-cli tfa delete --userid user1@pve --id tfa-id

# Domains
prox-cli domain list
prox-cli domain show pam
prox-cli domain create --realm myldap --type ldap --server1 ldap.example.com --base-dn "dc=example,dc=com"
prox-cli domain sync myldap
prox-cli domain delete myldap

# Node Firewall
prox-cli node-firewall list
prox-cli node-firewall add --action DROP --type in --source 10.0.1.0/24 --dport 22 --proto tcp
prox-cli node-firewall options
prox-cli node-firewall set-options --enable true --policy-in DROP
prox-cli node-firewall log --limit 50
prox-cli node-firewall refs

# Console
prox-cli console vm 300
prox-cli console ct 400
prox-cli console node

# Bulk
prox-cli bulk start-all
prox-cli bulk stop-all --vms 300,301,302
prox-cli bulk suspend-all

# Hardware
prox-cli hardware pci-list
prox-cli hardware pci-show 0000:01:00.0
prox-cli hardware usb-list

# Scan
prox-cli scan nfs --server 192.168.1.1
prox-cli scan cifs --server 192.168.1.1 --username admin --password secret
prox-cli scan iscsi --portal 192.168.1.1
prox-cli scan lvm
prox-cli scan zfs
prox-cli scan pbs --server pbs.local --username root@pam --password secret
```

### Flags globaux

| Flag | Description |
|------|-------------|
| `--json` | Sortie JSON (pour scripting, pipes avec jq) |
| `-y` / `--yes` | Skip les confirmations interactives |
| `--config <path>` | Chemin vers le fichier de config |

## Protections

- `vmbr0` et `lo` ne peuvent pas etre modifies/supprimes (interface management)
- Confirmation interactive sur toutes les actions destructrices (delete, rollback, restore)
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
    ├── ct.rs           # Gestion Containers LXC + OCI pull + templates
    ├── storage.rs      # Info stockage
    ├── network.rs      # Gestion reseau
    ├── user.rs         # Gestion utilisateurs + ACL
    ├── template.rs     # Gestion templates VM
    ├── firewall.rs     # Regles firewall cluster/VM/CT + IP sets + aliases
    ├── backup.rs       # Vzdump backup/restore + jobs schedules
    ├── task.rs         # Suivi des taches async
    ├── node.rs         # Info et diagnostics serveur
    ├── pool.rs         # Pools de ressources
    ├── apt.rs          # Gestion paquets APT
    ├── agent.rs        # Guest agent (exec, fichiers, etc.)
    ├── disk.rs         # Disks avances (SMART, LVM, wipe, GPT)
    ├── group.rs        # Groupes utilisateurs
    ├── tfa.rs          # Authentification 2FA
    ├── domain.rs       # Realms d'authentification
    ├── node_firewall.rs # Firewall node-level
    ├── console.rs      # Terminal proxy
    ├── bulk.rs         # Actions en masse
    ├── hardware.rs     # PCI/USB passthrough
    └── scan.rs         # Scan storage targets
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
