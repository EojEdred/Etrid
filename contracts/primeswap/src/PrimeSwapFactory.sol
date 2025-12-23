// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./VirtualReserveAMM.sol";
import "./EDSCOraclePool.sol";
import "./EDSCPegPool.sol";

/**
 * @title PrimeSwapFactory
 * @dev Factory contract for deploying and managing PrimeSwap pools
 *
 * Pool Types:
 * 1. VirtualReserveAMM - For ETR/wToken pairs (ETR seeded, virtual reserves)
 * 2. EDSCOraclePool - For ETR/EDSC (EDSC seeded, oracle priced)
 * 3. EDSCPegPool - For EDSC/USDT (stablecoin stability)
 *
 * Architecture:
 * - 11 VirtualReserveAMM pools (one per external PBC token)
 * - 1 EDSCOraclePool (ETR/EDSC)
 * - 1 EDSCPegPool (EDSC/USDT)
 */
contract PrimeSwapFactory is Ownable {

    // Token addresses
    address public etr;
    address public edsc;
    address public oracle;

    // Pool registries
    mapping(address => address) public virtualReservePools;  // pairedToken => pool
    address public edscOraclePool;
    address public edscPegPool;

    // All pools
    address[] public allPools;

    // PBC token metadata
    struct PBCToken {
        string symbol;
        address tokenAddress;
        uint256 marketCapWeight;      // Basis points (10000 = 100%)
        uint256 etrAllocation;        // ETR allocated to this pool
        bool isActive;
    }
    mapping(address => PBCToken) public pbcTokens;
    address[] public pbcTokenList;

    // Events
    event VirtualReservePoolCreated(
        address indexed pool,
        address indexed pairedToken,
        string symbol,
        uint256 etrAllocation
    );
    event EDSCOraclePoolCreated(address indexed pool);
    event EDSCPegPoolCreated(address indexed pool);
    event PoolInitialized(address indexed pool, uint256 amount0, uint256 amount1);

    constructor(
        address _etr,
        address _edsc,
        address _oracle
    ) Ownable(msg.sender) {
        etr = _etr;
        edsc = _edsc;
        oracle = _oracle;
    }

    /**
     * @dev Register a PBC token for pool creation
     * @param symbol Token symbol (e.g., "wBTC")
     * @param tokenAddress Token contract address
     * @param marketCapWeight Weight in basis points (10000 = 100%)
     */
    function registerPBCToken(
        string calldata symbol,
        address tokenAddress,
        uint256 marketCapWeight
    ) external onlyOwner {
        require(tokenAddress != address(0), "Invalid address");
        require(!pbcTokens[tokenAddress].isActive, "Already registered");

        pbcTokens[tokenAddress] = PBCToken({
            symbol: symbol,
            tokenAddress: tokenAddress,
            marketCapWeight: marketCapWeight,
            etrAllocation: 0,
            isActive: true
        });
        pbcTokenList.push(tokenAddress);
    }

    /**
     * @dev Create VirtualReserveAMM pool for a PBC token
     * @param pairedToken Address of the wrapped token (e.g., wBTC)
     * @param virtualPhaseOutThreshold When real liquidity exceeds this, virtual reserves = 0
     */
    function createVirtualReservePool(
        address pairedToken,
        uint256 virtualPhaseOutThreshold
    ) external onlyOwner returns (address pool) {
        require(virtualReservePools[pairedToken] == address(0), "Pool exists");
        require(pbcTokens[pairedToken].isActive, "Token not registered");

        pool = address(new VirtualReserveAMM(
            etr,
            pairedToken,
            oracle,
            virtualPhaseOutThreshold
        ));

        virtualReservePools[pairedToken] = pool;
        allPools.push(pool);

        emit VirtualReservePoolCreated(
            pool,
            pairedToken,
            pbcTokens[pairedToken].symbol,
            0
        );
    }

    /**
     * @dev Create EDSCOraclePool (ETR/EDSC)
     * @param ammTransitionThreshold ETR amount to trigger AMM transition
     */
    function createEDSCOraclePool(
        uint256 ammTransitionThreshold
    ) external onlyOwner returns (address pool) {
        require(edscOraclePool == address(0), "Pool exists");

        pool = address(new EDSCOraclePool(
            etr,
            edsc,
            oracle,
            ammTransitionThreshold
        ));

        edscOraclePool = pool;
        allPools.push(pool);

        emit EDSCOraclePoolCreated(pool);
    }

    /**
     * @dev Create EDSCPegPool (EDSC/USDT) - single-sided EDSC
     * @param usdt USDT token address
     * @param usdtDecimals USDT decimals (usually 6)
     * @param stableSwapThreshold USDT amount to activate StableSwap
     */
    function createEDSCPegPool(
        address usdt,
        uint256 usdtDecimals,
        uint256 stableSwapThreshold
    ) external onlyOwner returns (address pool) {
        require(edscPegPool == address(0), "Pool exists");

        pool = address(new EDSCPegPool(
            edsc,
            usdt,
            18,  // EDSC decimals
            usdtDecimals,
            stableSwapThreshold
        ));

        edscPegPool = pool;
        allPools.push(pool);

        emit EDSCPegPoolCreated(pool);
    }

    /**
     * @dev Calculate ETR allocation for each pool based on market cap weights
     * @param totalETR Total ETR to distribute
     */
    function calculateAllocations(uint256 totalETR) external view returns (
        address[] memory tokens,
        uint256[] memory allocations
    ) {
        uint256 count = pbcTokenList.length;
        tokens = new address[](count);
        allocations = new uint256[](count);

        uint256 totalWeight = 0;
        for (uint256 i = 0; i < count; i++) {
            totalWeight += pbcTokens[pbcTokenList[i]].marketCapWeight;
        }

        for (uint256 i = 0; i < count; i++) {
            tokens[i] = pbcTokenList[i];
            allocations[i] = (totalETR * pbcTokens[pbcTokenList[i]].marketCapWeight) / totalWeight;
        }
    }

    /**
     * @dev Update ETR allocation for a token
     */
    function updateAllocation(address token, uint256 allocation) external onlyOwner {
        require(pbcTokens[token].isActive, "Token not registered");
        pbcTokens[token].etrAllocation = allocation;
    }

    /**
     * @dev Get all registered PBC tokens
     */
    function getAllPBCTokens() external view returns (address[] memory) {
        return pbcTokenList;
    }

    /**
     * @dev Get all pools
     */
    function getAllPools() external view returns (address[] memory) {
        return allPools;
    }

    /**
     * @dev Get pool for a specific token pair
     */
    function getPool(address token) external view returns (address) {
        if (token == edsc) {
            return edscOraclePool;
        }
        return virtualReservePools[token];
    }

    /**
     * @dev Update oracle address
     */
    function setOracle(address _oracle) external onlyOwner {
        oracle = _oracle;
    }

    /**
     * @dev Transfer pool ownership (for governance)
     */
    function transferPoolOwnership(address pool, address newOwner) external onlyOwner {
        Ownable(pool).transferOwnership(newOwner);
    }
}
