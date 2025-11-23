# Ëtrid Primearc Core - Validator Secrets

**⚠️ CONFIDENTIAL - DO NOT COMMIT TO GIT**

---

## Overview

This folder contains sensitive validator information for the Ëtrid Primearc Core Mainnet .

### Files

- **VALIDATORS_MAINNET_REAL.json** - Complete validator registry with real chain data
  - 21 validators (9 Directors + 12 Validity Nodes)
  - Public IPs, Tailscale IPs, session keys
  - Real accounts from genesis (hardcoded in chain)

---

## 9 Decentralized Directors

The first 9 validators are **Decentralized Directors** with governance powers:

| ID | Name | Role | IP | Tailscale |
|----|------|------|-----|-----------|
| 0 | **Eoj** | Founder & CEO | TBD | `eoj-validator` |
| 1 | **Gizzi** | CTO & Infrastructure | 64.181.215.19 | `gizzi-io-validator` (100.96.84.69) |
| 2 | **Audit Dev** | Security Auditing (AI Dev) | TBD | `oracle-vm-audit` |
| 3 | **Security Dev** | Infrastructure Security | TBD | `oracle-vm-security` |
| 4 | **Governance Dev** | DAO Governance (AI Dev) | TBD | `oracle-vm-governance` |
| 5 | **Oracle Dev** | Oracle & Price Feeds (AI Dev) | TBD | `oracle-vm-oracle` |
| 6 | **Consensus Dev** | ASF Consensus (AI Dev) | TBD | `oracle-vm-consensus` |
| 7 | **Economics Dev** | Tokenomics (AI Dev) | TBD | `oracle-vm-economics` |
| 8 | **Compiler Dev** | Runtime & Compiler (AI Dev) | TBD | `oracle-vm-compiler` |

**Quorum**: 6 out of 9 directors required for emergency actions

---

## 12 Validity Nodes

Validators 9-20 are **regular Validity Nodes** (consensus only, no governance):

| ID | Public IP | Tailscale VM |
|----|-----------|--------------|
| 9  | 80.190.82.186 | TBD |
| 10 | 85.239.239.194 | TBD |
| 11 | 85.239.239.193 | TBD |
| 12 | 85.239.239.189 | TBD |
| 13 | 85.239.239.188 | TBD |
| 14 | 154.12.250.18 | TBD |
| 15-18 | TBD | vmi* |
| 19 | 129.80.122.34 | TBD |
| 20 | TBD | vmi* |

---

## Tailscale Network

**Gizzi Bootnode**:
- Public IP: `64.181.215.19`
- Tailscale IP: `100.96.84.69`
- Hostname: `gizzi-io-validator`
- Bootnode Peer: `/ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1`

**Available VMs** (from `tailscale status`):
```
vmi2896906 - 100.93.43.18
vmi2896907 - 100.71.127.127
vmi2896908 - 100.68.185.50
vmi2896909 - 100.70.73.10
vmi2896910 - 100.88.104.58
... (20 total VMs)
```

---

## TODO: Complete Mapping

To finish the secrets configuration:

### 1. Extract Session Keys from Chain

```bash
# Query each validator for their ASF keys
ssh validator "curl -s http://localhost:9944 -d '{\"method\":\"author_rotateKeys\"}'"
```

### 2. Map Directors to Tailscale VMs

Determine which `vmi*` corresponds to each AI Dev:
- `oracle-vm-audit` → vmi????
- `oracle-vm-security` → vmi????
- etc.

### 3. Fill TBD IPs

Some validators have TBD for public IP - get from deployment logs or Contabo dashboard.

### 4. Verify All Data

Cross-reference with:
- Genesis preset: `runtime/presets/flarechain_mainnet_v1_pure_asf.json`
- Deployment logs: `deployment-logs/`
- Chain state: Query `validatorCommittee.validators()`

---

## Security Notes

- **Keep this folder private** - Contains sensitive network topology
- **Backup securely** - Store encrypted backups off-site
- **Rotate keys** - Plan annual key rotation on Consensus Day
- **Multi-sig** - Consider multi-sig for Eoj & Gizzi keys
- **Access control** - Only core team should access this data

---

## Quick Reference

**Chain**: Ëtrid Primearc Core Mainnet
**Network ID**: `flarechain_mainnet` (backward compat)
**Consensus**: ASF (Ascending Scale of Finality)
**Quorum**: 15+ signatures out of 21 validators
**Current Status**: 20/21 signing ✅

**Emergency Actions**: Require 6/9 directors
**Director Term**: 365 days (annual Consensus Day election)

---

**Last Updated**: 2025-11-22
**Maintained By**: Eoj, Gizzi
**DO NOT SHARE PUBLICLY**
