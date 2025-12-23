# LayerZero + One-Sided Liquidity Pool Architecture
**ĒTRID Omnichain Token Flow Design**

---

## EXECUTIVE SUMMARY

This document outlines the complete token flow architecture integrating:
1. **LayerZero** - Omnichain messaging protocol
2. **One-Sided Liquidity Pools** - PrimeSwap bootstrap mechanism (ALREADY IMPLEMENTED)
3. **Bridge Adapters** - External DEX listings (Base, BSC, Solana, etc.)
4. **Wrapped Tokens** - wETR deployment across 5+ chains
5. **Native ÉTR Everywhere** - Seamless cross-chain ÉTR access

**Status**:
- ✅ One-sided pools: IMPLEMENTED (PrimeSwap)
- ✅ Bridge adapters: READY FOR DEPLOYMENT
- ✅ Wrapped token contracts: READY FOR DEPLOYMENT
- ❌ LayerZero integration: NOT IMPLEMENTED (design phase)

---

## PART 1: ONE-SIDED LIQUIDITY POOL MECHANISM

### 1.1 Current Implementation (PrimeSwap)

**Location**: `contracts/primeswap/`

#### Pool Type 1: VirtualReserveAMM (11 Pools)

**Pairs**: ÉTR / wBTC, wETH, wXRP, wDOGE, wSOL, wADA, wLINK, wMATIC, wBNB, wTRX, wXLM

