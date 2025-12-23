# EDSC Algorithmic Reserve Architecture

**Location:** `/contracts/edsc/`
**Purpose:** Transaction-driven stablecoin reserve with 1:1 USDC/USDT backing
**Initial Reserve:** 100M EDSC

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│              EDSC ALGORITHMIC RESERVE SYSTEM                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  RESERVE VAULT (Multi-Asset)                             │  │
│  │  ├─ USDC Reserve:  50M USDC (50%)                        │  │
│  │  ├─ USDT Reserve:  30M USDT (30%)                        │  │
│  │  ├─ DAI Reserve:   20M DAI  (20%)                        │  │
│  │  ├─ Total Value:   $100M                                 │  │
│  │  └─ Backing Ratio: 1:1 (100%)                            │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          ↕ Bidirectional                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  EDSC MINTING ENGINE                                     │  │
│  │  ├─ Mint EDSC when stablecoins deposited                │  │
│  │  ├─ Route all PBC stablecoin txs here                   │  │
│  │  ├─ Auto-swap BTC/ETH → USDC (1inch)                    │  │
│  │  └─ Circulating: 100M EDSC                              │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          ↓                                      │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  PEG STABILIZATION (Algorithmic)                         │  │
│  │  ├─ Oracle: Chainlink USDC/USD                          │  │
│  │  ├─ Trigger: ±2% deviation from $1                      │  │
│  │  ├─ Action: Mint/burn to rebalance                      │  │
│  │  └─ Target: 1 EDSC = $1 always                          │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Reserve Vault Architecture

### Contract: `EDSCReserveVault.sol`

**Purpose:** Hold multi-asset stablecoin reserves with automatic rebalancing

**Reserve Allocation:**
- 50% USDC (most liquid)
- 30% USDT (diversification)
- 20% DAI (decentralized backup)

**State Variables:**
```solidity
mapping(address => uint256) public reserves; // token → amount
uint256 public totalReserveValue;            // Total in USD
uint256 public targetUSDCPercentage = 50;    // 50%
uint256 public targetUSDTPercentage = 30;    // 30%
uint256 public targetDAIPercentage = 20;     // 20%
```

**Core Functions:**
```solidity
function depositUSDC(uint256 amount) external
function depositUSDT(uint256 amount) external
function depositDAI(uint256 amount) external
function rebalance() external // Auto-rebalance to target %
function getReserveRatio() external view returns (uint256) // Should be 100%
```

**Rebalancing Logic:**
```
IF USDC > 55%: Swap excess USDC → USDT/DAI
IF USDC < 45%: Swap USDT/DAI → USDC
Maintain target: 50% USDC, 30% USDT, 20% DAI
```

---

## EDSC Minting Engine

### Contract: `EDSCMintingEngine.sol`

**Purpose:** Mint EDSC backed 1:1 by reserves from transaction flow

**Minting Methods:**

**1. Direct Stablecoin Minting**
```solidity
function mintWithUSDC(uint256 usdcAmount) external returns (uint256)
function mintWithUSDT(uint256 usdtAmount) external returns (uint256)
function mintWithDAI(uint256 daiAmount) external returns (uint256)
```

**Flow:**
```
User sends USDC → Reserve vault receives → Mint EDSC 1:1 → User receives EDSC
```

**2. Volatile Asset Minting (with auto-swap)**
```solidity
function mintWithBTC(uint256 btcAmount) external returns (uint256)
function mintWithETH(uint256 ethAmount) external returns (uint256)
function mintWithSOL(uint256 solAmount) external returns (uint256)
```

**Flow:**
```
User sends BTC → Auto-swap BTC→USDC (1inch) → USDC to reserve → Mint EDSC → User receives EDSC
```

**3. Cross-PBC Routing**
```solidity
function routeStablecoinPurchase(
    uint32 sourcePBC,
    address user,
    address stablecoin,
    uint256 amount
) external returns (uint256)
```

**Flow:**
```
User buys USDC on ETH-PBC → XCMP message to EDSC-PBC → Route to reserve → Mint EDSC
```

