# Phase 6: Visual Reference & UI Guide

## Component Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│                    WalletDashboard                          │
│  ┌──────────────────────────────────────────────────────┐   │
│  │         ChainSelectorProvider (Context)              │   │
│  │                                                      │   │
│  │  ┌────────────────────────────────────────────┐     │   │
│  │  │           Header Bar                       │     │   │
│  │  │  ┌──────────┐  ┌──────────────────────┐   │     │   │
│  │  │  │   Logo   │  │  ChainSelector       │   │     │   │
│  │  │  └──────────┘  │  [ETR ▼] [Address]   │   │     │   │
│  │  │                └──────────────────────┘   │     │   │
│  │  └────────────────────────────────────────────┘     │   │
│  │                                                      │   │
│  │  ┌────────────────────────────────────────────┐     │   │
│  │  │     Chain Info Banner                      │     │   │
│  │  │  [Icon] Primearc Core                      │     │   │
│  │  │         Native ETR token        [🟢 Connected]   │   │
│  │  └────────────────────────────────────────────┘     │   │
│  │                                                      │   │
│  │  ┌────────────────────────────────────────────┐     │   │
│  │  │     Portfolio Overview                      │     │   │
│  │  │  Balance on Primearc Core                  │     │   │
│  │  │  ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┓            │     │   │
│  │  │  ┃    1,234.56 ETR            ┃  [🔄]      │     │   │
│  │  │  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┛            │     │   │
│  │  │  $12,345.67 USD                            │     │   │
│  │  │                      [Send] [Receive]      │     │   │
│  │  └────────────────────────────────────────────┘     │   │
│  │                                                      │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Chain Selector Dropdown

```
┌──────────────────────────────────────────────┐
│  Chain Selector                               │
│  ┌──────────────────────────────────────┐   │
│  │ [🟢] [E] ETR       ▼               │   │
│  │      1,234.56                        │   │
│  └──────────────────────────────────────┘   │
│                                              │
│  ⬇️ When clicked:                            │
│                                              │
│  ┌──────────────────────────────────────┐   │
│  │  ╔════════════════════════════════╗  │   │
│  │  ║  RELAY CHAIN                    ║  │   │
│  │  ╠════════════════════════════════╣  │   │
│  │  ║ [E] Primearc Core         ✓   ║  │   │
│  │  ║     Native ETR token           ║  │   │
│  │  ║     Token: ETR                 ║  │   │
│  │  ╠════════════════════════════════╣  │   │
│  │  ║  PARTITION BURST CHAINS (PBCs) ║  │   │
│  │  ╠════════════════════════════════╣  │   │
│  │  ║ [X] XRP-PBC                    ║  │   │
│  │  ║     Bridged XRP from Ripple    ║  │   │
│  │  ║     Token: bXRP | Port: 9945   ║  │   │
│  │  ╠────────────────────────────────╣  │   │
│  │  ║ [B] BTC-PBC                    ║  │   │
│  │  ║     Bridged Bitcoin            ║  │   │
│  │  ║     Token: bBTC | Port: 9946   ║  │   │
│  │  ╠────────────────────────────────╣  │   │
│  │  ║ [A] ADA-PBC                    ║  │   │
│  │  ║     Bridged Cardano            ║  │   │
│  │  ║     Token: bADA | Port: 9947   ║  │   │
│  │  ╠────────────────────────────────╣  │   │
│  │  ║ ... (9 more chains)            ║  │   │
│  │  ╚════════════════════════════════╝  │   │
│  └──────────────────────────────────────┘   │
└──────────────────────────────────────────────┘
```

## Connection Status Indicators

```
Status Colors:
┌─────────────────────────────────────────┐
│  🟢 Green Dot (Pulsing)                 │
│     → Connected and active              │
│                                         │
│  🟡 Yellow Dot (Pulsing)                │
│     → Connecting / Loading              │
│                                         │
│  🔴 Red Dot (Solid)                     │
│     → Connection error                  │
│                                         │
│  ⚡ Spinner                             │
│     → Switching chains                  │
└─────────────────────────────────────────┘
```