```
┌─────────────────────────────────────────────────────────────────────────┐
│                   ONE-SIDED LIQUIDITY BOOTSTRAP                         │
│                   Foundation Seeds ÉTR ONLY                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  STEP 1: INITIALIZATION (t=0)                                          │
│  ┌───────────────────────────────────────────────────────────────┐    │
│  │   Real ÉTR Reserve:       845,750,000 ÉTR                     │    │
│  │   Real wBTC Reserve:      0 BTC                               │    │
│  │   Virtual wBTC Reserve:   33.83 BTC (oracle-calculated)       │    │
│  │                                                                │    │
│  │   Oracle Data:                                                │    │
│  │   • 1 BTC = $25,000                                           │    │
│  │   • 1 ÉTR = $0.004                                            │    │
│  │                                                                │    │
│  │   Formula: Virtual wBTC = (845.75M * $0.004) / $25,000        │    │
│  │           = $3,383,000 / $25,000 = 33.83 BTC                  │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                ↓                                        │
│  STEP 2: FIRST USER BUYS ÉTR WITH wBTC                                │
│  ┌───────────────────────────────────────────────────────────────┐    │
│  │   User Action:                                                │    │
│  │   • Deposits 1 wBTC                                           │    │
│  │   • Wants to buy ÉTR                                          │    │
│  │                                                                │    │
│  │   AMM Calculation:                                            │    │
│  │   • k = 845.75M ÉTR * 33.83 wBTC (virtual)                    │    │
│  │   • New wBTC = 33.83 + 1 = 34.83 wBTC                         │    │
│  │   • New ÉTR = k / 34.83                                       │    │
│  │   • ÉTR out ≈ 24.5M ÉTR (price impact: ~$0.0041/ÉTR)         │    │
│  │                                                                │    │
│  │   Post-Swap State:                                            │    │
│  │   • Real ÉTR: 821.25M                                         │    │
│  │   • Real wBTC: 1.0                                            │    │
│  │   • Virtual wBTC: 33.83 wBTC (unchanged yet)                  │    │
│  │   • Total wBTC (for AMM): 1.0 + 33.83 = 34.83                │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                ↓                                        │
│  STEP 3: VIRTUAL RESERVE PHASE-OUT (Gradual)                          │
│  ┌───────────────────────────────────────────────────────────────┐    │
│  │   After Many Swaps:                                           │    │
│  │   • Real wBTC grows (10 BTC, 20 BTC, 30 BTC...)              │    │
│  │   • Virtual wBTC decreases proportionally                     │    │
│  │   • Formula: Virtual wBTC = max(0, Initial - Real * Factor)   │    │
│  │                                                                │    │
│  │   Example at 20 BTC real:                                     │    │
│  │   • Real wBTC: 20.0                                           │    │
│  │   • Virtual wBTC: 13.83 (phasing out)                         │    │
│  │   • Total wBTC for AMM: 33.83                                 │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                ↓                                        │
│  STEP 4: PURE AMM (Threshold Reached)                                 │
│  ┌───────────────────────────────────────────────────────────────┐    │
│  │   When Real wBTC > Threshold (e.g., 34 BTC):                 │    │
│  │   • Virtual wBTC = 0 (fully phased out)                       │    │
│  │   • Pure AMM: x * y = k                                       │    │
│  │   • Market-driven pricing                                     │    │
│  │                                                                │    │
│  │   Final State:                                                │    │
│  │   • Real ÉTR: 600M                                            │    │
│  │   • Real wBTC: 50                                             │    │
│  │   • Price: Purely determined by supply/demand                 │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Benefits**:
- ✅ Foundation only provides ÉTR (no need to acquire BTC, ETH, etc.)
- ✅ Immediate liquidity for users
- ✅ Price discovery via oracle → market transition
- ✅ Automatic transition to pure AMM

#### Pool Type 2: EDSCOraclePool (1 Pool)

**Pair**: ÉTR / EDSC

```
┌──────────────────────────────────────────────────────────────┐
│         EDSC/ÉTR POOL (Reverse One-Sided Bootstrap)          │
│         Protocol Seeds EDSC ONLY                             │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  INITIALIZATION:                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │   EDSC Reserve: 100,000,000 EDSC (protocol-minted) │    │
│  │   ÉTR Reserve:  0 ÉTR                              │    │
│  │                                                     │    │
│  │   Oracle Rate:                                     │    │
│  │   • 1 EDSC = $1.00 (by design)                    │    │
│  │   • 1 ÉTR = $0.004 (market)                       │    │
│  │   • Exchange: 1 EDSC = 250 ÉTR                    │    │
│  └────────────────────────────────────────────────────┘    │
│                        ↓                                    │
│  USER WANTS EDSC:                                          │
│  ┌────────────────────────────────────────────────────┐    │
│  │   • User deposits 250 ÉTR                          │    │
│  │   • Gets 1 EDSC (at oracle rate)                   │    │
│  │   • EDSC reserve ↓ 99,999,999                      │    │
│  │   • ÉTR reserve ↑ 250                              │    │
│  └────────────────────────────────────────────────────┘    │
│                        ↓                                    │
│  TRANSITION TO AMM:                                        │
│  ┌────────────────────────────────────────────────────┐    │
│  │   When ÉTR > threshold (e.g., 1M ÉTR):            │    │
│  │   • AMM pricing activates                          │    │
│  │   • Market determines ÉTR/EDSC rate                │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

#### Pool Type 3: EDSCPegPool (1 Pool)

**Pair**: EDSC / USDT (Stability Peg)

