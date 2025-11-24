# Governance Integration - Quick Start Guide

## Testing the Governance Module

### Prerequisites

1. **Unified Portal Running**
   ```bash
   cd /Users/macbook/Desktop/etrid/apps/unified-portal
   npm run dev
   ```
   Portal should be accessible at: `http://localhost:3000`

2. **Snapshot UI Running** (Optional, for iframe test)
   ```bash
   cd /Users/macbook/Desktop/etrid/apps/governance-ui/etrid-snapshot
   yarn install
   yarn dev --port=8082
   ```
   Snapshot UI should be accessible at: `http://localhost:8082`

---

## Testing Scenarios

### 1. Governance Overview Page

**URL:** `http://localhost:3000/governance`

**What to Check:**
- [ ] Stats cards display (12 proposals, 8 votes, 1,250 ÉTR, 67% participation)
- [ ] Three navigation cards (Snapshot, Directors, Consensus Day)
- [ ] Recent proposals section shows 3 proposals
- [ ] Voting progress bars display correctly
- [ ] Info banner explains governance model
- [ ] All links are clickable

**Expected Behavior:**
- Page loads instantly
- All cards render without errors
- Gradient text appears correctly
- Icons display (Vote, Users, Calendar, Shield)

---

### 2. Snapshot Embed Page

**URL:** `http://localhost:3000/governance/snapshot`

#### Scenario A: Snapshot UI Running ✅

**Steps:**
1. Start Snapshot UI on port 8082
2. Navigate to `/governance/snapshot`
3. Wait for iframe to load (~2 seconds)

**Expected:**
- [ ] Connection status shows "Connected" (green badge)
- [ ] Snapshot UI loads inside iframe
- [ ] Debug panel shows wallet address (dev mode)
- [ ] "Open in New Tab" button works
- [ ] No console errors

#### Scenario B: Snapshot UI NOT Running ⚠️

**Steps:**
1. Ensure Snapshot UI is NOT running
2. Navigate to `/governance/snapshot`

**Expected:**
- [ ] Warning alert appears (amber color)
- [ ] Instructions to start Snapshot shown
- [ ] Connection status shows "Connecting" or "Error"
- [ ] Iframe shows connection error

---

### 3. Director Dashboard

**URL:** `http://localhost:3000/governance/directors`

**What to Check:**
- [ ] Four stat cards display (9/9 directors, 1 pending, 6/9 quorum, 45 approvals)
- [ ] Emergency proposals section shows 2 proposals
- [ ] Proposal #1: "Emergency Network Halt" (4/6 approvals, pending)
- [ ] Proposal #2: "Runtime Upgrade" (7/6 approvals, approved, 24h timelock)
- [ ] Progress bars show correct percentages
- [ ] Director list shows all 9 directors
- [ ] Recent actions timeline displays 3 items

**Expected Behavior:**
- Access granted (mock: always director)
- Approve/Reject buttons log to console
- Badges show correct colors (green=approved, amber=pending)
- Timelock countdown displays

**Testing Buttons:**
1. Click "Approve" on Proposal #1
2. Check browser console for: `"Approving proposal: 1"`
3. Click "Reject" on Proposal #1
4. Check browser console for: `"Rejecting proposal: 1"`

---

### 4. Consensus Day Interface

**URL:** `http://localhost:3000/governance/consensus-day`

**What to Check:**
- [ ] Four stat cards (190 days until, 3 proposals, 0% participation, 1/3 votes)
- [ ] Alert shows "Next Consensus Day: June 1, 2026"
- [ ] Alert is amber (not active)
- [ ] Three proposals display with correct types:
  - Constitution (75% required)
  - Director Election (50% required)
  - Protocol Change (67% required)
- [ ] Voting progress bars show correct percentages
- [ ] Candidate section shows 3 candidates
- [ ] Voting buttons are disabled (not June 1st)

**Expected Behavior:**
- Countdown shows days until June 1, 2026
- Supermajority percentages display correctly
- Candidate votes sum to ~100%
- Info panel explains Consensus Day rules

---

## Interactive Testing

### Test 1: Navigation Flow

1. Start at homepage: `http://localhost:3000`
2. Click "Governance" service card
3. Verify arrival at `/governance` overview
4. Click "Active Proposals" card
5. Verify arrival at `/governance/snapshot`
6. Click browser back button
7. Click "Director Dashboard" card
8. Verify arrival at `/governance/directors`

**Expected:** All navigation works without errors

---

### Test 2: Responsive Design

**Steps:**
1. Open `/governance` in browser
2. Open DevTools (F12)
3. Toggle device toolbar (Ctrl+Shift+M)
4. Test breakpoints:
   - Mobile: 375px width
   - Tablet: 768px width
   - Desktop: 1440px width

**Expected:**
- Mobile: Cards stack vertically
- Tablet: 2 columns for navigation cards
- Desktop: 3 columns for navigation cards
- All text remains readable
- No horizontal scroll

