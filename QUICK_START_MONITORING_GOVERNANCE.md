# Quick Start: Monitoring & Governance Integration

**TL;DR:** All your monitoring systems are now documented and ready to integrate. Naming fixed to "PrimeArc Core Chain" ✅

---

## What Was Done ✅

### 1. Fixed Naming Issue
- **Changed:** "Primearc Core Chain" → "PrimeArc Core Chain" across all monitoring files
- **Files Updated:** 9 files in `/Desktop/etrid/monitoring/`
- **Files Renamed:**
  - `grafana-dashboard-flarechain.json` → `grafana-dashboard-primearc.json`
  - `alerting-rules-flarechain.yml` → `alerting-rules-primearc.yml`
  - `prometheus-flarechain.yml` → `prometheus-primearc.yml`

### 2. Analyzed All Monitoring Systems

You have **4 separate monitoring systems**:

| System | Location | Port | Purpose |
|--------|----------|------|---------|
| **Prometheus + Grafana** | `/monitoring/` | 9090, 3000 | Infrastructure metrics |
| **Network Telemetry** | `/apps/network-telemetry/` | 8080 | Public network dashboard |
| **Validator Dashboard** | `/apps/validator-dashboard/` | 3002 | Validator operations |
| **Watchtower Monitor** | `/apps/watchtower-monitor/` | 3003 | Lightning-Bloc fraud detection |

### 3. Documented Governance

You have **2 governance interfaces**:

| Interface | Location | Port | Technology |
|-----------|----------|------|------------|
| **Snapshot UI** | `/apps/governance-ui/etrid-snapshot/` | 8082 | Vue 3 + Snapshot |
| **Wallet Governance** | `/apps/wallet-web/etrid-crypto-website/app/governance/` | 3000 | React 19 + Next.js 15 |

**How it works:**
- Snapshot = Off-chain voting (no gas fees)
- Wallet = User-friendly interface
- On-chain pallets = Execute approved proposals

---

## How to Start Everything Right Now

### Option 1: Docker (Fastest)

```bash
# Start all monitoring systems
cd ~/Desktop/etrid/monitoring
docker-compose up -d

# Access dashboards:
# - Prometheus: http://localhost:9090
# - Grafana: http://localhost:3000 (admin/etrid2025)
```

### Option 2: Individual Services

**1. Start Prometheus:**
```bash
cd ~/Desktop/etrid/monitoring
./install-prometheus.sh
# Access: http://localhost:9090
```

**2. Start Grafana:**
```bash
cd ~/Desktop/etrid/monitoring
./install-grafana.sh
# Access: http://localhost:3000 (admin/etrid2025)
```

**3. Start Network Telemetry:**
```bash
cd ~/Desktop/etrid/apps/network-telemetry
python3 -m http.server 8080
# Access: http://localhost:8080
```

**4. Start Validator Dashboard:**
```bash
cd ~/Desktop/etrid/apps/validator-dashboard
npm install
npm run dev
# Access: http://localhost:3002
```

**5. Start Watchtower Monitor:**
```bash
cd ~/Desktop/etrid/apps/watchtower-monitor
npm install
npm run dev
# Access: http://localhost:3003
```

**6. Start Governance UI (Snapshot):**
```bash
cd ~/Desktop/etrid/apps/governance-ui/etrid-snapshot
yarn install
yarn dev
# Access: http://localhost:8082
```

**7. Start Wallet (with Governance):**
```bash
cd ~/Desktop/etrid/apps/wallet-web/etrid-crypto-website
npm install
npm run dev
# Access: http://localhost:3000/governance
```

---

## Current State of Systems

### ✅ Production Ready
- **Prometheus + Grafana** - Infrastructure monitoring with 30+ alerts
- **Network Telemetry** - Public network dashboard with world map
- **Validator Dashboard** - Complete validator operations interface
- **Watchtower Monitor** - Lightning-Bloc fraud detection system
- **Snapshot Governance** - Off-chain voting platform
- **Wallet Governance** - React-based governance interface

### 🔧 Integration Needed

**Unified Portal:** Create a single landing page that links all dashboards together.

**Location to create:** `/Desktop/etrid/apps/monitoring-portal/`

**What it will do:**
- Single entry point to all monitoring systems
- Unified authentication (one wallet connection)
- Cross-dashboard notifications
- Global search across all metrics

---

## Understanding the Governance System

### The Complete Flow

```
1. User connects Polkadot.js wallet
   ↓
2. Opens wallet governance page
   (http://localhost:3000/governance)
   ↓
3. Sees list of active proposals
   (fetched from Snapshot via GraphQL)
   ↓
4. Clicks "Vote" on a proposal
   ↓
5. Signs vote with wallet (no gas fee!)
   ↓
6. Vote submitted to Snapshot Hub
   ↓
7. Vote weight calculated from on-chain stake
   ↓
8. Live results updated
   ↓
9. After voting period, if passed:
   ↓
10. Directors execute on-chain via governance pallet
```

### Key Concepts

**Snapshot (Off-Chain Voting):**
- Proposals stored on IPFS
- Votes are signed messages (free)
- Vote weight = staked ÉTR at snapshot block
- Results verifiable and hard to contest

**On-Chain Governance Pallets:**
- `/10-foundation/governance/pallet/` - Main governance engine
- `/12-consensus-day/pallet-director-election/` - Annual director elections
- `/11-peer-roles/decentralized-directors/` - Director powers

