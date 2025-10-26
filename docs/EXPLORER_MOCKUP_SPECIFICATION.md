# Ëtrid Block Explorer - Complete Mockup Specification

**Document ID**: ETRID-EXPLORER-SPEC-2025
**Date**: October 24, 2025
**Purpose**: Full specification for public block explorer
**Target**: GizziClaude for Figma implementation

---

## 🎯 Overview

The Ëtrid Explorer makes the network's state **transparent and accessible** to everyone:
- Real-time block and transaction data
- Governance proposals and voting status
- Treasury and fiscal operations
- Validator and network statistics

**Philosophy**: "The entire decision process is observable, immutable, and verifiable."

---

## 🖥️ Platform Specifications

### Desktop Viewport
- **Resolution**: 1920x1080px (primary)
- **Min Width**: 1280px
- **Max Width**: Unlimited (responsive)

### Tablet Viewport
- **Resolution**: 768x1024px
- **Breakpoint**: 768px - 1279px

### Mobile Viewport
- **Resolution**: 375x812px (iPhone X baseline)
- **Breakpoint**: <768px

---

## 🎨 Design System

### Color Palette (Dark Mode Primary)

```
Background: #000000 (Base Black)
Card Background: #0A1929 (Deep Space Blue)
Border: #475569 (Slate Gray)
Primary Text: #F8FAFC (Warm White)
Secondary Text: #94A3B8 (Light Slate)
Accent: #4FE2C9 (Tech Green-Blue)
Success: #10B981 (Green)
Warning: #F59E0B (Orange)
Error: #EF4444 (Red)
```

### Typography

```
Headers: Inter Semi-Bold
Body: Inter Regular
Code/Data: JetBrains Mono Regular
Icons: Heroicons (outline, 24px)
```

### Spacing Scale (8px Grid)
```
XS: 4px
SM: 8px
MD: 16px
LG: 24px
XL: 32px
2XL: 48px
```

---

## 📐 Layout Structure

### Global Navigation (Fixed Top)

```
┌─────────────────────────────────────────────────────────┐
│ [Ë Logo]  Home  Blocks  Epochs  Governance  Treasury    │
│                                    [Search] [Theme] [•••]│
└─────────────────────────────────────────────────────────┘
```

**Height**: 72px
**Background**: #0A1929 with `backdrop-filter: blur(12px)`
**Border-bottom**: 1px solid #475569 (30% opacity)

**Components**:
- Logo: 48px height, left-aligned (16px margin)
- Nav Links: Inter Medium, 16px, #F8FAFC
- Link Spacing: 32px between items
- Active State: Border-bottom 2px solid #4FE2C9
- Search Bar: 320px width, right-aligned
- Theme Toggle: Dark/Light mode icon
- Menu: Mobile hamburger (768px breakpoint)

---

## 🏠 Page 1: Home / Overview

### Hero Section (Network Status)

**Layout**:
```
┌──────────────────────────────────────────────────────┐
│                  ËTRID NETWORK                        │
│                                                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │Network   │  │Validators│  │Block     │          │
│  │Online    │  │  127     │  │Time 6.2s │          │
│  │ 99.9%    │  │          │  │          │          │
│  └──────────┘  └──────────┘  └──────────┘          │
│                                                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │Current   │  │Treasury  │  │24h Txs   │          │
│  │Epoch #142│  │12.5M ÉTR │  │1.2M      │          │
│  └──────────┘  └──────────┘  └──────────┘          │
└──────────────────────────────────────────────────────┘
```

**Stats Cards**:
- Size: 280px × 120px
- Background: #0A1929
- Border: 1px solid #475569
- Border-radius: 12px
- Padding: 24px
- Icon: 48px (Heroicons)
- Value: Inter Bold, 32px, #F8FAFC
- Label: Inter Regular, 14px, #94A3B8

---

### Recent Blocks Section

```
┌────────────────────────────────────────────────────┐
│  Recent Blocks                         [View All →]│
├────────────────────────────────────────────────────┤
│  Block      Time         Validator     Txs  Status │
│  #1,234,567 6s ago       validator-01  150  ✓     │
│  #1,234,566 12s ago      validator-02  200  ✓     │
│  #1,234,565 18s ago      validator-03  180  ✓     │
│  #1,234,564 24s ago      validator-01  165  ✓     │
│  #1,234,563 30s ago      validator-04  190  ✓     │
└────────────────────────────────────────────────────┘
```

