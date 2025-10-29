# Ëtrid FlareChain Production Network Architecture

**Date**: October 29, 2025
**Purpose**: Multi-tier secure network for validators, AIDIDs, and directors

---

## Network Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                          PUBLIC INTERNET                             │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
                    ↓              ↓              ↓
        ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
        │ Bootstrap Node 1│ │ Bootstrap Node 2│ │ Bootstrap Node 3│
        │ 20.186.91.207   │ │ 172.177.44.73   │ │ <VM3-Public-IP> │
        │                 │ │                 │ │                 │
        │ NSG: Port 30334 │ │ NSG: Port 30334 │ │ NSG: Port 30334 │
        │ Source: ANY ⚠️  │ │ Source: ANY ⚠️  │ │ Source: ANY ⚠️  │
        │ (Public Access) │ │ (Public Access) │ │ (Public Access) │
        └─────────────────┘ └─────────────────┘ └─────────────────┘
                    │              │              │
                    │         VNet Peering        │
                    └──────────────┼──────────────┘
                                   │
        ┌──────────────────────────┴─────────────────────────┐
        │                                                      │
        │          Azure VNet: etrid-validator-vnet           │
        │          Address Space: 172.16.0.0/16               │
        │                                                      │
        │  ┌────────────────────────────────────────────┐    │
        │  │  Subnet: validator-subnet (172.16.1.0/24)  │    │
        │  │                                             │    │
        │  │  Regular Validators (Whitelisted)          │    │
        │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐   │    │
        │  │  │ VM #6    │ │ VM #7    │ │ VM #8    │   │    │
        │  │  │172.16.1.6│ │172.16.1.7│ │172.16.1.8│   │    │
        │  │  │          │ │          │ │          │   │    │
        │  │  │ NSG: Whitelist only bootstrap IPs  │   │    │
        │  │  └──────────┘ └──────────┘ └──────────┘   │    │
        │  └────────────────────────────────────────────┘    │
        │                                                      │
        │  ┌────────────────────────────────────────────┐    │
        │  │  Subnet: aidid-subnet (172.16.2.0/24)      │    │
        │  │                                             │    │
        │  │  AIDID Nodes (VNet Only - PRIVATE)         │    │
        │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐   │    │
        │  │  │ AIDID #1 │ │ AIDID #2 │ │ AIDID #3 │   │    │
        │  │  │172.16.2.10│ │172.16.2.11│ │172.16.2.12│  │    │
        │  │  │          │ │          │ │          │   │    │
        │  │  │ NSG: VirtualNetwork only (no internet) │    │
        │  │  └──────────┘ └──────────┘ └──────────┘   │    │
        │  └────────────────────────────────────────────┘    │
        │                                                      │
        │  ┌────────────────────────────────────────────┐    │
        │  │  Subnet: director-subnet (172.16.3.0/24)   │    │
        │  │                                             │    │
        │  │  Decentralized Directors (VNet Only)       │    │
        │  │  ┌──────────┐ ┌──────────┐                │    │
        │  │  │Director#1│ │Director#2│                │    │
        │  │  │172.16.3.20│ │172.16.3.21│               │    │
        │  │  │          │ │          │                │    │
        │  │  │ NSG: VirtualNetwork only               │    │
        │  │  └──────────┘ └──────────┘                │    │
        │  └────────────────────────────────────────────┘    │
        └──────────────────────────────────────────────────┘
```

---

## Network Tiers Explained

### Tier 1: Public Bootstrap Nodes (5 nodes)

**Purpose**: Entry points for new validators joining the network

**Security Profile**:
- ⚠️ Port 30334 open to internet
- Rate limiting enabled (10 new connections/minute)
- Heavy monitoring and alerts
- Auto-ban malicious IPs
- Regular security audits

**VMs**: VM #1, #2, #3, #4, #5

**NSG Rule**:
```
Source: Any
Destination Port: 30334
Action: Allow
Priority: 115
Name: Allow-DETR-P2P-Bootstrap-Public
```

**Additional Hardening**:
```bash
# On each bootstrap node
# Rate limit new connections
sudo iptables -A INPUT -p tcp --dport 30334 -m state --state NEW -m recent --set --name DETR_CONN
sudo iptables -A INPUT -p tcp --dport 30334 -m state --state NEW -m recent --update --seconds 60 --hitcount 10 --name DETR_CONN -j DROP

