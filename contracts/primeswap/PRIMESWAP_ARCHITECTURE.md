# PrimeSwap Architecture

## Overview

PrimeSwap is Ëtrid's native decentralized exchange running on Primearc Core via ËtwasmVM. It provides liquidity for trading ETR against wrapped external assets (wBTC, wETH, etc.) and the EDSC stablecoin.

## Pool Types

### 1. VirtualReserveAMM Pools (11 pools)

For trading ETR against wrapped PBC tokens (wBTC, wETH, wXRP, etc.)

```
┌─────────────────────────────────────────────────────────────────┐
│              VIRTUAL RESERVE AMM MECHANISM                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   INITIALIZATION (Foundation seeds ETR only)                   │
│   ┌────────────────────────────────────────┐                   │
│   │   Real ETR:     845,750,000            │                   │
│   │   Real wBTC:    0                      │                   │
│   │   Virtual wBTC: 33.83 (oracle-derived) │                   │
│   │                                        │                   │
│   │   Initial price: $0.004/ETR            │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   AS USERS BUY ETR WITH BTC:                                   │
│   ┌────────────────────────────────────────┐                   │
│   │   Real ETR:     ↓ decreasing           │                   │
│   │   Real wBTC:    ↑ increasing           │                   │
│   │   Virtual wBTC: ↓ phasing out          │                   │
│   │                                        │                   │
│   │   Price adjusts via AMM formula        │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   AFTER THRESHOLD (real liquidity > threshold):                │
│   ┌────────────────────────────────────────┐                   │
│   │   Real ETR:     600,000,000            │                   │
│   │   Real wBTC:    50                     │                   │
│   │   Virtual wBTC: 0 (phased out)         │                   │
│   │                                        │                   │
│   │   Pure AMM pricing (x * y = k)         │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Single-sided liquidity bootstrapping (ETR only)
- Virtual reserves establish initial price via oracle
- Real liquidity builds as users buy ETR
- Automatic transition to pure AMM when threshold reached
- 0.3% swap fee

### 2. EDSCOraclePool (1 pool)

For trading ETR ↔ EDSC (native stablecoin)

```
┌─────────────────────────────────────────────────────────────────┐
│                 ETR/EDSC ORACLE POOL                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   INITIALIZATION (Protocol mints EDSC)                         │
│   ┌────────────────────────────────────────┐                   │
│   │   Real EDSC:    100,000,000            │                   │
│   │   Real ETR:     0                      │                   │
│   │   Oracle Price: $0.004/ETR             │                   │
│   │                                        │                   │
│   │   Rate: 1 EDSC = 250 ETR               │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   USER WANTS EDSC → DEPOSITS ETR:                              │
│   ┌────────────────────────────────────────┐                   │
│   │   User sends: 250 ETR                  │                   │
│   │   User gets:  1 EDSC (at oracle rate)  │                   │
│   │                                        │                   │
│   │   Real EDSC: ↓ 99,999,999              │                   │
│   │   Real ETR:  ↑ 250                     │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   AFTER AMM TRANSITION (ETR > threshold):                      │
│   ┌────────────────────────────────────────┐                   │
│   │   Pure AMM pricing takes over          │                   │
│   │   Price reflects real supply/demand    │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- EDSC seeded first (reverse of other pools)
- Oracle determines ETR:EDSC rate (EDSC = $1)
- As ETR accumulates, pool grows naturally
- Transitions to AMM after ETR threshold reached

### 3. EDSCPegPool (1 pool)

For EDSC/USDT stability - **Single-sided EDSC seeding**

