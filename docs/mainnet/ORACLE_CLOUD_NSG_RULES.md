# Oracle Cloud NSG Rules for Ëtrid Validators

**Date:** October 30, 2025
**Purpose:** Network Security Group configuration for Ëtrid validator nodes on Oracle Cloud Infrastructure (OCI)

---

## 🔐 Oracle Cloud Network Security Overview

Oracle Cloud offers **two types of network security**:

### 1. **Security Lists** (Legacy, VCN-level)
- Applied at the subnet level
- Stateful rules
- All instances in subnet inherit rules
- Older approach, less flexible

### 2. **Network Security Groups (NSGs)** (Recommended)
- Applied at the VNIC (network interface) level
- More granular control
- Can be assigned to specific instances
- Better for security isolation
- **Use this for validators**

---

## 🔥 Required NSG Rules for Validators

### Ingress Rules (Inbound)

| Rule # | Protocol | Source | Port | Description | Priority |
|--------|----------|--------|------|-------------|----------|
| 1 | TCP | YOUR_IP/32 | 22 | SSH management | **HIGH** |
| 2 | TCP | 0.0.0.0/0 | 30333 | P2P networking | **CRITICAL** |
| 3 | ICMP | 0.0.0.0/0 | All | Ping (optional) | LOW |

### Egress Rules (Outbound)

| Rule # | Protocol | Destination | Port | Description |
|--------|----------|-------------|------|-------------|
| 1 | All | 0.0.0.0/0 | All | Allow all outbound |

---

## 🌐 Oracle Cloud Console Configuration

### Step 1: Create Network Security Group

1. **Login to Oracle Cloud Console:**
   - https://cloud.oracle.com/

2. **Navigate to NSGs:**
   ```
   Menu → Networking → Virtual Cloud Networks
   → Select your VCN
   → Network Security Groups (left menu)
   ```

3. **Create NSG:**
   - Click **"Create Network Security Group"**
   - **Name:** `etrid-validator-nsg`
   - **Compartment:** Select your compartment
   - Click **"Create"**

---

### Step 2: Add Ingress Rules

Click on your newly created NSG (`etrid-validator-nsg`), then **"Add Rules"**:

#### **Rule 1: SSH Access**
```yaml
Direction: Ingress
Stateless: No
Source Type: CIDR
Source CIDR: YOUR_IP_ADDRESS/32  # Replace with your actual IP
IP Protocol: TCP
Source Port Range: All
Destination Port Range: 22
Description: SSH access for management
```

#### **Rule 2: Validator P2P (CRITICAL)**
```yaml
Direction: Ingress
Stateless: No
Source Type: CIDR
Source CIDR: 0.0.0.0/0
IP Protocol: TCP
Source Port Range: All
Destination Port Range: 30333
Description: Validator P2P networking - MUST be public
```

#### **Rule 3: ICMP Ping (Optional)**
```yaml
Direction: Ingress
Stateless: No
Source Type: CIDR
Source CIDR: 0.0.0.0/0
IP Protocol: ICMP
Type: 8 (Echo Request)
Code: All
Description: Allow ping for diagnostics
```

---

### Step 3: Add Egress Rules

Still in your NSG, click **"Add Rules"** again:

#### **Rule 1: Allow All Outbound**
```yaml
Direction: Egress
Stateless: No
Destination Type: CIDR
Destination CIDR: 0.0.0.0/0
IP Protocol: All Protocols
Description: Allow all outbound traffic
```

---

### Step 4: Attach NSG to Validator Instance

1. **Navigate to your instance:**
   ```
   Menu → Compute → Instances
   → Select your validator instance
   ```

2. **Attach NSG:**
   ```
   Resources (left menu) → Attached VNICs
   → Click on the VNIC
   → Edit VNIC
   → Network Security Groups section
   → Select "etrid-validator-nsg"
   → Save Changes
   ```

---

## 💻 Oracle Cloud CLI (OCI CLI) Configuration

### Prerequisites

Install OCI CLI:
```bash
# Install OCI CLI
bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"

# Configure CLI
oci setup config
```

---

### Create NSG via CLI

