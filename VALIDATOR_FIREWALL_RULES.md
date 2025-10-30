# Validator Firewall Rules & Security

**Date:** October 30, 2025
**Purpose:** Required network ingress rules for Ëtrid validator nodes

---

## 🔥 Required Firewall Rules

### Essential Ports (MUST OPEN)

| Port | Protocol | Purpose | Access | Priority |
|------|----------|---------|--------|----------|
| **30333** | TCP | P2P networking (libp2p) | 0.0.0.0/0 (public) | **CRITICAL** |
| **22** | TCP | SSH management | Your IP only | HIGH |

### Optional Ports (Conditional)

| Port | Protocol | Purpose | Access | When to Open |
|------|----------|---------|--------|--------------|
| **9944** | TCP/WS | WebSocket RPC | Restricted IPs | If exposing RPC |
| **9933** | TCP | HTTP RPC | Restricted IPs | If exposing RPC |
| **9615** | TCP | Prometheus metrics | Monitoring IP | If using monitoring |

---

## 🛡️ Security Configuration by Provider

### For Vultr (Your Current VM)

**Via Vultr Console:**

1. Go to: https://my.vultr.com/
2. Select your server (64.181.215.19)
3. Go to "Settings" → "Firewall"
4. Add these rules:

```
Priority 1: Allow SSH
- Protocol: TCP
- Port: 22
- Source: Your_IP_Address/32
- Notes: SSH access for management

Priority 2: Allow P2P (REQUIRED)
- Protocol: TCP
- Port: 30333
- Source: 0.0.0.0/0 (Anywhere)
- Notes: Validator P2P networking - MUST be public

Priority 3: Allow Ping (Optional)
- Protocol: ICMP
- Source: 0.0.0.0/0
- Notes: Network diagnostics
```

**Via UFW (on the VM):**

```bash
# SSH to your VM first
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Install UFW if not installed
sudo apt-get install -y ufw

# Set default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH (FIRST! or you'll lock yourself out)
sudo ufw allow 22/tcp comment 'SSH'

# Allow validator P2P port
sudo ufw allow 30333/tcp comment 'Validator P2P'

# Optional: Allow RPC from specific IP only
# sudo ufw allow from YOUR_IP to any port 9944 proto tcp comment 'RPC WS'
# sudo ufw allow from YOUR_IP to any port 9933 proto tcp comment 'RPC HTTP'

# Optional: Allow Prometheus from monitoring server
# sudo ufw allow from MONITORING_IP to any port 9615 proto tcp comment 'Metrics'

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status numbered
```

---

### For Hetzner

**Via Hetzner Cloud Console:**

1. Go to: https://console.hetzner.cloud/
2. Select "Firewalls" from left menu
3. Create new firewall: "etrid-validator-fw"
4. Add inbound rules:

```yaml
Inbound Rules:
  - Name: SSH
    Protocol: TCP
    Port: 22
    Source: Your_IP/32

  - Name: Validator P2P
    Protocol: TCP
    Port: 30333
    Source: 0.0.0.0/0, ::/0

  - Name: ICMP (optional)
    Protocol: ICMP
    Source: 0.0.0.0/0, ::/0
```

5. Apply firewall to all validator servers

**Via hcloud CLI:**

```bash
# Create firewall
hcloud firewall create --name etrid-validator-fw

# Add SSH rule
hcloud firewall add-rule etrid-validator-fw \
  --direction in \
  --protocol tcp \
  --port 22 \
  --source-ips YOUR_IP/32

# Add P2P rule
hcloud firewall add-rule etrid-validator-fw \
  --direction in \
  --protocol tcp \
  --port 30333 \
  --source-ips 0.0.0.0/0 \
  --source-ips ::/0

# Apply to server
hcloud firewall apply-to-resource etrid-validator-fw \
  --type server \
  --server gizzi-bootstrap-1
```

---

### For DigitalOcean

**Via DO Console:**

1. Go to: https://cloud.digitalocean.com/
2. Networking → Firewalls
3. Create Firewall: "etrid-validator-fw"
4. Inbound Rules:

```
SSH (TCP, 22, Your IP)
Custom (TCP, 30333, All IPv4, All IPv6)
```

**Via doctl:**

```bash
# Create firewall
doctl compute firewall create \
  --name etrid-validator-fw \
  --inbound-rules "protocol:tcp,ports:22,sources:addresses:YOUR_IP" \
  --inbound-rules "protocol:tcp,ports:30333,sources:addresses:0.0.0.0/0,::/0"

# Apply to droplet
doctl compute firewall add-droplets FIREWALL_ID --droplet-ids DROPLET_ID
```

---

## 🔒 Port Descriptions & Security

### Port 30333 - P2P Networking (CRITICAL)

**Purpose:**
- Peer discovery and connection
- Block propagation
- Transaction gossip
- Validator communication