## Chain Icons

Each chain has a colored circular icon with the first letter of its token:

```
┌───────────────────────────────────────────────┐
│  Primearc Core:  [E]  #66D9E6 (Cyan)         │
│  XRP-PBC:        [X]  #23292F (Dark Gray)    │
│  BTC-PBC:        [B]  #F7931A (Orange)       │
│  ADA-PBC:        [A]  #0033AD (Blue)         │
│  DOGE-PBC:       [D]  #C2A633 (Gold)         │
│  TRX-PBC:        [T]  #FF060A (Red)          │
│  MATIC-PBC:      [M]  #8247E5 (Purple)       │
│  BNB-PBC:        [B]  #F3BA2F (Yellow)       │
│  LINK-PBC:       [L]  #2A5ADA (Blue)         │
│  SC-USDT-PBC:    [S]  #26A17B (Green)        │
│  EDSC-PBC:       [E]  #4A90E2 (Light Blue)   │
│  SOL-PBC:        [S]  #14F195 (Neon Green)   │
│  XLM-PBC:        [X]  #14B6E7 (Cyan)         │
└───────────────────────────────────────────────┘
```

## Chain Info Banner

```
┌──────────────────────────────────────────────────────┐
│  ┌──┐                                                │
│  │E │  Primearc Core                    ┌─────────┐ │
│  └──┘  ËTRID Relay Chain - Native ETR   │🟢 Connected│
│        token                             └─────────┘ │
└──────────────────────────────────────────────────────┘
       ↑                ↑                        ↑
    Icon with       Chain Name              Status Badge
  Chain Color     + Description
```

## Balance Display States

```
┌─────────────────────────────────────┐
│  State 1: Loading                   │
│  ┌────────────────────────────────┐ │
│  │ Balance on Primearc Core       │ │
│  │ ▓▓▓▓▓▓▓▓▓▓▓ (Animated pulse)   │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│  State 2: Loaded                    │
│  ┌────────────────────────────────┐ │
│  │ Balance on BTC-PBC             │ │
│  │ 0.12345678 bBTC        [🔄]    │ │
│  │ $4,567.89 USD                  │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│  State 3: Error                     │
│  ┌────────────────────────────────┐ │
│  │ Balance on SOL-PBC             │ │
│  │ ⚠️ Error loading balance        │ │
│  │ [Retry]                        │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

## Mobile Layout (Compact Mode)

```
┌─────────────────────────────┐
│  Mobile View (< 768px)      │
│  ┌────────────────────────┐ │
│  │ ☰  Logo   [E]▼  [...]│ │
│  │           ETR          │ │
│  │           1.2K         │ │
│  └────────────────────────┘ │
│                             │
│  Chain selector becomes     │
│  compact with icon only     │
│  and stacked layout         │
└─────────────────────────────┘
```

## Error States

```
┌──────────────────────────────────────────────┐
│  Connection Error Display                    │
│  ┌────────────────────────────────────────┐  │
│  │  ╔════════════════════════════════════╗ │  │
│  │  ║  ⚠️ CONNECTION ERROR               ║ │  │
│  │  ╠════════════════════════════════════╣ │  │
│  │  ║  Failed to connect to BTC-PBC      ║ │  │
│  │  ║                                    ║ │  │
│  │  ║  Error: WebSocket timeout after    ║ │  │
│  │  ║  5000ms                            ║ │  │
│  │  ║                                    ║ │  │
│  │  ║  [Reconnect]                       ║ │  │
│  │  ╚════════════════════════════════════╝ │  │
│  └────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

## Glass Morphism Styling

