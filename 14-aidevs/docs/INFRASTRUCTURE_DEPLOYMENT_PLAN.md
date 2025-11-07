# Ëtrid Protocol - Ember Testnet Infrastructure Deployment Plan

**Version:** 1.0
**Date:** October 24, 2025
**Target Launch:** Q1 2026
**Status:** Planning Phase

---

## Executive Summary

This document outlines the complete infrastructure deployment plan for the **Ëtrid Protocol Ember Testnet**, a Layer 0 multichain blockchain with 13 E³20 components operating across Partition Burst Chains (PBCs).

### Infrastructure Overview
- **3 Validator Nodes** (FlareChain relay chain)
- **13 Collator Nodes** (one per E³20 component/PBC)
- **1 Archive Node** (full historical data)
- **1 Block Explorer** (web interface)
- **Monitoring Stack** (Prometheus, Grafana, Alertmanager)
- **AI Devs Infrastructure** (6 autonomous agents)

### Budget Estimate
**Total Monthly Cost:** $1,650 - $2,100 USD
**Initial Setup Cost:** $200 - $400 USD (one-time)

### Timeline
**Phase 1 (Weeks 1-2):** Infrastructure provisioning and OS setup
**Phase 2 (Weeks 3-4):** Node deployment and configuration
**Phase 3 (Weeks 5-6):** Monitoring, testing, and optimization
**Launch:** Week 7 (Early Q1 2026)

---

## 1. Infrastructure Requirements

### 1.1 Validator Nodes (3 nodes)

**Purpose:** Secure the FlareChain relay chain through consensus participation

**Hardware Specifications:**
- **CPU:** 8 physical cores @ 3.4 GHz+
  - Recommended: Intel Xeon E-2288, E-2286, AMD Ryzen 9 5900X/5950X
  - Priority: Single-thread performance over core count
- **RAM:** 64 GB ECC (2666 MHz or faster)
- **Storage:** 2 TB NVMe SSD (prioritize latency over throughput)
- **Network:** 500 Mbps symmetric (to handle peak parachain traffic)
- **OS:** Ubuntu 22.04 LTS or Debian 12 (Kernel 5.16+)

**Justification:**
- Validators process relay chain blocks + all parachain traffic
- Strong single-threaded CPU required for block production (200-400ms window)
- 64GB RAM needed for full relay chain state + parachain validation
- 2TB storage accommodates chain growth for 12+ months

**Redundancy:** 3 validators provide geographic distribution and failover capacity

---

### 1.2 Collator Nodes (13 nodes)

**Purpose:** Produce blocks for each E³20 component's Partition Burst Chain

**Hardware Specifications:**
- **CPU:** 4 physical cores @ 3.0 GHz+
  - Recommended: Intel Xeon E-2386/2388, AMD Ryzen 9 5900X
  - Single-core performance critical for PoV block production
- **RAM:** 16 GB (32 GB for high-throughput PBCs like EDSC Bridge)
- **Storage:** 200 GB NVMe SSD (100GB relay + 100GB parachain data)
- **Network:** 100 Mbps+ symmetric, public IP
  - Ports: 30333 (P2P), 30334 (P2P backup), 9944 (WebSocket RPC)
- **OS:** Ubuntu 22.04 LTS

**E³20 Component Mapping:**
1. **Accounts PBC** (pallet-accounts)
2. **DID Registry PBC** (pallet-did-registry)
3. **Validator Committee PBC** (pallet-validator-committee)
4. **Governance PBC** (pallet-governance)
5. **Bitcoin Bridge PBC** (bitcoin-bridge)
6. **EDSC Bridge PBC** (edsc-bridge) - *32GB RAM*
7. **USDT Stablecoin Bridge PBC** (stablecoin-usdt-bridge)
8. **ETWasm VM PBC** (pallet-etwasm)
9. **Staking PBC** (pallet-staking)
10. **Reserve Oracle PBC** (pallet-reserve-oracle)
11. **Asset Management PBC** (pallet-asset-management)
12. **Nomination Pools PBC** (pallet-nomination-pools)
13. **Energy Economics PBC** (pallet-energy-economics)

