# Ëtrid Ember Testnet - Infrastructure Deployment Plan

**Document ID**: ETRID-INFRA-EMBER-2025
**Status**: DRAFT - Ready for Implementation
**Target Launch**: Q1 2026 (January-March)
**Owner**: Infrastructure Team
**Last Updated**: October 24, 2025

---

## 📋 EXECUTIVE SUMMARY

This document outlines the complete infrastructure deployment plan for the **Ëtrid Ember Testnet**, the first public test network for the Ëtrid Protocol.

### Objectives:
1. Deploy production-ready testnet infrastructure
2. Support 50-100 validators (target: 150 by month 3)
3. Achieve 99.9% uptime
4. Provide developer-friendly RPC endpoints
5. Enable community participation and testing

### Timeline: 10 Weeks (Nov 2025 → Jan 2026)

### Budget: $15,000-20,000 initial + $2,000-3,000/month operational

---

## 🏗️ INFRASTRUCTURE ARCHITECTURE

### Network Topology

```
┌─────────────────────────────────────────────────────────────┐
│                    Ëtrid Ember Testnet                      │
└─────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                   FLARECHAIN (Root Chain)                    │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Validator 1 │  │ Validator 2 │  │ Validator 3 │  ...   │
│  │ (Foundation)│  │ (Foundation)│  │ (Community) │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│          ↓                ↓                ↓               │
│  ┌──────────────────────────────────────────────┐         │
│  │         RPC Load Balancer (HAProxy)           │         │
│  │  • Public RPC: rpc.ember.etrid.org           │         │
│  │  • WebSocket: wss://ember.etrid.org          │         │
│  └──────────────────────────────────────────────┘         │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│            PARTITION BURST CHAINS (13 PBCs)                  │
│                                                              │
│  BTC-PBC  ETH-PBC  DOGE-PBC  SOL-PBC  ...  EDSC-PBC        │
│  (Collator) (Collator) (Collator) (Collator) ... (Collator) │
│                                                              │
│  Each PBC runs on separate collator node                    │
│  Each syncs with FlareChain via state proofs                │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                  SUPPORTING SERVICES                         │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │ Block Explorer│  │   Faucet     │  │  Monitoring  │     │
│  │   (Subscan)  │  │   Service    │  │ (Prometheus) │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Grafana    │  │  Portal UI   │  │   Archive    │     │
│  │  Dashboard   │  │   Website    │  │     Node     │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└──────────────────────────────────────────────────────────────┘
```

---

## 🖥️ SERVER SPECIFICATIONS

### 1. Validator Nodes (3 Foundation + Community)

**Role**: Produce and finalize blocks for FlareChain

**Specifications** (per validator):
```yaml
CPU: 8 cores (16 threads) - AMD EPYC or Intel Xeon
RAM: 32 GB DDR4 ECC
Storage: 1 TB NVMe SSD (with daily snapshots)
Network: 1 Gbps dedicated bandwidth
Location: Distributed (US, EU, Asia)
OS: Ubuntu 22.04 LTS Server
```

**Software Stack**:
```bash
# Node binary
etrid-node v0.9.x (Ember release)

# Systemd service
etrid-validator.service

# Monitoring agents
node_exporter (Prometheus)
filebeat (log shipping)
```

**Estimated Cost** (per validator):
- **Hetzner EX102**: €99/month (~$110/month)
- **OVH Advance-4**: $120/month
- **Total for 3**: ~$330-360/month

### 2. Collator Nodes (13 PBCs)

**Role**: Produce blocks for each Partition Burst Chain

**Specifications** (per collator):
```yaml
CPU: 4 cores (8 threads)
RAM: 16 GB DDR4
Storage: 500 GB NVMe SSD
Network: 1 Gbps bandwidth
Location: Co-located with validators (latency optimization)
OS: Ubuntu 22.04 LTS Server
```

