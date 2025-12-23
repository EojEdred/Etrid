# Intent Router & Auto-Swap Architecture

**Location:** `/contracts/intent-router/`
**Purpose:** User-facing abstraction layer hiding wrapped tokens and multi-tier complexity
**Scope:** Seamless BTC ↔ ÉTR conversions with zero user exposure to intermediate tokens

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    INTENT ROUTER SYSTEM                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  USER INTENT (Simple)                                           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  "I want to convert 0.5 BTC → ÉTR"                       │  │
│  │  No knowledge of: wBTC, pools, tiers, bridges            │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          ↓ Single call                          │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  INTENT ROUTER                                            │  │
│  │  ├─ Parse user intent                                    │  │
│  │  ├─ Calculate optimal route                              │  │
│  │  ├─ Estimate output (0.5 BTC = ~12,487 ÉTR)            │  │
│  │  └─ Execute atomic multi-step swap                      │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          ↓ Orchestrate                          │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  AUTO-SWAP EXECUTOR (Atomic)                             │  │
│  │  Step 1: Lock 0.5 BTC on Bitcoin chain                  │  │
│  │  Step 2: Bridge verifies (3-of-5 attestation)           │  │
│  │  Step 3: Tier 1 receives 0.5 BTC, mints 0.5 wBTC        │  │
│  │  Step 4: Tier 2 swaps 0.5 wBTC → 12,487 ÉTR (AMM)      │  │
│  │  Step 5: User receives 12,487 ÉTR                       │  │
│  │  wBTC NEVER touches user's wallet ✓                     │  │
│  └──────────────────────────────────────────────────────────┘  │
│                          ↓ Result                               │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  USER WALLET                                              │  │
│  │  Before: 0.5 BTC                                         │  │
│  │  After:  12,487 ÉTR                                      │  │
│  │  Experience: Single-click conversion ✓                   │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Contracts

### 1. IntentRouter.sol

**Location:** `contracts/intent-router/core/IntentRouter.sol`

**Purpose:** User-facing interface for all conversions

**State Variables:**
```solidity
// Routing configuration
mapping(address => address) public tier1Pools;      // externalToken → Tier1Pool
mapping(address => address) public tier2Pools;      // wrappedToken → Tier2Pool
mapping(address => address) public wrappedTokens;   // externalToken → wrappedToken
mapping(bytes32 => address) public bridgeTrackers;  // chainId → BridgeTracker

// Executor contracts
address public autoSwapExecutor;
address public twoTierBridgeRouter;

// Fee configuration
uint256 public platformFeePercent = 30;  // 0.3% (30 basis points)
address public feeCollector;

// Slippage protection
uint256 public defaultMaxSlippage = 50;  // 0.5% max slippage
```

**Core User Functions:**

#### External Currency → ÉTR (Buy ÉTR)
```solidity
/**
 * @notice Convert external currency (BTC, ETH, SOL) to ÉTR in a single transaction
 * @param sourceCurrency Address of external currency (address(0) for native)
 * @param amount Amount of external currency to convert
 * @param minEtrOut Minimum ÉTR to receive (slippage protection)
 * @param deadline Transaction deadline (Unix timestamp)
 * @return etrReceived Amount of ÉTR received
 */
function convertToEtr(
    address sourceCurrency,
    uint256 amount,
    uint256 minEtrOut,
    uint256 deadline
) external payable returns (uint256 etrReceived) {
    require(block.timestamp <= deadline, "Deadline expired");
    require(amount > 0, "Invalid amount");

    // 1. Get route configuration
    RouteConfig memory route = getRoute(sourceCurrency, ETR_TOKEN);

    // 2. Estimate output
    uint256 estimatedOutput = estimateOutput(
        sourceCurrency,
        ETR_TOKEN,
        amount
    );
    require(estimatedOutput >= minEtrOut, "Slippage too high");

    // 3. Execute atomic swap via AutoSwapExecutor
    etrReceived = IAutoSwapExecutor(autoSwapExecutor).executeSwap(
        SwapIntent({
            user: msg.sender,
            sourceToken: sourceCurrency,
            destToken: ETR_TOKEN,
            amountIn: amount,
            minAmountOut: minEtrOut,
            route: route,
            deadline: deadline
        })
    );

    // 4. Deduct platform fee
    uint256 fee = (etrReceived * platformFeePercent) / 10000;
    uint256 amountAfterFee = etrReceived - fee;

    // 5. Transfer to user
    IERC20(ETR_TOKEN).transfer(msg.sender, amountAfterFee);
    IERC20(ETR_TOKEN).transfer(feeCollector, fee);

    emit ConversionExecuted(
        msg.sender,
        sourceCurrency,
        ETR_TOKEN,
        amount,
        amountAfterFee,
        fee
    );

    return amountAfterFee;
}
```

