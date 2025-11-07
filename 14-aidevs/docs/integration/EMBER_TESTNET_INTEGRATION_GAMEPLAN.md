# Ember Testnet Infrastructure - Integration Game Plan

**Created:** October 24, 2025
**Status:** 🟢 READY TO EXECUTE
**Timeline:** 10 weeks to Q1 2026 launch
**Your Role:** Eoj - Project Lead

---

## 📊 Current Status Assessment

### ✅ What EXISTS (Completed by GizziClaude)

**Documentation** (`/docs/deployment/`):
- ✅ EMBER_TESTNET_INFRASTRUCTURE_PLAN.md (comprehensive 10-week plan)
- ✅ Budget breakdown ($27K/year operational)
- ✅ Server specifications (validators, collators, RPC nodes)
- ✅ Security considerations
- ✅ Monitoring strategy
- ✅ Deployment timeline

### ❌ What's MISSING (Must Be Created)

**Infrastructure Code** (`/infrastructure/ansible/` - **DOES NOT EXIST**):
- ❌ Directory structure
- ❌ Ansible playbooks (provision-base, deploy-validator, setup-monitoring)
- ❌ Inventory files (ember-testnet.yml)
- ❌ Deployment scripts (quick-deploy.sh)
- ❌ Service templates (systemd units)
- ❌ Configuration files

**Cloud Resources** (Not Provisioned):
- ❌ Hetzner account & servers
- ❌ OVH account & backup servers
- ❌ DNS configuration (ember.etrid.org)
- ❌ SSL certificates

---

## 🎯 Integration Game Plan

### Phase 1: Infrastructure Code Creation (Week 0 - THIS WEEK)

**Priority: CRITICAL** - Must complete before server provisioning

#### Step 1.1: Create Directory Structure
```bash
mkdir -p /Users/macbook/Desktop/etrid/infrastructure/ansible/{playbooks,inventory,templates,scripts,roles}
mkdir -p /Users/macbook/Desktop/etrid/infrastructure/ansible/group_vars
mkdir -p /Users/macbook/Desktop/etrid/infrastructure/ansible/host_vars
```

#### Step 1.2: Create Ansible Inventory
**File:** `infrastructure/ansible/inventory/ember-testnet.yml`

**Content:**
```yaml
all:
  vars:
    ansible_user: root
    ansible_ssh_private_key_file: ~/.ssh/etrid_ember
    ansible_python_interpreter: /usr/bin/python3

    # Network configuration
    p2p_port: 30333
    rpc_port: 9933
    ws_port: 9944
    substrate_metrics_port: 9615
    node_exporter_port: 9100
    prometheus_port: 9090
    grafana_port: 3000

    # Chain configuration
    chain_name: "ember"
    telemetry_url: "wss://telemetry.polkadot.io/submit/"

validators:
  hosts:
    validator1:
      ansible_host: 0.0.0.0  # UPDATE after provisioning
      validator_name: "Etrid-Validator-1"
      validator_index: 1
    validator2:
      ansible_host: 0.0.0.0
      validator_name: "Etrid-Validator-2"
      validator_index: 2
    validator3:
      ansible_host: 0.0.0.0
      validator_name: "Etrid-Validator-3"
      validator_index: 3

backup_validators:
  hosts:
    backup_validator1:
      ansible_host: 0.0.0.0
      validator_name: "Etrid-Backup-Validator-1"
    backup_validator2:
      ansible_host: 0.0.0.0
      validator_name: "Etrid-Backup-Validator-2"

rpc_nodes:
  hosts:
    rpc1:
      ansible_host: 0.0.0.0
    rpc2:
      ansible_host: 0.0.0.0

monitoring:
  hosts:
    monitoring1:
      ansible_host: 0.0.0.0

explorer:
  hosts:
    explorer1:
      ansible_host: 0.0.0.0
```

#### Step 1.3: Create Ansible Playbooks

**Playbook 1:** `playbooks/01-provision-base.yml`
- Update system packages
- Install essential tools (curl, git, build-essential)
- Configure swap (4GB)
- Harden SSH (disable password auth)
- Setup UFW firewall
- Configure fail2ban
- Install Rust toolchain
- Create etrid user
- Setup log rotation

