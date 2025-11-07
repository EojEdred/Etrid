// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/**
 * @title WrappedETR
 * @notice Wrapped version of the native ETR token from FlareChain
 * @dev ERC20 token with minting and burning capabilities for bridge operations
 *
 * Roles:
 * - MINTER_ROLE: Can mint new tokens (bridge contracts)
 * - BURNER_ROLE: Can burn tokens (bridge contracts)
 * - PAUSER_ROLE: Can pause/unpause transfers (emergency)
 * - DEFAULT_ADMIN_ROLE: Can manage roles (multi-sig)
 */
contract WrappedETR is ERC20, ERC20Burnable, AccessControl, Pausable {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @notice Total supply cap (10 billion ETR)
    uint256 public constant MAX_SUPPLY = 10_000_000_000 * 10**18;

    /// @notice Emitted when tokens are bridged in from FlareChain
    event TokensBridgedIn(address indexed to, uint256 amount, bytes32 indexed txHash);

    /// @notice Emitted when tokens are bridged out to FlareChain
    event TokensBridgedOut(address indexed from, uint256 amount, bytes32 indexed destinationAccount);

    error MaxSupplyExceeded();
    error ZeroAddress();
    error ZeroAmount();

    constructor(address admin) ERC20("Wrapped ETR", "wETR") {
        if (admin == address(0)) revert ZeroAddress();

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);

        // Admin can grant MINTER_ROLE and BURNER_ROLE to bridge contracts
    }

    /**
     * @notice Mint tokens when bridging from FlareChain
     * @param to Recipient address
     * @param amount Amount to mint
     * @param txHash Transaction hash from FlareChain
     */
    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyRole(MINTER_ROLE) {
        if (to == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (totalSupply() + amount > MAX_SUPPLY) revert MaxSupplyExceeded();

        _mint(to, amount);
        emit TokensBridgedIn(to, amount, txHash);
    }

    /**
     * @notice Burn tokens when bridging to FlareChain
     * @param from Address to burn from
     * @param amount Amount to burn
     * @param destinationAccount FlareChain destination account
     */
    function bridgeBurn(
        address from,
        uint256 amount,
        bytes32 destinationAccount
    ) external onlyRole(BURNER_ROLE) {
        if (from == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();

        _burn(from, amount);
        emit TokensBridgedOut(from, amount, destinationAccount);
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
     * @notice Get token decimals (18 to match native ETR)
     */
    function decimals() public pure override returns (uint8) {
        return 18;
    }
}