#### ÉTR → External Currency (Sell ÉTR)
```solidity
/**
 * @notice Convert ÉTR to external currency (BTC, ETH, SOL) and withdraw
 * @param targetCurrency Target external currency
 * @param etrAmount Amount of ÉTR to convert
 * @param minCurrencyOut Minimum external currency to receive
 * @param externalAddress Address on external chain to receive funds
 * @param deadline Transaction deadline
 * @return currencyWithdrawn Amount of external currency withdrawn
 */
function convertFromEtr(
    address targetCurrency,
    uint256 etrAmount,
    uint256 minCurrencyOut,
    bytes32 externalAddress,
    uint256 deadline
) external returns (uint256 currencyWithdrawn) {
    require(block.timestamp <= deadline, "Deadline expired");
    require(etrAmount > 0, "Invalid amount");
    require(externalAddress != bytes32(0), "Invalid external address");

    // 1. Get route configuration
    RouteConfig memory route = getRoute(ETR_TOKEN, targetCurrency);

    // 2. Estimate output
    uint256 estimatedOutput = estimateOutput(
        ETR_TOKEN,
        targetCurrency,
        etrAmount
    );
    require(estimatedOutput >= minCurrencyOut, "Slippage too high");

    // 3. Transfer ÉTR from user
    IERC20(ETR_TOKEN).transferFrom(msg.sender, address(this), etrAmount);

    // 4. Execute reverse swap (ÉTR → wToken → External)
    currencyWithdrawn = IAutoSwapExecutor(autoSwapExecutor).executeReverseSwap(
        SwapIntent({
            user: msg.sender,
            sourceToken: ETR_TOKEN,
            destToken: targetCurrency,
            amountIn: etrAmount,
            minAmountOut: minCurrencyOut,
            route: route,
            deadline: deadline
        }),
        externalAddress
    );

    emit WithdrawalInitiated(
        msg.sender,
        targetCurrency,
        currencyWithdrawn,
        externalAddress
    );

    return currencyWithdrawn;
}
```

#### Quote/Estimate Functions
```solidity
/**
 * @notice Get estimated output for a swap without executing
 * @param sourceToken Source token address
 * @param destToken Destination token address
 * @param amountIn Amount of source token
 * @return estimatedOut Estimated output amount
 * @return priceImpact Price impact percentage (basis points)
 * @return fee Platform fee amount
 */
function getQuote(
    address sourceToken,
    address destToken,
    uint256 amountIn
) external view returns (
    uint256 estimatedOut,
    uint256 priceImpact,
    uint256 fee
) {
    RouteConfig memory route = getRoute(sourceToken, destToken);

    if (route.routeType == RouteType.ExternalToEtr) {
        // External → wToken → ÉTR
        address wrappedToken = wrappedTokens[sourceToken];

        // Tier 2 swap: wToken → ÉTR
        estimatedOut = ITier2Pool(route.tier2Pool).getAmountOut(
            amountIn,
            wrappedToken,
            destToken
        );

    } else if (route.routeType == RouteType.EtrToExternal) {
        // ÉTR → wToken → External
        address wrappedToken = wrappedTokens[destToken];

        // Tier 2 swap: ÉTR → wToken
        estimatedOut = ITier2Pool(route.tier2Pool).getAmountOut(
            amountIn,
            sourceToken,
            wrappedToken
        );
    }

    // Calculate fee and price impact
    fee = (estimatedOut * platformFeePercent) / 10000;
    priceImpact = calculatePriceImpact(sourceToken, destToken, amountIn);

    return (estimatedOut, priceImpact, fee);
}
```

