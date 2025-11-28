# 📱 Quick Commands for Termius

## Check Single Node (Fast)
```bash
check-mainnet
```
Shows: Gizzi validator details + system health

## Check All 22 Nodes (Complete)
```bash
check-all
```
Shows:
- ✅ All 22 VMs status table (ALL RUNNING AS VALIDATORS)
- ✅ Quorum analysis (15/21 BFT requirement - from genesis)
- ✅ Network peering topology (15-21 peers per validator)
- ✅ Validator set & session info
- ✅ ASF-BFT consensus details
- ✅ Finalization lag for entire network

**Note**: 22 validators running, 21 configured in genesis

## What You'll See

### All Nodes Summary:
```
📡 ACTIVE NODES (22/22 VMs)

NAME                 IP                 BLOCK      FINALIZED  LAG   PEERS  STATUS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
gizzi-validator      100.96.84.69       19,717     19,711     6     8      SYNCED
auditdev             100.70.242.106     19,717     19,711     6     16     SYNCED
vmi2896906           100.93.43.18       19,717     19,711     6     18     SYNCED
... (all 22 nodes)
```

### Quorum Analysis:
```
⚡ ASF-BFT CONSENSUS & FINALITY

Consensus Mechanism: ASF-BFT (Asynchronous Stochastic Finality)
Block Production: PPFA (Pragmatic Probabilistic Finality Algorithm)

Network Best Block: 19,717
Network Finalized Block: 19,711
Finalization Lag: 6 blocks ✅ HEALTHY
Estimated Finality Time: ~30s

Quorum Analysis:
Total Validators (Genesis): 21
Active Validators (Connected): 21
Required for Quorum (15/21): 15
✅ Quorum: ACTIVE (21 ≥ 15)
```

### Network Peering:
```
🌐 NETWORK QUORUM & PEERING

Total Connected Peers: 21
Authority/Validator Peers: 21
Full Node Peers: 0
Light Client Peers: 0

Peer Block Heights:
  • AUTHORITY: Block 19710
  • AUTHORITY: Block 19711
  ... (21 validator peers)

✅ Peer consensus: STRONG (variance: 2 blocks)
📊 Network Coverage: 21/21 validators connected (100%)
```

## Quick Reference

| Command | What It Shows | Speed |
|---------|---------------|-------|
| `check-mainnet` | Single node + system metrics | ⚡ Fast (3s) |
| `check-all` | All 22 nodes + quorum + voting | 🔍 Detailed (15s) |

## From Your Phone

1. Open **Termius**
2. Connect to **auditdev**
3. Type: `check-all`
4. Get complete network health!

---

**Installed on:** auditdev (100.70.242.106)
