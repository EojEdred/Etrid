# Package Import Quick Reference

Quick reference for importing from @etrid packages in your portal app.

---

## @etrid/ui - Components

### Layout Components
```tsx
import {
  Button,
  Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter,
  Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription,
  Tabs, TabsList, TabsTrigger, TabsContent,
  Separator
} from '@etrid/ui'
```

### Data Display
```tsx
import {
  Table, TableHeader, TableBody, TableRow, TableHead, TableCell,
  Badge,
  Avatar, AvatarImage, AvatarFallback
} from '@etrid/ui'
```

### Forms
```tsx
import {
  Input,
  Select, SelectTrigger, SelectContent, SelectItem, SelectValue,
  Slider,
  Switch,
  Checkbox,
  Label
} from '@etrid/ui'
```

### Feedback
```tsx
import {
  Alert, AlertTitle, AlertDescription,
  Progress,
  Skeleton,
  LoadingSpinner,
  EmptyState
} from '@etrid/ui'
```

### Blockchain-Specific
```tsx
import {
  AddressDisplay,
  BalanceDisplay,
  BlockHeight,
  ValidatorBadge,
  NetworkStatus,
  StatCard
} from '@etrid/ui'
```

### Example Usage
```tsx
import { Button, Card, CardHeader, CardTitle, CardContent } from '@etrid/ui'

export function MyComponent() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>My Card</CardTitle>
      </CardHeader>
      <CardContent>
        <Button variant="default">Click Me</Button>
      </CardContent>
    </Card>
  )
}
```

---

## @etrid/hooks - React Hooks

### Blockchain Hooks
```tsx
import {
  usePolkadotApi,
  useWallet,
  useBalance,
  useBlockNumber,
  useValidatorStats
} from '@etrid/hooks'
```

### Utility Hooks
```tsx
import {
  useDebounce,
  useLocalStorage,
  useMediaQuery
} from '@etrid/hooks'
```

### Example Usage
```tsx
import { usePolkadotApi, useWallet, useBalance } from '@etrid/hooks'

export function WalletComponent() {
  const { api, isConnected } = usePolkadotApi('wss://rpc.etrid.org/primearc')
  const { connect, selectedAccount, accounts } = useWallet('Etrid Portal')
  const { balance, isLoading } = useBalance(api, selectedAccount?.address)

  return (
    <div>
      {!selectedAccount ? (
        <button onClick={connect}>Connect Wallet</button>
      ) : (
        <div>
          <p>Address: {selectedAccount.address}</p>
          <p>Balance: {balance?.free}</p>
        </div>
      )}
    </div>
  )
}
```

---

## @etrid/types - TypeScript Types

### Blockchain Types
```tsx
import type {
  Validator,
  Nominator,
  Reward,
  Balance,
  Block,
  Transaction,
  ChainInfo
} from '@etrid/types'
```

### Governance Types
```tsx
import type {
  Proposal,
  Vote,
  Director,
  ConsensusDay,
  GovernanceSpace
} from '@etrid/types'
```

### Monitoring Types
```tsx
import type {
  NetworkStats,
  ValidatorHealth,
  Alert,
  PerformanceMetric,
  NodeInfo,
  TelemetryData
} from '@etrid/types'
```

### Lightning Types
```tsx
import type {
  LightningChannel,
  LightningPayment,
  LightningInvoice,
  LightningNodeInfo,
  WatchtowerSubscription,
  FraudAlert
} from '@etrid/types'
```

### Wallet Types
```tsx
import type {
  WalletAccount,
  Token,
  TokenSwap,
  StakingPosition,
  LiquidityPool,
  LPPosition
} from '@etrid/types'
```

### Example Usage
```tsx
import type { Validator, NetworkStats } from '@etrid/types'

export function ValidatorList() {
  const [validators, setValidators] = useState<Validator[]>([])
  const [stats, setStats] = useState<NetworkStats | null>(null)

  // ... fetch logic

  return (
    <div>
      {validators.map((validator: Validator) => (
        <div key={validator.address}>
          <h3>{validator.name}</h3>
          <p>Commission: {validator.commission}%</p>
          <ValidatorBadge status={validator.status} />
        </div>
      ))}
    </div>
  )
}
```

---

## @etrid/utils - Utility Functions

### Formatters
```tsx
import {
  formatAddress,
  formatBalance,
  formatLargeNumber,
  formatTimeAgo,
  formatPercentage,
  formatDuration
} from '@etrid/utils'
```

### Validators
```tsx
import {
  isValidAddress,
  isValidEthereumAddress,
  isValidEmail,
  isValidUrl,
  isValidStakeAmount
} from '@etrid/utils'
```

### Helpers
```tsx
import {
  sleep,
  truncate,
  copyToClipboard,
  debounce,
  throttle,
  groupBy,
  unique,
  sortBy
} from '@etrid/utils'
```

### Class Name Utility
```tsx
import { cn } from '@etrid/utils'
```

### Example Usage
```tsx
import { formatAddress, formatBalance, cn } from '@etrid/utils'
import { AddressDisplay, BalanceDisplay } from '@etrid/ui'

export function ValidatorCard({ validator, isActive }) {
  return (
    <div className={cn("p-4 rounded-lg", isActive && "border-2 border-primary")}>
      <AddressDisplay address={validator.address} />
      <BalanceDisplay
        balance={formatBalance(validator.totalStake, 12)}
        symbol="ÉTR"
      />
    </div>
  )
}
```

