# Ëtrid FlareChain - Bootnode Quick Reference

## Current Network Status

**3 Azure Bootstrap Nodes (Running & Synced):**

```
VM1 - EojEdred-Director-02
  IP: 20.69.26.209
  Peer ID: 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
  Bootnode: /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm

VM2 - Governance-Director-03
  IP: 20.186.91.207
  Peer ID: 12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb
  Bootnode: /ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb

VM3 - Security-Director-04
  IP: 52.252.142.146
  Peer ID: 12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
  Bootnode: /ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
```

---

## Complete Bootnode Configuration String

### For Command Line
```bash
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb,/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
```

### For Systemd Service File
```ini
ExecStart=/home/ubuntu/flarechain-node \
  --chain /home/ubuntu/chainspec.json \
  --base-path /home/ubuntu/.etrid/validator \
  --validator \
  --name "YourValidatorName" \
  --port 30333 \
  --rpc-port 9944 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb,/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
```

---

## Quick Update Methods

### Option 1: Automated Script (Recommended)
```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
./update-validators-bootnode.sh
```

This script will:
- Connect to all 16 accessible validators
- Update systemd service files with bootnode configuration
- Restart validators automatically
- Report success/failure for each validator

---

### Option 2: Manual Update (Single Validator)

**For Systemd Service:**
```bash
# SSH to validator
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP

# Edit service file
sudo nano /etc/systemd/system/flarechain-validator.service

# Add this line to ExecStart (before any existing --rpc-port or similar):
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb,/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6 \

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart flarechain-validator

# Verify
sudo systemctl status flarechain-validator
sudo journalctl -u flarechain-validator -f
```

**For Manual Process:**
```bash
# Stop existing process
pkill -f flarechain-node

# Restart with bootnodes
./flarechain-node \
  --chain ~/chainspec.json \
  --base-path ~/.etrid/validator \
  --validator \
  --name "YourValidatorName" \
  --port 30333 \
  --rpc-port 9944 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb,/ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
```

---

## Validators to Update

**From `/Users/macbook/Desktop/etrid/config/validator-ips.json`:**

| # | Name | IP | SSH User | Status |
|---|------|-------|----------|--------|
| 6 | Runtime Dev | 20.224.104.239 | ubuntu | Accessible |
| 7 | Compiler Dev | 98.71.91.84 | ubuntu | Accessible |
| 8 | Network Dev | 20.169.114.25 | ubuntu | Accessible |
| 9 | SDK Dev | 20.75.92.203 | ubuntu | Accessible |
| 10 | DevTools Dev | 20.55.31.30 | ubuntu | Accessible |
| 11 | API Dev | 20.73.34.17 | ubuntu | Accessible |
| 12 | Docs Dev | 20.109.102.30 | ubuntu | Accessible |
| 13 | QA Dev | 52.250.61.132 | ubuntu | Accessible |
| 14 | Perf Dev | 20.218.66.251 | ubuntu | Accessible |
| 15 | Community Dev | 20.109.219.185 | ubuntu | Accessible |
| 16 | Analytics Dev | 20.83.208.17 | ubuntu | Accessible |
| 17 | Ethics Dev | 172.177.175.132 | ubuntu | Accessible |
| 18 | FlareNode 16 | 20.84.231.225 | ubuntu | Accessible |
| 19 | FlareNode 19 | 4.175.83.133 | ubuntu | Accessible |
| 20 | FlareNode 20 | 52.184.47.99 | ubuntu | Accessible |
| 21 | FlareNode 21 | 4.178.181.122 | ubuntu | Accessible |

**Total: 16 validators to update**

---

## Verification Commands

### Check Validator Process
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP "pgrep -a flarechain-node"
```

### Check Peer Connectivity
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP \
  "sudo journalctl -u flarechain-validator --since '5 minutes ago' | grep -i 'peer\|discovered'"
```

