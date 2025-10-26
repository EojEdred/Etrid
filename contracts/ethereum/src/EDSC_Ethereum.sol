// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title EDSC_Ethereum
 * @dev EDSC.e - Ëtrid Dollar Stablecoin on Ethereum
 *
 * This is the canonical Ethereum representation of EDSC from Ëtrid FlareChain.
 * EDSC is a fiat-pegged stablecoin (1 EDSC = $1.00 USD) backed by reserves.
 *
 * Key Features:
 * - Authorized Participants (APs) can mint/redeem against reserves
 * - Bridge integration for cross-chain transfers
 * - Reserve attestation hooks
 * - Emergency pause mechanism
 * - Strict mint/burn controls
 *
 * Symbol: EDSC
 * Name: Etrid Dollar Stablecoin (Ethereum)
 * Decimals: 18
 * Target Peg: $1.00 USD
 */
contract EDSCToken is
    ERC20,
    ERC20Burnable,
    ERC20Permit,
    AccessControl,
    Pausable,
    ReentrancyGuard
{
    /// @dev Role for Authorized Participants (can mint/redeem with reserves)
    bytes32 public constant AP_ROLE = keccak256("AP_ROLE");

    /// @dev Role for the bridge contract (can mint/burn for cross-chain)
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");

    /// @dev Role for pausing (emergency multisig)
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @dev Role for reserve oracle (attests to backing)
    bytes32 public constant ORACLE_ROLE = keccak256("ORACLE_ROLE");

    /// @dev Maximum supply cap (100 billion EDSC)
    uint256 public constant MAX_SUPPLY = 100_000_000_000 * 10**18;

    /// @dev Per-transaction mint limit (anti-exploit)
    uint256 public constant MAX_MINT_PER_TX = 500_000 * 10**18; // 500k EDSC

    /// @dev Daily mint limit (rolling 24h window)
    uint256 public constant MAX_MINT_PER_DAY = 5_000_000 * 10**18; // 5M EDSC

    /// @dev Minimum cooldown between large mints (6 hours)
    uint256 public constant MINT_COOLDOWN = 6 hours;

    /// @dev Reserve ratio (basis points, 10000 = 100%)
    /// Must maintain at least 100% reserves
    uint256 public reserveRatio = 10000; // 100%

    /// @dev Minimum reserve ratio before circuit breaker
    uint256 public constant MIN_RESERVE_RATIO = 10000; // 100%

    /// @dev Tracking for daily mint limit
    uint256 public mintedToday;
    uint256 public lastMintDay;

    /// @dev Last large mint timestamp (per AP)
    mapping(address => uint256) public lastLargeMint;

    /// @notice Emitted when AP mints against reserves
    event APMint(
        address indexed ap,
        address indexed to,
        uint256 amount,
        bytes32 reserveProof
    );

    /// @notice Emitted when AP burns and redeems reserves
    event APBurn(
        address indexed ap,
        address indexed from,
        uint256 amount,
        bytes32 redemptionId
    );

    /// @notice Emitted when tokens are minted via bridge
    event BridgeMint(address indexed to, uint256 amount, bytes32 txHash);

    /// @notice Emitted when tokens are burned for bridge withdrawal
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);

    /// @notice Emitted when reserve ratio is updated
    event ReserveRatioUpdated(uint256 oldRatio, uint256 newRatio);

    /**
     * @dev Constructor
     * @param admin Address that will have DEFAULT_ADMIN_ROLE
     * @param bridge Address of the bridge contract (gets BRIDGE_ROLE)
     */
    constructor(
        address admin,
        address bridge
    ) ERC20("Etrid Dollar Stablecoin (Ethereum)", "EDSC") ERC20Permit("Etrid Dollar Stablecoin") {
        require(admin != address(0), "EDSC: zero admin address");
        require(bridge != address(0), "EDSC: zero bridge address");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_ROLE, bridge);
        _grantRole(PAUSER_ROLE, admin);

        // Admin can manage roles
        _setRoleAdmin(AP_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(BRIDGE_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(PAUSER_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(ORACLE_ROLE, DEFAULT_ADMIN_ROLE);
    }

    /**
     * @notice Add Authorized Participant
     * @dev Only admin can call
     * @param ap Address to grant AP_ROLE
     */
    function addAP(address ap) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(ap != address(0), "EDSC: zero AP address");
        grantRole(AP_ROLE, ap);
    }

    /**
     * @notice Remove Authorized Participant
     * @dev Only admin can call
     * @param ap Address to revoke AP_ROLE
     */
    function removeAP(address ap) external onlyRole(DEFAULT_ADMIN_ROLE) {
        revokeRole(AP_ROLE, ap);
    }

    /**
     * @notice Mint EDSC against reserves (AP only)
     * @dev Called by AP when depositing reserves (USD/T-bills)
     * @param to Recipient address
     * @param amount Amount to mint
     * @param reserveProof Hash of reserve attestation document
     */
    function apMint(
        address to,
        uint256 amount,
        bytes32 reserveProof
    ) external onlyRole(AP_ROLE) whenNotPaused nonReentrant {
        require(to != address(0), "EDSC: mint to zero address");
        require(amount > 0, "EDSC: mint zero amount");
        require(amount <= MAX_MINT_PER_TX, "EDSC: exceeds per-tx limit");
        require(totalSupply() + amount <= MAX_SUPPLY, "EDSC: exceeds max supply");
        require(reserveProof != bytes32(0), "EDSC: invalid reserve proof");

        // Check reserve ratio
        require(reserveRatio >= MIN_RESERVE_RATIO, "EDSC: insufficient reserves");

        // Check daily limit
        uint256 currentDay = block.timestamp / 1 days;
        if (currentDay > lastMintDay) {
            mintedToday = 0;
            lastMintDay = currentDay;
        }

        require(mintedToday + amount <= MAX_MINT_PER_DAY, "EDSC: exceeds daily limit");
        mintedToday += amount;

        // Check cooldown for large mints (>100k EDSC)
        if (amount > 100_000 * 10**18) {
            require(
                block.timestamp >= lastLargeMint[msg.sender] + MINT_COOLDOWN,
                "EDSC: mint cooldown active"
            );
            lastLargeMint[msg.sender] = block.timestamp;
        }

        _mint(to, amount);
        emit APMint(msg.sender, to, amount, reserveProof);
    }

    /**
     * @notice Burn EDSC and redeem reserves (AP only)
     * @dev Called by AP when user redeems EDSC for USD
     * @param from Address to burn from (must have approval)
     * @param amount Amount to burn
     * @param redemptionId Unique ID for this redemption
     */
    function apBurn(
        address from,
        uint256 amount,
        bytes32 redemptionId
    ) external onlyRole(AP_ROLE) whenNotPaused nonReentrant {
        require(from != address(0), "EDSC: burn from zero address");
        require(amount > 0, "EDSC: burn zero amount");
        require(redemptionId != bytes32(0), "EDSC: invalid redemption ID");

        _burn(from, amount);
        emit APBurn(msg.sender, from, amount, redemptionId);
    }

    /**
     * @notice Mint tokens (bridge only)
     * @dev Called by bridge when user locks native EDSC on Ëtrid
     * @param to Recipient address
     * @param amount Amount to mint
     * @param txHash Transaction hash from Ëtrid
     */
    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyRole(BRIDGE_ROLE) whenNotPaused {
        require(to != address(0), "EDSC: mint to zero address");
        require(amount > 0, "EDSC: mint zero amount");
        require(totalSupply() + amount <= MAX_SUPPLY, "EDSC: exceeds max supply");

        // Bridge mints don't count toward daily AP limits
        // (they're 1:1 with native EDSC, not new issuance)

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @notice Burn tokens and emit withdrawal event
     * @dev User calls this to withdraw EDSC back to Ëtrid
     * @param amount Amount to burn
     * @param etridAddress Destination address on Ëtrid
     */
    function bridgeBurn(
        uint256 amount,
        string calldata etridAddress
    ) external whenNotPaused {
        require(amount > 0, "EDSC: burn zero amount");
        require(bytes(etridAddress).length > 0, "EDSC: empty etrid address");

        _burn(msg.sender, amount);
        emit BridgeBurn(msg.sender, amount, etridAddress);
    }

    /**
     * @notice Update reserve ratio (oracle only)
     * @dev Called by oracle after verifying reserves
     * @param newRatio New reserve ratio in basis points (10000 = 100%)
     */
    function updateReserveRatio(
        uint256 newRatio
    ) external onlyRole(ORACLE_ROLE) {
        require(newRatio >= MIN_RESERVE_RATIO, "EDSC: ratio below minimum");
        require(newRatio <= 20000, "EDSC: ratio too high"); // Max 200%

        uint256 oldRatio = reserveRatio;
        reserveRatio = newRatio;

        emit ReserveRatioUpdated(oldRatio, newRatio);

        // If ratio drops below minimum, trigger pause
        if (newRatio < MIN_RESERVE_RATIO) {
            _pause();
        }
    }

    /**
     * @notice Get current reserve ratio
     * @return Reserve ratio in basis points
     */
    function getReserveRatio() external view returns (uint256) {
        return reserveRatio;
    }

    /**
     * @notice Check if reserves are healthy
     * @return True if reserve ratio >= minimum
     */
    function isReserveHealthy() external view returns (bool) {
        return reserveRatio >= MIN_RESERVE_RATIO;
    }

    /**
     * @notice Pause token transfers (emergency only)
     * @dev Only PAUSER_ROLE can call
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause token transfers
     * @dev Only PAUSER_ROLE can call
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Hook that is called before any transfer of tokens
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
