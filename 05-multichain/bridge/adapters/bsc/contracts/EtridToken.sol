// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title EtridToken
 * @dev ÉTR token on Binance Smart Chain
 *
 * Features:
 * - ERC20 standard compliance
 * - Bridge-controlled minting (only bridge can mint)
 * - Burnable (for bridging back to Ëtrid chain)
 * - Pausable (emergency circuit breaker)
 * - Access control for governance
 */
contract EtridToken is ERC20, ERC20Burnable, AccessControl, Pausable {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");

    // Events
    event BridgeMint(address indexed to, uint256 amount, bytes32 indexed txHash);
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);

    /**
     * @dev Constructor
     * @param name Token name (e.g., "Etrid Coin")
     * @param symbol Token symbol (e.g., "ÉTR")
     */
    constructor(
        string memory name,
        string memory symbol
    ) ERC20(name, symbol) {
        // Grant roles to deployer (will be transferred to multi-sig later)
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        _grantRole(BRIDGE_ROLE, msg.sender);
    }

    /**
     * @dev Mint tokens (bridge only)
     * @param to Recipient address
     * @param amount Amount to mint
     * @param txHash Transaction hash from Ëtrid chain (for tracking)
     */
    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyRole(BRIDGE_ROLE) whenNotPaused {
        require(to != address(0), "EtridToken: mint to zero address");
        require(amount > 0, "EtridToken: mint amount must be positive");

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @dev Burn tokens for bridging back to Ëtrid chain
     * @param amount Amount to burn
     * @param etridAddress Destination address on Ëtrid chain
     */
    function bridgeBurn(
        uint256 amount,
        string memory etridAddress
    ) external whenNotPaused {
        require(amount > 0, "EtridToken: burn amount must be positive");
        require(bytes(etridAddress).length > 0, "EtridToken: invalid Etrid address");

        _burn(msg.sender, amount);
        emit BridgeBurn(msg.sender, amount, etridAddress);
    }

    /**
     * @dev Pause all token transfers (emergency only)
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Unpause token transfers
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Override to add pausable functionality
     */
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }

    /**
     * @dev Returns the number of decimals (18, standard for ERC20)
     */
    function decimals() public pure override returns (uint8) {
        return 18;
    }
}
