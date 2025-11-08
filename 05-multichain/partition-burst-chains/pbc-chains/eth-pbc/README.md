# ETH-PBC Isolated Workspace

**Purpose:** Separate Cargo workspace for eth-pbc-collator to resolve Polkadot SDK version conflicts with Frontier EVM.

## Why This Workspace Exists

The eth-pbc-collator requires:
- **Frontier EVM** (stable2506) - Latest available Frontier release
- **Ëtrid ASF Consensus** - Core consensus mechanism (uses stable2509 in main workspace)

This creates a version conflict that cannot be resolved in the main workspace. This isolated workspace uses **polkadot-stable2506** exclusively to maintain compatibility with Frontier.

## Important Notes

### ⚠️ This is NOT a Chain Fork

This workspace is **purely for compilation**. The resulting binary:
- ✅ Connects to the same FlareChain relay
- ✅ Uses the same ASF consensus algorithm
- ✅ Participates in the same multichain network
- ✅ Submits state roots to FlareChain like all other PBCs
- ✅ Uses the same genesis and validators

**The blockchain doesn't know the difference!**

### 🔧 Build Instructions

```bash
# From this directory
cargo build --release -p eth-pbc-collator

# Binary location
target/release/eth-pbc-collator
```

### 📦 Workspace Contents

```
eth-pbc-workspace/
├── Cargo.toml              # Workspace root (stable2506)
├── eth-pbc-runtime/        # ETH PBC runtime with Frontier EVM
├── eth-pbc-collator/       # ETH PBC collator node
├── consensus/              # ASF consensus modules (copied)
│   ├── primitives/consensus-asf/
│   ├── client/consensus-asf/
│   ├── pallet/
│   ├── asf-algorithm/
│   └── block-production/
├── 04-accounts/pallet/     # Accounts pallet dependency
└── pallets/pallet-etr-lock/  # ETR token lock pallet
```

### 🔄 Keeping in Sync

When updating ASF consensus or dependencies:
1. Update in main workspace: `/Users/macbook/Desktop/etrid/`
2. Copy changes to: `eth-pbc-workspace/consensus/`
3. Rebuild: `cargo build --release`

### 🚀 Deployment

The eth-pbc-collator binary deploys identically to other PBC collators:
- Same validator infrastructure (validators 6-21)
- Same session keys
- Same chainspec format
- Same network configuration

## Version Info

- **Polkadot SDK:** stable2506
- **Frontier:** frontier-stable2506
- **ASF Consensus:** v0.1.0 (Ëtrid custom)
- **EVM:** 0.41
- **Ethereum:** 0.18 (EIP-7702 support)