```
┌─────────────────────────────────────────────────────────────────┐
│            EDSC/USDT STABILITY POOL (Single-Sided)              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   INITIALIZATION (Protocol seeds EDSC only)                    │
│   ┌────────────────────────────────────────┐                   │
│   │   EDSC Reserve: 10,000,000             │                   │
│   │   USDT Reserve: 0 (empty)              │                   │
│   │   Pricing: 1:1 fixed rate              │                   │
│   │   Fee: 0.04%                           │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   USER SWAPS USDT → EDSC:                                      │
│   ┌────────────────────────────────────────┐                   │
│   │   User sends: 1000 USDT                │                   │
│   │   User gets:  ~999.96 EDSC (1:1 - fee) │                   │
│   │                                        │                   │
│   │   EDSC Reserve: ↓ decreasing           │                   │
│   │   USDT Reserve: ↑ increasing           │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
│   AFTER THRESHOLD (USDT > 1M):                                 │
│   ┌────────────────────────────────────────┐                   │
│   │   StableSwap (Curve-style) activates   │                   │
│   │   Amplification: 100 (tight peg)       │                   │
│   │   Both directions available            │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Features:**
- Single-sided bootstrap (EDSC only, no USDT needed)
- 1:1 fixed rate until USDT liquidity builds
- StableSwap activates when USDT > 1M threshold
- 0.04% fee (stablecoin standard)
- EDSC → USDT only available after USDT exists in pool

---

## Pool Distribution

### ETR Allocation (1.25B ETR from Foundation Treasury)

| Pool | Token | Market Cap | Weight | ETR Allocation |
|------|-------|------------|--------|----------------|
| 1 | wBTC | $1,900B | 67.66% | 845,750,000 |
| 2 | wETH | $430B | 15.31% | 191,375,000 |
| 3 | wXRP | $140B | 4.99% | 62,375,000 |
| 4 | wSOL | $100B | 3.56% | 44,500,000 |
| 5 | wBNB | $90B | 3.20% | 40,000,000 |
| 6 | wDOGE | $60B | 2.14% | 26,750,000 |
| 7 | wADA | $35B | 1.25% | 15,625,000 |
| 8 | wLINK | $20B | 0.71% | 8,875,000 |
| 9 | wTRX | $15B | 0.53% | 6,625,000 |
| 10 | wXLM | $10B | 0.36% | 4,500,000 |
| 11 | wMATIC | $8B | 0.28% | 3,625,000 |
| **Total** | | **$2,808B** | **100%** | **1,250,000,000** |

### EDSC Allocation (All Single-Sided)

| Pool | Allocation | Purpose |
|------|------------|---------|
| ETR/EDSC | 100,000,000 EDSC | Native trading pair (users add ETR) |
| EDSC/USDT | 10,000,000 EDSC | Peg stability (users add USDT) |

---

## Price Discovery Mechanism

### Initial State (Single-Sided Liquidity)

```
┌─────────────────────────────────────────────────────────────────┐
│                    PRICE DISCOVERY FLOW                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   1. ORACLE BOOTSTRAPPING                                      │
│   ┌────────────────────────────────────────┐                   │
│   │   Oracle provides ETR price: $0.004    │                   │
│   │   Oracle provides paired token prices  │                   │
│   │   Virtual reserves calculated          │                   │
│   └────────────────────────────────────────┘                   │
│              ↓                                                 │
│   2. USERS BUY ETR                                             │
│   ┌────────────────────────────────────────┐                   │
│   │   User sends wBTC to pool              │                   │
│   │   Pool sends ETR to user               │                   │
│   │   Real wBTC reserve increases          │                   │
│   │   Price adjusts via AMM                │                   │
│   └────────────────────────────────────────┘                   │
│              ↓                                                 │
│   3. LIQUIDITY BUILDS                                          │
│   ┌────────────────────────────────────────┐                   │
│   │   Real reserves grow                   │                   │
│   │   Virtual reserves phase out           │                   │
│   │   Price becomes market-driven          │                   │
│   └────────────────────────────────────────┘                   │
│              ↓                                                 │
│   4. PURE AMM                                                  │
│   ┌────────────────────────────────────────┐                   │
│   │   x * y = k (constant product)         │                   │
│   │   Price = reserve_paired / reserve_ETR │                   │
│   │   No oracle dependency                 │                   │
│   └────────────────────────────────────────┘                   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Contract Architecture

```
PrimeSwapFactory
├── VirtualReserveAMM (11 instances)
│   ├── ETR/wBTC Pool
│   ├── ETR/wETH Pool
│   ├── ETR/wXRP Pool
│   ├── ETR/wSOL Pool
│   ├── ETR/wBNB Pool
│   ├── ETR/wDOGE Pool
│   ├── ETR/wADA Pool
│   ├── ETR/wLINK Pool
│   ├── ETR/wTRX Pool
│   ├── ETR/wXLM Pool
│   └── ETR/wMATIC Pool
├── EDSCOraclePool (1 instance)
│   └── ETR/EDSC Pool
└── EDSCPegPool (1 instance)
    └── EDSC/USDT Pool
```

---

## Deployment Steps

1. **Deploy Tokens** (if not already deployed)
   ```bash
   # ETR, EDSC, wrapped tokens on Primearc
   ```

2. **Deploy Oracle**
   ```bash
   # Price oracle for ETR and paired tokens
   ```

3. **Deploy Factory**
   ```bash
   npx hardhat run scripts/deploy-factory.js --network primearc
   ```

4. **Register PBC Tokens**
   ```bash
   # Register all 11 wrapped tokens with market cap weights
   ```

5. **Create Pools**
   ```bash
   # Create all 13 pools via factory
   ```

6. **Seed Pools**
   ```bash
   npx hardhat run scripts/seed-pools.js --network primearc
   ```

---

## Fee Structure

| Pool Type | Fee | Recipient |
|-----------|-----|-----------|
| VirtualReserveAMM | 0.3% | LP providers |
| EDSCOraclePool | 0.3% | LP providers |
| EDSCPegPool | 0.04% | LP providers |

---

## Security Considerations

1. **Oracle Manipulation**
   - Virtual reserves can only decrease, never increase
   - Multiple oracle sources for redundancy
   - TWAP for price stability

2. **Flash Loan Attacks**
   - Reentrancy guards on all swap functions
   - Price impact limits on large trades

3. **Peg Stability**
   - High amplification coefficient prevents depegging
   - Automatic alerts on deviation > 1%

4. **Liquidity Bootstrapping**
   - Foundation seeds initial liquidity
   - Virtual reserves prevent manipulation during bootstrap

---

## Files Created

```
contracts/primeswap/src/
├── VirtualReserveAMM.sol      # Single-sided liquidity AMM
├── EDSCOraclePool.sol         # ETR/EDSC oracle-priced pool
├── EDSCPegPool.sol            # EDSC/USDT stability pool
└── PrimeSwapFactory.sol       # Factory and registry

contracts/primeswap/scripts/
└── seed-pools.js              # Pool deployment and seeding
```
