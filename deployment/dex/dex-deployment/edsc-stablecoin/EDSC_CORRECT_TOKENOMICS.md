# EDSC (Ëtrid Dollar Stablecoin) - Correct Tokenomics

**IMPORTANT:** This document explains the ACTUAL EDSC tokenomics, not the previous incorrect version.

---

## ❌ Previous INCORRECT Model

The old documentation incorrectly stated:
- Requires $1M+ upfront backing before launch
- 150% collateralization requirement
- Owner mints on demand
- Pre-collateralized like MakerDAO/DAI

**This was WRONG.**

---

## ✅ Actual EDSC Model

### How EDSC Really Works

**1. Initial Deployment:**
```
Deploy EDSC contract
↓
Mint 1 billion EDSC
↓
Send ALL tokens to RESERVE VAULT address
↓
Vault holds all EDSC (not in circulation)
```

**2. Users Purchase EDSC:**
```
User wants to buy EDSC
↓
User sends BTC/ETH/SOL/USDC to reserve
↓
Reserve releases equivalent EDSC to user
↓
Reserve now holds: User's crypto
User now holds: EDSC
```

**3. Backing Accumulates Organically:**
```
More users buy EDSC
→ More crypto flows into reserve
→ Reserve value grows
→ EDSC becomes backed by purchase value
```

---

## Reserve Vault Mechanics

### Initial State (Day 1)
```
Reserve Vault holds: 1 billion EDSC
Reserve crypto value: $0
EDSC in circulation: 0
Backing ratio: 0% (nothing in circulation yet)
```

### After First Purchase
```
User 1 buys 10,000 EDSC for 10,000 USDC

Reserve Vault holds: 999,990,000 EDSC + 10,000 USDC
EDSC in circulation: 10,000
Reserve value: $10,000
Backing ratio: 100% ($10,000 USDC backing 10,000 EDSC)
```

### After Many Purchases
```
1000 users buy 1M EDSC total for $1M in crypto

Reserve Vault holds: 999M EDSC + $1M crypto (BTC/ETH/USDC/etc)
EDSC in circulation: 1M
Reserve value: $1M
Backing ratio: 100% ($1M crypto backing 1M EDSC)
```

---

## Key Differences from Traditional Stablecoins

### Traditional Model (MakerDAO, Circle USDC)
```
1. Lock $150M in collateral FIRST
2. THEN mint $100M stablecoin
3. Over-collateralized from day 1
4. Requires huge upfront capital
```

### EDSC Model (Treasury-Backed)
```
1. Mint ALL tokens to reserve vault FIRST
2. Users buy with crypto (backing flows in)
3. Backing = Purchase value (100% ratio)
4. No upfront capital required
5. Organic growth model
```

---

## Where Initial EDSC Goes

### ❌ WRONG: Mint to Owner
```solidity
// WRONG - Don't do this!
constructor(address owner) {
    _mint(owner, 1_000_000_000 * 10**18); // Mints to owner ❌
}
```

### ✅ CORRECT: Mint to Reserve Vault
```solidity
// CORRECT - Do this!
constructor(address owner, address reserveVault) {
    _mint(reserveVault, 1_000_000_000 * 10**18); // Mints to vault ✅
}
```

**Reserve Vault Address:**
- Multi-sig controlled
- Foundation/protocol controlled
- NOT an individual wallet
- Has release mechanisms for purchases

---

## Purchase Flow

### Step-by-Step Purchase Example

**User wants to buy 1,000 EDSC:**

```
1. User connects wallet
2. User selects: Buy 1,000 EDSC
3. User chooses payment: 1,000 USDC (or 0.3 ETH, or 0.02 BTC)
4. Smart contract checks: Reserve has ≥1,000 EDSC available
5. User sends 1,000 USDC to reserve vault
6. Reserve vault releases 1,000 EDSC to user
7. Transaction complete
   ├─ User holds: 1,000 EDSC
   └─ Reserve holds: 1,000 USDC (backing)
```

**Backing automatically created:**
- Reserve gained: $1,000 in USDC
- User gained: 1,000 EDSC
- Backing ratio: 100% (1:1)

---

## Redemption/Sell Flow

**User wants to sell 1,000 EDSC back:**