---

### Test 3: Snapshot postMessage Bridge

**Prerequisites:** Snapshot UI running on port 8082

**Steps:**
1. Navigate to `/governance/snapshot`
2. Open browser console (F12)
3. Look for log: `[Portal → Snapshot] Sent wallet data: 5GrwvaEF...`
4. Verify wallet address in debug panel matches

**Expected Console Logs:**
```
[Portal → Snapshot] Sent wallet data: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
[Portal] Snapshot iframe loaded
```

**To Modify Snapshot UI** (adds listener):
Edit `apps/governance-ui/etrid-snapshot/src/App.vue`:

```typescript
mounted() {
  window.addEventListener('message', (event) => {
    if (event.data.source === 'etrid-portal') {
      console.log('[Snapshot] Received from portal:', event.data)
      // Handle wallet data here
    }
  })
}
```

---

### Test 4: Director Access Control

**Modify Mock Data** to test access denial:

Edit `/app/governance/directors/page.tsx`:

```typescript
// Line 31: Change isDirector to false
const [currentUser] = useState({
  address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  isDirector: false, // Changed from true
  directorId: 1
})
```

**Expected:**
- Red alert appears: "Access Denied"
- Dashboard content hidden
- Only error message visible

**Restore:** Change back to `isDirector: true`

---

### Test 5: Snapshot API Hooks

**Create Test Page:**

Create `/app/governance/test-api/page.tsx`:

```typescript
"use client"

import { useSnapshotProposals } from "@/lib/hooks/useSnapshotProposals"

export default function TestAPIPage() {
  const { proposals, loading, error } = useSnapshotProposals({
    spaceId: "ens.eth", // Use a known space
    state: "active",
    first: 5
  })

  if (loading) return <div>Loading...</div>
  if (error) return <div>Error: {error}</div>

  return (
    <div className="p-8">
      <h1>Snapshot API Test</h1>
      <pre>{JSON.stringify(proposals, null, 2)}</pre>
    </div>
  )
}
```

**Navigate to:** `http://localhost:3000/governance/test-api`

**Expected:**
- Loading indicator appears briefly
- Proposals from ENS space display
- JSON data is well-formatted
- No errors in console

---

## Common Issues & Solutions

### Issue 1: Snapshot iframe won't load

**Symptoms:** Blank iframe, connection error

**Solution:**
```bash
# Start Snapshot UI
cd apps/governance-ui/etrid-snapshot
yarn dev --port=8082

# Verify it's running
curl http://localhost:8082
```

---

### Issue 2: Components not found

**Symptoms:** Import errors for UI components

**Solution:**
```bash
# Reinstall dependencies
cd apps/unified-portal
npm install
```

---

### Issue 3: TypeScript errors

**Symptoms:** Red squiggly lines, build failures

**Solution:**
```bash
# Clear Next.js cache
rm -rf .next
npm run build
```

---

### Issue 4: Styles not applying

**Symptoms:** Unstyled components, missing Tailwind classes

**Solution:**
```bash
# Rebuild Tailwind
npm run dev

# Or force rebuild
rm -rf .next
npm run dev
```

---

## Verification Checklist

Before reporting completion:

### Functional Tests
- [ ] All 4 governance routes load without errors
- [ ] Navigation between pages works
- [ ] Stats display correctly
- [ ] Buttons are clickable (even if mocked)
- [ ] Progress bars animate

### Visual Tests
- [ ] Colors match design (purple/indigo for governance)
- [ ] Icons display correctly
- [ ] Badges show proper status colors
- [ ] Cards have proper spacing
- [ ] Text is readable in dark/light mode

### Technical Tests
- [ ] No console errors
- [ ] No TypeScript errors
- [ ] No React warnings
- [ ] postMessage logs appear (when Snapshot running)
- [ ] API hooks fetch data (when tested)

### Responsive Tests
- [ ] Mobile view works (375px)
- [ ] Tablet view works (768px)
- [ ] Desktop view works (1440px+)
- [ ] No horizontal scroll on any breakpoint

---

## Next Steps

After testing:

1. **Report Issues**
   - Create GitHub issues for bugs
   - Include browser console logs
   - Attach screenshots

2. **Integrate Wallet** (Agent 4)
   - Connect Polkadot.js wallet
   - Replace mock wallet state
   - Enable real transaction signing

3. **Connect On-chain Data**
   - Query governance pallet
   - Fetch real proposals
   - Display actual votes

4. **Production Deployment**
   - Update environment variables
   - Configure Snapshot production URL
   - Enable analytics

---

## Support

**Agent:** Agent 5 - Governance Integration
**Status:** Ready for testing
**Date:** November 22, 2025

For issues, check:
- `/monitoring/GOVERNANCE_INTEGRATION_REPORT.md` (full documentation)
- Browser console for error messages
- Next.js terminal output for build errors

---

**Happy Testing!** 🎉
