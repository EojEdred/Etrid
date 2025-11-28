# PBC Key Management Reference

**Status:** Production Configuration
**Last Updated:** 2025-11-26

---

## Overview

Each validator runs:
- 1x Primearc Core Chain validator (main chain)
- 14x PBC collators (one per supported chain)

Each requires separate cryptographic keys for security isolation.

---

## Key Types

| Key Type | ID | Crypto | Purpose |
|----------|-----|--------|---------|
| ASF Key | `asfk` | sr25519 | Block production (PPFA) + Consensus voting (ASF BFT) |
| Node Key | N/A | ed25519 | libp2p P2P identity |

### Important: One Key, Two Purposes

In ASF BFT, the `asfk` key serves **dual purposes**:

1. **Block Production (PPFA)**: The key is used to sign block proposals during the Primearc Progressive Finality Algorithm
2. **Consensus Voting (ASF)**: The same key signs votes during the Ascending Scale of Finality voting process

This is different from some other consensus systems where block authoring and finality use separate keys (e.g., AURA + GRANDPA in traditional Substrate). In ASF BFT, we use a unified key approach for simplicity and security.

---

## Main Chain Keys (Primearc Core)

### ASF Keys
```
Key Type: asfk
Scheme: sr25519
Derivation: //Genesis{index}
Example: //Genesis0, //Genesis1, ... //Genesis19
```

### Node Keys
```
Formula: genesis_index + 100
Format: 64-character hex, zero-padded
Example: Genesis-0 → 0000000000000000000000000000000000000000000000000000000000000100
```

---

## PBC Collator Keys

### Key Derivation Pattern

Each PBC uses a unique namespace to derive keys:

| PBC | ASF Key Derivation | Node Key Base |
|-----|-------------------|---------------|
| BTC-PBC | `//BTC-PBC/Collator{index}` | index + 1000 |
| ETH-PBC | `//ETH-PBC/Collator{index}` | index + 2000 |
| SOL-PBC | `//SOL-PBC/Collator{index}` | index + 3000 |
| XRP-PBC | `//XRP-PBC/Collator{index}` | index + 4000 |
| BNB-PBC | `//BNB-PBC/Collator{index}` | index + 5000 |
| TRX-PBC | `//TRX-PBC/Collator{index}` | index + 6000 |
| ADA-PBC | `//ADA-PBC/Collator{index}` | index + 7000 |
| DOGE-PBC | `//DOGE-PBC/Collator{index}` | index + 8000 |
| MATIC-PBC | `//MATIC-PBC/Collator{index}` | index + 9000 |
| XLM-PBC | `//XLM-PBC/Collator{index}` | index + 10000 |
| LINK-PBC | `//LINK-PBC/Collator{index}` | index + 11000 |
| SC-USDT-PBC | `//SC-USDT-PBC/Collator{index}` | index + 12000 |
| EDSC-PBC | `//EDSC-PBC/Collator{index}` | index + 13000 |
| AI-COMPUTE-PBC | `//AI-COMPUTE-PBC/Collator{index}` | index + 14000 |

### Insert Key Commands

For each PBC on each validator, run:

```bash
# BTC-PBC Key (example for validator index 0)
/usr/local/bin/btc-pbc-collator key insert \
    --base-path /var/lib/btc-pbc \
    --chain /var/lib/btc-pbc/btc-pbc-spec.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "//BTC-PBC/Collator0"

# ETH-PBC Key
/usr/local/bin/eth-pbc-collator key insert \
    --base-path /var/lib/eth-pbc \
    --chain /var/lib/eth-pbc/eth-pbc-spec.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "//ETH-PBC/Collator0"

# ... repeat for each PBC
```

---

## Port Allocation

Each service on a validator uses different ports:

| Service | P2P Port | RPC Port | Prometheus |
|---------|----------|----------|------------|
| Primearc Core | 30333 | 9944 | 9615 |
| BTC-PBC | 30334 | 9945 | 9616 |
| ETH-PBC | 30335 | 9946 | 9617 |
| SOL-PBC | 30336 | 9947 | 9618 |
| XRP-PBC | 30337 | 9948 | 9619 |
| BNB-PBC | 30338 | 9949 | 9620 |
| TRX-PBC | 30339 | 9950 | 9621 |
| ADA-PBC | 30340 | 9951 | 9622 |
| DOGE-PBC | 30341 | 9952 | 9623 |
| MATIC-PBC | 30342 | 9953 | 9624 |
| XLM-PBC | 30343 | 9954 | 9625 |
| LINK-PBC | 30344 | 9955 | 9626 |
| SC-USDT-PBC | 30345 | 9956 | 9627 |
| EDSC-PBC | 30346 | 9957 | 9628 |
| AI-COMPUTE-PBC | 30347 | 9958 | 9629 |

---

## Data Directories

Each PBC has its own data directory:

```
/var/lib/etrid/           # Main chain
/var/lib/btc-pbc/         # BTC PBC
/var/lib/eth-pbc/         # ETH PBC
/var/lib/sol-pbc/         # SOL PBC
/var/lib/xrp-pbc/         # XRP PBC
/var/lib/bnb-pbc/         # BNB PBC
/var/lib/trx-pbc/         # TRX PBC
/var/lib/ada-pbc/         # ADA PBC
/var/lib/doge-pbc/        # DOGE PBC
/var/lib/matic-pbc/       # MATIC PBC
/var/lib/xlm-pbc/         # XLM PBC
/var/lib/link-pbc/        # LINK PBC
/var/lib/sc-usdt-pbc/     # SC-USDT PBC
/var/lib/edsc-pbc/        # EDSC PBC
/var/lib/ai-compute-pbc/  # AI-COMPUTE PBC
```