---

### 2. AutoSwapExecutor.sol

**Location:** `contracts/intent-router/core/AutoSwapExecutor.sol`

**Purpose:** Execute atomic multi-step swaps across tiers

**Swap Intent Structure:**
```solidity
struct SwapIntent {
    address user;           // User initiating swap
    address sourceToken;    // Source token (BTC, ETH, or ÉTR)
    address destToken;      // Destination token (ÉTR or BTC, ETH, etc.)
    uint256 amountIn;       // Input amount
    uint256 minAmountOut;   // Minimum output (slippage protection)
    RouteConfig route;      // Routing configuration
    uint256 deadline;       // Expiry timestamp
}

struct RouteConfig {
    RouteType routeType;    // ExternalToEtr or EtrToExternal
    address tier1Pool;      // Tier 1 pool address
    address tier2Pool;      // Tier 2 pool address
    address wrappedToken;   // Intermediate wrapped token
    address bridgeTracker;  // Bridge tracking contract
}

enum RouteType {
    ExternalToEtr,   // BTC → wBTC → ÉTR
    EtrToExternal,   // ÉTR → wBTC → BTC
    EtrToEtr         // ÉTR → ÉTR (direct, no swap needed)
}
```

**Execute External → ÉTR:**
```solidity
/**
 * @notice Execute atomic swap: External Currency → ÉTR
 * Flow: BTC (locked) → Bridge → Tier 1 (mint wBTC) → Tier 2 (swap wBTC→ÉTR) → User
 */
function executeSwap(
    SwapIntent memory intent
) external returns (uint256 amountOut) {
    require(msg.sender == intentRouter, "Only router");
    require(intent.routeType == RouteType.ExternalToEtr, "Invalid route");

    // ═══════════════════════════════════════════════════════════════
    // STEP 1: BRIDGE DEPOSIT (External chain lock)
    // ═══════════════════════════════════════════════════════════════

    bytes32 depositId = IBridgeTracker(intent.route.bridgeTracker).recordDeposit(
        intent.user,
        intent.amountIn,
        externalTxHash,  // Provided by user or relayer
        blockNumber,
        attestation
    );

    // Wait for M-of-N attestation (handled by bridge)
    require(
        IBridgeTracker(intent.route.bridgeTracker).isVerified(depositId),
        "Deposit not verified"
    );

    // ═══════════════════════════════════════════════════════════════
    // STEP 2: TIER 1 POOL (Lock external, mint wrapped)
    // ═══════════════════════════════════════════════════════════════

    uint256 wrappedMinted = ITier1Pool(intent.route.tier1Pool).lockAndMint(
        intent.amountIn
    );

    // Wrapped tokens go directly to this contract (not user)
    require(
        IERC20(intent.route.wrappedToken).balanceOf(address(this)) >= wrappedMinted,
        "Wrapped token minting failed"
    );

    // ═══════════════════════════════════════════════════════════════
    // STEP 3: TIER 2 POOL (Swap wrapped → ÉTR)
    // ═══════════════════════════════════════════════════════════════

    // Approve Tier 2 pool to spend wrapped tokens
    IERC20(intent.route.wrappedToken).approve(
        intent.route.tier2Pool,
        wrappedMinted
    );

    // Execute swap
    amountOut = ITier2Pool(intent.route.tier2Pool).swapWrappedForETR(
        wrappedMinted,
        intent.minAmountOut
    );

    // ═══════════════════════════════════════════════════════════════
    // STEP 4: VERIFICATION
    // ═══════════════════════════════════════════════════════════════

    require(amountOut >= intent.minAmountOut, "Slippage exceeded");
    require(block.timestamp <= intent.deadline, "Deadline passed");

    // ÉTR now in this contract, will be transferred by IntentRouter

    emit SwapExecuted(
        intent.user,
        intent.sourceToken,
        intent.destToken,
        intent.amountIn,
        amountOut,
        depositId
    );

    return amountOut;
}
```