**Playbook 2:** `playbooks/02-deploy-validator.yml`
- Deploy Substrate binary
- Generate chain specification
- Create node keys
- Generate session keys
- Setup systemd service
- Configure monitoring (Prometheus)
- Install Node Exporter
- Create backup scripts

**Playbook 3:** `playbooks/03-setup-monitoring.yml`
- Install Prometheus
- Configure Grafana
- Setup AlertManager
- Create dashboards
- Configure alerts (Slack, PagerDuty)

#### Step 1.4: Create Quick Deploy Script
**File:** `infrastructure/ansible/quick-deploy.sh`

**Features:**
- Check prerequisites (Ansible, SSH keys)
- Verify connectivity to servers
- Run playbooks in sequence
- Display deployment status
- Save session keys securely

#### Step 1.5: Create Service Templates
**File:** `templates/etrid-validator.service.j2`

Systemd service for running validators with:
- Auto-restart on failure
- Resource limits
- Security hardening
- Log management

---

### Phase 2: Cloud Account Setup (Week 1)

#### Step 2.1: Open Hetzner Account (DAY 1)
**Actions:**
1. Go to: https://www.hetzner.com/cloud
2. Create account: eoj@etrid.network
3. Enable 2FA immediately
4. Add payment method
5. Create project: "etrid-ember-testnet"
6. Generate API token (save in 1Password/Vault)
7. Request server limit increase (15+ servers)

**Expected Time:** 2 hours (including approval wait)

#### Step 2.2: Open OVH Account (DAY 1)
**Actions:**
1. Go to: https://www.ovhcloud.com/
2. Create account with 2FA
3. Select US region (geographic diversity)
4. Add payment method
5. Generate API credentials

**Expected Time:** 1 hour

#### Step 2.3: Generate SSH Keys (DAY 1)
```bash
# Generate key pair
ssh-keygen -t ed25519 -C "etrid-ember-testnet" -f ~/.ssh/etrid_ember

# Add to agent
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/etrid_ember

# Upload public key to Hetzner & OVH
cat ~/.ssh/etrid_ember.pub
```

---

### Phase 3: Server Provisioning (Week 1-2)

#### Step 3.1: Provision Hetzner Servers

**Install Hetzner CLI:**
```bash
# macOS
brew install hcloud

# Configure
hcloud context create ember-testnet
# Enter API token when prompted
```

**Provision Validators:**
```bash
# Create provisioning script
cat > provision-hetzner.sh << 'EOF'
#!/bin/bash
SSH_KEY_ID=$(hcloud ssh-key list -o columns=id -o noheader | head -1)

# 3 Validators (CPX51: 16 vCPU, 32GB RAM)
for i in {1..3}; do
  hcloud server create \
    --name validator$i \
    --type cpx51 \
    --image ubuntu-22.04 \
    --ssh-key $SSH_KEY_ID \
    --location fsn1
done

# 2 RPC Nodes (CPX41: 8 vCPU, 16GB RAM)
for i in {1..2}; do
  hcloud server create \
    --name rpc$i \
    --type cpx41 \
    --image ubuntu-22.04 \
    --ssh-key $SSH_KEY_ID \
    --location fsn1
done

# Monitoring (CPX31: 8 vCPU, 16GB RAM)
hcloud server create \
  --name monitoring1 \
  --type cpx31 \
  --image ubuntu-22.04 \
  --ssh-key $SSH_KEY_ID \
  --location fsn1

# Explorer (CPX31)
hcloud server create \
  --name explorer1 \
  --type cpx31 \
  --image ubuntu-22.04 \
  --ssh-key $SSH_KEY_ID \
  --location fsn1

echo "Provisioning complete! Waiting 60s for boot..."
sleep 60
hcloud server list
EOF

chmod +x provision-hetzner.sh
./provision-hetzner.sh
```

