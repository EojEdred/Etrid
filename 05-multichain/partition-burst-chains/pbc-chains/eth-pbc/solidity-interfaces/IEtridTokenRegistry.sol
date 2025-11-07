// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEtridTokenRegistry
 * @notice Interface for auto-discovering and managing bridged ERC-20 tokens
 * @dev Precompile address: 0x0000000000000000000000000000000000000805
 *
 * This precompile automatically discovers ERC-20 tokens from Ethereum mainnet
 * and maintains a registry of bridged tokens with their metadata.
 *
 * Features:
 * - Auto-fetch token name, symbol, decimals from mainnet
 * - Track total bridged supply
 * - List all registered tokens
 * - No manual configuration required
 *
 * Example Usage:
 * ```solidity
 * IEtridTokenRegistry registry = IEtridTokenRegistry(0x0000000000000000000000000000000000000805);
 *
 * // Register USDC from mainnet
 * registry.registerToken(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48);
 *
 * // Get token info
 * (string memory name, string memory symbol, uint8 decimals, uint256 supply)
 *     = registry.getTokenInfo(usdcAddress);
 * ```
 */
interface IEtridTokenRegistry {
    /**
     * @notice Emitted when a new token is registered
     * @param mainnetToken The mainnet address of the registered token
     */
    event TokenRegistered(address indexed mainnetToken);

    /**
     * @notice Emitted when token metadata is updated
     * @param token The token address
     * @param name The token name
     * @param symbol The token symbol
     */
    event TokenMetadataUpdated(address indexed token, string name, string symbol);

    /**
     * @notice Auto-register an ERC-20 token from Ethereum mainnet
     * @param mainnetToken The mainnet address of the token to register
     * @return success True if registration was successful
     *
     * This function:
     * 1. Queries mainnet for token metadata (name, symbol, decimals)
     * 2. Stores metadata in on-chain registry
     * 3. Initializes bridged supply tracking
     * 4. Emits TokenRegistered event
     *
     * Requirements:
     * - Token must be a valid ERC-20 contract on mainnet
     * - Token must implement name(), symbol(), decimals()
     * - Token not already registered
     *
     * Use Cases:
     * - Bridge new tokens without manual configuration
     * - Discover token metadata automatically
     * - Reduce friction for multi-token bridges
     *
     * Example:
     * ```solidity
     * // Register popular tokens
     * registry.registerToken(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48); // USDC
     * registry.registerToken(0xdAC17F958D2ee523a2206206994597C13D831ec7); // USDT
     * registry.registerToken(0x6B175474E89094C44Da98b954EedeAC495271d0F); // DAI
     * ```
     */
    function registerToken(address mainnetToken) external returns (bool success);

    /**
     * @notice Get comprehensive token information
     * @param token The token address to query
     * @return name The full token name (e.g., "USD Coin")
     * @return symbol The token symbol (e.g., "USDC")
     * @return decimals The number of decimals (e.g., 6 for USDC)
     * @return totalBridgedSupply The total amount bridged to ETH-PBC
     *
     * Returns complete metadata for a registered token including the
     * total supply that has been bridged from mainnet.
     *
     * Example:
     * ```solidity
     * (string memory name, string memory symbol, uint8 decimals, uint256 supply)
     *     = registry.getTokenInfo(usdcAddress);
     *
     * console.log("Token:", name, symbol);
     * console.log("Decimals:", decimals);
     * console.log("Bridged:", supply / 10**decimals);
     * ```
     */
    function getTokenInfo(address token)
        external
        view
        returns (
            string memory name,
            string memory symbol,
            uint8 decimals,
            uint256 totalBridgedSupply
        );

    /**
     * @notice Get list of all registered bridged tokens
     * @return tokens Array of token addresses
     *
     * Returns all tokens that have been registered and bridged to ETH-PBC.
     * Useful for:
     * - Building token lists for UIs
     * - Iterating over all bridged assets
     * - Discovery of available tokens
     *
     * Example:
     * ```solidity
     * address[] memory tokens = registry.getBridgedTokens();
     *
     * for (uint i = 0; i < tokens.length; i++) {
     *     (string memory name, string memory symbol, , uint256 supply)
     *         = registry.getTokenInfo(tokens[i]);
     *
     *     console.log(name, symbol, "Supply:", supply);
     * }
     * ```
     */
    function getBridgedTokens() external view returns (address[] memory tokens);
}

/**
 * @title Advanced Usage Examples
 * @dev These examples show common patterns for token registry usage
 */