**Execute ÉTR → External:**
```solidity
/**
 * @notice Execute atomic reverse swap: ÉTR → External Currency
 * Flow: User ÉTR → Tier 2 (swap ÉTR→wBTC) → Tier 1 (burn wBTC) → Bridge (release BTC)
 */
function executeReverseSwap(
    SwapIntent memory intent,
    bytes32 externalAddress
) external returns (uint256 amountOut) {
    require(msg.sender == intentRouter, "Only router");
    require(intent.routeType == RouteType.EtrToExternal, "Invalid route");

    // ═══════════════════════════════════════════════════════════════
    // STEP 1: TIER 2 POOL (Swap ÉTR → wrapped)
    // ═══════════════════════════════════════════════════════════════

    // ÉTR already transferred to this contract by IntentRouter
    IERC20(intent.sourceToken).approve(intent.route.tier2Pool, intent.amountIn);

    uint256 wrappedReceived = ITier2Pool(intent.route.tier2Pool).swapETRForWrapped(
        intent.amountIn,
        intent.minAmountOut  // Min wrapped tokens
    );

    // ═══════════════════════════════════════════════════════════════
    // STEP 2: TIER 1 POOL (Burn wrapped, prepare release)
    // ═══════════════════════════════════════════════════════════════

    IERC20(intent.route.wrappedToken).approve(
        intent.route.tier1Pool,
        wrappedReceived
    );

    amountOut = ITier1Pool(intent.route.tier1Pool).burnAndRelease(
        wrappedReceived,
        externalAddress  // External chain address to receive funds
    );

    // ═══════════════════════════════════════════════════════════════
    // STEP 3: BRIDGE WITHDRAWAL (Request release on external chain)
    // ═══════════════════════════════════════════════════════════════

    bytes32 withdrawalId = IBridgeTracker(intent.route.bridgeTracker).recordWithdrawal(
        intent.user,
        amountOut,
        externalAddress
    );

    // Withdrawal will be processed by multi-sig on external chain
    // User can track status via withdrawalId

    emit ReverseSwapExecuted(
        intent.user,
        intent.sourceToken,
        intent.destToken,
        intent.amountIn,
        amountOut,
        withdrawalId
    );

    return amountOut;
}
```

---

### 3. TwoTierBridgeRouter.sol

**Location:** `contracts/intent-router/core/TwoTierBridgeRouter.sol`

**Purpose:** Route messages between Bridge → Tier 1 → Tier 2