```bash
# Set variables
COMPARTMENT_ID="ocid1.compartment.oc1..aaaa..."  # Your compartment OCID
VCN_ID="ocid1.vcn.oc1..aaaa..."                  # Your VCN OCID
YOUR_IP="203.0.113.100"                          # Your actual IP

# Create NSG
oci network nsg create \
  --compartment-id $COMPARTMENT_ID \
  --vcn-id $VCN_ID \
  --display-name "etrid-validator-nsg" \
  --wait-for-state AVAILABLE

# Get NSG OCID (save this)
NSG_ID=$(oci network nsg list \
  --compartment-id $COMPARTMENT_ID \
  --vcn-id $VCN_ID \
  --query "data[?\"display-name\"=='etrid-validator-nsg'].id | [0]" \
  --raw-output)

echo "NSG ID: $NSG_ID"
```

---

### Add Ingress Rules via CLI

```bash
# Rule 1: SSH from your IP
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "SSH access",
    "direction": "INGRESS",
    "protocol": "6",
    "source": "'$YOUR_IP'/32",
    "source-type": "CIDR_BLOCK",
    "tcp-options": {
      "destination-port-range": {
        "max": 22,
        "min": 22
      }
    },
    "is-stateless": false
  }]'

# Rule 2: P2P networking (public)
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "Validator P2P networking",
    "direction": "INGRESS",
    "protocol": "6",
    "source": "0.0.0.0/0",
    "source-type": "CIDR_BLOCK",
    "tcp-options": {
      "destination-port-range": {
        "max": 30333,
        "min": 30333
      }
    },
    "is-stateless": false
  }]'

# Rule 3: ICMP (ping) - Optional
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "ICMP ping",
    "direction": "INGRESS",
    "protocol": "1",
    "source": "0.0.0.0/0",
    "source-type": "CIDR_BLOCK",
    "icmp-options": {
      "type": 8
    },
    "is-stateless": false
  }]'
```

---

### Add Egress Rules via CLI

```bash
# Allow all outbound
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "Allow all outbound",
    "direction": "EGRESS",
    "protocol": "all",
    "destination": "0.0.0.0/0",
    "destination-type": "CIDR_BLOCK",
    "is-stateless": false
  }]'
```

---

### Attach NSG to Instance via CLI

```bash
# Get instance OCID
INSTANCE_ID="ocid1.instance.oc1..aaaa..."  # Your instance OCID

# Get VNIC OCID
VNIC_ID=$(oci compute instance list-vnics \
  --instance-id $INSTANCE_ID \
  --query "data[0].id" \
  --raw-output)

# Attach NSG to VNIC
oci network vnic update \
  --vnic-id $VNIC_ID \
  --nsg-ids '["'$NSG_ID'"]' \
  --force
```

---

## 🔍 Verify NSG Configuration

### Via Console

1. **Check NSG Rules:**
   ```
   Networking → Virtual Cloud Networks → Your VCN
   → Network Security Groups → etrid-validator-nsg
   → Security Rules (tab)
   ```

2. **Verify Instance Attachment:**
   ```
   Compute → Instances → Your Instance
   → Attached VNICs → Primary VNIC
   → Network Security Groups section
   ```

### Via CLI

```bash
# List NSG rules
oci network nsg rules list --nsg-id $NSG_ID

# Check VNIC NSGs
oci network vnic get --vnic-id $VNIC_ID \
  --query "data.\"nsg-ids\""
```

---

## 🆚 NSG vs Security List - Key Differences

| Feature | NSG (Recommended) | Security List (Legacy) |
|---------|-------------------|------------------------|
| **Scope** | VNIC-level (per instance) | Subnet-level (all instances) |
| **Granularity** | Very specific | Broad |
| **Multiple assignments** | Yes (up to 5 per VNIC) | One per subnet |
| **Rule limit** | 120 per NSG | 300 per Security List |
| **Best for** | Production validators | Simple dev environments |
| **Management** | Independent of subnet | Tied to subnet |

---

## ⚠️ Important Oracle Cloud Considerations

### 1. **Default Security List Still Applies**

Even with NSGs, the subnet's Security List rules still apply. Ensure your subnet allows:

**Edit Subnet Security List:**
```
Networking → Virtual Cloud Networks → Your VCN
→ Subnets → Your Subnet
→ Default Security List → Edit Rules
```

Add these if not present:
- **Ingress:** Allow TCP 22 from 0.0.0.0/0 (or your IP)
- **Ingress:** Allow TCP 30333 from 0.0.0.0/0
- **Egress:** Allow all protocols to 0.0.0.0/0

### 2. **Stateful vs Stateless Rules**

