# ADR 0005: Home Proxmox hosting

Status: accepted, 2026-11-17

## Context

The reference deployment in the video and docs is a rented VPS (the video used a Hostinger sponsor). We run our own Proxmox host at home and want the Bots' shared computer on it instead.

## Decision

Document a Proxmox deployment (docs/proxmox.md) as a first-class target:

- One Debian 13 LXC (unprivileged, nesting enabled) or VM runs `botd` and the harness CLIs.
- Sizing tracks the shared-VM architecture we are replicating: 4-8 vCPU, 8-16 GB RAM, no GPU, since every harness calls hosted models.
- LAN-first access (`HOST=0.0.0.0` + `AUTH_PASSWORD`), Tailscale or Caddy/HTTPS for remote access.
- Proxmox snapshots/backup of `DATA_DIR` cover Bot memory and sessions.

## Consequences

- No new code: the server is already a single static binary plus CLIs on one Linux machine.
- Hostnames, IPs, and secrets stay out of the repo (standing rule); the docs use placeholders only.