```
CSS Properties Applied:
┌──────────────────────────────────────┐
│  .glass-card {                       │
│    background: rgba(255,255,255,0.05)│
│    backdrop-filter: blur(10px)       │
│    border: 1px solid rgba(255,255,255,0.1)
│    border-radius: 12px               │
│    box-shadow: 0 8px 32px rgba(0,0,0,0.1)
│  }                                   │
└──────────────────────────────────────┘

Visual Effect:
┌──────────────────────────────────────┐
│  ╔════════════════════════════════╗  │
│  ║                                ║  │
│  ║  Frosted glass appearance      ║  │
│  ║  with slight transparency      ║  │
│  ║  and blur effect               ║  │
│  ║                                ║  │
│  ╚════════════════════════════════╝  │
└──────────────────────────────────────┘
```

## Gradient Text Effect

```
.gradient-text {
  background: linear-gradient(135deg, #66D9E6, #4DB3CC);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

Example Usage:
┌────────────────────────────┐
│  ╔═══════════════════════╗ │
│  ║  1,234.56 ETR         ║ │
│  ║  (with cyan gradient) ║ │
│  ╚═══════════════════════╝ │
└────────────────────────────┘
```

## Animation States

```
1. Pulse Animation (Connection Status)
   🟢 → 🟢 → 🟢 (Opacity: 1 → 0.5 → 1)
   Duration: 2s, Infinite

2. Spin Animation (Loading/Switching)
   ⚡ → ⚡ → ⚡ (Rotate: 0° → 360°)
   Duration: 1s, Infinite

3. Fade In/Out (Balance Updates)
   Opacity: 0 → 1 (500ms ease-in-out)

4. Slide In (Chain Selector Dropdown)
   Transform: translateY(-10px) → translateY(0)
   Opacity: 0 → 1
   Duration: 200ms
```

## Responsive Breakpoints

```
┌────────────────────────────────────────┐
│  Desktop (≥ 1024px)                    │
│  ┌────────────────────────────────┐    │
│  │  Full layout with all features │    │
│  │  Large chain selector          │    │
│  │  Expanded balance display      │    │
│  └────────────────────────────────┘    │
└────────────────────────────────────────┘

┌────────────────────────────────────────┐
│  Tablet (768px - 1023px)               │
│  ┌────────────────────────────────┐    │
│  │  Stacked layout                │    │
│  │  Moderate chain selector       │    │
│  │  Wrapped action buttons        │    │
│  └────────────────────────────────┘    │
└────────────────────────────────────────┘

┌────────────────────────────────────────┐
│  Mobile (< 768px)                      │
│  ┌────────────────────────────────┐    │
│  │  Compact chain selector        │    │
│  │  Hidden address text           │    │
│  │  Stacked everything            │    │
│  └────────────────────────────────┘    │
└────────────────────────────────────────┘
```

## Color Palette

```
┌──────────────────────────────────────────────┐
│  Primary Brand Colors                        │
│  ─────────────────────                       │
│  Cyan Gradient:   #66D9E6 → #4DB3CC         │
│  Dark Background: #0a0014 (Deep Purple)     │
│  Glass Overlay:   rgba(255,255,255,0.05)    │
│                                              │
│  Status Colors                               │
│  ──────────────                              │
│  Success:         #10B981 (Green)           │
│  Warning:         #F59E0B (Yellow)          │
│  Error:           #EF4444 (Red)             │
│  Info:            #3B82F6 (Blue)            │
│                                              │
│  Text Colors                                 │
│  ───────────                                 │
│  Primary:         #FFFFFF (White)           │
│  Secondary:       rgba(255,255,255,0.60)    │
│  Tertiary:        rgba(255,255,255,0.40)    │
└──────────────────────────────────────────────┘
```

## User Interaction Flow

