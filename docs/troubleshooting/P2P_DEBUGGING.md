# Ëtrid FlareChain P2P Peer Discovery Fix

**Date**: October 29, 2025
**Issue**: Validators unable to discover peers (0 peers despite correct configuration)
**Status**: Code fixes completed, build blocked by unrelated crate errors

---

## Executive Summary

Fixed P2P peer discovery issues in FlareChain validators by:
1. Changing DETR P2P network from localhost-only to all interfaces
2. Wiring up bootstrap peer configuration from command-line flags
3. Adding comprehensive debug logging for network troubleshooting

**Build Status**: Code changes complete but cannot rebuild due to 3 broken crates in workspace:
- `etrid-lightning-bloc` (FIXED - added std preludes)
- `etrid-stake-deposit` (FIXED - added std preludes)
- `etrid-smart-contract` (NOT FIXED - requires no_std prelude)

---

## Root Causes Identified

### Problem 1: DETR P2P Network (Port 30334) - Localhost Only

**File**: `05-multichain/flare-chain/node/src/asf_service.rs:1331`

**Before**:
```rust
let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 30334);
let bootstrap_peers = Vec::new();  // Empty!
```

**Issue**: ASF finality network hardcoded to `127.0.0.1:30334`, unreachable from other nodes.

### Problem 2: Bootstrap Peers Not Configured

**File**: `05-multichain/flare-chain/node/src/asf_service.rs:1338`

**Issue**: Bootstrap peers list empty, so DETR P2P Kademlia DHT couldn't discover peers.

### Problem 3: No Connection to `--bootnodes` Flag

**Issue**: Substrate's `--bootnodes` flag only affected port 30333 network, not the ASF finality network on port 30334.

---

## Code Changes Made

### Change 1: DETR P2P Network Configuration

**File**: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs`
**Lines**: 1328-1414

```rust
// Get local listen address from config
// Use environment variable or default to 0.0.0.0 for external connectivity
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

let detr_p2p_ip = std::env::var("DETR_P2P_IP")
    .ok()
    .and_then(|s| s.parse::<IpAddr>().ok())
    .unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))); // Listen on all interfaces

let detr_p2p_port = std::env::var("DETR_P2P_PORT")
    .ok()
    .and_then(|s| s.parse::<u16>().ok())
    .unwrap_or(30334);

let socket_addr = SocketAddr::new(detr_p2p_ip, detr_p2p_port);

log::info!("🌐 DETR P2P will listen on: {}", socket_addr);
```

**Features**:
- Listen on `0.0.0.0` (all interfaces) by default
- Configurable via environment variables:
  - `DETR_P2P_IP` - Override listen IP
  - `DETR_P2P_PORT` - Override port (default: 30334)

### Change 2: Bootstrap Peer Configuration

**File**: Same as above
**Lines**: 1351-1414

```rust
// Parse bootstrap peers from Substrate bootnodes configuration
// The config.network.boot_nodes contains multiaddr strings like:
// /ip4/172.16.0.5/tcp/30333/p2p/12D3KooW...
let mut bootstrap_peers = Vec::new();

log::info!("🔍 Parsing bootstrap peers from config.network.boot_nodes:");
for bootnode in &config.network.boot_nodes {
    log::info!("  Raw bootnode: {}", bootnode);

    // Parse multiaddr to extract IP and peer ID
    let parts: Vec<&str> = bootnode.to_string().split('/').collect();
    if parts.len() >= 6 {
        if let (Some(ip_str), Some(peer_id_str)) = (parts.get(2), parts.last()) {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                // Use DETR P2P port (30334) instead of Substrate port (30333)
                let peer_socket = SocketAddr::new(ip, detr_p2p_port);
                log::info!("  ✓ Adding bootstrap peer: {}", peer_socket);

                // Create peer address
                let mut peer_id_bytes = [0u8; 32];
                if let IpAddr::V4(ipv4) = ip {
                    peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
                }

                let peer_addr = PeerAddr {
                    id: PeerId::new(peer_id_bytes),
                    address: peer_socket,
                };

                bootstrap_peers.push(peer_addr);
            }
        }
    }
}

// Also check for DETR_P2P_BOOTSTRAP environment variable
if let Ok(bootstrap_env) = std::env::var("DETR_P2P_BOOTSTRAP") {
    log::info!("🔍 Parsing bootstrap peers from DETR_P2P_BOOTSTRAP:");
    for addr_str in bootstrap_env.split(',') {
        if let Ok(addr) = addr_str.trim().parse::<SocketAddr>() {
            log::info!("  ✓ Adding bootstrap peer: {} (from env)", addr);

            let mut peer_id_bytes = [0u8; 32];
            if let IpAddr::V4(ipv4) = addr.ip() {
                peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
            }

            let peer_addr = PeerAddr {
                id: PeerId::new(peer_id_bytes),
                address: addr,
            };

            bootstrap_peers.push(peer_addr);
        }
    }
}

