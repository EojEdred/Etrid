# Ivory Papers - EDSC Tokenomics Updates Required

**Status:** EDSC smart contracts have been corrected to treasury-backed model.
**Action Needed:** Update Ivory Papers and pallet documentation to match.

---

## Files That Need Updating

### 1. Ivory Paper Vol 2 (Technical)
**File:** `/Users/macbook/Desktop/etrid/docs/specifications/ivory-paper-vol2-technical.md`

**Search for and UPDATE:**
- "150% collateralization"
- "over-collateralized"
- "Users lock ËTR to mint EDSC"
- "Liquidation at 120%"

**Replace with:**
- "Treasury-backed model"
- "100% backed by purchase value"
- "Users buy EDSC from reserve with BTC/ETH/SOL/USDC"
- "No liquidations (direct purchase/redemption model)"

---

### 2. Ivory Paper Vol 3 (Governance)
**File:** `/Users/macbook/Desktop/etrid/docs/specifications/ivory-paper-vol3-governance.md`

**Search for and UPDATE:**
- References to collateralization ratios
- Minting mechanisms
- Backing requirements

**Replace with:**
- Treasury reserve model
- Purchase/redemption mechanisms
- Organic backing accumulation

---

### 3. EDSC Pallet Documentation
**File:** `/Users/macbook/Desktop/etrid/src/pallets/pallet-edsc-stability/README.md`

**Current (INCORRECT):**
```markdown
## Key Features

### 1. Reserve Backing
- **Multi-Asset Reserve**: 40% ËTR, 30% sBTC, 20% sETH, 10% Other
- **150% Minimum Collateralization**: Users must maintain at least 150% collateral ratio
- **120% Liquidation Threshold**: Positions can be liquidated below 120%

### 2. Minting & Burning
- **Collateralized Minting**: Lock ËTR (or other approved assets) to mint EDSC
- **Proportional Redemption**: Burn EDSC to retrieve collateral
- **Interest Accrual**: Positions accrue interest based on current rate
```

**Should be (CORRECT):**
```markdown
## Key Features

### 1. Treasury Reserve Model
- **Multi-Asset Treasury**: Holds BTC, ETH, SOL, USDC, and other cryptos
- **100% Backing**: Backed by actual purchase value (not over-collateralized)
- **No Pre-Funding**: Reserve builds organically as users purchase EDSC

### 2. Purchase & Redemption
- **Direct Purchase**: Users buy EDSC from reserve at $1.00 with any approved crypto
- **Direct Redemption**: Users sell EDSC back to reserve at $1.00 for crypto
- **No Liquidations**: Simple buy/sell model, no debt positions
```

---

## Key Concept Changes

### OLD Model (Incorrect - MakerDAO-style)
```
❌ Pre-funded with $1M+ collateral
❌ Users lock ËTR to borrow EDSC
❌ 150% collateralization required
❌ Liquidations at 120% ratio
❌ Interest charged on borrowed EDSC
❌ Over-collateralized like DAI
```

### NEW Model (Correct - Treasury-backed)
```
✅ No pre-funding required
✅ Users buy EDSC from reserve with crypto
✅ 100% backed by purchase value
✅ No liquidations (direct buy/sell only)
✅ No interest charges (not a loan)
✅ Treasury-backed like FRAX/RAI
```

---

## Tokenomics Summary (Correct Version)

### EDSC Launch Process

**Day 1: Contract Deployment**
```
1. Deploy EDSC contract on each chain
2. Mint 1 billion EDSC to Reserve Vault
3. Reserve Vault = Multisig address
4. Total circulation: 0 EDSC (all in vault)
5. Reserve backing: $0 (nothing circulating yet)
```

**Day 2+: Users Start Buying**
```
User wants 10,000 EDSC:
1. User sends 10,000 USDC to Reserve Vault
2. Reserve Vault releases 10,000 EDSC to user
3. Result:
   - User holds: 10,000 EDSC
   - Reserve holds: 10,000 USDC
   - Backing ratio: 100% (10K USDC backing 10K EDSC)
```

**After Many Purchases**
```
After 1000 users buy 1M EDSC total:
- Circulation: 1M EDSC
- Reserve holds: $1M in crypto (BTC/ETH/USDC/etc)
- Backing ratio: 100% ($1M backing 1M EDSC)
- Reserve composition:
  * 30% USDC
  * 25% ETH
  * 20% BTC
  * 15% SOL
  * 10% Other
```

---

## Peg Maintenance Mechanism

### How $1.00 Peg is Maintained

**Old (Incorrect) Method:**
```
❌ Adjust interest rates to control supply
❌ Liquidate under-collateralized positions
❌ Complex CDP (Collateralized Debt Position) system
```

**New (Correct) Method:**
```
✅ Reserve always buys/sells at $1.00
✅ Arbitrage keeps DEX prices near $1.00
✅ Simple purchase/redemption mechanism
```

