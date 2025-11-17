// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridBridge
 * @dev Interface for XCM Bridge Precompile (0x804)
 *
 * Enables cross-chain transfers between ETH PBC and FlareChain via XCM.
 * Supports bidirectional asset transfers with atomic execution.
 *
 * Address: 0x0000000000000000000000000000000000000804
 *
 * @notice This is a simplified interface for the XCM bridge
 *         Full XCM functionality requires additional message encoding
 */
interface IEtridBridge {
    /**
     * @notice Bridge assets from ETH PBC to FlareChain
     * @param amount Amount of ETH to bridge (in wei)
     * @return messageId XCM message identifier for tracking
     *
     * @dev Example:
     *   // Bridge 5 ETH to FlareChain
     *   bytes32 msgId = bridge.bridgeToFlareChain{value: 5 ether}(5 ether);
     *   // Track via msgId - funds arrive on FlareChain after finality
     *
     * @dev Execution:
     *   1. ETH locked in precompile reserve
     *   2. XCM message sent to FlareChain
     *   3. Equivalent assets minted on FlareChain
     *   4. Receipt emitted when confirmed
     */
    function bridgeToFlareChain(uint256 amount)
        external
        payable
        returns (bytes32 messageId);

    /**
     * @notice Bridge assets from FlareChain to ETH PBC
     * @param amount Amount to bridge from FlareChain
     * @return messageId XCM message identifier
     *
     * @dev This initiates the reverse bridge operation
     *      Requires sufficient balance on FlareChain
     *
     * @dev Example:
     *   bytes32 msgId = bridge.bridgeFromFlareChain(3 ether);
     *   // Wait for XCM message processing
     *   // ETH will be released from reserve to msg.sender
     */
    function bridgeFromFlareChain(uint256 amount)
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
     * @notice Get total amount bridged to FlareChain
     * @return total Total ETH locked in bridge reserve
     */
    function getTotalBridgedToFlareChain()
        external
        view
        returns (uint256 total);

    /**
     * @notice Emitted when assets are bridged to FlareChain
     * @param sender Address that initiated bridge
     * @param amount Amount bridged
     * @param messageId XCM message ID
     */
    event BridgedToFlareChain(
        address indexed sender,
        uint256 amount,
        bytes32 indexed messageId
    );

    /**
     * @notice Emitted when assets are received from FlareChain
     * @param recipient Address receiving bridged assets
     * @param amount Amount received
     * @param messageId XCM message ID
     */
    event BridgedFromFlareChain(
        address indexed recipient,
        uint256 amount,
        bytes32 indexed messageId
    );
}
