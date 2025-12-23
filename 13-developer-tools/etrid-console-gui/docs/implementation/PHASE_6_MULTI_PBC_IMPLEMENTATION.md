# Phase 6: Multi-PBC Chain Support Implementation

## Overview
Phase 6 adds comprehensive support for ËTRID's Multi-Chain architecture, allowing users to seamlessly switch between the Primearc Core (Relay Chain) and 12 Partition Burst Chains (PBCs).

## Implementation Date
December 3, 2025

## Architecture

### 1. Chain Configuration (`/lib/chains/config.ts`)
Central configuration for all ËTRID chains including:
- **Primearc Core**: Main relay chain (ETR token, 12 decimals)
- **12 PBC Chains**: Specialized parachains for bridged assets

#### PBC Chain List
| Chain | Token | RPC Port | Decimals | Description |
|-------|-------|----------|----------|-------------|
| XRP-PBC | bXRP | 9945 | 6 | Bridged XRP from Ripple |
| BTC-PBC | bBTC | 9946 | 8 | Bridged Bitcoin |
| ADA-PBC | bADA | 9947 | 6 | Bridged Cardano |
| DOGE-PBC | bDOGE | 9948 | 8 | Bridged Dogecoin |
| TRX-PBC | bTRX | 9949 | 6 | Bridged TRON |
| MATIC-PBC | bMATIC | 9950 | 18 | Bridged Polygon |
| BNB-PBC | bBNB | 9951 | 18 | Bridged BNB from BSC |
| LINK-PBC | bLINK | 9952 | 18 | Bridged Chainlink |
| SC-USDT-PBC | scUSDT | 9953 | 6 | Stablecoin USDT |
| EDSC-PBC | EDSC | 9954 | 18 | ËTRID DeFi Stablecoin |
| SOL-PBC | bSOL | 9955 | 9 | Bridged Solana |
| XLM-PBC | bXLM | 9956 | 7 | Bridged Stellar Lumens |

#### Key Features
```typescript
export interface ChainConfig {
  id: string;              // Unique identifier
  name: string;            // Display name
  token: string;           // Token symbol
  decimals: number;        // Token decimals
  wsEndpoint: string;      // WebSocket RPC
  httpEndpoint?: string;   // HTTP RPC (fallback)
  isRelay: boolean;        // True for Primearc Core
  color: string;           // UI color theme
  description: string;     // Chain description
  port?: number;           // RPC port
}
```

#### Utility Functions
- `getChainConfig(chainId)` - Get chain by ID
- `getChainByToken(tokenSymbol)` - Get chain by token
- `getPBCChains()` - Get all PBC chains
- `isValidChainId(chainId)` - Validate chain ID
- `formatTokenAmount(amount, decimals)` - Format token display
- `parseTokenAmount(amount, decimals)` - Parse user input

### 2. Chain Selector Hook (`/hooks/useChainSelector.tsx`)

#### Context Provider: `ChainSelectorProvider`
Manages global chain state and connections.

**Props:**
- `initialChainId` - Starting chain (default: `'primearc-core'`)

**Context Values:**
```typescript
interface ChainSelectorContextType {
  selectedChain: ChainConfig;        // Current chain
  availableChains: ChainConfig[];    // All chains
  switchChain: (chainId) => Promise  // Switch function
  isConnected: boolean;              // Connection status
  isSwitching: boolean;              // Switching in progress
  error: string | null;              // Error message
  getBalance: (address) => Promise   // Get balance
  api: ApiPromise | null;            // Polkadot API
  reconnect: () => Promise;          // Reconnect function
}
```

#### Primary Hook: `useChainSelector()`
Access the chain selector context from any component.

**Usage:**
```typescript
const {
  selectedChain,
  switchChain,
  isConnected
} = useChainSelector();

// Switch to BTC-PBC
await switchChain('btc-pbc');
```

#### Balance Hook: `useChainBalance(address)`
Auto-updates balance for current chain and address.

**Features:**
- Auto-refreshes every 15 seconds
- Updates when chain changes
- Graceful error handling

**Usage:**
```typescript
const { balance, isLoading, error } = useChainBalance(walletAddress);

if (balance) {
  console.log(`Balance: ${balance.formatted} ${selectedChain.token}`);
}
```

### 3. Chain Selector Component (`/components/wallet/ChainSelector.tsx`)

#### Main Component: `<ChainSelector />`
Dropdown menu for selecting chains with live balance display.

**Props:**
```typescript
interface ChainSelectorProps {
  address?: string | null;   // Wallet address
  showBalance?: boolean;     // Show balance in selector
  compact?: boolean;         // Compact mode for mobile
}
```

**Features:**
- Dropdown menu with all chains organized by type
- Real-time connection status indicator
- Live balance display per chain
- Chain-specific color coding
- Error handling with reconnect option
- Mobile-responsive design
- Glass morphism styling

**Visual Indicators:**
- 🟢 Green dot - Connected
- 🟡 Yellow dot - Connecting
- 🔴 Red dot - Connection error
- ⚡ Spinner - Switching chains

#### Compact Component: `<ChainBadge />`
Minimal chain indicator for tight spaces.

**Usage:**
```typescript
<ChainBadge className="ml-2" />
```

### 4. WalletDashboard Integration

#### Provider Wrapper
The entire dashboard is wrapped in `ChainSelectorProvider`:

```typescript
export default function WalletDashboard() {
  return (
    <ChainSelectorProvider initialChainId="primearc-core">
      <WalletDashboardContent />
    </ChainSelectorProvider>
  );
}
```

#### Dashboard Features
1. **Header Integration**
   - Chain selector in top-right
   - Shows current chain and balance
   - Visible when wallet connected