```
┌────────────────────────────────────────────────┐
│  User Journey: Switching Chains                │
│                                                │
│  Step 1: Click Chain Selector                 │
│  [ETR ▼] ────────► [Dropdown Opens]           │
│                                                │
│  Step 2: View Available Chains                │
│  [Relay Chain]                                 │
│  ✓ Primearc Core (Current)                    │
│  [PBCs]                                        │
│  • XRP-PBC                                     │
│  • BTC-PBC    ◄─── User clicks                 │
│  • ...                                         │
│                                                │
│  Step 3: Switching Animation                  │
│  [⚡ Switching...] ──► [🟡 Connecting...]     │
│                                                │
│  Step 4: Connected to New Chain               │
│  [🟢 BTC-PBC] [0.123 bBTC]                    │
│                                                │
│  Step 5: Balance Auto-Updates                 │
│  Balance on BTC-PBC                           │
│  0.12345678 bBTC ◄─── Fetched automatically   │
│  $4,567.89 USD                                │
└────────────────────────────────────────────────┘
```

## Component Props Reference

```typescript
// ChainSelector Props
<ChainSelector
  address="5GrwvaEF..."     // Wallet address
  showBalance={true}         // Show balance in selector
  compact={false}            // Use compact mobile mode
/>

// ChainBadge Props
<ChainBadge
  className="ml-2"           // Additional CSS classes
/>

// ChainSelectorProvider Props
<ChainSelectorProvider
  initialChainId="primearc-core"  // Starting chain
>
  {children}
</ChainSelectorProvider>
```

## Event Flow Diagram

```
User Action          Hook Function         API Call         State Update
───────────         ─────────────         ────────         ────────────

Click Chain    ──►  switchChain()    ──►  ApiPromise   ──►  setSelectedChain()
  Selector                                  .create()        setIsConnected()
                                                             setApi()

Page Load      ──►  useEffect()      ──►  initApi()    ──►  setIsConnected(true)

Balance Query  ──►  getBalance()     ──►  api.query    ──►  setBalance()
                                          .system
                                          .account()

Error Occurs   ──►  catch(error)     ──►  provider     ──►  setError()
                                          .disconnect()     setIsConnected(false)
```

---

## Quick Visual Reference

```
COMPLETE WALLET DASHBOARD WITH MULTI-CHAIN SUPPORT

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  Header                                                ┃
┃  [ËTRID Logo]  etrid.org   [ETR ▼] [5Grw...7Db] [DC] ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃                                                        ┃
┃  ╔════════════════════════════════════════════════╗   ┃
┃  ║ [E] Primearc Core             🟢 Connected    ║   ┃
┃  ║     ËTRID Relay Chain - Native ETR token      ║   ┃
┃  ╚════════════════════════════════════════════════╝   ┃
┃                                                        ┃
┃  ╔════════════════════════════════════════════════╗   ┃
┃  ║  Balance on Primearc Core                      ║   ┃
┃  ║  ┌──────────────────────────┐          [🔄]   ║   ┃
┃  ║  │  1,234.56 ETR            │                  ║   ┃
┃  ║  └──────────────────────────┘                  ║   ┃
┃  ║  $12,345.67 USD                                ║   ┃
┃  ║                              [Send] [Receive]  ║   ┃
┃  ╚════════════════════════════════════════════════╝   ┃
┃                                                        ┃
┃  ╔════════════════════════════════════════════════╗   ┃
┃  ║  Assets                        [Refresh]       ║   ┃
┃  ╠════════════════════════════════════════════════╣   ┃
┃  ║  [E] ETR                      1,234.56 ETR     ║   ┃
┃  ║      Etrid                    $12,345.67       ║   ┃
┃  ╚════════════════════════════════════════════════╝   ┃
┃                                                        ┃
┃  ╔═══════════╦═══════════╦═══════════╗               ┃
┃  ║ 🟢 Network║ ⚡ TPS    ║ 📦 Block  ║               ┃
┃  ║ Connected ║ 171,000+  ║ 1,234,567 ║               ┃
┃  ╚═══════════╩═══════════╩═══════════╝               ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

This visual guide provides a complete reference for the Phase 6 implementation!