**PBC Assignments**:
1. BTC-PBC Collator
2. ETH-PBC Collator
3. DOGE-PBC Collator
4. SOL-PBC Collator
5. XLM-PBC Collator
6. XRP-PBC Collator
7. BNB-PBC Collator
8. TRX-PBC Collator
9. ADA-PBC Collator
10. LINK-PBC Collator
11. MATIC-PBC Collator
12. USDT-PBC Collator
13. EDSC-PBC Collator

**Estimated Cost** (per collator):
- **Hetzner EX52**: €59/month (~$65/month)
- **Total for 13**: ~$845/month

### 3. RPC Nodes (2 public endpoints)

**Role**: Serve RPC requests from users, wallets, dApps

**Specifications** (per RPC node):
```yaml
CPU: 8 cores
RAM: 32 GB
Storage: 1 TB NVMe SSD
Network: 10 Gbps bandwidth (high traffic expected)
Location: Behind load balancer (HAProxy or Nginx)
OS: Ubuntu 22.04 LTS Server
```

**Endpoints**:
```
HTTP RPC:  https://rpc.ember.etrid.org
WebSocket: wss://ws.ember.etrid.org
Archive:   https://archive.ember.etrid.org (optional, slower but complete history)
```

**Rate Limiting**:
```yaml
Free tier: 100 requests/second per IP
Authenticated: 1,000 requests/second (with API key)
Archive queries: 10 requests/second (expensive)
```

**Estimated Cost** (per RPC node):
- **Hetzner AX102**: €149/month (~$165/month)
- **Total for 2**: ~$330/month

### 4. Archive Node (1 optional)

**Role**: Store complete historical state for explorers, analytics

**Specifications**:
```yaml
CPU: 8 cores
RAM: 64 GB
Storage: 4 TB NVMe SSD (grows over time)
Network: 1 Gbps
Location: Same region as block explorer
```

**Estimated Cost**:
- **Hetzner AX162**: €279/month (~$310/month)

### 5. Monitoring Server (1)

**Role**: Run Prometheus, Grafana, Alertmanager

**Specifications**:
```yaml
CPU: 4 cores
RAM: 16 GB
Storage: 500 GB SSD (time-series data)
Network: 1 Gbps
```

**Stack**:
```yaml
Services:
  - Prometheus (metrics collection)
  - Grafana (dashboards)
  - Alertmanager (alerts)
  - Loki (log aggregation)
  - PagerDuty integration (on-call alerts)
```

**Estimated Cost**:
- **Hetzner EX52**: €59/month (~$65/month)

### 6. Block Explorer Server (1)

**Role**: Host Subscan or Polkadot.js-based explorer

**Specifications**:
```yaml
CPU: 8 cores
RAM: 32 GB
Storage: 1 TB SSD
Network: 1 Gbps
Database: PostgreSQL (separate or same server)
```

**Software**:
- **Option A**: Subscan (commercial, hosted)
- **Option B**: Polkadot.js Explorer (self-hosted, free)
- **Option C**: Custom explorer (using Polkadot.js API)

**Estimated Cost**:
- **Self-hosted**: €99/month (~$110/month)
- **Subscan (hosted)**: $500-1,000/month (commercial service)

**Recommendation**: Start with self-hosted Polkadot.js, upgrade to Subscan at mainnet

### 7. Faucet Service (1)

**Role**: Distribute testnet ÉTR to developers

**Specifications**:
```yaml
CPU: 2 cores
RAM: 4 GB
Storage: 50 GB SSD
Network: 100 Mbps
```

**Features**:
```yaml
Distribution:
  - Amount: 1,000 ÉTR per request
  - Cooldown: 24 hours per address
  - Rate limit: 10 requests/minute per IP
  - Captcha: hCaptcha or reCAPTCHA
  - GitHub OAuth: Optional (prevents Sybil)
```

**Estimated Cost**:
- **Hetzner CX31**: €7.49/month (~$8/month)

---

## 💰 TOTAL COST BREAKDOWN

### Initial Setup Costs (One-Time)