**Core Functionality:**
```solidity
/**
 * @notice Route verified bridge deposit to appropriate pools
 * Called by BridgeTracker after attestation verified
 */
function routeBridgeDeposit(
    bytes32 depositId,
    address user,
    address externalCurrency,
    uint256 amount
) external onlyBridgeTracker {
    // 1. Get pool addresses
    address tier1Pool = tier1Pools[externalCurrency];
    address tier2Pool = tier2Pools[externalCurrency];
    address wrappedToken = wrappedTokens[externalCurrency];

    require(tier1Pool != address(0), "No Tier 1 pool");
    require(tier2Pool != address(0), "No Tier 2 pool");

    // 2. Notify Tier 1 pool
    ITier1Pool(tier1Pool).onBridgeDeposit(depositId, user, amount);

    // 3. Verify wrapped tokens minted
    uint256 wrappedBalance = IERC20(wrappedToken).balanceOf(tier2Pool);
    require(wrappedBalance >= amount, "Wrapped minting failed");

    // 4. Emit routing event
    emit DepositRouted(depositId, user, tier1Pool, tier2Pool, amount);
}

/**
 * @notice Route withdrawal request from Tier 1 back to bridge
 */
function routeWithdrawal(
    bytes32 withdrawalId,
    address user,
    uint256 amount,
    bytes32 externalAddress
) external onlyTier1Pool {
    address bridgeTracker = bridgeTrackerForPool[msg.sender];
    require(bridgeTracker != address(0), "Unknown pool");

    // Forward to bridge tracker
    IBridgeTracker(bridgeTracker).confirmWithdrawalRequest(
        withdrawalId,
        user,
        amount,
        externalAddress
    );

    emit WithdrawalRouted(withdrawalId, user, bridgeTracker, amount);
}
```

---

## User Experience Flows

### Flow 1: Buy ÉTR with BTC (Deposit)

```
┌─────────────────────────────────────────────────────────────┐
│ USER ACTION: "Buy 12,487 ÉTR with 0.5 BTC"                 │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: User calls IntentRouter.convertToEtr()             │
│ ├─ sourceCurrency: BTC                                     │
│ ├─ amount: 0.5 BTC                                         │
│ ├─ minEtrOut: 12,400 ÉTR (0.7% slippage tolerance)        │
│ └─ deadline: timestamp + 10 minutes                        │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: IntentRouter.getQuote() estimates output           │
│ ├─ Route: BTC → wBTC (Tier 1) → ÉTR (Tier 2)             │
│ ├─ Estimated: 12,487 ÉTR                                  │
│ ├─ Fee: 37.4 ÉTR (0.3%)                                   │
│ └─ User sees: "You will receive ~12,450 ÉTR"              │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: User approves, locks 0.5 BTC on Bitcoin            │
│ ├─ Send to multi-sig address: bc1q...                     │
│ ├─ Transaction confirmed (6 blocks)                       │
│ └─ Tx hash: 0xabc123...                                   │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 4: AutoSwapExecutor.executeSwap() (automatic)         │
│ ├─ Bridge verifies deposit (3-of-5 attestation)           │
│ ├─ Tier 1: Lock 0.5 BTC, mint 0.5 wBTC                   │
│ ├─ Tier 2: Swap 0.5 wBTC → 12,487 ÉTR                    │
│ └─ Transfer 12,450 ÉTR to user (after 0.3% fee)          │
└─────────────────────────────────────────────────────────────┘
                         ↓
         User wallet: +12,450 ÉTR ✓
         wBTC never seen by user ✓
         Single transaction experience ✓
```

### Flow 2: Sell ÉTR for BTC (Withdrawal)

```
┌─────────────────────────────────────────────────────────────┐
│ USER ACTION: "Sell 12,487 ÉTR for BTC"                     │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: User calls IntentRouter.convertFromEtr()           │
│ ├─ targetCurrency: BTC                                     │
│ ├─ etrAmount: 12,487 ÉTR                                  │
│ ├─ minCurrencyOut: 0.49 BTC (2% slippage)                 │
│ └─ externalAddress: bc1q... (user's Bitcoin address)      │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: AutoSwapExecutor.executeReverseSwap()              │
│ ├─ Tier 2: Swap 12,487 ÉTR → 0.5 wBTC                    │
│ ├─ Tier 1: Burn 0.5 wBTC                                  │
│ └─ Bridge: Request 0.5 BTC release to bc1q...             │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: Multi-sig approval (4-of-7 validators)             │
│ ├─ Withdrawal request verified                            │
│ ├─ Multi-sig signs Bitcoin transaction                    │
│ └─ 0.5 BTC sent to user's address                         │
└─────────────────────────────────────────────────────────────┘
                         ↓
         User receives: 0.5 BTC on Bitcoin ✓
         Withdrawal ID for tracking ✓
         Estimated time: 30 minutes ✓
```