#### Step 3.2: Update Inventory
```bash
# Get server IPs
hcloud server list -o columns=name,ipv4

# Update inventory file
vi infrastructure/ansible/inventory/ember-testnet.yml
# Replace all 0.0.0.0 with actual IPs
```

#### Step 3.3: Test Connectivity
```bash
cd infrastructure/ansible
ansible all -i inventory/ember-testnet.yml -m ping
```

---

### Phase 4: Base Infrastructure Deployment (Week 2)

#### Step 4.1: Run Base Provisioning
```bash
cd infrastructure/ansible
ansible-playbook -i inventory/ember-testnet.yml \
  playbooks/01-provision-base.yml

# This will:
# ✅ Update packages
# ✅ Install tools
# ✅ Harden security
# ✅ Setup firewall
# ✅ Install Rust
# ✅ Create etrid user
```

**Expected Time:** 15-20 minutes

#### Step 4.2: Verify Security
```bash
# Check firewall
ansible all -i inventory/ember-testnet.yml -a "ufw status"

# Check fail2ban
ansible all -i inventory/ember-testnet.yml -a "systemctl status fail2ban"

# Verify SSH hardening
ansible all -i inventory/ember-testnet.yml -a "grep PasswordAuthentication /etc/ssh/sshd_config"
```

---

### Phase 5: Build & Deploy Etrid Node (Week 3)

#### Step 5.1: Build Substrate Node
```bash
cd /Users/macbook/Desktop/etrid

# Build optimized release
cargo build --release --locked

# Verify binary
./target/release/etrid --version
```

**Expected Time:** 30-60 minutes (first build)

#### Step 5.2: Generate Chain Specification
```bash
# Generate chain spec
./target/release/etrid build-spec --chain staging \
  --disable-default-bootnode > ember-testnet-spec.json

# Customize genesis (edit JSON)
# - Set initial validators
# - Configure token economics
# - Set governance parameters

# Generate raw chain spec
./target/release/etrid build-spec \
  --chain ember-testnet-spec.json \
  --raw > ember-testnet-raw.json
```

#### Step 5.3: Deploy First Validator
```bash
cd infrastructure/ansible

# Upload binary to validator1
scp ../target/release/etrid validator1:/opt/etrid/

# Upload chain spec
ansible all -i inventory/ember-testnet.yml \
  -m copy \
  -a "src=ember-testnet-raw.json dest=/opt/etrid/ember-testnet.json owner=etrid"

# Deploy validator
ansible-playbook -i inventory/ember-testnet.yml \
  playbooks/02-deploy-validator.yml \
  --limit validator1

# CRITICAL: Save session keys displayed!
```

**Expected Time:** 20 minutes

---

### Phase 6: Deploy Validator Network (Week 3-4)

#### Step 6.1: Get Bootnode Address
```bash
ssh validator1
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' \
     http://localhost:9933

# Construct bootnode:
# /ip4/<VALIDATOR1_IP>/tcp/30333/p2p/<PEER_ID>
```

#### Step 6.2: Update Inventory with Bootnode
```bash
vi inventory/ember-testnet.yml

# Add:
all:
  vars:
    bootnodes:
      - "/ip4/X.X.X.X/tcp/30333/p2p/12D3KooW..."
```

#### Step 6.3: Deploy Remaining Validators
```bash
ansible-playbook -i inventory/ember-testnet.yml \
  playbooks/02-deploy-validator.yml \
  --limit validator2,validator3

# Deploy OVH backups
ansible-playbook -i inventory/ember-testnet.yml \
  playbooks/02-deploy-validator.yml \
  --limit backup_validators
```

#### Step 6.4: Verify Network
```bash
# Check peer connections
ansible validators -i inventory/ember-testnet.yml \
  -m shell \
  -a "curl -s http://localhost:9933 -H 'Content-Type: application/json' \
      -d '{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"id\":1}' | jq"

# Expected: {"peers": 2+, "isSyncing": false}

# Check block production
ansible validators -i inventory/ember-testnet.yml \
  -m shell \
  -a "curl -s http://localhost:9933 -H 'Content-Type: application/json' \
      -d '{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"id\":1}' | jq .result.number"
```

---

