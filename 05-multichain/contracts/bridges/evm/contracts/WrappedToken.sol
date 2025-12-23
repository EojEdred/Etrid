// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";

/**
 * @title WrappedToken
 * @notice Wrapped token for cross-chain bridges (e.g., wBTC, wETH, wSOL on ETRID)
 * @dev Only bridge contracts can mint/burn
 */
contract WrappedToken is ERC20, ERC20Burnable, AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    uint8 private immutable _decimals;

    /// @notice Original token details
    string public originalChain;
    string public originalSymbol;
    address public originalAddress;

    event Minted(address indexed to, uint256 amount, address indexed minter);
    event Burned(address indexed from, uint256 amount, address indexed burner);

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        string memory originalChain_,
        string memory originalSymbol_,
        address originalAddress_,
        address admin
    ) ERC20(name_, symbol_) {
        _decimals = decimals_;
        originalChain = originalChain_;
        originalSymbol = originalSymbol_;
        originalAddress = originalAddress_;

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MINTER_ROLE, admin);
        _grantRole(BURNER_ROLE, admin);
    }

    function decimals() public view virtual override returns (uint8) {
        return _decimals;
    }

    /**
     * @notice Mint wrapped tokens (bridge only)
     */
    function mint(address to, uint256 amount) external onlyRole(MINTER_ROLE) {
        _mint(to, amount);
        emit Minted(to, amount, msg.sender);
    }

    /**
     * @notice Burn tokens from address (bridge only)
     */
    function burnFrom(
        address from,
        uint256 amount
    ) public virtual override onlyRole(BURNER_ROLE) {
        _burn(from, amount);
        emit Burned(from, amount, msg.sender);
    }

    /**
     * @notice Grant minter role to bridge contract
     */
    function grantMinterRole(address bridge) external onlyRole(DEFAULT_ADMIN_ROLE) {
        grantRole(MINTER_ROLE, bridge);
        grantRole(BURNER_ROLE, bridge);
    }
}