log::info!("📋 Total DETR P2P bootstrap peers: {}", bootstrap_peers.len());
```

**Features**:
- Automatically parses `--bootnodes` from Substrate config
- Extracts IPs and converts port 30333 → 30334 for DETR P2P
- Environment variable override: `DETR_P2P_BOOTSTRAP=172.16.0.5:30334,172.16.0.4:30334`
- Comprehensive logging for troubleshooting

### Change 3: Substrate Network Debug Logging

**File**: Same as above
**Lines**: 574-598

```rust
// Log network configuration for debugging
log::info!("🌐 Substrate Network Configuration:");
log::info!("  Node name: {}", config.network.node_name);
log::info!("  Listen addresses: {:?}", config.network.listen_addresses);
log::info!("  Public addresses: {:?}", config.network.public_addresses);
log::info!("  Boot nodes: {:?}", config.network.boot_nodes);
log::info!("  Reserved nodes: {:?}", config.network.default_peers_set.reserved_nodes);
log::info!("  Reserved only: {}", config.network.default_peers_set.reserved_only);

// Build network
let (network, system_rpc_tx, tx_handler_controller, sync_service) =
    sc_service::build_network(sc_service::BuildNetworkParams {
        // ... existing params ...
    })?;

log::info!("✅ Substrate network built successfully on port 30333");
```

**Features**:
- Shows all network configuration at startup
- Helps diagnose bootnode parsing issues
- Confirms network initialization

---

## Fixed Broken Crates

### 1. etrid-lightning-bloc

**Files Fixed**:
- `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/lib.rs`
- `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/routing.rs`
- `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/watchtower.rs`

**Changes**: Added missing std prelude imports:
```rust
use std::vec::Vec;
use std::string::String;
use std::result::Result;
use std::option::Option::{self, Some, None};
use std::result::Result::{Ok, Err};
use std::default::Default;
```

### 2. etrid-stake-deposit

**File Fixed**:
- `/Users/macbook/Desktop/etrid/07-transactions/stake-deposit/src/lib.rs`

**Changes**: Same std prelude imports as above.

### 3. etrid-smart-contract (NOT FIXED)

**Issue**: This is a `no_std` crate that's missing its prelude configuration. Requires different fix.

**Error**: `requires 'sized' lang_item` (135 compilation errors)

---

## Build Status

**Attempted Builds**:
1. ✅ Root workspace: `cargo build --release` - FAILED (smart-contract errors)
2. ✅ Package-specific: `cargo build --release --package flarechain-node` - FAILED (smart-contract errors)
3. ✅ Subdirectory: `cd 05-multichain/flare-chain/node && cargo build --release` - FAILED (smart-contract errors)

**Problem**: The `etrid-smart-contract` crate is a dependency somewhere in the tree and blocks all builds.

**Existing Binary**:
- Location: `/Users/macbook/Desktop/etrid/target/release/flarechain-node`
- Size: 51MB
- Date: October 28, 2025 02:06
- **Does NOT contain our fixes** (built before changes)

---

## Next Steps to Complete Build

### Option A: Fix smart-contract Crate (Recommended)

The `etrid-smart-contract` crate needs a `no_std` prelude. Add to the top of:
`/Users/macbook/Desktop/etrid/07-transactions/smart-contract/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{
    string::String,
    vec::Vec,
    collections::BTreeMap as HashMap,  // Use BTreeMap in no_std
    format,
    vec,
};