### Check All Validators at Once
```bash
for ip in 20.224.104.239 98.71.91.84 20.169.114.25 20.75.92.203 20.55.31.30 20.73.34.17 20.109.102.30 52.250.61.132 20.218.66.251 20.109.219.185 20.83.208.17 172.177.175.132 20.84.231.225 4.175.83.133 52.184.47.99 4.178.181.122; do
  echo "Checking $ip:"
  ssh -i ~/.ssh/gizzi-validator -o ConnectTimeout=5 ubuntu@$ip \
    "pgrep -c flarechain-node 2>/dev/null || echo '0'" | sed 's/^/  Processes: /'
done
```

---

## Expected Results After Update

### In Validator Logs
Look for these success indicators:
```
✓ Discovered new external address: /ip4/20.69.26.209/tcp/30333
✓ Connection established to peer: 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
✓ 💤 Idle (3 peers), best: #25 (0x...), finalized #23 (0x...)
```

### Peer Count Progression
- **Before:** `0 peers` (isolated)
- **After 1 min:** `1 peers` or `2 peers` (connecting to bootnodes)
- **After 5 min:** `3+ peers` (connected to bootnodes + discovered other validators)

### Block Sync
All validators should converge to the same block number:
```bash
# On any validator
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"system_syncState","params":[],"id":1}' \
  http://localhost:9944 | jq
```

---

## Troubleshooting

### Validator Still Shows 0 Peers

**Check firewall rules:**
```bash
# Ensure port 30333 is open
sudo ufw status
sudo ufw allow 30333/tcp
```

**Verify bootnodes in service:**
```bash
sudo systemctl cat flarechain-validator | grep bootnodes
```

**Check network key exists:**
```bash
ls -la ~/.etrid/validator/chains/flarechain_mainnet/network/
```

### Cannot Connect to Validator via SSH

**Test connectivity:**
```bash
ping -c 3 VALIDATOR_IP
telnet VALIDATOR_IP 22
```

**Check SSH key permissions:**
```bash
chmod 600 ~/.ssh/gizzi-validator
```

### Service Won't Start After Update

**Check service status:**
```bash
sudo systemctl status flarechain-validator
sudo journalctl -u flarechain-validator -n 50
```

**Common issues:**
- Binary missing: verify `which flarechain-node`
- Chainspec missing: verify `ls -lh ~/chainspec.json`
- Permission denied: verify paths in service file

---

## Quick Commands Summary

```bash
# Update all validators (automated)
./update-validators-bootnode.sh

# Manual update single validator
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP
sudo nano /etc/systemd/system/flarechain-validator.service
# Add --bootnodes line, then:
sudo systemctl daemon-reload && sudo systemctl restart flarechain-validator

# Verify connectivity
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP \
  "sudo journalctl -u flarechain-validator -f"

# Check all validators
for ip in <IP_LIST>; do
  echo "=== $ip ==="
  ssh -i ~/.ssh/gizzi-validator ubuntu@$ip "pgrep -c flarechain-node"
done
```

---

## Network Architecture

```
┌─────────────────────────────────────────────────┐
│  Azure Bootstrap Nodes (Already Synced)         │
├─────────────────────────────────────────────────┤
│  VM1: 20.69.26.209 (EojEdred) - 2 peers         │
│  VM2: 20.186.91.207 (Governance) - 1 peer       │
│  VM3: 52.252.142.146 (Security) - 1 peer        │
│  Block: #21, Epoch: 3                           │
└────────────┬────────────────────────────────────┘
             │ Bootnode Discovery
             ▼
┌─────────────────────────────────────────────────┐
│  13 Isolated Validators (Need Update)           │
├─────────────────────────────────────────────────┤
│  All currently running with 0 peers             │
│  Producing blocks locally (not synced)          │
│  Need --bootnodes flag to connect               │
└─────────────────────────────────────────────────┘
             │ After Update
             ▼
┌─────────────────────────────────────────────────┐
│  16 Validators Fully Connected                  │
├─────────────────────────────────────────────────┤
│  All synced to same block height                │
│  Peer count: 3-15 peers each                    │
│  GRANDPA finality active (15/21 supermajority)  │
│  Network healthy and producing blocks           │
└─────────────────────────────────────────────────┘
```

---

**Status:** Ready to execute bootnode configuration update
**Last Updated:** 2025-11-03
