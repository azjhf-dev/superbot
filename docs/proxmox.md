# Deploying on a home Proxmox host

Run `botd` on your own Proxmox machine instead of a rented VPS. Same architecture: one shared Linux computer for all the Bots, just hardware you own.

## 1. Create the container or VM

A Debian 13 LXC is enough; a VM works too.

- LXC: unprivileged, with nesting enabled (the harness CLIs and Bot screens need it).
- Sizing, close to the machine Grok Bot described for itself: 4-8 vCPU, 8-16 GB RAM, 60-128 GB disk. No GPU needed; the models run in the cloud.
- Give it a static DHCP lease or fixed IP on your LAN. Keep that address out of this repo.

## 2. Install the stack

Inside the container/VM, as a normal user (not root - Claude Code refuses to skip permissions as root):

```sh
# Rust, for botd
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js ^22.19 or >=24, for pi and dsh
# (your distro's package or nvm)

# Harness CLIs you want
npm i -g @earendil-works/pi-coding-agent   # pi
npm i -g @deepseek-ai/dsh                  # dsh (DeepSeek Harness)
# claude and codex per their own install docs
```

Log in to each harness as this same user:

- `claude login` or `ANTHROPIC_API_KEY`
- `codex login` or `OPENAI_API_KEY`
- `pi`: provider key, e.g. `OPENROUTER_API_KEY`
- `dsh`: the headless profile reads `DEEPSEEK_API_KEY` from the launch env or `.env`; the default route is DeepSeek V4.1 Flash. GUI sign-ins saved by `dsh web` are ignored by the headless profile, so use the key.

## 3. Build and run botd

```sh
git clone <this-repo> && cd superbot
cargo build --release
cp .env.example .env   # fill it in; see below
set -a; . ./.env; set +a
./target/release/botd
```

In `.env` on a home LAN:

- `HOST=0.0.0.0` and a strong `AUTH_PASSWORD` so you can open the app from other devices on the LAN.
- `TZ` to your time zone; routines run on it.
- `DEEPSEEK_API_KEY=...` if you use dsh Bots.

Run `botd` under systemd with `Restart=always`; it shuts down cleanly on SIGTERM. A minimal unit:

```ini
[Unit]
Description=SuperBot
After=network-online.target

[Service]
Type=simple
User=botd
WorkingDirectory=/home/botd/superbot
EnvironmentFile=/home/botd/superbot/.env
ExecStart=/home/botd/superbot/target/release/botd
Restart=always

[Install]
WantedBy=multi-user.target
```

## 4. Reach it from your other devices

- LAN only: open `http://<container-ip>:3000`, log in with any username and your `AUTH_PASSWORD`.
- Away from home: Tailscale on the container is the simple option, or put Caddy in front with HTTPS if you expose it. Proxies must not buffer `/api/events` (it's an SSE stream).

## 5. Backups

`DATA_DIR` holds the database, Bot memory, and harness sessions - that's the state that matters. Snapshot the container with Proxmox Backup Server or ZFS snapshots. `WORKSPACE_DIR` is the Bots' scratch space.

## Notes

- No GPU passthrough needed: every harness calls hosted models.
- One container can run several `botd` instances on different ports and `SCREEN_BASE` values, e.g. one per family member.
- If the box also runs other services, give the Bot container its own user and keep `AUTH_PASSWORD` set - the Bots run with permission prompts off, so the server is the computer.
