// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title VirtualReserveAMM
 * @dev AMM pool that starts with single-sided liquidity (ETR only)
 *
 * Key Features:
 * - Pools start with only ETR seeded by Foundation
 * - Virtual reserves establish initial price via oracle
 * - As users buy ETR with other tokens, real liquidity builds
 * - Virtual reserves phase out as real liquidity grows
 * - Price naturally adjusts via AMM formula (x * y = k)
 *
 * Flow:
 * 1. Foundation seeds pool with ETR
 * 2. Oracle sets initial ETR price -> virtual paired reserve calculated
 * 3. Users buy ETR with wBTC/wETH/etc -> real paired tokens enter pool
 * 4. Once real liquidity > threshold, virtual reserves = 0
 * 5. Pure AMM pricing takes over
 */
contract VirtualReserveAMM is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    // Pool tokens
    IERC20 public immutable etr;           // ETR token (base)
    IERC20 public immutable pairedToken;   // Paired token (wBTC, wETH, etc.)

    // Pool state
    uint256 public reserveETR;             // Real ETR in pool
    uint256 public reservePaired;          // Real paired token in pool
    uint256 public virtualPaired;          // Virtual paired reserve (phases out)

    // Oracle integration
    address public oracle;                  // Price oracle address
    uint256 public lastOraclePrice;         // Last fetched price (ETR in USD, 8 decimals)
    uint256 public lastOracleUpdate;        // Timestamp of last update

    // Configuration
    uint256 public constant FEE_NUMERATOR = 3;      // 0.3% fee
    uint256 public constant FEE_DENOMINATOR = 1000;
    uint256 public constant PRICE_DECIMALS = 8;
    uint256 public virtualPhaseOutThreshold;        // When real liquidity exceeds this, virtual = 0

    // LP tracking (simplified - no LP tokens, Foundation owns all)
    uint256 public totalLiquidity;
    mapping(address => uint256) public liquidityOf;

    // Events
    event Swap(
        address indexed user,
        bool etrToToken,
        uint256 amountIn,
        uint256 amountOut,
        uint256 newReserveETR,
        uint256 newReservePaired
    );
    event LiquidityAdded(address indexed provider, uint256 etrAmount, uint256 pairedAmount);
    event LiquidityRemoved(address indexed provider, uint256 etrAmount, uint256 pairedAmount);
    event OraclePriceUpdated(uint256 newPrice, uint256 newVirtualReserve);
    event VirtualReservePhaseOut(uint256 finalRealReserve);

    constructor(
        address _etr,
        address _pairedToken,
        address _oracle,
        uint256 _virtualPhaseOutThreshold
    ) Ownable(msg.sender) {
        etr = IERC20(_etr);
        pairedToken = IERC20(_pairedToken);
        oracle = _oracle;
        virtualPhaseOutThreshold = _virtualPhaseOutThreshold;
    }

    /**
     * @dev Initialize pool with single-sided ETR liquidity
     * @param etrAmount Amount of ETR to seed
     * @param initialPrice Initial ETR price in USD (8 decimals)
     * @param pairedTokenPrice Price of paired token in USD (8 decimals)
     */
    function initializePool(
        uint256 etrAmount,
        uint256 initialPrice,
        uint256 pairedTokenPrice
    ) external onlyOwner {
        require(reserveETR == 0, "Pool already initialized");
        require(etrAmount > 0, "ETR amount must be > 0");
        require(initialPrice > 0, "Price must be > 0");
        require(pairedTokenPrice > 0, "Paired price must be > 0");

        // Transfer ETR from Foundation
        etr.safeTransferFrom(msg.sender, address(this), etrAmount);
        reserveETR = etrAmount;

        // Calculate virtual paired reserve based on oracle price
        // virtualPaired = (etrAmount * etrPrice) / pairedTokenPrice
        // This establishes the initial price ratio
        virtualPaired = (etrAmount * initialPrice) / pairedTokenPrice;

        lastOraclePrice = initialPrice;
        lastOracleUpdate = block.timestamp;

        // Foundation is initial liquidity provider
        totalLiquidity = etrAmount;
        liquidityOf[msg.sender] = etrAmount;

        emit LiquidityAdded(msg.sender, etrAmount, 0);
        emit OraclePriceUpdated(initialPrice, virtualPaired);
    }

    /**
     * @dev Get effective reserves (real + virtual)
     */
    function getEffectiveReserves() public view returns (uint256 effectiveETR, uint256 effectivePaired) {
        effectiveETR = reserveETR;
        effectivePaired = reservePaired + virtualPaired;
    }

    /**
     * @dev Calculate output amount for a swap
     * @param amountIn Input amount
     * @param reserveIn Input reserve
     * @param reserveOut Output reserve
     */
    function getAmountOut(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) public pure returns (uint256 amountOut) {
        require(amountIn > 0, "Insufficient input amount");
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");

        // Apply 0.3% fee
        uint256 amountInWithFee = amountIn * (FEE_DENOMINATOR - FEE_NUMERATOR);
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = (reserveIn * FEE_DENOMINATOR) + amountInWithFee;
        amountOut = numerator / denominator;
    }

    /**
     * @dev Swap paired token for ETR (user buys ETR)
     * @param pairedAmountIn Amount of paired token to swap
     * @param minEtrOut Minimum ETR to receive (slippage protection)
     */
    function swapPairedForETR(
        uint256 pairedAmountIn,
        uint256 minEtrOut
    ) external nonReentrant returns (uint256 etrOut) {
        require(pairedAmountIn > 0, "Amount must be > 0");

        (uint256 effectiveETR, uint256 effectivePaired) = getEffectiveReserves();

        // Calculate ETR output
        etrOut = getAmountOut(pairedAmountIn, effectivePaired, effectiveETR);
        require(etrOut >= minEtrOut, "Slippage exceeded");
        require(etrOut <= reserveETR, "Insufficient ETR in pool");

        // Transfer paired token in
        pairedToken.safeTransferFrom(msg.sender, address(this), pairedAmountIn);

        // Transfer ETR out
        etr.safeTransfer(msg.sender, etrOut);

        // Update reserves
        reservePaired += pairedAmountIn;
        reserveETR -= etrOut;

        // Check if virtual reserves should phase out
        _checkVirtualPhaseOut();

        emit Swap(msg.sender, false, pairedAmountIn, etrOut, reserveETR, reservePaired);
    }

    /**
     * @dev Swap ETR for paired token (user sells ETR)
     * @param etrAmountIn Amount of ETR to swap
     * @param minPairedOut Minimum paired token to receive
     */
    function swapETRForPaired(
        uint256 etrAmountIn,
        uint256 minPairedOut
    ) external nonReentrant returns (uint256 pairedOut) {
        require(etrAmountIn > 0, "Amount must be > 0");

        (uint256 effectiveETR, uint256 effectivePaired) = getEffectiveReserves();

        // Calculate paired token output
        pairedOut = getAmountOut(etrAmountIn, effectiveETR, effectivePaired);
        require(pairedOut >= minPairedOut, "Slippage exceeded");

        // Can only output real paired tokens, not virtual
        require(pairedOut <= reservePaired, "Insufficient paired tokens in pool");

        // Transfer ETR in
        etr.safeTransferFrom(msg.sender, address(this), etrAmountIn);

        // Transfer paired token out
        pairedToken.safeTransfer(msg.sender, pairedOut);

        // Update reserves
        reserveETR += etrAmountIn;
        reservePaired -= pairedOut;

        emit Swap(msg.sender, true, etrAmountIn, pairedOut, reserveETR, reservePaired);
    }

    /**
     * @dev Update oracle price (called by oracle or authorized updater)
     * @param newPrice New ETR price in USD (8 decimals)
     * @param pairedTokenPrice Current paired token price in USD (8 decimals)
     */
    function updateOraclePrice(
        uint256 newPrice,
        uint256 pairedTokenPrice
    ) external {
        require(msg.sender == oracle || msg.sender == owner(), "Not authorized");
        require(newPrice > 0 && pairedTokenPrice > 0, "Invalid prices");

        // Only update virtual reserves if they haven't phased out
        if (virtualPaired > 0) {
            // Recalculate virtual reserve based on remaining ETR and new price
            // But cap it to prevent manipulation
            uint256 newVirtual = (reserveETR * newPrice) / pairedTokenPrice;

            // Virtual can only decrease (as real liquidity builds), never increase
            if (newVirtual < virtualPaired) {
                virtualPaired = newVirtual;
            }
        }

        lastOraclePrice = newPrice;
        lastOracleUpdate = block.timestamp;

        emit OraclePriceUpdated(newPrice, virtualPaired);
    }

    /**
     * @dev Check if virtual reserves should phase out
     */
    function _checkVirtualPhaseOut() internal {
        if (virtualPaired > 0 && reservePaired >= virtualPhaseOutThreshold) {
            // Real liquidity exceeds threshold, phase out virtual
            virtualPaired = 0;
            emit VirtualReservePhaseOut(reservePaired);
        }
    }

    /**
     * @dev Add liquidity (both tokens required after initialization)
     */
    function addLiquidity(
        uint256 etrAmount,
        uint256 pairedAmount,
        uint256 minLiquidity
    ) external nonReentrant returns (uint256 liquidity) {
        require(reserveETR > 0, "Pool not initialized");

        (uint256 effectiveETR, uint256 effectivePaired) = getEffectiveReserves();

        // Calculate optimal amounts based on current ratio
        uint256 optimalPaired = (etrAmount * effectivePaired) / effectiveETR;
        uint256 optimalETR = (pairedAmount * effectiveETR) / effectivePaired;

        uint256 actualETR;
        uint256 actualPaired;

        if (optimalPaired <= pairedAmount) {
            actualETR = etrAmount;
            actualPaired = optimalPaired;
        } else {
            actualETR = optimalETR;
            actualPaired = pairedAmount;
        }

        // Calculate liquidity tokens
        liquidity = (actualETR * totalLiquidity) / reserveETR;
        require(liquidity >= minLiquidity, "Insufficient liquidity minted");

        // Transfer tokens
        etr.safeTransferFrom(msg.sender, address(this), actualETR);
        pairedToken.safeTransferFrom(msg.sender, address(this), actualPaired);

        // Update state
        reserveETR += actualETR;
        reservePaired += actualPaired;
        totalLiquidity += liquidity;
        liquidityOf[msg.sender] += liquidity;

        emit LiquidityAdded(msg.sender, actualETR, actualPaired);
    }

    /**
     * @dev Remove liquidity
     */
    function removeLiquidity(
        uint256 liquidity,
        uint256 minETR,
        uint256 minPaired
    ) external nonReentrant returns (uint256 etrAmount, uint256 pairedAmount) {
        require(liquidity > 0, "Liquidity must be > 0");
        require(liquidityOf[msg.sender] >= liquidity, "Insufficient liquidity");

        // Calculate token amounts
        etrAmount = (liquidity * reserveETR) / totalLiquidity;
        pairedAmount = (liquidity * reservePaired) / totalLiquidity;

        require(etrAmount >= minETR, "Insufficient ETR output");
        require(pairedAmount >= minPaired, "Insufficient paired output");

        // Update state
        liquidityOf[msg.sender] -= liquidity;
        totalLiquidity -= liquidity;
        reserveETR -= etrAmount;
        reservePaired -= pairedAmount;

        // Transfer tokens
        etr.safeTransfer(msg.sender, etrAmount);
        if (pairedAmount > 0) {
            pairedToken.safeTransfer(msg.sender, pairedAmount);
        }

        emit LiquidityRemoved(msg.sender, etrAmount, pairedAmount);
    }

    /**
     * @dev Get current spot price (ETR per paired token)
     */
    function getSpotPrice() external view returns (uint256) {
        (uint256 effectiveETR, uint256 effectivePaired) = getEffectiveReserves();
        if (effectivePaired == 0) return 0;
        return (effectiveETR * 1e18) / effectivePaired;
    }

    /**
     * @dev Get pool info
     */
    function getPoolInfo() external view returns (
        uint256 _reserveETR,
        uint256 _reservePaired,
        uint256 _virtualPaired,
        uint256 _totalLiquidity,
        uint256 _lastOraclePrice,
        bool _virtualActive
    ) {
        return (
            reserveETR,
            reservePaired,
            virtualPaired,
            totalLiquidity,
            lastOraclePrice,
            virtualPaired > 0
        );
    }

    /**
     * @dev Update oracle address
     */
    function setOracle(address _oracle) external onlyOwner {
        oracle = _oracle;
    }

    /**
     * @dev Update phase out threshold
     */
    function setPhaseOutThreshold(uint256 _threshold) external onlyOwner {
        virtualPhaseOutThreshold = _threshold;
    }
}
