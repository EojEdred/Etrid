// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./WrappedToken.sol";

/**
 * @title WrappedTokenFactory
 * @notice Factory for deploying wrapped tokens for cross-chain bridges
 */
contract WrappedTokenFactory is Ownable {
    /// @notice Deployed wrapped tokens
    mapping(bytes32 => address) public wrappedTokens;

    /// @notice Bridge contract that can mint/burn
    address public bridge;

    event WrappedTokenCreated(
        bytes32 indexed tokenId,
        address indexed wrappedToken,
        string name,
        string symbol,
        string originalChain
    );

    event BridgeUpdated(address indexed newBridge);

    error TokenExists();
    error ZeroAddress();

    constructor(address _bridge) Ownable(msg.sender) {
        if (_bridge == address(0)) revert ZeroAddress();
        bridge = _bridge;
    }

    /**
     * @notice Create a new wrapped token
     * @param name Token name (e.g., "Wrapped Bitcoin")
     * @param symbol Token symbol (e.g., "wBTC")
     * @param decimals Token decimals
     * @param originalChain Original chain name (e.g., "Bitcoin")
     * @param originalSymbol Original token symbol (e.g., "BTC")
     * @param originalAddress Original token address (or zero for native)
     * @return wrappedToken Address of deployed wrapped token
     */
    function createWrappedToken(
        string memory name,
        string memory symbol,
        uint8 decimals,
        string memory originalChain,
        string memory originalSymbol,
        address originalAddress
    ) external onlyOwner returns (address wrappedToken) {
        // Generate unique token ID
        bytes32 tokenId = keccak256(abi.encodePacked(
            originalChain,
            originalSymbol,
            originalAddress
        ));

        if (wrappedTokens[tokenId] != address(0)) revert TokenExists();

        // Deploy wrapped token
        WrappedToken token = new WrappedToken(
            name,
            symbol,
            decimals,
            originalChain,
            originalSymbol,
            originalAddress,
            address(this)
        );

        // Grant bridge minter/burner role
        token.grantMinterRole(bridge);

        wrappedToken = address(token);
        wrappedTokens[tokenId] = wrappedToken;

        emit WrappedTokenCreated(tokenId, wrappedToken, name, symbol, originalChain);

        return wrappedToken;
    }

    /**
     * @notice Update bridge contract
     */
    function setBridge(address newBridge) external onlyOwner {
        if (newBridge == address(0)) revert ZeroAddress();
        bridge = newBridge;
        emit BridgeUpdated(newBridge);
    }

    /**
     * @notice Grant minter role to new bridge for existing token
     */
    function grantBridgeRole(address token) external onlyOwner {
        WrappedToken(token).grantMinterRole(bridge);
    }
}