# Log dropped connections
sudo iptables -A INPUT -p tcp --dport 30334 -j LOG --log-prefix "DETR-P2P-REJECT: " --log-level 4

# Connection tracking limits
sudo iptables -A INPUT -p tcp --dport 30334 -m connlimit --connlimit-above 50 --connlimit-mask 32 -j REJECT
```

---

### Tier 2: Regular Validators (Whitelisted)

**Purpose**: Core network validators with higher security

**Security Profile**:
- ✅ Port 30334 whitelisted (only known validators)
- Private subnet within VNet
- Can communicate with bootstrap nodes
- Monitored traffic patterns

**VMs**: VM #6+

**NSG Rule**:
```
Source: 20.186.91.207,172.177.44.73,<bootstrap-IPs>
Destination Port: 30334
Action: Allow
Priority: 120
Name: Allow-DETR-P2P-Whitelist-Validators
```

**Subnet**: `172.16.1.0/24`

**Adding New Validator**:
1. Deploy VM in validator-subnet
2. Assign private IP: `172.16.1.X`
3. Add public IP to bootstrap nodes' whitelist
4. Configure with bootstrap node addresses

---

### Tier 3: AIDID Nodes (Private Network)

**Purpose**: AI-powered Decentralized Identity nodes with privileged access

**Security Profile**:
- 🔒 NO public internet access
- VNet only communication
- Subnet-level NSG isolation
- Can ONLY communicate with validators in same VNet
- Cannot be reached from internet

**Subnet**: `172.16.2.0/24`

**NSG Rule**:
```
Source: VirtualNetwork (Service Tag)
Destination: VirtualNetwork
Destination Port: 30334, 30333, 9944
Action: Allow
Priority: 100
Name: Allow-VNet-Internal-AIDID

# DENY all internet inbound
Source: Internet
Destination: Any
Destination Port: *
Action: Deny
Priority: 200
Name: Deny-Internet-AIDID
```

**Configuration**:
```bash
# AIDID nodes connect only to private IPs
export DETR_P2P_IP="172.16.2.10"  # AIDID node's private IP
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334,172.16.0.4:30334"  # Bootstrap private IPs only
```

**No Public IP**: AIDID VMs do not have public IPs assigned

---

### Tier 4: Decentralized Directors (Most Secure)

**Purpose**: Governance and protocol-level decision makers

**Security Profile**:
- 🔐 Highest security tier
- VNet only, no public IP
- Separate subnet from AIDIDs
- NSG denies all except VNet traffic
- Additional application-level authentication

**Subnet**: `172.16.3.0/24`

**NSG Rule**: Same as AIDID tier (VNet only)

**Special Requirements**:
- Multi-signature authentication
- Audit logging of all connections
- Immutable change logs
- Regular penetration testing

---

## Azure VNet Configuration

### Step 1: Create VNet

```bash
# Create VNet
az network vnet create \
  --resource-group etrid-validators \
  --name etrid-validator-vnet \
  --address-prefix 172.16.0.0/16 \
  --location eastus

# Create validator subnet
az network vnet subnet create \
  --resource-group etrid-validators \
  --vnet-name etrid-validator-vnet \
  --name validator-subnet \
  --address-prefix 172.16.1.0/24

# Create AIDID subnet
az network vnet subnet create \
  --resource-group etrid-validators \
  --vnet-name etrid-validator-vnet \
  --name aidid-subnet \
  --address-prefix 172.16.2.0/24

# Create director subnet
az network vnet subnet create \
  --resource-group etrid-validators \
  --vnet-name etrid-validator-vnet \
  --name director-subnet \
  --address-prefix 172.16.3.0/24
