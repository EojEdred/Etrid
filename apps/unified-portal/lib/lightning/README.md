# Lightning-Bloc UI Integration

Complete UI integration for the Ëtrid Lightning Cross-Chain Payment Network.

## Overview

This Lightning UI provides:
- **Cross-chain payments** - Send ETH, receive BTC (or any combination of 14 chains)
- **Channel management** - View and manage Lightning channels
- **Payment history** - Track all transactions
- **Network statistics** - Real-time network metrics
- **Oracle integration** - Live exchange rates

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    React/Next.js UI                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ PaymentCard  │  │ ChannelsList │  │ PaymentHistory│     │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │              │
│         └──────────────────┴──────────────────┘              │
│                            │                                 │
│                    ┌───────▼────────┐                        │
│                    │  useLightning  │                        │
│                    │   React Hook   │                        │
│                    └───────┬────────┘                        │
│                            │                                 │
│                    ┌───────▼────────┐                        │
│                    │ LightningClient│                        │
│                    │   HTTP/WS API  │                        │
│                    └───────┬────────┘                        │
└────────────────────────────┼─────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│              Substrate Node with Lightning Pallet           │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Cross-PBC Router (Rust)                 │  │
│  │  - Route finding                                     │  │
│  │  - HTLC creation                                     │  │
│  │  - Oracle integration                                │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Files Created

### Core Infrastructure
- **`lib/lightning/types.ts`** - TypeScript types for Lightning network
- **`lib/lightning/client.ts`** - API client for backend communication
- **`lib/lightning/useLightning.ts`** - React hook for Lightning state management

### UI Components
- **`app/lightning/page.tsx`** - Main Lightning payments page
- **`components/lightning/payment-card.tsx`** - Cross-chain payment form
- **`components/lightning/lightning-header.tsx`** - Page header with wallet
- **`components/lightning/channels-list.tsx`** - Lightning channels display
- **`components/lightning/payment-history.tsx`** - Transaction history
- **`components/lightning/network-stats.tsx`** - Network statistics
- **`components/lightning/cross-chain-info.tsx`** - Info card about features

## Setup

### 1. Install Dependencies (Already Done)

The required dependencies are already in `package.json`:
- `@polkadot/api` - Substrate connection
- `@polkadot/extension-dapp` - Wallet integration
- Radix UI components
- Next.js 15 + React 19

### 2. Configure Backend Connection

Add to `.env.local`:

```bash
# Lightning Network Backend
NEXT_PUBLIC_LIGHTNING_API_URL=http://localhost:9944
NEXT_PUBLIC_LIGHTNING_WS_URL=ws://localhost:9944
```

For production:
```bash
NEXT_PUBLIC_LIGHTNING_API_URL=https://lightning-api.etrid.io
NEXT_PUBLIC_LIGHTNING_WS_URL=wss://lightning-api.etrid.io
```

### 3. Add Navigation Link

Update your main navigation to include the Lightning page:

```tsx
// In your main nav component
<Link href="/lightning">
  <Button>
    <Zap className="mr-2 h-4 w-4" />
    Lightning
  </Button>
</Link>
```

### 4. Backend API Requirements

Your Substrate node needs to expose these HTTP endpoints:

```rust
// In your runtime or API server

// GET /lightning/channels
// Returns: { channels: Channel[] }

// GET /lightning/payments
// Returns: { payments: Payment[] }

// GET /lightning/stats
// Returns: { stats: NetworkStats }

// POST /lightning/route
// Body: { source_chain, dest_chain, source_address, dest_address, amount }
// Returns: { route: CrossPBCRoute }

// POST /lightning/send
// Body: { route, source_address, dest_address }
// Returns: { payment_id, status }

// POST /lightning/channels/open
// Body: { chain, counterparty, capacity }
// Returns: { channel_id }

// POST /lightning/channels/:id/close
// Returns: { success: true }

// GET /lightning/rates?from=eth-pbc&to=btc-pbc
// Returns: { rate: ExchangeRate }
```

### 5. WebSocket Events

For real-time updates, implement WebSocket events:

```json
{
  "type": "payment_update",
  "payment_id": "0x123...",
  "status": "completed"
}

{
  "type": "channel_update",
  "channel_id": "0x456...",
  "state": "active"
}

{
  "type": "network_stats",
  "stats": { ... }
}
```

## Usage Examples

### Basic Payment Flow