```
1. User selects: Sell 1,000 EDSC
2. User sends 1,000 EDSC to reserve vault
3. Reserve vault burns EDSC (removes from circulation)
4. Reserve vault sends equivalent crypto back (USDC/ETH/etc.)
5. Transaction complete
   ├─ User holds: 1,000 USDC (or equivalent)
   └─ Reserve burned: 1,000 EDSC
```

**Backing remains balanced:**
- Reserve releases crypto equal to EDSC burned
- Circulation decreases
- Ratio stays 100%

---

## No Pre-Funding Needed

### Why This Model is Better for Launch

**Traditional stablecoin launch:**
```
Need $1M-$150M upfront → Impossible for new projects
Must lock collateral → Can't launch without capital
High barrier to entry → Only big players can do it
```

**EDSC launch:**
```
Deploy contract → Free (just gas)
Mint to vault → Free (no capital needed)
Users provide backing → Organic growth
Backing grows with adoption → Sustainable
```

---

## Backing Composition

### What's in the Reserve?

**Multi-Asset Reserve:**
```
Users can buy EDSC with:
- USDC/USDT/DAI (stablecoins)
- ETH (Ethereum)
- BTC (Bitcoin)
- SOL (Solana)
- BNB (BSC)
- Other approved cryptos
```

**Reserve holds ALL of these:**
```
After 1000 transactions:
Reserve might hold:
- 30% USDC
- 25% ETH
- 20% BTC
- 15% SOL
- 10% Other

Total value: $1M
EDSC in circulation: 1M
Backing: 100% (diversified)
```

---

## Peg Maintenance ($1.00 Target)

### How EDSC Maintains $1.00 Peg

**1. Purchase/Redemption Arbitrage:**
```
If EDSC trades at $1.05 on DEX:
→ Arbitrageur buys from reserve at $1.00
→ Sells on DEX at $1.05
→ Profit: $0.05 per EDSC
→ Brings DEX price down to $1.00

If EDSC trades at $0.95 on DEX:
→ Arbitrageur buys on DEX at $0.95
→ Redeems with reserve at $1.00
→ Profit: $0.05 per EDSC
→ Brings DEX price up to $1.00
```

**2. Reserve Always Buys/Sells at $1.00:**
- Buy EDSC from reserve: Pay $1.00 in crypto
- Sell EDSC to reserve: Get $1.00 in crypto
- This creates natural price floor/ceiling

**3. No Interest Rate Adjustments Needed:**
- Traditional model: Adjust rates to control supply
- EDSC model: Reserve buys/sells maintain peg naturally
- Simpler mechanism, less management needed

---

## Reserve Vault Address

### Where EDSC is Stored Initially

**Reserve Vault Requirements:**
- Multi-signature wallet (6-of-9 Foundation multisig)
- Time-locked for security
- Transparent on-chain
- Auditable reserves

**Recommended Setup:**
```
Use Foundation Multisig from FOUNDATION_CHARTER.md:
- 6-of-9 signature requirement
- Board of Directors control
- Emergency pause capability
- Quarterly reserve audits
```

**For testnet/early deployment:**
```
Can use simple wallet address temporarily
Then upgrade to proper multisig before mainnet
```

---

## Deployment Configuration

### Updated Constructor Parameters

**Old (incorrect):**
```solidity
constructor(address owner) {
    _mint(owner, 100_000 * 10**18); // ❌ Mints to owner
}
```

**New (correct):**
```solidity
constructor(
    address owner,           // Admin/governance
    address reserveVault     // Where initial supply goes
) {
    _owner = owner;
    _mint(reserveVault, 1_000_000_000 * 10**18); // ✅ Mints to vault
}
```

---

## Initial Supply Distribution

### Total Supply: 1 Billion EDSC

**Day 1 Allocation:**
```
Reserve Vault: 1,000,000,000 EDSC (100%)
Circulating:   0 EDSC (0%)
Backing:       $0 (nothing circulating yet)
```

**NOT distributed to:**
- ❌ Team wallets
- ❌ Investors
- ❌ Public sale
- ❌ Airdrops
- ❌ Liquidity mining

**ALL tokens stay in vault until purchased.**

---

## Liquidity Pool Strategy

### Different from ÉTR Pools

**ÉTR Pools (Pre-funded):**
```
Deploy ÉTR
↓
Owner has initial supply
↓
Owner adds to liquidity pools
↓
Public trades on DEXes
```

**EDSC Pools (Reserve-funded):**
```
Deploy EDSC
↓
Vault has initial supply
↓
Users buy from reserve (not DEXes initially)
↓
LATER: Add liquidity to DEXes from reserve
```