### Phase 7: Monitoring Deployment (Week 4)

#### Step 7.1: Deploy Monitoring Stack
```bash
ansible-playbook -i inventory/ember-testnet.yml \
  playbooks/03-setup-monitoring.yml
```

#### Step 7.2: Access Grafana
```
URL: http://<MONITORING_IP>:3000
Login: admin / CHANGE_ME_IMMEDIATELY

Actions:
1. Change admin password IMMEDIATELY
2. Import Substrate dashboards (ID: 13759)
3. Import Node Exporter dashboard (ID: 1860)
4. Configure alerts (Slack webhook)
```

---

### Phase 8: RPC & Explorer (Week 5-6)

**Note:** RPC and Explorer playbooks need to be created (not in original plan).

#### Step 8.1: Create RPC Playbook
Similar to validator playbook but:
- No validator keys
- Archive mode enabled
- Higher connection limits
- Public-facing

#### Step 8.2: Deploy Block Explorer
Options:
- **Subscan** (commercial, $500-1000/month)
- **Polkadot.js Apps** (free, self-hosted)
- **Custom** (Substrate API Sidecar + React)

**Recommendation:** Start with Polkadot.js Apps

---

## 🔄 Mainnet Reusability Assessment

### ✅ Components HIGHLY REUSABLE (90%+ code reuse)

#### 1. Ansible Playbooks
**Reusability: 95%**

What transfers:
- ✅ Base provisioning (security, firewall, SSH hardening)
- ✅ Service templates (systemd units)
- ✅ Monitoring setup (Prometheus, Grafana)
- ✅ Backup procedures
- ✅ Log rotation and management

What needs modification:
- ⚠️ Chain specification (mainnet genesis)
- ⚠️ Validator key management (more secure for mainnet)
- ⚠️ Network topology (more validators, geographic distribution)

#### 2. Infrastructure Architecture
**Reusability: 80%**

What transfers:
- ✅ Server specifications (validators, RPC, monitoring)
- ✅ Network topology (validators → RPC → load balancer)
- ✅ Monitoring dashboards and alerts
- ✅ Backup and recovery procedures

What needs enhancement:
- ⚠️ Scale up validators (5 testnet → 50+ mainnet)
- ⚠️ Add HSM for key management
- ⚠️ Implement multi-region RPC (CDN)
- ⚠️ Add DDoS protection (Cloudflare Enterprise)

#### 3. Operational Procedures
**Reusability: 90%**

What transfers:
- ✅ Deployment procedures
- ✅ Health checks
- ✅ Incident response workflows
- ✅ Backup strategies
- ✅ Monitoring dashboards

What needs enhancement:
- ⚠️ Add security audit requirements
- ⚠️ Implement key rotation automation
- ⚠️ Add formal change management
- ⚠️ Create disaster recovery drills

### ⚠️ Components Requiring MODIFICATION (50-80% reuse)

#### 1. Security Configuration
**Reusability: 60%**

**Testnet:**
- SSH keys managed manually
- Basic firewall (UFW)
- Self-signed SSL certificates
- Standard monitoring

**Mainnet Enhancements:**
- ✅ HSM integration (YubiKey, AWS CloudHSM)
- ✅ VPN mesh between validators (WireGuard)
- ✅ Commercial SSL (DigiCert, Comodo)
- ✅ SIEM integration (Splunk, ELK)
- ✅ Intrusion detection (OSSEC, Wazuh)
- ✅ Regular penetration testing
- ✅ Bug bounty program

#### 2. Key Management
**Reusability: 40%**

**Testnet:**
- Keys generated on validator nodes
- Stored in `/var/lib/etrid/keystore`
- Backed up to encrypted storage