| Item | Cost | Notes |
|------|------|-------|
| Server provisioning | $500 | Setup fees, initial deposits |
| SSL certificates | $200/year | Wildcard cert (*.ember.etrid.org) |
| Domain registration | $50/year | ember.etrid.org subdomain |
| Initial snapshot storage | $200 | 3 months of daily backups |
| Load balancer setup | $100 | HAProxy configuration |
| Monitoring setup | $200 | Grafana dashboards, alert rules |
| **Total Initial** | **~$1,250** | |

### Monthly Operational Costs

| Item | Quantity | Unit Cost | Total |
|------|----------|-----------|-------|
| Validator nodes | 3 | $110 | $330 |
| Collator nodes | 13 | $65 | $845 |
| RPC nodes | 2 | $165 | $330 |
| Archive node | 1 | $310 | $310 |
| Monitoring server | 1 | $65 | $65 |
| Block explorer | 1 | $110 | $110 |
| Faucet service | 1 | $8 | $8 |
| Backup storage | 1 | $100 | $100 |
| **Monthly Total** | | | **$2,098** |

### Annual Cost Projection

```
Month 1 (Initial):  $1,250 (setup) + $2,098 (ops) = $3,348
Months 2-12:        $2,098 × 11 = $23,078
Year 1 Total:       ~$26,426

Rounded estimate: $27,000/year for full testnet infrastructure
```

### Cost Optimization Options

**Budget Tier (Minimum viable)**:
- 2 validators (instead of 3): -$110/month
- 10 collators (instead of 13): -$195/month
- 1 RPC node (instead of 2): -$165/month
- No archive node: -$310/month
- **Savings**: ~$780/month → **$1,318/month minimum**

**Premium Tier (Production-ready)**:
- 5 validators: +$220/month
- 3 RPC nodes: +$165/month
- Subscan explorer: +$500/month
- CDN for website: +$50/month
- **Total**: ~$3,033/month

---

## 📅 DEPLOYMENT TIMELINE (10 Weeks)

### Week 1-2: Planning & Provisioning (Nov 1-15)