contract TokenRegistryExamples {
    IEtridTokenRegistry constant REGISTRY = IEtridTokenRegistry(0x0000000000000000000000000000000000000805);

    /**
     * @notice Example: Auto-register multiple tokens
     */
    function registerPopularTokens() external {
        // Mainnet addresses of popular tokens
        address[] memory tokens = new address[](5);
        tokens[0] = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48; // USDC
        tokens[1] = 0xdAC17F958D2ee523a2206206994597C13D831ec7; // USDT
        tokens[2] = 0x6B175474E89094C44Da98b954EedeAC495271d0F; // DAI
        tokens[3] = 0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599; // WBTC
        tokens[4] = 0x514910771AF9Ca656af840dff83E8264EcF986CA; // LINK

        for (uint i = 0; i < tokens.length; i++) {
            REGISTRY.registerToken(tokens[i]);
        }
    }

    /**
     * @notice Example: Get token display info
     */
    function getTokenDisplay(address token)
        external
        view
        returns (string memory display)
    {
        (string memory name, string memory symbol, uint8 decimals, )
            = REGISTRY.getTokenInfo(token);

        // Format: "USD Coin (USDC) - 6 decimals"
        return string(abi.encodePacked(name, " (", symbol, ") - ", uint2str(decimals), " decimals"));
    }

    /**
     * @notice Example: Calculate total bridged value
     */
    function getTotalBridgedValue() external view returns (uint256 totalValue) {
        address[] memory tokens = REGISTRY.getBridgedTokens();

        for (uint i = 0; i < tokens.length; i++) {
            (, , uint8 decimals, uint256 supply) = REGISTRY.getTokenInfo(tokens[i]);

            // Normalize to 18 decimals and accumulate
            if (decimals < 18) {
                totalValue += supply * 10 ** (18 - decimals);
            } else {
                totalValue += supply / 10 ** (decimals - 18);
            }
        }
    }

    /**
     * @notice Example: Check if token is registered
     */
    function isTokenRegistered(address token) external view returns (bool) {
        try REGISTRY.getTokenInfo(token) returns (string memory, string memory, uint8, uint256) {
            return true;
        } catch {
            return false;
        }
    }

    /**
     * @notice Example: Multi-token bridge with auto-registration
     */
    function bridgeTokenIfNeeded(address mainnetToken, uint256 amount) external {
        // Auto-register if not already registered
        if (!isRegistered(mainnetToken)) {
            REGISTRY.registerToken(mainnetToken);
        }

        // Get token metadata
        (, string memory symbol, uint8 decimals, ) = REGISTRY.getTokenInfo(mainnetToken);

        // Proceed with bridge logic...
        // bridge(mainnetToken, amount, decimals);
    }

    /**
     * @notice Example: Token whitelist check
     */
    mapping(address => bool) public trustedTokens;

    function addTrustedToken(address token) external {
        // Verify token is registered
        (, string memory symbol, , ) = REGISTRY.getTokenInfo(token);
        require(bytes(symbol).length > 0, "Token not registered");

        trustedTokens[token] = true;
    }

    // Helper functions
    function isRegistered(address token) private view returns (bool) {
        try REGISTRY.getTokenInfo(token) returns (string memory, string memory, uint8, uint256) {
            return true;
        } catch {
            return false;
        }
    }

    function uint2str(uint _i) private pure returns (string memory) {
        if (_i == 0) {
            return "0";
        }
        uint j = _i;
        uint len;
        while (j != 0) {
            len++;
            j /= 10;
        }
        bytes memory bstr = new bytes(len);
        uint k = len;
        while (_i != 0) {
            k = k-1;
            uint8 temp = (48 + uint8(_i - _i / 10 * 10));
            bytes1 b1 = bytes1(temp);
            bstr[k] = b1;
            _i /= 10;
        }
        return string(bstr);
    }
}

/**
 * @title Token Discovery Dashboard
 * @dev Example contract for building a token discovery UI
 */
contract TokenDiscoveryDashboard {
    IEtridTokenRegistry constant REGISTRY = IEtridTokenRegistry(0x0000000000000000000000000000000000000805);

    struct TokenStats {
        address tokenAddress;
        string name;
        string symbol;
        uint8 decimals;
        uint256 bridgedSupply;
        uint256 normalizedSupply; // Normalized to 18 decimals
    }

    /**
     * @notice Get comprehensive stats for all bridged tokens
     */
    function getAllTokenStats() external view returns (TokenStats[] memory) {
        address[] memory tokens = REGISTRY.getBridgedTokens();
        TokenStats[] memory stats = new TokenStats[](tokens.length);

        for (uint i = 0; i < tokens.length; i++) {
            (string memory name, string memory symbol, uint8 decimals, uint256 supply)
                = REGISTRY.getTokenInfo(tokens[i]);

            // Normalize supply to 18 decimals for comparison
            uint256 normalized;
            if (decimals < 18) {
                normalized = supply * 10 ** (18 - decimals);
            } else {
                normalized = supply / 10 ** (decimals - 18);
            }

            stats[i] = TokenStats({
                tokenAddress: tokens[i],
                name: name,
                symbol: symbol,
                decimals: decimals,
                bridgedSupply: supply,
                normalizedSupply: normalized
            });
        }

        return stats;
    }
}
