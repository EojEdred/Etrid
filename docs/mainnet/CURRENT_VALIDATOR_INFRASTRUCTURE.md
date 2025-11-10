# Current FlareChain Mainnet Validator Infrastructure

**Last Updated:** November 9, 2025

## Active Validators

### Oracle Cloud Directors (6 total)

**Primary Directors:**
- **Gizzi** (Validator-1)
  - IP: 64.181.215.19
  - Role: Primary bootnode and director
  - SSH: `ubuntu@64.181.215.19` (key: `~/.ssh/gizzi-validator`)
  - Status: ✅ ACTIVE - Block producing

- **AuditDev** (Validator-2)
  - IP: 129.80.122.34
  - Role: Director validator
  - SSH: `ubuntu@129.80.122.34` (key: `~/.ssh/gizzi-validator`)
  - Status: ✅ ACTIVE

**Additional Oracle Directors:**
- Director-1: 157.173.200.86
- Director-2: 157.173.200.84
- Director-3: 157.173.200.81
- Director-4: 157.173.200.80
- Status: ✅ Port 30333 OPEN (validators may or may not be running)

### Contabo Validators (16 total)

**Region 1: vmi2896xxx (Germany)**
- Validator-6: 85.239.239.194
- Validator-7: 85.239.239.193
- Validator-8: 85.239.239.190
- Validator-9: 85.239.239.189
- Validator-10: 85.239.239.188

**Region 2: vmi3xxx (Germany)**
- Validator-11: 80.190.82.186
- Validator-12: 80.190.82.185
- Validator-13: 80.190.82.184
- Validator-14: 80.190.82.183

**Region 3: vmi4xxx (USA)**
- Validator-15: 158.220.83.146
- Validator-16: 158.220.83.66
- Validator-17: 154.12.250.18
- Validator-18: 154.12.250.17
- Validator-19: 154.12.250.15
- Validator-20: 154.12.249.223
- Validator-21: 154.12.249.182

**SSH Access:** `root@<ip>` (key: `~/.ssh/contabo-validators`)
**Status:** ✅ ALL ACTIVE - Syncing mainnet

## Network Configuration

- **Genesis Hash:** `0xca40...4da8`
- **Chain:** FlareChain Mainnet
- **Total Active Validators:** 23 (6 Oracle directors + 16 Contabo + Gizzi bootnode)
- **P2P Port:** 30333
- **RPC Port:** 9944
- **Prometheus:** 9615

## Bootnodes

Primary bootnodes for validator configuration:
```
--bootnodes "/ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1"
--bootnodes "/ip4/85.239.239.194/tcp/30333/p2p/12D3KooWSrYpSQ6SiDR3uduqbiepyfVp8xmaC8mzY6RmU29MEHGv"
```

## Decommissioned Infrastructure

The following **AZURE** VMs have been **DECOMMISSIONED** and should not be referenced:

| Name | IP | Former Role | Decomm Date |
|------|----|-----------||--------------|
| Azure VM1 | 20.69.26.209 | Validator | Nov 2025 |
| Azure VM2 | 20.186.91.207 | Validator | Nov 2025 |
| Azure VM3 | 52.252.142.146 | Validator | Nov 2025 |

**Note:** 129.80.122.34 is NOT Azure - it's **AuditDev on Oracle Cloud** and is ACTIVE.

## Binary Locations

All validators run the same binaries:
- **FlareChain Node:** `/usr/local/bin/flarechain-node`
- **PBC Collators:** `/usr/local/bin/*-pbc-collator` (14 chains)
- **Chainspec:** `/var/lib/etrid/chainspec-mainnet-raw-FIXED.json`

## Session Keys

All validators (6-21) have their session keys installed:
- AURA (sr25519) - Block production
- GRANDPA (ed25519) - Finality
- ASF (sr25519) - Consensus

Keys are stored in: `/var/lib/etrid/chains/flarechain_mainnet/keystore/`

## Monitoring

- **Check Status:** `bash ~/Desktop/etrid/docs/mainnet/check-current-status.sh`
- **Validator Health:** All nodes syncing to block ~#76,200+
- **Peer Count:** 11-20 peers per validator
- **Network Status:** ✅ HEALTHY

## Firewall Configuration

Port 30333 must be open for P2P networking:
- **Gizzi (OCI):** iptables rule allows all IPs (fixed Nov 9, 2025)
- **Contabo:** Firewall open by default

---

**For operational procedures, see:**
- [How to Run a Validator](HOW_TO_RUN_A_VALIDATOR.md)
- [Validator Quickstart](../secrets/validator-keys/docs/VALIDATOR_QUICKSTART.md)
