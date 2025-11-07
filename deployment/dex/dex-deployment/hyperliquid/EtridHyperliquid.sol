// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EtridHyperliquid
 * @dev ÉTR Token on Hyperliquid (ERC-20)
 *
 * Official ÉTR token deployment for Hyperliquid HyperEVM chain.
 * Designed for Hyperliquid Perps V3 integration and low-cost DeFi operations.
 *
 * Token Specifications:
 * - Name: Ëtrid Coin
 * - Symbol: ÉTR
 * - Decimals: 18 (Hyperliquid/EVM standard)
 * - Initial Supply: 15M ÉTR (for Hyperliquid Perps V3 liquidity)
 * - Max Supply: 1B ÉTR (enforced by bridge protocol)
 *
 * Reference Documents:
 * - FOUNDATION_CHARTER.md (governance)
 * - protocol-charter.md (token economics)
 * - COMPLETE_DEX_DEPLOYMENT_GUIDE.md (deployment strategy)
 *
 * @notice This contract is controlled by the Foundation multisig
 * @custom:security-contact security@etrid.org
 */
contract EtridHyperliquid is ERC20, ERC20Burnable, Ownable {

    /// @dev Maximum supply cap (1 billion ÉTR)
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    /// @dev Hyperliquid HyperEVM Bridge contract address (for Ethereum ↔ Hyperliquid bridging)
    address public hyperliquidBridge;

    /// @dev Cross-chain bridge contract address (for other chains)
    address public crossChainBridge;

    /// @notice Emitted when Hyperliquid bridge is updated
    event HyperliquidBridgeUpdated(address indexed oldBridge, address indexed newBridge);

    /// @notice Emitted when cross-chain bridge is updated
    event CrossChainBridgeUpdated(address indexed oldBridge, address indexed newBridge);

    /// @notice Emitted when tokens are minted via bridge
    event BridgeMint(address indexed to, uint256 amount, string sourceChain);

    /// @notice Emitted when tokens are burned via bridge
    event BridgeBurn(address indexed from, uint256 amount, string destinationChain);

    /**
     * @dev Constructor mints initial supply for Hyperliquid Perps V3 liquidity
     * @param initialOwner Foundation multisig address
     */
    constructor(address initialOwner) ERC20("Etrid Coin", "ETR") Ownable(initialOwner) {
        // Mint 100K ÉTR for bootstrap liquidity (expand later)
        _mint(initialOwner, 100_000 * 10**18);
    }

    /**
     * @dev Set the Hyperliquid HyperEVM bridge contract address
     * @param _hyperliquidBridge Address of the official Hyperliquid bridge
     *
     * Requirements:
     * - Caller must be owner (Foundation multisig)
     * - Bridge address cannot be zero
     */
    function setHyperliquidBridge(address _hyperliquidBridge) external onlyOwner {
        require(_hyperliquidBridge != address(0), "Bridge cannot be zero address");
        address oldBridge = hyperliquidBridge;
        hyperliquidBridge = _hyperliquidBridge;
        emit HyperliquidBridgeUpdated(oldBridge, _hyperliquidBridge);
    }

    /**
     * @dev Set the cross-chain bridge contract address (Wormhole, LayerZero, etc.)
     * @param _crossChainBridge Address of the cross-chain bridge
     *
     * Requirements:
     * - Caller must be owner (Foundation multisig)
     * - Bridge address cannot be zero
     */
    function setCrossChainBridge(address _crossChainBridge) external onlyOwner {
        require(_crossChainBridge != address(0), "Bridge cannot be zero address");
        address oldBridge = crossChainBridge;
        crossChainBridge = _crossChainBridge;
        emit CrossChainBridgeUpdated(oldBridge, _crossChainBridge);
    }

    /**
     * @dev Mint tokens when bridging TO Hyperliquid
     * @param to Recipient address on Hyperliquid
     * @param amount Amount to mint (in wei, 18 decimals)
     * @param sourceChain Name of source chain (for tracking)
     *
     * Requirements:
     * - Caller must be authorized bridge
     * - Total supply cannot exceed MAX_SUPPLY
     *
     * @notice This is called when users lock ÉTR on another chain and mint on Hyperliquid
     */
    function bridgeMint(address to, uint256 amount, string memory sourceChain) external {
        require(
            msg.sender == hyperliquidBridge || msg.sender == crossChainBridge,
            "Only authorized bridge can mint"
        );
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        require(to != address(0), "Cannot mint to zero address");

        _mint(to, amount);
        emit BridgeMint(to, amount, sourceChain);
    }

    /**
     * @dev Burn tokens when bridging FROM Hyperliquid
     * @param from Address to burn from
     * @param amount Amount to burn (in wei, 18 decimals)
     * @param destinationChain Name of destination chain (for tracking)
     *
     * Requirements:
     * - Caller must be authorized bridge
     * - Account must have sufficient balance
     *
     * @notice This is called when users burn ÉTR on Hyperliquid to unlock on another chain
     */
    function bridgeBurn(address from, uint256 amount, string memory destinationChain) external {
        require(
            msg.sender == hyperliquidBridge || msg.sender == crossChainBridge,
            "Only authorized bridge can burn"
        );
        require(balanceOf(from) >= amount, "Insufficient balance");

        _burn(from, amount);
        emit BridgeBurn(from, amount, destinationChain);
    }

    /**
     * @dev Emergency pause function (circuit breaker)
     * @notice Can only be called by owner in case of critical security issue
     *
     * This function is intentionally left empty but can be extended
     * to implement ERC20Pausable if needed in future upgrades.
     *
     * Requires 7-of-9 Foundation multisig per charter.
     */
    function emergencyPause() external onlyOwner {
        // Reserved for future emergency functionality
    }
}
