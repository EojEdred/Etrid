// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EDSCPegPool
 * @dev Single-sided stablecoin pool for EDSC/USDT (1:1 peg)
 *
 * KEY: EDSC is seeded, USDT is added by users
 *
 * Flow:
 * 1. Protocol seeds pool with EDSC (e.g., 10M EDSC)
 * 2. Users swap USDT → EDSC (USDT enters pool)
 * 3. Users swap EDSC → USDT (if USDT exists in pool)
 * 4. As USDT builds, pool becomes balanced
 * 5. StableSwap pricing activates when both sides have liquidity
 *
 * Pricing:
 * - Before USDT exists: 1:1 fixed rate (minus fee)
 * - After USDT exists: StableSwap (Curve-style) for low slippage
 */
contract EDSCPegPool is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    // Pool tokens
    IERC20 public immutable edsc;      // EDSC stablecoin (seeded)
    IERC20 public immutable usdt;      // USDT stablecoin (added by users)

    // Pool state
    uint256 public reserveEDSC;
    uint256 public reserveUSDT;
    uint256 public initialEDSC;        // Track initial seed amount

    // StableSwap parameters
    uint256 public amplificationCoefficient = 100;  // A parameter
    uint256 public constant FEE_NUMERATOR = 4;       // 0.04% fee
    uint256 public constant FEE_DENOMINATOR = 10000;
    uint256 public constant PRECISION = 1e18;

    // Decimal handling (EDSC: 18, USDT: 6)
    uint256 public immutable edscDecimals;
    uint256 public immutable usdtDecimals;

    // Transition to StableSwap
    uint256 public stableSwapThreshold;    // USDT amount to activate StableSwap
    bool public useStableSwap;

    // LP tokens
    uint256 public totalLPTokens;
    mapping(address => uint256) public lpBalanceOf;

    // Events
    event PoolInitialized(uint256 edscAmount);
    event Swap(
        address indexed user,
        address indexed tokenIn,
        address indexed tokenOut,
        uint256 amountIn,
        uint256 amountOut
    );
    event LiquidityAdded(address indexed provider, uint256 edscAmount, uint256 usdtAmount);
    event LiquidityRemoved(address indexed provider, uint256 edscAmount, uint256 usdtAmount);
    event StableSwapActivated(uint256 reserveEDSC, uint256 reserveUSDT);
    event PegDeviation(uint256 edscPrice, uint256 deviationPercent);

    constructor(
        address _edsc,
        address _usdt,
        uint256 _edscDecimals,
        uint256 _usdtDecimals,
        uint256 _stableSwapThreshold
    ) Ownable(msg.sender) {
        edsc = IERC20(_edsc);
        usdt = IERC20(_usdt);
        edscDecimals = _edscDecimals;
        usdtDecimals = _usdtDecimals;
        stableSwapThreshold = _stableSwapThreshold;
    }

    /**
     * @dev Initialize pool with EDSC only (single-sided)
     * @param edscAmount EDSC to seed
     */
    function initializePool(uint256 edscAmount) external onlyOwner {
        require(totalLPTokens == 0, "Pool already initialized");
        require(edscAmount > 0, "Amount must be > 0");

        // Transfer EDSC from protocol
        edsc.safeTransferFrom(msg.sender, address(this), edscAmount);

        reserveEDSC = edscAmount;
        initialEDSC = edscAmount;
        reserveUSDT = 0;  // No USDT initially

        // Initial LP tokens = EDSC amount
        totalLPTokens = edscAmount;
        lpBalanceOf[msg.sender] = edscAmount;

        emit PoolInitialized(edscAmount);
    }

    /**
     * @dev Normalize USDT to 18 decimals
     */
    function _normalizeUSDT(uint256 amount) internal view returns (uint256) {
        return amount * (10 ** (18 - usdtDecimals));
    }

    function _denormalizeUSDT(uint256 amount) internal view returns (uint256) {
        return amount / (10 ** (18 - usdtDecimals));
    }

    /**
     * @dev Calculate StableSwap output (Curve-style)
     */
    function _getStableSwapOutput(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) internal view returns (uint256 amountOut) {
        // Simplified StableSwap: very low slippage for pegged assets
        // Uses amplified constant product: (A*n^n + D/n^n) * x * y = k

        uint256 A = amplificationCoefficient;
        uint256 D = reserveIn + reserveOut;  // Invariant for 1:1 peg

        // For stablecoins, use near-linear pricing with slight curve
        // amountOut ≈ amountIn * (1 - fee) * (reserveOut / (reserveIn + amountIn))
        // But with high A, it's nearly 1:1

        uint256 newReserveIn = reserveIn + amountIn;

        // Newton's method for StableSwap
        uint256 y = reserveOut;
        for (uint256 i = 0; i < 255; i++) {
            uint256 yPrev = y;

            // y = (A * D + D^2 / (2 * x * y)) / (A + D / (2 * y))
            uint256 numerator = A * D + (D * D) / (2 * newReserveIn);
            uint256 denominator = A + D / (2 * y);
            y = numerator / denominator;

            if (y > yPrev) {
                if (y - yPrev <= 1) break;
            } else {
                if (yPrev - y <= 1) break;
            }
        }

        amountOut = reserveOut > y ? reserveOut - y : 0;
    }

    /**
     * @dev Swap USDT for EDSC (user adds USDT, receives EDSC)
     * @param usdtAmountIn USDT amount to swap (6 decimals)
     * @param minEDSCOut Minimum EDSC to receive (18 decimals)
     */
    function swapUSDTForEDSC(
        uint256 usdtAmountIn,
        uint256 minEDSCOut
    ) external nonReentrant returns (uint256 edscOut) {
        require(usdtAmountIn > 0, "Amount must be > 0");

        // Normalize USDT to 18 decimals for calculation
        uint256 normalizedUSDT = _normalizeUSDT(usdtAmountIn);

        if (useStableSwap && reserveUSDT > 0) {
            // Use StableSwap pricing
            uint256 normalizedReserveUSDT = _normalizeUSDT(reserveUSDT);
            edscOut = _getStableSwapOutput(normalizedUSDT, normalizedReserveUSDT, reserveEDSC);
        } else {
            // 1:1 fixed rate (before USDT liquidity builds)
            edscOut = normalizedUSDT;
        }

        // Apply fee
        edscOut = edscOut - (edscOut * FEE_NUMERATOR / FEE_DENOMINATOR);

        require(edscOut >= minEDSCOut, "Slippage exceeded");
        require(edscOut <= reserveEDSC, "Insufficient EDSC");

        // Transfer USDT in
        usdt.safeTransferFrom(msg.sender, address(this), usdtAmountIn);

        // Transfer EDSC out
        edsc.safeTransfer(msg.sender, edscOut);

        // Update reserves
        reserveUSDT += usdtAmountIn;
        reserveEDSC -= edscOut;

        // Check if StableSwap should activate
        _checkStableSwapActivation();

        emit Swap(msg.sender, address(usdt), address(edsc), usdtAmountIn, edscOut);
    }

    /**
     * @dev Swap EDSC for USDT (requires USDT in pool)
     * @param edscAmountIn EDSC amount to swap (18 decimals)
     * @param minUSDTOut Minimum USDT to receive (6 decimals)
     */
    function swapEDSCForUSDT(
        uint256 edscAmountIn,
        uint256 minUSDTOut
    ) external nonReentrant returns (uint256 usdtOut) {
        require(edscAmountIn > 0, "Amount must be > 0");
        require(reserveUSDT > 0, "No USDT liquidity yet");

        uint256 normalizedOut;

        if (useStableSwap) {
            // Use StableSwap pricing
            uint256 normalizedReserveUSDT = _normalizeUSDT(reserveUSDT);
            normalizedOut = _getStableSwapOutput(edscAmountIn, reserveEDSC, normalizedReserveUSDT);
        } else {
            // 1:1 fixed rate
            normalizedOut = edscAmountIn;
        }

        // Apply fee
        normalizedOut = normalizedOut - (normalizedOut * FEE_NUMERATOR / FEE_DENOMINATOR);

        // Denormalize to USDT decimals
        usdtOut = _denormalizeUSDT(normalizedOut);

        require(usdtOut >= minUSDTOut, "Slippage exceeded");
        require(usdtOut <= reserveUSDT, "Insufficient USDT");

        // Transfer EDSC in
        edsc.safeTransferFrom(msg.sender, address(this), edscAmountIn);

        // Transfer USDT out
        usdt.safeTransfer(msg.sender, usdtOut);

        // Update reserves
        reserveEDSC += edscAmountIn;
        reserveUSDT -= usdtOut;

        emit Swap(msg.sender, address(edsc), address(usdt), edscAmountIn, usdtOut);
    }

    /**
     * @dev Check if StableSwap should activate
     */
    function _checkStableSwapActivation() internal {
        if (!useStableSwap && reserveUSDT >= stableSwapThreshold) {
            useStableSwap = true;
            emit StableSwapActivated(reserveEDSC, reserveUSDT);
        }
    }

    /**
     * @dev Add liquidity (both tokens, after USDT exists)
     */
    function addLiquidity(
        uint256 edscAmount,
        uint256 usdtAmount,
        uint256 minLPTokens
    ) external nonReentrant returns (uint256 lpTokens) {
        require(totalLPTokens > 0, "Pool not initialized");

        // Calculate LP tokens based on contribution
        uint256 normalizedUSDT = _normalizeUSDT(usdtAmount);
        uint256 normalizedReserveUSDT = _normalizeUSDT(reserveUSDT);

        uint256 totalValue = reserveEDSC + normalizedReserveUSDT;
        uint256 addedValue = edscAmount + normalizedUSDT;

        if (totalValue == 0) {
            lpTokens = addedValue;
        } else {
            lpTokens = (addedValue * totalLPTokens) / totalValue;
        }

        require(lpTokens >= minLPTokens, "Insufficient LP tokens");

        // Transfer tokens
        if (edscAmount > 0) {
            edsc.safeTransferFrom(msg.sender, address(this), edscAmount);
            reserveEDSC += edscAmount;
        }
        if (usdtAmount > 0) {
            usdt.safeTransferFrom(msg.sender, address(this), usdtAmount);
            reserveUSDT += usdtAmount;
        }

        totalLPTokens += lpTokens;
        lpBalanceOf[msg.sender] += lpTokens;

        _checkStableSwapActivation();

        emit LiquidityAdded(msg.sender, edscAmount, usdtAmount);
    }

    /**
     * @dev Remove liquidity
     */
    function removeLiquidity(
        uint256 lpTokensIn,
        uint256 minEDSC,
        uint256 minUSDT
    ) external nonReentrant returns (uint256 edscOut, uint256 usdtOut) {
        require(lpTokensIn > 0, "LP tokens must be > 0");
        require(lpBalanceOf[msg.sender] >= lpTokensIn, "Insufficient LP balance");

        // Calculate proportional share
        edscOut = (lpTokensIn * reserveEDSC) / totalLPTokens;
        usdtOut = (lpTokensIn * reserveUSDT) / totalLPTokens;

        require(edscOut >= minEDSC, "Insufficient EDSC output");
        require(usdtOut >= minUSDT, "Insufficient USDT output");

        // Update state
        lpBalanceOf[msg.sender] -= lpTokensIn;
        totalLPTokens -= lpTokensIn;
        reserveEDSC -= edscOut;
        reserveUSDT -= usdtOut;

        // Transfer tokens
        if (edscOut > 0) {
            edsc.safeTransfer(msg.sender, edscOut);
        }
        if (usdtOut > 0) {
            usdt.safeTransfer(msg.sender, usdtOut);
        }

        emit LiquidityRemoved(msg.sender, edscOut, usdtOut);
    }

    /**
     * @dev Get current EDSC price relative to USDT (1e18 = $1)
     */
    function getEDSCPrice() external view returns (uint256) {
        if (reserveUSDT == 0) return PRECISION;  // 1:1 before USDT exists

        uint256 normalizedUSDT = _normalizeUSDT(reserveUSDT);
        if (reserveEDSC == 0) return PRECISION;

        return (normalizedUSDT * PRECISION) / reserveEDSC;
    }

    /**
     * @dev Get pool info
     */
    function getPoolInfo() external view returns (
        uint256 _reserveEDSC,
        uint256 _reserveUSDT,
        uint256 _totalLPTokens,
        bool _useStableSwap,
        uint256 _edscPrice
    ) {
        uint256 price;
        if (reserveUSDT == 0 || reserveEDSC == 0) {
            price = PRECISION;
        } else {
            uint256 normalizedUSDT = _normalizeUSDT(reserveUSDT);
            price = (normalizedUSDT * PRECISION) / reserveEDSC;
        }

        return (
            reserveEDSC,
            reserveUSDT,
            totalLPTokens,
            useStableSwap,
            price
        );
    }

    /**
     * @dev Check if USDT → EDSC swap is available
     */
    function canSwapUSDTForEDSC() external view returns (bool) {
        return reserveEDSC > 0;
    }

    /**
     * @dev Check if EDSC → USDT swap is available
     */
    function canSwapEDSCForUSDT() external view returns (bool) {
        return reserveUSDT > 0;
    }

    /**
     * @dev Update configuration (governance)
     */
    function setAmplification(uint256 newA) external onlyOwner {
        require(newA >= 1 && newA <= 1000, "A out of range");
        amplificationCoefficient = newA;
    }

    function setStableSwapThreshold(uint256 _threshold) external onlyOwner {
        stableSwapThreshold = _threshold;
    }
}