---

## Cross-PBC Routing (EDSC Integration)

### Routing Stablecoin Purchases to EDSC Reserve

**Scenario:** User buys USDC on ETH-PBC, should route to EDSC reserve

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│ USER ON ETH-PBC: Swaps ETH → USDC                          │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ ETH-PBC Runtime detects stablecoin purchase                │
│ ├─ Event: StablecoinPurchased(user, USDC, 500)            │
│ └─ Trigger: StablecoinRouter pallet                       │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ StablecoinRouter.routeToEDSC()                             │
│ ├─ Send XCMP message to Primearc                          │
│ ├─ Payload: {user, USDC, 500, sourcePBC: ETH}             │
│ └─ Forward to EDSC-PBC via HRMP                           │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ EDSC-PBC receives routing request                          │
│ ├─ EDSCMintingEngine.routeStablecoinPurchase()            │
│ ├─ User prompt: "Convert USDC → EDSC?" (optional)         │
│ └─ If yes: Add to reserve, mint EDSC                      │
└─────────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ EDSCReserveVault.depositUSDC(500)                          │
│ ├─ Reserve: +500 USDC                                     │
│ ├─ Mint: 500 EDSC to user                                 │
│ └─ Send EDSC back to ETH-PBC via XCMP                     │
└─────────────────────────────────────────────────────────────┘
                         ↓
         User on ETH-PBC receives: 500 EDSC ✓
         Reserve backing: +500 USDC ✓
```

**Implementation:**
```solidity
// In IntentRouter.sol
function routeStablecoinToEDSC(
    address stablecoin,
    uint256 amount,
    uint32 sourcePBC
) external returns (uint256 edscMinted) {
    require(
        isStablecoin(stablecoin),
        "Not a stablecoin"
    );

    // 1. Transfer stablecoin to this contract
    IERC20(stablecoin).transferFrom(msg.sender, address(this), amount);

    // 2. If volatile asset, swap to USDC first
    if (stablecoin != USDC && stablecoin != USDT) {
        amount = swapToUSDC(stablecoin, amount);
        stablecoin = USDC;
    }

    // 3. Send cross-PBC message to EDSC-PBC
    bytes memory payload = abi.encode(
        msg.sender,
        stablecoin,
        amount,
        sourcePBC
    );

    sendXCMPMessage(
        EDSC_PBC_ID,
        payload,
        XCMPMessageType.StablecoinRouting
    );

    // 4. Await response (EDSC minted amount)
    // This happens asynchronously via XCMP callback

    emit StablecoinRoutedToEDSC(
        msg.sender,
        stablecoin,
        amount,
        sourcePBC
    );

    return amount; // 1:1 EDSC minting
}
```

---

## Security Features

### 1. Atomic Execution
```solidity
// All steps must succeed or entire transaction reverts
modifier atomic() {
    uint256 snapshot = vm.snapshot();
    _;
    if (!success) {
        vm.revertTo(snapshot);
    }
}
```

### 2. Slippage Protection
```solidity
require(amountOut >= minAmountOut, "Slippage exceeded");
```

### 3. Deadline Enforcement
```solidity
require(block.timestamp <= deadline, "Transaction expired");
```

### 4. Reentrancy Protection
```solidity
modifier nonReentrant() {
    require(!locked, "Reentrant call");
    locked = true;
    _;
    locked = false;
}
```

### 5. Access Control
```solidity
modifier onlyIntentRouter() {
    require(msg.sender == intentRouter, "Unauthorized");
    _;
}
```

### 6. Rate Limiting
```solidity
mapping(address => uint256) public lastSwapTime;
uint256 public constant MIN_SWAP_INTERVAL = 10; // 10 seconds

