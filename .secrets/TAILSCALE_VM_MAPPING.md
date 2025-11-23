# Tailscale VM Mapping - Ëtrid Primearc Core Validators

**Extracted**: 2025-11-22
**Source**: `tailscale status`

---

## Available Tailscale VMs

### Confirmed Validators

| Tailscale IP | Hostname | Public IP | Validator | Role |
|--------------|----------|-----------|-----------|------|
| 100.96.84.69 | gizzi-io-validator | 64.181.215.19 | Director 1 (Gizzi) | CTO & Bootnode |

### Available VMs (Need Mapping)

| Tailscale IP | Hostname | Status | Potential Use |
|--------------|----------|--------|---------------|
| 100.93.43.18 | vmi2896906 | idle | TBD |
| 100.71.127.127 | vmi2896907 | idle | TBD |
| 100.68.185.50 | vmi2896908 | - | TBD |
| 100.70.73.10 | vmi2896909 | - | TBD |
| 100.88.104.58 | vmi2896910 | idle | TBD |
| 100.117.43.53 | vmi2896911 | - | TBD |
| 100.109.252.56 | vmi2896914 | - | TBD |
| 100.80.84.82 | vmi2896915 | - | TBD |
| 100.125.147.88 | vmi2896916 | - | TBD |
| 100.86.111.37 | vmi2896917 | - | TBD |
| 100.95.0.72 | vmi2896918 | idle | TBD |
| 100.113.226.111 | vmi2896921 | - | TBD |
| 100.114.244.62 | vmi2896922 | - | TBD |
| 100.125.251.60 | vmi2896923 | - | TBD |
| 100.74.204.23 | vmi2896924 | - | TBD |
| 100.124.117.73 | vmi2896925 | - | TBD |
| 100.89.102.75 | vmi2897381 | - | TBD |
| 100.74.84.28 | vmi2897382 | - | TBD |
| 100.71.242.104 | vmi2897383 | - | TBD |
| 100.102.128.51 | vmi2897384 | idle | TBD |

**Total Available**: 20 VMs (excluding gizzi-io-validator)

---

## Required Mappings

### Directors (Need vmi Assignment)

| Director | Expected Hostname | vmi ID | Tailscale IP | Status |
|----------|-------------------|--------|--------------|--------|
| 0 (Eoj) | eoj-validator | TBD | TBD | ❌ Not found |
| 2 (Audit Dev) | oracle-vm-audit | TBD | TBD | ❌ Not found |
| 3 (Security Dev) | oracle-vm-security | TBD | TBD | ❌ Not found |
| 4 (Governance Dev) | oracle-vm-governance | TBD | TBD | ❌ Not found |
| 5 (Oracle Dev) | oracle-vm-oracle | TBD | TBD | ❌ Not found |
| 6 (Consensus Dev) | oracle-vm-consensus | TBD | TBD | ❌ Not found |
| 7 (Economics Dev) | oracle-vm-economics | TBD | TBD | ❌ Not found |
| 8 (Compiler Dev) | oracle-vm-compiler | TBD | TBD | ❌ Not found |

### Validity Nodes (Partial Mapping)

| Validator | Public IP | vmi ID | Tailscale IP | Status |
|-----------|-----------|--------|--------------|--------|
| 9 | 80.190.82.186 | TBD | TBD | ❌ |
| 10 | 85.239.239.194 | TBD | TBD | ❌ |
| 11 | 85.239.239.193 | TBD | TBD | ❌ |
| 12 | 85.239.239.189 | TBD | TBD | ❌ |
| 13 | 85.239.239.188 | TBD | TBD | ❌ |
| 14 | 154.12.250.18 | TBD | TBD | ❌ |
| 15 | **Missing** | TBD | TBD | ❌ |
| 16 | **Missing** | TBD | TBD | ❌ |
| 17 | **Missing** | TBD | TBD | ❌ |
| 18 | **Missing** | TBD | TBD | ❌ |
| 19 | 129.80.122.34 | TBD | TBD | ❌ |
| 20 | **Missing** | TBD | TBD | ❌ |

---

## How to Complete Mapping

### Option 1: SSH to Each VM and Check
```bash
for IP in 100.93.43.18 100.71.127.127 100.68.185.50; do
  echo "Checking $IP..."
  ssh root@$IP "hostname && ip addr show | grep 'inet ' | grep -v '127.0.0.1'"
done
```

### Option 2: Query Tailscale DNS
```bash
# Set Tailscale hostnames
tailscale set hostname oracle-vm-audit --hostname vmi2896906
tailscale set hostname oracle-vm-security --hostname vmi2896907
# ... etc
```

### Option 3: Check Deployment Logs
```bash
grep -r "vmi\|oracle-vm\|validator.*deployed" ~/Desktop/etrid/deployment-logs/
```

### Option 4: Contabo Dashboard
- Log into Contabo dashboard
- Check VM names and public IPs
- Cross-reference with validator accounts

---

## Session Keys Extraction

To get ASF session keys from each validator:

```bash
# For validators with exposed RPC
curl -X POST http://<VALIDATOR_IP>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_rotateKeys","params":[],"id":1}'
```

⚠️ **Note**: Most validators returned empty `0x` responses, suggesting:
- RPC is not exposed publicly (security)
- Need to query via Tailscale private network
- Or extract keys from on-chain storage

---

## Next Steps

1. **Eoj to provide**: His validator VM details (IP, Tailscale hostname, vmi ID)
2. **SSH to oracle-vm VMs**: Identify which vmi corresponds to each AI Dev
3. **Query chain storage**: Extract session keys from ValidatorCommittee storage
4. **Update secrets**: Fill in all TBD fields in `VALIDATORS_MAINNET_REAL.json`

---

**Status**: Partial mapping completed
**Blocking**: Need Eoj's validator info + oracle-vm to vmi mapping
