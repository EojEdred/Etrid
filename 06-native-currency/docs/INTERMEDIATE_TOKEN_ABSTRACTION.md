# ETRID Intermediate Token Abstraction Layer

## Executive Summary

This document defines the **Intermediate Token Abstraction Layer** for ETRID, where wrapped tokens (wBTC, wETH, wSOL, etc.) are completely invisible to end users. Users interact ONLY with external currencies (BTC, ETH, SOL) and ÉTR - wrapped tokens exist purely as an internal bridging mechanism, similar to how Ethereum users never directly interact with wei or gas tokens.

**User Experience Vision:**
- User sends BTC → Receives ÉTR directly
- User sends ÉTR → Receives BTC directly
- wBTC exists only as a transient intermediate state
- No wrapped tokens appear in user wallets
- Seamless multi-hop transactions appear as single operations

---

## Table of Contents

1. [Research: Abstraction Patterns](#research-abstraction-patterns)
2. [Core Abstraction Architecture](#core-abstraction-architecture)
3. [User Experience Flows](#user-experience-flows)
4. [Smart Contract Architecture](#smart-contract-architecture)
5. [Security Considerations](#security-considerations)
6. [Implementation Plan](#implementation-plan)
7. [Edge Cases & Failure Modes](#edge-cases--failure-modes)

---

## Research: Abstraction Patterns

### 1. Ethereum Wei/Gwei Denomination Pattern

**Pattern Analysis:**
- Users transact in ETH, internally converted to wei (10^18)
- Gas prices shown in gwei (10^9) for readability
- Wallets and dApps handle conversion automatically
- Users never manually convert between denominations

**Application to ETRID:**
- External currencies (BTC, ETH) = User-facing denomination
- Wrapped tokens (wBTC, wETH) = Internal denomination (like wei)
- Automatic conversion at interface layer
- Users never see wrapped token balances

**Sources:**
- [Understanding Gwei: Ethereum's Smallest Denominations](https://platform.chainbase.com/blog/article/understanding-wei-and-gwei-ethereum-s-smallest-denominations-explained)
- [Ethereum Gas Fees Explained](https://plisio.net/blog/gwei-and-gas-fueling-the-ethereum-network)

### 2. ERC-4337 Account Abstraction & Gas Abstraction

**Pattern Analysis:**
- Paymaster contracts sponsor gas fees, users pay in ERC-20 tokens
- Meta-transactions: relayers execute transactions, charging non-native tokens
- Bundlers aggregate operations, hiding complexity
- Users express intent ("swap 1 ETH to USDC"), system handles execution

**Application to ETRID:**
- Bridge contract acts as "Paymaster" for wrapped token handling
- Relayers handle bridge → swap operations atomically
- Users submit intent: "I want X BTC worth of ÉTR"
- System executes: Bridge → Mint wBTC → Swap wBTC → ÉTR → Deliver ÉTR

**Sources:**
- [2025 Account Abstraction Trends](https://coingape.com/brandtalk/opinion/2025-account-abstraction-trends-our-senior-smart-contract-engineer-weighs-in/)
- [Gas Abstraction and Multichain AA Wallets](https://blog.web3auth.io/multichain-account-gas-abstraction/)

### 3. Intent-Based Architecture (2025 DeFi Trend)

**Pattern Analysis:**
- User specifies WHAT they want (end result), not HOW to get it
- AI agents route across chains/bridges/DEXs automatically
- Example: "Buy $100 of Token X" → agent handles chain selection, bridging, swapping
- Abstraction protocols use natural language commands

**Application to ETRID:**
- User intent: "Buy ÉTR with BTC"
- Router contract: Detects optimal path (Bridge → Wrap → Swap)
- Execution: Atomic multi-step transaction
- Result: User receives ÉTR, never sees wBTC

**Sources:**
- [DeFi in 2025: Intent-Based UX](https://idesignstrategy.medium.com/defi-in-2025-whats-real-what-s-noise-and-what-comes-next-4498f6a5c136)
- [Mix, Match, Automate DeFi Swaps](https://medium.com/@factor.fi/mix-match-automate-defi-swaps-341d1030bf31)

### 4. Cross-Chain Bridge Abstraction (Stargate, Symbiosis)

**Pattern Analysis:**
- Modern bridges remove synthetic/wrapped token steps from UX
- Users specify: "Send 100 USDC to Arbitrum" (single action)
- Backend: Lock → Mint → Route → Swap (invisible to user)
- Native asset delivery (not wrapped versions)

**Application to ETRID:**
- User deposits native BTC/ETH/SOL
- Bridge receives, mints wrapped version internally
- Immediate auto-swap to ÉTR
- User sees only: "Depositing BTC" → "Received ÉTR"

**Sources:**
- [Types of Crypto Bridges 2025](https://across.to/blog/types-of-crypto-bridges-2025)
- [Best Crypto Bridges 2025: Intent-Based Systems](https://coincodex.com/article/33034/best-crypto-bridges/)

---

## Core Abstraction Architecture

### High-Level Flow

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    INTERMEDIATE TOKEN ABSTRACTION LAYER                   │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│  USER VIEW (What Users See):                                             │
│  ┌───────────────────────────────────────────────────────────────────┐   │
│  │                                                                   │   │
│  │   [Send BTC] ──────────────────────────> [Receive ÉTR]          │   │
│  │                                                                   │   │
│  │   Simple, one-step transaction                                   │   │
│  │   Status: "Converting BTC to ÉTR..."                             │   │
│  │                                                                   │   │
│  └───────────────────────────────────────────────────────────────────┘   │
│                                                                           │
│  SYSTEM VIEW (What Actually Happens):                                    │
│  ┌───────────────────────────────────────────────────────────────────┐   │
│  │                                                                   │   │
│  │   1. Bridge Deposits BTC                                         │   │
│  │      └─> Lock BTC on Bitcoin chain                               │   │
│  │                                                                   │   │
│  │   2. Mint wBTC (Intermediate - ABSTRACTED)                       │   │
│  │      └─> Wrapped token created on ETRID                          │   │
│  │      └─> Exists for ~2 seconds                                   │   │
│  │      └─> User never owns this token                              │   │
│  │                                                                   │   │
│  │   3. Auto-Swap wBTC → ÉTR                                         │   │
│  │      └─> PrimeSwap VirtualReserveAMM pool                        │   │
│  │      └─> wBTC burned immediately after swap                      │   │
│  │                                                                   │   │
│  │   4. Deliver ÉTR to User                                          │   │
│  │      └─> User receives native ÉTR                                 │   │
│  │                                                                   │   │
│  └───────────────────────────────────────────────────────────────────┘   │
│                                                                           │
│  CRITICAL DESIGN PRINCIPLE:                                              │
│  Wrapped tokens are EPHEMERAL - they exist only within a single         │
│  atomic transaction, never appearing in user balances or wallets.        │
│                                                                           │
└──────────────────────────────────────────────────────────────────────────┘
```

### Abstraction Layer Components

```
┌────────────────────────────────────────────────────────────────────────┐
│                      COMPONENT ARCHITECTURE                             │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Layer 1: USER INTERFACE                                               │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  - Show native currency balances (BTC, ETH, SOL, ÉTR)            │  │
│  │  - Hide wrapped token balances (wBTC, wETH, wSOL)                │  │
│  │  - Display unified transaction: "BTC → ÉTR"                       │  │
│  │  - Show single progress bar for multi-hop operation               │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Layer 2: INTENT ROUTER                                                │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  IntentRouter Contract:                                           │  │
│  │  - Parse user intent: convertToETR(sourceCurrency, amount)       │  │
│  │  - Determine optimal route (bridge + DEX)                        │  │
│  │  - Calculate expected output (with slippage)                     │  │
│  │  - Execute atomic multi-step transaction                         │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Layer 3: BRIDGE ABSTRACTION                                           │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  BridgeAbstraction Contract:                                      │  │
│  │  - Receive bridge confirmation (wBTC minted)                     │  │
│  │  - Immediately approve wBTC to SwapRouter                        │  │
│  │  - Never transfer wBTC to user address                           │  │
│  │  - Emit internal event for monitoring                            │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Layer 4: AUTO-SWAP EXECUTOR                                           │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  AutoSwapExecutor Contract:                                       │  │
│  │  - Execute swap: wBTC → ÉTR                                       │  │
│  │  - Use PrimeSwap VirtualReserveAMM pools                         │  │
│  │  - Apply slippage protection                                     │  │
│  │  - Deliver ÉTR directly to user address                          │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Layer 5: EMERGENCY FALLBACK                                           │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Fallback Mechanism:                                              │  │
│  │  - If auto-swap fails → transfer wBTC to user                    │  │
│  │  - Emit event: "Manual swap required"                            │  │
│  │  - Provide UI for user to complete swap manually                 │  │
│  │  - Refund bridge fees if applicable                              │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## User Experience Flows

### Flow 1: Deposit (BTC → ÉTR)

```
┌────────────────────────────────────────────────────────────────────────┐
│                    DEPOSIT FLOW: BTC → ÉTR                             │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  USER PERSPECTIVE:                                                     │
│  ════════════════                                                      │
│                                                                         │
│  Step 1: User initiates deposit                                        │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  [Wallet UI]                                                      │  │
│  │                                                                   │  │
│  │  Select Currency:  [BTC ▼]                                       │  │
│  │  Amount:           [0.5 BTC]                                     │  │
│  │  You will receive: ~12,500 ÉTR (@ $0.004/ÉTR)                   │  │
│  │                                                                   │  │
│  │  [Convert to ÉTR]  ← Single button, one action                  │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Step 2: Transaction progress (unified status)                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Converting 0.5 BTC to ÉTR...                                     │  │
│  │                                                                   │  │
│  │  [████████░░░░░░] 60%                                            │  │
│  │                                                                   │  │
│  │  Status: Processing transaction                                  │  │
│  │  Estimated time: ~2 minutes                                      │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Step 3: Completion                                                    │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  ✓ Successfully received 12,487 ÉTR                               │  │
│  │                                                                   │  │
│  │  Transaction Details:                                            │  │
│  │  - Converted: 0.5 BTC                                            │  │
│  │  - Received:  12,487 ÉTR                                         │  │
│  │  - Rate:      ~24,974 ÉTR per BTC                               │  │
│  │  - Fee:       0.1% (13 ÉTR)                                      │  │
│  │                                                                   │  │
│  │  [View on Explorer]                                              │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  WHAT USER NEVER SEES:                                                 │
│  - wBTC token being minted                                             │
│  - wBTC balance in wallet                                              │
│  - Separate "swap" transaction                                         │
│  - Multiple transaction confirmations                                  │
│                                                                         │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  SYSTEM PERSPECTIVE:                                                   │
│  ═══════════════════                                                   │
│                                                                         │
│  Transaction 1: Lock BTC on Bitcoin chain                              │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  User → Bitcoin Bridge Contract                                  │  │
│  │  Amount: 0.5 BTC                                                 │  │
│  │  Destination: ETRID address (SS58)                              │  │
│  │  Status: Confirmed (6 blocks)                                    │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Transaction 2: Directors sign attestation                             │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  5-of-9 Director signatures collected                            │  │
│  │  Attestation: "0.5 BTC locked for user 0x..."                   │  │
│  │  Relayer submits attestation to ETRID                           │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Transaction 3: Mint wBTC + Auto-Swap (ATOMIC)                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Call: IntentRouter.convertToETR(BTC, 0.5, minETROut)          │  │
│  │                                                                   │  │
│  │  Internal execution:                                             │  │
│  │  1. EtridBridge.mintTokens()                                     │  │
│  │     └─> Mint 0.5 wBTC to IntentRouter address (NOT user)       │  │
│  │                                                                   │  │
│  │  2. AutoSwapExecutor.executeSwap()                               │  │
│  │     └─> wBTC.approve(PrimeSwapRouter, 0.5 wBTC)                 │  │
│  │     └─> PrimeSwapRouter.swapExactTokensForTokens()              │  │
│  │         Path: [wBTC, ÉTR]                                         │  │
│  │         AmountIn: 0.5 wBTC                                       │  │
│  │         MinOut: 12,375 ÉTR (1% slippage)                         │  │
│  │         To: USER_ADDRESS ← Direct delivery                       │  │
│  │                                                                   │  │
│  │  3. wBTC balance of IntentRouter: 0 (fully swapped)             │  │
│  │                                                                   │  │
│  │  Result: User receives 12,487 ÉTR                                │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ⚠️ CRITICAL: wBTC exists ONLY during step 3, within single tx         │
│  wBTC lifetime: ~2 seconds (EVM block time)                            │
│  wBTC never appears in user wallet or balance                          │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

### Flow 2: Withdrawal (ÉTR → BTC)

```
┌────────────────────────────────────────────────────────────────────────┐
│                   WITHDRAWAL FLOW: ÉTR → BTC                           │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  USER PERSPECTIVE:                                                     │
│  ════════════════                                                      │
│                                                                         │
│  Step 1: User initiates withdrawal                                     │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  [Wallet UI]                                                      │  │
│  │                                                                   │  │
│  │  Select Currency:  [BTC ▼]                                       │  │
│  │  ÉTR Amount:       [25,000 ÉTR]                                  │  │
│  │  You will receive: ~0.998 BTC                                    │  │
│  │                                                                   │  │
│  │  BTC Address:      [1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa]         │  │
│  │                                                                   │  │
│  │  [Convert to BTC]  ← Single button                              │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Step 2: Transaction progress                                          │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Converting 25,000 ÉTR to BTC...                                  │  │
│  │                                                                   │  │
│  │  [████████████░░] 80%                                            │  │
│  │                                                                   │  │
│  │  Status: Awaiting Bitcoin confirmation                           │  │
│  │  Estimated time: ~10 minutes                                     │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  Step 3: Completion                                                    │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  ✓ Successfully sent 0.998 BTC                                    │  │
│  │                                                                   │  │
│  │  Transaction Details:                                            │  │
│  │  - Converted: 25,000 ÉTR                                         │  │
│  │  - Sent:      0.998 BTC                                          │  │
│  │  - To:        1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa                │  │
│  │  - Fee:       0.002 BTC (bridge + network)                       │  │
│  │                                                                   │  │
│  │  [View on Bitcoin Explorer]                                      │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  SYSTEM PERSPECTIVE:                                                   │
│  ═══════════════════                                                   │
│                                                                         │
│  Transaction 1: ÉTR → wBTC Swap                                         │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Call: IntentRouter.convertFromETR(BTC, 25000 ÉTR, btcAddress)  │  │
│  │                                                                   │  │
│  │  Internal execution:                                             │  │
│  │  1. Transfer ÉTR from user to IntentRouter                       │  │
│  │     ETR.transferFrom(user, IntentRouter, 25000 ÉTR)             │  │
│  │                                                                   │  │
│  │  2. Swap ÉTR → wBTC                                               │  │
│  │     PrimeSwapRouter.swapExactTokensForTokens()                   │  │
│  │     Path: [ÉTR, wBTC]                                             │  │
│  │     AmountIn: 25,000 ÉTR                                         │  │
│  │     MinOut: 0.99 wBTC (1% slippage)                              │  │
│  │     To: IntentRouter ← Receives wBTC (NOT user)                 │  │
│  │                                                                   │  │
│  │  3. wBTC received: 1.0 wBTC                                      │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Transaction 2: Burn wBTC + Initiate Bridge                            │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Call: EtridBridge.burnTokens()                                  │  │
│  │  Amount: 1.0 wBTC                                                │  │
│  │  Destination: Bitcoin address (Base58)                           │  │
│  │                                                                   │  │
│  │  wBTC burned from IntentRouter                                   │  │
│  │  Bridge request created with ID: 0xabc123...                     │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Transaction 3: Directors sign & unlock BTC                            │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Directors provide 5-of-9 attestation                            │  │
│  │  Bitcoin bridge unlocks 1.0 BTC                                  │  │
│  │  BTC sent to user's Bitcoin address                              │  │
│  │  User receives: 0.998 BTC (after fees)                           │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
│  ⚠️ CRITICAL: User never holds wBTC                                    │
│  wBTC held temporarily by IntentRouter, immediately burned              │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

### Flow 3: Error Handling

```
┌────────────────────────────────────────────────────────────────────────┐
│                    ERROR HANDLING & FALLBACK FLOWS                      │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Scenario 1: Auto-swap fails (insufficient liquidity)                  │
│  ══════════════════════════════════════════════════════                │
│                                                                         │
│  System Detection:                                                     │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  AutoSwapExecutor attempts swap                                  │  │
│  │  PrimeSwapRouter reverts: "INSUFFICIENT_OUTPUT_AMOUNT"           │  │
│  │  Catch error, trigger fallback mode                              │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Fallback Execution:                                                   │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  1. Transfer wBTC to user address                                │  │
│  │     wBTC.transfer(user, amount)                                  │  │
│  │                                                                   │  │
│  │  2. Emit event: SwapFailed(user, wBTC, amount, reason)          │  │
│  │                                                                   │  │
│  │  3. Update UI immediately                                        │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  User Notification:                                                    │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  ⚠️ Automatic conversion temporarily unavailable                  │  │
│  │                                                                   │  │
│  │  Your 0.5 wBTC has been credited to your wallet.                │  │
│  │  You can:                                                        │  │
│  │                                                                   │  │
│  │  Option 1: [Wait & Retry Auto-Swap] (when liquidity returns)    │  │
│  │  Option 2: [Manually Swap on PrimeSwap]                         │  │
│  │  Option 3: [Bridge Back to Bitcoin]                             │  │
│  │                                                                   │  │
│  │  We'll notify you when auto-swap is available again.            │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Scenario 2: Slippage exceeds user tolerance                           │
│  ═══════════════════════════════════════                               │
│                                                                         │
│  System Detection:                                                     │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  Expected output: 12,500 ÉTR                                      │  │
│  │  Actual output: 12,250 ÉTR (2% slippage)                         │  │
│  │  User tolerance: 1% max                                          │  │
│  │  Transaction reverts before execution                            │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  User Notification:                                                    │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  ⚠️ Price moved unfavorably                                       │  │
│  │                                                                   │  │
│  │  Expected: 12,500 ÉTR                                             │  │
│  │  Current:  12,250 ÉTR (2% worse)                                 │  │
│  │                                                                   │  │
│  │  [Retry with Updated Price]                                      │  │
│  │  [Increase Slippage Tolerance to 2%]                             │  │
│  │  [Cancel Transaction]                                            │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Scenario 3: Bridge succeeds but swap contract paused                  │
│  ═══════════════════════════════════════════════════════              │
│                                                                         │
│  System Detection:                                                     │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  wBTC minted successfully                                        │  │
│  │  AutoSwapExecutor calls PrimeSwapRouter                          │  │
│  │  Router is paused (emergency maintenance)                        │  │
│  │  Transaction reverts                                             │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  Automatic Retry Logic:                                                │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  1. Store pending swap in PendingSwaps storage                  │  │
│  │  2. Transfer wBTC to user (fallback)                             │  │
│  │  3. Monitor for unpause event                                    │  │
│  │  4. Offer one-click re-swap when available                       │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                         ↓                                              │
│  User Notification:                                                    │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │  ⚠️ DEX temporarily under maintenance                             │  │
│  │                                                                   │  │
│  │  Your 0.5 wBTC is safe in your wallet.                          │  │
│  │  Estimated maintenance completion: 30 minutes                    │  │
│  │                                                                   │  │
│  │  [Enable Auto-Retry] ← Automatically swap when DEX resumes      │  │
│  │                                                                   │  │
│  │  We'll complete your swap as soon as maintenance ends.           │  │
│  └──────────────────────────────────────────────────────────────────┘  │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Smart Contract Architecture

### Contract 1: IntentRouter

**Purpose:** Entry point for all user conversions, orchestrates multi-step operations

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title IntentRouter
 * @notice User-facing contract that abstracts wrapped tokens
 * @dev Coordinates bridge and swap operations atomically
 */
contract IntentRouter {
    // ============ State Variables ============

    IEtridBridge public immutable bridge;
    IPrimeSwapRouter public immutable swapRouter;
    IAutoSwapExecutor public immutable autoSwap;

    // Supported external currencies
    enum Currency { BTC, ETH, SOL, BNB, XRP, TRX, MATIC, DOGE, ADA, LINK, XLM }

    // Mapping: Currency → Wrapped token address
    mapping(Currency => address) public wrappedTokens;

    // Pending swaps (if auto-swap fails)
    struct PendingSwap {
        address user;
        address wrappedToken;
        uint256 amount;
        uint256 minETROut;
        uint256 timestamp;
        bool completed;
    }
    mapping(bytes32 => PendingSwap) public pendingSwaps;

    // User preferences
    mapping(address => uint16) public userSlippageTolerance; // basis points (100 = 1%)

    // ============ Events ============

    event ConversionInitiated(
        address indexed user,
        Currency currency,
        uint256 amount,
        bytes32 requestId
    );

    event ConversionCompleted(
        address indexed user,
        Currency currency,
        uint256 amountIn,
        uint256 etrOut,
        bytes32 requestId
    );

    event AutoSwapFailed(
        address indexed user,
        address wrappedToken,
        uint256 amount,
        string reason
    );

    event PendingSwapCreated(
        bytes32 indexed swapId,
        address indexed user,
        address wrappedToken,
        uint256 amount
    );

    // ============ User-Facing Functions ============

    /**
     * @notice Convert external currency to ÉTR (primary user function)
     * @dev Handles bridge attestation → mint wToken → swap → deliver ÉTR
     * @param requestId Bridge request ID from external chain
     * @param currency External currency type
     * @param amount Amount of external currency
     * @param minETROut Minimum ÉTR to receive (slippage protection)
     * @param signatures Director attestations (5-of-9)
     */
    function convertToETR(
        bytes32 requestId,
        Currency currency,
        uint256 amount,
        uint256 minETROut,
        bytes[] calldata signatures
    ) external returns (uint256 etrReceived) {
        address user = msg.sender;
        address wrappedToken = wrappedTokens[currency];

        require(wrappedToken != address(0), "Unsupported currency");

        emit ConversionInitiated(user, currency, amount, requestId);

        // Step 1: Mint wrapped token to THIS contract (not user)
        bridge.mintTokens(
            requestId,
            /* sourceDomain */ uint32(currency),
            address(this), // ← Wrapped token goes to router
            wrappedToken,
            amount,
            signatures
        );

        // Step 2: Attempt automatic swap
        try autoSwap.executeSwap(
            wrappedToken,
            amount,
            minETROut,
            user // ← ÉTR delivered directly to user
        ) returns (uint256 etrOut) {
            etrReceived = etrOut;

            emit ConversionCompleted(user, currency, amount, etrOut, requestId);

        } catch Error(string memory reason) {
            // Auto-swap failed, fallback to manual mode
            _handleSwapFailure(user, wrappedToken, amount, minETROut, reason);

            revert("Auto-swap unavailable. Wrapped tokens credited to wallet.");
        }

        return etrReceived;
    }

    /**
     * @notice Convert ÉTR to external currency
     * @dev Swaps ÉTR → wToken → burns wToken → bridge unlocks native currency
     * @param currency Target external currency
     * @param etrAmount Amount of ÉTR to convert
     * @param destinationAddress Address on external chain (bytes32 for compatibility)
     * @param minAmountOut Minimum external currency to receive
     */
    function convertFromETR(
        Currency currency,
        uint256 etrAmount,
        bytes32 destinationAddress,
        uint256 minAmountOut
    ) external returns (bytes32 bridgeRequestId) {
        address user = msg.sender;
        address wrappedToken = wrappedTokens[currency];

        require(wrappedToken != address(0), "Unsupported currency");

        // Step 1: Transfer ÉTR from user to this contract
        IERC20(ETR_TOKEN).transferFrom(user, address(this), etrAmount);

        // Step 2: Swap ÉTR → wToken (to THIS contract)
        IERC20(ETR_TOKEN).approve(address(swapRouter), etrAmount);

        address[] memory path = new address[](2);
        path[0] = ETR_TOKEN;
        path[1] = wrappedToken;

        uint256[] memory amounts = swapRouter.swapExactTokensForTokens(
            etrAmount,
            minAmountOut,
            path,
            address(this), // ← wToken stays in router
            block.timestamp + 300
        );

        uint256 wrappedAmount = amounts[1];

        // Step 3: Burn wToken and initiate bridge transfer
        IERC20(wrappedToken).approve(address(bridge), wrappedAmount);

        bridgeRequestId = bridge.burnTokens(
            wrappedToken,
            wrappedAmount,
            destinationAddress,
            uint32(currency) // destination domain
        );

        emit ConversionCompleted(user, currency, etrAmount, wrappedAmount, bridgeRequestId);

        return bridgeRequestId;
    }

    /**
     * @notice Get quote for conversion (read-only)
     * @dev Shows user expected output before transaction
     */
    function getConversionQuote(
        Currency currency,
        uint256 amount,
        bool toETR // true = currency→ÉTR, false = ÉTR→currency
    ) external view returns (
        uint256 expectedOutput,
        uint256 minimumOutput, // with slippage
        uint256 priceImpact,
        uint256 fee
    ) {
        address wrappedToken = wrappedTokens[currency];
        require(wrappedToken != address(0), "Unsupported currency");

        address tokenIn = toETR ? wrappedToken : ETR_TOKEN;
        address tokenOut = toETR ? ETR_TOKEN : wrappedToken;

        // Get reserves and calculate output
        (uint256 reserveIn, uint256 reserveOut) = _getReserves(tokenIn, tokenOut);

        expectedOutput = swapRouter.getAmountOut(amount, reserveIn, reserveOut);

        uint16 slippage = userSlippageTolerance[msg.sender];
        if (slippage == 0) slippage = 100; // default 1%

        minimumOutput = (expectedOutput * (10000 - slippage)) / 10000;

        priceImpact = ((amount * 10000) / reserveIn); // basis points
        fee = (amount * 30) / 10000; // 0.3%

        return (expectedOutput, minimumOutput, priceImpact, fee);
    }

    /**
     * @notice Retry pending swap manually
     * @dev If auto-swap failed, user can retry when liquidity returns
     */
    function retryPendingSwap(bytes32 swapId) external {
        PendingSwap storage swap = pendingSwaps[swapId];

        require(swap.user == msg.sender, "Not your swap");
        require(!swap.completed, "Already completed");
        require(block.timestamp <= swap.timestamp + 7 days, "Swap expired");

        // Transfer wToken from user back to router
        IERC20(swap.wrappedToken).transferFrom(msg.sender, address(this), swap.amount);

        // Retry auto-swap
        uint256 etrOut = autoSwap.executeSwap(
            swap.wrappedToken,
            swap.amount,
            swap.minETROut,
            msg.sender
        );

        swap.completed = true;

        emit ConversionCompleted(
            msg.sender,
            _currencyFromToken(swap.wrappedToken),
            swap.amount,
            etrOut,
            swapId
        );
    }

    // ============ Internal Functions ============

    function _handleSwapFailure(
        address user,
        address wrappedToken,
        uint256 amount,
        uint256 minETROut,
        string memory reason
    ) internal {
        // Create pending swap record
        bytes32 swapId = keccak256(abi.encodePacked(
            user,
            wrappedToken,
            amount,
            block.timestamp
        ));

        pendingSwaps[swapId] = PendingSwap({
            user: user,
            wrappedToken: wrappedToken,
            amount: amount,
            minETROut: minETROut,
            timestamp: block.timestamp,
            completed: false
        });

        // Transfer wToken to user (fallback)
        IERC20(wrappedToken).transfer(user, amount);

        emit AutoSwapFailed(user, wrappedToken, amount, reason);
        emit PendingSwapCreated(swapId, user, wrappedToken, amount);
    }

    // ============ Admin Functions ============

    function setWrappedToken(Currency currency, address tokenAddress) external onlyOwner {
        wrappedTokens[currency] = tokenAddress;
    }

    function setSlippageTolerance(uint16 bps) external {
        require(bps <= 1000, "Max 10% slippage"); // 1000 bps = 10%
        userSlippageTolerance[msg.sender] = bps;
    }
}
```

### Contract 2: AutoSwapExecutor

**Purpose:** Executes wrapped token → ÉTR swaps atomically

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title AutoSwapExecutor
 * @notice Automatically swaps wrapped tokens to ÉTR
 * @dev Called by IntentRouter, never by users directly
 */
contract AutoSwapExecutor {
    IPrimeSwapRouter public immutable swapRouter;
    address public immutable ETR_TOKEN;
    address public immutable intentRouter;

    // Emergency circuit breaker
    bool public paused;

    // Slippage protection: max price impact allowed
    uint256 public maxPriceImpact = 500; // 5% in basis points

    event SwapExecuted(
        address indexed wrappedToken,
        uint256 amountIn,
        uint256 amountOut,
        address recipient
    );

    event SwapReverted(
        address indexed wrappedToken,
        uint256 amountIn,
        string reason
    );

    modifier onlyIntentRouter() {
        require(msg.sender == intentRouter, "Only IntentRouter");
        _;
    }

    modifier whenNotPaused() {
        require(!paused, "Executor paused");
        _;
    }

    /**
     * @notice Execute swap: wrappedToken → ÉTR
     * @dev Called atomically after wrapped token mint
     * @param wrappedToken Address of wrapped token (wBTC, wETH, etc.)
     * @param amountIn Amount of wrapped token to swap
     * @param minETROut Minimum ÉTR to receive (slippage protection)
     * @param recipient Address to receive ÉTR (end user)
     * @return etrOut Amount of ÉTR sent to recipient
     */
    function executeSwap(
        address wrappedToken,
        uint256 amountIn,
        uint256 minETROut,
        address recipient
    ) external onlyIntentRouter whenNotPaused returns (uint256 etrOut) {
        require(recipient != address(0), "Invalid recipient");
        require(amountIn > 0, "Zero amount");

        // Approve router to spend wrapped token
        IERC20(wrappedToken).approve(address(swapRouter), amountIn);

        // Build swap path: wrappedToken → ÉTR
        address[] memory path = new address[](2);
        path[0] = wrappedToken;
        path[1] = ETR_TOKEN;

        // Check price impact
        uint256[] memory amountsOut = swapRouter.getAmountsOut(amountIn, path);
        uint256 expectedOut = amountsOut[1];

        _validatePriceImpact(wrappedToken, amountIn, expectedOut);

        // Execute swap via PrimeSwap
        uint256[] memory amounts = swapRouter.swapExactTokensForTokens(
            amountIn,
            minETROut,
            path,
            recipient, // ← ÉTR goes directly to user
            block.timestamp + 300 // 5 min deadline
        );

        etrOut = amounts[1];

        emit SwapExecuted(wrappedToken, amountIn, etrOut, recipient);

        return etrOut;
    }

    /**
     * @notice Validate price impact is within acceptable range
     * @dev Reverts if price impact too high (potential manipulation)
     */
    function _validatePriceImpact(
        address wrappedToken,
        uint256 amountIn,
        uint256 expectedOut
    ) internal view {
        // Get pool reserves
        address pair = IPrimeSwapFactory(swapRouter.factory()).getPair(
            wrappedToken,
            ETR_TOKEN
        );

        (uint256 reserveWrapped, uint256 reserveETR) = _getReserves(
            pair,
            wrappedToken,
            ETR_TOKEN
        );

        // Calculate price impact: (amountIn / reserveIn) * 10000
        uint256 priceImpact = (amountIn * 10000) / reserveWrapped;

        require(priceImpact <= maxPriceImpact, "Price impact too high");
    }

    /**
     * @notice Emergency pause (admin only)
     */
    function pause() external onlyOwner {
        paused = true;
    }

    function unpause() external onlyOwner {
        paused = false;
    }

    function setMaxPriceImpact(uint256 bps) external onlyOwner {
        require(bps <= 2000, "Max 20%"); // 2000 bps = 20%
        maxPriceImpact = bps;
    }
}
```

### Contract 3: WrappedTokenRegistry

**Purpose:** Manages wrapped token metadata and visibility settings

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title WrappedTokenRegistry
 * @notice Manages wrapped token information and UI visibility
 * @dev Tells wallets which tokens to hide from user view
 */
contract WrappedTokenRegistry {
    struct TokenInfo {
        address tokenAddress;
        string symbol; // e.g., "wBTC"
        string originalSymbol; // e.g., "BTC"
        string originalChain; // e.g., "Bitcoin"
        bool isAbstracted; // true = hide from UI
        address swapRouter; // PrimeSwap router for this token
    }

    mapping(address => TokenInfo) public wrappedTokens;
    address[] public allWrappedTokens;

    event TokenRegistered(
        address indexed tokenAddress,
        string symbol,
        string originalSymbol,
        bool isAbstracted
    );

    /**
     * @notice Register wrapped token with abstraction flag
     * @dev isAbstracted=true tells wallets to hide from balance view
     */
    function registerToken(
        address tokenAddress,
        string memory symbol,
        string memory originalSymbol,
        string memory originalChain,
        bool isAbstracted,
        address swapRouter
    ) external onlyOwner {
        require(tokenAddress != address(0), "Invalid address");

        wrappedTokens[tokenAddress] = TokenInfo({
            tokenAddress: tokenAddress,
            symbol: symbol,
            originalSymbol: originalSymbol,
            originalChain: originalChain,
            isAbstracted: isAbstracted,
            swapRouter: swapRouter
        });

        allWrappedTokens.push(tokenAddress);

        emit TokenRegistered(tokenAddress, symbol, originalSymbol, isAbstracted);
    }

    /**
     * @notice Check if token should be hidden from UI
     * @dev Wallets call this to determine visibility
     */
    function isTokenAbstracted(address token) external view returns (bool) {
        return wrappedTokens[token].isAbstracted;
    }

    /**
     * @notice Get all abstracted tokens (for wallet integration)
     * @dev Wallet filters these tokens from balance displays
     */
    function getAbstractedTokens() external view returns (address[] memory) {
        uint256 count = 0;

        // Count abstracted tokens
        for (uint256 i = 0; i < allWrappedTokens.length; i++) {
            if (wrappedTokens[allWrappedTokens[i]].isAbstracted) {
                count++;
            }
        }

        // Build array
        address[] memory abstracted = new address[](count);
        uint256 index = 0;

        for (uint256 i = 0; i < allWrappedTokens.length; i++) {
            if (wrappedTokens[allWrappedTokens[i]].isAbstracted) {
                abstracted[index] = allWrappedTokens[i];
                index++;
            }
        }

        return abstracted;
    }

    /**
     * @notice Get user-facing display name
     * @dev "wBTC" → "BTC" for UI purposes
     */
    function getDisplaySymbol(address token) external view returns (string memory) {
        TokenInfo memory info = wrappedTokens[token];
        return info.isAbstracted ? info.originalSymbol : info.symbol;
    }
}
```

### Contract Interaction Flow

```
┌────────────────────────────────────────────────────────────────────────┐
│                  SMART CONTRACT CALL FLOW                               │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  User Action: "Convert BTC to ÉTR"                                      │
│  ══════════════════════════════                                        │
│                                                                         │
│  1. USER → IntentRouter.convertToETR()                                 │
│     ├─ Parameters:                                                     │
│     │  ├─ requestId: 0xabc... (from Bitcoin bridge)                   │
│     │  ├─ currency: Currency.BTC                                       │
│     │  ├─ amount: 0.5 * 10^8 (50,000,000 sats)                        │
│     │  ├─ minETROut: 12,375 * 10^18 (1% slippage)                     │
│     │  └─ signatures: [sig1, sig2, sig3, sig4, sig5] (5-of-9)         │
│     │                                                                  │
│     └─> 2. IntentRouter → EtridBridge.mintTokens()                    │
│         ├─ Mints 0.5 wBTC to IntentRouter address                     │
│         └─ wBTC balance of IntentRouter: 0.5 wBTC                     │
│             │                                                          │
│             └─> 3. IntentRouter → AutoSwapExecutor.executeSwap()      │
│                 ├─ Parameters:                                        │
│                 │  ├─ wrappedToken: wBTC_ADDRESS                      │
│                 │  ├─ amountIn: 0.5 wBTC                              │
│                 │  ├─ minETROut: 12,375 ÉTR                           │
│                 │  └─ recipient: USER_ADDRESS                         │
│                 │                                                     │
│                 └─> 4. AutoSwapExecutor → PrimeSwapRouter             │
│                     ├─ Approves wBTC spending                        │
│                     ├─ Validates price impact                        │
│                     └─> 5. PrimeSwapRouter.swapExactTokensForTokens()│
│                         ├─ Path: [wBTC, ÉTR]                          │
│                         ├─ AmountIn: 0.5 wBTC                         │
│                         ├─ AmountOutMin: 12,375 ÉTR                   │
│                         ├─ To: USER_ADDRESS ← Direct delivery        │
│                         └─ Deadline: block.timestamp + 300           │
│                             │                                         │
│                             └─> 6. VirtualReserveAMM._swap()          │
│                                 ├─ Burns 0.5 wBTC from IntentRouter  │
│                                 ├─ Calculates output: 12,487 ÉTR     │
│                                 ├─ Transfers 12,487 ÉTR to USER      │
│                                 └─ Updates reserves                  │
│                                                                       │
│  Result:                                                              │
│  ├─ User receives: 12,487 ÉTR                                         │
│  ├─ wBTC balance of IntentRouter: 0 (fully swapped)                  │
│  ├─ wBTC never appeared in user wallet                                │
│  └─ Total transaction time: ~5 seconds (single block)                 │
│                                                                       │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Security Considerations

### 1. Atomicity & Reentrancy

**Risk:** Multi-step operations could be exploited during intermediate states

**Mitigation:**
```solidity
// All public functions use nonReentrant modifier
function convertToETR(...) external nonReentrant {
    // Step 1: Mint wToken
    bridge.mintTokens(...);

    // Step 2: Swap (within same transaction)
    autoSwap.executeSwap(...);

    // If step 2 fails, entire transaction reverts
    // wToken is never left in intermediate state
}
```

**Additional protections:**
- Checks-Effects-Interactions pattern
- State updates before external calls
- No external calls during critical sections

### 2. Price Manipulation

**Risk:** Attacker manipulates AMM price during mint → swap window

**Mitigation:**
```solidity
// 1. Price impact limits
uint256 priceImpact = (amountIn * 10000) / reserveIn;
require(priceImpact <= maxPriceImpact, "Price impact too high");

// 2. TWAP oracle cross-check
uint256 spotPrice = getSpotPrice();
uint256 oraclePrice = getTWAPPrice();
require(
    spotPrice >= oraclePrice * 98 / 100 &&
    spotPrice <= oraclePrice * 102 / 100,
    "Price deviation too high"
);

// 3. Rate limiting
require(
    swapVolume[block.number] + amountIn <= maxVolumePerBlock,
    "Volume limit exceeded"
);
```

### 3. Front-Running

**Risk:** MEV bots sandwich attack user swaps

**Mitigation:**
- **Private mempool:** Use Flashbots/Eden for transaction privacy
- **Commit-reveal:** User commits hash, reveals after delay
- **Batch execution:** Aggregate multiple swaps, execute at same price
- **Slippage protection:** User-defined minAmountOut enforced

```solidity
// Commit phase
function commitSwap(bytes32 commitHash) external {
    commits[msg.sender] = Commit({
        hash: commitHash,
        timestamp: block.timestamp
    });
}

// Reveal phase (after 1 block delay)
function revealSwap(
    uint256 amount,
    uint256 nonce,
    bytes32 salt
) external {
    require(
        block.timestamp >= commits[msg.sender].timestamp + 12,
        "Too soon"
    );

    bytes32 hash = keccak256(abi.encodePacked(amount, nonce, salt));
    require(hash == commits[msg.sender].hash, "Invalid reveal");

    // Execute swap at revealed parameters
    _executeSwap(amount);
}
```

### 4. Liquidity Availability

**Risk:** Insufficient liquidity causes auto-swap to fail

**Mitigation:**
```solidity
// Pre-flight check before minting wToken
function checkLiquidityAvailable(
    address wrappedToken,
    uint256 amountIn,
    uint256 minOut
) public view returns (bool sufficient) {
    (uint256 reserveWrapped, uint256 reserveETR) = getReserves(wrappedToken);

    // Ensure pool can handle swap
    uint256 expectedOut = getAmountOut(amountIn, reserveWrapped, reserveETR);

    return expectedOut >= minOut && reserveETR >= expectedOut;
}

// IntentRouter checks before minting
function convertToETR(...) external {
    // Check liquidity BEFORE minting
    require(
        checkLiquidityAvailable(wrappedToken, amount, minETROut),
        "Insufficient liquidity - try smaller amount"
    );

    // Proceed with mint + swap
    bridge.mintTokens(...);
    autoSwap.executeSwap(...);
}
```

### 5. Access Control

**Risk:** Unauthorized contracts mint wrapped tokens directly to users

**Mitigation:**
```solidity
// EtridBridge: Only authorized recipients
mapping(address => bool) public authorizedRecipients;

function mintTokens(
    bytes32 requestId,
    uint32 sourceDomain,
    address recipient,
    address wrappedToken,
    uint256 amount,
    bytes[] calldata signatures
) external {
    // CRITICAL: Only mint to authorized contracts
    require(
        authorizedRecipients[recipient],
        "Recipient not authorized"
    );

    // IntentRouter and AutoSwapExecutor are authorized
    // User addresses are NOT authorized

    // ... rest of minting logic
}

// Admin function
function authorizeRecipient(address recipient) external onlyOwner {
    authorizedRecipients[recipient] = true;
}
```

### 6. Emergency Pause Mechanism

**Risk:** Critical bug requires immediate halt of operations

**Mitigation:**
```solidity
// Global pause (Pausable pattern)
contract IntentRouter is Pausable {
    function convertToETR(...) external whenNotPaused {
        // ... execution
    }

    // Multi-sig admin can pause
    function emergencyPause() external onlyEmergencyRole {
        _pause();
        emit EmergencyPause(msg.sender, block.timestamp);
    }
}

// Partial pause (granular control)
mapping(address => bool) public tokenPaused;

function pauseToken(address token) external onlyOwner {
    tokenPaused[token] = true;
}

function convertToETR(...) external {
    require(!tokenPaused[wrappedToken], "Token paused");
    // ... execution
}
```

### 7. Audit Trail & Monitoring

**Risk:** Silent failures or exploits go undetected

**Mitigation:**
```solidity
// Comprehensive event logging
event SwapExecuted(
    address indexed user,
    address indexed wrappedToken,
    uint256 amountIn,
    uint256 etrOut,
    uint256 priceImpact,
    uint256 timestamp
);

event AnomalyDetected(
    string anomalyType,
    address indexed token,
    uint256 value,
    uint256 threshold
);

// Real-time monitoring checks
function _checkAnomalies(address token, uint256 amount) internal {
    // Check 1: Unusual volume
    if (amount > averageVolume[token] * 10) {
        emit AnomalyDetected("HIGH_VOLUME", token, amount, averageVolume[token]);
    }

    // Check 2: Rapid price movement
    uint256 priceChange = _calculatePriceChange(token);
    if (priceChange > 1000) { // 10%
        emit AnomalyDetected("PRICE_SPIKE", token, priceChange, 1000);
    }

    // Check 3: Reserve depletion
    uint256 reserveRatio = _getReserveRatio(token);
    if (reserveRatio < 2000) { // 20%
        emit AnomalyDetected("LOW_RESERVES", token, reserveRatio, 2000);
    }
}
```

---

## Implementation Plan

### Phase 1: Smart Contract Development (Weeks 1-3)

**Week 1: Core Contracts**
- [ ] Develop `IntentRouter.sol`
  - User-facing conversion functions
  - Quote calculation
  - Fallback handling
- [ ] Develop `AutoSwapExecutor.sol`
  - Atomic swap execution
  - Price impact validation
  - Emergency pause
- [ ] Develop `WrappedTokenRegistry.sol`
  - Token metadata storage
  - Abstraction flags
  - Wallet integration helpers

**Week 2: Integration Layer**
- [ ] Extend `EtridBridge.sol`
  - Add authorized recipient checks
  - Integrate with IntentRouter
  - Add direct-to-router minting
- [ ] Extend `PrimeSwapRouter.sol`
  - Add batch swap support
  - Implement TWAP oracle checks
  - Add MEV protection

**Week 3: Testing & Security**
- [ ] Unit tests for all contracts (100% coverage)
- [ ] Integration tests for full flow
- [ ] Fuzzing tests for edge cases
- [ ] Gas optimization
- [ ] External audit preparation

### Phase 2: Wallet Integration (Weeks 4-5)

**Week 4: Backend Integration**
- [ ] Update wallet backend to query `WrappedTokenRegistry`
- [ ] Implement token filtering (hide abstracted tokens)
- [ ] Add conversion quote API endpoint
- [ ] Implement real-time transaction tracking

**Week 5: UI/UX Updates**
- [ ] Design unified conversion interface
  - Single "Convert" button
  - Currency selector dropdown
  - Real-time quote updates
- [ ] Implement progress tracking
  - Multi-step transaction visualization
  - Estimated time remaining
- [ ] Add fallback UI for manual swaps
  - Show pending swaps
  - One-click retry button

### Phase 3: Bridge Relayer Updates (Week 6)

**Week 6: Relayer Service**
- [ ] Update relayer to call `IntentRouter` instead of direct minting
- [ ] Add liquidity pre-checks before attestation
- [ ] Implement automatic retry logic
- [ ] Add monitoring for failed auto-swaps

### Phase 4: Testing & Deployment (Weeks 7-8)

**Week 7: Testnet Deployment**
- [ ] Deploy contracts to testnet
- [ ] Seed liquidity pools
- [ ] Test full user journey
  - BTC → ÉTR
  - ÉTR → BTC
  - All 11 supported currencies
- [ ] Load testing (simulate 1000 concurrent swaps)

**Week 8: Mainnet Deployment**
- [ ] Security audit review
- [ ] Multi-sig governance approval
- [ ] Mainnet contract deployment
- [ ] Initialize liquidity pools
- [ ] Enable IntentRouter
- [ ] Monitor for 72 hours before full rollout

### Phase 5: Monitoring & Optimization (Ongoing)

**Post-Launch (Weeks 9+)**
- [ ] Real-time monitoring dashboard
  - Swap success rate
  - Average execution time
  - Price impact distribution
  - Failed swap rate
- [ ] Weekly performance reports
- [ ] Continuous gas optimization
- [ ] User feedback collection & iteration

---

## Edge Cases & Failure Modes

### Edge Case 1: Partial Liquidity

**Scenario:** User wants to convert 10 BTC, but pool only has 8 BTC worth of liquidity

**Detection:**
```solidity
function checkLiquidityAvailable(uint256 amount) internal view returns (bool) {
    uint256 maxSwappable = getMaxSwappableAmount(wrappedToken);
    return amount <= maxSwappable;
}
```

**Resolution:**
1. **Pre-flight Check:** UI shows max convertible amount
   ```
   "Maximum convertible: 8 BTC (limited by liquidity)"
   [Convert 8 BTC] or [Enter Custom Amount]
   ```

2. **Batch Splitting:** Break large swap into multiple transactions
   ```solidity
   function convertToETRBatched(
       uint256 totalAmount,
       uint256 batchSize
   ) external {
       uint256 remaining = totalAmount;

       while (remaining > 0) {
           uint256 batchAmount = min(remaining, batchSize);
           _convertToETR(batchAmount);
           remaining -= batchAmount;
       }
   }
   ```

### Edge Case 2: Network Congestion

**Scenario:** Ethereum gas fees spike during transaction, causing timeout

**Detection:**
```solidity
// Set reasonable deadline
uint256 deadline = block.timestamp + 5 minutes;

// If deadline exceeded, transaction reverts
require(block.timestamp <= deadline, "Transaction expired");
```

**Resolution:**
1. **Dynamic Gas Pricing:** Wallet adjusts gas price automatically
2. **Transaction Queuing:** User can queue transaction for later execution
3. **User Notification:**
   ```
   "Network congestion detected.
    Estimated gas fee: $50
    Wait for lower fees? [Yes] [No, send anyway]"
   ```

### Edge Case 3: Oracle Failure

**Scenario:** Price oracle goes offline during transaction

**Detection:**
```solidity
function getPriceWithFallback() internal view returns (uint256) {
    try primaryOracle.getPrice() returns (uint256 price) {
        return price;
    } catch {
        // Fallback to secondary oracle
        return secondaryOracle.getPrice();
    }
}
```

**Resolution:**
1. **Multiple Oracle Sources:**
   - Primary: Chainlink
   - Secondary: Band Protocol
   - Tertiary: On-chain TWAP
2. **Stale Price Check:**
   ```solidity
   require(
       block.timestamp - lastOracleUpdate <= 1 hours,
       "Oracle price stale"
   );
   ```
3. **Pure AMM Fallback:** Use spot price if all oracles fail

### Edge Case 4: Bridge Director Unavailability

**Scenario:** Only 4 of 9 Directors are online (need 5 for attestation)

**Detection:**
```
Relayer monitors Director availability
If < 5 online → alert user before transaction
```

**Resolution:**
1. **User Warning:**
   ```
   "Bridge attestation may be delayed (4/5 Directors online).
    Continue anyway? Transaction will complete when 5th Director comes online."
   ```
2. **Automatic Retry:** Relayer retries attestation every 5 minutes
3. **Timeout Refund:** If not attested within 24 hours, initiate refund

### Edge Case 5: Wrapped Token Contract Upgrade

**Scenario:** wBTC contract is upgraded mid-transaction

**Detection:**
```solidity
// Store expected wBTC address at transaction start
bytes32 expectedCodeHash = wrappedToken.codehash;

// Verify code hasn't changed
require(
    wrappedToken.codehash == expectedCodeHash,
    "Contract upgraded during transaction"
);
```

**Resolution:**
1. **Version Pinning:** IntentRouter stores approved wBTC versions
2. **Upgrade Lockout:** Prevent swaps during upgrade window
3. **Automatic Migration:** If upgrade detected, migrate to new contract

### Edge Case 6: Slippage Exceeds Limit Mid-Transaction

**Scenario:** Price moves between quote and execution

**Detection:**
```solidity
uint256 expectedOut = getQuote(amountIn);
uint256 actualOut = executeSwap(amountIn);

require(
    actualOut >= expectedOut * (10000 - slippage) / 10000,
    "Slippage exceeded"
);
```

**Resolution:**
1. **Pre-Transaction Price Lock:** Reserve price for 30 seconds
2. **User Confirmation:**
   ```
   "Price changed!
    Previous: 12,500 ÉTR
    Current:  12,250 ÉTR (2% worse)
    Continue? [Yes] [No, cancel]"
   ```
3. **Automatic Retry:** Retry with updated price if user consents

### Edge Case 7: User Wallet Disconnect During Transaction

**Scenario:** Mobile wallet loses connection after transaction submitted

**Detection:**
```
Wallet monitors connection status
If disconnected → cache transaction details locally
```

**Resolution:**
1. **Transaction Persistence:** Store pending tx in local storage
2. **Automatic Reconnect:** Resume monitoring when connection restored
3. **Push Notifications:** Alert user when transaction completes
4. **Status Recovery:**
   ```
   "Checking status of your BTC → ÉTR conversion...
    [Transaction still pending]
    [View on Explorer]"
   ```

### Edge Case 8: Double-Spend Attack on Bridge

**Scenario:** Attacker tries to claim same Bitcoin lock twice

**Detection:**
```solidity
mapping(bytes32 => bool) public processedRequests;

function mintTokens(bytes32 requestId, ...) external {
    require(!processedRequests[requestId], "Already processed");
    processedRequests[requestId] = true;
    // ... mint logic
}
```

**Resolution:**
1. **Nonce System:** Each bridge request has unique nonce
2. **Idempotency:** Same requestId can't be processed twice
3. **Director Verification:** All 5 Directors verify unique request
4. **On-Chain Verification:** Smart contract validates requestId uniqueness

### Edge Case 9: Gas Price Spike During Multi-Step Transaction

**Scenario:** Gas price jumps 10x mid-transaction, user can't afford continuation

**Detection:**
```solidity
// Estimate total gas needed upfront
uint256 totalGasNeeded = estimateTotalGas();
uint256 totalCost = totalGasNeeded * tx.gasprice;

require(msg.value >= totalCost, "Insufficient gas");
```

**Resolution:**
1. **Gas Reservation:** User pays estimated gas upfront
2. **Refund Excess:** Return unused gas at end
3. **Subsidization:** Protocol covers gas for small transactions
4. **Batching:** Aggregate multiple users' swaps to share gas cost

### Edge Case 10: Smart Contract Blacklisting

**Scenario:** Centralized wrapped token (e.g., USDC) blacklists IntentRouter

**Detection:**
```solidity
// Check blacklist before operations
require(!token.isBlacklisted(address(this)), "Contract blacklisted");
```

**Resolution:**
1. **Multi-Contract Architecture:** Deploy multiple IntentRouter instances
2. **Automatic Failover:** Switch to backup contract if primary blacklisted
3. **Direct User Interaction:** Fall back to manual swap if all routers blacklisted
4. **Decentralized Alternatives:** Prefer non-blacklistable tokens (wBTC over WBTC)

---

## Conclusion

The **Intermediate Token Abstraction Layer** transforms ETRID's user experience by completely hiding wrapped tokens (wBTC, wETH, etc.) from end users. Similar to how Ethereum users never directly interact with wei or gas tokens, ETRID users will only see external currencies (BTC, ETH, SOL) and ÉTR.

**Key Achievements:**
1. **Simplified UX:** Single-button conversions (BTC → ÉTR)
2. **Atomic Operations:** Wrapped tokens exist only within transactions (~2 seconds)
3. **Invisible Complexity:** Users never own or manage wrapped tokens
4. **Robust Fallbacks:** Graceful degradation if auto-swap fails
5. **Security-First:** Multiple layers of protection against exploits

**Next Steps:**
1. Begin smart contract development (Phase 1)
2. Conduct security audit before testnet deployment
3. Integrate with wallet UI (Phase 2)
4. Deploy to testnet and iterate based on testing (Phase 4)
5. Monitor closely post-mainnet launch (Phase 5)

This architecture positions ETRID as having the most seamless cross-chain UX in the industry, where the complexity of bridging and wrapping is completely abstracted away from users.

---

## References

### Academic & Industry Research
1. [Understanding Wei and Gwei: Ethereum's Smallest Denominations](https://platform.chainbase.com/blog/article/understanding-wei-and-gwei-ethereum-s-smallest-denominations-explained)
2. [Ethereum Gas Fees: Understand Gwei and Crypto Gas](https://plisio.net/blog/gwei-and-gas-fueling-the-ethereum-network)
3. [2025 Account Abstraction Trends](https://coingape.com/brandtalk/opinion/2025-account-abstraction-trends-our-senior-smart-contract-engineer-weighs-in/)
4. [Gas Abstraction and Multichain AA Wallets](https://blog.web3auth.io/multichain-account-gas-abstraction/)

### DeFi Patterns & Intent-Based Architecture
5. [DeFi in 2025: Intent-Based UX](https://idesignstrategy.medium.com/defi-in-2025-whats-real-what-s-noise-and-what-comes-next-4498f6a5c136)
6. [Mix, Match, Automate DeFi Swaps](https://medium.com/@factor.fi/mix-match-automate-defi-swaps-341d1030bf31)
7. [DeFi Protocol Architecture: Builder's Guide](https://medium.com/@athguy.dev/defi-protocol-architecture-a-builders-guide-to-what-actually-happens-under-the-hood-38713f93b4e5)

### Cross-Chain Bridges & Abstraction
8. [Types of Crypto Bridges 2025](https://across.to/blog/types-of-crypto-bridges-2025)
9. [Best Crypto Bridges 2025](https://coincodex.com/article/33034/best-crypto-bridges/)
10. [Understanding Cross-Chain Bridges](https://medium.com/@ancilartech/understanding-cross-chain-bridges-token-and-data-movement-between-blockchains-ca0594d81b3a)

### Smart Contract Design Patterns
11. [Smart Contract Design Patterns in Solidity](https://metana.io/blog/smart-contract-design-patterns-in-solidity-explained/)
12. [Smart Contract Gas Optimization](https://fxis.ai/edu/smart-contract-gas-optimization/)

---

**Document Version:** 1.0
**Last Updated:** 2025-12-08
**Author:** Claude Sonnet 4.5 (ETRID Architecture Team)
**Status:** DRAFT - Pending Review
