# Ëtrid FlareChain - Port Requirements for Deployment

**Date:** November 2, 2025
**Critical Finding:** Port conflicts on same VNet/subscription
**Status:** ⚠️ **REQUIRES PLANNING**

---

## Executive Summary

If multiple validators are deployed on the same Azure subscription/VNet or any shared network namespace, **port conflicts will occur** for both Substrate P2P and DETR P2P networks.

**Affected Ports:**
- **30333** - Substrate P2P (standard blockchain network)
- **30334** - DETR P2P (ASF finality gadget network)

**Impact:** Each validator on the same network requires unique port assignments.

---

## Port Usage Per Validator

Each Ëtrid FlareChain validator node requires **3 ports**:

| Service | Default Port | Configurable | Required |
|---------|--------------|--------------|----------|
| Substrate P2P | 30333 | ✅ Yes (`--port`) | ✅ Yes |
| DETR P2P (ASF) | 30334 | ❌ **No (hardcoded)** | ✅ Yes |
| JSON-RPC | 9933/9944 | ✅ Yes (`--rpc-port`) | Optional* |

*RPC port only needed if exposing RPC externally

---

## Deployment Scenarios

### Scenario 1: Each Validator on Separate Machine/VM ✅ RECOMMENDED

**Configuration:** 21 validators across 21 different VMs

```
Validator 1 (VM #1 - 64.181.215.19):
  - Substrate P2P: 30333
  - DETR P2P: 30334
  - RPC: 9933

Validator 2 (VM #2 - 52.252.142.146):
  - Substrate P2P: 30333
  - DETR P2P: 30334
  - RPC: 9933

...and so on for all 21 validators
```

**Firewall Rules:**
- Open TCP 30333 (Substrate P2P)
- Open TCP/UDP 30334 (DETR P2P)
- Open TCP 9933/9944 (RPC - only if external access needed)

**Pros:**
✅ No port conflicts
✅ Simple configuration
✅ Maximum network isolation
✅ Easy firewall management

**Cons:**
💰 Higher cost (21 separate VMs)

---

### Scenario 2: Multiple Validators per VNet ⚠️ REQUIRES CUSTOM PORTS

**Configuration:** Multiple validators on same Azure VNet/subscription

**Example:** 3 validators on same VNet

```
Validator 1 (VM #1):
  - Substrate P2P: 30333
  - DETR P2P: 30334 (WILL CONFLICT!)
  - RPC: 9933

Validator 2 (VM #2 - Same VNet):
  - Substrate P2P: 30333 (WILL CONFLICT!)
  - DETR P2P: 30334 (WILL CONFLICT!)
  - RPC: 9933

Validator 3 (VM #3 - Same VNet):
  - Substrate P2P: 30333 (WILL CONFLICT!)
  - DETR P2P: 30334 (WILL CONFLICT!)
  - RPC: 9933
```

**Problem:** All validators try to bind to same ports.

**Solution Required:** Custom port allocation

```
Validator 1 (VM #1):
  --port 30333
  # DETR P2P: 30334 (no CLI option - hardcoded)
  --rpc-port 9933

Validator 2 (VM #2):
  --port 30335
  # DETR P2P: 30334 (STILL CONFLICTS!)
  --rpc-port 9934

Validator 3 (VM #3):
  --port 30336
  # DETR P2P: 30334 (STILL CONFLICTS!)
  --rpc-port 9935
```

**Critical Issue:** DETR P2P port 30334 is **hardcoded** and cannot be changed via CLI.

---

## DETR P2P Port Limitation

### Current Implementation

The DETR P2P network (used by ASF finality gadget) uses **hardcoded port 30334**.

**Location:** `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/asf_service.rs` (approximate)

```rust
// Hardcoded port
let detr_port = 30334;
```

### Impact on Deployment

**✅ Works:** Each validator on separate machine
**❌ Fails:** Multiple validators on same VNet/subscription/localhost

### Workaround Options

#### Option 1: Ensure 1 Validator Per Machine/Network ✅ RECOMMENDED

Deploy validators such that no two share the same network namespace.

**Azure Example:**
- 21 separate VMs
- Each in different resource group (optional)
- Each with own private IP
- Network Security Groups (NSGs) per VM

**Cost Estimation:**
- 21 × B2s VMs @ ~$30/month = ~$630/month
- Or 21 × spot instances for lower cost

#### Option 2: Code Modification to Make DETR P2P Port Configurable

**Implementation needed:**

```rust
// In node CLI arguments
--detr-p2p-port <PORT>

// Or environment variable
DETR_P2P_PORT=30335

// Code change in asf_service.rs:
let detr_port = std::env::var("DETR_P2P_PORT")
    .ok()
    .and_then(|p| p.parse().ok())
    .unwrap_or(30334);
```

