// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EDSCOraclePool
 * @dev Special pool for ETR/EDSC trading
 *
 * KEY DIFFERENCE FROM OTHER POOLS:
 * - EDSC is seeded first (by protocol minting)
 * - Users add ETR to receive EDSC
 * - Oracle determines ETR price, EDSC is always $1
 * - As ETR is added, pool grows naturally
 *
 * Flow:
 * 1. Protocol mints and seeds 100M EDSC into pool
 * 2. User wants EDSC -> deposits ETR at oracle rate
 * 3. ETR accumulates in pool (real liquidity builds)
 * 4. User wants ETR -> deposits EDSC, receives ETR
 * 5. Price adjusts via AMM after initial oracle bootstrapping
 *
 * Price Mechanism:
 * - EDSC = $1 (stablecoin)
 * - ETR price from oracle (e.g., $0.004)
 * - 1 EDSC = 250 ETR (at $0.004)
 * - As pool builds liquidity, AMM takes over from oracle
 */
contract EDSCOraclePool is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    // Pool tokens
    IERC20 public immutable etr;
    IERC20 public immutable edsc;

    // Pool state
    uint256 public reserveETR;      // Real ETR in pool
    uint256 public reserveEDSC;     // Real EDSC in pool
    uint256 public initialEDSC;     // Initial EDSC seeded (for tracking)

    // Oracle
    address public oracle;
    uint256 public oracleETRPrice;  // ETR price in USD (8 decimals, e.g., 400000 = $0.004)
    uint256 public lastOracleUpdate;

    // Configuration
    uint256 public constant EDSC_PRICE = 1e8;           // EDSC = $1.00 (8 decimals)
    uint256 public constant FEE_NUMERATOR = 3;          // 0.3% fee
    uint256 public constant FEE_DENOMINATOR = 1000;
    uint256 public constant PRICE_DECIMALS = 8;

    // AMM transition
    uint256 public ammTransitionThreshold;              // ETR amount to transition to AMM
    bool public useAMMPricing;                          // True when AMM takes over

    // LP tracking
    uint256 public totalLiquidity;
    mapping(address => uint256) public liquidityOf;

    // Events
    event PoolInitialized(uint256 edscAmount);
    event SwapETRForEDSC(address indexed user, uint256 etrIn, uint256 edscOut);
    event SwapEDSCForETR(address indexed user, uint256 edscIn, uint256 etrOut);
    event OraclePriceUpdated(uint256 newETRPrice);
    event AMMTransitionActivated(uint256 reserveETR, uint256 reserveEDSC);
    event LiquidityAdded(address indexed provider, uint256 etrAmount, uint256 edscAmount);

    constructor(
        address _etr,
        address _edsc,
        address _oracle,
        uint256 _ammTransitionThreshold
    ) Ownable(msg.sender) {
        etr = IERC20(_etr);
        edsc = IERC20(_edsc);
        oracle = _oracle;
        ammTransitionThreshold = _ammTransitionThreshold;
    }

    /**
     * @dev Initialize pool with EDSC (protocol mints and deposits)
     * @param edscAmount Amount of EDSC to seed
     * @param initialETRPrice Initial ETR price in USD (8 decimals)
     */
    function initializePool(
        uint256 edscAmount,
        uint256 initialETRPrice
    ) external onlyOwner {
        require(reserveEDSC == 0, "Pool already initialized");
        require(edscAmount > 0, "EDSC amount must be > 0");
        require(initialETRPrice > 0, "ETR price must be > 0");

        // Transfer EDSC from protocol
        edsc.safeTransferFrom(msg.sender, address(this), edscAmount);
        reserveEDSC = edscAmount;
        initialEDSC = edscAmount;

        // Set oracle price
        oracleETRPrice = initialETRPrice;
        lastOracleUpdate = block.timestamp;

        // Foundation is initial liquidity provider
        totalLiquidity = edscAmount;
        liquidityOf[msg.sender] = edscAmount;

        emit PoolInitialized(edscAmount);
        emit OraclePriceUpdated(initialETRPrice);
    }

    /**
     * @dev Calculate ETR amount for given EDSC using oracle price
     * @param edscAmount Amount of EDSC
     * @return etrAmount Equivalent ETR amount
     */
    function getETRForEDSCOracle(uint256 edscAmount) public view returns (uint256 etrAmount) {
        // edscAmount (18 decimals) * EDSC_PRICE / oracleETRPrice * 10^(etrDecimals - priceDecimals)
        // Simplified: etrAmount = edscAmount * EDSC_PRICE / oracleETRPrice
        // At $0.004 ETR: 1 EDSC = 250 ETR
        require(oracleETRPrice > 0, "Oracle price not set");
        etrAmount = (edscAmount * EDSC_PRICE) / oracleETRPrice;
    }

    /**
     * @dev Calculate EDSC amount for given ETR using oracle price
     * @param etrAmount Amount of ETR
     * @return edscAmount Equivalent EDSC amount
     */
    function getEDSCForETROracle(uint256 etrAmount) public view returns (uint256 edscAmount) {
        // At $0.004 ETR: 250 ETR = 1 EDSC
        require(oracleETRPrice > 0, "Oracle price not set");
        edscAmount = (etrAmount * oracleETRPrice) / EDSC_PRICE;
    }

    /**
     * @dev Get AMM-based output amount
     */
    function getAmountOutAMM(
        uint256 amountIn,
        uint256 reserveIn,
        uint256 reserveOut
    ) public pure returns (uint256 amountOut) {
        require(amountIn > 0, "Insufficient input amount");
        require(reserveIn > 0 && reserveOut > 0, "Insufficient liquidity");

        uint256 amountInWithFee = amountIn * (FEE_DENOMINATOR - FEE_NUMERATOR);
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = (reserveIn * FEE_DENOMINATOR) + amountInWithFee;
        amountOut = numerator / denominator;
    }

    /**
     * @dev Swap ETR for EDSC (user deposits ETR, receives EDSC)
     * @param etrAmountIn Amount of ETR to deposit
     * @param minEDSCOut Minimum EDSC to receive
     */
    function swapETRForEDSC(
        uint256 etrAmountIn,
        uint256 minEDSCOut
    ) external nonReentrant returns (uint256 edscOut) {
        require(etrAmountIn > 0, "Amount must be > 0");

        if (useAMMPricing) {
            // Use AMM pricing
            edscOut = getAmountOutAMM(etrAmountIn, reserveETR, reserveEDSC);
        } else {
            // Use oracle pricing (minus fee)
            uint256 grossEDSC = getEDSCForETROracle(etrAmountIn);
            edscOut = grossEDSC - (grossEDSC * FEE_NUMERATOR / FEE_DENOMINATOR);
        }

        require(edscOut >= minEDSCOut, "Slippage exceeded");
        require(edscOut <= reserveEDSC, "Insufficient EDSC in pool");

        // Transfer ETR in
        etr.safeTransferFrom(msg.sender, address(this), etrAmountIn);

        // Transfer EDSC out
        edsc.safeTransfer(msg.sender, edscOut);

        // Update reserves
        reserveETR += etrAmountIn;
        reserveEDSC -= edscOut;

        // Check AMM transition
        _checkAMMTransition();

        emit SwapETRForEDSC(msg.sender, etrAmountIn, edscOut);
    }

    /**
     * @dev Swap EDSC for ETR (user deposits EDSC, receives ETR)
     * @param edscAmountIn Amount of EDSC to deposit
     * @param minETROut Minimum ETR to receive
     */
    function swapEDSCForETR(
        uint256 edscAmountIn,
        uint256 minETROut
    ) external nonReentrant returns (uint256 etrOut) {
        require(edscAmountIn > 0, "Amount must be > 0");

        if (useAMMPricing) {
            // Use AMM pricing
            etrOut = getAmountOutAMM(edscAmountIn, reserveEDSC, reserveETR);
        } else {
            // Use oracle pricing (minus fee)
            uint256 grossETR = getETRForEDSCOracle(edscAmountIn);
            etrOut = grossETR - (grossETR * FEE_NUMERATOR / FEE_DENOMINATOR);
        }

        require(etrOut >= minETROut, "Slippage exceeded");
        require(etrOut <= reserveETR, "Insufficient ETR in pool");

        // Transfer EDSC in
        edsc.safeTransferFrom(msg.sender, address(this), edscAmountIn);

        // Transfer ETR out
        etr.safeTransfer(msg.sender, etrOut);

        // Update reserves
        reserveEDSC += edscAmountIn;
        reserveETR -= etrOut;

        emit SwapEDSCForETR(msg.sender, edscAmountIn, etrOut);
    }

    /**
     * @dev Check if pool should transition to AMM pricing
     */
    function _checkAMMTransition() internal {
        if (!useAMMPricing && reserveETR >= ammTransitionThreshold) {
            useAMMPricing = true;
            emit AMMTransitionActivated(reserveETR, reserveEDSC);
        }
    }

    /**
     * @dev Update oracle price
     * @param newETRPrice New ETR price in USD (8 decimals)
     */
    function updateOraclePrice(uint256 newETRPrice) external {
        require(msg.sender == oracle || msg.sender == owner(), "Not authorized");
        require(newETRPrice > 0, "Invalid price");

        oracleETRPrice = newETRPrice;
        lastOracleUpdate = block.timestamp;

        emit OraclePriceUpdated(newETRPrice);
    }

    /**
     * @dev Add liquidity to pool (requires both tokens after ETR exists)
     */
    function addLiquidity(
        uint256 etrAmount,
        uint256 edscAmount,
        uint256 minLiquidity
    ) external nonReentrant returns (uint256 liquidity) {
        require(reserveEDSC > 0, "Pool not initialized");

        // Calculate liquidity based on current ratio
        if (reserveETR == 0) {
            // First ETR deposit - use ratio based on oracle
            uint256 expectedEDSC = getEDSCForETROracle(etrAmount);
            require(edscAmount >= expectedEDSC, "Insufficient EDSC for ratio");
            liquidity = etrAmount;
        } else {
            // Proportional add
            uint256 liquidityFromETR = (etrAmount * totalLiquidity) / reserveETR;
            uint256 liquidityFromEDSC = (edscAmount * totalLiquidity) / reserveEDSC;
            liquidity = liquidityFromETR < liquidityFromEDSC ? liquidityFromETR : liquidityFromEDSC;
        }

        require(liquidity >= minLiquidity, "Insufficient liquidity minted");

        // Transfer tokens
        if (etrAmount > 0) {
            etr.safeTransferFrom(msg.sender, address(this), etrAmount);
            reserveETR += etrAmount;
        }
        if (edscAmount > 0) {
            edsc.safeTransferFrom(msg.sender, address(this), edscAmount);
            reserveEDSC += edscAmount;
        }

        totalLiquidity += liquidity;
        liquidityOf[msg.sender] += liquidity;

        emit LiquidityAdded(msg.sender, etrAmount, edscAmount);
    }

    /**
     * @dev Get current pool state
     */
    function getPoolInfo() external view returns (
        uint256 _reserveETR,
        uint256 _reserveEDSC,
        uint256 _oracleETRPrice,
        bool _useAMMPricing,
        uint256 _totalLiquidity
    ) {
        return (
            reserveETR,
            reserveEDSC,
            oracleETRPrice,
            useAMMPricing,
            totalLiquidity
        );
    }

    /**
     * @dev Get current effective price (ETR per EDSC)
     */
    function getEffectivePrice() external view returns (uint256) {
        if (useAMMPricing && reserveEDSC > 0) {
            return (reserveETR * 1e18) / reserveEDSC;
        } else {
            return getETRForEDSCOracle(1e18);
        }
    }

    /**
     * @dev Update configuration
     */
    function setOracle(address _oracle) external onlyOwner {
        oracle = _oracle;
    }

    function setAMMTransitionThreshold(uint256 _threshold) external onlyOwner {
        ammTransitionThreshold = _threshold;
    }

    /**
     * @dev Force AMM mode (emergency/governance)
     */
    function forceAMMMode(bool enabled) external onlyOwner {
        useAMMPricing = enabled;
    }
}