2. **Chain Info Banner**
   - Large display of selected chain
   - Connection status indicator
   - Chain description

3. **Dynamic Balance Display**
   - Shows balance for selected chain
   - Auto-updates on chain switch
   - Token symbol changes per chain

4. **Responsive Design**
   - Desktop: Full chain selector
   - Mobile: Compact mode
   - Tablet: Adaptive layout

## RPC Endpoints

### Production (Mainnet)
```
Base URL: wss://rpc.etrid.org:{port}
Ports: 9945-9956 (see table above)
```

### Development (Local/Tailscale)
```
Base URL: ws://100.96.84.69:{port}
Ports: 9945-9956
```

### Relay Chain
```
Primary: wss://ws.etrid.org/primearc
Fallback: ws://localhost:9944
```

## Usage Examples

### Switching Chains
```typescript
import { useChainSelector } from '@/hooks/useChainSelector';

function MyComponent() {
  const { selectedChain, switchChain, isConnected } = useChainSelector();

  const handleSwitchToBTC = async () => {
    try {
      await switchChain('btc-pbc');
      console.log('Switched to BTC-PBC');
    } catch (error) {
      console.error('Failed to switch:', error);
    }
  };

  return (
    <div>
      <p>Current: {selectedChain.name}</p>
      <button onClick={handleSwitchToBTC}>Switch to BTC</button>
    </div>
  );
}
```

### Getting Balance
```typescript
import { useChainBalance } from '@/hooks/useChainSelector';

function BalanceDisplay({ address }: { address: string }) {
  const { balance, isLoading, error } = useChainBalance(address);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error}</div>;
  if (!balance) return <div>No balance</div>;

  return (
    <div>
      <p>Free: {balance.formatted}</p>
      <p>Reserved: {formatTokenAmount(balance.reserved, decimals)}</p>
    </div>
  );
}
```

### Manual Balance Query
```typescript
const { getBalance, selectedChain } = useChainSelector();

const fetchBalance = async (address: string) => {
  const balance = await getBalance(address);
  if (balance) {
    console.log(`${balance.formatted} ${selectedChain.token}`);
  }
};
```

## Styling

### Glass Card Effect
```css
.glass-card {
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
}
```

### Chain-Specific Colors
Each chain has a unique color (from `ChainConfig.color`):
- Primearc Core: `#66D9E6` (cyan)
- BTC-PBC: `#F7931A` (orange)
- ETH-related: Various blues
- Stablecoins: Green tones

### Gradient Text
```css
.gradient-text {
  background: linear-gradient(135deg, #66D9E6, #4DB3CC);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}
```

## Error Handling

### Connection Errors
- Auto-retry logic (3 attempts)
- Fallback to previous chain on failure
- User-friendly error messages
- Manual reconnect option

### Balance Query Errors
- Graceful degradation
- Shows last known balance
- Retry mechanism

### Network Errors
- Timeout handling (5s)
- WebSocket reconnection
- Provider fallback

## Testing

### Manual Testing Checklist
- [ ] Switch between all 13 chains
- [ ] Verify balance displays correctly per chain
- [ ] Test with and without wallet connection
- [ ] Verify mobile responsive behavior
- [ ] Test error handling (disconnect network)
- [ ] Verify reconnection works
- [ ] Check balance auto-refresh
- [ ] Test chain switching speed
- [ ] Verify status indicators
- [ ] Check dropdown scrolling

### Browser Testing
- [ ] Chrome/Edge (Chromium)
- [ ] Firefox
- [ ] Safari
- [ ] Mobile Safari (iOS)
- [ ] Chrome Mobile (Android)

## Performance Considerations

1. **Connection Pooling**: Single API instance per chain
2. **Balance Caching**: 15-second refresh interval
3. **Lazy Loading**: Components load on-demand
4. **Optimistic Updates**: UI updates before confirmation
5. **Debounced Switching**: Prevents rapid chain switches

## Future Enhancements

### Phase 7 (Proposed)
- [ ] Multi-chain portfolio view (all balances)
- [ ] Cross-chain swaps (PBC to PBC)
- [ ] Transaction history per chain
- [ ] Chain-specific transaction builder
- [ ] Batch operations across chains

### Phase 8 (Proposed)
- [ ] Chain analytics dashboard
- [ ] Bridge monitoring
- [ ] Network health metrics
- [ ] Cross-chain governance voting

## Files Created

1. `/lib/chains/config.ts` - Chain configurations
2. `/hooks/useChainSelector.tsx` - Chain management hook
3. `/components/wallet/ChainSelector.tsx` - UI component

## Files Modified

1. `/components/wallet/WalletDashboard.tsx` - Integrated chain selector

## Dependencies

All required dependencies are already installed:
- `@polkadot/api` (v16.4.9)
- `@polkadot/extension-dapp` (v0.62.2)
- `@polkadot/util` (v13.5.7)

## Troubleshooting

### Chain Not Connecting
1. Check RPC endpoint is reachable
2. Verify port is correct (9945-9956)
3. Check firewall/network settings
4. Try reconnect button

### Balance Shows 0
1. Ensure address is correct format
2. Verify chain has balance
3. Check connection status
4. Wait for block sync

### Slow Switching
1. Network latency issue
2. Multiple rapid switches
3. Node overload
4. Use local node for testing

## Support

For issues or questions:
- GitHub Issues: https://github.com/etrid
- Docs: https://docs.etrid.org
- Discord: https://discord.gg/etrid

---

**Implementation Status**: ✅ Complete
**Testing Status**: ⚠️ Requires testing
**Documentation**: ✅ Complete