**Justification:**
- Each E³20 component requires dedicated collator for PoV block production
- Collators are performance-sensitive (limited time to produce/broadcast PoV)
- Bare metal preferred over VMs for maximum single-thread performance

---

### 1.3 Archive Node (1 node)

**Purpose:** Full historical blockchain data for block explorer and analytics

**Hardware Specifications:**
- **CPU:** 4 cores @ 3.0 GHz
- **RAM:** 32 GB
- **Storage:** 4 TB NVMe SSD (with expansion capacity)
- **Network:** 100 Mbps
- **OS:** Ubuntu 22.04 LTS

**Justification:**
- Archive nodes store complete blockchain history (no pruning)
- Required for block explorer queries and historical analytics
- Storage grows ~500 GB/year (testnet estimate)

---

### 1.4 Block Explorer (1 instance)

**Purpose:** Web-based blockchain explorer (Polkadot.js Apps + custom UI)

**Deployment:** Vercel (free tier, 100 GB bandwidth/month)

**Components:**
- Frontend: Next.js (already deployed at etrid-crypto-website)
- Backend API: Connects to Archive Node RPC
- Database: PostgreSQL for indexed data (optional)

**Cost:** $0 (Vercel free tier sufficient for testnet)

---

### 1.5 Monitoring Stack

**Purpose:** System health monitoring, alerting, and performance tracking

**Components:**
- **Prometheus:** Metrics collection (all nodes export metrics)
- **Grafana:** Dashboards and visualization
- **Alertmanager:** Alerts via email/Slack/Discord
- **Node Exporter:** System metrics (CPU, RAM, disk, network)

**Deployment:** Single 4GB RAM VPS hosting all monitoring services

**Hardware Specifications:**
- **CPU:** 2 cores @ 2.4 GHz
- **RAM:** 4 GB
- **Storage:** 100 GB SSD
- **Network:** 50 Mbps

---

### 1.6 AI Devs Infrastructure

**Purpose:** 6 autonomous AI agents for development, governance, and operations

**Current Deployment:** Docker Compose (running locally)

**Production Deployment Options:**
1. **Co-locate on monitoring VPS** (cost-effective, already running)
2. **Dedicated VPS** (recommended for production isolation)

**Hardware Specifications (if dedicated):**
- **CPU:** 2 cores @ 2.4 GHz
- **RAM:** 4 GB
- **Storage:** 50 GB SSD
- **Network:** 50 Mbps

**Components:**
- Orchestrator (FastAPI)
- 6 AI Agents (Compiler, Governance, Runtime, Economics, Security, Oracle)
- VectorDB (Qdrant) for persistent memory
- Grafana dashboard for AI metrics

---

## 2. VPS Provider Comparison

### 2.1 Hetzner (Recommended Primary Provider)

**Strengths:**
- Excellent price/performance ratio
- Unlimited traffic on dedicated servers
- NVMe storage standard on all servers
- Located in Germany/Finland (EU) - good for global latency
- Strong reputation in blockchain community

**Pricing (2025):**
- **Validator-class:** AX102 (AMD Ryzen 9 5950X, 64GB RAM, 2x1TB NVMe) - €109/month (~$120 USD)
- **Collator-class:** CCX33 (4 vCPU, 16GB RAM, 160GB NVMe) - €24.50/month (~$27 USD)
- **Archive Node:** AX52 (AMD Ryzen 9 3900, 64GB RAM, 2x2TB NVMe) - €79/month (~$87 USD)
- **Monitoring:** CX22 (2 vCPU, 4GB RAM, 80GB SSD) - €5.83/month (~$6.50 USD)

**Total Estimated Cost (Hetzner):**
- 3 Validators: 3 × $120 = $360/month
- 13 Collators: 13 × $27 = $351/month
- 1 Archive Node: $87/month
- 1 Monitoring: $6.50/month
- **Total: $804.50/month**

**Setup Fees:** €0-50 per server (varies by availability)

---

### 2.2 OVHcloud (Alternative/Backup Provider)

**Strengths:**
- Aggressive pricing for high-performance VPS
- Simple billing, no hidden costs
- Multiple global locations
- Good for geographic redundancy

