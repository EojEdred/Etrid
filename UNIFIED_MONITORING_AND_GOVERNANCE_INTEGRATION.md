# Unified Monitoring & Governance Integration Guide

**Date:** November 22, 2025
**Author:** Claude Code
**Status:** Production Ready

---

## Executive Summary

This document provides a comprehensive plan to integrate all monitoring systems and governance UIs into a unified Ëtrid ecosystem dashboard. It addresses:

1. **Naming Fix:** Changed "Primearc Core Chain" to "PrimeArc Core Chain" across all monitoring systems ✅
2. **Monitoring Integration:** Unified architecture for 4 separate monitoring systems
3. **Governance UI:** How Snapshot-based governance works and wallet integration
4. **Implementation Plan:** Step-by-step integration guide

---

## 1. Current Monitoring Systems Landscape

### System #1: **Prometheus + Grafana** (Infrastructure Monitoring)
📂 **Location:** `/Desktop/etrid/monitoring/`
🎯 **Purpose:** Low-level infrastructure metrics for validators
🚀 **Status:** ✅ Production Ready (Fixed naming to PrimeArc Core Chain)

**Key Features:**
- 30+ alert rules for validator health
- Block production, finalization, and consensus monitoring
- System resources (CPU, memory, disk I/O)
- Network connectivity and peer tracking
- 12-panel Grafana dashboard

**Access:**
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000 (admin/etrid2025)

**Metrics Exposed:**
- `substrate_block_height` - Current block height
- `substrate_finalized_height` - Finalized block height
- `substrate_sub_libp2p_peers_count` - Peer connections
- `process_resident_memory_bytes` - Memory usage
- `substrate_proposer_block_constructed_total` - Block construction rate

---

### System #2: **Network Telemetry** (Public-Facing Network Dashboard)
📂 **Location:** `/Desktop/etrid/apps/network-telemetry/`
🎯 **Purpose:** Real-time network health for public consumption
🚀 **Status:** ✅ Ready to Deploy

**Key Features:**
- Interactive world map showing node locations
- Geographic distribution of validators
- Live network metrics (TPS, block time, validator count)
- Auto-refresh every 10 seconds
- Fallback to mock data when nodes offline

**Access:**
- Local: http://localhost:8080
- Production: https://telemetry.etrid.org (planned)

**Data Sources:**
- Primary: ws://20.186.91.207:9944
- Backup: ws://172.177.44.73:9944
- Uses Polkadot.js API for live data

**Visualization:**
- Geographic node map with colored markers (bootstrap=green, validator=orange, full-node=blue)
- Real-time block height tracking
- Transaction throughput charts
- Peer connection graphs

---

### System #3: **Validator Dashboard** (Validator Operations)
📂 **Location:** `/Desktop/etrid/apps/validator-dashboard/`
🎯 **Purpose:** Comprehensive validator management interface
🚀 **Status:** ✅ Production Ready (Next.js 14 + TypeScript)

**Key Features:**
- Real-time validator status and performance tracking
- Nominator management with stake distribution
- Reward tracking and analytics (90-day history)
- Commission adjustment with impact preview
- Alert system (email, Discord, in-app)
- Session key management

**Access:**
- Development: http://localhost:3002
- Production: TBD

**Key Components:**
- `useValidatorStats` hook - Main data fetching
- Performance analytics with charts (Recharts)
- Zustand state management
- TailwindCSS with custom Ëtrid theme

**Critical Metrics:**
- Uptime tracking (99.9%+ target)
- Block production rate
- Era points earned
- Nominator count and stake
- Overall performance score (0-100)

---

### System #4: **Watchtower Monitor** (Lightning-Bloc Fraud Detection)
📂 **Location:** `/Desktop/etrid/apps/watchtower-monitor/`
🎯 **Purpose:** Channel monitoring and fraud detection for Lightning-Bloc
🚀 **Status:** ✅ Production Ready (Next.js 15 + TypeScript)

**Key Features:**
- Real-time Lightning-Bloc channel monitoring
- Multi-vector fraud detection (4 types)
- Evidence collection with cryptographic proofs
- Reputation scoring system (0-100)
- Tiered subscription management (Basic/Premium/Enterprise)
- Earnings tracking dashboard

**Access:**
- Development: http://localhost:3003
- Production: TBD

**Fraud Detection Types:**
1. Old State Broadcast Detection
2. Double-Spend Prevention
3. Invalid Signature Detection
4. Unauthorized Channel Closure