**Effort:** Low (~1 hour)
**Benefit:** Enables multiple validators per VNet
**Risk:** Minimal (just adds configurability)

---

## Recommended Deployment Architecture

### For Mainnet Launch

**Phase 1: Bootstrap Validators (5 nodes)**

| Validator | IP | Substrate P2P | DETR P2P | RPC | Location |
|-----------|-----|---------------|----------|-----|----------|
| Gizzi | 64.181.215.19 | 30333 | 30334 | 9933 | Oracle Cloud |
| EojEdred | [Local] | 30333 | 30334 | 9933 | Local Machine |
| governance-dev01 | [TBD] | 30333 | 30334 | 9933 | Azure |
| security-dev01 | 52.252.142.146 | 30333 | 30334 | 9933 | Azure |
| audit-dev01 | 129.80.122.34 | 30333 | 30334 | 9933 | Oracle Cloud |

**Phase 2: Remaining Validators (16 nodes)**

Deploy across additional VMs, each with standard ports:
- Substrate P2P: 30333
- DETR P2P: 30334
- RPC: 9933

---

## Firewall Configuration

### Required Inbound Rules

```
# Substrate P2P (blockchain network)
Allow TCP 30333 from: 0.0.0.0/0 (or validator IP range)

# DETR P2P (ASF finality)
Allow TCP 30334 from: 0.0.0.0/0 (or validator IP range)
Allow UDP 30334 from: 0.0.0.0/0 (or validator IP range)

# RPC (optional - only if external access needed)
Allow TCP 9933 from: [Specific IPs only]
Allow TCP 9944 from: [Specific IPs only]

# Prometheus (optional - monitoring)
Allow TCP 9615 from: [Monitoring server IP]
```

### Security Best Practices

1. **RPC Access:**
   - DO NOT expose RPC to 0.0.0.0/0
   - Whitelist specific IPs only
   - Use `--rpc-methods=Safe` in production
   - Only use `--rpc-methods=Unsafe` temporarily for key insertion

2. **P2P Access:**
   - Substrate P2P (30333) can be open to all validators
   - DETR P2P (30334) can be open to all validators
   - Consider VPN for additional security

3. **Monitoring:**
   - Prometheus (9615) - restrict to monitoring server
   - Use Grafana dashboards for metrics visualization

---

## Testing Results

### Single-Node Test
- **Result:** Timeout after ~60 seconds
- **Cause:** No peers available (expected)
- **Status:** ✅ Explained

### 5-Node Test
- **Result:** NO timeout for 90+ seconds
- **Cause:** Peer connectivity satisfied background tasks
- **Port Issue:** DETR P2P conflict (30334) on localhost
- **Status:** ✅ Validated (localhost limitation only)

### 21-Node Test
- **Status:** ⚠️ Cannot test on localhost due to DETR P2P port limitation
- **Alternative:** Will be validated during Phase 1 deployment across separate VMs

---

## Action Items

### Immediate (Before Mainnet Launch)

1. ✅ **Confirm Deployment Architecture**
   - Decision: 1 validator per VM?
   - OR: Multiple validators per VNet with code changes?

2. ⚠️ **If Multiple Validators Per VNet:**
   - Implement DETR P2P port configuration
   - Test with modified code
   - Rebuild node binary

3. ✅ **Firewall Rules**
   - Configure NSGs/security groups
   - Open required ports (30333, 30334)
   - Restrict RPC access

### Future Enhancements

1. **Make DETR P2P Port Configurable**
   - Add CLI parameter `--detr-p2p-port`
   - Add environment variable `DETR_P2P_PORT`
   - Update documentation

2. **Port Range Allocation**
   - Define standard port ranges for multi-validator deployments
   - Document in deployment guide

---

## Conclusion

**For Standard Deployment (1 Validator Per VM):** ✅ No changes needed

**For Multi-Validator Per VNet:** ⚠️ Code modification required to make DETR P2P port configurable

**Recommendation:** Deploy with 1 validator per VM for mainnet launch to avoid any port conflicts.

---

**Prepared By:** Claude AI + Eoj
**Network:** Ëtrid FlareChain Mainnet
**Priority:** HIGH - Must decide before deployment

---

## Quick Reference: Port Summary

```
STANDARD DEPLOYMENT (Recommended):
├── 21 separate VMs
├── Each uses: 30333 (Substrate P2P), 30334 (DETR P2P), 9933 (RPC)
└── No conflicts, simple configuration

MULTI-VALIDATOR PER VNET (Requires Code Change):
├── Substrate P2P: --port 30333, 30335, 30336, ...
├── DETR P2P: NEEDS CODE CHANGE (currently hardcoded to 30334)
└── RPC: --rpc-port 9933, 9934, 9935, ...
```