**Table Styling**:
- Background: #0A1929
- Border: 1px solid #475569
- Header: Inter Semi-Bold, 14px, #94A3B8 (uppercase)
- Rows: JetBrains Mono, 14px, #F8FAFC
- Row Height: 56px
- Hover: Background #1E3A5F
- Border-bottom: 1px solid #475569 (20% opacity)

**Columns**:
1. Block Number: Monospace, link (#4FE2C9)
2. Time: Relative (6s ago), Inter Regular
3. Validator: Truncated address with tooltip
4. Transactions: Count (150)
5. Status: ✓ (green) or ⚠️ (yellow) or ✗ (red)

---

### Live Activity Feed

```
┌────────────────────────────────────────────────┐
│  Live Activity                    [Pause] [•••]│
├────────────────────────────────────────────────┤
│  ⚡ New block #1,234,567 validated             │
│  🗳️ Proposal #42 received 100 votes           │
│  💰 Treasury distributed 1,000 ÉTR             │
│  👤 New validator joined: validator-127        │
└────────────────────────────────────────────────┘
```

**Styling**:
- Auto-scroll (can be paused)
- Icon: 20px emoji or SVG
- Text: Inter Regular, 14px
- Background: Subtle gradient per event type
- Animation: Slide-in from bottom

---

## 📦 Page 2: Blocks

### Block List View

**Filters Bar**:
```
[All Blocks ▼]  [Epoch: Current ▼]  [Validator: All ▼]  [Search: Block #]
```

**Block Cards** (Grid or List):

```
┌──────────────────────────────────┐
│  Block #1,234,567               │
│  ────────────────────────────   │
│  Validator: validator-01         │
│  Time: 2025-10-24 14:32:15 UTC  │
│  Transactions: 150               │
│  VMw Used: 2.5M                  │
│  Hash: 0x7a3b...                 │
│  Status: ✓ Finalized (99.9%)    │
└──────────────────────────────────┘
```

**Finality Bar** (ASF Visualization):
```
Progress Bar:
├──────────────────────────────────────────┤ 99.9%
0%                 50%                   100%
[Low]              [Medium]            [High]
```
- Gradient fill (#EF4444 → #F59E0B → #10B981)
- Shows current finality confidence
- Updates in real-time

---

### Block Detail Page

**URL**: `/block/1234567`

```
┌────────────────────────────────────────────────┐
│  Block #1,234,567                              │
│  Status: ✓ Finalized (99.9%)                  │
├────────────────────────────────────────────────┤
│  Overview                                      │
│  ────────                                      │
│  Block Height:      1,234,567                  │
│  Timestamp:         2025-10-24 14:32:15 UTC    │
│  Validator:         validator-01               │
│  Parent Hash:       0x3f2a...                  │
│  Block Hash:        0x7a3b...                  │
│  State Root:        0x9c4d...                  │
│  Transactions:      150                        │
│  VMw Used:          2,500,000 (25% of limit)   │
│  Finality:          99.9% (142 confirmations)  │
├────────────────────────────────────────────────┤
│  Transactions (150)                            │
│  ──────────────                                │
│  [Table of transactions]                       │
└────────────────────────────────────────────────┘
```

---

## 🗳️ Page 3: Governance

### Active Proposals Tab

```
┌────────────────────────────────────────────────┐
│  [Active] [Voting] [Passed] [Rejected]        │
├────────────────────────────────────────────────┤
│                                                │
│  ┌──────────────────────────────────────────┐ │
│  │ 🗳️ Proposal #42                         │ │
│  │ Title: Increase Validator Count to 150   │ │
│  │ ─────────────────────────────────────    │ │
│  │ Proposer: did:etrid:director-05          │ │
│  │ Category: System Evolution               │ │
│  │ Status: Voting (8h remaining)            │ │
│  │                                           │ │
│  │ Citizen Quorum: 68% Yes (50% required)   │ │
│  │ ├───────────────────────────────┤        │ │
│  │                                           │ │
│  │ Validator Quorum: 73% Yes (66% required) │ │
│  │ ├────────────────────────────────┤       │ │
│  │                                           │ │
│  │ [Vote Yes] [Vote No] [View Details →]    │ │
│  └──────────────────────────────────────────┘ │
│                                                │
└────────────────────────────────────────────────┘
```

**Proposal Card**:
- Size: Full-width, min-height 200px
- Background: #0A1929
- Border: 1px solid category color
- Border-radius: 12px
- Padding: 24px

**Quorum Bars**:
- Two progress bars (citizen + validator)
- Color: Green if passing, orange if close, red if failing
- Percentage label on right
- Threshold line marked

---

### Governance Dashboard

**Consensus Day Countdown**:
```
┌────────────────────────────────────────┐
│    NEXT CONSENSUS DAY                  │
│                                        │
│       📅 December 1, 2025              │
│                                        │
│    ⏰ 38 days  12 hours  45 minutes    │
│                                        │
│    Current Phase: Preparation          │
└────────────────────────────────────────┘
```

**4-Phase Visual** (Circular diagram from earlier spec):
- Shows current phase highlighted
- Time remaining per phase
- Phase descriptions

**Historical Data**:
```
┌────────────────────────────────────────┐
│  Past Consensus Days                   │
│  ────────────────                      │
│  Epoch #141 (Oct 2025)                 │
│    Proposals: 12 (10 passed, 2 failed) │
│    Participation: 67.3%                │
│    Minted: 2,000,000 ÉTR               │
│                                        │
│  Epoch #140 (Sep 2025)                 │
│    Proposals: 8 (7 passed, 1 failed)   │
│    Participation: 65.1%                │
│    Minted: 1,800,000 ÉTR               │
└────────────────────────────────────────┘
```

---

## 💰 Page 4: Treasury

### Treasury Overview

```
┌────────────────────────────────────────────────────┐
│  Treasury Dashboard                                │
├────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │Current   │  │Cap       │  │This Epoch│        │
│  │Balance   │  │Remaining │  │Minted    │        │
│  │12.5M ÉTR │  │8M ÉTR    │  │2M ÉTR    │        │
│  └──────────┘  └──────────┘  └──────────┘        │
└────────────────────────────────────────────────────┘
```

**Mint/Burn Chart** (Line Chart):
```
ÉTR
10M ┤                           ╭─ Mint
    │                       ╭───╯
8M  ┤                   ╭───╯
    │               ╭───╯
6M  ┤           ╭───╯
    │       ╭───╯
4M  ┤   ╭───╯
    │╭──╯
2M  ┼─────────────────────────── Burn
    │
0   └──────────────────────────────────→
    Jan  Feb  Mar  Apr  May  Jun  Jul
```

**Styling**:
- Library: Chart.js or similar
- Mint Line: #10B981 (green), 3px
- Burn Line: #EF4444 (red), 3px
- Fill: 20% opacity gradient
- Grid: #475569, 1px, 30% opacity
- Hover: Show exact values

---

### Distribution Breakdown (Pie Chart)

```
     Validators (45%)
          ╱  ╲
    Voters    Directors
    (25%)       (10%)
       ╲      ╱
     Foundation (5%)
         │
     Treasury (15%)
```

**Styling**:
- Donut chart style (center hole)
- Colors match role categories
- Hover: Show percentage + amount
- Legend on right side

---

### Fiscal Ledger Table

```
┌────────────────────────────────────────────────────┐
│  Epoch  Date        Minted    Burned   Net Change  │
├────────────────────────────────────────────────────┤
│  142    Oct 2025    2,000,000 200,000  +1,800,000  │
│  141    Sep 2025    1,800,000 180,000  +1,620,000  │
│  140    Aug 2025    2,200,000 220,000  +1,980,000  │
└────────────────────────────────────────────────────┘
```

**Features**:
- Sortable columns
- Export to CSV
- Filter by epoch range
- Show cumulative totals

---

## 📊 Page 5: Validators

### Validator Leaderboard

```
┌─────────────────────────────────────────────────────┐
│  Rank  Validator         Stake      Uptime  Rewards │
├─────────────────────────────────────────────────────┤
│  #1    validator-01      1.2M ÉTR   99.9%  12.5K    │
│  #2    validator-02      1.1M ÉTR   99.8%  11.8K    │
│  #3    validator-03      1.0M ÉTR   99.5%  11.2K    │
│  ...                                                 │
└─────────────────────────────────────────────────────┘
```

**Columns**:
1. Rank: Based on performance
2. Validator: Name/DID with identicon
3. Stake: Total staked amount
4. Uptime: Last 30 days percentage
5. Rewards: This epoch earned

**Filters**:
- Active / Inactive
- Sort by: Stake, Uptime, Rewards
- Search by name/address

---

### Validator Detail Page

**URL**: `/validator/validator-01`

```
┌────────────────────────────────────────────┐
│  Validator: validator-01                   │
│  Status: 🟢 Active                         │
├────────────────────────────────────────────┤
│  Overview                                  │
│  ────────                                  │
│  DID:           did:etrid:flare:validator-01│
│  Total Stake:   1,200,000 ÉTR              │
│  Self Stake:    200,000 ÉTR (17%)          │
│  Delegations:   1,000,000 ÉTR (83%)        │
│  Commission:    5%                         │
│  Uptime:        99.9% (last 30 days)       │
│  Blocks:        12,345 produced            │
│  Rewards:       12,500 ÉTR (this epoch)    │
├────────────────────────────────────────────┤
│  Performance Chart                         │
│  [Line chart showing uptime over time]     │
├────────────────────────────────────────────┤
│  Recent Blocks (10)                        │
│  [Table of last 10 blocks produced]        │
└────────────────────────────────────────────┘
```

---

## 🔍 Search Functionality

### Global Search Bar

**Supported Queries**:
- Block number: `1234567`
- Block hash: `0x7a3b...`
- Transaction hash: `0x9f2c...`
- Address: `did:etrid:...` or `0x1234...`
- Validator name: `validator-01`
- Proposal ID: `#42`

**Results Format**:
```
┌────────────────────────────────────┐
│  Results for "1234567"             │
├────────────────────────────────────┤
│  📦 Block #1,234,567               │
│     Finalized, 150 transactions    │
│                                    │
│  💳 Transaction 0x1234...          │
│     Block #1,234,567, 6s ago       │
└────────────────────────────────────┘
```

**Autocomplete**: Shows suggestions as you type

---

## 📱 Responsive Design

### Mobile Adaptations

**Navigation**: Hamburger menu
**Cards**: Stack vertically (1 column)
**Tables**: Horizontal scroll or card view
**Charts**: Simplified, touch-friendly
**Typography**: Larger touch targets (48px min)

---

## 🔗 JSON API Structure

### Endpoint: `/api/v1/blocks`

**Response**:
```json
{
  "blocks": [
    {
      "height": 1234567,
      "hash": "0x7a3b...",
      "parentHash": "0x3f2a...",
      "stateRoot": "0x9c4d...",
      "timestamp": "2025-10-24T14:32:15Z",
      "validator": "did:etrid:flare:validator-01",
      "transactions": 150,
      "vmwUsed": 2500000,
      "vmwLimit": 10000000,
      "finality": 99.9,
      "confirmations": 142
    }
  ],
  "pagination": {
    "page": 1,
    "perPage": 20,
    "total": 1234567
  }
}
```

### Endpoint: `/api/v1/governance/proposals`

**Response**:
```json
{
  "proposals": [
    {
      "id": 42,
      "title": "Increase Validator Count to 150",
      "proposer": "did:etrid:director-05",
      "category": "SystemEvolution",
      "status": "Voting",
      "epoch": 142,
      "deposit": 1000,
      "citizenQuorum": {
        "yes": 6800,
        "no": 3200,
        "percentage": 68.0,
        "required": 50.0
      },
      "validatorQuorum": {
        "yes": 92,
        "no": 35,
        "percentage": 73.0,
        "required": 66.0
      },
      "votingEndsAt": "2025-10-25T22:00:00Z"
    }
  ]
}
```

---

## ✅ Implementation Checklist

### Phase 1 (Week 1):
- [ ] Design system setup (colors, typography, spacing)
- [ ] Header navigation component
- [ ] Home page layout (6 stat cards + recent blocks)
- [ ] Footer component

### Phase 2 (Week 2):
- [ ] Block list page
- [ ] Block detail page
- [ ] Search functionality
- [ ] Finality bar component

### Phase 3 (Week 3):
- [ ] Governance dashboard
- [ ] Proposal cards
- [ ] Voting interface (if authenticated)
- [ ] Consensus Day countdown

### Phase 4 (Week 4):
- [ ] Treasury dashboard
- [ ] Charts (mint/burn line, distribution pie)
- [ ] Fiscal ledger table
- [ ] Export functionality

### Phase 5 (Polish):
- [ ] Validator leaderboard
- [ ] Validator detail pages
- [ ] Mobile responsive views
- [ ] Dark/light theme toggle
- [ ] Loading states & animations

---

**Status**: ✅ Complete Specification
**Ready For**: GizziClaude Figma implementation
**Estimated Design Time**: 2-3 weeks

---

*"Transparency through visualization. Trust through verification."*
