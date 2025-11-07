// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/**
 * @title EDSC
 * @notice Ëtrid Decentralized Stablecoin - Multi-chain stablecoin for the Ëtrid ecosystem
 * @dev ERC20 stablecoin with burn-and-mint bridge mechanism
 *
 * Features:
 * - Burn-and-mint cross-chain transfers
 * - M-of-N oracle attestation (3-of-5)
 * - Rate limiting for security
 * - Emergency pause functionality
 *
 * Roles:
 * - MINTER_ROLE: TokenMessenger contracts on each chain
 * - BURNER_ROLE: TokenMessenger contracts on each chain
 * - PAUSER_ROLE: Emergency pause (multi-sig)
 * - DEFAULT_ADMIN_ROLE: Role management (multi-sig)
 */
contract EDSC is ERC20, ERC20Burnable, AccessControl, Pausable {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @notice Maximum tokens that can be minted per day (rate limiting)
    uint256 public dailyMintLimit = 1_000_000 * 10**18; // 1M EDSC

    /// @notice Timestamp of current rate limit window
    uint256 public currentWindow;

    /// @notice Amount minted in current window
    uint256 public windowMinted;

    /// @notice Window duration (24 hours)
    uint256 public constant WINDOW_DURATION = 24 hours;

    event DailyMintLimitUpdated(uint256 newLimit);
    event CrossChainTransfer(
        address indexed from,
        uint256 amount,
        uint32 indexed destinationDomain,
        bytes32 destinationAddress
    );

    error MaxDailyMintExceeded();
    error ZeroAddress();
    error ZeroAmount();

    constructor(address admin) ERC20("Etrid Decentralized Stablecoin", "EDSC") {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);

        currentWindow = block.timestamp;
    }

    /**
     * @notice Mint tokens after receiving cross-chain message
     * @param to Recipient address
     * @param amount Amount to mint
     */
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        // Reset window if needed
        if (block.timestamp >= currentWindow + WINDOW_DURATION) {
            currentWindow = block.timestamp;
            windowMinted = 0;
        }

        // Check rate limit
        if (windowMinted + amount > dailyMintLimit) {
            revert MaxDailyMintExceeded();
        }

        windowMinted += amount;
        _mint(to, amount);
    }

    /**
     * @notice Burn tokens for cross-chain transfer
     * @param from Address to burn from
     * @param amount Amount to burn
     */
    function burnFrom(address from, uint256 amount) public override {
        if (from == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        super.burnFrom(from, amount);
    }

    /**
     * @notice Update daily mint limit
     * @param newLimit New limit amount
     */
    function setDailyMintLimit(uint256 newLimit) external onlyRole(DEFAULT_ADMIN_ROLE) {
        dailyMintLimit = newLimit;
        emit DailyMintLimitUpdated(newLimit);
    }

    /**
     * @notice Pause all token transfers (emergency only)
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause token transfers
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Override _update to add pause functionality
     */
    function _update(
        address from,
        address to,
        uint256 value
    ) internal override whenNotPaused {
        super._update(from, to, value);
    }

    /**
     * @notice Get token decimals (6 to match USDC/USDT)
     */
    function decimals() public pure override returns (uint8) {
        return 6;
    }
}