```
┌──────────────────────────────────────────────────────────────┐
│      EDSC/USDT STABILITY POOL (Single-Sided EDSC)            │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  INITIALIZATION:                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │   EDSC Reserve: 10,000,000 EDSC (protocol-minted)  │    │
│  │   USDT Reserve: 0 USDT                             │    │
│  │   Rate: 1:1 fixed (EDSC = USDT = $1)              │    │
│  │   Fee: 0.04% (stablecoin standard)                 │    │
│  └────────────────────────────────────────────────────┘    │
│                        ↓                                    │
│  USER SWAPS USDT → EDSC:                                   │
│  ┌────────────────────────────────────────────────────┐    │
│  │   • User deposits 1000 USDT                        │    │
│  │   • Gets 999.96 EDSC (1:1 - 0.04% fee)            │    │
│  │   • EDSC reserve ↓ 9,000,004                       │    │
│  │   • USDT reserve ↑ 1,000                           │    │
│  └────────────────────────────────────────────────────┘    │
│                        ↓                                    │
│  STABLESWAP ACTIVATION:                                    │
│  ┌────────────────────────────────────────────────────┐    │
│  │   When USDT > 1M threshold:                        │    │
│  │   • Curve-style StableSwap activates               │    │
│  │   • Amplification: 100 (tight peg)                 │    │
│  │   • Both EDSC→USDT and USDT→EDSC enabled          │    │
│  │   • Minimal slippage (~0.01% for large swaps)     │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Benefits**:
- ✅ No USDT needed upfront
- ✅ Guarantees EDSC = $1 peg
- ✅ StableSwap ensures tight peg when active
- ✅ Emergency liquidity for EDSC holders

---

## PART 2: LAYERZERO INTEGRATION ARCHITECTURE

### 2.1 LayerZero Overview

**What is LayerZero?**
- Omnichain messaging protocol
- Connects 50+ blockchains
- Enables cross-chain token transfers, message passing, and state sync
- Used by: Stargate Finance, Aptos Bridge, etc.

**Why LayerZero for ĒTRID?**
- Native ÉTR accessible from any LayerZero-connected chain
- Users on Arbitrum can buy ÉTR using ETH without bridging first
- Single ÉTR token contract deployed on all chains (Omnichain Fungible Token - OFT)
- Seamless UX: "Buy ÉTR" button works on any chain

### 2.2 Proposed LayerZero Integration

#### Component 1: Omnichain Fungible Token (OFT)

**Contract**: `EtridOFT.sol` (to be created)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@layerzerolabs/oft-evm/contracts/OFT.sol";

/**
 * @title EtridOFT
 * @notice Omnichain Fungible Token for ÉTR
 * @dev Enables native ÉTR on all LayerZero-connected chains
 */
contract EtridOFT is OFT {
    constructor(
        string memory _name,
        string memory _symbol,
        address _lzEndpoint,
        address _delegate
    ) OFT(_name, _symbol, _lzEndpoint, _delegate) {
        // Mint initial supply on Primearc Core Chain only
        if (block.chainid == PRIMEARC_CHAIN_ID) {
            _mint(msg.sender, INITIAL_SUPPLY);
        }
    }

    // Primearc Core Chain mints real ÉTR
    // Other chains receive ÉTR via LayerZero cross-chain transfer
    // Burn on source → Mint on destination (trustless)
}
```

**Deployment Plan**:
1. Deploy on Primearc Core Chain (source of truth)
2. Deploy on Ethereum, Arbitrum, Optimism, Base, BSC, Polygon, Avalanche, etc.
3. Configure LayerZero peer mappings
4. Test cross-chain transfers

#### Component 2: Cross-Chain Purchase Handler

**Contract**: `EtridOmniPurchase.sol` (to be created)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@layerzerolabs/lz-evm-oapp-v2/contracts/oapp/OApp.sol";
import "./interfaces/IPrimeSwap.sol";

/**
 * @title EtridOmniPurchase
 * @notice Allows users to buy ÉTR from any chain using native tokens
 * @dev LayerZero message → Primearc → PrimeSwap swap → Bridge ÉTR back
 */
