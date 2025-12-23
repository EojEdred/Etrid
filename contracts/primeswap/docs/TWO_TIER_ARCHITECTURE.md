# PrimeSwap Two-Tier Liquidity Pool Architecture

**Location:** `/contracts/primeswap/`
**Purpose:** Capture external currency value (Tier 1) + Provide trading liquidity (Tier 2)
**Initial Allocation:** 1.25B ÉTR total across 11 pools

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    TWO-TIER POOL SYSTEM                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  TIER 1: External Currency Reserve Pools                       │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  BTC/wBTC Pool    │  ETH/wETH Pool   │  SOL/wSOL Pool   │  │
│  │  Reserve Vault    │  Reserve Vault   │  Reserve Vault   │  │
│  │  1:1 Backing      │  1:1 Backing     │  1:1 Backing     │  │
│  │  Multi-sig        │  Multi-sig       │  Multi-sig       │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          │ wBTC/wETH/wSOL flows down           │
│                          ↓                                      │
│  TIER 2: ÉTR/Wrapped Token Trading Pools                       │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  ÉTR/wBTC Pool   │  ÉTR/wETH Pool  │  ÉTR/wSOL Pool    │  │
│  │  845.75M ÉTR     │  191.4M ÉTR     │  44.5M ÉTR        │  │
│  │  VirtualReserve  │  VirtualReserve │  VirtualReserve   │  │
│  │  AMM (x*y=k)     │  AMM (x*y=k)    │  AMM (x*y=k)      │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          │ ÉTR to users                        │
│                          ↓                                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Pool Allocations (1.25B ÉTR Total)

### Tier 2 Pools (ÉTR Distribution)

| Pool | ÉTR Allocation | Percentage | Virtual Reserve |
|------|----------------|------------|-----------------|
| ÉTR/wBTC | 845,750,000 | 67.66% | 33.83 BTC |
| ÉTR/wETH | 191,400,000 | 15.31% | 95.7 ETH |
| ÉTR/wXRP | 62,400,000 | 4.99% | 312,000 XRP |
| ÉTR/wSOL | 44,500,000 | 3.56% | 2,225 SOL |
| ÉTR/wBNB | 40,000,000 | 3.20% | 160 BNB |
| ÉTR/wDOGE | 26,800,000 | 2.14% | 2,680,000 DOGE |
| ÉTR/wADA | 15,600,000 | 1.25% | 78,000 ADA |
| ÉTR/wLINK | 8,900,000 | 0.71% | 890 LINK |
| ÉTR/wTRX | 6,600,000 | 0.53% | 660,000 TRX |
| ÉTR/wXLM | 4,500,000 | 0.36% | 45,000 XLM |
| ÉTR/wMATIC | 3,500,000 | 0.28% | 7,000 MATIC |
| **TOTAL** | **1,250,000,000** | **100%** | - |

---

## Tier 1: External Currency Reserve Pool

### Contract: `ExternalCurrencyReservePool.sol`

**Purpose:** Permanently lock external currencies, mint wrapped tokens 1:1

**Key Features:**
- One-sided (only external currency)
- 1:1 backing guarantee
- Multi-sig withdrawal only
- Proof of reserves

**State Variables:**
```solidity
address public externalCurrency;      // BTC, ETH, SOL
address public wrappedToken;          // wBTC, wETH, wSOL
uint256 public totalReserves;         // Total locked
address public tier2Pool;             // Authorized recipient
address public multiSig;              // 3-of-5 multi-sig
```

**Core Functions:**
```solidity
function lockAndMint(uint256 amount) external returns (uint256)
function burnAndRelease(uint256 amount, address recipient) external
function getReserveRatio() external view returns (uint256)
```

**Flow:**
```
User locks BTC → BTC added to reserves → wBTC minted → wBTC sent to Tier 2
```

---

## Tier 2: ÉTR/Wrapped Token Trading Pool

### Contract: `ETRWrappedTokenTradingPool.sol`

**Purpose:** Provide trading liquidity for ÉTR ↔ wrapped token swaps

**Key Features:**
- Dual-sided (ÉTR + wrapped token)
- VirtualReserveAMM mechanism
- Constant product formula (x × y = k)
- 0.3% swap fee

**State Variables:**
```solidity
address public etrToken;              // ÉTR
address public wrappedToken;          // wBTC, wETH, etc.
uint256 public etrReserve;            // Real ÉTR
uint256 public wrappedReserveReal;    // Real wrapped
uint256 public wrappedReserveVirtual; // Virtual wrapped
uint256 public k;                     // AMM constant
```