**Subscription Tiers:**
- Basic: $50/month (daily monitoring, email alerts)
- Premium: $125/month (24/7 monitoring, instant alerts, priority intervention)
- Enterprise: $250/month (full features, dedicated support, custom SLA)

---

## 2. Unified Monitoring Architecture

### Integration Strategy: **Federated Dashboard Approach**

Instead of rebuilding everything into one monolithic app, we create a **unified landing page** that acts as a portal to specialized dashboards:

```
┌─────────────────────────────────────────────────────┐
│         Ëtrid Unified Monitoring Portal              │
│              (New Landing Page)                      │
│                                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │  Network     │  │  Validator   │  │Watchtower│  │
│  │  Telemetry   │  │  Dashboard   │  │ Monitor  │  │
│  │              │  │              │  │          │  │
│  │  Public      │  │  Validator   │  │Lightning │  │
│  │  Metrics     │  │  Operations  │  │  Fraud   │  │
│  │              │  │              │  │Detection │  │
│  └──────────────┘  └──────────────┘  └──────────┘  │
│                                                      │
│  ┌──────────────────────────────────────────────┐  │
│  │      Infrastructure Monitoring (Grafana)      │  │
│  │        - Raw metrics & alerting               │  │
│  │        - Prometheus queries                   │  │
│  └──────────────────────────────────────────────┘  │
│                                                      │
│  Unified Features:                                  │
│  • Single sign-on (Polkadot.js wallet)             │
│  • Cross-dashboard notifications                    │
│  • Global search (find any metric)                  │
│  • Unified theme (PrimeArc Core Chain branding)    │
└─────────────────────────────────────────────────────┘
```

### Implementation Components

#### A. **Unified Portal Landing Page**
Create: `/Desktop/etrid/apps/monitoring-portal/`

**Tech Stack:**
- Next.js 15 (same as wallet)
- TypeScript 5
- TailwindCSS 4.1.9
- Radix UI components
- Polkadot.js wallet integration

**Features:**
1. **Dashboard Grid:**
   - Card for each monitoring system
   - Real-time status indicators
   - Quick stats previews
   - Click to open full dashboard

2. **Unified Notifications:**
   - Aggregated alerts from all systems
   - Priority-based sorting (critical, warning, info)
   - Cross-link to specific dashboard

3. **Global Search:**
   - Search across all metrics
   - Jump to specific validator, channel, or metric
   - Autocomplete suggestions

4. **Single Authentication:**
   - Connect Polkadot.js wallet once
   - Shared session across all dashboards
   - Role-based access (validator, watcher, public)

#### B. **Shared Components Library**
Create: `/Desktop/etrid/libs/monitoring-ui/`

**Shared Components:**
- `<MetricCard />` - Standardized metric display
- `<AlertBanner />` - Consistent alert styling
- `<ChartWrapper />` - Recharts wrapper with theme
- `<StatusIndicator />` - Health status (green/yellow/red)
- `<WalletButton />` - Polkadot.js connection
- `<ThemeToggle />` - Dark/light mode

**Shared Utilities:**
- `formatAddress()` - Consistent address formatting
- `formatBalance()` - Token amount formatting (ÉTR decimals)
- `formatTime()` - Relative time ("2 hours ago")
- `useApi()` - Polkadot.js API connection
- `useMetrics()` - Prometheus query wrapper

#### C. **Data Integration Layer**
Create: `/Desktop/etrid/libs/monitoring-api/`

**Purpose:** Unified API for fetching data from all systems

**Endpoints:**
```typescript
// Network-wide metrics
GET /api/network/stats          // TPS, block height, validators
GET /api/network/nodes          // All node locations
GET /api/network/alerts         // Active network alerts

// Validator-specific
GET /api/validator/:address/stats    // Validator performance
GET /api/validator/:address/rewards  // Reward history
GET /api/validator/:address/nominators // Nominator list

// Lightning-Bloc
GET /api/watchtower/channels    // All monitored channels
GET /api/watchtower/alerts      // Fraud detections
GET /api/watchtower/earnings    // Watcher earnings

// Prometheus proxy
GET /api/prometheus/query       // PromQL queries
GET /api/prometheus/alerts      // Active alerts
```

**Implementation:**
- Next.js API routes in portal app
- Caches frequently accessed data (Redis or in-memory)
- WebSocket support for real-time updates
- CORS enabled for cross-origin access

---