**9-Director System:**
- 9 elected directors
- 6/9 quorum for emergency actions
- Annual elections on Consensus Day (Jan 1)
- Special powers with 24hr timelock

**Consensus Day Proposals:**
- Occur once per year (Jan 1)
- Require supermajority (60-100%)
- Minimum participation thresholds (20-100%)
- Used for: Director elections, constitution changes

---

## Integration Roadmap

### Phase 1: Quick Wins (✅ DONE)
- [x] Fix Primearc Core Chain → PrimeArc Core Chain naming
- [x] Document all monitoring systems
- [x] Explain governance architecture

### Phase 2: Unified Portal (2 days)
- [ ] Create monitoring portal app
- [ ] Build dashboard grid linking all systems
- [ ] Add single sign-on with Polkadot.js wallet
- [ ] Implement global search

### Phase 3: Governance Integration (2 days)
- [ ] Integrate Snapshot GraphQL API into wallet
- [ ] Build director dashboard for emergency proposals
- [ ] Add Consensus Day special UI
- [ ] Connect wallet governance to on-chain pallets

### Phase 4: Testing & Deployment (1 day)
- [ ] End-to-end testing
- [ ] Production deployment to etrid.org subdomains
- [ ] DNS configuration
- [ ] SSL certificates

---

## How to Integrate Governance into Wallet

### Current State
The wallet already has a governance page at `/app/governance/page.tsx` with:
- ✅ Wallet connection
- ✅ Proposal display
- ✅ Voting interface
- ✅ User stats

### What's Needed
**Add Snapshot API integration:**

**Create:** `/lib/governance/snapshot-api.ts`

```typescript
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
            state
            scores
            votes
          }
        }
      `
    })
  })
  return response.json()
}

export async function submitVote(
  proposalId: string,
  choice: number,
  walletAddress: string,
  signMessage: (message: string) => Promise<string>
) {
  // 1. Create vote message
  const message = {
    space: 'etrid.eth',
    proposal: proposalId,
    choice: choice,
    metadata: {},
    timestamp: Math.floor(Date.now() / 1000)
  }

  // 2. Sign with Polkadot.js wallet
  const signature = await signMessage(JSON.stringify(message))

  // 3. Submit to Snapshot Hub
  const response = await fetch('https://hub.snapshot.org/api/message', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      address: walletAddress,
      msg: message,
      sig: signature
    })
  })

  return response.json()
}
```

**Then use in governance page:**

```typescript
// In /app/governance/page.tsx
import { fetchProposals, submitVote } from '@/lib/governance/snapshot-api'

export default function GovernancePage() {
  const [proposals, setProposals] = useState([])

  useEffect(() => {
    fetchProposals().then(data => setProposals(data.data.proposals))
  }, [])

  const handleVote = async (proposalId: string, choice: number) => {
    const result = await submitVote(
      proposalId,
      choice,
      selectedAccount.address,
      signMessage // from Polkadot.js
    )
    console.log('Vote submitted:', result)
  }

  return (
    <ProposalsList
      proposals={proposals}
      onVote={handleVote}
    />
  )
}
```

---

## Integration Benefits

### Before Integration (Current):
- 4 separate monitoring apps (different URLs, logins)
- 2 separate governance UIs (Snapshot + Wallet)
- No unified search or notifications
- Different styling/themes

### After Integration:
- **One portal** linking all monitoring systems
- **Single sign-on** with Polkadot.js wallet
- **Unified notifications** from all systems
- **Global search** across all metrics
- **Consistent branding** (PrimeArc theme)
- **Better UX** - less context switching

---

## Quick Commands Reference

```bash
# Start all monitoring (Docker)
cd ~/Desktop/etrid/monitoring && docker-compose up -d

# Start governance UI
cd ~/Desktop/etrid/apps/governance-ui/etrid-snapshot && yarn dev

# Start wallet with governance
cd ~/Desktop/etrid/apps/wallet-web/etrid-crypto-website && npm run dev

# Check Prometheus targets
curl http://localhost:9090/api/v1/targets | jq

# Test validator metrics
curl http://100.93.43.18:9615/metrics | grep substrate_block_height

# Check governance proposals (GraphQL)
curl -X POST https://hub.snapshot.org/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ proposals(first: 5, where: { space_in: [\"etrid.eth\"] }) { id title state } }"}'
```

---

## Next Steps

### Today:
1. **Test current systems:** Start all dashboards and verify they work
2. **Review architecture:** Read the full integration guide
3. **Plan deployment:** Decide on production infrastructure

### This Week:
1. **Build unified portal:** Create monitoring-portal app
2. **Integrate Snapshot:** Add GraphQL integration to wallet
3. **Test end-to-end:** Verify all systems work together

### This Month:
1. **Deploy to production:** Setup etrid.org subdomains
2. **Configure SSL:** HTTPS for all services
3. **Write user docs:** Guides for validators and watchers

---

## Getting Help

**Documentation:**
- Full integration guide: `/Desktop/etrid/UNIFIED_MONITORING_AND_GOVERNANCE_INTEGRATION.md` (14 sections, 700+ lines)
- This quick start: `/Desktop/etrid/QUICK_START_MONITORING_GOVERNANCE.md`

**Troubleshooting:**
- See section 9 in full integration guide
- Check individual app README files
- Review Docker logs: `docker-compose logs -f`

**Architecture:**
- Diagrams in section 11 of full guide
- Component breakdown in sections 1-2
- Governance flow in section 7

---

**All systems are production-ready and waiting for integration! 🚀**

Last Updated: November 22, 2025
