// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridTokenRegistry
 * @dev Interface for Token Registry Precompile (0x805)
 *
 * Auto-discovers and indexes ERC-20 tokens from Ethereum mainnet.
 * Reduces friction for cross-chain token bridging.
 *
 * Address: 0x0000000000000000000000000000000000000805
 *
 * @notice Novel Feature: Automatically fetches token metadata from Ethereum L1
 *         without requiring manual registration or trusted oracles
 */
interface IEtridTokenRegistry {
    /**
     * @notice Token metadata structure
     */
    struct TokenInfo {
        string name;
        string symbol;
        uint8 decimals;
        uint256 totalBridgedSupply;
    }

    /**
     * @notice Register a token from Ethereum mainnet
     * @param tokenAddress ERC-20 token address on Ethereum
     * @return success true if registration succeeded
     *
     * @dev Example:
     *   address USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
     *   bool registered = registry.registerToken(USDC);
     *   // Token metadata automatically fetched from Ethereum L1
     *
     * @dev Process:
     *   1. Query Ethereum L1 for token.name(), token.symbol(), token.decimals()
     *   2. Store metadata in on-chain registry
     *   3. Enable bridging for this token
     *   4. Emit TokenRegistered event
     */
    function registerToken(address tokenAddress)
        external
        returns (bool success);

    /**
     * @notice Get token information from registry
     * @param tokenAddress ERC-20 token address
     * @return name Token name
     * @return symbol Token symbol
     * @return decimals Token decimals
     * @return totalBridgedSupply Total supply bridged to ETH PBC
     *
     * @dev Example:
     *   (string memory name,
     *    string memory symbol,
     *    uint8 decimals,
     *    uint256 supply) = registry.getTokenInfo(USDC);
     *
     *   require(decimals == 6, "Unexpected decimals for USDC");
     */
    function getTokenInfo(address tokenAddress)
        external
        view
        returns (
            string memory name,
            string memory symbol,
            uint8 decimals,
            uint256 totalBridgedSupply
        );

    /**
     * @notice Get list of all bridged tokens
     * @return tokens Array of registered token addresses
     *
     * @dev Example:
     *   address[] memory tokens = registry.getBridgedTokens();
     *   for (uint i = 0; i < tokens.length; i++) {
     *       (string memory name,,,) = registry.getTokenInfo(tokens[i]);
     *       console.log("Bridged token:", name);
     *   }
     */
    function getBridgedTokens()
        external
        view
        returns (address[] memory tokens);

    /**
     * @notice Check if a token is registered
     * @param tokenAddress Token to check
     * @return registered true if token is in registry
     */
    function isTokenRegistered(address tokenAddress)
        external
        view
        returns (bool registered);

    /**
     * @notice Emitted when a new token is registered
     * @param tokenAddress Token that was registered
     * @param name Token name
     * @param symbol Token symbol
     * @param decimals Token decimals
     */
    event TokenRegistered(
        address indexed tokenAddress,
        string name,
        string symbol,
        uint8 decimals
    );
}