**Core Functions:**
```solidity
function swapETRForWrapped(uint256 amountETR, uint256 minOut) external returns (uint256)
function swapWrappedForETR(uint256 amountWrapped, uint256 minOut) external returns (uint256)
function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut) public pure
```

**AMM Formula:**
```
(x + Δx)(y - Δy) = k
where k = etrReserve × (wrappedReal + wrappedVirtual)
```

---

## Token Flow Diagram

### Deposit Flow (BTC → ÉTR)

```
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: User locks 1 BTC on Bitcoin                        │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: TIER 1 - ExternalCurrencyReservePool (BTC/wBTC)    │
│ ├─ BTC added to reserve vault                              │
│ ├─ totalReserves: 0 → 1 BTC                                │
│ ├─ Mint 1 wBTC                                             │
│ └─ Send wBTC to Tier 2 pool                                │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: TIER 2 - ETRWrappedTokenTradingPool (ÉTR/wBTC)     │
│ ├─ IntentRouter receives 1 wBTC                            │
│ ├─ Calculate: 1 wBTC = 24,974 ÉTR (AMM)                   │
│ ├─ Execute swap                                             │
│ ├─ wBTC consumed by pool                                   │
│ └─ 24,974 ÉTR sent to user                                 │
└─────────────────────────────────────────────────────────────┘
                         ↓
                  User receives ÉTR ✓
```

### Withdrawal Flow (ÉTR → BTC)

```
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: User wants to withdraw BTC                         │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: TIER 2 - Swap ÉTR → wBTC                           │
│ ├─ User sends 25,000 ÉTR                                   │
│ ├─ Calculate: → 1.0 wBTC (AMM)                             │
│ ├─ wBTC transferred to IntentRouter                        │
│ └─ ÉTR added to pool                                       │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: TIER 1 - Burn and Release                          │
│ ├─ Burn 1.0 wBTC                                           │
│ ├─ totalReserves: 1 → 0 BTC                                │
│ ├─ Multi-sig approves                                      │
│ └─ Release 1.0 BTC to user                                 │
└─────────────────────────────────────────────────────────────┘
                         ↓
              User receives BTC on Bitcoin ✓
```

---

## Integration Points

### 1. Bridge Integration
```
Bridge detects BTC lock → Calls Tier1.lockAndMint() → wBTC minted
```

### 2. Router Integration
```
IntentRouter.convert() → Tier2.swapWrappedForETR() → ÉTR to user
```

### 3. Multi-Sig Security
```
Tier1.burnAndRelease() requires 3-of-5 multi-sig approval
```

---

## Security Mechanisms

1. **Reentrancy Protection:** All functions use `nonReentrant` modifier
2. **Access Control:** Only authorized contracts can mint/burn
3. **Rate Limiting:** Max per-transaction and daily limits
4. **Multi-sig:** Required for Tier 1 withdrawals
5. **Circuit Breaker:** Emergency pause capability
6. **Slippage Protection:** `minOut` parameter on all swaps

---

## File Structure

```
contracts/primeswap/
├── TWO_TIER_ARCHITECTURE.md (this file)
├── tier1/
│   ├── ExternalCurrencyReservePool.sol
│   ├── interfaces/
│   │   └── IExternalCurrencyReservePool.sol
│   └── test/
│       └── ExternalCurrencyReservePool.test.js
├── tier2/
│   ├── ETRWrappedTokenTradingPool.sol
│   ├── interfaces/
│   │   └── IETRWrappedTokenTradingPool.sol
│   └── test/
│       └── ETRWrappedTokenTradingPool.test.js
└── router/
    ├── TwoTierBridgeRouter.sol
    └── test/
        └── TwoTierBridgeRouter.test.js
```

---

## Implementation Checklist

- [ ] Deploy 11 Tier 1 pools (BTC, ETH, SOL, BNB, TRX, XRP, ADA, DOGE, LINK, XLM, MATIC)
- [ ] Deploy 11 Tier 2 pools with ÉTR allocations
- [ ] Configure multi-sig (3-of-5) for each Tier 1 pool
- [ ] Wire Tier 1 → Tier 2 connections
- [ ] Deploy TwoTierBridgeRouter
- [ ] Integration testing
- [ ] Security audit

---

**Status:** Architecture defined, ready for implementation
**Next:** Code implementation phase