modifier rateLimit() {
    require(
        block.timestamp >= lastSwapTime[msg.sender] + MIN_SWAP_INTERVAL,
        "Swap too frequent"
    );
    lastSwapTime[msg.sender] = block.timestamp;
    _;
}
```

---

## File Structure

```
contracts/intent-router/
├── INTENT_ROUTER_ARCHITECTURE.md (this file)
│
├── core/
│   ├── IntentRouter.sol
│   ├── AutoSwapExecutor.sol
│   └── TwoTierBridgeRouter.sol
│
├── routing/
│   ├── RouteOptimizer.sol       // Find best swap route
│   ├── StablecoinRouter.sol     // Route stablecoins to EDSC
│   └── PriceOracle.sol          // Price feeds for quotes
│
├── interfaces/
│   ├── IIntentRouter.sol
│   ├── IAutoSwapExecutor.sol
│   ├── ITwoTierBridgeRouter.sol
│   ├── ITier1Pool.sol
│   └── ITier2Pool.sol
│
├── libraries/
│   ├── SwapMath.sol             // AMM calculations
│   ├── RouteEncoding.sol        // Route compression
│   └── SlippageCalculator.sol   // Dynamic slippage
│
└── test/
    ├── IntentRouter.test.js
    ├── AutoSwapExecutor.test.js
    └── integration/
        ├── FullSwapFlow.test.js
        └── CrossPBCRouting.test.js
```

---

## Integration Points

### 1. Bridge System
```solidity
interface IBridgeTracker {
    function recordDeposit(...) external returns (bytes32);
    function isVerified(bytes32 depositId) external view returns (bool);
    function recordWithdrawal(...) external returns (bytes32);
}
```

### 2. Tier 1 Pools
```solidity
interface ITier1Pool {
    function lockAndMint(uint256 amount) external returns (uint256);
    function burnAndRelease(uint256 amount, bytes32 target) external returns (uint256);
    function onBridgeDeposit(bytes32 depositId, address user, uint256 amount) external;
}
```

### 3. Tier 2 Pools
```solidity
interface ITier2Pool {
    function swapETRForWrapped(uint256 etrIn, uint256 minOut) external returns (uint256);
    function swapWrappedForETR(uint256 wrappedIn, uint256 minOut) external returns (uint256);
    function getAmountOut(uint256 amountIn, address tokenIn, address tokenOut) external view returns (uint256);
}
```

### 4. EDSC Minting Engine
```solidity
interface IEDSCMintingEngine {
    function routeStablecoinPurchase(
        uint32 sourcePBC,
        address user,
        address stablecoin,
        uint256 amount
    ) external returns (uint256 edscMinted);
}
```

---

## Implementation Checklist

**Phase 1: Core Contracts**
- [ ] Deploy IntentRouter.sol
- [ ] Deploy AutoSwapExecutor.sol
- [ ] Deploy TwoTierBridgeRouter.sol

**Phase 2: Routing**
- [ ] Deploy RouteOptimizer.sol
- [ ] Deploy StablecoinRouter.sol
- [ ] Configure route mappings (11 currencies)

**Phase 3: Integration**
- [ ] Wire IntentRouter → AutoSwapExecutor
- [ ] Wire AutoSwapExecutor → Tier 1 pools
- [ ] Wire AutoSwapExecutor → Tier 2 pools
- [ ] Wire TwoTierBridgeRouter → BridgeTrackers
- [ ] Wire StablecoinRouter → EDSC-PBC

**Phase 4: Testing**
- [ ] Unit tests for each contract
- [ ] Integration test: Full BTC → ÉTR flow
- [ ] Integration test: Full ÉTR → BTC flow
- [ ] Cross-PBC routing test (stablecoin → EDSC)

---

**Status:** Architecture defined, ready for implementation
**Next:** Implementation phase with parallel contract coding