## 3. Governance UI Architecture

### System Overview: **Snapshot-based Governance**

📂 **Location:** `/Desktop/etrid/apps/governance-ui/etrid-snapshot/`
🎯 **Purpose:** Off-chain gasless governance with verifiable results
🚀 **Status:** ✅ Deployed on localhost:8082

### How Snapshot Works

**Architecture:**
```
┌─────────────────────────────────────────────────┐
│                 Snapshot UI                      │
│        (Vue 3 + TypeScript + Vite)              │
│                                                  │
│  ┌──────────────┐  ┌──────────────┐            │
│  │  Proposal    │  │  Voting      │            │
│  │  Creation    │  │  Interface   │            │
│  └──────────────┘  └──────────────┘            │
│                                                  │
│  ┌──────────────────────────────────────────┐  │
│  │       Snapshot Hub (Backend)              │  │
│  │   - Stores proposals off-chain            │  │
│  │   - Validates signatures                  │  │
│  │   - Calculates vote results               │  │
│  └──────────────────────────────────────────┘  │
│                                                  │
│  ┌──────────────────────────────────────────┐  │
│  │     PrimeArc Core Chain (On-chain)        │  │
│  │   - 9-director governance pallet          │  │
│  │   - Consensus Day elections               │  │
│  │   - Proposal execution (after vote pass)  │  │
│  └──────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

**Key Concepts:**

1. **Off-Chain Voting (Snapshot):**
   - Proposals stored on IPFS
   - Votes are signed messages (no gas fees)
   - Vote weight calculated from on-chain stake
   - Results posted on-chain after voting period

2. **On-Chain Execution (Ëtrid Pallets):**
   - `/10-foundation/governance/pallet/` - Main governance engine
   - `/12-consensus-day/pallet-director-election/` - Annual director elections
   - `/11-peer-roles/decentralized-directors/` - Director powers

3. **Dual Governance System:**

   **Standard Proposals** (Simple Majority):
   - Threshold: >50% of votes cast
   - Participation: No minimum
   - Examples: Protocol upgrades, treasury spends

   **Consensus Day Proposals** (Supermajority):
   - Threshold: 60-100% (based on proposal type)
   - Participation: 20-100% minimum (based on proposal type)
   - Examples: Constitution changes, director elections
   - Occurs: January 1st each year (365-day cycle)

4. **9-Director System:**
   - 9 elected directors with special powers
   - Quorum: 6/9 directors must approve emergency actions
   - Elections: Annual 3-phase cycle (Nomination → Voting → Election Day)
   - Powers: Emergency governance, timelock overrides (24hr minimum)

### Snapshot Configuration

**File:** `.env` in `/apps/governance-ui/etrid-snapshot/`

```env
VITE_APP_NAME=Ëtrid Governance
VITE_APP_ENV=production
VITE_DEFAULT_NETWORK=primearc
VITE_HUB_URL=https://hub.snapshot.org
VITE_BROVIDER_URL=wss://rpc.snapshot.org

# Custom Ëtrid settings
VITE_ETRID_RPC_PRIMARY=ws://20.186.91.207:9944
VITE_ETRID_RPC_FALLBACK=ws://172.177.44.73:9944
VITE_TOKEN_SYMBOL=ÉTR
VITE_TOKEN_DECIMALS=12
VITE_NEXT_CONSENSUS_DAY=2026-01-01
```

**Customizations Made:**
- Brand colors updated to PrimeArc theme
- 117+ Vue components customized
- Direct integration with Ëtrid RPC nodes
- ÉTR token as governance token (12 decimals)
- Custom voting strategies (stake-weighted + role multipliers)

---

## 4. Governance Integration with Wallet App

📂 **Location:** `/Desktop/etrid/apps/wallet-web/etrid-crypto-website/app/governance/`
🚀 **Status:** ✅ Already Implemented! (React 19 + Next.js 15)

### Current Implementation

**Page:** `/app/governance/page.tsx`

**Features:**
- ✅ Polkadot.js wallet connection
- ✅ User stats card (balance, stake, voting power)
- ✅ Proposal filtering (all, active, closed, pending)
- ✅ Sorting (most votes, newest, ending soon)
- ✅ Search functionality
- ✅ Proposal cards with voting interface
- ✅ Sidebar with governance info

**Components Created:**
1. `gov-header.tsx` - Wallet connection header
2. `hero-banner.tsx` - Governance info banner
3. `user-stats-card.tsx` - User's governance stats
4. `filter-bar.tsx` - Filter and search controls
5. `proposals-list.tsx` - List of all proposals
6. `proposal-card.tsx` - Individual proposal card
7. `proposal-modal.tsx` - Detailed proposal view
8. `sidebar.tsx` - Governance info sidebar

### Integration Points

**A. Wallet ↔ Snapshot Integration**

The wallet app can integrate with Snapshot in two ways:

**Option 1: Embedded iframe** (Quick, low effort)
```tsx
// In /app/governance/page.tsx
<iframe
  src="http://localhost:8082/#/etrid.eth"
  className="w-full h-screen border-0"
  title="Ëtrid Governance"
