# Primearc Core Node Sidecar (Linux GNU) — Packaging + Integration Checklist

This document is the “Bitcoin Core-style desktop” plan for bundling the **Primearc Core node** together with the **ËTRID Console GUI** using **Tauri sidecar binaries**.

It is written against the current repo state (static Next.js export + Tauri v2).

## Current repo state (what exists today)

- The desktop app bundles the GUI static export:
  - Next.js exports to `out/` (`next.config.js`)
  - Tauri loads `out/` (`src-tauri/tauri.conf.json` → `build.frontendDist: "../out"`)
- Rust “node” commands exist but are **stubs** (no real process spawning):
  - `src-tauri/src/lib.rs` has `start_node`, `stop_node`, `restart_node` that only toggle an in-memory flag.
- Frontend currently does **not** call Tauri commands:
  - No `@tauri-apps/api` dependency in `package.json`
  - UI “Terminal” is implemented client-side in React and does not `invoke()` Rust.

## Goal

For Linux builds, ship:

- `etrid-console` (GUI) + `out/` assets
- `etrid-primearc-core` (Primearc Core node binary) as a **sidecar**

So the GUI can:

- Start/stop the node
- Stream logs
- Connect to the node over localhost Substrate WS (e.g. `ws://127.0.0.1:9944`)

## 1) Node binary deliverable (Linux GNU)

You will need a Primearc Core node binary built for:

- `x86_64-unknown-linux-gnu` (and optionally `aarch64-unknown-linux-gnu`)

Operational requirements for “embedded” mode:

- Must support setting a data directory (chain DB) via CLI flag (recommended) so it can live in user data dir, not inside the application bundle.
- Must support configuring RPC ports / WS endpoint / CORS / unsafe RPC if required for local GUI control.
- Must run without interactive prompts.

## 2) Where to place the sidecar in this repo

Tauri looks for sidecar binaries relative to `src-tauri/`.

Recommended layout:

```
src-tauri/
  bin/
    etrid-primearc-core-x86_64-unknown-linux-gnu
    # (optional later)
    # etrid-primearc-core-aarch64-unknown-linux-gnu
```

Notes:
- Don’t rename the file to include `.exe` on Linux.
- Keep it executable (`chmod +x`).
- If the binary is too large for git, store it as a CI artifact and copy into `src-tauri/bin/` during the release pipeline.

## 3) Tauri bundling configuration (`externalBin`)

Tauri sidecar bundling is configured via `bundle.externalBin` in `src-tauri/tauri.conf.json`.

Tauri expects platform-specific files following:

`binary-name{-target-triple}{.system-extension}`

For Linux GNU, the naming includes `-x86_64-unknown-linux-gnu`.

Example config (to be added in a future session):

```json
{
  "bundle": {
    "active": true,
    "externalBin": ["bin/etrid-primearc-core"]
  }
}
```

With that, Linux builds will bundle:

- `src-tauri/bin/etrid-primearc-core-x86_64-unknown-linux-gnu`

## 4) Spawning the sidecar (Rust backend)

Today, `start_node` / `stop_node` are stubs (`src-tauri/src/lib.rs`). To actually run the node:

### Add a process handle to state

Extend the existing `AppState` (in `src-tauri/src/lib.rs`) to store:

- `node_child: Option<...>` (a child process handle)
- `node_logs` (ring buffer) or forward logs to the existing logging plugin and optionally tail a log file

### Use a sidecar-spawn mechanism

Recommended for Tauri v2:

- Add `tauri-plugin-shell` (Rust) and spawn with “sidecar” APIs so the binary path resolves correctly inside bundled apps.

Alternative:

- Use `std::process::Command` and resolve the binary path manually (less ideal inside a bundled app).

### Implement `start_node` / `stop_node` / `restart_node`

Minimum behavior:

- `start_node`: if already running, return “already running”; else spawn sidecar.
- `stop_node`: send SIGTERM and wait; then SIGKILL if needed; clear handle.
- `restart_node`: stop then start.
- `get_node_status`: return “running”, pid, ports, and last error if any.

### Data directory & ports

Store chain data under the OS app data directory (not in bundle):

- Linux example: `~/.local/share/etrid-console/primearc-core/`

Ports should be configurable, but default to typical Substrate values (e.g. 9944).

## 5) Tauri capability permissions (required for safe spawning)

This repo uses capabilities in `src-tauri/capabilities/default.json`.

To allow spawning a node sidecar, you will need to:

1. Add the shell plugin (future session).
2. Add the corresponding shell permissions into the capability file (permissions are namespaced like `shell:...`).

See `src-tauri/gen/schemas/desktop-schema.json` for how plugin permissions are referenced (e.g. `shell:allow-open` is shown as an example in the schema).

## 6) Frontend wiring (GUI controls)

Right now the GUI does not invoke Rust commands.

To control the node from the GUI:

1. Add `@tauri-apps/api` (frontend) to call `invoke()`.
2. Create a small wrapper (e.g. `lib/tauri/node.ts`) for:
   - `start_node`
   - `stop_node`
   - `restart_node`
   - `get_node_status`
   - (optional) `get_node_logs`
3. Add UI controls in the “Network / Peers” area to:
   - start/stop/restart
   - show current status + endpoint (local vs remote)
   - show logs

The existing “Network” tab is the right home for this (it already contains health + peers).

## 7) Release packaging model (3 builds)

You’re correct: ship separate platform builds like most desktop apps.

With a Linux-only node initially:

- **Linux build:** GUI + bundled node sidecar (full experience)
- **macOS/Windows builds:** GUI-only until native node binaries exist; GUI connects to remote nodes via configured WS endpoints

When macOS/Windows node binaries exist:
- Bundle those sidecars using the same `externalBin` base name with the appropriate target-triple filenames.

## “Done when” checklist

- [ ] Primearc Core node binary builds for `x86_64-unknown-linux-gnu`
- [ ] `src-tauri/bin/etrid-primearc-core-x86_64-unknown-linux-gnu` exists (or is added in pipeline)
- [ ] `src-tauri/tauri.conf.json` updated to include `bundle.externalBin`
- [ ] Rust backend spawns/stops the node (replaces current stubs in `src-tauri/src/lib.rs`)
- [ ] Capability permissions updated to allow sidecar spawn
- [ ] Frontend adds node controls + status UI and uses `invoke()`
- [ ] Linux build produces AppImage/deb/rpm containing the sidecar