**Pricing (2025 estimates):**
- **Validator-class:** Rise-4 (8 vCores, 64GB RAM, 1.6TB NVMe) - €140/month (~$155 USD)
- **Collator-class:** VPS Elite (4 vCores, 16GB RAM, 160GB NVMe) - €35/month (~$39 USD)
- **Archive Node:** Rise-2 (4 vCores, 32GB RAM, 800GB NVMe) - €90/month (~$100 USD)
- **Monitoring:** VPS Starter (2 vCores, 4GB RAM, 80GB SSD) - €8/month (~$9 USD)

**Total Estimated Cost (OVHcloud):**
- 3 Validators: 3 × $155 = $465/month
- 13 Collators: 13 × $39 = $507/month
- 1 Archive Node: $100/month
- 1 Monitoring: $9/month
- **Total: $1,081/month**

---

### 2.3 Cherry Servers (High-Performance Option)

**Strengths:**
- Blockchain-optimized bare metal
- Up to 60% savings vs hyperscale cloud
- 24/7 technical support
- Highly customizable dedicated hardware

**Pricing:** Not publicly listed, requires quote request

**Estimated Cost:** 10-20% higher than Hetzner, but better performance

**Use Case:** Consider for validators if maximum performance needed

---

### 2.4 OnFinality (Specialized Substrate Provider)

**Strengths:**
- Substrate/Polkadot-specialized infrastructure
- 99.99% uptime SLA
- One-click Substrate node deployment
- Elastic APIs for frontend integration

**Pricing:** Not publicly listed, requires quote

**Use Case:** Consider for Archive Node + Block Explorer API

---

### 2.5 AWS / DigitalOcean / Vultr

**Evaluation:** Not recommended for cost-sensitive testnet

**Pricing:** 2-3× more expensive than Hetzner/OVH for equivalent specs

**Use Case:** Reserved for production mainnet with enterprise support needs

---

## 3. Cost Estimates & Budget

### 3.1 Monthly Operating Costs

**Option A: Hetzner (Recommended - Most Cost-Effective)**

| Component | Quantity | Unit Price | Subtotal |
|-----------|----------|------------|----------|
| Validators (AX102) | 3 | $120 | $360 |
| Collators (CCX33) | 12 | $27 | $324 |
| EDSC Bridge Collator (CCX43 - 32GB) | 1 | $45 | $45 |
| Archive Node (AX52) | 1 | $87 | $87 |
| Monitoring VPS (CX22) | 1 | $6.50 | $6.50 |
| Block Explorer | 1 | $0 (Vercel) | $0 |
| **Total Monthly** | | | **$822.50** |

**Annual Cost:** $9,870

---

**Option B: Hybrid (Hetzner + OVH for redundancy)**

| Component | Provider | Quantity | Unit Price | Subtotal |
|-----------|----------|----------|------------|----------|
| Validators | Hetzner (2) + OVH (1) | 3 | $120/$155 | $395 |
| Collators | Hetzner (7) + OVH (6) | 13 | $27/$39 | $423 |
| Archive Node | Hetzner | 1 | $87 | $87 |
| Monitoring VPS | Hetzner | 1 | $6.50 | $6.50 |
| **Total Monthly** | | | | **$911.50** |

**Annual Cost:** $10,938

---

**Option C: OVHcloud (Higher performance, higher cost)**

| Component | Quantity | Unit Price | Subtotal |
|-----------|----------|------------|----------|
| Validators (Rise-4) | 3 | $155 | $465 |
| Collators (VPS Elite) | 13 | $39 | $507 |
| Archive Node (Rise-2) | 1 | $100 | $100 |
| Monitoring VPS | 1 | $9 | $9 |
| **Total Monthly** | | | **$1,081** |

**Annual Cost:** $12,972

---

### 3.2 One-Time Setup Costs

| Item | Cost | Notes |
|------|------|-------|
| Hetzner Setup Fees | $0-300 | Varies by server availability |
| Domain Names | $30/year | etrid.network, explorer.etrid.network, api.etrid.network |
| SSL Certificates | $0 | Let's Encrypt (free) |
| Initial Testing | $50-100 | Temporary VPS for testing |
| **Total One-Time** | **$80-430** | |

---

### 3.3 Scaling Costs (Future)

