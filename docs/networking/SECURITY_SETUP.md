# Ëtrid FlareChain Security & Firewall Setup

**Date**: October 29, 2025
**Critical**: Required NSG rules and security configuration for validator deployment

---

## Critical Security Improvement

### ❌ Previous Insecure Approach
```rust
// INSECURE - exposed DETR P2P to entire internet
let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 30334);
```

### ✅ New Secure Approach
```rust
// SECURE - binds to specific network interface
// Priority:
// 1. DETR_P2P_IP environment variable (explicit)
// 2. Auto-detect from Substrate public_addresses
// 3. Auto-detect from Substrate listen_addresses
// 4. Fallback to 0.0.0.0 with security warnings
```

**Result**: DETR P2P now binds to specific IP (`172.16.0.5` or `172.16.0.4`) instead of all interfaces.

---

## Required Azure NSG Rules

### Current NSG Rules (Existing)

Your Resource Group NSG likely has these rules already:

```
Priority  Name             Port   Protocol  Source      Destination  Action
--------  ---------------  -----  --------  ----------  -----------  ------
100       Allow-SSH-Home   22     TCP       73.185.*    Any          Allow
110       Allow-Substrate  30333  TCP       Internet    Any          Allow
120       Allow-RPC        9944   TCP       Internet    Any          Allow  (Optional)
130       Allow-WS         9945   TCP       Internet    Any          Allow  (Optional)
```

### NEW Rule Required for DETR P2P

**Add this rule to your NSG**:

```
Priority: 115
Name: Allow-DETR-P2P
Port: 30334
Protocol: TCP
Source: Internet (or specific validator IPs for better security)
Destination: Any
Action: Allow
Description: DETR P2P network for ASF finality (Kademlia DHT)
```

### Azure CLI Command

```bash
# Get your NSG name
az network nsg list --resource-group <your-rg> --output table

# Add the rule (replace <nsg-name> and <rg-name>)
az network nsg rule create \
  --resource-group <rg-name> \
  --nsg-name <nsg-name> \
  --name Allow-DETR-P2P \
  --priority 115 \
  --source-address-prefixes Internet \
  --source-port-ranges '*' \
  --destination-address-prefixes '*' \
  --destination-port-ranges 30334 \
  --access Allow \
  --protocol Tcp \
  --description "DETR P2P network for ASF finality"
```

### Azure Portal Steps

1. Navigate to: **Azure Portal** → **Resource Groups** → **Your RG** → **Network Security Group**
2. Click **Inbound security rules**
3. Click **+ Add**
4. Configure:
   - **Source**: `IP Addresses` or `Any` (see security options below)
   - **Source IP addresses/CIDR ranges**: Leave blank for `Any`, or specify validator IPs
   - **Source port ranges**: `*`
   - **Destination**: `Any`
   - **Destination port ranges**: `30334`
   - **Protocol**: `TCP`
   - **Action**: `Allow`
   - **Priority**: `115` (between SSH and Substrate)
   - **Name**: `Allow-DETR-P2P`
   - **Description**: `DETR P2P network for ASF finality (Kademlia DHT)`
5. Click **Add**

---

## Security Options for Port 30334

### Option 1: Open to Internet (Least Secure, Simplest)

**NSG Rule**:
```
Source: Internet
Destination Port: 30334
Action: Allow
```

**Pros**:
- Simple configuration
- Works with any new validator that wants to join

**Cons**:
- Exposes DETR P2P to potential attacks
- Anyone can attempt to connect

**Recommended for**: Development/testnet only

### Option 2: Whitelist Validator IPs (Recommended)

**NSG Rule**:
```
Source: 20.186.91.207,172.177.44.73  (add more as validators join)
Destination Port: 30334
Action: Allow
```

**Pros**:
- Only known validators can connect
- Significantly more secure
- Defense in depth

**Cons**:
- Need to update NSG when adding new validators
- Requires coordination

**Recommended for**: Production mainnet

### Option 3: VNet Peering (Most Secure)

**Setup**:
- All validators in same Azure VNet or peered VNets
- Use private IPs only (172.16.0.x)
- No public exposure of port 30334

**NSG Rule**:
```
Source: VirtualNetwork
Destination Port: 30334
Action: Allow
```

**Pros**:
- Maximum security - no public internet exposure
- Private IP communication
- Lower latency

**Cons**:
- Requires all validators in Azure
- More complex network setup

**Recommended for**: Enterprise deployments

---

## Port Summary

FlareChain validators use THREE ports:

| Port  | Protocol | Purpose                          | Security Level | Required |
|-------|----------|----------------------------------|----------------|----------|
| 22    | TCP      | SSH access                       | Whitelisted IP | Yes      |
| 30333 | TCP      | Substrate P2P (blocks, GRANDPA)  | Public         | Yes      |
| 30334 | TCP      | DETR P2P (ASF finality, Kademlia)| **NEW** Public | **YES**  |
| 9944  | TCP      | RPC endpoint                     | Optional       | No       |
| 9945  | TCP      | WebSocket endpoint               | Optional       | No       |

**IMPORTANT**: Port 30334 is NEW and REQUIRED for ASF finality to work!

---

## Updated VM Deployment Scripts

### VM #1 (Bootstrap Node)

**File**: `/tmp/VM1_DEPLOY_SIMPLE.sh`

```bash
#!/bin/bash
# Export VM-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-validator-01"
export VALIDATOR_KEY="//Alice"

# SECURITY: Bind DETR P2P to private IP instead of 0.0.0.0
export DETR_P2P_IP="172.16.0.5"   # VM #1 private IP
export DETR_P2P_PORT="30334"

# Run the generic one-command validator script
exec /opt/etrid/one-command-validator.sh
```

### VM #2 (Connecting Node)

**File**: `/tmp/VM2_DEPLOY_SIMPLE.sh`

```bash
#!/bin/bash
# Export VM-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-validator-02"
export VALIDATOR_KEY="//Bob"

# SECURITY: Bind DETR P2P to private IP instead of 0.0.0.0
export DETR_P2P_IP="172.16.0.4"   # VM #2 private IP
export DETR_P2P_PORT="30334"
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334"  # Connect to VM #1's DETR P2P

# Get Alice's Substrate peer ID
read -p "Enter Alice's Peer ID (12D3KooW...): " ALICE_PEER_ID
ALICE_BOOTNODE="/ip4/172.16.0.5/tcp/30333/p2p/$ALICE_PEER_ID"
export BOOTNODE="$ALICE_BOOTNODE"

# Run the generic one-command validator script
exec /opt/etrid/one-command-validator.sh
```

---

## Security Verification

### Startup Logs to Check

After starting the node, you should see:

```
🔒 DETR P2P IP from DETR_P2P_IP env: 172.16.0.5
🌐 DETR P2P will listen on: 172.16.0.5:30334
🔒 SECURITY: Port 30334 bound to specific interface
```

**Good**: ✅ "bound to specific interface"

**Bad**: ⚠️ "Port 30334 exposed on ALL network interfaces" - means it's using 0.0.0.0

### Test Port Binding

On the VM, check what's listening:

```bash
# Check if DETR P2P is bound to specific IP
sudo netstat -tulpn | grep 30334

# Should show:
tcp  0  0  172.16.0.5:30334  0.0.0.0:*  LISTEN  12345/flarechain-node

# NOT:
tcp  0  0  0.0.0.0:30334     0.0.0.0:*  LISTEN  12345/flarechain-node
```

### Test Connectivity Between VMs

From VM #2, test if VM #1's DETR P2P port is accessible:

```bash
# Test port 30334 connectivity
nc -zv 172.16.0.5 30334

# Should output:
Connection to 172.16.0.5 30334 port [tcp/*] succeeded!
```

---

## Firewall Defense in Depth

Even with NSG rules, implement these security measures:

### 1. Rate Limiting (iptables on VMs)

```bash
# Limit new connections to port 30334 (prevent DoS)
sudo iptables -A INPUT -p tcp --dport 30334 -m state --state NEW -m recent --set
sudo iptables -A INPUT -p tcp --dport 30334 -m state --state NEW -m recent --update --seconds 60 --hitcount 20 -j DROP
```

### 2. Connection Tracking

```bash
# Only allow established connections
sudo iptables -A INPUT -p tcp --dport 30334 -m state --state ESTABLISHED,RELATED -j ACCEPT
```

### 3. Geographic Blocking (Optional)

If you know where your validators are located:

```bash
# Install geoip
sudo apt-get install xtables-addons-common

# Block all except specific countries (e.g., US, Canada)
sudo iptables -A INPUT -p tcp --dport 30334 -m geoip ! --src-cc US,CA -j DROP
```

---

## Monitoring & Alerts

### Monitor Port 30334 Traffic

```bash
# Real-time connection monitoring
watch -n 1 'sudo netstat -tn | grep :30334'

# Count active connections
sudo netstat -tn | grep :30334 | wc -l
```

### Azure Monitor Alert

Set up an alert for unusual traffic:

```bash
# Alert if more than 100 connections to port 30334
az monitor metrics alert create \
  --name High-DETR-P2P-Connections \
  --resource-group <rg> \
  --scopes <vm-id> \
  --condition "avg Network In > 1000000" \
  --window-size 5m \
  --evaluation-frequency 1m
```

---

## Troubleshooting

### Issue: "0 peers" after adding NSG rule

**Check**:
1. NSG rule is applied to correct NSG
2. NSG is associated with VM's subnet or NIC
3. Port 30334 is actually open: `sudo netstat -tulpn | grep 30334`
4. VM firewall (ufw/iptables) allows port 30334

**Solution**:
```bash
# On both VMs, allow port 30334
sudo ufw allow 30334/tcp
# Or if using iptables:
sudo iptables -A INPUT -p tcp --dport 30334 -j ACCEPT
```

### Issue: DETR P2P still binding to 0.0.0.0

**Check**:
```bash
# Verify environment variable is set
echo $DETR_P2P_IP
```

**Solution**:
- Ensure `export DETR_P2P_IP="172.16.0.5"` is in deployment script
- Restart validator node after setting variable

### Issue: Bootstrap peer not connecting

**Check**:
```bash
# From VM #2, test connection to VM #1
telnet 172.16.0.5 30334

# Or using netcat:
nc -zv 172.16.0.5 30334
```

**Solution**:
- If "Connection refused": VM #1's DETR P2P not running
- If "No route to host": NSG or subnet routing issue
- If "Connection timeout": Firewall blocking

---

## Production Checklist

Before going live with validators:

- [ ] NSG rule added for port 30334
- [ ] `DETR_P2P_IP` set to specific interface (not 0.0.0.0)
- [ ] `DETR_P2P_BOOTSTRAP` configured on non-bootstrap nodes
- [ ] VM firewall (ufw/iptables) allows port 30334
- [ ] Tested connectivity: `nc -zv <validator-ip> 30334`
- [ ] Verified binding: `netstat -tulpn | grep 30334` shows specific IP
- [ ] Startup logs show: "🔒 SECURITY: Port 30334 bound to specific interface"
- [ ] Both validators show "1 peers" (not "0 peers")
- [ ] Rate limiting configured (iptables rules)
- [ ] Monitoring alerts set up
- [ ] Documented all validator IPs for whitelist maintenance

---

## Architecture Diagram

```
Internet
   │
   │ Port 22 (SSH - Whitelisted IP)
   │ Port 30333 (Substrate P2P - Public)
   │ Port 30334 (DETR P2P - NEW, Controlled)
   ↓
┌─────────────────────────────────────────────┐
│         Azure Network Security Group        │
│  • Priority 100: Allow SSH (73.185.*.*)    │
│  • Priority 110: Allow Substrate (30333)    │
│  • Priority 115: Allow DETR P2P (30334) ✅  │ ← NEW
└─────────────────────────────────────────────┘
   │
   ↓
┌─────────────────────────────────────────────┐
│              Azure VNet                      │
│  Subnet: 172.16.0.0/24                      │
│                                             │
│  ┌──────────────────┐  ┌──────────────────┐│
│  │   VM #1          │  │   VM #2          ││
│  │   172.16.0.5     │←─┤   172.16.0.4     ││
│  │                  │  │                  ││
│  │  Port 30333 ✅   │  │  Port 30333 ✅   ││
│  │  (Substrate P2P) │  │  (Substrate P2P) ││
│  │                  │  │                  ││
│  │  Port 30334 ✅   │  │  Port 30334 ✅   ││
│  │  (DETR P2P)      │  │  (DETR P2P)      ││
│  │  Bound to        │  │  Bound to        ││
│  │  172.16.0.5      │  │  172.16.0.4      ││
│  └──────────────────┘  └──────────────────┘│
└─────────────────────────────────────────────┘
```

---

## Summary of Security Improvements

| Aspect | Before | After |
|--------|--------|-------|
| DETR P2P Binding | `127.0.0.1` (localhost only) | Specific IP `172.16.0.5` |
| Exposure Risk | Localhost (unreachable) | Controlled via NSG + specific binding |
| Firewall Rules | Missing port 30334 | NSG rule added for 30334 |
| Configuration | Hardcoded | Environment variables + auto-detection |
| Logging | None | Security warnings + status indicators |
| Bootstrap Discovery | Empty list | Auto-parsed from --bootnodes |

**Result**: Secure peer discovery with defense-in-depth protection.

---

**Generated**: October 29, 2025
**Location**: `/tmp/SECURITY_AND_FIREWALL_SETUP.md`