```

### Step 2: Create NSGs per Subnet

**AIDID NSG**:
```bash
az network nsg create \
  --resource-group etrid-validators \
  --name aidid-nsg

# Allow VNet internal traffic
az network nsg rule create \
  --resource-group etrid-validators \
  --nsg-name aidid-nsg \
  --name Allow-VNet-Internal \
  --priority 100 \
  --source-address-prefixes VirtualNetwork \
  --destination-port-ranges 30334 30333 9944 \
  --access Allow \
  --protocol Tcp

# DENY all internet
az network nsg rule create \
  --resource-group etrid-validators \
  --nsg-name aidid-nsg \
  --name Deny-Internet \
  --priority 200 \
  --source-address-prefixes Internet \
  --destination-port-ranges '*' \
  --access Deny \
  --protocol '*'

# Associate with subnet
az network vnet subnet update \
  --resource-group etrid-validators \
  --vnet-name etrid-validator-vnet \
  --name aidid-subnet \
  --network-security-group aidid-nsg
```

**Director NSG** (same as AIDID):
```bash
az network nsg create \
  --resource-group etrid-validators \
  --name director-nsg

# Same rules as AIDID NSG
az network nsg rule create \
  --resource-group etrid-validators \
  --nsg-name director-nsg \
  --name Allow-VNet-Internal \
  --priority 100 \
  --source-address-prefixes VirtualNetwork \
  --destination-port-ranges 30334 30333 9944 \
  --access Allow \
  --protocol Tcp

az network nsg rule create \
  --resource-group etrid-validators \
  --nsg-name director-nsg \
  --name Deny-Internet \
  --priority 200 \
  --source-address-prefixes Internet \
  --destination-port-ranges '*' \
  --access Deny \
  --protocol '*'

az network vnet subnet update \
  --resource-group etrid-validators \
  --vnet-name etrid-validator-vnet \
  --name director-subnet \
  --network-security-group director-nsg
```

### Step 3: Deploy VMs to Correct Subnets

**Bootstrap Validators** (VM #1-5):
- Deploy in default subnet or validator-subnet
- Assign public IPs
- Open NSG rules (port 30334 to Any)

**Regular Validators** (VM #6+):
- Deploy in validator-subnet
- Assign public IPs (optional)
- Whitelisted NSG rules

**AIDID Nodes**:
- Deploy in aidid-subnet
- **NO public IP**
- VNet-only NSG

**Directors**:
- Deploy in director-subnet
- **NO public IP**
- VNet-only NSG

---

## Bootstrap Node Configuration (5 Nodes Required)

### Why 5 Bootstrap Nodes?

**Industry Standard**:
- Bitcoin: 6-8 seed nodes
- Ethereum: 8+ boot nodes
- Polkadot: 7-10 boot nodes

**Benefits**:
- Fault tolerance (2 can fail, 3 still work)
- Geographic distribution
- Load balancing for new validators
- DDoS resilience

### Bootstrap Node IPs

You need **3 more** in addition to VM #1 and #2:

```
VM #1: 20.186.91.207 (existing)
VM #2: 172.177.44.73 (existing)
VM #3: <get-public-ip>
VM #4: <get-public-ip>
VM #5: <get-public-ip>
```

### Updated Chain Spec with 5 Bootstrap Nodes

**File**: `infrastructure/chain-specs/flarechain-local-raw.json`

```json
{
  "bootNodes": [
    "/ip4/20.186.91.207/tcp/30333/p2p/<VM1-PEER-ID>",
    "/ip4/172.177.44.73/tcp/30333/p2p/<VM2-PEER-ID>",
    "/ip4/<VM3-PUBLIC-IP>/tcp/30333/p2p/<VM3-PEER-ID>",
    "/ip4/<VM4-PUBLIC-IP>/tcp/30333/p2p/<VM4-PEER-ID>",
    "/ip4/<VM5-PUBLIC-IP>/tcp/30333/p2p/<VM5-PEER-ID>"
  ]
}
```

### Updated Validator Scripts

**Generic Startup Script** (`one-command-validator.sh`):

Already handles multiple bootstrap nodes automatically! The code parses all bootnodes from the chain spec.

**VM #6+ Deployment Script**:

```bash
#!/bin/bash
# VM #6+ - Regular Validator with 5 Bootstrap Nodes