**Security:**
- ✅ MUST be open to 0.0.0.0/0 (public internet)
- ✅ TCP only (not UDP)
- ❌ Cannot be restricted or validators won't sync
- ✅ This is normal and safe for Substrate validators

**Why public?**
Validators need to connect with other validators globally. Restricting this port will cause:
- ❌ Unable to sync blockchain
- ❌ Cannot receive blocks
- ❌ Cannot participate in consensus
- ❌ Network isolation

---

### Port 22 - SSH (HIGH PRIORITY)

**Purpose:**
- Remote server management
- Binary updates
- Log access
- Emergency maintenance

**Security:**
- ✅ ONLY allow from your IP address
- ✅ Use SSH keys (no passwords)
- ✅ Change default port (optional): `Port 2222`
- ✅ Disable root login (optional)

**Best practices:**
```bash
# Restrict to your IP in sshd_config
echo "Match Address YOUR_IP" | sudo tee -a /etc/ssh/sshd_config
echo "    PermitRootLogin yes" | sudo tee -a /etc/ssh/sshd_config
echo "Match Address !YOUR_IP" | sudo tee -a /etc/ssh/sshd_config
echo "    PermitRootLogin no" | sudo tee -a /etc/ssh/sshd_config

sudo systemctl restart sshd
```

---

### Port 9944/9933 - RPC Endpoints (CONDITIONAL)

**Purpose:**
- 9944: WebSocket RPC (for dApps, wallets)
- 9933: HTTP RPC (for queries)

**When to open:**
- ✅ If this is an RPC node (not just validator)
- ✅ If you need remote access to query blockchain
- ✅ If dApps need to connect

**When NOT to open:**
- ❌ For validators only (not needed)
- ❌ If you have separate RPC nodes
- ❌ Security best practice: keep closed on validators

**If you must open:**
```bash
# Restrict to specific IPs only
sudo ufw allow from YOUR_DAPP_IP to any port 9944
sudo ufw allow from YOUR_DAPP_IP to any port 9933

# Or use nginx reverse proxy with authentication
# Better: Use separate RPC node, not validator
```

**Security risk:**
- Open RPC = potential DoS attack vector
- Can expose validator to spam queries
- May leak validator IP address

**Recommendation:** Keep closed for validators, use separate RPC nodes

---

### Port 9615 - Prometheus Metrics (OPTIONAL)

**Purpose:**
- Expose metrics for monitoring (Grafana/Prometheus)
- Node health stats
- Block production metrics

**When to open:**
- ✅ If using monitoring stack
- ✅ Restrict to monitoring server IP only

**Configuration:**
```bash
# Only allow from monitoring server
sudo ufw allow from MONITORING_SERVER_IP to any port 9615

# Start validator with metrics enabled
/usr/local/bin/flarechain-node \
  --validator \
  --prometheus-external \
  --prometheus-port 9615
```

---

## 🎯 Recommended Configuration by Node Type

### Standard Validator (Most Secure)

```bash
# Only these ports:
✅ Port 22 (SSH) - Your IP only
✅ Port 30333 (P2P) - Public
❌ Port 9944 (RPC) - CLOSED
❌ Port 9933 (RPC) - CLOSED
❌ Port 9615 (Metrics) - CLOSED (or monitoring IP only)
```

**Rationale:** Minimal attack surface, maximum security

### Validator + Monitoring

```bash
✅ Port 22 (SSH) - Your IP only
✅ Port 30333 (P2P) - Public
✅ Port 9615 (Metrics) - Monitoring IP only
❌ Port 9944 (RPC) - CLOSED
❌ Port 9933 (RPC) - CLOSED
```

### RPC Node (NOT for validators)

```bash
✅ Port 22 (SSH) - Your IP only
✅ Port 30333 (P2P) - Public
✅ Port 9944 (RPC WS) - Public or restricted
✅ Port 9933 (RPC HTTP) - Public or restricted
✅ Port 9615 (Metrics) - Monitoring IP
```

**Note:** This is for separate RPC nodes, not validators!

---

## 🔐 Additional Security Measures

### 1. IP Whitelisting for SSH

```bash
# Only allow SSH from your IP
sudo ufw allow from YOUR_IP to any port 22
sudo ufw deny 22

# Or use /etc/hosts.allow and /etc/hosts.deny
echo "sshd: YOUR_IP" | sudo tee -a /etc/hosts.allow
echo "sshd: ALL" | sudo tee -a /etc/hosts.deny
```

### 2. Fail2Ban (Automatic IP Banning)