- **Stateful (recommended):** Automatically allows return traffic
- **Stateless:** Requires explicit egress rules for responses
- For validators, use **stateful rules** (set `is-stateless: false`)

### 3. **Rule Limits**

- Maximum **120 rules per NSG**
- Maximum **5 NSGs per VNIC**
- Total effective rules: 600 per instance

### 4. **Protocol Numbers**

When using CLI, specify protocols by number:
- **TCP:** 6
- **UDP:** 17
- **ICMP:** 1
- **All:** "all"

---

## 🎯 Recommended Configuration by Node Type

### Standard Validator (Most Secure)

**NSG Rules:**
```yaml
Ingress:
  - SSH (22) from YOUR_IP/32
  - P2P (30333) from 0.0.0.0/0

Egress:
  - All protocols to 0.0.0.0/0
```

**Why minimal?** Smallest attack surface, maximum security.

---

### Validator + Monitoring

**NSG Rules:**
```yaml
Ingress:
  - SSH (22) from YOUR_IP/32
  - P2P (30333) from 0.0.0.0/0
  - Prometheus (9615) from MONITORING_IP/32

Egress:
  - All protocols to 0.0.0.0/0
```

---

### RPC Node (NOT for validators)

**NSG Rules:**
```yaml
Ingress:
  - SSH (22) from YOUR_IP/32
  - P2P (30333) from 0.0.0.0/0
  - WebSocket RPC (9944) from 0.0.0.0/0 or restricted
  - HTTP RPC (9933) from 0.0.0.0/0 or restricted
  - Prometheus (9615) from MONITORING_IP/32

Egress:
  - All protocols to 0.0.0.0/0
```

---

## 🧪 Testing Your Configuration

### Test P2P Port (30333)

```bash
# From outside OCI
telnet <your-validator-public-ip> 30333

# Expected: Connection established
# If connection refused: Port not open
```

### Test SSH

```bash
# From your IP
ssh -i ~/.ssh/your-key ubuntu@<your-validator-public-ip>

# Should work
```

### Test from Blocked IP

```bash
# From a different IP (not YOUR_IP)
ssh -i ~/.ssh/your-key ubuntu@<your-validator-public-ip>

# Should fail: Connection refused or timeout
```

---

## 📋 Complete Deployment Script

Save this as `setup-oci-firewall.sh`:

```bash
#!/bin/bash
# Oracle Cloud NSG Setup for Ëtrid Validator

set -e

echo "🔐 Oracle Cloud NSG Setup for Ëtrid Validator"
echo ""

# Configuration
read -p "Enter your Compartment OCID: " COMPARTMENT_ID
read -p "Enter your VCN OCID: " VCN_ID
read -p "Enter your Instance OCID: " INSTANCE_ID
read -p "Enter your IP address: " YOUR_IP

# Create NSG
echo "Creating NSG..."
NSG_ID=$(oci network nsg create \
  --compartment-id $COMPARTMENT_ID \
  --vcn-id $VCN_ID \
  --display-name "etrid-validator-nsg" \
  --query "data.id" \
  --raw-output \
  --wait-for-state AVAILABLE)

echo "✅ NSG created: $NSG_ID"

# Add SSH rule
echo "Adding SSH rule..."
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "SSH access",
    "direction": "INGRESS",
    "protocol": "6",
    "source": "'$YOUR_IP'/32",
    "source-type": "CIDR_BLOCK",
    "tcp-options": {"destination-port-range": {"max": 22, "min": 22}},
    "is-stateless": false
  }]'

# Add P2P rule
echo "Adding P2P rule..."
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "Validator P2P networking",
    "direction": "INGRESS",
    "protocol": "6",
    "source": "0.0.0.0/0",
    "source-type": "CIDR_BLOCK",
    "tcp-options": {"destination-port-range": {"max": 30333, "min": 30333}},
    "is-stateless": false
  }]'

# Add egress rule
echo "Adding egress rule..."
oci network nsg rules add \
  --nsg-id $NSG_ID \
  --security-rules '[{
    "description": "Allow all outbound",
    "direction": "EGRESS",
    "protocol": "all",
    "destination": "0.0.0.0/0",
    "destination-type": "CIDR_BLOCK",
    "is-stateless": false
  }]'

# Attach to instance
echo "Attaching NSG to instance..."
VNIC_ID=$(oci compute instance list-vnics \
  --instance-id $INSTANCE_ID \
  --query "data[0].id" \
  --raw-output)

oci network vnic update \
  --vnic-id $VNIC_ID \
  --nsg-ids '["'$NSG_ID'"]' \
  --force

echo ""
echo "✅ NSG configuration complete!"
echo ""
echo "NSG ID: $NSG_ID"
echo "VNIC ID: $VNIC_ID"
echo ""
echo "Configured rules:"
echo "  ✓ SSH (22) from $YOUR_IP/32"
echo "  ✓ P2P (30333) from 0.0.0.0/0"
echo "  ✓ All outbound traffic allowed"
```

