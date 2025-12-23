# Staking UI Component Architecture

```
/app/staking/page.tsx
│
└─── <QueryProvider>                     (React Query context)
     │
     └─── <StakingContent>               (Main component - dynamic import)
          │
          ├─── Header Section
          │    ├─── Title & Description
          │    └─── Connect Wallet Button / Connected Address Display
          │
          ├─── Era Info Card
          │    ├─── Current Era
          │    ├─── Session Length
          │    ├─── Minimum Stake
          │    └─── Unbonding Period
          │
          ├─── User Staking Position Card (if wallet connected)
          │    ├─── Stats Grid
          │    │    ├─── Active Stake
          │    │    ├─── Total Bonded
          │    │    ├─── Unlocking Amount
          │    │    └─── Claimable Amount
          │    │
          │    ├─── Current Nominations Section
          │    │    ├─── Nominated Validator Addresses
          │    │    └─── Stop Nominating Button
          │    │
          │    └─── Unbonding Schedule
          │         ├─── Unbonding Items (amount + era)
          │         └─── Withdraw Button (if claimable)
          │
          ├─── Bond/Unbond Forms (2-column grid)
          │    │
          │    ├─── Bond Form Card
          │    │    ├─── Amount Input
          │    │    ├─── Minimum Stake Display
          │    │    └─── Bond/Bond More Button
          │    │
          │    └─── Unbond Form Card
          │         ├─── Amount Input
          │         ├─── Available Balance Display
          │         ├─── Unbond Button
          │         └─── Unbonding Duration Info
          │
          └─── Validators Section Card
               │
               ├─── Header
               │    ├─── Title & Selected Count
               │    └─── Nominate Button (if validators selected)
               │
               ├─── Filters
               │    ├─── Search Input
               │    └─── Tabs (Active / Waiting / All)
               │
               └─── Validator Grid (responsive, scrollable)
                    │
                    └─── <ValidatorCard> (multiple)
                         ├─── Selection Checkbox
                         ├─── Identity/Address Display
                         ├─── Status Badge (Active/Waiting)
                         ├─── Commission Percentage
                         ├─── Total Stake Amount
                         ├─── Nominator Count
                         ├─── Own Stake (if active)
                         ├─── Website Link (if available)
                         ├─── Warning Badge (if blocked)
                         └─── Selection Indicator
```

## Data Flow

```
User Action
    │
    ▼
Component Event Handler
    │
    ▼
Custom Hook (useStaking.ts)
    │
    ├─── Query Hook → Polkadot API → Chain RPC
    │                     │
    │                     ▼
    │                 React Query Cache
    │                     │
    │                     ▼
    │                 Auto-refresh (12s/60s)
    │
    └─── Mutation Hook → Polkadot API → Sign with Extension
                              │
                              ▼
                         Submit Transaction
                              │
                              ▼
                         Wait for Finalization
                              │
                              ▼
                         Toast Notification
                              │
                              ▼
                    Invalidate & Refetch Queries
```

## Hook Dependencies

```
/hooks/useStaking.ts
│
├─── Uses:
│    ├─── @tanstack/react-query (useQuery, useMutation, useQueryClient)
│    ├─── @polkadot/util (formatBalance)
│    └─── React (useState, useCallback)
│
└─── Imports from:
     ├─── /lib/polkadot/staking.ts
     │    ├─── getStakingInfo()
     │    ├─── getValidators()
     │    ├─── getWaitingValidators()
     │    ├─── getEraInfo()
     │    ├─── bond()
     │    ├─── bondExtra()
     │    ├─── unbond()
     │    ├─── withdrawUnbonded()
     │    ├─── nominate()
     │    ├─── chill()
     │    ├─── getMinimumStake()
     │    └─── getBondingDuration()
     │
     └─── /lib/polkadot/api.ts
          └─── initApi()
```

## State Management

### Local State (useState)
- `address` - Connected wallet address
- `isConnecting` - Wallet connection loading state
- `bondAmount` - Bond form input value
- `unbondAmount` - Unbond form input value
- `selectedValidators` - Array of selected validator addresses
- `searchQuery` - Validator search filter
- `validatorFilter` - Active/Waiting/All tab selection

### React Query State (Cached)
- Staking info per address
- Validators list
- Waiting validators list
- Era information
- Minimum stake requirement
- Bonding duration

### Mutation State (React Query)
- Bond transaction status
- Bond extra transaction status
- Unbond transaction status
- Withdraw transaction status
- Nominate transaction status
- Chill transaction status

## Styling Architecture

```
Global Styles (/app/globals.css)
│
├─── .gradient-bg-animated (page background)
├─── .glass-card (card background with blur)
├─── .gradient-text (title gradient)
└─── Tailwind utility classes

Component-Level Styles
│
├─── Responsive grids (grid-cols-1 md:grid-cols-2 lg:grid-cols-3)
├─── Conditional classes (isSelected, isActive, etc.)
├─── Hover effects (hover:border-accent/50)
└─── Loading animations (animate-pulse)
```

## API Connection

```
wss://ws.etrid.org/primearc
    │
    ▼
Polkadot.js ApiPromise
    │
    ├─── query.staking.*
    │    ├─── ledger()
    │    ├─── nominators()
    │    ├─── validators()
    │    ├─── activeEra()
    │    └─── bonded()
    │
    ├─── query.session.*
    │    ├─── validators()
    │    └─── currentIndex()
    │
    ├─── query.identity.*
    │    └─── identityOf()
    │
    ├─── tx.staking.*
    │    ├─── bond()
    │    ├─── bondExtra()
    │    ├─── unbond()
    │    ├─── withdrawUnbonded()
    │    ├─── nominate()
    │    └─── chill()
    │
    └─── consts.staking.*
         ├─── minNominatorBond
         └─── bondingDuration
```

## User Journey

1. **Visit /staking**
   - Page loads with dynamic import
   - Shows loading animation
   - Initializes API connection

2. **Connect Wallet**
   - Click "Connect Wallet"
   - Polkadot.js extension popup
   - Select account
   - Address displayed in header

3. **View Staking Status**
   - Auto-fetch staking info
   - Display active stake, bonded, unlocking
   - Show current nominations
   - Display unbonding schedule

4. **Bond Tokens**
   - Enter amount in bond form
   - Click "Bond Tokens" or "Bond More"
   - Sign transaction in extension
   - Wait for confirmation
   - See updated stake

5. **Select Validators**
   - Browse validator list
   - Use search/filters
   - Click validators to select (max 16)
   - See stats: commission, stake, nominators

6. **Nominate**
   - Click "Nominate X Validators"
   - Sign transaction
   - Wait for confirmation
   - See nominations in position card

7. **Unbond (if needed)**
   - Enter amount in unbond form
   - Click "Unbond Tokens"
   - Sign transaction
   - See unbonding schedule updated

8. **Withdraw (after unbonding period)**
   - Wait for unbonding period to complete
   - Click "Withdraw X ETR"
   - Sign transaction
   - Receive tokens back