---

## Chainspec Files

Each PBC needs its own chainspec:

| PBC | Chainspec Location |
|-----|-------------------|
| BTC-PBC | `/var/lib/btc-pbc/btc-pbc-spec.json` |
| ETH-PBC | `/var/lib/eth-pbc/eth-pbc-spec.json` |
| SOL-PBC | `/var/lib/sol-pbc/sol-pbc-spec.json` |
| XRP-PBC | `/var/lib/xrp-pbc/xrp-pbc-spec.json` |
| BNB-PBC | `/var/lib/bnb-pbc/bnb-pbc-spec.json` |
| TRX-PBC | `/var/lib/trx-pbc/trx-pbc-spec.json` |
| ADA-PBC | `/var/lib/ada-pbc/ada-pbc-spec.json` |
| DOGE-PBC | `/var/lib/doge-pbc/doge-pbc-spec.json` |
| MATIC-PBC | `/var/lib/matic-pbc/matic-pbc-spec.json` |
| XLM-PBC | `/var/lib/xlm-pbc/xlm-pbc-spec.json` |
| LINK-PBC | `/var/lib/link-pbc/link-pbc-spec.json` |
| SC-USDT-PBC | `/var/lib/sc-usdt-pbc/sc-usdt-pbc-spec.json` |
| EDSC-PBC | `/var/lib/edsc-pbc/edsc-pbc-spec.json` |
| AI-COMPUTE-PBC | `/var/lib/ai-compute-pbc/ai-compute-pbc-spec.json` |

---

## Bootnode Configuration

Each PBC needs its own bootnode. Initial bootnode will be on Gizzi:

| PBC | Bootnode Multiaddr (Gizzi) |
|-----|---------------------------|
| BTC-PBC | `/ip4/64.181.215.19/tcp/30334/p2p/{PEER_ID}` |
| ETH-PBC | `/ip4/64.181.215.19/tcp/30335/p2p/{PEER_ID}` |
| SOL-PBC | `/ip4/64.181.215.19/tcp/30336/p2p/{PEER_ID}` |
| ... | (to be filled after deployment) |

---

## Validator-to-Genesis Mapping (Reference)

From the main chain deployment:

| SSH Alias | Genesis Index | Main Chain Node Key |
|-----------|---------------|---------------------|
| ts-val-17 | 0 | ...0100 |
| ts-val-16 | 1 | ...0101 |
| ts-val-15 | 2 | ...0102 |
| ts-val-14 | 3 | ...0103 |
| ts-val-13 | 4 | ...0104 |
| ts-val-12 | 5 | ...0105 |
| ts-val-11 | 6 | ...0106 |
| ts-val-10 | 7 | ...0107 |
| ts-val-08 | 8 | ...0108 |
| ts-val-09 | 9 | ...0109 |
| ts-val-07 | 10 | ...0110 |
| ts-val-22 | 11 | ...0111 |
| ts-val-21 | 12 | ...0112 |
| ts-val-20 | 13 | ...0113 |
| ts-val-19 | 14 | ...0114 |
| ts-val-18 | 15 | ...0115 |
| ts-val-06 | 16 | ...0116 |
| ts-val-04 | 17 | ...0117 |
| ts-val-05 | 18 | ...0118 |
| ts-val-03 | 19 | ...0119 |

**For PBCs**: Use the same index mapping but with PBC-specific key derivation.

Example: `ts-val-17` (Genesis-0) runs:
- Main chain: `//Genesis0` with node key `...0100`
- BTC-PBC: `//BTC-PBC/Collator0` with node key `...1000`
- ETH-PBC: `//ETH-PBC/Collator0` with node key `...2000`
- etc.

---

## Security Notes

1. **Never share keys between chains** - Each PBC must have unique keys
2. **Backup keystores** - Located in `{data_dir}/chains/{chain_id}/keystore/`
3. **Key derivation is deterministic** - Same seed phrase + derivation path = same key
4. **Node keys are NOT derived from seed** - Generated separately, must be backed up

---

## Systemd Service Names

Each PBC runs as a separate systemd service:

```
etrid-validator.service      # Main chain
btc-pbc-collator.service     # BTC PBC
eth-pbc-collator.service     # ETH PBC
sol-pbc-collator.service     # SOL PBC
xrp-pbc-collator.service     # XRP PBC
bnb-pbc-collator.service     # BNB PBC
trx-pbc-collator.service     # TRX PBC
ada-pbc-collator.service     # ADA PBC
doge-pbc-collator.service    # DOGE PBC
matic-pbc-collator.service   # MATIC PBC
xlm-pbc-collator.service     # XLM PBC
link-pbc-collator.service    # LINK PBC
sc-usdt-pbc-collator.service # SC-USDT PBC
edsc-pbc-collator.service    # EDSC PBC
ai-compute-pbc-collator.service # AI-COMPUTE PBC
```

---

## Deployment Checklist Per Validator

For each validator, ensure:

- [ ] Main chain validator running
- [ ] All 14 PBC binaries installed
- [ ] All 14 PBC chainspecs copied
- [ ] All 14 PBC directories created
- [ ] All 14 ASF keys inserted (unique per PBC)
- [ ] All 14 node keys configured (unique per PBC)
- [ ] All 14 systemd services created
- [ ] All 14 ports open in firewall
- [ ] All services started and healthy