**Usage:**
```bash
chmod +x setup-oci-firewall.sh
./setup-oci-firewall.sh
```

---

## 🔒 Additional Security Best Practices

### 1. **Use Bastion Host**

For production, consider OCI Bastion service:
```
Identity & Security → Bastion → Create Bastion
```

Benefits:
- No public SSH exposure
- Session recording
- Time-limited access
- Audit logging

### 2. **Enable Cloud Guard**

Monitor security violations:
```
Identity & Security → Cloud Guard → Enable
```

### 3. **Use Private Subnets**

For maximum security:
- Place validators in private subnet
- Use NAT Gateway for outbound
- Use Bastion for management

### 4. **Enable VCN Flow Logs**

Monitor network traffic:
```
Networking → Virtual Cloud Networks → Your VCN
→ Resources → Flow Logs → Enable
```

---

## 📊 Quick Reference Table

### OCI Resource OCIDs Format

| Resource | Format | Example |
|----------|--------|---------|
| Compartment | `ocid1.compartment.oc1..aaaa...` | Root or custom |
| VCN | `ocid1.vcn.oc1.phx.aaaa...` | Region-specific |
| NSG | `ocid1.networksecuritygroup.oc1.phx.aaaa...` | Region-specific |
| Instance | `ocid1.instance.oc1.phx.aaaa...` | Region-specific |
| VNIC | `ocid1.vnic.oc1.phx.aaaa...` | Region-specific |

### Finding OCIDs

```bash
# List compartments
oci iam compartment list --all

# List VCNs
oci network vcn list --compartment-id <COMPARTMENT_ID>

# List instances
oci compute instance list --compartment-id <COMPARTMENT_ID>

# Get instance VNIC
oci compute instance list-vnics --instance-id <INSTANCE_ID>
```

---

## 🆘 Troubleshooting

### Issue: Can't SSH to Instance

**Check:**
1. NSG has SSH rule from your IP
2. Security List allows SSH
3. Instance has public IP
4. SSH key is correct

**Fix:**
```bash
# Verify NSG rules
oci network nsg rules list --nsg-id $NSG_ID

# Check Security List
oci network security-list get --security-list-id <SECURITY_LIST_ID>

# Use Instance Console
# OCI Console → Compute → Instances → Instance Console Connection
```

### Issue: P2P Port Not Accessible

**Check:**
1. NSG has 30333 from 0.0.0.0/0
2. Security List allows 30333
3. Application is running and listening

**Fix:**
```bash
# Test locally on instance
sudo netstat -tulpn | grep 30333

# Add NSG rule if missing
oci network nsg rules add --nsg-id $NSG_ID --security-rules '[...]'
```

### Issue: NSG Not Attached

**Verify:**
```bash
oci network vnic get --vnic-id $VNIC_ID --query "data.\"nsg-ids\""
```

**Fix:**
```bash
oci network vnic update --vnic-id $VNIC_ID --nsg-ids '["'$NSG_ID'"]'
```

---

## 📖 Related Documentation

- **OCI Documentation:** https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/networksecuritygroups.htm
- **VALIDATOR_FIREWALL_RULES.md** - General firewall guide
- **BUILD_FIXES_SUMMARY.md** - Deployment overview
- **validator-deployment-kit/** - Multi-provider deployment

---

## Summary

**Oracle Cloud NSG Configuration for Validators:**

```yaml
Minimum Required Rules:
  Ingress:
    - TCP 22 from YOUR_IP/32 (SSH)
    - TCP 30333 from 0.0.0.0/0 (P2P - CRITICAL)

  Egress:
    - All protocols to 0.0.0.0/0

Best Practice:
  - Use NSGs instead of Security Lists
  - Keep rules minimal
  - Restrict SSH to your IP
  - Never block port 30333
  - Enable Cloud Guard monitoring
  - Use private subnets for production
```

**That's it!** Your Oracle Cloud validator is now properly secured. 🛡️