```bash
# Install fail2ban
sudo apt-get install -y fail2ban

# Configure
sudo cat > /etc/fail2ban/jail.local <<EOF
[sshd]
enabled = true
port = 22
filter = sshd
logpath = /var/log/auth.log
maxretry = 3
bantime = 3600
EOF

sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### 3. DDoS Protection

**For validators, consider:**
- Use cloud provider DDoS protection
- Hetzner: Built-in DDoS protection
- Vultr: DDoS protection included
- DigitalOcean: Cloud Firewalls

**Additional tools:**
```bash
# Install iptables rate limiting
sudo iptables -A INPUT -p tcp --dport 30333 -m state --state NEW -m recent --set
sudo iptables -A INPUT -p tcp --dport 30333 -m state --state NEW -m recent --update --seconds 10 --hitcount 20 -j DROP
```

### 4. Connection Limits

```bash
# Limit connections to validator
/usr/local/bin/flarechain-node \
  --validator \
  --in-peers 50 \
  --out-peers 25 \
  --max-parallel-downloads 5
```

---

## 🧪 Testing Firewall Configuration

### Test P2P Port (30333)

```bash
# From outside the network
telnet 64.181.215.19 30333

# Expected: Connection established
# If refused: Port not open
```

### Test SSH

```bash
# From your IP
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Should work
```

### Test RPC (should be closed for validators)

```bash
# From outside
curl http://64.181.215.19:9944

# Expected: Connection refused (good for validators!)
```

### Verify UFW Status

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 'sudo ufw status verbose'
```

**Expected output:**
```
Status: active
Logging: on (low)
Default: deny (incoming), allow (outgoing), disabled (routed)
New profiles: skip

To                         Action      From
--                         ------      ----
22/tcp                     ALLOW IN    YOUR_IP
30333/tcp                  ALLOW IN    Anywhere
```

---

## 📋 Quick Setup Script

Copy this to your VM:

```bash
#!/bin/bash
# setup-firewall.sh - Quick firewall setup for Ëtrid validator

set -e

echo "🔥 Setting up firewall for Ëtrid validator..."

# Your IP address
YOUR_IP="YOUR_IP_HERE"  # Replace with your actual IP

# Install UFW
sudo apt-get update
sudo apt-get install -y ufw fail2ban

# Reset UFW
sudo ufw --force reset

# Default policies
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH from your IP only
sudo ufw allow from $YOUR_IP to any port 22 proto tcp comment 'SSH'

# Allow P2P for all validators
sudo ufw allow 30333/tcp comment 'Validator P2P'

# Enable UFW
sudo ufw --force enable

# Show status
sudo ufw status numbered

echo "✅ Firewall configured!"
echo ""
echo "Allowed ports:"
echo "  - SSH (22) from $YOUR_IP only"
echo "  - P2P (30333) from anywhere"
echo ""
echo "Test connection: ssh ubuntu@$(hostname -I | awk '{print $1}')"
```

---

## ⚠️ Common Mistakes

### ❌ WRONG: Blocking Port 30333

```bash
# This will break your validator!
sudo ufw deny 30333
```

**Result:** Validator cannot sync, cannot participate in consensus

### ❌ WRONG: Opening RPC to public on validator

```bash
# Security risk on validators
sudo ufw allow 9944
sudo ufw allow 9933
```

**Result:** DDoS risk, information leak, potential attack vector

### ❌ WRONG: Forgetting to allow your IP before enabling UFW

```bash
sudo ufw enable  # Without allowing SSH first
```

**Result:** 🔒 Locked out of your own server!

**Always allow SSH first:**
```bash
sudo ufw allow 22  # Then enable
sudo ufw enable
```

---

## 🎯 Final Checklist

**Before starting validator:**

- [ ] Port 30333 open to public (0.0.0.0/0)
- [ ] Port 22 restricted to your IP only
- [ ] Ports 9944/9933 CLOSED (unless RPC node)
- [ ] UFW or cloud firewall enabled
- [ ] Fail2ban installed and configured
- [ ] SSH key authentication working
- [ ] Tested SSH connection from your IP
- [ ] Tested that other IPs cannot SSH
- [ ] Verified port 30333 accessible externally

**Test command:**
```bash
# From your machine
telnet 64.181.215.19 30333
# Should connect

ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
# Should work
```

---

## 🚀 Production Best Practices

1. **Always restrict SSH** to known IPs
2. **Keep RPC closed** on validators
3. **Monitor connections:** `sudo netstat -tulpn | grep LISTEN`
4. **Regular security updates:** `sudo apt-get update && sudo apt-get upgrade`
5. **Use VPN** for management access (optional but recommended)
6. **Enable audit logging:** `sudo apt-get install auditd`
7. **Monitor failed login attempts:** `sudo tail -f /var/log/auth.log`

---

## 📞 Emergency Access

If you lock yourself out:

1. **Use cloud provider console** (web-based terminal)
2. **Vultr:** Console access via web interface
3. **Hetzner:** KVM console via Robot panel
4. **DigitalOcean:** Droplet Console (Access tab)

Then fix firewall:
```bash
sudo ufw allow from YOUR_NEW_IP to any port 22
```

---

## Summary

**Minimal validator firewall:**
```
Port 22:    SSH (your IP only)
Port 30333: P2P (public - REQUIRED)
```

**That's it!** Keep it simple and secure. 🛡️