**Mainnet Requirements:**
- ✅ Multi-signature key generation ceremonies
- ✅ Hardware security modules (Ledger, YubiHSM)
- ✅ Key sharding (Shamir's Secret Sharing)
- ✅ Offline cold storage for stash keys
- ✅ Formal key rotation procedures
- ✅ Audit trail for all key operations

#### 3. Network Configuration
**Reusability: 70%**

**Testnet:**
- 3 validators, 2 RPC nodes
- Single region (EU-Central)
- Basic load balancing (HAProxy)

**Mainnet Scale-Up:**
- ✅ 50-100+ validators (Foundation + community)
- ✅ Multi-region RPC (US, EU, Asia, LatAm)
- ✅ Global CDN (Cloudflare, Fastly)
- ✅ Dedicated archive nodes per region
- ✅ High-availability load balancers
- ✅ DDoS mitigation (enterprise-grade)

### ❌ Components Requiring COMPLETE REWRITE

#### 1. Governance & Compliance
**Reusability: 0%** (new for mainnet)

**Mainnet Requirements:**
- Legal compliance (jurisdiction analysis)
- KYC/AML for Foundation validators
- Regulatory reporting
- Data privacy (GDPR, CCPA)
- Terms of service and legal notices

#### 2. Token Economics
**Reusability: 0%** (testnet uses valueless tokens)

**Mainnet Requirements:**
- Real token distribution
- Staking economics (inflation, rewards)
- Treasury management
- Burn mechanisms
- Economic audits

#### 3. Community Governance
**Reusability: 30%** (testnet governance is simulated)

**Mainnet Requirements:**
- On-chain governance (referenda, council)
- Voting mechanisms
- Proposal submission
- Treasury spending
- Upgrade procedures

---

## 📊 Reusability Summary

| Component | Testnet Code | Mainnet Reuse | Modifications Needed |
|-----------|--------------|---------------|---------------------|
| Ansible Playbooks | 100% | 95% | Chain spec, key management |
| Infrastructure | 100% | 80% | Scale, security hardening |
| Monitoring | 100% | 90% | More metrics, SLA tracking |
| Operational Procedures | 100% | 90% | Formal processes, audits |
| Security Configuration | 100% | 60% | HSM, VPN, SIEM, pen testing |
| Key Management | 100% | 40% | Hardware keys, ceremonies |
| Network Configuration | 100% | 70% | Multi-region, CDN, DDoS |
| RPC Infrastructure | 100% | 80% | Scale, rate limiting |
| Block Explorer | 100% | 100% | Direct reuse |
| Documentation | 100% | 80% | Mainnet-specific guidance |

**Overall Reusability: 75-80%**

---

## 💰 Cost Comparison: Testnet vs Mainnet

### Testnet (Ember)
```
Monthly: $2,098
Annual: ~$27,000

Breakdown:
- 3 validators (Foundation)
- 13 collators (PBCs)
- 2 RPC nodes
- Basic monitoring
- Basic security
```

### Mainnet (Projected)
```
Monthly: $8,000 - $12,000
Annual: ~$100,000 - $150,000

Breakdown:
- 10 Foundation validators (global distribution)
- 50+ community validators (incentivized)
- 10 RPC nodes (multi-region CDN)
- 13 collators (PBCs)
- Enterprise monitoring (PagerDuty, Datadog)
- HSM key management
- Security audits (quarterly)
- Bug bounty program
- Legal/compliance
- DDoS protection (Cloudflare Enterprise)
```

**Cost Multiplier: 4-6x from testnet to mainnet**

---

## ⏱️ Timeline Comparison

### Testnet Deployment
```
Weeks 1-2:   Cloud setup, provisioning
Weeks 3-4:   Validator deployment
Weeks 5-6:   RPC, Explorer
Weeks 7-8:   Documentation
Weeks 9-10:  Testing, launch prep

Total: 10 weeks
```

### Mainnet Deployment (Projected)
```
Month 1-2:   Infrastructure scaling
Month 3:     Security audits
Month 4:     Key ceremonies, governance setup
Month 5:     Community validator onboarding
Month 6:     Final audits, launch prep

Total: 6 months (with testnet learnings applied)
```

**Without testnet experience: 9-12 months**

**Testnet saves: 3-6 months on mainnet timeline**

---

## 🎯 Immediate Action Items for Eoj

### Today (Next 4 Hours)
1. ✅ Review this game plan
2. ✅ Approve budget ($27K/year testnet)
3. ✅ Open Hetzner account
4. ✅ Open OVH account
5. ✅ Generate SSH keys

### This Week (Next 7 Days)
6. ✅ Create infrastructure/ansible directory structure
7. ✅ Create Ansible inventory file
8. ✅ Create playbook 01-provision-base.yml
9. ✅ Create playbook 02-deploy-validator.yml
10. ✅ Create playbook 03-setup-monitoring.yml
11. ✅ Create quick-deploy.sh script
12. ✅ Provision Hetzner servers
13. ✅ Update inventory with real IPs

### Week 2 (Days 8-14)
14. ✅ Run base provisioning playbook
15. ✅ Build Etrid Substrate node
16. ✅ Generate chain specification
17. ✅ Deploy first validator
18. ✅ Verify block production

### Week 3-4
19. ✅ Deploy full validator network (3 validators)
20. ✅ Deploy monitoring stack
21. ✅ Verify network producing blocks
22. ✅ Deploy backup validators (OVH)

---

## ✅ Success Criteria

### Week 2 Checkpoint
- [ ] All servers provisioned and reachable
- [ ] Base security configured (UFW, fail2ban, SSH)
- [ ] Monitoring stack operational
- [ ] First validator producing blocks

### Week 4 Checkpoint
- [ ] 3 validators operational
- [ ] Network achieving finality
- [ ] Grafana dashboards showing metrics
- [ ] Backup validators synced

### Week 10 Launch
- [ ] 99%+ uptime over 2 weeks
- [ ] RPC endpoints public and stable
- [ ] Block explorer functional
- [ ] Documentation complete
- [ ] Community ready to participate

---

## 🚨 Critical Decisions Needed

### Decision 1: Budget Approval
**Question:** Approve $27K/year for Ember testnet?

**Options:**
- ✅ Full deployment ($2,098/month)
- ⚠️ Budget tier ($1,318/month - 2 validators, 10 collators)
- ❌ Delay until funding secured

**Recommendation:** Approve full deployment. Testnet is critical for mainnet preparation.

### Decision 2: Cloud Provider
**Question:** Use Hetzner + OVH as planned?

**Alternatives:**
- AWS (4x more expensive, better support)
- DigitalOcean (2x more expensive, easier)
- Stick with Hetzner/OVH (best price/performance)

**Recommendation:** Hetzner + OVH. Cost-effective for testnet, can migrate mainnet to AWS if needed.

### Decision 3: Timeline Commitment
**Question:** Commit to 10-week timeline?

**Considerations:**
- Aggressive but achievable with automation
- Requires immediate start (this week)
- Missing Q1 2026 delays mainnet 3-6 months

**Recommendation:** Commit to 10 weeks. Use Ansible automation to accelerate.

---

## 📞 Support & Next Steps

### Need Help With?
- Creating Ansible playbooks → I can help!
- Provisioning servers → Follow game plan
- Building Substrate node → Check Rust installation
- Deployment issues → Detailed troubleshooting available

### Questions to Ask Me:
1. "Create the Ansible playbook for base provisioning"
2. "Create the Ansible playbook for validator deployment"
3. "Create the quick-deploy.sh script"
4. "Help me provision Hetzner servers"
5. "Troubleshoot validator connectivity issues"

---

## 🎉 Why This Will Work

### 1. Comprehensive Plan
GizziClaude created excellent documentation. You have a clear roadmap.

### 2. Automation First
Ansible playbooks make deployment repeatable and reliable.

### 3. Proven Architecture
Following Substrate/Polkadot best practices.

### 4. Mainnet-Ready Foundation
75-80% code reuse for mainnet saves months of work.

### 5. You Have Support
I'm here to help create all the missing pieces.

---

## 🚀 Let's Get Started!

**Your first command:**
```bash
# Open Hetzner account
open https://www.hetzner.com/cloud

# After account created, install CLI
brew install hcloud
```

**Then come back and say:**
"Claude, create the Ansible playbooks for me"

And we'll build this together! 💪

---

**Document Version:** 1.0
**Status:** 🟢 READY TO EXECUTE
**Next Review:** After Week 2 (validator deployment)
