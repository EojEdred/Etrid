// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title ETR_Base
 * @dev ÉTR.b - Wrapped ÉTR token on Base L2
 *
 * This is the canonical Base representation of native ÉTR from Ëtrid FlareChain.
 * Identical to ETR_Ethereum.sol but deployed on Base L2 for lower gas costs.
 *
 * Symbol: ÉTR
 * Name: Etrid Coin (Base)
 * Decimals: 18
 * Chain ID: 8453
 */
contract ETRToken is ERC20, ERC20Burnable, ERC20Permit, AccessControl, Pausable {
    bytes32 public constant BRIDGE_ROLE = keccak256("BRIDGE_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    uint256 public constant MAX_SUPPLY = 100_000_000_000 * 10**18;
    uint256 public constant MAX_MINT_PER_TX = 100_000 * 10**18;
    uint256 public constant MAX_MINT_PER_DAY = 1_000_000 * 10**18;

    uint256 public mintedToday;
    uint256 public lastMintDay;

    event BridgeMint(address indexed to, uint256 amount, bytes32 txHash);
    event BridgeBurn(address indexed from, uint256 amount, string etridAddress);

    constructor(
        address admin,
        address bridge
    ) ERC20("Etrid Coin (Base)", unicode"ÉTR") ERC20Permit("Etrid Coin") {
        require(admin != address(0), "ETR: zero admin address");
        require(bridge != address(0), "ETR: zero bridge address");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(BRIDGE_ROLE, bridge);
        _grantRole(PAUSER_ROLE, admin);

        _setRoleAdmin(BRIDGE_ROLE, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(PAUSER_ROLE, DEFAULT_ADMIN_ROLE);
    }

    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external onlyRole(BRIDGE_ROLE) whenNotPaused {
        require(to != address(0), "ETR: mint to zero address");
        require(amount > 0, "ETR: mint zero amount");
        require(amount <= MAX_MINT_PER_TX, "ETR: exceeds per-tx limit");
        require(totalSupply() + amount <= MAX_SUPPLY, "ETR: exceeds max supply");

        uint256 currentDay = block.timestamp / 1 days;
        if (currentDay > lastMintDay) {
            mintedToday = 0;
            lastMintDay = currentDay;
        }

        require(mintedToday + amount <= MAX_MINT_PER_DAY, "ETR: exceeds daily limit");
        mintedToday += amount;

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    function bridgeBurn(
        uint256 amount,
        string calldata etridAddress
    ) external whenNotPaused {
        require(amount > 0, "ETR: burn zero amount");
        require(bytes(etridAddress).length > 0, "ETR: empty etrid address");

        _burn(msg.sender, amount);
        emit BridgeBurn(msg.sender, amount, etridAddress);
    }

    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 amount
    ) internal override whenNotPaused {
        super._beforeTokenTransfer(from, to, amount);
    }
}
