# prox-cli — Endpoints restants a implementer

Endpoints Proxmox API non couverts qui seraient utiles pour le cyber range.
Mettre a jour ce fichier au fur et a mesure de l'implementation.

## Implemente

- [x] VM (qemu) — list, status, start, stop, shutdown, config, set, create, delete, clone, snapshot, rollback
- [x] CT (lxc) — list, status, start, stop, config, set, create, delete, pull OCI, templates, snapshot, rollback
- [x] Storage — list, pools, disks, status
- [x] Network — list, create, delete, apply, revert
- [x] Users — list, show, create, delete, set-password, acl, roles, acls
- [x] Templates — list, create, clone
- [x] Firewall — cluster/VM/CT rules, IP sets, aliases (24 sous-commandes)
- [x] Backup — create, list, restore, delete, jobs, job-create
- [x] Tasks — list, status, log, cancel
- [x] Node — status, time, dns, version, services, syslog
- [x] Pools — list, show, create, delete, add, remove
- [x] APT — repos, update, upgrade, list-upgrades, versions, changelog
- [x] Guest Agent — exec, file-read, file-write, ping, info, network, fsfreeze/thaw, set-password, shutdown
- [x] Disks avance — smart, init-gpt, wipe, lvm-list/create/delete, lvmthin-list/create/delete, dir-list/create, zfs-detail
- [x] Access/Groups — list, show, create, update, delete
- [x] Access/TFA — list, user-list, add, show, update, delete
- [x] Access/Domains — list, show, create, update, delete, sync

- [x] Node Firewall — list, add, show, update, delete rules, options, set-options, log, refs
- [x] Console — vm, ct, node terminal proxy
- [x] Bulk — start-all, stop-all, migrate-all, suspend-all
- [x] Hardware — pci-list, pci-show, pci-mdev, usb-list
- [x] Scan — nfs, cifs, iscsi, lvm, lvmthin, zfs, pbs, glusterfs

## Pas prioritaire (single node, pas de Ceph)

- [ ] Ceph (47 endpoints) — On utilise ZFS
- [ ] SDN (72 endpoints) — Networking avance, overkill pour notre setup
- [ ] Cluster/Config (10 endpoints) — Single node
- [ ] Cluster/HA (21 endpoints) — Pas de HA en single node
- [ ] Cluster/ACME (15 endpoints) — Certificats SSL auto
- [ ] Cluster/Notifications (31 endpoints) — Alertes email/webhook
- [ ] Cluster/Replication (5 endpoints) — Single node
- [ ] Cluster/Mapping (16 endpoints) — Device passthrough avance
- [ ] Cluster/Metrics (7 endpoints) — Export metriques
- [ ] Cluster/Jobs (7 endpoints) — Realm sync jobs