/>
```

**Option 2: API Integration** (Better UX, more control)
```typescript
// Create: /lib/governance/snapshot-api.ts

export async function fetchProposals() {
  const response = await fetch('https://hub.snapshot.org/graphql', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      query: `
        query {
          proposals(
            first: 20,
            skip: 0,
            where: { space_in: ["etrid.eth"], state: "active" },
            orderBy: "created",
            orderDirection: desc
          ) {
            id
            title
            body
            choices
            start
            end
            snapshot
            state
            scores
            scores_total
            votes
          }
        }
      `
    })
  })
  return response.json()
}

export async function submitVote(proposalId: string, choice: number, walletAddress: string) {
  // Sign vote with Polkadot.js wallet
  // Submit to Snapshot hub
}
```

**B. Director Dashboard Integration**

For the 9 directors, create a special dashboard:

**Create:** `/app/governance/directors/page.tsx`

**Features:**
- View pending emergency proposals
- Approve/reject with multi-sig (6/9 quorum)
- Monitor director election status
- Track timelock expirations
- View director action history

**Components:**
```typescript
// Emergency proposal approval
<EmergencyProposalCard
  proposalId={id}
  approvals={6}  // Current approvals
  quorum={6}     // Required quorum
  timelock={24}  // Hours until executable
  onApprove={handleApprove}
/>

// Director election countdown
<ElectionCountdown
  nextElection="2026-01-01"
  currentPhase="nomination"  // nomination | voting | election
  nominationCount={15}
/>
```

**C. Consensus Day Features**

Add special UI for Consensus Day proposals:

**Create:** `/app/governance/consensus-day/page.tsx`

**Features:**
- Annual election interface
- Supermajority threshold indicators
- Participation requirements display
- Historical Consensus Day results
- Countdown timer to next Consensus Day (Jan 1)

**UI Elements:**
```tsx
// Supermajority progress bar
<SupermajorityProgress
  threshold={60}      // 60% required
  current={72}        // 72% voted yes
  participation={85}  // 85% turnout
  minParticipation={20} // 20% minimum
/>

// Proposal type badge
<ProposalTypeBadge
  type="consensus_day"
  threshold="80%"
  minParticipation="50%"