contract EtridOmniPurchase is OApp {
    IPrimeSwap public primeSwap;
    IEtridBridge public bridge;

    /**
     * @notice Buy ÉTR using native token (e.g., ETH on Arbitrum)
     * @param minEtrOut Minimum ÉTR to receive (slippage protection)
     * @param recipient Address to receive ÉTR
     */
    function buyEtrWithNative(
        uint256 minEtrOut,
        address recipient
    ) external payable {
        // Step 1: Bridge native token to Primearc
        // (Using existing bridge adapter)
        uint256 wrappedAmount = _bridgeNativeToken(msg.value);

        // Step 2: Send LayerZero message to Primearc
        bytes memory payload = abi.encode(
            BUY_ETR_COMMAND,
            wrappedAmount,
            minEtrOut,
            recipient,
            block.chainid // Source chain
        );

        _lzSend(
            PRIMEARC_ENDPOINT_ID,
            payload,
            _getDefaultSendOptions(),
            MessagingFee(msg.value, 0),
            payable(msg.sender)
        );

        emit CrossChainPurchaseInitiated(msg.sender, msg.value, recipient);
    }

    /**
     * @notice Receive LayerZero message from Primearc (swap complete)
     * @param _payload Encoded swap result
     */
    function _lzReceive(
        Origin calldata _origin,
        bytes32 _guid,
        bytes calldata _payload,
        address _executor,
        bytes calldata _extraData
    ) internal override {
        (address recipient, uint256 etrAmount, uint256 sourceChain) = abi.decode(
            _payload,
            (address, uint256, uint256)
        );

        // Step 3: ÉTR received from Primearc → Send to user
        // (LayerZero OFT handles cross-chain transfer automatically)

        emit CrossChainPurchaseCompleted(recipient, etrAmount, sourceChain);
    }
}
```

### 2.3 Complete Flow Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                    LAYERZERO + ONE-SIDED POOL FLOW                       │
│              User Buys ÉTR from Arbitrum Using ETH                       │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  CHAIN: ARBITRUM (User's Perspective)                                   │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  USER ACTION:                                                   │    │
│  │  • Opens wallet on Arbitrum                                     │    │
│  │  • Clicks "Buy ÉTR" button                                      │    │
│  │  • Enters amount: 1 ETH                                         │    │
│  │  • Confirms transaction                                         │    │
│  │                                                                  │    │
│  │  CONTRACT: EtridOmniPurchase (Arbitrum)                        │    │
│  │  • Receives 1 ETH from user                                     │    │
│  │  • Encodes LayerZero message:                                   │    │
│  │    - Command: BUY_ETR                                           │    │
│  │    - Amount: 1 ETH                                              │    │
│  │    - Min ÉTR: 2000 ÉTR (slippage protection)                   │    │
│  │    - Recipient: 0xUser...                                       │    │
│  │    - Source Chain: 42161 (Arbitrum)                             │    │
│  │  • Calls LayerZero Endpoint                                     │    │
│  └────────────────────────────────────────────────────────────────┘    │
│                                ↓                                         │
│                    [LayerZero Relayer Network]                           │
│                                ↓                                         │
│  CHAIN: PRIMEARC CORE (Ëtrid Relay Chain)                              │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  CONTRACT: EtridOmniPurchase (Primearc)                        │    │
│  │  • Receives LayerZero message                                   │    │
│  │  • Decodes payload: BUY_ETR, 1 ETH, 2000 min ÉTR, 0xUser...   │    │
│  │  • Looks up wETH bridge adapter                                 │    │
│  │  • Receives wETH from Arbitrum bridge                           │    │
│  │                                                                  │    │
│  │  STEP 1: BRIDGE ADAPTER CONFIRMS DEPOSIT                        │    │
│  │  • ETH-PBC receives 1 ETH lock event from Arbitrum             │    │
│  │  • Mints 1 wETH on ETH-PBC                                      │    │
│  │  • Bridges 1 wETH to Primearc Core Chain                        │    │
│  │                                                                  │    │
│  │  STEP 2: PRIMESWAP SWAP                                         │    │
│  │  • Contract calls PrimeSwap Router                              │    │
│  │  • Finds pool: ÉTR/wETH (VirtualReserveAMM)                    │    │
│  │                                                                  │    │
│  │  Pool State BEFORE Swap:                                        │    │
│  │  ┌──────────────────────────────────────────────────────┐      │    │
│  │  │  ÉTR Reserve (real):      191,375,000 ÉTR            │      │    │
│  │  │  wETH Reserve (real):     5 wETH                     │      │    │
│  │  │  wETH Reserve (virtual):  75 wETH                    │      │    │
│  │  │  Total wETH for AMM:      80 wETH                    │      │    │
│  │  │  k = 191.375M * 80 = 15.31B                          │      │    │
│  │  └──────────────────────────────────────────────────────┘      │    │
│  │                                                                  │    │
│  │  Swap Execution:                                                │    │
│  │  • User swaps 1 wETH for ÉTR                                   │    │
│  │  • New wETH total = 80 + 1 = 81                                │    │
│  │  • New ÉTR = 15.31B / 81 ≈ 189.01M                             │    │
│  │  • ÉTR out = 191.375M - 189.01M ≈ 2.365M ÉTR                  │    │
│  │  • Fee (0.3%): 7,095 ÉTR → Treasury                           │    │
│  │  • User gets: 2,357,905 ÉTR                                    │    │
│  │                                                                  │    │
│  │  Pool State AFTER Swap:                                         │    │
│  │  ┌──────────────────────────────────────────────────────┐      │    │
│  │  │  ÉTR Reserve (real):      189,017,095 ÉTR            │      │    │
│  │  │  wETH Reserve (real):     6 wETH                     │      │    │
│  │  │  wETH Reserve (virtual):  75 wETH (unchanged)        │      │    │
│  │  │  Total wETH for AMM:      81 wETH                    │      │    │
│  │  └──────────────────────────────────────────────────────┘      │    │
│  │                                                                  │    │
│  │  STEP 3: SEND ÉTR TO USER ON ARBITRUM                          │    │
│  │  • Contract has 2,357,905 ÉTR                                   │    │
│  │  • Calls EtridOFT.send() (LayerZero OFT)                       │    │
│  │  • Burns 2,357,905 ÉTR on Primearc                             │    │
│  │  • Encodes LayerZero message:                                   │    │
│  │    - Destination: Arbitrum (42161)                              │    │
│  │    - Recipient: 0xUser...                                       │    │
│  │    - Amount: 2,357,905 ÉTR                                      │    │
│  │  • Sends to LayerZero Endpoint                                  │    │
│  └────────────────────────────────────────────────────────────────┘    │
│                                ↓                                         │
│                    [LayerZero Relayer Network]                           │
│                                ↓                                         │
│  CHAIN: ARBITRUM (User Receives ÉTR)                                   │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  CONTRACT: EtridOFT (Arbitrum)                                  │    │
│  │  • Receives LayerZero message from Primearc                     │    │
│  │  • Mints 2,357,905 ÉTR to 0xUser...                            │    │
│  │                                                                  │    │
│  │  USER WALLET:                                                    │    │
│  │  • Balance before: 0 ÉTR                                        │    │
│  │  • Balance after: 2,357,905 ÉTR                                 │    │
│  │  • Gas spent: ~$5 (Arbitrum)                                    │    │
│  │  • LayerZero fee: ~$1                                           │    │
│  │  • PrimeSwap fee: 7,095 ÉTR                                     │    │
│  │                                                                  │    │
│  │  RESULT: ✅ User bought ÉTR using ETH without leaving Arbitrum! │    │
│  └────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  TOTAL TIME: 2-5 minutes (depending on LayerZero relayer speed)         │
│  TOTAL COST: ~$6 (gas + LayerZero fee)                                 │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

### 2.4 Supported Use Cases

With LayerZero + One-Sided Pools, users can:

1. **Buy ÉTR from Any Chain**
   - Arbitrum user: ETH → ÉTR (via ÉTR/wETH pool)
   - Base user: ETH → ÉTR (via ÉTR/wETH pool)
   - BSC user: BNB → ÉTR (via ÉTR/wBNB pool)
   - Polygon user: MATIC → ÉTR (via ÉTR/wMATIC pool)
   - Solana user: SOL → ÉTR (via ÉTR/wSOL pool)

2. **Buy EDSC from Any Chain**
   - Any user: Any token → ÉTR → EDSC (via ÉTR/EDSC pool)
   - Direct USDT → EDSC (via EDSC/USDT stability pool)

3. **Provide Liquidity on Any Chain**
   - Arbitrum LP: Add ETH → Gets wETH/ÉTR LP tokens
   - LP tokens tradable on Arbitrum
   - Rewards claimable on Arbitrum
   - No need to interact with Primearc directly

4. **Farm on Any Chain**
   - Stake LP tokens on source chain
   - LayerZero message → Primearc MasterChef
   - Rewards (ÉTR) bridged back to source chain
   - Claim rewards on original chain

---

## PART 3: WRAPPED TOKEN + BRIDGE ADAPTER FLOW

### 3.1 Current Deployment Status

**Bridge Adapters** (`05-multichain/bridges/adapters/`):
- ✅ Base L2 - Ready
- ✅ BSC - Ready
- ✅ Solana - Ready
- ✅ Hyperliquid - Ready
- ✅ BullEx - Ready

**User Note**: "i launched contracts after this was developed keep that in mind"

**Implication**: Contracts deployed but addresses not updated in README

### 3.2 Wrapped Token Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│              WRAPPED ETR (wETR) DEPLOYMENT MATRIX                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Chain         │ Contract           │ Standard │ Status             │
│  ──────────────┼────────────────────┼──────────┼───────────────────│
│  BSC           │ WrappedETR.sol     │ BEP-20   │ ✅ Deployed (TBD) │
│  Base L2       │ WrappedToken.sol   │ ERC-20   │ ✅ Deployed (TBD) │
│  Ethereum      │ WrappedToken.sol   │ ERC-20   │ ⏳ Pending        │
│  Arbitrum      │ WrappedToken.sol   │ ERC-20   │ ⏳ Pending        │
│  Optimism      │ WrappedToken.sol   │ ERC-20   │ ⏳ Pending        │
│  Polygon       │ WrappedToken.sol   │ ERC-20   │ ⏳ Pending        │
│  Avalanche     │ WrappedToken.sol   │ ERC-20   │ ⏳ Pending        │
│  Solana        │ SPL Token Mint     │ SPL      │ ✅ Deployed (TBD) │
│  Hyperliquid   │ N/A (API-based)    │ Native   │ ✅ Listed (TBD)   │
│  BullEx        │ N/A (Omnichain)    │ Multi    │ ✅ Listed (TBD)   │
│                                                                     │
│  TBD = Address not updated in README                               │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.3 Bridge Adapter Flow (Without LayerZero)

**Current Implementation** (Traditional Bridge):

```
USER ON BSC:
    ↓