**Mainnet Launch (estimate +40%):**
- Additional redundancy (5 validators instead of 3)
- Geographic distribution (EU + US + Asia)
- Enhanced monitoring and logging
- Estimated mainnet cost: $1,150 - $1,500/month

---

## 4. Recommended Infrastructure Plan

### 4.1 Proposed Architecture (Option A - Hetzner Primary)

**Network Topology:**

```
┌─────────────────────────────────────────────────────────────┐
│                    Internet / Public Access                  │
└────────────────────────┬────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   ┌────▼─────┐    ┌────▼─────┐    ┌────▼─────┐
   │Validator │    │Validator │    │Validator │
   │   #1     │    │   #2     │    │   #3     │
   │(Germany) │    │(Finland) │    │(Germany) │
   └────┬─────┘    └────┬─────┘    └────┬─────┘
        │                │                │
        └────────────────┼────────────────┘
                         │
              ┌──────────▼──────────┐
              │   FlareChain        │
              │   Relay Chain       │
              └──────────┬──────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   ┌────▼─────┐    ┌────▼─────┐    ┌────▼─────┐
   │Collator  │    │Collator  │    │Collator  │
   │ (PBC 1)  │    │ (PBC 2)  │... │ (PBC 13) │
   └──────────┘    └──────────┘    └──────────┘
                         │
                    ┌────▼─────┐
                    │ Archive  │
                    │   Node   │
                    └────┬─────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
   ┌────▼─────┐    ┌────▼─────┐    ┌────▼─────┐
   │ Block    │    │Monitoring│    │ AI Devs  │
   │ Explorer │    │  Stack   │    │  (MCP)   │
   └──────────┘    └──────────┘    └──────────┘
```

---

### 4.2 Geographic Distribution

**Location Strategy:**
- **Primary Location:** Hetzner Germany (Falkenstein or Nuremberg)
  - 2 Validators
  - 8 Collators
  - Archive Node
  - Monitoring Stack

- **Secondary Location:** Hetzner Finland (Helsinki)
  - 1 Validator
  - 5 Collators

**Rationale:**
- Low latency between Germany and Finland (~30-50ms)
- Both locations within EU for regulatory consistency
- Hetzner's fiber backbone ensures fast inter-datacenter connectivity
- Future expansion to US/Asia for mainnet

---

### 4.3 Network Security

**Firewall Rules (per node type):**

**Validators:**
- Port 30333 (TCP): P2P (relay chain) - OPEN to public
- Port 30334 (TCP): P2P backup - OPEN to public
- Port 9944 (TCP): WebSocket RPC - RESTRICTED to monitoring/explorer only
- Port 9615 (TCP): Prometheus metrics - RESTRICTED to monitoring VPS only
- Port 22 (TCP): SSH - RESTRICTED to admin IPs only

**Collators:**
- Port 30333 (TCP): P2P (parachain) - OPEN to public
- Port 30334 (TCP): P2P (relay chain) - OPEN to public
- Port 9944 (TCP): WebSocket RPC - RESTRICTED
- Port 9615 (TCP): Prometheus metrics - RESTRICTED
- Port 22 (TCP): SSH - RESTRICTED

**Archive Node:**
- Port 9944 (TCP): WebSocket RPC - OPEN to public (for explorer)
- Port 9933 (TCP): HTTP RPC - OPEN to public (for explorer)
- Port 9615 (TCP): Prometheus metrics - RESTRICTED
- Port 22 (TCP): SSH - RESTRICTED

**Monitoring VPS:**
- Port 443 (TCP): HTTPS (Grafana) - OPEN to public (with authentication)
- Port 9090 (TCP): Prometheus - RESTRICTED (internal only)
- Port 22 (TCP): SSH - RESTRICTED

**Security Measures:**
- UFW (Uncomplicated Firewall) on all nodes
- Fail2ban for SSH brute-force protection
- SSH key-only authentication (no passwords)
- Regular security updates (unattended-upgrades)
- DDoS protection via Cloudflare (for public endpoints)

---

### 4.4 Monitoring Configuration

**Prometheus Targets:**
- All 3 validators (metrics on port 9615)
- All 13 collators (metrics on port 9615)
- Archive node (metrics on port 9615)
- System metrics (CPU, RAM, disk, network) via node_exporter

