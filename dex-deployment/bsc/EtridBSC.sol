// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EtridBSC
 * @dev ÉTR Token on Binance Smart Chain (BEP-20)
 *
 * Token Specifications:
 * - Name: Etrid Coin
 * - Symbol: ETR
 * - Decimals: 18 (BSC standard, differs from native 5 decimals)
 * - Initial Supply: 100,000 ÉTR (bootstrap liquidity)
 * - Bridge: Supports minting/burning for cross-chain bridge
 *
 * Governance: Controlled by Ëtrid Foundation 6-of-9 multisig
 * Charter Reference: FOUNDATION_CHARTER.md Section IV (Treasury Governance)
 */
contract EtridBSC is ERC20, ERC20Burnable, Ownable {

    // Bridge contract address (set after deployment)
    address public bridge;

    // Maximum supply cap (1 billion ÉTR with 18 decimals)
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    // Events
    event BridgeSet(address indexed oldBridge, address indexed newBridge);
    event BridgeMint(address indexed to, uint256 amount, bytes32 indexed txHash);
    event BridgeBurn(address indexed from, uint256 amount, bytes32 indexed txHash);

    /**
     * @dev Constructor mints initial supply for PancakeSwap liquidity
     * Initial mint: 100K ÉTR for bootstrap deployment
     */
    constructor() ERC20("Etrid Coin", "ETR") {
        // Mint 100K ÉTR to deployer (Foundation multisig)
        _mint(msg.sender, 100_000 * 10**18);
    }

    /**
     * @dev Set bridge contract address (only owner)
     * @param _bridge Address of the cross-chain bridge contract
     */
    function setBridge(address _bridge) external onlyOwner {
        require(_bridge != address(0), "Invalid bridge address");
        emit BridgeSet(bridge, _bridge);
        bridge = _bridge;
    }

    /**
     * @dev Mint ÉTR tokens when bridged from FlareChain
     * Only callable by authorized bridge contract
     * @param to Recipient address on BSC
     * @param amount Amount to mint (with 18 decimals)
     * @param txHash Transaction hash from source chain
     */
    function bridgeMint(
        address to,
        uint256 amount,
        bytes32 txHash
    ) external {
        require(msg.sender == bridge, "Only bridge can mint");
        require(to != address(0), "Invalid recipient");
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @dev Burn ÉTR tokens when bridging back to FlareChain
     * Only callable by authorized bridge contract
     * @param from Address to burn from
     * @param amount Amount to burn (with 18 decimals)
     * @param txHash Transaction hash for tracking
     */
    function bridgeBurn(
        address from,
        uint256 amount,
        bytes32 txHash
    ) external {
        require(msg.sender == bridge, "Only bridge can burn");
        require(from != address(0), "Invalid address");

        _burn(from, amount);
        emit BridgeBurn(from, amount, txHash);
    }

    /**
     * @dev Emergency pause function (not implemented)
     * Governance can upgrade contract if needed through timelock
     */

    /**
     * @dev Returns token metadata for display
     */
    function tokenInfo() external pure returns (
        string memory name,
        string memory symbol,
        uint8 decimals,
        string memory website,
        string memory description
    ) {
        return (
            "Etrid Coin",
            "ETR",
            18,
            "https://etrid.org",
            "Native token of Ëtrid Multichain Protocol - Decentralized, Democratic, Transparent"
        );
    }
}