export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-validator-06"
export VALIDATOR_KEY="//Charlie"

# SECURITY: Bind to specific IP
export DETR_P2P_IP="172.16.1.6"
export DETR_P2P_PORT="30334"

# Bootstrap to all 5 nodes (private IPs for VNet peers)
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334,172.16.0.4:30334,172.16.1.3:30334,172.16.1.4:30334,172.16.1.5:30334"

# Substrate bootnodes (automatically from chain spec)
exec /opt/etrid/one-command-validator.sh
```

### Maintenance: Adding Bootstrap Node to Whitelist

When you add VM #3 as a bootstrap node:

1. **Update NSG on VM #3**:
   ```
   Source: Any
   Destination Port: 30334
   Action: Allow
   ```

2. **Update whitelists on VM #6+**:
   ```
   Source: 20.186.91.207,172.177.44.73,<VM3-IP>,<VM4-IP>,<VM5-IP>
   ```

3. **Update all deployment scripts**:
   Add VM #3's IP to `DETR_P2P_BOOTSTRAP` environment variable

---

## AIDID Node Deployment

**Purpose**: AI identity verification with high security

**Script**: `/tmp/AIDID_DEPLOY.sh`

```bash
#!/bin/bash
# AIDID Node Deployment (Private Network Only)

set -e

echo "================================================================================"
echo "  Ëtrid AIDID Node Deployment (Private Network)"
echo "================================================================================"
echo ""
echo "⚠️  This node has NO public IP and communicates via VNet only"
echo ""

# Export AIDID-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-aidid-01"
export VALIDATOR_KEY="//AIDID//01"  # Special key type for AIDIDs

# SECURITY: Bind to private IP ONLY (no public exposure)
export DETR_P2P_IP="172.16.2.10"  # AIDID subnet private IP
export DETR_P2P_PORT="30334"

# Bootstrap ONLY to private IPs of bootstrap validators
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334,172.16.0.4:30334"

# Substrate bootnodes using PRIVATE IPs
export BOOTNODE="/ip4/172.16.0.5/tcp/30333/p2p/<VM1-PEER-ID>,/ip4/172.16.0.4/tcp/30333/p2p/<VM2-PEER-ID>"

echo "🔒 AIDID Mode: Private network only"
echo "🔒 DETR P2P: $DETR_P2P_IP:$DETR_P2P_PORT"
echo "🔒 Bootstrap: Private validator IPs only"
echo ""
read -p "Press Enter to start AIDID node..."

# Run the generic one-command validator script
exec /opt/etrid/one-command-validator.sh --aidid-mode
```

**Key Differences**:
- Uses private IP only (`172.16.2.10`)
- Connects to bootstrap nodes via private IPs
- No public internet exposure
- Special validator key type

---

## Director Node Deployment

**Purpose**: Governance with maximum security

**Script**: `/tmp/DIRECTOR_DEPLOY.sh`

```bash
#!/bin/bash
# Decentralized Director Deployment (Maximum Security)

set -e

echo "================================================================================"
echo "  Ëtrid Decentralized Director Node (Maximum Security)"
echo "================================================================================"
echo ""
echo "🔐 This node operates in the most secure tier"
echo "🔐 VNet-only communication"
echo "🔐 Multi-signature authentication required"
echo ""

# Export Director-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-director-01"
export VALIDATOR_KEY="//Director//01"

# MAXIMUM SECURITY: Private IP only
export DETR_P2P_IP="172.16.3.20"  # Director subnet
export DETR_P2P_PORT="30334"