**Arbitrage Example:**
```
If EDSC trades at $1.05 on Uniswap:
1. Arbitrageur buys from Reserve at $1.00
2. Sells on Uniswap at $1.05
3. Profit: $0.05 per EDSC
4. This selling brings Uniswap price down to $1.00

If EDSC trades at $0.95 on Uniswap:
1. Arbitrageur buys on Uniswap at $0.95
2. Redeems with Reserve at $1.00
3. Profit: $0.05 per EDSC
4. This buying brings Uniswap price up to $1.00
```

**No interest rates needed. Peg maintains automatically through arbitrage.**

---

## Smart Contract Functions Needed

### Current Deployment (Partial)
```solidity
✅ constructor(owner, reserveVault) - Mints to vault
✅ transfer() / transferFrom() - Standard ERC-20
✅ burn() - Users can burn EDSC
⏳ buyEDSC() - NOT IMPLEMENTED YET
⏳ sellEDSC() - NOT IMPLEMENTED YET
⏳ getReserveValue() - NOT IMPLEMENTED YET
⏳ getBackingRatio() - NOT IMPLEMENTED YET
```

### Functions to Add (Future)
```solidity
// Purchase EDSC from reserve
function buyEDSC(
    address paymentToken,  // USDC, ETH, BTC address
    uint256 paymentAmount, // Amount to pay
    uint256 minEDSC        // Minimum EDSC to receive
) external;

// Redeem EDSC for crypto from reserve
function sellEDSC(
    uint256 edscAmount,         // EDSC to sell
    address preferredPayment,   // USDC/ETH/BTC
    uint256 minPayment          // Minimum crypto to receive
) external;

// Check reserve backing
function getReserveValue() external view returns (uint256);
function getBackingRatio() external view returns (uint256);
function getReserveComposition() external view returns (
    uint256 usdcAmount,
    uint256 ethAmount,
    uint256 btcAmount,
    uint256 solAmount
);
```

---

## Multi-Chain Deployment

### Current Setup (Correct)
```
Each chain gets EDSC deployed:
- Base: EdscBase contract → 1B EDSC to vault
- Arbitrum: EdscArbitrum → 1B EDSC to vault
- Polygon: EdscPolygon → 1B EDSC to vault
- BSC: EdscBSC → 1B EDSC to vault
- Ethereum: EdscEthereum → 1B EDSC to vault
- Solana: EDSC SPL token → 1B EDSC to vault
```

**Each chain has its own reserve:**
- Base reserve holds crypto for Base EDSC
- Arbitrum reserve holds crypto for Arbitrum EDSC
- NOT one global reserve across all chains

**Bridging between chains:**
- Users can bridge EDSC between chains later
- Bridge burns on source chain, mints on destination
- Reserves don't interact (separate per chain)

---

## Documentation Structure

### Recommended Organization

**1. Ivory Paper Vol 2 Section:**
```markdown
## 5. EDSC Stablecoin System

### 5.1 Treasury-Backed Model
EDSC is a USD-pegged stablecoin backed by a multi-asset treasury...

### 5.2 Purchase & Redemption
Users purchase EDSC directly from the reserve at $1.00...

### 5.3 Peg Stability
The $1.00 peg is maintained through arbitrage...

### 5.4 Reserve Composition
The treasury holds a diversified basket of crypto assets...

### 5.5 No Pre-Funding Required
Unlike over-collateralized stablecoins, EDSC does not require...
```

**2. Pallet Documentation:**
```markdown
# pallet-edsc-stability

## Treasury-Backed Stablecoin System

This pallet implements a treasury-backed stablecoin where:
- Users purchase EDSC with BTC/ETH/SOL/USDC
- Reserve vault holds all initial supply
- Backing accumulates organically from purchases
- No over-collateralization required
- Simple buy/sell model maintains $1.00 peg
```

---

## Action Items

### Immediate (Before Mainnet Launch)
- [ ] Update Ivory Paper Vol 2 (technical section)
- [ ] Update Ivory Paper Vol 3 (governance section)
- [ ] Update pallet-edsc-stability/README.md
- [ ] Remove references to "150% collateralization"
- [ ] Remove references to "liquidations"
- [ ] Add treasury-backed model explanation

### Future (After Launch)
- [ ] Implement buyEDSC() function
- [ ] Implement sellEDSC() function
- [ ] Add reserve value tracking
- [ ] Add multi-asset support (BTC, ETH, SOL)
- [ ] Create purchase interface (web UI)
- [ ] Add reserve transparency dashboard

---

## Summary

**What Changed:**
- ❌ OLD: Over-collateralized loan system (like MakerDAO)
- ✅ NEW: Treasury-backed purchase system (like Frax)

**What's Correct Now:**
- ✅ Smart contracts mint to reserve vault
- ✅ 1 billion EDSC goes to multisig
- ✅ No team/investor distribution
- ✅ Treasury-backed model

**What Still Needs Updating:**
- ⏳ Ivory Papers (Vol 2 & 3)
- ⏳ Pallet documentation
- ⏳ Technical specification docs
- ⏳ Any other docs mentioning 150% collateral

---

**Created:** October 31, 2025
**Status:** Contracts corrected, documentation needs updating
**Priority:** HIGH - Update before mainnet launch