**Grafana Dashboards:**
1. **Validator Dashboard:**
   - Block production rate
   - Finality lag
   - Peer count
   - Session/era participation
   - System resources (CPU, RAM, disk I/O)

2. **Collator Dashboard:**
   - PoV block production rate
   - Inclusion rate (blocks accepted by validators)
   - Parachain finality
   - System resources

3. **Network Overview:**
   - Total blocks produced (relay + all parachains)
   - Network health (peer count, sync status)
   - Geographic distribution map

4. **AI Devs Dashboard:**
   - Skill execution rate
   - Agent health status
   - LLM token usage
   - System resources

**Alerting Rules:**
- Validator offline for >5 minutes → CRITICAL (Slack/Discord)
- Collator offline for >10 minutes → WARNING
- Disk usage >80% → WARNING
- Block production stopped → CRITICAL
- High CPU usage (>90% for >5 minutes) → WARNING
- Memory usage >95% → WARNING

**Alert Channels:**
- Email: dev@etrid.network
- Discord: #alerts channel
- Slack: #blockchain-alerts (optional)

---

## 5. Deployment Timeline

### Phase 1: Infrastructure Provisioning (Weeks 1-2)

**Week 1:**
- [ ] Order Hetzner servers (3 validators, 13 collators, 1 archive, 1 monitoring)
- [ ] Register domain names (etrid.network, explorer.etrid.network, api.etrid.network)
- [ ] Set up SSH keys and secure admin access
- [ ] Configure DNS records (A, AAAA, CNAME)

**Week 2:**
- [ ] Install Ubuntu 22.04 LTS on all servers
- [ ] Configure firewall rules (UFW)
- [ ] Install fail2ban and security hardening
- [ ] Set up unattended-upgrades for automatic security patches
- [ ] Configure NTP for time synchronization
- [ ] Install Docker on monitoring VPS

**Deliverable:** All servers provisioned, secured, and ready for node deployment

---

### Phase 2: Node Deployment (Weeks 3-4)

**Week 3:**
- [ ] Compile Ëtrid FlareChain binary (release build with optimizations)
- [ ] Deploy validator nodes:
  - [ ] Validator 1 (Germany) - with session keys
  - [ ] Validator 2 (Finland) - with session keys
  - [ ] Validator 3 (Germany) - with session keys
- [ ] Generate chain spec for Ember testnet
- [ ] Configure validator session keys and rotate
- [ ] Start validators and verify peer connectivity

**Week 4:**
- [ ] Deploy collator nodes for all 13 E³20 components:
  - [ ] Generate collator keys for each PBC
  - [ ] Configure parachain chain specs
  - [ ] Start collators and connect to relay chain
  - [ ] Verify PoV block production
- [ ] Deploy archive node:
  - [ ] Sync from genesis (with `--pruning archive`)
  - [ ] Configure RPC for block explorer access
- [ ] Deploy monitoring stack:
  - [ ] Prometheus + Grafana + Alertmanager
  - [ ] Import dashboards
  - [ ] Configure alert rules
  - [ ] Test alerting (email/Discord)

**Deliverable:** Full Ember testnet operational with monitoring

---

### Phase 3: Testing & Optimization (Weeks 5-6)

**Week 5:**
- [ ] Load testing:
  - [ ] Submit test transactions to all PBCs
  - [ ] Monitor block production under load
  - [ ] Verify finality and inclusion rates
- [ ] Performance tuning:
  - [ ] Optimize validator block production (target <400ms)
  - [ ] Tune collator PoV production (target <200ms)
  - [ ] Adjust networking parameters (peer counts, bandwidth limits)
- [ ] Security audit:
  - [ ] Penetration testing (firewall rules)
  - [ ] SSH access audit
  - [ ] Review logs for anomalies

**Week 6:**
- [ ] Deploy block explorer (Vercel):
  - [ ] Connect to archive node RPC
  - [ ] Test block/transaction lookup
  - [ ] Add custom branding and Ëtrid components
- [ ] Documentation:
  - [ ] Write operator runbook
  - [ ] Document emergency procedures
  - [ ] Create backup/restore procedures
  - [ ] Write user guides (how to connect, submit transactions)