1. Locks ÉTR in BSC bridge contract
    ↓
2. Bridge Adapter monitors lock event
    ↓
3. Adapter sends message to BNB-PBC
    ↓
4. BNB-PBC validates message via pallet-bridge-attestation (M-of-N sigs)
    ↓
5. BNB-PBC mints native ÉTR on Primearc via pallet-token-messenger
    ↓
6. User has ÉTR on Primearc
```

**REVERSE (Primearc → BSC):**

```
USER ON PRIMEARC:
    ↓
1. Burns ÉTR via pallet-token-messenger
    ↓
2. Message created with attestation
    ↓
3. Relayer transmits to BSC
    ↓
4. BSC bridge contract verifies attestation
    ↓
5. BSC bridge mints wETR to user
    ↓
6. User has wETR on BSC (can trade on PancakeSwap)
```

### 3.4 Liquidity Pool Creation on External DEXs

**Example: PancakeSwap on BSC**

**Adapter**: `05-multichain/bridges/adapters/bsc/deploy-pancakeswap.ts`

```typescript
// 1. Deploy wETR token on BSC
const wETR = await deployWrappedETR({
  name: "Wrapped Etrid",
  symbol: "wETR",
  decimals: 18,
  bridge: BSC_BRIDGE_ADDRESS
});

