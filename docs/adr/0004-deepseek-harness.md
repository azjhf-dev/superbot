# ADR 0004: DeepSeek Harness (dsh) adapter

Status: accepted, 2026-11-17

## Context

We want Bots running on DeepSeek models - specifically DeepSeek V4.1 Flash - alongside the existing Claude Code, Codex, and Pi harnesses. DeepSeek's own open-source agent harness is DeepSeek Harness (`dsh`, github.com/deepseek-ai/deepseek-harness, MIT).

## Decision

Add a `dsh` adapter that runs `dsh --profile headless --json` per task:

- The headless profile fits the Bot model: one task per invocation, no GUI, no listening port, exits when done. The `web` profile is the human UI, not a Bot backend.
- Sessions: dsh assigns new Session ids itself and reports them in the opening `session` NDJSON event; the adapter captures that id on the first run and passes `--session-id` on later runs. An unknown id is an error in dsh, so a lost session record fails loudly.
- Unattended runs: `--permission-mode workspace-write --approval-policy never`. Headless has no live approval channel; calls needing approval are rejected. The shared workspace is the Bot's computer, so workspace-write matches the architecture.
- No TextDelta: dsh `--json` events are committed messages, not token streams, so dsh Bots post text in blocks instead of streaming.
- Model and effort: the headless CLI has no model flag; the route comes from the dsh profile's `agentDefaultModel` (set in `dsh web` or the profile config, default DeepSeek V4.1 Flash). The Bot model picker is informational for dsh.
- Credentials: the headless profile ignores GUI-saved sign-ins and reads `DEEPSEEK_API_KEY` from the launch environment or `.env`.

## Consequences

- dsh is in developer preview and warns of breaking changes; event field names in the adapter must be checked on dsh upgrades.
- `dsh` needs Node.js on the host; everything else in the server stays Rust.