**Recommended EDSC Launch:**
1. Deploy EDSC to vault (Day 1)
2. Enable reserve purchases (Week 1)
3. Build reserve value ($100K-$1M)
4. THEN add DEX liquidity from reserve
5. Public trades on both reserve + DEXes

---

## Smart Contract Architecture

### Required Functions

**Purchase EDSC:**
```solidity
function buyEDSC(
    address paymentToken,  // USDC, ETH, BTC, etc.
    uint256 paymentAmount, // Amount of payment token
    uint256 edscAmount     // Amount of EDSC to receive
) external {
    // Transfer payment token to reserve
    // Calculate $1.00 rate
    // Transfer EDSC from vault to buyer
}
```

**Redeem EDSC:**
```solidity
function sellEDSC(
    uint256 edscAmount,         // EDSC to sell
    address preferredPayment    // USDC/ETH/BTC/etc.
) external {
    // Burn EDSC from user
    // Calculate $1.00 rate
    // Send payment token from reserve
}
```

**Check Reserve:**
```solidity
function getReserveValue() external view returns (uint256) {
    // Calculate total value of all assets in reserve
    // Return in USD
}

function getBackingRatio() external view returns (uint256) {
    // Reserve value / EDSC in circulation
    // Should be ~100%
}
```

---

## Migration Path from Current Contracts

### Current Contracts (Incorrect)
```
✅ Deployment scripts work
✅ Chain configuration correct
❌ Mints to owner instead of vault
❌ No purchase/redemption functions
❌ Documentation has wrong backing model
```

### What Needs to Change

**1. Contract Constructor:**
```solidity
// Add reserveVault parameter
// Mint to vault instead of owner
```

**2. Add Purchase Functions:**
```solidity
// buyEDSC() function
// sellEDSC() function
// getReserveValue() function
```

**3. Update Deployment Scripts:**
```javascript
// Pass reserveVault address
// Don't send initial supply to owner
```

**4. Update Documentation:**
```
// Remove 150% collateralization
// Remove upfront backing requirement
// Explain organic backing model
```

---

## Treasury vs Algorithmic vs Collateralized

### Comparison of Stablecoin Models

**1. Collateralized (MakerDAO, DAI):**
```
Backing: 150%+ over-collateralization
Launch cost: $1M-$150M upfront
Peg mechanism: Liquidations + interest rates
Risk: Requires massive capital
```

**2. Algorithmic (Terra Luna - Failed):**
```
Backing: Nothing (algorithm only)
Launch cost: $0
Peg mechanism: Mint/burn algorithm
Risk: Death spiral (Terra Luna collapsed)
```

**3. Fiat-Backed (USDC, USDT):**
```
Backing: 1:1 USD in bank
Launch cost: Huge (banking relationships)
Peg mechanism: Redemption at $1.00
Risk: Centralization, regulations
```

**4. EDSC (Treasury-Backed):**
```
Backing: 100% from user purchases (multi-asset)
Launch cost: $0 (gas only)
Peg mechanism: Reserve buys/sells at $1.00
Risk: Low (backed by real assets, not algorithm)
```

---

## Summary of Corrections

### What Changed

**❌ Removed:**
- 150% collateralization requirement
- Upfront backing requirement
- Minting to owner wallet
- Pre-funded launch model
- Over-collateralization language

**✅ Added:**
- Mint to reserve vault
- Organic backing from purchases
- Purchase/redemption mechanisms
- Treasury-backed model
- 100% backing ratio (not 150%)

**Updated:**
- Ivory Papers need updating
- Smart contracts need vault parameter
- Deployment scripts need vault address
- Documentation reflects correct model

---

## Next Steps

1. **Update Smart Contracts:**
   - Add `reserveVault` parameter to constructor
   - Mint to vault instead of owner
   - Add purchase/redemption functions

2. **Set Reserve Vault Address:**
   - Use Foundation multisig: [to be determined]
   - Or temporary address for testnet

3. **Update Ivory Papers:**
   - Remove 150% collateralization language
   - Explain treasury-backed model
   - Document purchase/redemption flow

4. **Deploy with Correct Architecture:**
   - All EDSC mints to vault
   - No distribution to team/investors
   - Organic backing from day 1

---

**This is the CORRECT EDSC tokenomics model.** The previous documentation was wrong about backing requirements.