```tsx
import { useLightning } from "@/lib/lightning/useLightning"

function MyComponent() {
  const lightning = useLightning()

  async function sendPayment() {
    // 1. Find route
    const route = await lightning.findRoute({
      sourceChain: "eth-pbc",
      destChain: "btc-pbc",
      sourceAddress: "0x123...",
      destAddress: "bc1q...",
      amount: "1000000000000000000", // 1 ETH in wei
    })

    // 2. Send payment
    if (route) {
      await lightning.sendPayment({
        route,
        sourceAddress: "0x123...",
        destAddress: "bc1q...",
      })
    }
  }

  return <button onClick={sendPayment}>Send</button>
}
```

### Open Lightning Channel

```tsx
async function openChannel() {
  await lightning.openChannel({
    chain: "eth-pbc",
    counterparty: "0xabc...",
    capacity: "5000000000000000000", // 5 ETH
  })
}
```

### Real-time Updates

```tsx
useEffect(() => {
  const unsubscribe = lightning.client.subscribeToEvents((event) => {
    if (event.type === "payment_update") {
      console.log("Payment updated:", event)
      lightning.refresh()
    }
  })

  return unsubscribe
}, [])
```

## UI Features

### Payment Card
- **Chain selection** - Choose source and destination chains
- **Amount input** - Enter payment amount
- **Route preview** - See exchange rate and fees before confirming
- **Two-step confirmation** - Preview then confirm

### Channels List
- **Visual indicators** - See channel states (active, pending, closing)
- **Balance bars** - Visual representation of local/remote balances
- **Chain grouping** - Organized by blockchain

### Payment History
- **Send/Receive icons** - Clear visual distinction
- **Status badges** - Pending, completed, failed
- **Cross-chain details** - See both source and destination

### Network Stats
- **Total channels** - Network-wide channel count
- **Total capacity** - Liquidity across all chains
- **Active chains** - Number of chains with activity
- **Success rate** - Payment reliability metric

## Integration with Existing Wallet

The Lightning UI integrates seamlessly with your existing Polkadot wallet:

```tsx
// Uses your existing useWallet hook
const wallet = useWallet()

// All Lightning operations require wallet connection
{wallet.account ? (
  <PaymentCard wallet={wallet} lightning={lightning} />
) : (
  <Button onClick={wallet.connect}>Connect Wallet</Button>
)}
```

## API Client Implementation

The `LightningClient` handles all backend communication:

```typescript
const client = new LightningClient()

// Automatically handles:
// - JSON serialization
// - Error handling
// - Authentication (add JWT if needed)
// - WebSocket reconnection
```

## Styling

All components use:
- **Tailwind CSS** - Utility-first styling
- **Radix UI** - Accessible components
- **Lucide Icons** - Consistent icon set
- **Dark mode ready** - Works with next-themes

## Next Steps

### Required Backend Work

1. **Create HTTP API Server**
   - Wrap your Rust Lightning-Bloc in HTTP server
   - Use `axum` or `actix-web`
   - Expose endpoints listed above

2. **Add WebSocket Support**
   - Real-time event streaming
   - Payment status updates
   - Channel state changes

3. **Authentication**
   - Add wallet signature verification
   - JWT tokens for session management

### Optional Enhancements

1. **QR Code Support**
   - Generate Lightning invoices
   - Scan QR codes for payments

2. **Mobile Optimization**
   - Responsive design (already included)
   - Mobile-specific UX improvements

3. **Advanced Features**
   - Multi-hop route visualization
   - Fee optimization settings
   - Channel rebalancing UI

## Testing

### Local Development

```bash
# Start Next.js dev server
cd apps/wallet-web/etrid-crypto-website
npm run dev

# Open http://localhost:3000/lightning
```

### Mock Data

The UI works with mock data if backend is not available. The `useLightning` hook will show loading states and handle errors gracefully.

## Production Deployment

### Environment Variables

```bash
# .env.production
NEXT_PUBLIC_LIGHTNING_API_URL=https://api.etrid.io
NEXT_PUBLIC_LIGHTNING_WS_URL=wss://api.etrid.io
```

### Build

```bash
npm run build
npm run start
```

## Security Considerations

1. **Never expose private keys in frontend**
2. **Always verify signatures on backend**
3. **Use HTTPS/WSS in production**
4. **Implement rate limiting on API**
5. **Validate all user inputs**
6. **Use CSP headers**

## Support

For questions or issues:
- Check `/07-transactions/lightning-bloc/` for Rust implementation
- Review integration docs at `/LIGHTNING_ORACLE_INTEGRATION_SUMMARY.md`
- See backend API at `lib/lightning/client.ts`

---

**Created:** November 5, 2025
**For:** Ëtrid Lightning Network
**By:** Claude Code