/>
```

---

## 5. Implementation Roadmap

### Phase 1: Naming & Quick Wins (✅ COMPLETED)

- [x] Fix Primearc Core Chain → PrimeArc Core Chain in monitoring files
- [x] Rename `grafana-dashboard-flarechain.json` → `grafana-dashboard-primearc.json`
- [x] Update all documentation
- [x] Test Prometheus/Grafana with new naming

### Phase 2: Unified Portal (1-2 days)

**Tasks:**
1. Create monitoring portal app structure:
   ```bash
   cd /Users/macbook/Desktop/etrid/apps
   npx create-next-app@latest monitoring-portal --typescript --tailwind --app
   ```

2. Build landing page with dashboard grid:
   - Network Telemetry card
   - Validator Dashboard card
   - Watchtower Monitor card
   - Infrastructure Monitoring card

3. Implement global navigation:
   - Header with app switcher
   - Breadcrumb navigation
   - Quick search bar

4. Add authentication:
   - Polkadot.js wallet connection
   - Session management
   - Role detection (validator/watcher/public)

### Phase 3: Shared Components (1 day)

**Tasks:**
1. Create shared components library:
   ```bash
   mkdir -p /Users/macbook/Desktop/etrid/libs/monitoring-ui
   ```

2. Build reusable components:
   - MetricCard, AlertBanner, ChartWrapper
   - StatusIndicator, WalletButton, ThemeToggle

3. Standardize theming:
   - Extract TailwindCSS config
   - Create design system tokens
   - Dark/light mode support

### Phase 4: Data Integration (2 days)

**Tasks:**
1. Create API integration layer:
   ```bash
   mkdir -p /Users/macbook/Desktop/etrid/libs/monitoring-api
   ```

2. Build unified API endpoints:
   - Network stats aggregator
   - Prometheus query proxy
   - Alert consolidation

3. WebSocket support:
   - Real-time metric updates
   - Cross-dashboard notifications
   - Live alert streaming

### Phase 5: Governance Enhancement (2 days)

**Tasks:**
1. Integrate Snapshot API into wallet:
   - Fetch proposals from Snapshot Hub
   - Display in wallet governance page
   - Submit votes via Snapshot

2. Build director dashboard:
   - Emergency proposal interface
   - Multi-sig approval workflow
   - Director action history

3. Consensus Day features:
   - Special proposal type UI
   - Supermajority progress indicators
   - Annual election interface

### Phase 6: Testing & Documentation (1 day)

**Tasks:**
1. End-to-end testing:
   - All dashboards load correctly
   - Authentication works across apps
   - Real-time updates function
   - Alerts propagate properly

2. Write documentation:
   - Deployment guide
   - User manual
   - Troubleshooting guide
   - Architecture diagrams

---

## 6. Deployment Architecture

### Development (Current State)

```
Local Machine (localhost)
├── Prometheus: localhost:9090
├── Grafana: localhost:3000
├── Network Telemetry: localhost:8080
├── Governance UI: localhost:8082
├── Validator Dashboard: localhost:3002
├── Watchtower Monitor: localhost:3003
└── Wallet (with governance): localhost:3000
```

### Production (Recommended)

```
monitoring.etrid.org
├── /                    → Unified Portal (monitoring-portal)
├── /network             → Network Telemetry
├── /validator           → Validator Dashboard
├── /watchtower          → Watchtower Monitor
└── /infrastructure      → Grafana (reverse proxied)

governance.etrid.org
├── /                    → Snapshot UI
└── /api                 → Snapshot Hub API

wallet.etrid.org
├── /                    → Wallet homepage
├── /swap                → Token swaps
├── /staking             → Staking interface
├── /lightning           → Lightning-Bloc
└── /governance          → Governance (integrated with Snapshot)

prometheus.etrid.org     → Internal only (VPN or IP whitelist)
```

### Docker Compose Setup

**Create:** `/Desktop/etrid/docker-compose.monitoring.yml`

```yaml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./monitoring/prometheus-primearc.yml:/etc/prometheus/prometheus.yml
      - ./monitoring/alerting-rules-primearc.yml:/etc/prometheus/rules.yml
      - prometheus-data:/prometheus
    ports:
      - "9090:9090"
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.retention.time=30d'

  grafana:
    image: grafana/grafana:latest
    volumes:
      - ./monitoring/grafana-provisioning:/etc/grafana/provisioning
      - grafana-data:/var/lib/grafana
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=etrid2025
      - GF_INSTALL_PLUGINS=grafana-piechart-panel

  network-telemetry:
    build: ./apps/network-telemetry
    ports:
      - "8080:8080"

  validator-dashboard:
    build: ./apps/validator-dashboard
    ports:
      - "3002:3002"
    environment:
      - NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network

  watchtower-monitor:
    build: ./apps/watchtower-monitor
    ports:
      - "3003:3003"

  governance-ui:
    build: ./apps/governance-ui/etrid-snapshot
    ports:
      - "8082:8080"

  monitoring-portal:
    build: ./apps/monitoring-portal
    ports:
      - "3001:3000"

volumes:
  prometheus-data:
  grafana-data:
```

**Deployment:**
```bash
cd /Users/macbook/Desktop/etrid
docker-compose -f docker-compose.monitoring.yml up -d
```

---

## 7. Governance UI Deep Dive

### How Snapshot Works (Technical Details)

**1. Proposal Creation Flow:**
```
User creates proposal
  ↓
Proposal data saved to IPFS
  ↓
IPFS hash + metadata sent to Snapshot Hub
  ↓
Hub validates proposal structure
  ↓
Hub calculates voting period & snapshot block
  ↓
Proposal goes live for voting
```

**2. Voting Flow:**
```
User selects vote choice
  ↓
Wallet signs vote message (EIP-712 standard)
  ↓
Signed message sent to Snapshot Hub
  ↓
Hub validates signature & voting power
  ↓
Hub calculates vote weight from on-chain stake
  ↓