**Week 1 (Nov 1-8)**:
- [ ] Finalize infrastructure design
- [ ] Create deployment scripts (Ansible, Terraform)
- [ ] Open accounts with Hetzner/OVH
- [ ] Order servers (3 validators, 13 collators, 2 RPC)
- [ ] Set up DNS records (*.ember.etrid.org)
- [ ] Generate SSL certificates (Let's Encrypt or commercial)

**Week 2 (Nov 9-15)**:
- [ ] Receive and configure servers
- [ ] Install Ubuntu 22.04 on all nodes
- [ ] Set up SSH keys and firewall rules
- [ ] Install monitoring agents (node_exporter, filebeat)
- [ ] Deploy Prometheus + Grafana on monitoring server
- [ ] Test internal connectivity

**Deliverables**:
- All servers provisioned and reachable
- Monitoring stack operational
- DNS configured

---

### Week 3-4: Node Deployment (Nov 16-30)

**Week 3 (Nov 16-22)**:
- [ ] Build `etrid-node` binary for Ember testnet
- [ ] Deploy binary to all validator and collator nodes
- [ ] Generate validator keys (session keys, stash/controller)
- [ ] Create genesis state (chain spec)
- [ ] Deploy systemd services
- [ ] Start validator nodes in sync mode

**Week 4 (Nov 23-30)**:
- [ ] Start collator nodes for all 13 PBCs
- [ ] Verify block production (FlareChain + PBCs)
- [ ] Deploy RPC nodes (archive sync)
- [ ] Configure HAProxy load balancer
- [ ] Test RPC endpoints (curl, Polkadot.js)
- [ ] Enable telemetry (optional)

**Deliverables**:
- FlareChain producing blocks
- 13 PBCs operational
- RPC endpoints responding

---

### Week 5-6: Block Explorer & Faucet (Dec 1-15)

**Week 5 (Dec 1-8)**:
- [ ] Deploy Polkadot.js Explorer
- [ ] Configure chain metadata (types, RPC methods)
- [ ] Test block browsing, account lookups
- [ ] Deploy PostgreSQL for indexed data
- [ ] Set up archive node connection
- [ ] Deploy explorer frontend

**Week 6 (Dec 9-15)**:
- [ ] Build faucet service (Node.js or Rust)
- [ ] Deploy faucet smart contract (or use keyring)
- [ ] Configure rate limiting (Redis)
- [ ] Add hCaptcha for Sybil resistance
- [ ] Test faucet distribution
- [ ] Document faucet usage

**Deliverables**:
- Block explorer live at explorer.ember.etrid.org
- Faucet operational at faucet.ember.etrid.org

---

### Week 7-8: Portal & Documentation (Dec 16-31)

**Week 7 (Dec 16-22)**:
- [ ] Deploy Ember testnet portal website
  - What is Ember?
  - How to run a validator
  - RPC endpoints
  - Faucet instructions
  - Known issues
- [ ] Write validator documentation
- [ ] Create community Discord/Telegram
- [ ] Set up status page (status.ember.etrid.org)

**Week 8 (Dec 23-31)**:
- [ ] Create video tutorial (running a validator)
- [ ] Write integration guide for developers
- [ ] Document RPC methods and SDKs
- [ ] Create Postman collection for RPC
- [ ] Set up bug bounty program (scope: testnet)

**Deliverables**:
- Ember portal live at ember.etrid.org
- Complete validator documentation
- Community channels active

---

### Week 9-10: Testing & Launch Prep (Jan 1-15, 2026)

**Week 9 (Jan 1-8)**:
- [ ] Internal security audit (penetration testing)
- [ ] Load testing (stress test RPC nodes)
- [ ] Disaster recovery testing (validator failover)
- [ ] Backup and restore testing
- [ ] Update monitoring dashboards
- [ ] Configure alerts (PagerDuty)

**Week 10 (Jan 9-15)**:
- [ ] Invite community validators (whitelist, then open)
- [ ] Announce Ember launch (blog post, social media)
- [ ] Monitor network health (first 72 hours)
- [ ] Fix any critical issues
- [ ] Gather feedback from early validators
- [ ] Plan first governance simulation

**Deliverables**:
- **🚀 Ember Testnet Genesis (Jan 15, 2026)**
- 50+ validators active
- Public faucet distributing tokens
- Block explorer indexing live

---

## 🔒 SECURITY CONSIDERATIONS

### Network Security

**Firewall Rules** (UFW or iptables):
```bash
# Validator nodes
Allow: 30333 (p2p), 9944 (RPC - restricted), 9615 (telemetry)
Deny: All other inbound

# RPC nodes
Allow: 443 (HTTPS), 9944 (WebSocket - public)
Deny: Direct p2p access (behind load balancer)

# Monitoring
Allow: 9090 (Prometheus), 3000 (Grafana - VPN only)
```

**DDoS Protection**:
- Cloudflare (free tier) for website/explorer
- HAProxy rate limiting for RPC
- Fail2ban for SSH brute force
- Consider AWS Shield or Cloudflare Argo for production

**SSH Hardening**:
```bash
# Disable password auth
PasswordAuthentication no

# Use SSH keys only
PubkeyAuthentication yes

# Disable root login
PermitRootLogin no

# Use non-standard port (optional)
Port 2222
```

### Validator Key Management

**Security Model**:
```
┌─────────────────────────────────────────┐
│         Cold Wallet (Offline)           │
│  • Stash account (holds stake)          │
│  • Never connected to internet          │
│  • Paper backup in safe                 │
└─────────────────────────────────────────┘
              ↓ (delegates)
┌─────────────────────────────────────────┐
│       Controller Account (Hot)          │
│  • Controls validator operations        │
│  • Stored on validator node (encrypted) │
│  • Limited funds (transaction fees)     │
└─────────────────────────────────────────┘
              ↓ (uses)
┌─────────────────────────────────────────┐
│      Session Keys (Ephemeral)           │
│  • GRANDPA, BABE, ImOnline keys         │
│  • Rotated regularly (weekly)           │
│  • Generated on validator node          │
└─────────────────────────────────────────┘
```

**Key Backup**:
```bash
# Backup session keys
etrid-node key inspect --keystore-path /var/lib/etrid/chains/ember/keystore

# Encrypt backup
gpg --encrypt --recipient foundation@etrid.org keys-backup-$(date +%F).tar.gz

# Store in multiple locations
# - Encrypted cloud storage (AWS S3 + KMS)
# - Hardware security module (YubiKey)
# - Paper wallet (cold storage)
```

### Monitoring & Alerts

**Critical Alerts** (PagerDuty):
- Validator offline >5 minutes
- Block production stopped >10 blocks
- Disk usage >85%
- Memory usage >90%
- Network unreachable

**Warning Alerts** (Email):
- Finality lag >50 blocks
- Peer count <10
- RPC response time >500ms
- Database replication lag >1 minute

---

## 📊 MONITORING DASHBOARDS

### Grafana Dashboard: Ember Network Health

**Panels**:
1. **Block Production**
   - Blocks produced per minute
   - Finality lag (ASF percentage)
   - Validator participation rate

2. **Network Stats**
   - Active validators
   - Total stake
   - Transaction throughput (TPS)
   - Unique active accounts

3. **Node Health**
   - CPU usage (per validator)
   - Memory usage
   - Disk I/O
   - Network bandwidth

4. **PBC Status**
   - Blocks produced per PBC
   - Cross-chain messages relayed
   - Collator uptime

5. **RPC Performance**
   - Requests per second
   - Average response time
   - Error rate (4xx, 5xx)
   - Active connections

### Prometheus Metrics

**Custom Metrics**:
```yaml
# Block production
etrid_block_height{chain="flarechain"}
etrid_finality_lag{chain="flarechain"}

# Validator status
etrid_validator_active{validator="0x..."}
etrid_validator_uptime_percentage{validator="0x..."}

# PBC metrics
etrid_pbc_block_height{pbc="btc-pbc"}
etrid_pbc_messages_relayed{pbc="eth-pbc"}

# RPC metrics
etrid_rpc_requests_total{method="state_call"}
etrid_rpc_response_time_seconds{method="chain_getBlock"}
```

---

## 🛠️ OPERATIONAL PROCEDURES

### Daily Operations

**Morning Checklist** (15 minutes):
```bash
# Check network health
curl https://rpc.ember.etrid.org/health

# Verify block production
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  https://rpc.ember.etrid.org

# Check validator status (Grafana)
# - All validators online?
# - Finality lag <50 blocks?
# - No critical alerts?

# Review logs
ssh validator1 "journalctl -u etrid-validator -n 50 --no-pager"

# Check disk space
df -h /var/lib/etrid
```

**Weekly Tasks**:
- [ ] Review monitoring metrics (30 min)
- [ ] Rotate session keys (if needed)
- [ ] Update validator documentation
- [ ] Check for node upgrades
- [ ] Backup chain state (snapshots)

**Monthly Tasks**:
- [ ] Security audit (log review, access audit)
- [ ] Cost review (optimize if needed)
- [ ] Performance tuning (database optimization)
- [ ] Community update (blog post, stats)

### Incident Response

**Levels**:

**Level 1: Minor** (validator offline <10 min)
- Action: Automatic restart (systemd)
- Notification: Log only
- Response time: 5 minutes

**Level 2: Moderate** (validator offline >10 min, <1 hour)
- Action: Manual intervention
- Notification: Email + Slack
- Response time: 15 minutes

**Level 3: Major** (network degraded, multiple validators down)
- Action: Emergency response (all hands)
- Notification: PagerDuty + Phone call
- Response time: Immediate

**Level 4: Critical** (network halted, data loss risk)
- Action: Foundation Directors notified
- Notification: All channels
- Response time: Immediate, 24/7

### Backup & Recovery

**Backup Strategy**:
```yaml
Daily:
  - Database snapshots (explorer, monitoring)
  - Chain state (full node snapshot)
  - Configuration files (/etc/systemd, /etc/nginx)

Weekly:
  - Full system backup (all servers)
  - Encrypted and stored off-site

Monthly:
  - Disaster recovery drill (restore from backup)
  - Verify all backups readable
```

**Recovery Time Objectives (RTO)**:
- Validator failure: <15 minutes
- RPC node failure: <30 minutes
- Full network failure: <2 hours
- Complete disaster: <24 hours (from backups)

---

## 🧪 TESTING STRATEGY

### Pre-Launch Testing

**Week 9: Load Testing**
```bash
# Stress test RPC endpoints
wrk -t12 -c400 -d30s --latency \
  -H "Content-Type: application/json" \
  -s rpc-test.lua \
  https://rpc.ember.etrid.org

# Expected results:
#   Requests/sec: >1,000
#   Latency (avg): <100ms
#   Error rate: <0.1%
```

**Week 9: Failover Testing**
```bash
# Simulate validator failure
ssh validator1 "systemctl stop etrid-validator"

# Verify:
# - Network continues producing blocks
# - Finality not significantly impacted
# - Alerts triggered correctly

# Recovery:
ssh validator1 "systemctl start etrid-validator"
# Verify validator rejoins consensus within 5 minutes
```

**Week 10: Disaster Recovery**
```bash
# Simulate complete data loss
rm -rf /var/lib/etrid/chains/ember/db

# Restore from snapshot
wget https://snapshots.etrid.org/ember-latest.tar.gz
tar -xzf ember-latest.tar.gz -C /var/lib/etrid/chains/ember/

# Verify sync completes in <1 hour
```

---

## 📞 CONTACTS & ESCALATION

### On-Call Rotation

**Primary**: Infrastructure Lead
**Secondary**: Senior Developer
**Escalation**: Foundation Director

**Contact Methods**:
- PagerDuty (immediate)
- Telegram (emergency group)
- Email (non-urgent)

### Vendor Contacts

**Hetzner Support**: support@hetzner.com (24/7)
**Cloudflare Support**: Ticket system (business plan required for phone)
**PagerDuty Support**: support@pagerduty.com

---

## ✅ LAUNCH CHECKLIST

### Pre-Launch (Week 10)

**Infrastructure**:
- [ ] All validators online and producing blocks
- [ ] All 13 PBCs operational
- [ ] RPC endpoints responding (<100ms latency)
- [ ] Block explorer indexing correctly
- [ ] Faucet distributing testnet ÉTR
- [ ] Monitoring dashboards complete
- [ ] Alerts configured and tested
- [ ] Backups automated and verified

**Documentation**:
- [ ] Validator guide published
- [ ] Developer integration guide ready
- [ ] RPC API reference complete
- [ ] Faucet instructions clear
- [ ] Known issues documented
- [ ] Discord/Telegram channels active

**Security**:
- [ ] SSL certificates installed and auto-renewing
- [ ] Firewall rules applied
- [ ] SSH hardened (keys only, no password)
- [ ] DDoS protection enabled
- [ ] Rate limiting configured
- [ ] Incident response plan documented

**Communication**:
- [ ] Launch announcement draft ready
- [ ] Social media posts scheduled
- [ ] Community notified (Discord, Telegram)
- [ ] Blog post published
- [ ] Status page live

### Launch Day (Jan 15, 2026)

**Hour 0-1**:
- [ ] Announce genesis block
- [ ] Open faucet to public
- [ ] Monitor network health (real-time)
- [ ] Respond to community questions

**Hour 1-24**:
- [ ] Invite first community validators
- [ ] Monitor validator onboarding
- [ ] Track block production stability
- [ ] Gather feedback from developers

**Day 1-7**:
- [ ] Daily status updates
- [ ] Fix any critical bugs
- [ ] Onboard 50+ validators
- [ ] Achieve 99%+ uptime
- [ ] Celebrate successful launch! 🎉

---

## 🚀 POST-LAUNCH ROADMAP

### Month 1 (Jan 2026):
- Onboard 50-100 validators
- Achieve 99.9% uptime
- Handle 10,000+ RPC requests/day
- Distribute 1M+ testnet ÉTR via faucet

### Month 2 (Feb 2026):
- Deploy first dApps on Ember
- Conduct first governance simulation
- Stress test with 1,000 TPS
- Expand monitoring (custom dashboards)

### Month 3 (Mar 2026):
- Schedule security audits (mainnet prep)
- Conduct Consensus Day simulation
- Document lessons learned
- Plan mainnet launch (Q2 2026)

---

## 📝 APPENDIX A: Server Hostnames

```
# Validators (Foundation)
validator1.ember.etrid.org (US-East)
validator2.ember.etrid.org (EU-Central)
validator3.ember.etrid.org (Asia-Pacific)

# Collators (PBCs)
btc-collator.ember.etrid.org
eth-collator.ember.etrid.org
doge-collator.ember.etrid.org
sol-collator.ember.etrid.org
xlm-collator.ember.etrid.org
xrp-collator.ember.etrid.org
bnb-collator.ember.etrid.org
trx-collator.ember.etrid.org
ada-collator.ember.etrid.org
link-collator.ember.etrid.org
matic-collator.ember.etrid.org
usdt-collator.ember.etrid.org
edsc-collator.ember.etrid.org

# RPC Nodes
rpc1.ember.etrid.org
rpc2.ember.etrid.org

# Supporting Services
monitor.ember.etrid.org (Grafana/Prometheus)
explorer.ember.etrid.org (Block Explorer)
faucet.ember.etrid.org (Faucet)
archive.ember.etrid.org (Archive Node)
portal.ember.etrid.org (Website)
```

---

## 📝 APPENDIX B: Deployment Scripts

### Ansible Playbook: `deploy-validator.yml`

```yaml
---
- name: Deploy Ëtrid Ember Validator
  hosts: validators
  become: yes
  vars:
    node_version: "0.9.0-ember"
    chain_spec: "ember"

  tasks:
    - name: Update system packages
      apt:
        update_cache: yes
        upgrade: dist

    - name: Install dependencies
      apt:
        name:
          - curl
          - git
          - build-essential
          - clang
          - libssl-dev
          - pkg-config
        state: present

    - name: Create etrid user
      user:
        name: etrid
        system: yes
        shell: /bin/bash

    - name: Download etrid-node binary
      get_url:
        url: "https://github.com/EojEdred/Etrid/releases/download/v{{ node_version }}/etrid-node"
        dest: /usr/local/bin/etrid-node
        mode: '0755'

    - name: Create data directory
      file:
        path: /var/lib/etrid
        state: directory
        owner: etrid
        group: etrid

    - name: Deploy systemd service
      template:
        src: templates/etrid-validator.service.j2
        dest: /etc/systemd/system/etrid-validator.service

    - name: Enable and start service
      systemd:
        name: etrid-validator
        enabled: yes
        state: started

    - name: Install monitoring agent
      apt:
        name: prometheus-node-exporter
        state: present
```

---

## 📝 APPENDIX C: Cost Comparison by Provider

| Provider | Validator (8c/32GB) | Collator (4c/16GB) | RPC (8c/32GB) | Notes |
|----------|---------------------|-------------------|---------------|-------|
| Hetzner (Germany) | €99/mo | €59/mo | €149/mo | Best price/performance |
| OVH (France) | $120/mo | $70/mo | $160/mo | Good EU presence |
| DigitalOcean | $192/mo | $96/mo | $192/mo | Easy to use, higher cost |
| AWS EC2 (on-demand) | ~$250/mo | ~$125/mo | ~$250/mo | Most expensive |
| Vultr | $192/mo | $96/mo | $192/mo | Similar to DO |

**Recommendation**: Hetzner for cost optimization, OVH for diversity

---

**End of Infrastructure Plan**

**Next Steps**:
1. Review and approve budget ($27K/year)
2. Open Hetzner/OVH accounts
3. Begin Week 1 tasks (server provisioning)
4. Schedule security audit for Q1 2026

---

*This plan is a living document. Update as deployment progresses.*
