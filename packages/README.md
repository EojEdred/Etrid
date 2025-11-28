# Etrid Shared Packages

This directory contains shared packages used across all Etrid applications.

## Packages

### @etrid/ui

Shared UI component library built on Radix UI and Tailwind CSS.

**Components (24 total):**
- Layout: Button, Card, Dialog, Tabs, Separator
- Data Display: Table, Badge, Avatar
- Forms: Input, Select, Slider, Switch, Checkbox, Label
- Feedback: Alert, Progress, Skeleton, LoadingSpinner, EmptyState
- Blockchain-specific: AddressDisplay, BalanceDisplay, BlockHeight, ValidatorBadge, NetworkStatus, StatCard

**Usage:**
```tsx
import { Button, Card, AddressDisplay } from '@etrid/ui'

function MyComponent() {
  return (
    <Card>
      <Button>Click me</Button>
      <AddressDisplay address="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" />
    </Card>
  )
}
```

### @etrid/hooks

Shared React hooks for blockchain and utility functions.

**Hooks (8 total):**
- Blockchain: `usePolkadotApi`, `useWallet`, `useBalance`, `useBlockNumber`, `useValidatorStats`
- Utility: `useDebounce`, `useLocalStorage`, `useMediaQuery`

**Usage:**
```tsx
import { useWallet, useBalance } from '@etrid/hooks'

function WalletComponent() {
  const { connect, selectedAccount } = useWallet()
  const { balance } = useBalance(api, selectedAccount?.address)

  return (
    <div>
      <button onClick={connect}>Connect Wallet</button>
      {balance && <p>Balance: {balance.free}</p>}
    </div>
  )
}
```

### @etrid/types

TypeScript type definitions for blockchain entities.

**Type Categories:**
- `blockchain.ts`: Validator, Nominator, Reward, Balance, Block, Transaction, ChainInfo
- `governance.ts`: Proposal, Vote, Director, ConsensusDay, GovernanceSpace
- `monitoring.ts`: NetworkStats, ValidatorHealth, Alert, PerformanceMetric, NodeInfo, TelemetryData
- `lightning.ts`: LightningChannel, LightningPayment, LightningInvoice, LightningNodeInfo, WatchtowerSubscription, FraudAlert
- `wallet.ts`: WalletAccount, Token, TokenSwap, StakingPosition, LiquidityPool, LPPosition

**Usage:**
```tsx
import type { Validator, Proposal, NetworkStats } from '@etrid/types'

const validator: Validator = {
  address: '5GrwvaEF...',
  name: 'My Validator',
  commission: 5,
  totalStake: '1000000000000',
  // ...
}
```

### @etrid/utils

Utility functions for formatting, validation, and helpers.

**Utilities:**
- `formatters.ts`: `formatAddress`, `formatBalance`, `formatLargeNumber`, `formatTimeAgo`, `formatPercentage`, `formatDuration`
- `validators.ts`: `isValidAddress`, `isValidEthereumAddress`, `isValidEmail`, `isValidUrl`, `isValidStakeAmount`
- `helpers.ts`: `sleep`, `truncate`, `copyToClipboard`, `debounce`, `throttle`, `groupBy`, `unique`, `sortBy`
- `cn.ts`: `cn` - Class name merger for Tailwind

**Usage:**
```tsx
import { formatAddress, formatBalance, cn } from '@etrid/utils'

const shortAddr = formatAddress('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY')
// Result: "5Grwva...tQY"

const formatted = formatBalance('1000000000000', 12)
// Result: "1,000"

const className = cn("text-lg", isActive && "font-bold", "text-primary")
```

## Installation

From the monorepo root:

```bash
# Install all dependencies
pnpm install

# The packages are automatically linked via pnpm workspace
```

## Using in Portal App

In your portal app's `package.json`:

```json
{
  "dependencies": {
    "@etrid/ui": "workspace:*",
    "@etrid/hooks": "workspace:*",
    "@etrid/types": "workspace:*",
    "@etrid/utils": "workspace:*"
  }
}
```

Then import:

```tsx
import { Button, Card } from '@etrid/ui'
import { useWallet } from '@etrid/hooks'
import type { Validator } from '@etrid/types'
import { formatAddress } from '@etrid/utils'
```

## Development

Each package has its own `package.json` with dependencies. Changes to packages are automatically reflected in consuming apps during development.

## Package Structure

```
packages/
├── ui/
│   ├── src/
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── address-display.tsx
│   │   └── index.ts
│   └── package.json
├── hooks/
│   ├── src/
│   │   ├── useWallet.ts
│   │   ├── useBalance.ts
│   │   └── index.ts
│   └── package.json
├── types/
│   ├── src/
│   │   ├── blockchain.ts
│   │   ├── governance.ts
│   │   └── index.ts
│   └── package.json
└── utils/
    ├── src/
    │   ├── formatters.ts
    │   ├── validators.ts
    │   └── index.ts
    └── package.json
```

## Component Count Summary

- **@etrid/ui**: 24 components (20 generic + 4 blockchain-specific)
- **@etrid/hooks**: 8 hooks (5 blockchain + 3 utility)
- **@etrid/types**: 35+ type definitions across 5 files
- **@etrid/utils**: 20+ utility functions across 4 files

**Total**: 87+ reusable pieces