// 2. Create PancakeSwap pool: wETR/BUSD
const pool = await pancakeFactory.createPair(
  wETR.address,
  BUSD_ADDRESS // Binance-Peg BUSD
);

// 3. Add initial liquidity (if desired)
// Foundation could seed pool, or wait for market makers
await router.addLiquidity(
  wETR.address,
  BUSD_ADDRESS,
  ethers.utils.parseEther("1000000"), // 1M wETR
  ethers.utils.parseEther("4000"),    // $4,000 BUSD ($0.004/wETR)
  0, // slippage
  0,
  FOUNDATION_ADDRESS,
  deadline
);

// 4. Now BSC users can trade wETR/BUSD on PancakeSwap!
```

**Benefits**:
- ✅ No new PBC needed (reuses BNB-PBC)
- ✅ Fast deployment (1-2 days)
- ✅ Low cost ($50-500 gas)
- ✅ Access to BSC DeFi ecosystem

---

## PART 4: COMPLETE INTEGRATION MATRIX

### 4.1 Token Access Points

**ÉTR is accessible via:**

| Method | Chains Supported | Mechanism | Status |
|--------|------------------|-----------|--------|
| **Native ÉTR** | Primearc Core + 14 PBCs | On-chain native | ✅ LIVE |
| **Wrapped ÉTR** | BSC, Base, Solana | Bridge adapters | ✅ DEPLOYED (TBD) |
| **OFT ÉTR** | 50+ LayerZero chains | LayerZero OFT | ❌ NOT IMPLEMENTED |
| **External DEX** | 10+ chains | Wrapped tokens | ⏳ PENDING |

### 4.2 Liquidity Sources

| Pool Type | Location | Mechanism | Liquidity Source | Status |
|-----------|----------|-----------|------------------|--------|
| VirtualReserveAMM | Primearc (PrimeSwap) | Single-sided ÉTR | Foundation | ✅ IMPLEMENTED |
| EDSCOraclePool | Primearc (PrimeSwap) | Single-sided EDSC | Protocol | ✅ IMPLEMENTED |
| EDSCPegPool | Primearc (PrimeSwap) | Single-sided EDSC | Protocol | ✅ IMPLEMENTED |
| PancakeSwap | BSC | wETR/BUSD | Market makers | ⏳ PENDING |
| Uniswap V3 | Base L2 | wETR/ETH | Market makers | ⏳ PENDING |
| Raydium | Solana | wETR/SOL | Market makers | ⏳ PENDING |

### 4.3 User Journey Comparison

**WITHOUT LayerZero** (Current):
```
Arbitrum User wants ÉTR:
1. Bridge ETH from Arbitrum → Ethereum (5-10 min, $10-50)
2. Buy wETH on Ethereum
3. Bridge wETH to Primearc via ETH-PBC (10-20 min, $5-20)
4. Swap wETH for ÉTR on PrimeSwap (1 min, $0.10)
5. Total time: 30-60 minutes
6. Total cost: $15-70
```

**WITH LayerZero** (Proposed):
```
Arbitrum User wants ÉTR:
1. Click "Buy ÉTR" on Arbitrum
2. Confirm 1 transaction
3. LayerZero handles everything
4. Receive ÉTR on Arbitrum
5. Total time: 2-5 minutes
6. Total cost: $6 (gas + LayerZero + PrimeSwap fee)
```

**Improvement**:
- ⚡ 10x faster
- 💰 3x cheaper
- 🎯 100x easier UX

---

## PART 5: IMPLEMENTATION ROADMAP

### Phase 1: Fix Existing Infrastructure (Immediate)

**Duration**: 12-16 hours

1. **Fix Primearc Bridge** (2h)
   - Replace EDSC pallets with generic pallets
   - Wire to Balances pallet for ÉTR

2. **Fix TokenOperations** (1h)
   - Wire `PbcTokenOperations<Runtime>` to 4 ASF PBCs
   - Replace `type TokenOperations = ();` placeholders

3. **Activate Slashing** (1.5h)
   - Uncomment SlashingInterface in 5 chains

4. **Complete ASF Integration** (9h)
   - Fix ETH-PBC (remove Aura, add ASF) (3h)
   - Add ASF to 9 missing PBCs (6h parallel with 3 agents)

### Phase 2: Wrapped Token Deployment (1-2 weeks)

1. **Get Orphaned Token Addresses** (User provides)
   - 5 deployed contract addresses

2. **Verify Deployments**
   - Check BscScan, Basescan, Solscan for contracts
   - Verify bridge minter roles

3. **Create Liquidity Pools**
   - PancakeSwap: wETR/BUSD (BSC)
   - Uniswap V3: wETR/ETH (Base L2)
   - Raydium: wETR/SOL (Solana)

4. **Configure Bridge Adapters**
   - Wire adapters to deployed contracts
   - Start monitoring burn/mint events
   - Test transfers both directions

### Phase 3: LayerZero Integration (4-8 weeks)

1. **Research & Design** (1 week)
   - Study LayerZero V2 documentation
   - Design OFT architecture
   - Security audit planning

2. **Contract Development** (2 weeks)
   - EtridOFT.sol (Omnichain Fungible Token)
   - EtridOmniPurchase.sol (Cross-chain purchase handler)
   - Unit tests + integration tests

3. **Deployment** (1 week)
   - Deploy on 10+ LayerZero-connected chains
   - Configure peer mappings
   - Test cross-chain transfers

4. **Integration with PrimeSwap** (1 week)
   - Wire LayerZero messages to PrimeSwap Router
   - Implement slippage protection
   - Error handling + refunds

5. **Audit & Launch** (1-2 weeks)
   - Security audit (recommended: Trail of Bits, OpenZeppelin)
   - Testnet deployment
   - Mainnet launch

### Phase 4: Advanced Features (Future)

1. **Omnichain Liquidity**
   - LP tokens tradable on any chain
   - Rewards claimable on any chain

2. **Omnichain Governance**
   - Vote on proposals from any chain
   - Cross-chain execution

3. **Omnichain Staking**
   - Stake ÉTR from any chain
   - Validator rewards on source chain

---

## SUMMARY

### What's Already Built

✅ **PrimeSwap One-Sided Liquidity Pools** - Fully implemented
- 11 VirtualReserveAMM pools (single-sided ÉTR)
- 1 EDSCOraclePool (single-sided EDSC)
- 1 EDSCPegPool (single-sided EDSC for stability)

✅ **Bridge Adapters** - Ready for deployment
- Base L2, BSC, Solana, Hyperliquid, BullEx
- Code complete, contracts deployed (addresses TBD)

✅ **Generic Bridge Pallets** - Production ready
- pallet-token-messenger
- pallet-bridge-attestation
- PbcTokenOperations implementation

### What Needs to be Fixed

❌ **Primearc Bridge** - Using wrong pallets (EDSC instead of generic)
❌ **TokenOperations Placeholder** - BTC/ETH/SOL/BNB/TRX have `type TokenOperations = ()`
❌ **Slashing Not Activated** - Commented out in 5 chains
❌ **Incomplete ASF** - 9 PBCs missing full ASF integration

### What Needs to be Built

❌ **LayerZero Integration** - Not implemented
- EtridOFT.sol (Omnichain Fungible Token)
- EtridOmniPurchase.sol (Cross-chain purchase handler)
- 10+ chain deployments
- Security audit

### Recommended Next Steps

1. **Complete audit findings fixes** (12-16h)
2. **Get orphaned token addresses from user**
3. **Wire bridge adapters to deployed contracts** (2h)
4. **Design LayerZero integration** (research phase)
5. **Implement LayerZero OFT** (4-8 weeks with audit)

---

**End of Architecture Document**
