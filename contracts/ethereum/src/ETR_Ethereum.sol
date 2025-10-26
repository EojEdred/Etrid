// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title ETR_Ethereum
 * @dev ÉTR.e - Wrapped ÉTR token on Ethereum
 *
 * This is the canonical Ethereum representation of native ÉTR from Ëtrid FlareChain.
 * Tokens are minted 1:1 when users lock native ÉTR via the Ëtrid bridge.
 * Tokens are burned when users withdraw back to native ÉTR.
 *
 * Features:
 * - ERC-20 standard compliant
 * - Burnable (for bridge withdrawals)
 * - Permit (EIP-2612 for gasless approvals)
 * - Pausable (emergency circuit breaker)
 * - Access controlled minting (only bridge contract)
 *
 * Symbol: ÉTR
 * Name: Etrid Coin (Ethereum)
 * Decimals: 18
 */
contract ETRToken is ERC20, ERC20Burnable, ERC20Permit, AccessControl, Pausable {
    /// @dev Role for the bridge contract (can mint/burn)
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");

    /// @dev Role for pausing (emergency multisig)
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @dev Maximum supply cap (100 billion ÉTR)
    uint256 public constant MAX_SUPPLY = 100_000_000_000 * 10**18;

    /// @dev Per-transaction mint limit (anti-exploit)
    uint256 public constant MAX_MINT_PER_TX = 100_000 * 10**18; // 100k ÉTR

    /// @dev Daily mint limit (rolling 24h window)
    uint256 public constant MAX_MINT_PER_DAY = 1_000_000 * 10**18; // 1M ÉTR

    /// @dev Tracking for daily mint limit
    uint256 public mintedToday;
    uint256 public lastMintDay;

    /// @notice Emitted when tokens are minted via bridge
    event BridgeMint(address indexed to, uint256 amount, bytes32 txHash);

    /// @notice Emitted when tokens are burned for bridge withdrawal
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);

    /**
     * @dev Constructor
     * @param admin Address that will have DEFAULT_ADMIN_ROLE
     * @param bridge Address of the bridge contract (gets BRIDGE_ROLE)
     */
    constructor(
        address admin,
        address bridge
    ) ERC20("Etrid Coin (Ethereum)", unicode"ÉTR") ERC20Permit("Etrid Coin") {
        require(admin != address(0), "ETR: zero admin address");
        require(bridge != address(0), "ETR: zero bridge address");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_ROLE, bridge);
        _grantRole(PAUSER_ROLE, admin);

        // Admin can grant/revoke roles but cannot mint
        _setRoleAdmin(BRIDGE_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(PAUSER_ROLE, DEFAULT_ADMIN_ROLE);
    }

    /**
     * @notice Mint tokens (bridge only)
     * @dev Called by bridge when user locks native ÉTR on Ëtrid
     * @param to Recipient address
     * @param amount Amount to mint
     * @param txHash Transaction hash from Ëtrid (for tracking)
     */
    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyRole(BRIDGE_ROLE) whenNotPaused {
        require(to != address(0), "ETR: mint to zero address");
        require(amount > 0, "ETR: mint zero amount");
        require(amount <= MAX_MINT_PER_TX, "ETR: exceeds per-tx limit");
        require(totalSupply() + amount <= MAX_SUPPLY, "ETR: exceeds max supply");

        // Check daily limit
        uint256 currentDay = block.timestamp / 1 days;
        if (currentDay > lastMintDay) {
            // New day, reset counter
            mintedToday = 0;
            lastMintDay = currentDay;
        }

        require(mintedToday + amount <= MAX_MINT_PER_DAY, "ETR: exceeds daily limit");
        mintedToday += amount;

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @notice Burn tokens and emit withdrawal event
     * @dev User calls this to withdraw ÉTR back to Ëtrid
     * @param amount Amount to burn
     * @param etridAddress Destination address on Ëtrid (SS58 or hex)
     */
    function bridgeBurn(
        uint256 amount,
        string calldata etridAddress
    ) external whenNotPaused {
        require(amount > 0, "ETR: burn zero amount");
        require(bytes(etridAddress).length > 0, "ETR: empty etrid address");

        _burn(msg.sender, amount);
        emit BridgeBurn(msg.sender, amount, etridAddress);
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
     * @param from Address tokens are transferred from
     * @param to Address tokens are transferred to
     * @param amount Amount of tokens
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