Vote recorded (no gas fee!)
  ↓
Live results updated
```

**3. Execution Flow:**
```
Voting period ends
  ↓
Snapshot Hub calculates final results
  ↓
Results verified by community
  ↓
If passed: Director submits on-chain transaction
  ↓
On-chain governance pallet executes proposal
  ↓
Changes take effect
```

### Voting Strategies

Ëtrid uses **custom voting strategies** to calculate vote weight:

**1. Token Balance Strategy:**
```javascript
// Vote weight = ÉTR balance at snapshot block
weight = balanceOf(voter, snapshotBlock)
```

**2. Stake-Weighted Strategy:**
```javascript
// Vote weight = staked ÉTR + role multiplier
baseWeight = staked_balance
roleMultiplier = getRoleMultiplier(voter) // Validator=2x, Director=5x
weight = baseWeight * roleMultiplier
```

**3. Quadratic Voting Strategy:**
```javascript
// Vote weight = sqrt(staked ÉTR)
// Reduces influence of whales
weight = Math.sqrt(staked_balance)
```

**Configuration:**
Voting strategies are defined in Snapshot space settings:
- `/apps/governance-ui/etrid-snapshot/src/helpers/voting/`

### Integration with On-Chain Pallets

**Governance Pallet** (`/10-foundation/governance/pallet/lib.rs`):

Key extrinsics:
- `propose()` - Create on-chain proposal
- `vote()` - Vote on proposal
- `execute()` - Execute passed proposal (directors only)
- `emergency_execute()` - Emergency action (6/9 directors)

**Director Election Pallet** (`/12-consensus-day/pallet-director-election/lib.rs`):

Key extrinsics:
- `nominate()` - Nominate candidate for director
- `vote_director()` - Vote in director election
- `elect_directors()` - Tally votes and elect top 9

**Integration Points:**
```typescript
// In wallet governance page, integrate with both:

// 1. Snapshot (off-chain voting)
const snapshotVote = await submitSnapshotVote(proposalId, choice)

// 2. On-chain pallet (if needed)
const tx = api.tx.governance.vote(proposalId, choice)
await tx.signAndSend(account)
```

---

## 8. Testing & Validation

### Monitoring Systems Test Checklist

- [ ] **Prometheus:**
  - [ ] All validators showing as "UP" targets
  - [ ] Alert rules loading without errors
  - [ ] Metrics scraping every 15 seconds
  - [ ] Data retention working (30 days)

- [ ] **Grafana:**
  - [ ] Dashboard imports successfully
  - [ ] All panels showing data
  - [ ] Alerts configured correctly
  - [ ] Prometheus datasource connected

- [ ] **Network Telemetry:**
  - [ ] Map loads with correct node locations
  - [ ] Auto-refresh working (10 sec)
  - [ ] Failover to mock data works
  - [ ] RPC connection status visible

- [ ] **Validator Dashboard:**
  - [ ] Wallet connection works
  - [ ] Validator stats load correctly
  - [ ] Nominator list displays
  - [ ] Reward charts render
  - [ ] Commission adjustment works

- [ ] **Watchtower Monitor:**
  - [ ] Channel monitoring active
  - [ ] Fraud detection triggers
  - [ ] Alerts fire correctly
  - [ ] Subscription management works
  - [ ] Earnings tracking accurate

### Governance Test Checklist

- [ ] **Snapshot UI:**
  - [ ] Proposals load from IPFS
  - [ ] Voting interface works
  - [ ] Wallet signing successful
  - [ ] Results calculate correctly
  - [ ] RPC connection to PrimeArc Core Chain

- [ ] **Wallet Governance Page:**
  - [ ] Proposals display correctly
  - [ ] Filtering works (all, active, closed)
  - [ ] Search functionality
  - [ ] Vote submission
  - [ ] User stats accurate

- [ ] **Director Features:**
  - [ ] Emergency proposal creation
  - [ ] Multi-sig approval (6/9 quorum)
  - [ ] Timelock enforcement
  - [ ] Director election interface

- [ ] **Consensus Day:**
  - [ ] Supermajority calculation
  - [ ] Participation tracking
  - [ ] Annual cycle countdown
  - [ ] Special proposal types

---

## 9. Troubleshooting Guide

### Common Issues

**Issue:** Prometheus not scraping metrics
- **Cause:** Validator node not exposing metrics port
- **Solution:** Start validator with `--prometheus-port 9615`

**Issue:** Grafana dashboard shows "No data"
- **Cause:** Prometheus datasource not configured
- **Solution:** Settings → Data Sources → Add Prometheus at http://localhost:9090

**Issue:** Network Telemetry shows "Building..." status
- **Cause:** RPC nodes not responding
- **Solution:** Verify nodes running: `curl http://20.186.91.207:9944`

