// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridBridge
 * @dev Interface for XCM Bridge Precompile (0x804)
 *
 * Enables cross-chain transfers between ETH PBC and Primearc Core Chain via XCM.
 * Supports bidirectional asset transfers with atomic execution.
 *
 * Address: 0x0000000000000000000000000000000000000804
 *
 * @notice This is a simplified interface for the XCM bridge
 *         Full XCM functionality requires additional message encoding
 */
interface IEtridBridge {
    /**
     * @notice Bridge assets from ETH PBC to Primearc Core Chain
     * @param amount Amount of ETH to bridge (in wei)
     * @return messageId XCM message identifier for tracking
     *
     * @dev Example:
     *   // Bridge 5 ETH to Primearc Core Chain
     *   bytes32 msgId = bridge.bridgeToPrimearc Core Chain{value: 5 ether}(5 ether);
     *   // Track via msgId - funds arrive on Primearc Core Chain after finality
     *
     * @dev Execution:
     *   1. ETH locked in precompile reserve
     *   2. XCM message sent to Primearc Core Chain
     *   3. Equivalent assets minted on Primearc Core Chain
     *   4. Receipt emitted when confirmed
     */
    function bridgeToPrimearc Core Chain(uint256 amount)
        external
        payable
        returns (bytes32 messageId);

    /**
     * @notice Bridge assets from Primearc Core Chain to ETH PBC
     * @param amount Amount to bridge from Primearc Core Chain
     * @return messageId XCM message identifier
     *
     * @dev This initiates the reverse bridge operation
     *      Requires sufficient balance on Primearc Core Chain
     *
     * @dev Example:
     *   bytes32 msgId = bridge.bridgeFromPrimearc Core Chain(3 ether);
     *   // Wait for XCM message processing
     *   // ETH will be released from reserve to msg.sender
     */
    function bridgeFromPrimearc Core Chain(uint256 amount)
        external
        returns (bytes32 messageId);

    /**
     * @notice Check status of a bridge message
     * @param messageId XCM message ID
     * @return status 0=Pending, 1=Confirmed, 2=Failed
     * @return amount Amount bridged (if confirmed)
     *
     * @dev Example:
     *   (uint8 status, uint256 amt) = bridge.getBridgeStatus(msgId);
     *   if (status == 1) {
     *       // Bridge completed successfully
     *   }
     */
    function getBridgeStatus(bytes32 messageId)
        external
        view
        returns (uint8 status, uint256 amount);

    /**
     * @notice Get total amount bridged to Primearc Core Chain
     * @return total Total ETH locked in bridge reserve
     */
    function getTotalBridgedToPrimearc Core Chain()
        external
        view
        returns (uint256 total);

    /**
     * @notice Emitted when assets are bridged to Primearc Core Chain
     * @param sender Address that initiated bridge
     * @param amount Amount bridged
     * @param messageId XCM message ID
     */
    event BridgedToPrimearc Core Chain(
        address indexed sender,
        uint256 amount,
        bytes32 indexed messageId
    );

    /**
     * @notice Emitted when assets are received from Primearc Core Chain
     * @param recipient Address receiving bridged assets
     * @param amount Amount received
     * @param messageId XCM message ID
     */
    event BridgedFromPrimearc Core Chain(
        address indexed recipient,
        uint256 amount,
        bytes32 indexed messageId
    );
}