# Bootstrap ONLY to private IPs
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334,172.16.0.4:30334"

# Additional security: Multi-sig requirement
export REQUIRE_MULTISIG="true"
export MIN_SIGNATURES="3"

echo "🔐 Director Mode: Maximum security tier"
echo "🔐 Private IP: $DETR_P2P_IP"
echo "🔐 Multi-sig required: $MIN_SIGNATURES signatures"
echo ""
read -p "Press Enter to start Director node..."

exec /opt/etrid/one-command-validator.sh --director-mode --require-multisig
```

---

## Communication Matrix

| Source → Destination | Public Bootstrap | Regular Validators | AIDID Nodes | Directors |
|---------------------|-----------------|-------------------|-------------|-----------|
| **Internet** | ✅ Yes (30334) | ✅ Yes (whitelisted) | ❌ No | ❌ No |
| **Public Bootstrap** | ✅ Yes | ✅ Yes | ✅ Yes (VNet) | ✅ Yes (VNet) |
| **Regular Validators** | ✅ Yes | ✅ Yes (whitelisted) | ✅ Yes (VNet) | ✅ Yes (VNet) |
| **AIDID Nodes** | ✅ Yes (VNet) | ✅ Yes (VNet) | ✅ Yes (VNet) | ✅ Yes (VNet) |
| **Directors** | ✅ Yes (VNet) | ✅ Yes (VNet) | ✅ Yes (VNet) | ✅ Yes (VNet) |

**Key**:
- ✅ Yes = Can communicate
- ❌ No = Blocked by NSG
- (VNet) = Only via private network
- (whitelisted) = Only known IPs

---

## Summary of Security Tiers

| Tier | Port 30334 Exposure | NSG Rule | Use Case |
|------|-------------------|----------|----------|
| **Bootstrap** | Public (Any) | ⚠️ Open | Network entry points (5 nodes) |
| **Regular Validators** | Whitelisted IPs | ✅ Secure | Core validation (6+ nodes) |
| **AIDID Nodes** | VNet Only | 🔒 Private | Identity management |
| **Directors** | VNet Only | 🔐 Maximum | Governance & protocol |

---

## Production Deployment Checklist

**Phase 1: Bootstrap Nodes**
- [ ] Deploy 5 bootstrap VMs (VM #1-5)
- [ ] Assign public IPs to all 5
- [ ] Configure NSG: Port 30334 open to `Any`
- [ ] Add rate limiting (iptables)
- [ ] Configure monitoring and alerts
- [ ] Get peer IDs from all 5 nodes
- [ ] Update chain spec with all 5 bootnodes

**Phase 2: VNet Setup**
- [ ] Create etrid-validator-vnet (172.16.0.0/16)
- [ ] Create validator-subnet (172.16.1.0/24)
- [ ] Create aidid-subnet (172.16.2.0/24)
- [ ] Create director-subnet (172.16.3.0/24)
- [ ] Create and configure NSGs for each subnet
- [ ] Test VNet connectivity

**Phase 3: Regular Validators**
- [ ] Deploy regular validators in validator-subnet
- [ ] Configure whitelisted NSG rules
- [ ] Add all bootstrap IPs to whitelist
- [ ] Test connectivity to bootstrap nodes
- [ ] Verify peer discovery works

**Phase 4: Private Nodes**
- [ ] Deploy AIDID nodes in aidid-subnet (no public IP)
- [ ] Deploy director nodes in director-subnet (no public IP)
- [ ] Verify VNet-only communication
- [ ] Test multi-signature authentication
- [ ] Audit security configuration

**Phase 5: Testing**
- [ ] All validators show connected peers
- [ ] AIDID nodes can reach validators
- [ ] Directors can reach all nodes
- [ ] Internet cannot reach AIDIDs/Directors
- [ ] Rate limiting on bootstrap nodes works
- [ ] Monitoring alerts functioning

---

**Generated**: October 29, 2025
**Location**: `/tmp/PRODUCTION_NETWORK_ARCHITECTURE.md`
