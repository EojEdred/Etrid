// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title WrappedETR
 * @dev BEP-20 token representing wrapped Etrid (ETR) on Binance Smart Chain
 *
 * Features:
 * - Mintable only by authorized bridge
 * - Burnable by users and bridge
 * - Pausable for emergency situations
 * - Role-based access control
 * - Reentrancy protection
 */
contract WrappedETR is ERC20, ERC20Burnable, Pausable, AccessControl, ReentrancyGuard {
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    // Events
    event BridgeMint(address indexed to, uint256 amount, bytes32 indexed txHash);
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);
    event BridgeAddressUpdated(address indexed oldBridge, address indexed newBridge);

    /**
     * @dev Constructor sets up the token with initial roles
     * @param admin Address that will have DEFAULT_ADMIN_ROLE
     */
    constructor(address admin) ERC20("Wrapped Etrid", "wETR") {
        require(admin != address(0), "WrappedETR: admin is zero address");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);
    }

    /**
     * @dev Returns the number of decimals used for token amounts
     * @return uint8 Number of decimals (18)
     */
    function decimals() public pure override returns (uint8) {
        return 18;
    }

    /**
     * @dev Mints tokens to a specified address
     * Can only be called by addresses with BRIDGE_ROLE
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     * @param txHash Hash of the Etrid transaction that triggered this mint
     */
    function mint(address to, uint256 amount, bytes32 txHash)
        external
        nonReentrant
        onlyRole(BRIDGE_ROLE)
        whenNotPaused
    {
        require(to != address(0), "WrappedETR: mint to zero address");
        require(amount > 0, "WrappedETR: mint amount must be greater than 0");

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @dev Alias for mint() to maintain compatibility with EtridBridge contract
     * The EtridBridge contract calls bridgeMint() via IEtridToken interface
     * @param to Address to mint tokens to
     * @param amount Amount of tokens to mint
     * @param txHash Hash of the Etrid transaction that triggered this mint
     */
    function bridgeMint(address to, uint256 amount, bytes32 txHash)
        external
        nonReentrant
        onlyRole(BRIDGE_ROLE)
        whenNotPaused
    {
        require(to != address(0), "WrappedETR: mint to zero address");
        require(amount > 0, "WrappedETR: mint amount must be greater than 0");

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @dev Burns tokens from caller's account and emits event for bridge
     * @param amount Amount of tokens to burn
     * @param etridAddress Etrid address to receive the native ETR
     */
    function burnToBridge(uint256 amount, string calldata etridAddress)
        external
        nonReentrant
        whenNotPaused
    {
        require(amount > 0, "WrappedETR: burn amount must be greater than 0");
        require(bytes(etridAddress).length > 0, "WrappedETR: etrid address is empty");

        _burn(_msgSender(), amount);
        emit BridgeBurn(_msgSender(), amount, etridAddress);
    }

    /**
     * @dev Adds bridge role to an address
     * Can only be called by admin
     * @param bridge Address to grant bridge role
     */
    function addBridge(address bridge) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(bridge != address(0), "WrappedETR: bridge is zero address");
        grantRole(BRIDGE_ROLE, bridge);
        emit BridgeAddressUpdated(address(0), bridge);
    }

    /**
     * @dev Removes bridge role from an address
     * Can only be called by admin
     * @param bridge Address to revoke bridge role from
     */
    function removeBridge(address bridge) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(bridge != address(0), "WrappedETR: bridge is zero address");
        revokeRole(BRIDGE_ROLE, bridge);
        emit BridgeAddressUpdated(bridge, address(0));
    }

    /**
     * @dev Pauses all token transfers and minting
     * Can only be called by addresses with PAUSER_ROLE
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @dev Unpauses all token transfers and minting
     * Can only be called by addresses with PAUSER_ROLE
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @dev Hook that is called before any transfer of tokens
     * Ensures contract is not paused
     */
    function _beforeTokenTransfer(address from, address to, uint256 amount)
        internal
        whenNotPaused
        override
    {
        super._beforeTokenTransfer(from, to, amount);
    }
}