- [ ] Community testing:
  - [ ] Invite testnet validators (external)
  - [ ] Distribute test ETR tokens
  - [ ] Run testnet for 1 week under real load

**Deliverable:** Production-ready Ember testnet with documentation

---

### Phase 4: Launch (Week 7)

- [ ] Public announcement (Discord, Twitter, blog)
- [ ] Invite external validators and collators
- [ ] Begin 24/7 monitoring and on-call rotation
- [ ] Weekly incident reports and uptime tracking
- [ ] Start collecting feedback for mainnet improvements

---

## 6. Operational Procedures

### 6.1 Validator Operations

**Daily Tasks (Automated):**
- Monitor block production via Grafana
- Check for missed sessions/eras
- Verify peer count (target: 20-50 peers)

**Weekly Tasks (Manual):**
- Review logs for errors/warnings
- Check disk usage and prune if needed
- Update software (if security patches available)
- Review validator rewards and APY

**Monthly Tasks:**
- Rotate session keys (best practice)
- Review network topology and peer diversity
- Conduct failover test (stop validator, verify backup takes over)

**Emergency Procedures:**
- **Validator Offline:** Restart systemd service, check logs, alert team
- **Chain Stall:** Coordinate with other validators, check for consensus issues
- **Security Breach:** Immediately rotate keys, isolate server, forensic analysis

---

### 6.2 Collator Operations

**Daily Tasks (Automated):**
- Monitor PoV block production
- Check inclusion rate (target: >90%)
- Verify relay chain sync status

**Weekly Tasks:**
- Review parachain-specific metrics
- Check for runtime upgrades
- Test PBC-specific functionality (bridges, DID, governance)

**Monthly Tasks:**
- Optimize collator performance (CPU/RAM tuning)
- Review parachain state growth
- Test parachain upgrades on dev environment

---

### 6.3 Backup & Disaster Recovery

**Backup Strategy:**
- **Chain Data:** Not backed up (can re-sync from network)
- **Keys:** Backed up offline (encrypted USB drives, stored securely)
- **Configuration:** Git repository (private, encrypted)

**Recovery Time Objectives (RTO):**
- Validator failure: 10 minutes (automatic failover to backup validator)
- Collator failure: 30 minutes (restart or redeploy)
- Archive node failure: 2 hours (restore from backup or re-sync)
- Complete datacenter failure: 4 hours (provision new servers, restore from backup)

**Disaster Recovery Plan:**
1. Detect failure via monitoring alerts
2. Assess scope (single node, datacenter, or multi-region)
3. Activate backup validator (if primary validator failed)
4. Provision replacement servers (if hardware failure)
5. Restore configuration from Git
6. Re-sync chain data (or restore from backup if available)
7. Verify network participation
8. Post-incident report and root cause analysis

---

## 7. Success Metrics

### 7.1 Testnet KPIs (Weeks 1-12)

| Metric | Target | Measurement |
|--------|--------|-------------|
| Validator Uptime | >99% | Prometheus (up/down time) |
| Collator Uptime | >98% | Prometheus (up/down time) |
| Block Production Rate | 12s (relay) / 6s (parachains) | Block explorer |
| Finality Lag | <30s | Grafana dashboard |
| PoV Inclusion Rate | >90% | Parachain metrics |
| Transaction Throughput | 1,000+ TPS (across all PBCs) | Load testing |
| Peer Count (validators) | 20-50 | Prometheus |
| Peer Count (collators) | 10-30 | Prometheus |

---

### 7.2 Cost Efficiency

| Metric | Target | Notes |
|--------|--------|-------|
| Monthly Cost per Validator | <$150 | Hetzner target |
| Monthly Cost per Collator | <$30 | Hetzner target |
| Total Monthly Cost | <$900 | Option A target |
| Cost per 1M Transactions | <$1 | Calculate after load testing |

---

### 7.3 Community Engagement

| Metric | Target (3 months) | Measurement |
|--------|-------------------|-------------|
| External Validators | 5-10 | On-chain validator set |
| External Collators | 10-20 | On-chain collator set |
| Active Addresses | 500+ | Block explorer analytics |
| Daily Transactions | 10,000+ | On-chain metrics |
| Discord Members | 1,000+ | Discord server stats |