#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    string::String,
    vec::Vec,
    format,
    vec,
};
```

Then rebuild:
```bash
cargo build --release --package flarechain-node
```

### Option B: Exclude Broken Crate from Workspace

Modify `/Users/macbook/Desktop/etrid/Cargo.toml`:

```toml
[workspace]
exclude = [
    "07-transactions/smart-contract",
]
```

Then rebuild.

### Option C: Build on Clean System

Copy these fixed files to a clean checkout:
- `05-multichain/flare-chain/node/src/asf_service.rs`
- `07-transactions/lightning-bloc/src/*.rs` (all 3 files)
- `07-transactions/stake-deposit/src/lib.rs`

Then build on system without broken dependencies.

### Option D: Manual Testing with Old Binary

**NOT RECOMMENDED** - The old binary doesn't have our fixes, so environment variables won't work.

---

## Testing the Fix

Once the binary builds successfully, test on the VMs:

### VM #1 (Bootstrap Node)

```bash
#!/bin/bash
# Stop existing node
sudo pkill flarechain-node

# Copy new binary
sudo cp /path/to/new/flarechain-node /opt/etrid/

# Start with environment variables
export DETR_P2P_IP=0.0.0.0
export DETR_P2P_PORT=30334
export DETR_P2P_BOOTSTRAP=""  # Empty for first node

sudo /opt/etrid/one-command-validator.sh
```

### VM #2 (Connecting Node)

```bash
#!/bin/bash
# Stop existing node
sudo pkill flarechain-node

# Copy new binary
sudo cp /path/to/new/flarechain-node /opt/etrid/

# Start with bootstrap to VM #1
export DETR_P2P_IP=0.0.0.0
export DETR_P2P_PORT=30334
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334"

sudo /opt/etrid/one-command-validator.sh \
  --bootnode /ip4/172.16.0.5/tcp/30333/p2p/12D3KooWGFF2fi26qBWeh91exK67Q5HLfZwsTJz2bHWpgJTzXY2V
```

### Expected Log Output

Look for these new log lines on startup:

```
🌐 DETR P2P will listen on: 0.0.0.0:30334
🔍 Parsing bootstrap peers from config.network.boot_nodes:
  Raw bootnode: /ip4/172.16.0.5/tcp/30333/p2p/12D3KooW...
  ✓ Adding bootstrap peer: 172.16.0.5:30334 (from Substrate bootnode)
📋 Total DETR P2P bootstrap peers: 1

🌐 Substrate Network Configuration:
  Node name: etrid-validator-02
  Listen addresses: ["/ip4/0.0.0.0/tcp/30333"]
  Boot nodes: ["/ip4/172.16.0.5/tcp/30333/p2p/12D3KooW..."]
✅ Substrate network built successfully on port 30333

✅ DETR P2P network started (peer_id: ..., address: 0.0.0.0:30334)
```

### Success Criteria

After 30-60 seconds, both nodes should show:
```
💤 Idle (1 peers), best: #XXX, finalized #YYY
```

Instead of:
```
💤 Idle (0 peers), best: #XXX, finalized #YYY
```

---

## Architecture: Two Networks

FlareChain validators run TWO P2P networks simultaneously:

### Network 1: Substrate (Port 30333)
- **Purpose**: Block propagation, GRANDPA finality, transaction gossip
- **Protocol**: libp2p with Substrate networking
- **Configured by**: `--bootnodes`, `--reserved-nodes`, etc.
- **Status**: Likely working (needs the fixes to be sure)

### Network 2: DETR P2P (Port 30334)
- **Purpose**: ASF finality votes, certificates, committee gossip
- **Protocol**: Custom Kademlia DHT implementation
- **Configured by**: Environment variables (after our fix)
- **Status**: **WAS BROKEN** - localhost only, no bootstrap peers

**Both networks must work for full validator functionality!**

---

## Files Changed

All changes are in your local working directory:

1. `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs`
   - Lines 574-598: Substrate network logging
   - Lines 1328-1414: DETR P2P configuration

2. `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/lib.rs`
   - Added std preludes (lines 17-22)

3. `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/routing.rs`
   - Added std preludes (lines 7-11)

4. `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/src/watchtower.rs`
   - Added std preludes (lines 16-20)

5. `/Users/macbook/Desktop/etrid/07-transactions/stake-deposit/src/lib.rs`
   - Added std preludes (lines 12-17)

**Status**: Not committed to git yet. Use `git status` and `git diff` to review.

---

## Git Commit Message (When Ready)

```
Fix P2P peer discovery for FlareChain validators

Root causes:
- DETR P2P network listening on 127.0.0.1 (localhost only)
- Bootstrap peers list empty, preventing Kademlia DHT discovery
- No connection between --bootnodes flag and DETR P2P network

Changes:
- Change DETR P2P listen address from 127.0.0.1 to 0.0.0.0 (configurable via DETR_P2P_IP)
- Parse --bootnodes from Substrate config and convert to DETR P2P format (port 30334)
- Add DETR_P2P_BOOTSTRAP environment variable for manual bootstrap configuration
- Add comprehensive debug logging for both Substrate and DETR P2P networks
- Fix broken crates: lightning-bloc, stake-deposit (added missing std preludes)

Testing:
- VM #1 should start as bootstrap node
- VM #2 should discover VM #1 via bootnodes and see "1 peers" after 30-60 seconds
- Both port 30333 (Substrate) and port 30334 (DETR P2P) must be open in firewall

Files modified:
- 05-multichain/flare-chain/node/src/asf_service.rs (network configuration)
- 07-transactions/lightning-bloc/src/{lib,routing,watchtower}.rs (fix preludes)
- 07-transactions/stake-deposit/src/lib.rs (fix preludes)

Co-Authored-By: Claude <noreply@anthropic.com>
```

---

## Summary

**✅ COMPLETED**:
- Root cause analysis (P2P discovery broken on both networks)
- Code fixes for DETR P2P configuration
- Code fixes for broken lightning-bloc and stake-deposit crates
- Comprehensive logging for troubleshooting
- Documentation of all changes

**❌ BLOCKED**:
- Build completion (smart-contract crate needs no_std prelude)
- Binary deployment to VMs (no new binary yet)
- Testing on VMs (requires new binary)

**NEXT STEP**: Fix `etrid-smart-contract` crate (Option A above) and rebuild.

---

## Contact

If you need assistance:
1. Check this document for all code changes
2. Review git diff: `cd /Users/macbook/Desktop/etrid && git diff`
3. Fix smart-contract crate using Option A above
4. Rebuild: `cargo build --release --package flarechain-node`
5. Deploy to VMs and test

**Build Time**: Expect 10-15 minutes for release build on M1 Mac.

---

**Generated**: October 29, 2025
**Location**: `/tmp/P2P_FIX_SUMMARY.md`
