# Versioning + Packaging (Console GUI + ËTRID Node)

This project ships in two forms:

- **Web** (static export): `next build` outputs `out/`
- **Desktop** (Tauri): bundles `out/` into an installable app

## 1) Versioning Strategy

### Recommended: one release version for the desktop bundle

Use semantic versioning (SemVer): `MAJOR.MINOR.PATCH` (example: `1.3.2`).

For a desktop release, keep these in sync:

- Frontend version: `package.json` (`package.json:1`)
- Desktop app version: `src-tauri/tauri.conf.json` (`src-tauri/tauri.conf.json:1`)

Why:
- Tauri uses the Tauri version for bundle metadata and updaters.
- The UI should display the same version so support/debugging is consistent.

### Updating the GUI

Because the desktop UI is bundled into the app, **updating the GUI in production normally means shipping a new desktop app build** (new Tauri version).

If you want automatic updates:
- Use the **Tauri updater** so the desktop app can fetch and install a new signed version. (Best practice; avoids loading remote JS at runtime.)

## 2) How the GUI is packaged today

Tauri is configured to load the static export output:

- `src-tauri/tauri.conf.json:1` → `"frontendDist": "../out"`

So a release build effectively does:

1. `npm run build` → generates `out/`
2. `npx tauri build` / `npx tauri build --bundles app` → embeds that `out/` into the desktop app bundle

## 3) Packaging the GUI “with the ËTRID binary” (Bitcoin Core style)

To match the Bitcoin Core model (node + GUI together), ship the node as a **sidecar binary** inside the Tauri app:

### What this gives you

- One installer contains:
  - `etrid-console` (GUI)
  - `etrid` node binary (sidecar)
- The GUI can:
  - Start/stop the node
  - Display logs
  - Connect to the node over localhost Substrate RPC (ex: `ws://127.0.0.1:9944`)

### Implementation approach (recommended)

1. **Add platform-specific node binaries** (per target OS/arch).
2. Configure Tauri bundling to include them as `externalBin` (Tauri “sidecar” binaries).
3. Spawn/monitor the process from Rust using `tauri-plugin-shell` (or Rust `Command` if preferred).
4. Store chain data in the user app data directory (not inside the `.app` bundle).

### Linux-only node note

If the Primearc Core node is only built for Linux, then:

- You can bundle it **only in Linux desktop builds** (AppImage/deb/rpm).
- A macOS/Windows desktop build cannot run a Linux node binary; those platforms would need their own native node build, or the GUI must connect to a remote node.

### `externalBin` naming (important)

Tauri expects binaries to follow the pattern:

`binary-name{-target-triple}{.system-extension}`

Example for a sidecar named `etrid-primearc-core`:

- Linux: `etrid-primearc-core-x86_64-unknown-linux-gnu` (or `...-aarch64-unknown-linux-gnu`)
- macOS: `etrid-primearc-core-x86_64-apple-darwin` / `...-aarch64-apple-darwin`
- Windows: `etrid-primearc-core-x86_64-pc-windows-msvc.exe`

In `src-tauri/tauri.conf.json`, you add the base name once (Tauri picks the correct platform file):

```json
{
  "bundle": {
    "externalBin": ["bin/etrid-primearc-core"]
  }
}
```

### Release model options

- **Simple + safest:** ship node + GUI together, same version, updated together.
- **Split versions:** ship node as sidecar but track a separate “node version” and allow swapping it independently (requires signature verification and careful rollout).

## 4) Operational notes (desktop)

- macOS DMG creation can fail in some environments; `npx tauri build --bundles app` still produces a working `.app`.
- Avoid loading remote JS in the desktop UI; it breaks Tauri’s security model and makes updates harder to trust.