---

## 8. Risk Assessment

### 8.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Validator Slashing | MEDIUM | HIGH | Run 3 validators, monitor uptime, test failover |
| Chain Stall | LOW | CRITICAL | Multiple validators, coordination protocol |
| DDoS Attack | MEDIUM | MEDIUM | Cloudflare protection, firewall rules, peer limits |
| Data Loss | LOW | MEDIUM | Keys backed up offline, config in Git |
| Network Partition | LOW | HIGH | Geographic distribution, multiple ISPs |
| Hardware Failure | MEDIUM | LOW | Provider SLA, redundancy, fast replacement |

---

### 8.2 Operational Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Key Compromise | LOW | CRITICAL | Offline key storage, SSH key-only auth, regular rotation |
| Human Error | MEDIUM | MEDIUM | Runbooks, peer review, staging environment |
| Provider Downtime | LOW | MEDIUM | Multi-provider strategy (Hetzner + OVH) |
| Cost Overruns | LOW | LOW | Fixed pricing, monitoring usage, budget alerts |

---

### 8.3 Regulatory Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| EU Data Regulations (GDPR) | LOW | MEDIUM | No personal data stored on-chain, privacy policy |
| Crypto Regulations | MEDIUM | LOW | Testnet (no real value), legal review for mainnet |

---

## 9. Next Steps

### Immediate Actions (This Week)

1. **Finalize VPS Provider Decision:**
   - Review pricing and availability
   - Confirm Hetzner as primary provider
   - Decide on hybrid vs single-provider strategy

2. **Get Budget Approval:**
   - Present this plan to project stakeholders
   - Confirm monthly budget allocation ($800-900/month for 6+ months)
   - Allocate one-time setup budget ($200-400)

3. **Order Servers:**
   - Create Hetzner account
   - Order 3 validators (AX102)
   - Order 13 collators (12× CCX33, 1× CCX43)
   - Order 1 archive node (AX52)
   - Order 1 monitoring VPS (CX22)

4. **Prepare Deployment Scripts:**
   - Ansible playbooks for server provisioning
   - Docker Compose for monitoring stack
   - Systemd service files for validators/collators
   - Backup scripts

### Short-Term (Next 2 Weeks)

- Provision and secure all servers
- Deploy initial validator nodes
- Begin testing FlareChain relay chain
- Set up monitoring infrastructure

### Medium-Term (Weeks 3-6)

- Deploy all 13 collator nodes
- Launch Ember testnet (initial private testing)
- Deploy block explorer
- Begin external validator onboarding

---

## 10. Appendix

### A. Glossary

- **Validator:** Node that secures the relay chain through consensus participation
- **Collator:** Node that produces blocks for a parachain/PBC
- **PBC:** Partition Burst Chain (Ëtrid's parachain implementation for E³20 components)
- **PoV:** Proof-of-Validity (parachain block submitted to validators)
- **Relay Chain:** FlareChain (Ëtrid's Layer 0 coordination chain)
- **E³20:** Ëtrid Enhanced Ethereum (13 core protocol components)

### B. Reference Links

- **Hetzner:** https://www.hetzner.com/
- **OVHcloud:** https://www.ovhcloud.com/
- **Polkadot Validator Docs:** https://docs.polkadot.com/infrastructure/running-a-validator/
- **Substrate Collator Docs:** https://paritytech.github.io/devops-guide/guides/collator_deployment.html
- **Ëtrid Protocol Repo:** https://github.com/etrid/etrid-protocol (private)

### C. Contact Information

- **Project Lead:** Eoj
- **DevOps Lead:** (TBD - hire or assign)
- **Emergency Contact:** dev@etrid.network
- **Discord:** https://discord.gg/etrid (TBD)

### D. Change Log

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | Oct 24, 2025 | Claude AI | Initial infrastructure deployment plan |

---

**Document Status:** DRAFT - Awaiting stakeholder review and budget approval

**Next Review:** After Phase 1 completion (server provisioning)

---

*Infrastructure is the foundation of decentralization. Let's build it right.* 🚀