---

## Common Patterns

### 1. Validator Dashboard
```tsx
import { usePolkadotApi, useValidatorStats } from '@etrid/hooks'
import { Card, CardHeader, CardTitle, CardContent, ValidatorBadge } from '@etrid/ui'
import { formatBalance } from '@etrid/utils'
import type { Validator } from '@etrid/types'

export function ValidatorDashboard({ address }: { address: string }) {
  const { api } = usePolkadotApi('wss://rpc.etrid.org/primearc')
  const { stats, isLoading } = useValidatorStats(api, address)

  if (isLoading) return <LoadingSpinner />

  return (
    <Card>
      <CardHeader>
        <CardTitle>Validator Stats</CardTitle>
      </CardHeader>
      <CardContent>
        <ValidatorBadge status={stats?.isActive ? 'active' : 'inactive'} />
        <p>Commission: {stats?.commission}%</p>
        <p>Total Stake: {formatBalance(stats?.totalStake || '0', 12)} ÉTR</p>
        <p>Nominators: {stats?.nominatorCount}</p>
      </CardContent>
    </Card>
  )
}
```

### 2. Wallet Connection
```tsx
import { useWallet, useBalance } from '@etrid/hooks'
import { Button, Card, AddressDisplay, BalanceDisplay } from '@etrid/ui'
import type { WalletAccount } from '@etrid/types'

export function WalletWidget() {
  const { connect, disconnect, selectedAccount, accounts } = useWallet()
  const { balance } = useBalance(api, selectedAccount?.address)

  if (!selectedAccount) {
    return <Button onClick={connect}>Connect Wallet</Button>
  }

  return (
    <Card>
      <AddressDisplay address={selectedAccount.address} />
      <BalanceDisplay balance={balance?.free || '0'} />
      <Button variant="outline" onClick={disconnect}>Disconnect</Button>
    </Card>
  )
}
```

### 3. Network Status Bar
```tsx
import { useBlockNumber } from '@etrid/hooks'
import { NetworkStatus, BlockHeight } from '@etrid/ui'
import type { NetworkStats } from '@etrid/types'

export function StatusBar() {
  const { blockNumber, finalizedBlock } = useBlockNumber(api)

  return (
    <div className="flex items-center gap-4">
      <NetworkStatus
        status={api?.isConnected ? 'connected' : 'disconnected'}
        peersCount={25}
      />
      <BlockHeight
        height={blockNumber || 0}
        finalized={finalizedBlock || 0}
      />
    </div>
  )
}
```

### 4. Proposal List (Governance)
```tsx
import { Card, Badge, Button } from '@etrid/ui'
import { formatTimeAgo } from '@etrid/utils'
import type { Proposal } from '@etrid/types'

export function ProposalList({ proposals }: { proposals: Proposal[] }) {
  return (
    <div className="space-y-4">
      {proposals.map((proposal) => (
        <Card key={proposal.id}>
          <h3>{proposal.title}</h3>
          <Badge variant={proposal.state === 'active' ? 'success' : 'default'}>
            {proposal.state}
          </Badge>
          <p>{formatTimeAgo(proposal.end)}</p>
          <Button>Vote</Button>
        </Card>
      ))}
    </div>
  )
}
```

---

## Import All from One Package

If you need everything:

```tsx
// Import all UI components
import * as UI from '@etrid/ui'

// Import all hooks
import * as Hooks from '@etrid/hooks'

// Import all types
import * as Types from '@etrid/types'

// Import all utils
import * as Utils from '@etrid/utils'

// Usage
<UI.Button>Click</UI.Button>
const formatted = Utils.formatAddress(address)
const { api } = Hooks.usePolkadotApi(endpoint)
```

---

## Package.json Setup

Add to your portal app's `package.json`:

```json
{
  "name": "portal",
  "dependencies": {
    "@etrid/ui": "workspace:*",
    "@etrid/hooks": "workspace:*",
    "@etrid/types": "workspace:*",
    "@etrid/utils": "workspace:*"
  }
}
```

Then run:
```bash
pnpm install
```

---

## TypeScript Configuration

Ensure your `tsconfig.json` includes:

```json
{
  "compilerOptions": {
    "paths": {
      "@etrid/ui": ["../../packages/ui/src"],
      "@etrid/hooks": ["../../packages/hooks/src"],
      "@etrid/types": ["../../packages/types/src"],
      "@etrid/utils": ["../../packages/utils/src"]
    }
  }
}
```

---

## Tailwind Configuration

Include package paths in `tailwind.config.js`:

```js
module.exports = {
  content: [
    './app/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
    '../../packages/ui/src/**/*.{ts,tsx}',
  ],
}
```

---

## Auto-Import Setup (VS Code)

Add to `.vscode/settings.json`:

```json
{
  "typescript.preferences.importModuleSpecifier": "non-relative",
  "javascript.preferences.importModuleSpecifier": "non-relative"
}
```

This enables auto-imports like:
```tsx
import { Button } from '@etrid/ui'  // ✅ Good
```

Instead of:
```tsx
import { Button } from '../../packages/ui/src/button'  // ❌ Bad
```

---

**Happy coding with @etrid packages!**