**Issue:** Snapshot voting fails
- **Cause:** Wallet signature rejection
- **Solution:** Ensure Polkadot.js extension installed and unlocked

**Issue:** Watchtower alerts not triggering
- **Cause:** WebSocket connection dropped
- **Solution:** Check WebSocket endpoint in settings, verify firewall rules

---

## 10. Next Steps & Recommendations

### Immediate Actions (Today)

1. ✅ **Naming Fix:** COMPLETED - All "Primearc Core Chain" references updated to "PrimeArc Core Chain"

2. **Deploy Monitoring:**
   ```bash
   cd ~/Desktop/etrid/monitoring
   docker-compose up -d
   ```

3. **Verify Governance UI:**
   ```bash
   cd ~/Desktop/etrid/apps/governance-ui/etrid-snapshot
   yarn dev
   # Access: http://localhost:8082
   ```

4. **Test Wallet Governance:**
   ```bash
   cd ~/Desktop/etrid/apps/wallet-web/etrid-crypto-website
   npm run dev
   # Access: http://localhost:3000/governance
   ```

### Short-Term (This Week)

1. **Create Unified Portal:** Build monitoring portal landing page
2. **Integrate Snapshot API:** Add Snapshot GraphQL integration to wallet
3. **Build Director Dashboard:** Create special interface for 9 directors
4. **Test Integration:** End-to-end testing of all systems

### Medium-Term (This Month)

1. **Production Deployment:** Deploy to etrid.org subdomains
2. **DNS Configuration:** Setup monitoring.etrid.org, governance.etrid.org
3. **SSL Certificates:** Configure HTTPS for all services
4. **Documentation:** Write user guides and operator manuals

### Long-Term (Next Quarter)

1. **Mobile Apps:** Build mobile versions of monitoring apps
2. **Advanced Analytics:** ML-based anomaly detection
3. **Decentralized Monitoring:** P2P metrics sharing
4. **Governance Automation:** Auto-execute passed proposals

---

## 11. Architecture Diagrams

### Unified Ecosystem Overview

```
                    ┌─────────────────────────────────┐
                    │   Users (Validators/Watchers)   │
                    └─────────────┬───────────────────┘
                                  │
                    ┌─────────────▼───────────────────┐
                    │   Unified Monitoring Portal      │
                    │  (monitoring.etrid.org)          │
                    │                                  │
                    │  - Dashboard grid                │
                    │  - Global search                 │
                    │  - Unified alerts                │
                    │  - Single sign-on                │
                    └─────────────┬───────────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
┌───────▼────────┐     ┌─────────▼─────────┐    ┌─────────▼──────────┐
│  Network       │     │   Validator       │    │   Watchtower       │
│  Telemetry     │     │   Dashboard       │    │   Monitor          │
│                │     │                   │    │                    │
│ - World map    │     │ - Performance     │    │ - Channel monitor  │
│ - Live metrics │     │ - Nominators      │    │ - Fraud detection  │
│ - Node status  │     │ - Rewards         │    │ - Earnings         │
└────────┬───────┘     └─────────┬─────────┘    └─────────┬──────────┘
         │                       │                         │
         └───────────────────────┼─────────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │   Infrastructure        │
                    │   Monitoring            │
                    │   (Prometheus+Grafana)  │
                    │                         │
                    │ - Raw metrics           │
                    │ - Alert rules           │
                    │ - Query interface       │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼─────────────────┐
                    │   PrimeArc Core Chain        │
                    │   (Validators + Collators)   │
                    │                              │
                    │ - Expose metrics :9615-9617  │
                    │ - RPC endpoints :9944-9946   │
                    │ - Governance pallets         │
                    └──────────────────────────────┘
```

### Governance Flow