---

## External Swap Integration

### Contract: `ExternalSwapRouter.sol`

**Purpose:** Swap volatile assets → USDC via 1inch/ParaSwap

**Supported Aggregators:**
1. **Primary: 1inch Network**
   - Fusion+ (MEV-protected)
   - 100+ DEXs aggregated
   - 0.5% max slippage

2. **Secondary: ParaSwap**
   - MultiPath routing
   - Augustus V6 contracts
   - Backup if 1inch fails

3. **Tertiary: THORChain**
   - Native cross-chain swaps
   - BTC, SOL direct support

**Function:**
```solidity
function swapToUSDC(
    address assetIn,
    uint256 amountIn,
    uint256 minUSDCOut
) external returns (uint256 usdcReceived)
```

**Integration:**
```javascript
// Example: Swap 0.01 BTC → USDC via 1inch
const response = await fetch('https://api.1inch.dev/swap/v6.0/1/swap', {
    params: {
        src: wBTC_address,
        dst: USDC_address,
        amount: btcAmount * 1e8,
        from: edsc_minter_address,
        slippage: 0.5
    }
});
```

---

## Peg Stabilization Algorithm

### Contract: `EDSCPegStabilizer.sol`

**Purpose:** Maintain 1 EDSC = $1 via autonomous mint/burn

**Oracle Integration:**
```solidity
AggregatorV3Interface public usdcPriceFeed;  // Chainlink USDC/USD
AggregatorV3Interface public edscPriceFeed;  // Chainlink EDSC/USD (future)
```

**Stabilization Logic:**
```
EVERY BLOCK:
    price = getEDSCPrice() // from oracle

    IF price < $0.98:
        // EDSC undervalued
        buyAmount = calculateBuyAmount(price)
        buyEDSC(buyAmount)  // with USDC reserves
        burnEDSC(buyAmount) // reduce supply
        // Supply↓ → Price↑ → Returns to $1

    IF price > $1.02:
        // EDSC overvalued
        mintAmount = calculateMintAmount(price)
        mintEDSC(mintAmount)      // increase supply
        sellEDSC(mintAmount)       // for USDC
        depositToReserve(usdcReceived) // increase reserves
        // Supply↑ → Price↓ → Returns to $1
```

**Parameters:**
```solidity
uint256 public deviationThreshold = 2;  // ±2% trigger
uint256 public maxActionPerBlock = 1e6; // 1M EDSC max per block
uint256 public circuitBreakerThreshold = 10; // ±10% = emergency pause
```

---

## Transaction Flow Diagrams

### Direct Stablecoin Purchase

```
┌──────────────────────────────────────────────────────┐
│ User sends 1000 USDC                                 │
└──────────────────────────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ EDSCMintingEngine.mintWithUSDC(1000)                 │
│ ├─ Transfer 1000 USDC to ReserveVault               │
│ ├─ ReserveVault.depositUSDC(1000)                   │
│ ├─ Verify deposit successful                        │
│ ├─ Mint 1000 EDSC to user                           │
│ └─ Emit: EDSCMinted(user, 1000 USDC, 1000 EDSC)     │
└──────────────────────────────────────────────────────┘
                    ↓
         User receives 1000 EDSC ✓
         Reserve: +1000 USDC
         Backing: 1:1 maintained
```

### Volatile Asset Purchase (Auto-Swap)

```
┌──────────────────────────────────────────────────────┐
│ User sends 0.02 BTC (worth $1000)                    │
└──────────────────────────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ EDSCMintingEngine.mintWithBTC(0.02)                  │
│ ├─ ExternalSwapRouter.swapToUSDC(BTC, 0.02)         │
│ │   ├─ Call 1inch API                               │
│ │   ├─ Execute swap: 0.02 BTC → 995 USDC            │
│ │   └─ Return 995 USDC (5$ slippage)                │
│ ├─ ReserveVault.depositUSDC(995)                    │
│ ├─ Mint 995 EDSC to user (1:1 with USDC received)   │
│ └─ Emit: EDSCMintedFromSwap(user, BTC, 995 EDSC)    │
└──────────────────────────────────────────────────────┘
                    ↓
         User receives 995 EDSC ✓
         Reserve: +995 USDC
         Backing: 1:1 maintained
```

