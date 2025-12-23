# ĒTRID Bridge Flow Charts
**Comprehensive Visual Token Flow Architecture**

**Created:** December 8, 2025
**Purpose:** Visual representation of token flows through ĒTRID's bridge ecosystem

---

## TABLE OF CONTENTS

1. [Flow Chart 1: External Currency → ĒTRID Liquidity Pool](#flow-chart-1-external-currency--ētrid-liquidity-pool)
2. [Flow Chart 2: User Swaps wBTC for ÉTR](#flow-chart-2-user-swaps-wbtc-for-étr)
3. [Flow Chart 3: User Bridges ÉTR to External Chain](#flow-chart-3-user-bridges-étr-to-external-chain)
4. [Flow Chart 4: Reverse Flow - External Currency Back Out](#flow-chart-4-reverse-flow---external-currency-back-out)
5. [Complete System Overview](#complete-system-overview)
6. [Actor Reference Guide](#actor-reference-guide)

---

## FLOW CHART 1: External Currency → ĒTRID Liquidity Pool

**Scenario:** User has BTC on Bitcoin blockchain and wants to add it to PrimeSwap one-sided liquidity pool

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                   EXTERNAL BTC → ĒTRID wBTC LIQUIDITY POOL                      │
│                         (One-Sided Bootstrap Mechanism)                         │
└─────────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 1: USER INITIATES BRIDGE (Bitcoin Network)                              │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER WALLET]                                                               │
│   Balance: 1.5 BTC                                                            │
│        │                                                                       │
│        │ (1) User sends 1.0 BTC to bridge address                            │
│        ↓                                                                       │
│   ┌─────────────────────────┐                                                 │
│   │  Bitcoin Bridge Address │  ← Multi-sig address (3-of-5 custodians)       │
│   │  bc1q...bridge...xyz     │                                                │
│   └─────────────────────────┘                                                 │
│        │                                                                       │
│        │ (2) Bitcoin transaction confirmed (6 blocks ≈ 60 minutes)           │
│        │                                                                       │
│        ↓                                                                       │
│   [BITCOIN BLOCKCHAIN]                                                         │
│   Transaction ID: 0xabc123...                                                 │
│   Confirmations: 6/6 ✓                                                        │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (3) Event detected by monitoring service
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 2: BRIDGE RELAYERS DETECT & ATTEST (BTC-PBC Collator)                   │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐    │
│   │  RELAYER 1      │      │  RELAYER 2      │      │  RELAYER 3      │    │
│   │  SPV Monitor    │      │  SPV Monitor    │      │  SPV Monitor    │    │
│   └────────┬────────┘      └────────┬────────┘      └────────┬────────┘    │
│            │                         │                         │             │
│            │ (4) All detect BTC deposit via SPV proofs         │             │
│            │                         │                         │             │
│            └─────────────────────────┼─────────────────────────┘             │
│                                      ↓                                        │
│              ┌────────────────────────────────────────┐                       │
│              │   BTC-PBC Collator Node                │                       │
│              │   (Partition Burst Chain)              │                       │
│              ├────────────────────────────────────────┤                       │
│              │  • SPV proof verification              │                       │
│              │  • Bitcoin light client validation     │                       │
│              │  • 6-block confirmation check          │                       │
│              └────────────┬───────────────────────────┘                       │
│                           │                                                   │
│                           │ (5) Deposit confirmed                             │
│                           ↓                                                   │
│              ┌────────────────────────────────────────┐                       │
│              │  pallet-bitcoin-bridge                 │                       │
│              │  confirm_btc_deposit() extrinsic       │                       │
│              ├────────────────────────────────────────┤                       │
│              │  Params:                               │                       │
│              │  • tx_id: 0xabc123...                  │                       │
│              │  • amount: 1.0 BTC                     │                       │
│              │  • recipient: 0xUser_ĒTRID_Address     │                       │
│              │  • spv_proof: [merkle_proof]           │                       │
│              └────────────┬───────────────────────────┘                       │
│                           │                                                   │
│                           │ (6) Extrinsic executed                            │
│                           ↓                                                   │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (7) Cross-chain message to Primearc
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 3: ATTESTATION & MINTING (Primearc Core Chain)                          │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────┐               │
│   │  XCMP/HRMP Message from BTC-PBC                          │               │
│   │  (Cross-Consensus Message Passing)                       │               │
│   ├──────────────────────────────────────────────────────────┤               │
│   │  Message Type: BridgeDeposit                             │               │
│   │  Source: BTC-PBC                                         │               │
│   │  Payload: {                                              │               │
│   │    asset: "BTC",                                         │               │
│   │    amount: "1.0",                                        │               │
│   │    recipient: "0xUser_ĒTRID_Address",                   │               │
│   │    attestations: [sig1, sig2, sig3]  ← M-of-N (3-of-5) │               │
│   │  }                                                       │               │
│   └─────────────────────────┬────────────────────────────────┘               │
│                             │                                                 │
│                             │ (8) Message received                            │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────┐               │
│   │  pallet-bridge-attestation                               │               │
│   │  verify_attestation()                                    │               │
│   ├──────────────────────────────────────────────────────────┤               │
│   │  • Checks 3-of-5 attester signatures                     │               │
│   │  • Verifies ECDSA signatures cryptographically           │               │
│   │  • Ensures attesters are active                          │               │
│   │  • Validates message hash integrity                      │               │
│   │                                                           │               │
│   │  Result: ✓ ATTESTATION VALID                             │               │
│   └─────────────────────────┬────────────────────────────────┘               │
│                             │                                                 │
│                             │ (9) Attestation approved                        │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────┐               │
│   │  pallet-token-messenger                                  │               │
│   │  receive_and_mint()                                      │               │
│   ├──────────────────────────────────────────────────────────┤               │
│   │  • Mints 1.0 wBTC on Primearc Core Chain                │               │
│   │  • Credits user account: 0xUser_ĒTRID_Address            │               │
│   │  • Emits TokenMinted event                               │               │
│   │  • Updates bridge nonce (replay protection)              │               │
│   │                                                           │               │
│   │  [USER BALANCE]                                          │               │
│   │  wBTC: 0 → 1.0 wBTC ✓                                    │               │
│   └──────────────────────────────────────────────────────────┘               │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (10) User now has wBTC on ĒTRID
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 4: LIQUIDITY ADDED TO PRIMESWAP (Primearc Core Chain)                   │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   Note: PrimeSwap pools use ONE-SIDED BOOTSTRAP                               │
│   Foundation already seeded 845,750,000 ÉTR                                   │
│   Virtual wBTC reserve: 33.83 BTC (oracle-calculated)                        │
│                                                                               │
│   ┌──────────────────────────────────────────────────────────┐               │
│   │  PrimeSwap: ÉTR/wBTC Pool (VirtualReserveAMM)           │               │
│   ├──────────────────────────────────────────────────────────┤               │
│   │  BEFORE User's First Swap:                               │               │
│   │  ┌────────────────────────────────────────────────┐      │               │
│   │  │  Real ÉTR Reserve:      845,750,000 ÉTR       │      │               │
│   │  │  Real wBTC Reserve:     0 wBTC                 │      │               │
│   │  │  Virtual wBTC Reserve:  33.83 wBTC             │      │               │
│   │  │  Total wBTC for AMM:    33.83 wBTC             │      │               │
│   │  │                                                 │      │               │
│   │  │  k = 845,750,000 × 33.83 = 28,615,762,500     │      │               │
│   │  └────────────────────────────────────────────────┘      │               │
│   │                                                           │               │
│   │  User swaps 1.0 wBTC → ÉTR (see Flow Chart 2)           │               │
│   │                                                           │               │
│   │  AFTER User's Swap:                                      │               │
│   │  ┌────────────────────────────────────────────────┐      │               │
│   │  │  Real ÉTR Reserve:      821,250,000 ÉTR       │      │               │
│   │  │  Real wBTC Reserve:     1.0 wBTC               │      │               │
│   │  │  Virtual wBTC Reserve:  33.83 wBTC             │      │               │
│   │  │  Total wBTC for AMM:    34.83 wBTC             │      │               │
│   │  │                                                 │      │               │
│   │  │  Pool now has REAL BTC liquidity!              │      │               │
│   │  └────────────────────────────────────────────────┘      │               │
│   │                                                           │               │
│   │  KEY POINT: wBTC doesn't "enter" pool in traditional     │               │
│   │  sense. It's swapped immediately. Pool holds wBTC,       │               │
│   │  user receives ÉTR.                                      │               │
│   └──────────────────────────────────────────────────────────┘               │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ ROLE SUMMARY                                                                  │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  • USER: Deposits BTC, receives wBTC, swaps for ÉTR                          │
│  • RELAYERS: Monitor Bitcoin, submit SPV proofs to BTC-PBC                   │
│  • BTC-PBC COLLATORS: Validate deposits, secure the PBC                      │
│  • ATTESTERS (3-of-5): Sign cross-chain messages for security                │
│  • PRIMEARC VALIDATORS: Verify attestations, mint wBTC                       │
│  • PRIMESWAP POOL: Provides immediate liquidity (one-sided)                  │
│                                                                               │
│  TIME: ~70 minutes (6 BTC confirmations + ĒTRID finality)                    │
│  COST: Bitcoin tx fee (~$2-5) + ĒTRID tx fee (~$0.001)                       │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## FLOW CHART 2: User Swaps wBTC for ÉTR

**Scenario:** User holds 1.0 wBTC on ĒTRID and wants to swap for ÉTR through PrimeSwap one-sided pool

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         USER SWAPS wBTC → ÉTR ON PRIMESWAP                      │
│                      (VirtualReserveAMM One-Sided Pool)                         │
└─────────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ BEFORE SWAP: Pool State                                                      │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  PrimeSwap Pool: ÉTR/wBTC                                    │           │
│   │  Type: VirtualReserveAMM                                     │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Real ÉTR Reserve:       845,750,000 ÉTR                    │           │
│   │  Real wBTC Reserve:      0 wBTC                              │           │
│   │  Virtual wBTC Reserve:   33.83 wBTC (oracle-derived)        │           │
│   │  ───────────────────────────────────────────────────────     │           │
│   │  Total wBTC for AMM:     33.83 wBTC                          │           │
│   │                                                               │           │
│   │  Oracle Prices:                                              │           │
│   │  • 1 BTC = $25,000                                           │           │
│   │  • 1 ÉTR = $0.004                                            │           │
│   │  • Implied rate: 1 BTC = 6,250,000 ÉTR                      │           │
│   │                                                               │           │
│   │  k = 845,750,000 × 33.83 = 28,615,762,500                   │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 1: USER INITIATES SWAP                                                  │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER WALLET]                                                               │
│   wBTC Balance: 1.0 wBTC                                                      │
│   ÉTR Balance:  0 ÉTR                                                         │
│        │                                                                       │
│        │ (1) User calls PrimeSwapRouter.swapExactTokensForTokens()           │
│        │                                                                       │
│        ↓                                                                       │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  PrimeSwapRouter Contract                                    │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Function: swapExactTokensForTokens()                        │           │
│   │  Params:                                                     │           │
│   │    amountIn:  1.0 wBTC (1e18 wei)                           │           │
│   │    amountOutMin: 24,000,000 ÉTR (slippage: 4%)              │           │
│   │    path: [wBTC_ADDRESS, ETR_ADDRESS]                        │           │
│   │    to: 0xUser_ĒTRID_Address                                 │           │
│   │    deadline: block.timestamp + 600                          │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (2) Router queries pool                         │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 2: AMM CALCULATION (VirtualReserveAMM Logic)                            │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  PrimeSwapPair: ÉTR/wBTC Pool Contract                       │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (3) Calculate output amount                                 │           │
│   │                                                               │           │
│   │  Current State:                                              │           │
│   │    reserve_etr = 845,750,000                                 │           │
│   │    reserve_wbtc_real = 0                                     │           │
│   │    reserve_wbtc_virtual = 33.83                              │           │
│   │    reserve_wbtc_total = 33.83                                │           │
│   │                                                               │           │
│   │  User Input:                                                 │           │
│   │    amount_wbtc_in = 1.0                                      │           │
│   │                                                               │           │
│   │  AMM Formula: x × y = k                                      │           │
│   │    k = 845,750,000 × 33.83 = 28,615,762,500                 │           │
│   │                                                               │           │
│   │  New wBTC total:                                             │           │
│   │    reserve_wbtc_new = 33.83 + 1.0 = 34.83                   │           │
│   │                                                               │           │
│   │  New ÉTR reserve:                                            │           │
│   │    reserve_etr_new = k / reserve_wbtc_new                   │           │
│   │    reserve_etr_new = 28,615,762,500 / 34.83                 │           │
│   │    reserve_etr_new = 821,639,011 ÉTR                        │           │
│   │                                                               │           │
│   │  ÉTR output (before fees):                                   │           │
│   │    amount_etr_out = 845,750,000 - 821,639,011               │           │
│   │    amount_etr_out = 24,110,989 ÉTR                          │           │
│   │                                                               │           │
│   │  Trading Fee (0.3%):                                         │           │
│   │    fee = 24,110,989 × 0.003 = 72,333 ÉTR                    │           │
│   │    fee_destination = Treasury pallet                         │           │
│   │                                                               │           │
│   │  Final ÉTR to user:                                          │           │
│   │    amount_etr_final = 24,110,989 - 72,333                   │           │
│   │    amount_etr_final = 24,038,656 ÉTR                        │           │
│   │                                                               │           │
│   │  Slippage Check:                                             │           │
│   │    24,038,656 > 24,000,000 (amountOutMin) ✓ PASS            │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (4) Execute swap
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 3: TOKEN TRANSFER EXECUTION                                             │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  (5) Transfer wBTC from user to pool                         │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  wBTC.transfer(                                              │           │
│   │    from: 0xUser_ĒTRID_Address,                              │           │
│   │    to: POOL_ADDRESS,                                         │           │
│   │    amount: 1.0 wBTC                                          │           │
│   │  )                                                           │           │
│   │                                                               │           │
│   │  [USER wBTC BALANCE]                                         │           │
│   │  Before: 1.0 wBTC                                            │           │
│   │  After:  0 wBTC                                              │           │
│   │                                                               │           │
│   │  [POOL wBTC BALANCE]                                         │           │
│   │  Before: 0 wBTC (real)                                       │           │
│   │  After:  1.0 wBTC (real)                                     │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (6) Transfer ÉTR from pool to user              │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  ÉTR.transfer(                                               │           │
│   │    from: POOL_ADDRESS,                                       │           │
│   │    to: 0xUser_ĒTRID_Address,                                │           │
│   │    amount: 24,038,656 ÉTR                                    │           │
│   │  )                                                           │           │
│   │                                                               │           │
│   │  [POOL ÉTR BALANCE]                                          │           │
│   │  Before: 845,750,000 ÉTR                                     │           │
│   │  After:  821,639,011 ÉTR (minus fees = 821,711,344)         │           │
│   │                                                               │           │
│   │  [USER ÉTR BALANCE]                                          │           │
│   │  Before: 0 ÉTR                                               │           │
│   │  After:  24,038,656 ÉTR ✓                                    │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (7) Transfer fees to Treasury                   │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  ÉTR.transfer(                                               │           │
│   │    from: POOL_ADDRESS,                                       │           │
│   │    to: TREASURY_PALLET_ADDRESS,                              │           │
│   │    amount: 72,333 ÉTR                                        │           │
│   │  )                                                           │           │
│   │                                                               │           │
│   │  [TREASURY BALANCE]                                          │           │
│   │  Fees accumulated: +72,333 ÉTR                               │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (8) Emit events
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 4: EVENTS & POOL STATE UPDATE                                           │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Events Emitted:                                             │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  1. Swap(                                                    │           │
│   │       sender: 0xUser_ĒTRID_Address,                         │           │
│   │       amount0In: 0,                                          │           │
│   │       amount1In: 1.0 wBTC,                                   │           │
│   │       amount0Out: 24,038,656 ÉTR,                           │           │
│   │       amount1Out: 0,                                         │           │
│   │       to: 0xUser_ĒTRID_Address                              │           │
│   │     )                                                        │           │
│   │                                                               │           │
│   │  2. Sync(                                                    │           │
│   │       reserve0: 821,711,344 ÉTR,                            │           │
│   │       reserve1: 1.0 wBTC                                     │           │
│   │     )                                                        │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ AFTER SWAP: Pool State                                                       │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  PrimeSwap Pool: ÉTR/wBTC                                    │           │
│   │  Type: VirtualReserveAMM                                     │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Real ÉTR Reserve:       821,711,344 ÉTR (↓ from 845.75M)  │           │
│   │  Real wBTC Reserve:      1.0 wBTC (↑ from 0)                │           │
│   │  Virtual wBTC Reserve:   33.83 wBTC (unchanged)              │           │
│   │  ───────────────────────────────────────────────────────     │           │
│   │  Total wBTC for AMM:     34.83 wBTC                          │           │
│   │                                                               │           │
│   │  Pool now has REAL wBTC liquidity!                           │           │
│   │  Virtual reserve phases out as real liquidity grows          │           │
│   │                                                               │           │
│   │  k = 821,711,344 × 34.83 = 28,620,226,001                   │           │
│   │  (k increased slightly due to fees staying in pool)          │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ WHAT HAPPENS TO wBTC?                                                        │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  ✓ wBTC STAYS IN POOL (Not burned)                                           │
│                                                                               │
│  • The wBTC is held by the pool contract as real liquidity                   │
│  • It increases the real wBTC reserve (0 → 1.0)                              │
│  • Virtual wBTC remains at 33.83 (unchanged for now)                         │
│  • Future swaps will use Total wBTC = Real + Virtual = 34.83                 │
│  • As real wBTC grows, virtual wBTC phases out gradually                     │
│  • When real wBTC > threshold (e.g., 34 BTC), virtual drops to 0             │
│  • Pool becomes pure AMM (x × y = k) with market pricing                     │
│                                                                               │
│  WHY NOT BURN?                                                                │
│  • wBTC is a bridged asset (backed 1:1 by real BTC locked on Bitcoin)        │
│  • Burning would break the 1:1 peg                                           │
│  • Pool holds wBTC to provide liquidity for reverse swaps (ÉTR → wBTC)       │
│  • Users can later swap ÉTR back to wBTC and bridge to Bitcoin               │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ USER SUMMARY                                                                  │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  BEFORE SWAP:                          AFTER SWAP:                            │
│  • wBTC: 1.0                           • wBTC: 0                              │
│  • ÉTR:  0                             • ÉTR:  24,038,656                     │
│                                                                               │
│  TRANSACTION COST: ~$0.001 (ĒTRID gas fee)                                   │
│  TRADING FEE: 72,333 ÉTR (0.3%)                                              │
│  TIME: ~12 seconds (2 ĒTRID blocks)                                          │
│  SLIPPAGE: ~3.8% (due to large trade size vs pool depth)                     │
│                                                                               │
│  EFFECTIVE RATE: 1 BTC = 24,038,656 ÉTR ($0.00104 per ÉTR)                  │
│  ORACLE RATE:    1 BTC = 25,000,000 ÉTR ($0.001 per ÉTR)                    │
│  PRICE IMPACT: +4% premium due to AMM slippage                                │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## FLOW CHART 3: User Bridges ÉTR to External Chain

**Scenario:** User has ÉTR on Primearc Core Chain, wants to bridge to Base L2, receives wÉTR (ERC-20), can trade on Uniswap

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                   ÉTR (Primearc) → wÉTR (Base L2) BRIDGE                       │
│                   Via Bridge Adapter & LayerZero (Future)                       │
└─────────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 1: USER INITIATES BRIDGE (Primearc Core Chain)                          │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER WALLET - PRIMEARC]                                                    │
│   ÉTR Balance: 24,038,656 ÉTR (from previous swap)                           │
│        │                                                                       │
│        │ (1) User wants to bridge 10,000,000 ÉTR to Base L2                  │
│        │     (To trade on Uniswap or use in Base DeFi)                       │
│        │                                                                       │
│        ↓                                                                       │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  pallet-token-messenger (Primearc Core Chain)                │           │
│   │  Extrinsic: burn_and_send()                                  │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Params:                                                     │           │
│   │    amount: 10,000,000 ÉTR                                    │           │
│   │    destination_chain: BASE_L2_CHAIN_ID (8453)               │           │
│   │    recipient: 0xUser_Base_Address                           │           │
│   │    adapter_type: EVM_BRIDGE_ADAPTER                         │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (2) Burn ÉTR on Primearc                        │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Balances Pallet (Native ÉTR)                                │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Operation: burn_from(user, 10_000_000 ÉTR)                 │           │
│   │                                                               │           │
│   │  [USER BALANCE]                                              │           │
│   │  Before: 24,038,656 ÉTR                                      │           │
│   │  After:  14,038,656 ÉTR                                      │           │
│   │                                                               │           │
│   │  [TOTAL ÉTR SUPPLY]                                          │           │
│   │  Before: 10,000,000,000 ÉTR                                  │           │
│   │  After:  9,990,000,000 ÉTR                                   │           │
│   │                                                               │           │
│   │  ✓ ÉTR BURNED (removed from circulation)                     │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (3) Create cross-chain message                  │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 2: ATTESTATION & MESSAGE CREATION (Primearc Core Chain)                 │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  pallet-bridge-attestation                                   │           │
│   │  create_attestation_request()                                │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Message Hash Calculation:                                   │           │
│   │    hash = keccak256(                                         │           │
│   │      message_type: "BurnAndMint",                            │           │
│   │      source_chain: PRIMEARC_CHAIN_ID,                       │           │
│   │      dest_chain: BASE_L2_CHAIN_ID (8453),                   │           │
│   │      amount: 10,000,000 ÉTR,                                 │           │
│   │      recipient: 0xUser_Base_Address,                        │           │
│   │      nonce: 12345                                            │           │
│   │    )                                                         │           │
│   │    hash = 0xabc123def456...                                  │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (4) Attesters sign message                      │
│                             ↓                                                 │
│   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│   │  ATTESTER 1     │  │  ATTESTER 2     │  │  ATTESTER 3     │             │
│   │  (Active)       │  │  (Active)       │  │  (Active)       │             │
│   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│            │                     │                     │                      │
│            │ (5) All sign with ECDSA private keys     │                      │
│            │        (Off-chain signing service)        │                      │
│            │                     │                     │                      │
│            └─────────────────────┼─────────────────────┘                      │
│                                  ↓                                            │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Attestation Bundle (M-of-N: 3-of-5)                         │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  message_hash: 0xabc123def456...                             │           │
│   │  signatures: [                                               │           │
│   │    sig1: 0x1234...(65 bytes, ECDSA),                        │           │
│   │    sig2: 0x5678...(65 bytes, ECDSA),                        │           │
│   │    sig3: 0x9abc...(65 bytes, ECDSA)                         │           │
│   │  ]                                                           │           │
│   │  attester_ids: [attester1, attester2, attester3]            │           │
│   │  threshold_met: true (3-of-5) ✓                              │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (6) Relayer picks up attested message           │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 3: RELAYER SUBMITS TO BASE L2 (Off-Chain → Base L2)                     │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bridge Relayer Service (TypeScript/Node.js)                 │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (7) Relayer monitors Primearc for BurnAndSend events        │           │
│   │  (8) Detects fully-attested message (3-of-5 signatures)      │           │
│   │  (9) Constructs Ethereum transaction for Base L2             │           │
│   │                                                               │           │
│   │  Transaction:                                                │           │
│   │    to: BASE_MESSAGE_TRANSMITTER_CONTRACT                     │           │
│   │    function: receiveMessage(message, attestation)            │           │
│   │    gas: 150,000                                              │           │
│   │    gasPrice: 0.02 gwei (Base L2 is cheap)                   │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (10) Submit transaction to Base L2              │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Base L2 Blockchain (Ethereum L2)                            │           │
│   │  Transaction Hash: 0xdef456abc789...                         │           │
│   │  Block: 15,234,567                                           │           │
│   │  Confirmations: 1/1 (instant finality on L2) ✓               │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (11) Contract receives message                  │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 4: MESSAGE VERIFICATION & MINTING (Base L2)                             │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  BaseMessageTransmitter.sol                                  │           │
│   │  Function: receiveMessage()                                  │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (12) Verify attestation signatures                          │           │
│   │                                                               │           │
│   │  Signature Verification:                                     │           │
│   │    for each signature in attestation:                        │           │
│   │      recovered = ecrecover(                                  │           │
│   │        message_hash,                                         │           │
│   │        v, r, s  // ECDSA signature components                │           │
│   │      )                                                       │           │
│   │      require(recovered == attester_address, "Bad sig")      │           │
│   │      require(isActiveAttester(recovered), "Inactive")       │           │
│   │                                                               │           │
│   │  Threshold Check:                                            │           │
│   │    valid_sigs = 3                                            │           │
│   │    threshold = 3                                             │           │
│   │    require(valid_sigs >= threshold, "Threshold not met")    │           │
│   │                                                               │           │
│   │  Nonce Check (Replay Protection):                            │           │
│   │    require(!usedNonces[nonce], "Nonce already used")        │           │
│   │    usedNonces[nonce] = true                                  │           │
│   │                                                               │           │
│   │  ✓ ALL CHECKS PASSED                                         │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (13) Call token minter                          │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  WrappedETR.sol (ERC-20 on Base L2)                          │           │
│   │  Function: mint()                                            │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (14) Mint wÉTR to user on Base L2                           │           │
│   │                                                               │           │
│   │  Operation:                                                  │           │
│   │    _mint(                                                    │           │
│   │      to: 0xUser_Base_Address,                               │           │
│   │      amount: 10,000,000 * 1e18  // 10M wÉTR                 │           │
│   │    )                                                         │           │
│   │                                                               │           │
│   │  [USER BALANCE ON BASE L2]                                   │           │
│   │  Before: 0 wÉTR                                              │           │
│   │  After:  10,000,000 wÉTR ✓                                   │           │
│   │                                                               │           │
│   │  [TOTAL wÉTR SUPPLY ON BASE]                                 │           │
│   │  Before: 50,000,000 wÉTR                                     │           │
│   │  After:  60,000,000 wÉTR                                     │           │
│   │                                                               │           │
│   │  Event Emitted:                                              │           │
│   │    Transfer(address(0), 0xUser_Base_Address, 10_000_000)    │           │
│   │    BridgeMintCompleted(nonce, recipient, amount)            │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (15) User now has wÉTR on Base L2
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 5: USER TRADES ON UNISWAP (Base L2)                                     │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER WALLET - BASE L2]                                                     │
│   wÉTR Balance: 10,000,000 wÉTR                                              │
│   ETH Balance:  0.5 ETH                                                       │
│        │                                                                       │
│        │ (16) User wants to provide liquidity or trade on Uniswap            │
│        │                                                                       │
│        ↓                                                                       │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Uniswap V3 (Base L2)                                        │           │
│   │  Pool: wÉTR/ETH (0.3% fee tier)                              │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Option 1: Provide Liquidity                                 │           │
│   │  ─────────────────────────────                               │           │
│   │  • Add 5,000,000 wÉTR + 0.1 ETH                             │           │
│   │  • Receive LP NFT (Uniswap V3 position)                     │           │
│   │  • Earn trading fees (0.3% per swap)                        │           │
│   │  • Can manage position on Base L2 UI                        │           │
│   │                                                               │           │
│   │  Option 2: Swap wÉTR for ETH                                 │           │
│   │  ──────────────────────────────                              │           │
│   │  • Swap 1,000,000 wÉTR → 0.02 ETH                           │           │
│   │  • Instant execution (L2 speed)                              │           │
│   │  • Low gas cost (~$0.05)                                     │           │
│   │                                                               │           │
│   │  Option 3: Swap wÉTR for USDC                                │           │
│   │  ───────────────────────────────                             │           │
│   │  • Route: wÉTR → ETH → USDC                                 │           │
│   │  • Final balance: 4,000 USDC                                │           │
│   │  • Can off-ramp to fiat via Coinbase                        │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
│   KEY POINT: User never needs to touch Primearc directly!                     │
│   All trading happens natively on Base L2 (Uniswap interface)                │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ BRIDGE ADAPTER ARCHITECTURE                                                  │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   Location: /05-multichain/bridges/adapters/base/                            │
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bridge Adapter Components:                                  │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  1. deploy-tokens.ts                                         │           │
│   │     • Deploys WrappedETR.sol on Base L2                     │           │
│   │     • Sets up minter role (MessageTransmitter)              │           │
│   │     • Configures bridge authority (multi-sig)               │           │
│   │                                                               │           │
│   │  2. bridge.ts                                                │           │
│   │     • Monitors Primearc for burn events                     │           │
│   │     • Collects attestations from 3-of-5 attesters          │           │
│   │     • Submits to Base L2 MessageTransmitter                 │           │
│   │     • Monitors Base for burn events (reverse flow)          │           │
│   │                                                               │           │
│   │  3. WrappedToken.sol (ERC-20)                               │           │
│   │     • Standard ERC-20 with mint/burn                        │           │
│   │     • Only MessageTransmitter can mint                      │           │
│   │     • Anyone can burn (for bridging back)                   │           │
│   │                                                               │           │
│   │  4. BaseMessageTransmitter.sol                              │           │
│   │     • Receives attested messages                            │           │
│   │     • Verifies M-of-N signatures (3-of-5)                   │           │
│   │     • Calls WrappedETR.mint()                               │           │
│   │     • Nonce management (replay protection)                  │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
│   Status: ✅ Ready for deployment                                            │
│   Estimated Deployment Cost: $50-100 (Base L2 gas)                           │
│   Deployment Time: 1-2 hours                                                  │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ SUMMARY                                                                       │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  PRIMEARC CORE CHAIN:                    BASE L2:                             │
│  • ÉTR burned: 10,000,000               • wÉTR minted: 10,000,000            │
│  • Total supply decreased               • Total wÉTR supply increased         │
│  • User balance: 14,038,656 ÉTR         • User balance: 10,000,000 wÉTR     │
│                                                                               │
│  TRANSACTION COSTS:                                                           │
│  • Primearc burn: ~$0.001 (native gas)                                       │
│  • Base L2 mint:  ~$0.10 (relayer pays, user may pay optional tip)          │
│  • Bridge fee:    ~$1 (paid to relayers/attesters)                           │
│  • Total cost:    ~$1.11                                                      │
│                                                                               │
│  TIME: 5-10 minutes (attestation collection + L2 finality)                    │
│                                                                               │
│  SECURITY: 3-of-5 multi-sig attestation + cryptographic verification         │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## FLOW CHART 4: Reverse Flow - External Currency Back Out

**Scenario:** User burns wBTC on ĒTRID, bridge attesters verify, bridge relay unlocks BTC on Bitcoin, user receives native BTC

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    REVERSE BRIDGE: wBTC (ĒTRID) → BTC (Bitcoin)                │
│                         User Exits to Native Bitcoin                           │
└─────────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 1: USER INITIATES WITHDRAWAL (Primearc Core Chain)                      │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER WALLET - PRIMEARC]                                                    │
│   wBTC Balance: 5.0 wBTC (acquired from swaps or bridges)                    │
│   ÉTR Balance:  1,000,000 ÉTR                                                │
│        │                                                                       │
│        │ (1) User wants to withdraw 2.0 wBTC to native Bitcoin               │
│        │     (To custody in cold wallet or sell on Bitcoin DEX)              │
│        │                                                                       │
│        ↓                                                                       │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  pallet-token-messenger (Primearc Core Chain)                │           │
│   │  Extrinsic: burn_for_withdrawal()                            │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Params:                                                     │           │
│   │    asset: "wBTC"                                             │           │
│   │    amount: 2.0 wBTC                                          │           │
│   │    destination_chain: BITCOIN_CHAIN_ID                      │           │
│   │    recipient_btc_address: "bc1q...user...xyz"               │           │
│   │                                                               │           │
│   │  Note: User provides Bitcoin address (P2WPKH or P2WSH)      │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (2) Burn wBTC on Primearc                       │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  pallet-wbtc-token (Wrapped BTC Pallet)                      │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Operation: burn_from(user, 2.0 wBTC)                       │           │
│   │                                                               │           │
│   │  [USER wBTC BALANCE]                                         │           │
│   │  Before: 5.0 wBTC                                            │           │
│   │  After:  3.0 wBTC                                            │           │
│   │                                                               │           │
│   │  [TOTAL wBTC SUPPLY ON PRIMEARC]                             │           │
│   │  Before: 100.0 wBTC                                          │           │
│   │  After:  98.0 wBTC                                           │           │
│   │                                                               │           │
│   │  ✓ wBTC BURNED (removed from ĒTRID circulation)              │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (3) Create withdrawal message                   │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 2: ATTESTATION REQUEST (Primearc Core Chain)                            │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  pallet-bridge-attestation                                   │           │
│   │  create_withdrawal_attestation()                             │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (4) Create withdrawal message                               │           │
│   │                                                               │           │
│   │  Withdrawal Message:                                         │           │
│   │    {                                                         │           │
│   │      message_type: "BurnAndUnlock",                          │           │
│   │      asset: "BTC",                                           │           │
│   │      amount: 2.0 BTC (200,000,000 satoshis),                │           │
│   │      source_chain: PRIMEARC_CHAIN_ID,                       │           │
│   │      dest_chain: BITCOIN_CHAIN_ID,                          │           │
│   │      recipient: "bc1q...user...xyz",                        │           │
│   │      nonce: 67890,                                           │           │
│   │      timestamp: 1733678400                                   │           │
│   │    }                                                         │           │
│   │                                                               │           │
│   │  Message Hash:                                               │           │
│   │    hash = keccak256(message_bytes)                           │           │
│   │    hash = 0xdef789ghi012...                                  │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (5) Broadcast to attesters                      │
│                             ↓                                                 │
│   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│   │  ATTESTER 1     │  │  ATTESTER 2     │  │  ATTESTER 3     │             │
│   │  (Custodian)    │  │  (Custodian)    │  │  (Custodian)    │             │
│   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│            │                     │                     │                      │
│            │ (6) Each attester verifies:              │                      │
│            │     • Burn event occurred on Primearc    │                      │
│            │     • Amount matches (2.0 BTC)           │                      │
│            │     • Recipient address valid            │                      │
│            │     • No replay (nonce not used)         │                      │
│            │                     │                     │                      │
│            │ (7) Sign with ECDSA private key          │                      │
│            │                     │                     │                      │
│            └─────────────────────┼─────────────────────┘                      │
│                                  ↓                                            │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Attestation Bundle (3-of-5 Multi-Sig)                       │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  message_hash: 0xdef789ghi012...                             │           │
│   │  signatures: [                                               │           │
│   │    sig1: 0xabcd...(65 bytes),                               │           │
│   │    sig2: 0xefgh...(65 bytes),                               │           │
│   │    sig3: 0xijkl...(65 bytes)                                │           │
│   │  ]                                                           │           │
│   │  threshold: 3-of-5 ✓ MET                                     │           │
│   │  timestamp: 1733678410 (10 seconds after request)            │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (8) Attestation complete                        │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 3: BITCOIN BRIDGE CUSTODIANS PROCESS WITHDRAWAL                         │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bitcoin Bridge Multi-Sig Wallet (3-of-5 Custodians)        │           │
│   │  Address: bc1q...bridge...custodians...xyz                  │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Locked BTC:                                                 │           │
│   │  • Total: 150.0 BTC                                          │           │
│   │  • Available: 140.0 BTC                                      │           │
│   │  • Pending withdrawals: 10.0 BTC                             │           │
│   │                                                               │           │
│   │  (9) Custodians receive attested withdrawal request          │           │
│   │      via off-chain relayer service                           │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (10) Verify attestation                         │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bitcoin Bridge Relay Service (Off-Chain)                   │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Verification Steps:                                         │           │
│   │  1. Check 3-of-5 signatures valid                            │           │
│   │  2. Verify message hash matches withdrawal request           │           │
│   │  3. Confirm burn event on Primearc (SPV proof)               │           │
│   │  4. Check nonce not used (prevent replay)                    │           │
│   │  5. Validate recipient Bitcoin address format                │           │
│   │  6. Check sufficient BTC in custody wallet                   │           │
│   │                                                               │           │
│   │  ✓ ALL CHECKS PASSED                                         │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (11) Construct Bitcoin transaction              │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bitcoin Transaction Construction                            │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Input:                                                      │           │
│   │    • UTXO from custody wallet: 5.0 BTC                      │           │
│   │    • Previous tx: 0x123abc...                               │           │
│   │    • Index: 0                                                │           │
│   │                                                               │           │
│   │  Outputs:                                                    │           │
│   │    1. To user: bc1q...user...xyz                            │           │
│   │       Amount: 2.0 BTC (200,000,000 satoshis)                │           │
│   │                                                               │           │
│   │    2. Change back to custody wallet                          │           │
│   │       Amount: 2.9995 BTC (299,950,000 satoshis)             │           │
│   │                                                               │           │
│   │  Fee: 0.0005 BTC (50,000 satoshis ≈ $12.50)                │           │
│   │  (High priority for 1-2 block confirmation)                 │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (12) Multi-sig signing ceremony                 │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 4: MULTI-SIG SIGNING & BROADCAST                                        │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐             │
│   │  CUSTODIAN 1    │  │  CUSTODIAN 2    │  │  CUSTODIAN 3    │             │
│   │  Signs tx       │  │  Signs tx       │  │  Signs tx       │             │
│   └────────┬────────┘  └────────┬────────┘  └────────┬────────┘             │
│            │                     │                     │                      │
│            │ (13) Each custodian independently:       │                      │
│            │      • Reviews withdrawal request         │                      │
│            │      • Verifies attestation valid         │                      │
│            │      • Signs Bitcoin tx with private key  │                      │
│            │                     │                     │                      │
│            └─────────────────────┼─────────────────────┘                      │
│                                  ↓                                            │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Fully Signed Bitcoin Transaction                            │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Transaction ID: 0xbitcoin_tx_456def...                      │           │
│   │  Size: 250 bytes (SegWit)                                    │           │
│   │  Signatures: 3-of-5 (valid multi-sig)                        │           │
│   │  Locktime: 0 (immediate)                                     │           │
│   │  Version: 2                                                   │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (14) Broadcast to Bitcoin network               │
│                             ↓                                                 │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bitcoin Network                                             │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  • Transaction broadcasted to mempool                        │           │
│   │  • Picked up by miners (high fee = priority)                │           │
│   │  • Included in block: 865432                                 │           │
│   │  • Confirmations: 0/6...                                     │           │
│   └─────────────────────────┬────────────────────────────────────┘           │
│                             │                                                 │
│                             │ (15) Wait for confirmations                     │
│                             ↓                                                 │
└───────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ (Time: ~60 minutes for 6 confirmations)
                                    ↓
┌───────────────────────────────────────────────────────────────────────────────┐
│ STEP 5: USER RECEIVES NATIVE BTC                                             │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   [USER BITCOIN WALLET]                                                       │
│   Address: bc1q...user...xyz                                                 │
│        │                                                                       │
│        │ (16) Transaction confirmed (6 blocks)                               │
│        ↓                                                                       │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  Bitcoin Balance                                             │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  Before: 0.5 BTC                                             │           │
│   │  After:  2.5 BTC                                             │           │
│   │                                                               │           │
│   │  Incoming Transaction:                                       │           │
│   │  • TX ID: 0xbitcoin_tx_456def...                            │           │
│   │  • Amount: +2.0 BTC                                          │           │
│   │  • Confirmations: 6/6 ✓ FINAL                                │           │
│   │  • Block: 865432                                             │           │
│   │  • Timestamp: [timestamp]                                    │           │
│   │                                                               │           │
│   │  ✓ USER RECEIVED NATIVE BTC                                  │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
│   User can now:                                                               │
│   • Transfer to cold storage (hardware wallet)                                │
│   • Sell on Bitcoin-native exchange (Bisq, HodlHodl)                         │
│   • Spend as native Bitcoin                                                   │
│   • No wrapped token anymore - pure BTC!                                      │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ PARALLEL: BTC-PBC RECORDS WITHDRAWAL                                          │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────┐           │
│   │  BTC-PBC Collator Node                                       │           │
│   │  (Monitors Bitcoin blockchain)                               │           │
│   ├──────────────────────────────────────────────────────────────┤           │
│   │  (17) Detects withdrawal transaction on Bitcoin              │           │
│   │       via Bitcoin light client / SPV proofs                  │           │
│   │                                                               │           │
│   │  (18) Calls pallet-bitcoin-bridge extrinsic:                 │           │
│   │       confirm_btc_withdrawal(tx_id, nonce)                   │           │
│   │                                                               │           │
│   │  (19) Updates bridge state:                                  │           │
│   │       • Mark nonce as completed                              │           │
│   │       • Record withdrawal in bridge history                  │           │
│   │       • Update total BTC locked (150 → 148 BTC)              │           │
│   │                                                               │           │
│   │  (20) Send XCMP message to Primearc:                         │           │
│   │       "Withdrawal complete: 2.0 BTC to user"                 │           │
│   │                                                               │           │
│   │  ✓ BRIDGE STATE UPDATED                                      │           │
│   └──────────────────────────────────────────────────────────────┘           │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ SECURITY MECHANISMS                                                           │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  1. MULTI-SIG CUSTODY (3-of-5)                                                │
│     • Prevents single custodian theft                                         │
│     • Requires collusion of 3+ parties to steal                               │
│     • Custodian keys stored in HSMs or cold storage                           │
│                                                                               │
│  2. M-OF-N ATTESTATION (3-of-5)                                               │
│     • Independent attesters verify burn on ĒTRID                              │
│     • Cryptographic signatures (ECDSA)                                        │
│     • Cannot be forged or replayed                                            │
│                                                                               │
│  3. NONCE PROTECTION                                                          │
│     • Each withdrawal has unique nonce                                        │
│     • Prevents replay attacks                                                 │
│     • Nonces tracked on both ĒTRID and Bitcoin sides                          │
│                                                                               │
│  4. SPV PROOFS                                                                │
│     • BTC-PBC validates Bitcoin transactions                                  │
│     • Merkle proofs ensure transaction in block                               │
│     • 6-block confirmation requirement                                        │
│                                                                               │
│  5. RATE LIMITING                                                             │
│     • Maximum withdrawal per transaction: 10 BTC                              │
│     • Daily withdrawal limit: 100 BTC (adjustable by governance)              │
│     • Prevents mass drain attack                                              │
│                                                                               │
│  6. TIMELOCK (Optional)                                                       │
│     • Large withdrawals (>5 BTC) require 24h delay                            │
│     • Allows emergency response if attack detected                            │
│     • Can be cancelled by governance if malicious                             │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ SUMMARY                                                                       │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  BEFORE WITHDRAWAL:                     AFTER WITHDRAWAL:                     │
│  • User wBTC (ĒTRID): 5.0              • User wBTC (ĒTRID): 3.0              │
│  • User BTC (Bitcoin): 0.5             • User BTC (Bitcoin): 2.5             │
│  • Bridge locked BTC: 150.0            • Bridge locked BTC: 148.0            │
│  • Total wBTC supply: 100.0            • Total wBTC supply: 98.0             │
│                                                                               │
│  TRANSACTION COSTS:                                                           │
│  • ĒTRID burn: ~$0.001 (Primearc gas)                                        │
│  • Bitcoin tx: ~$12.50 (high-priority mining fee)                             │
│  • Bridge fee:  ~$2 (paid to custodians/relayers)                            │
│  • Total cost:  ~$14.50                                                       │
│                                                                               │
│  TIME: ~70 minutes                                                            │
│  • Attestation collection: 1-2 minutes                                        │
│  • Multi-sig signing: 3-5 minutes                                             │
│  • Bitcoin broadcast: instant                                                 │
│  • 6 confirmations: ~60 minutes                                               │
│                                                                               │
│  SECURITY: 3-of-5 multi-sig + 3-of-5 attestation + SPV proofs                │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘
```

---

## COMPLETE SYSTEM OVERVIEW

**High-level view of all token flows in the ĒTRID ecosystem**

```
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                      ĒTRID COMPLETE BRIDGE ECOSYSTEM                                     │
│                  (All External Chains ↔ ĒTRID ↔ External Chains)                        │
└─────────────────────────────────────────────────────────────────────────────────────────┘

                          EXTERNAL BLOCKCHAINS
┌────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                    │
│   [Bitcoin]    [Ethereum]   [Solana]   [BSC]   [Polygon]   [Base L2]   [Others]  │
│      │             │            │         │         │           │          │       │
│      │ Lock/Unlock │            │         │         │           │          │       │
│      │             │            │         │         │           │          │       │
└──────┼─────────────┼────────────┼─────────┼─────────┼───────────┼──────────┼───────┘
       │             │            │         │         │           │          │
       │             │            │         │         │           │          │
       ↓             ↓            ↓         ↓         ↓           ↓          ↓
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                   PARTITION BURST CHAINS (PBCs) - LAYER 1                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ BTC-PBC  │  │ ETH-PBC  │  │ SOL-PBC  │  │ BNB-PBC  │  │ MATIC-PBC│  ... (13)    │
│  │ Collator │  │ Collator │  │ Collator │  │ Collator │  │ Collator │              │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘              │
│       │ SPV Proofs  │ Event Logs  │ State Proofs│ Dual Verify │                     │
│       │             │             │             │             │                      │
│  ┌────┴─────────────┴─────────────┴─────────────┴─────────────┴────┐                │
│  │                 XCMP/HRMP (Cross-Chain Messages)                 │                │
│  │              Secure relay chain communication layer              │                │
│  └──────────────────────────────┬───────────────────────────────────┘                │
│                                 │                                                    │
└─────────────────────────────────┼────────────────────────────────────────────────────┘
                                  │
                                  ↓
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                   PRIMEARC CORE CHAIN (Relay Chain) - LAYER 0                        │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │  BRIDGE INFRASTRUCTURE                                                        │  │
│  │  ┌─────────────────────────┐  ┌──────────────────────────┐                   │  │
│  │  │ pallet-token-messenger  │  │ pallet-bridge-attestation│                   │  │
│  │  │ • burn_and_send()       │  │ • M-of-N verification    │                   │  │
│  │  │ • receive_and_mint()    │  │ • ECDSA signature check  │                   │  │
│  │  │ • Nonce management      │  │ • Replay protection      │                   │  │
│  │  └─────────────────────────┘  └──────────────────────────┘                   │  │
│  │                                                                                │  │
│  │  NATIVE TOKENS:                                                               │  │
│  │  • ÉTR (native coin) - 10B supply                                             │  │
│  │  • ËDSC (stablecoin) - Elastic supply                                         │  │
│  │                                                                                │  │
│  │  WRAPPED TOKENS:                                                              │  │
│  │  • wBTC, wETH, wSOL, wBNB, wMATIC, wADA, wLINK, wXRP, wDOGE, wTRX, wXLM     │  │
│  │  • All backed 1:1 by locked assets on external chains                        │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                      │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │  PRIMESWAP DEX (On-Chain AMM)                                                 │  │
│  │  ┌────────────────────────────────────────────────────────────────────────┐  │  │
│  │  │  ONE-SIDED LIQUIDITY POOLS (Foundation Bootstrap)                      │  │  │
│  │  │  ───────────────────────────────────────────────────────────────────   │  │  │
│  │  │  11 Pools (VirtualReserveAMM):                                         │  │  │
│  │  │  • ÉTR/wBTC    • ÉTR/wETH    • ÉTR/wSOL    • ÉTR/wBNB                 │  │  │
│  │  │  • ÉTR/wMATIC  • ÉTR/wADA    • ÉTR/wLINK   • ÉTR/wXRP                 │  │  │
│  │  │  • ÉTR/wDOGE   • ÉTR/wTRX    • ÉTR/wXLM                               │  │  │
│  │  │                                                                         │  │  │
│  │  │  Mechanism:                                                            │  │  │
│  │  │  ┌──────────────────────────────────────────────────┐                 │  │  │
│  │  │  │  Real ÉTR:     845.75M (seeded by Foundation)    │                 │  │  │
│  │  │  │  Real wBTC:    0 initially                       │                 │  │  │
│  │  │  │  Virtual wBTC: 33.83 (oracle-calculated)         │                 │  │  │
│  │  │  │  ─────────────────────────────────────────────   │                 │  │  │
│  │  │  │  Total for AMM: Real + Virtual                   │                 │  │  │
│  │  │  │  Price: Oracle → Market (gradual transition)     │                 │  │  │
│  │  │  └──────────────────────────────────────────────────┘                 │  │  │
│  │  │                                                                         │  │  │
│  │  │  1 Pool (EDSCOraclePool):                                              │  │  │
│  │  │  • ÉTR/ËDSC - One-sided ËDSC bootstrap                                 │  │  │
│  │  │                                                                         │  │  │
│  │  │  1 Pool (EDSCPegPool):                                                 │  │  │
│  │  │  • ËDSC/USDT - Stability pool (StableSwap curve)                       │  │  │
│  │  └────────────────────────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                      │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │  GOVERNANCE & TREASURY                                                        │  │
│  │  • pallet-treasury: Collects 0.3% swap fees                                  │  │
│  │  • pallet-consensus-day: Annual voting                                        │  │
│  │  • pallet-validator-committee: Stake-weighted governance                     │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
│                                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────┘
                                  │
                                  │ Bridge Adapters
                                  ↓
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                   EXTERNAL DEX LISTINGS (Bridge Adapters)                            │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │  EVM Chains: Base L2, Arbitrum, Optimism, BSC, Polygon, Avalanche           │  │
│  │  ┌────────────────────────────────────────────────────────────────────────┐  │  │
│  │  │  Wrapped ÉTR (ERC-20/BEP-20)                                           │  │  │
│  │  │  • Deployed via bridge adapter scripts                                 │  │  │
│  │  │  • Minted on lock, burned on unlock                                    │  │  │
│  │  │  • Tradable on: Uniswap, PancakeSwap, QuickSwap                       │  │  │
│  │  └────────────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                                │  │
│  │  Solana:                                                                       │  │
│  │  ┌────────────────────────────────────────────────────────────────────────┐  │  │
│  │  │  SPL ÉTR Token                                                         │  │  │
│  │  │  • Mint authority: Bridge program                                      │  │  │
│  │  │  • Tradable on: Raydium, Orca                                          │  │  │
│  │  └────────────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                                │  │
│  │  Status: ✅ Contracts deployed (addresses TBD)                                │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────────────┘
                                  │
                                  │ (Future)
                                  ↓
┌──────────────────────────────────────────────────────────────────────────────────────┐
│                   LAYERZERO INTEGRATION (Planned)                                    │
│  ┌───────────────────────────────────────────────────────────────────────────────┐  │
│  │  Omnichain Fungible Token (OFT)                                               │  │
│  │  ┌────────────────────────────────────────────────────────────────────────┐  │  │
│  │  │  EtridOFT.sol                                                          │  │  │
│  │  │  • Deployed on 50+ LayerZero-connected chains                         │  │  │
│  │  │  • Burn on source → Mint on destination                               │  │  │
│  │  │  • Unified global ÉTR supply                                           │  │  │
│  │  │  • Users buy ÉTR from ANY chain using ANY token                       │  │  │
│  │  └────────────────────────────────────────────────────────────────────────┘  │  │
│  │                                                                                │  │
│  │  Status: ❌ Not implemented (design phase)                                    │  │
│  │  Estimated Work: 4-8 weeks + security audit                                   │  │
│  └───────────────────────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────────────────────────┘


┌─────────────────────────────────────────────────────────────────────────────────────┐
│                              TOKEN FLOW SUMMARY                                      │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                      │
│  INBOUND (External → ĒTRID):                                                        │
│  1. User locks BTC on Bitcoin                                                       │
│  2. BTC-PBC detects via SPV proofs                                                  │
│  3. Attesters sign (3-of-5)                                                         │
│  4. Primearc mints wBTC to user                                                     │
│  5. User swaps wBTC → ÉTR on PrimeSwap                                              │
│                                                                                      │
│  OUTBOUND (ĒTRID → External):                                                       │
│  1. User burns ÉTR on Primearc                                                      │
│  2. Attesters sign withdrawal (3-of-5)                                              │
│  3. Relayer submits to external chain                                               │
│  4. External bridge mints wÉTR to user                                              │
│  5. User trades wÉTR on external DEX (Uniswap, PancakeSwap, etc.)                  │
│                                                                                      │
│  REVERSE (External → ĒTRID → External):                                             │
│  1. User burns wBTC on Primearc                                                     │
│  2. Attesters sign (3-of-5)                                                         │
│  3. Bitcoin custodians unlock native BTC                                            │
│  4. User receives native BTC on Bitcoin blockchain                                  │
│                                                                                      │
│  KEY ADVANTAGES:                                                                    │
│  • No wrapped assets needed between ĒTRID chains (native XCMP/HRMP)                 │
│  • One-sided pools = Foundation only provides ÉTR                                   │
│  • M-of-N attestation = Decentralized security                                      │
│  • Multi-sig custody = No single point of failure                                   │
│  • Low cost (~$0.001 ĒTRID gas, ~$1 bridge fees)                                   │
│  • Fast finality (~12 seconds on ĒTRID, ~2-5 min cross-chain)                      │
│                                                                                      │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

## ACTOR REFERENCE GUIDE

**Who's who in the ĒTRID bridge ecosystem**

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           ACTOR ROLES & RESPONSIBILITIES                        │
└─────────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 1. END USERS                                                                  │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Interact with ĒTRID ecosystem                                          │
│                                                                               │
│  Actions:                                                                     │
│  • Bridge tokens from external chains → ĒTRID                                 │
│  • Swap tokens on PrimeSwap                                                   │
│  • Provide liquidity (future)                                                 │
│  • Bridge tokens from ĒTRID → external chains                                 │
│  • Trade wrapped ÉTR on external DEXs                                         │
│                                                                               │
│  Requirements:                                                                │
│  • Wallet with native tokens for gas fees                                    │
│  • Understanding of bridge risks (custody, attestation)                       │
│  • Patience for cross-chain finality (5-70 minutes depending on chain)       │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 2. BRIDGE RELAYERS                                                            │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Monitor source chains and submit messages to destination chains        │
│                                                                               │
│  Actions:                                                                     │
│  • Monitor external blockchains for deposit events (Bitcoin, Ethereum, etc.) │
│  • Monitor ĒTRID for burn events                                              │
│  • Collect attestation signatures from attesters (wait for 3-of-5)           │
│  • Submit cross-chain messages to destination chains                          │
│  • Pay gas fees on destination chains (reimbursed via bridge fees)           │
│                                                                               │
│  Technical Stack:                                                             │
│  • TypeScript/Node.js service                                                 │
│  • WebSocket connections to multiple RPC nodes                                │
│  • Database for nonce tracking and message queue                              │
│  • Prometheus metrics for monitoring                                          │
│                                                                               │
│  Incentives:                                                                  │
│  • Earn bridge fees (~$1-2 per transfer)                                     │
│  • May receive ÉTR rewards from Treasury                                      │
│                                                                               │
│  Security:                                                                    │
│  • Cannot forge attestations (no private keys)                                │
│  • Cannot steal funds (only message passing)                                  │
│  • Slashing not applicable (permissionless role)                              │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 3. ATTESTERS (M-of-N: 3-of-5)                                                 │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Sign cross-chain messages to verify burns/locks are legitimate         │
│                                                                               │
│  Actions:                                                                     │
│  • Monitor ĒTRID for burn/lock events                                         │
│  • Verify event authenticity (check on-chain state)                           │
│  • Sign message hash with ECDSA private key                                   │
│  • Submit signature to attestation pallet                                     │
│  • Coordinate with other attesters (off-chain communication)                  │
│                                                                               │
│  Requirements:                                                                │
│  • Hardware Security Module (HSM) or secure key management                    │
│  • High uptime (>99.9% availability)                                          │
│  • Fast response time (<30 seconds per attestation)                           │
│  • Stake requirement (e.g., 100,000 ÉTR) - may be added                      │
│                                                                               │
│  Incentives:                                                                  │
│  • Share of bridge fees (~30% split among active attesters)                  │
│  • ÉTR rewards from inflation (if implemented)                                │
│                                                                               │
│  Security:                                                                    │
│  • Private keys stored in HSM or cold storage                                 │
│  • Cannot collude alone (need 3-of-5 threshold)                               │
│  • Slashable if caught signing false messages                                 │
│  • Can be removed by governance vote if malicious                             │
│                                                                               │
│  Current Attesters (Example Set):                                             │
│  1. ĒTRID Foundation                                                          │
│  2. Trusted Validator 1                                                       │
│  3. Trusted Validator 2                                                       │
│  4. Third-party Security Firm                                                 │
│  5. Community-elected Attester                                                │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 4. BRIDGE CUSTODIANS (Multi-Sig: 3-of-5)                                      │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Control locked assets on external chains (Bitcoin, Ethereum, etc.)     │
│                                                                               │
│  Actions:                                                                     │
│  • Hold private keys for multi-sig wallets                                    │
│  • Review withdrawal requests (verify attestations)                           │
│  • Sign Bitcoin/Ethereum transactions to release locked assets                │
│  • Monitor custody wallet balances                                            │
│  • Coordinate multi-sig signing ceremonies                                    │
│                                                                               │
│  Requirements:                                                                │
│  • Hardware Security Module (HSM) mandatory                                   │
│  • Physical security (secure location)                                        │
│  • 24/7 availability for emergency responses                                  │
│  • Regular audits and key rotation                                            │
│  • Insurance coverage (recommended)                                           │
│                                                                               │
│  Custody Assets (Example):                                                    │
│  • Bitcoin: 150 BTC (~$3.75M at $25k/BTC)                                     │
│  • Ethereum: 5,000 ETH (~$10M at $2k/ETH)                                     │
│  • Solana: 500,000 SOL (~$10M at $20/SOL)                                     │
│  • Total: ~$23.75M in custody                                                 │
│                                                                               │
│  Incentives:                                                                  │
│  • Share of bridge fees (~50% split among custodians)                        │
│  • Annual retainer (paid in ÉTR from Treasury)                               │
│                                                                               │
│  Security:                                                                    │
│  • Private keys never touch internet (air-gapped signing)                     │
│  • Multi-sig prevents single custodian theft                                  │
│  • Insurance against key loss or theft                                        │
│  • Can be replaced by governance vote                                         │
│                                                                               │
│  Current Custodians (Example Set):                                            │
│  1. ĒTRID Foundation (Gnosis Safe)                                            │
│  2. Institutional Custodian (Fireblocks)                                      │
│  3. Trusted Validator 1                                                       │
│  4. Trusted Validator 2                                                       │
│  5. Third-party Custody Service (BitGo)                                       │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 5. PBC COLLATORS                                                              │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Secure Partition Burst Chains and validate external chain data         │
│                                                                               │
│  Actions:                                                                     │
│  • Run light clients for external chains (BTC, ETH, SOL, etc.)               │
│  • Validate SPV proofs (Bitcoin)                                              │
│  • Monitor event logs (Ethereum)                                              │
│  • Validate state proofs (Solana)                                             │
│  • Produce blocks for PBC (consensus)                                         │
│  • Send XCMP/HRMP messages to Primearc Core Chain                             │
│                                                                               │
│  Requirements:                                                                │
│  • High-performance server (16+ cores, 64GB+ RAM)                             │
│  • RPC connections to external chains                                         │
│  • Stake requirement (varies by PBC)                                          │
│  • Uptime requirement (>98%)                                                  │
│                                                                               │
│  Incentives:                                                                  │
│  • Block rewards (ÉTR inflation)                                              │
│  • Transaction fees from PBC                                                  │
│  • Share of bridge fees (10-20%)                                             │
│                                                                               │
│  Security:                                                                    │
│  • Slashable for downtime or invalid proofs                                   │
│  • Secured by Primearc Core Chain (shared security)                           │
│  • Can be removed by governance if malicious                                  │
│                                                                               │
│  Examples:                                                                    │
│  • BTC-PBC Collator: Runs Bitcoin light client, validates SPV proofs         │
│  • ETH-PBC Collator: Monitors Ethereum event logs, validates signatures      │
│  • SOL-PBC Collator: Validates Solana state proofs, monitors account updates │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 6. PRIMEARC VALIDATORS                                                        │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Secure Primearc Core Chain and finalize cross-chain messages           │
│                                                                               │
│  Actions:                                                                     │
│  • Produce blocks using ASF consensus                                         │
│  • Validate XCMP/HRMP messages from PBCs                                      │
│  • Execute bridge pallet extrinsics (burn_and_send, receive_and_mint)        │
│  • Verify attestation signatures (3-of-5)                                     │
│  • Vote on governance proposals                                               │
│                                                                               │
│  Requirements:                                                                │
│  • Minimum stake: 1,000,000 ÉTR                                               │
│  • High-performance server (32+ cores, 128GB+ RAM)                            │
│  • Uptime: >99.5%                                                             │
│  • Fast network connection (1Gbps+)                                           │
│                                                                               │
│  Incentives:                                                                  │
│  • Block rewards (ÉTR inflation)                                              │
│  • Transaction fees                                                           │
│  • Voting rewards (Consensus Day)                                             │
│                                                                               │
│  Security:                                                                    │
│  • Slashable for equivocation, downtime, or malicious behavior                │
│  • Session keys rotated regularly                                             │
│  • Can be removed by governance vote                                          │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 7. PRIMESWAP LIQUIDITY PROVIDERS (Future)                                     │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Provide additional liquidity to PrimeSwap pools                         │
│                                                                               │
│  Actions:                                                                     │
│  • Add ÉTR + wBTC (or other assets) to pools                                  │
│  • Receive LP tokens representing pool share                                  │
│  • Earn trading fees (0.3% of swap volume)                                   │
│  • Stake LP tokens in MasterChef for additional rewards                       │
│                                                                               │
│  Status: Currently one-sided (Foundation only)                                 │
│  Future: Two-sided pools will open when sufficient liquidity reached          │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────────────────┐
│ 8. ĒTRID FOUNDATION                                                           │
├───────────────────────────────────────────────────────────────────────────────┤
│                                                                               │
│  Role: Bootstrap ecosystem and maintain core infrastructure                   │
│                                                                               │
│  Actions:                                                                     │
│  • Seed one-sided liquidity pools with ÉTR                                    │
│  • Deploy bridge adapters on external chains                                  │
│  • Operate initial attesters and custodians                                   │
│  • Fund protocol development                                                  │
│  • Coordinate security audits                                                 │
│  • Manage Treasury funds (via governance)                                     │
│                                                                               │
│  Assets Controlled:                                                           │
│  • ÉTR treasury: 2B ÉTR (20% of supply)                                       │
│  • PrimeSwap liquidity: ~1.2B ÉTR (seeded in 13 pools)                       │
│  • Development fund: 500M ÉTR                                                 │
│                                                                               │
└───────────────────────────────────────────────────────────────────────────────┘


┌─────────────────────────────────────────────────────────────────────────────────┐
│ ACTOR INTERACTION MATRIX                                                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  USER → RELAYER:            Deposit/withdrawal request                          │
│  RELAYER → ATTESTER:        Request signature for message                       │
│  ATTESTER → RELAYER:        Provide signature (3-of-5 threshold)                │
│  RELAYER → CUSTODIAN:       Submit attested withdrawal request                  │
│  CUSTODIAN → BITCOIN:       Sign and broadcast multi-sig transaction            │
│  PBC COLLATOR → PRIMEARC:   Send XCMP message (deposit confirmed)               │
│  PRIMEARC → PBC COLLATOR:   Send XCMP message (withdrawal approved)             │
│  USER → PRIMESWAP:          Swap tokens (wBTC → ÉTR)                            │
│  PRIMESWAP → TREASURY:      Send 0.3% swap fees                                 │
│  FOUNDATION → PRIMESWAP:    Seed liquidity pools                                │
│  VALIDATORS → ATTESTERS:    Verify signatures on-chain                          │
│                                                                                  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## TECHNICAL NOTES

### Bridge Adapter Locations
```
/05-multichain/bridges/adapters/
├── base/                      # Base L2 (Ethereum L2)
│   ├── deploy-tokens.ts       # Deploy WrappedETR.sol
│   ├── bridge.ts              # Monitor & relay
│   └── README.md
├── bsc/                       # Binance Smart Chain
│   ├── deploy-pancakeswap.ts # Deploy & create pools
│   └── README.md
├── solana/                    # Solana
│   ├── RAYDIUM_DEPLOYMENT_GUIDE.md
│   └── bridge-adapter.ts
└── ADAPTER_DEVELOPMENT_GUIDE.md
```

### Bridge Pallet Locations
```
/05-multichain/pallets-shared/
├── pallet-token-messenger/
│   ├── src/lib.rs             # burn_and_send(), receive_and_mint()
│   └── README.md
├── pallet-bridge-attestation/
│   ├── src/lib.rs             # M-of-N signature verification
│   └── README.md
└── README.md
```

### PBC Locations
```
/05-multichain/partition-burst-chains/
├── pbc-chains/
│   ├── btc-pbc/               # Bitcoin PBC
│   ├── eth-pbc/               # Ethereum PBC
│   ├── sol-pbc/               # Solana PBC
│   ├── bnb-pbc/               # BSC PBC
│   └── ...                    # 13 total PBCs
```

### PrimeSwap Locations
```
/contracts/primeswap/
├── src/
│   ├── core/
│   │   ├── PrimeSwapFactory.sol
│   │   ├── PrimeSwapPair.sol  # VirtualReserveAMM, EDSCOraclePool, EDSCPegPool
│   │   └── PrimeSwapERC20.sol
│   ├── periphery/
│   │   └── PrimeSwapRouter.sol
│   └── PrimeSwapOracle.sol
└── scripts/
    ├── deploy-primeswap-pools.js
    └── seed-etr-pools.js
```

---

## SECURITY CONSIDERATIONS

### Multi-Sig Thresholds
- **Attesters:** 3-of-5 (M-of-N signature verification)
- **Custodians:** 3-of-5 (multi-sig wallet control)
- **Rationale:** Prevents single point of failure, requires collusion of 3+ parties

### Replay Protection
- **Nonces:** Each cross-chain message has unique nonce
- **Tracking:** Nonces tracked on both source and destination chains
- **Rejection:** Duplicate nonces rejected by pallet-token-messenger

### SPV Proofs (Bitcoin)
- **Light Client:** BTC-PBC runs Bitcoin light client
- **Merkle Proofs:** Validates transaction inclusion in blocks
- **Confirmations:** Requires 6 Bitcoin block confirmations (~60 minutes)

### Rate Limiting
- **Per Transaction:** Max 10 BTC (or equivalent) per single withdrawal
- **Daily Limit:** Max 100 BTC (adjustable by governance)
- **Emergency Halt:** Governance can pause bridge in case of attack

### Slashing
- **Validators:** Slashed for equivocation or prolonged downtime
- **Attesters:** Slashed for signing false messages (future implementation)
- **Collators:** Slashed for invalid proofs or downtime

---

## GLOSSARY

- **ASF:** Ascending Scale of Finality - ĒTRID's consensus algorithm
- **PBC:** Partition Burst Chain - Specialized chain for each external blockchain
- **XCMP/HRMP:** Cross-Consensus Message Passing / Horizontal Relay-routed Message Passing
- **SPV:** Simplified Payment Verification (Bitcoin light client)
- **M-of-N:** Multi-signature threshold (e.g., 3-of-5 = need 3 out of 5 signatures)
- **wBTC/wETH/etc:** Wrapped versions of external tokens on ĒTRID
- **ÉTR:** ĒTRID native coin (pronounced "ee-ter")
- **ËDSC:** ĒTRID Dollar Stablecoin (pronounced "ee-des-see")
- **OFT:** Omnichain Fungible Token (LayerZero standard)
- **VirtualReserveAMM:** One-sided liquidity pool with oracle-calculated virtual reserves
- **EDSCOraclePool:** One-sided ËDSC pool for ÉTR/ËDSC swaps
- **EDSCPegPool:** Stability pool for ËDSC/USDT (StableSwap curve)

---

**Document Version:** 1.0
**Last Updated:** December 8, 2025
**Status:** Complete
**Purpose:** Developer reference for ĒTRID bridge architecture

---

## RELATED DOCUMENTATION

- **[LAYERZERO_CCTP_RESEARCH.md](/Users/macbook/Desktop/etrid/LAYERZERO_CCTP_RESEARCH.md)** - LayerZero & CCTP analysis
- **[LAYERZERO_ONE_SIDED_POOL_ARCHITECTURE.md](/Users/macbook/Desktop/etrid/LAYERZERO_ONE_SIDED_POOL_ARCHITECTURE.md)** - Detailed one-sided pool mechanics
- **[MAINNET_BRIDGE_IMPLEMENTATION_PLAN.md](/Users/macbook/Desktop/etrid/docs/MAINNET_BRIDGE_IMPLEMENTATION_PLAN.md)** - Bridge implementation roadmap
- **[Bridge Adapter README](/Users/macbook/Desktop/etrid/05-multichain/bridges/adapters/README.md)** - Adapter deployment guide
- **[EDSC Bridge README](/Users/macbook/Desktop/etrid/05-multichain/bridges/protocols/edsc-bridge/README.md)** - EDSC bridge architecture

---

**End of Document**
