# Local Dev + Desktop Build (Next.js Static Export + Tauri)

This repo is a **Next.js App Router** UI exported as static files (`output: 'export'`) and embedded into a **Tauri** desktop app.

## Prerequisites

- Node.js + npm
- Rust toolchain (for Tauri)
- Tauri system prerequisites (Xcode tools on macOS)

## Environment Variables

Create `.env.local` from `.env.example`:

```bash
cp .env.example .env.local
```

### Required (wallet connectivity)

- `NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID`  
  Needed for WalletConnect; MetaMask-in-browser can still work without it, but WalletConnect will not.

### Primearc Core (Substrate) connectivity

Used by the console (network tab, peers, balances, etc). These are **Substrate WS** endpoints (`@polkadot/api` compatible).

- `NEXT_PUBLIC_PRIMEARC_WS_ENDPOINTS`  
  Comma-separated failover list (example: `wss://rpc.etrid.org,ws://157.173.200.80:9944`).

Runtime override: onboarding can store a local list in the browser/desktop app (see “Onboarding / Setup”).

### Explorer links

- `NEXT_PUBLIC_EXPLORER_BASE_URL` (default: `https://explorer.etrid.org`)

Used by “Open Explorer / My Account / Block #…” buttons and peer “View” links.

### EVM (MetaMask-style page)

Used by `/evm` (RainbowKit/wagmi). These are **EVM JSON-RPC** endpoints.

- `NEXT_PUBLIC_EVM_RPC_HTTP_URL` (example: `http://127.0.0.1:8545`)
- `NEXT_PUBLIC_EVM_CHAIN_ID` (example: `8888`)
- `NEXT_PUBLIC_EVM_CHAIN_NAME` (example: `Primearc Core (EVM)`)
- `NEXT_PUBLIC_EVM_EXPLORER_URL` (optional)

Legacy (deprecated) env vars `NEXT_PUBLIC_ETH_PBC_*` are still accepted as fallbacks for older configs.

## Onboarding / Setup

On first launch, the app shows onboarding. You can reopen it any time via the **“?” button** in the header.

The onboarding “Network & Explorer Setup” step stores:

- Primearc WS endpoints (localStorage key: `etrid_primearc_ws_endpoints`)
- Explorer base URL (localStorage key: `etrid_explorer_base_url`)

To re-run onboarding:

- Open the in-app Terminal and run `reset`, or
- Clear localStorage key `etrid_onboarding_completed`.

## Run Web (localhost)

```bash
npm run dev
```

Open `http://localhost:3000`.

## Build Static Export (required for desktop)

```bash
npm run build
```

Output goes to `out/` (Tauri loads this directory in production builds).

## Run Desktop (Tauri)

### Dev mode

```bash
npx tauri dev
```

### Release build

```bash
npx tauri build --bundles app
```

This produces a `.app` under:

`src-tauri/target/release/bundle/macos/etrid-console.app`

### If DMG bundling fails

Some environments fail `npx tauri build` at the DMG step (`bundle_dmg.sh`). Use `--bundles app` (above) to still produce a working `.app`.

## Node sidecar (Primearc Core)

If you want a Bitcoin Core-style “GUI + node in one installer”, see:

`docs/development/PRIMEARC_CORE_NODE_SIDECAR_LINUX_GNU.md`

## Known Desktop Gotchas

### `ChunkLoadError` / missing `tauri://localhost/_next/static/chunks/...`

This usually means the app is loading stale chunk references.

Fix:

1. Re-run `npm run build`
2. Re-run `npx tauri build --bundles app`
3. Launch the **newly built** `.app` (don’t reuse an older installed copy).

### `SyntaxError: The only valid numeric escape in strict mode is '\\0'`

This was caused by production minification output not being compatible with some WebKit environments. The project keeps Next.js minification disabled in `next.config.js` to avoid this.