### Cross-PBC Routing

```
┌──────────────────────────────────────────────────────┐
│ User on ETH-PBC swaps ETH → USDC                     │
└──────────────────────────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ ETH-PBC detects stablecoin purchase                  │
│ ├─ Send XCMP message to EDSC-PBC                    │
│ └─ Payload: {user, USDC, 500}                       │
└──────────────────────────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ EDSC-PBC receives routing request                    │
│ ├─ User prompt: "Convert USDC → EDSC?" (optional)   │
│ ├─ If yes: routeStablecoinPurchase(ETH-PBC, ...)    │
│ │   ├─ Transfer USDC to ReserveVault                │
│ │   ├─ Mint 500 EDSC                                │
│ │   └─ Send EDSC back to user on ETH-PBC            │
│ └─ If no: Return USDC to user                       │
└──────────────────────────────────────────────────────┘
                    ↓
         User receives 500 EDSC on ETH-PBC ✓
         Reserve: +500 USDC
```

---

## DAO Governance

### Contract: `EDSCGovernance.sol`

**Governance Powers:**
- Adjust peg deviation threshold (±2%)
- Adjust reserve allocation (50/30/20%)
- Add/remove reserve assets
- Emergency pause/unpause
- Update oracle addresses

**Proposal Lifecycle:**
```
1. Discussion: 3 days (off-chain forum)
2. Voting: 7 days (on-chain)
3. Timelock: 48 hours
4. Execution: Automatic if passed
```

**Voting Power:**
```
1 Staked ÉTR = 1 vote
Time-weighted: 2x for 1-year stakes
```

**Emergency Guardians:**
```
4-of-7 multi-sig can:
- Pause contracts (not modify)
- 24-hour max pause duration
- DAO vote required to extend
```

---

## Security Features

1. **Reserve-First Minting:** Reserve deposit MUST succeed before minting
2. **Reentrancy Protection:** All functions use `nonReentrant`
3. **Oracle Redundancy:** 3 oracles (Chainlink, Band, custom), median price
4. **Circuit Breaker:** Auto-pause if ±10% deviation
5. **Slippage Protection:** 0.5% max on external swaps
6. **Rate Limiting:** Max 1M EDSC mint per block
7. **Multi-sig Control:** 4-of-7 for emergency actions

---

## File Structure

```
contracts/edsc/
├── EDSC_RESERVE_ARCHITECTURE.md (this file)
├── core/
│   ├── EDSCToken.sol
│   ├── EDSCMintingEngine.sol
│   └── EDSCReserveVault.sol
├── stabilization/
│   ├── EDSCPegStabilizer.sol
│   └── ExternalSwapRouter.sol
├── governance/
│   └── EDSCGovernance.sol
├── interfaces/
│   ├── IEDSCMintingEngine.sol
│   ├── IEDSCReserveVault.sol
│   └── IExternalSwapRouter.sol
└── test/
    ├── EDSCMintingEngine.test.js
    ├── EDSCPegStabilizer.test.js
    └── EDSCReserveVault.test.js
```

---

## Implementation Checklist

- [ ] Deploy EDSCToken (ERC20)
- [ ] Deploy EDSCReserveVault with multi-asset support
- [ ] Deploy EDSCMintingEngine
- [ ] Deploy ExternalSwapRouter (1inch integration)
- [ ] Deploy EDSCPegStabilizer
- [ ] Configure Chainlink oracle
- [ ] Set up cross-PBC routing (14 PBCs)
- [ ] Deploy EDSCGovernance
- [ ] Initialize 100M EDSC reserve
- [ ] Integration testing
- [ ] Security audit

---

**Status:** Architecture defined, ready for implementation
**Next:** Code implementation phase