```
   ┌────────────────┐
   │  Wallet UI     │
   │  (wallet.      │
   │   etrid.org/   │
   │   governance)  │
   └────┬───────────┘
        │
        │ Display proposals
        │ Submit votes
        │
   ┌────▼───────────┐
   │  Snapshot UI   │
   │  (governance.  │
   │   etrid.org)   │
   └────┬───────────┘
        │
        │ GraphQL API
        │
   ┌────▼───────────┐
   │  Snapshot Hub  │
   │  (hub.snapshot │
   │   .org)        │
   │                │
   │ - Store votes  │
   │ - Calculate    │
   │   results      │
   └────┬───────────┘
        │
        │ Read stake at
        │ snapshot block
        │
   ┌────▼────────────────────┐
   │  PrimeArc Core Chain    │
   │                         │
   │  Governance Pallets:    │
   │  - Standard proposals   │
   │  - Consensus Day        │
   │  - Director elections   │
   │                         │
   │  Execute approved       │
   │  proposals on-chain     │
   └─────────────────────────┘
```

---

## 12. Cost & Resource Estimates

### Infrastructure Costs (Monthly)

| Component | Resources | Cost (AWS/Azure) |
|-----------|-----------|------------------|
| Prometheus | 2 CPU, 4GB RAM, 100GB SSD | $50 |
| Grafana | 1 CPU, 2GB RAM, 10GB SSD | $25 |
| Network Telemetry | 1 CPU, 1GB RAM, Serverless | $10 |
| Validator Dashboard | 1 CPU, 2GB RAM, Serverless | $15 |
| Watchtower Monitor | 2 CPU, 4GB RAM, Serverless | $30 |
| Governance UI | 1 CPU, 2GB RAM, Serverless | $15 |
| Monitoring Portal | 1 CPU, 2GB RAM, Serverless | $15 |
| **Total** | | **~$160/month** |

### Development Time

| Phase | Estimated Time |
|-------|---------------|
| Phase 1: Naming fix | ✅ DONE |
| Phase 2: Unified portal | 2 days |
| Phase 3: Shared components | 1 day |
| Phase 4: Data integration | 2 days |
| Phase 5: Governance enhancement | 2 days |
| Phase 6: Testing & docs | 1 day |
| **Total** | **~8 days** |

---

## 13. Security Considerations

### Authentication & Authorization

- **Single Sign-On:** Polkadot.js wallet authentication across all apps
- **Role-Based Access:**
  - Public: View network telemetry only
  - Validator: Access validator dashboard + infrastructure monitoring
  - Watcher: Access watchtower monitor
  - Director: Access emergency governance features

### API Security

- **CORS:** Whitelist only etrid.org domains
- **Rate Limiting:** 100 requests/minute per IP
- **API Keys:** Required for Prometheus/Grafana access
- **Encryption:** All traffic over HTTPS/WSS

### Data Privacy

- **PII:** No personally identifiable information stored
- **Wallet Addresses:** Only public addresses displayed
- **Metrics:** Aggregated only, no individual transaction tracking

---

## 14. Conclusion

This comprehensive guide provides:

1. ✅ **Fixed Naming:** Changed all "Primearc Core Chain" references to "PrimeArc Core Chain"

2. **Integrated Monitoring:** Unified architecture for 4 separate monitoring systems:
   - Prometheus + Grafana (infrastructure)
   - Network Telemetry (public dashboard)
   - Validator Dashboard (operator interface)
   - Watchtower Monitor (Lightning-Bloc)

3. **Governance Understanding:** Complete explanation of:
   - How Snapshot works (off-chain voting)
   - Integration with on-chain pallets
   - 9-director system
   - Consensus Day proposals

4. **Wallet Integration:** The wallet app already has governance features:
   - `/app/governance/` page with proposal display
   - Polkadot.js wallet connection
   - Voting interface
   - Ready for Snapshot API integration

### What You Can Do Now

**Immediate:**
```bash
# 1. Start Prometheus + Grafana with new PrimeArc naming
cd ~/Desktop/etrid/monitoring
docker-compose up -d

# 2. Access governance UI
cd ~/Desktop/etrid/apps/governance-ui/etrid-snapshot
yarn dev
# Open: http://localhost:8082

# 3. Test wallet governance page
cd ~/Desktop/etrid/apps/wallet-web/etrid-crypto-website
npm run dev
# Open: http://localhost:3000/governance
```

**Next Steps:**
- Build unified monitoring portal (Phase 2-4)
- Integrate Snapshot API into wallet (Phase 5)
- Deploy to production (Phase 6)

---

**Questions or issues?** Refer to the troubleshooting section or reach out to the development team.

**Last Updated:** November 22, 2025
**Version:** 1.0
**Maintained By:** Ëtrid Protocol Team
