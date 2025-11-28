// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "./EDSC.sol";
import "./AttesterRegistry.sol";

/**
 * @title EDSCMessageTransmitter
 * @notice Receives and processes cross-chain messages from Ëtrid
 * @dev Verifies attestations and mints EDSC tokens
 *
 * Message Flow (Ëtrid → Ethereum):
 * 1. User burns EDSC on Ëtrid via TokenMessenger
 * 2. Attesters sign the burn message
 * 3. Relayer calls receiveMessage() with message + signatures
 * 4. This contract verifies signatures via AttesterRegistry
 * 5. This contract mints EDSC to recipient
 *
 * Security Model:
 * - M-of-N threshold signature verification
 * - Nonce-based replay protection
 * - Domain validation
 * - Pausable for emergencies
 */
contract EDSCMessageTransmitter is Ownable2Step {
    /// @notice EDSC token contract
    EDSC public immutable edscToken;

    /// @notice AttesterRegistry contract
    AttesterRegistry public immutable attesterRegistry;

    /// @notice This chain's domain ID
    uint32 public constant LOCAL_DOMAIN = 0; // Ethereum

    /// @notice Ëtrid domain ID
    uint32 public constant ETRID_DOMAIN = 2;

    /// @notice Pause state
    bool public paused;

    /// @notice Cross-chain message structure
    struct CrossChainMessage {
        uint32 version;
        uint32 sourceDomain;
        uint32 destinationDomain;
        uint64 nonce;
        bytes sender;           // Ëtrid account (32 bytes)
        bytes recipient;        // Ethereum address (20 bytes)
        bytes messageBody;      // Encoded BurnMessage
    }

    /// @notice Burn message structure (from Ëtrid)
    struct BurnMessage {
        uint32 version;
        bytes burnToken;        // "EDSC" identifier
        bytes mintRecipient;    // Ethereum address
        uint128 amount;         // Amount with 18 decimals
    }

    /// @notice Statistics
    uint256 public totalMessagesReceived;
    uint256 public totalEDSCMinted;

    // Events
    event MessageReceived(
        uint32 indexed sourceDomain,
        uint64 indexed nonce,
        address indexed recipient,
        uint256 amount
    );

    event EDSCMinted(
        address indexed recipient,
        uint256 amount,
        uint64 nonce
    );

    event PauseStateChanged(bool paused);

    // Errors
    error Paused();
    error InvalidDomain();
    error InvalidVersion();
    error MessageAlreadyProcessed();
    error AttestationVerificationFailed();
    error InvalidRecipient();
    error InvalidAmount();
    error DecodingFailed();

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /**
     * @notice Constructor
     * @param _owner Initial owner (governance multisig)
     * @param _edscToken EDSC token contract address
     * @param _attesterRegistry AttesterRegistry contract address
     */
    constructor(
        address _owner,
        address _edscToken,
        address _attesterRegistry
    ) Ownable(_owner) {
        if (_owner == address(0)) revert InvalidRecipient();
        if (_edscToken == address(0)) revert InvalidRecipient();
        if (_attesterRegistry == address(0)) revert InvalidRecipient();

        edscToken = EDSC(_edscToken);
        attesterRegistry = AttesterRegistry(_attesterRegistry);
    }

    /**
     * @notice Receive and process a cross-chain message from Ëtrid
     * @param _message Encoded CrossChainMessage
     * @param _signatures Array of attester signatures
     */
    function receiveMessage(bytes calldata _message, bytes[] calldata _signatures)
        external
        whenNotPaused
    {
        // Decode message
        CrossChainMessage memory message = _decodeCrossChainMessage(_message);

        // Validate message
        if (message.version != 1) revert InvalidVersion();
        if (message.sourceDomain != ETRID_DOMAIN) revert InvalidDomain();
        if (message.destinationDomain != LOCAL_DOMAIN) revert InvalidDomain();

        // Compute message hash
        bytes32 messageHash = keccak256(_message);

        // Verify attestation signatures (also checks nonce usage)
        bool verified = attesterRegistry.verifySignatures(
            messageHash,
            _signatures,
            message.sourceDomain,
            message.nonce
        );

        if (!verified) revert AttestationVerificationFailed();

        // Decode burn message from body
        BurnMessage memory burnMsg = _decodeBurnMessage(message.messageBody);

        // Validate burn message
        if (burnMsg.version != 1) revert InvalidVersion();
        if (burnMsg.amount == 0) revert InvalidAmount();

        // Extract recipient address
        address recipient = _bytesToAddress(burnMsg.mintRecipient);
        if (recipient == address(0)) revert InvalidRecipient();

        // Mint EDSC to recipient
        edscToken.mint(recipient, uint256(burnMsg.amount), message.nonce);

        // Update statistics
        totalMessagesReceived++;
        totalEDSCMinted += uint256(burnMsg.amount);

        emit MessageReceived(
            message.sourceDomain,
            message.nonce,
            recipient,
            uint256(burnMsg.amount)
        );

        emit EDSCMinted(recipient, uint256(burnMsg.amount), message.nonce);
    }

    /**
     * @notice Pause message processing
     */
    function pause() external onlyOwner {
        paused = true;
        emit PauseStateChanged(true);
    }

    /**
     * @notice Unpause message processing
     */
    function unpause() external onlyOwner {
        paused = false;
        emit PauseStateChanged(false);
    }

    /**
     * @notice Decode CrossChainMessage from bytes
     * @param _data Encoded message data
     * @return message Decoded CrossChainMessage
     */
    function _decodeCrossChainMessage(bytes calldata _data)
        internal
        pure
        returns (CrossChainMessage memory message)
    {
        // Manual decoding to match Substrate SCALE encoding
        uint256 offset = 0;

        // Version (4 bytes)
        bytes memory versionBytes = _sliceBytes(_data, offset, 4);
        message.version = uint32(bytes4(versionBytes));
        offset += 4;

        // Source domain (4 bytes)
        bytes memory sourceBytes = _sliceBytes(_data, offset, 4);
        message.sourceDomain = uint32(bytes4(sourceBytes));
        offset += 4;

        // Destination domain (4 bytes)
        bytes memory destBytes = _sliceBytes(_data, offset, 4);
        message.destinationDomain = uint32(bytes4(destBytes));
        offset += 4;

        // Nonce (8 bytes)
        bytes memory nonceBytes = _sliceBytes(_data, offset, 8);
        message.nonce = uint64(bytes8(nonceBytes));
        offset += 8;

        // Sender (BoundedVec<u8, 64>)
        // First byte is length
        uint8 senderLen = uint8(_data[offset]);
        offset += 1;
        message.sender = _sliceBytes(_data, offset, senderLen);
        offset += senderLen;

        // Recipient (BoundedVec<u8, 64>)
        // First byte is length
        uint8 recipientLen = uint8(_data[offset]);
        offset += 1;
        message.recipient = _sliceBytes(_data, offset, recipientLen);
        offset += recipientLen;

        // Message body (BoundedVec<u8, 512>)
        // First 2 bytes are length (u16)
        bytes memory bodyLenBytes = _sliceBytes(_data, offset, 2);
        uint16 bodyLen = uint16(bytes2(bodyLenBytes));
        offset += 2;
        message.messageBody = _sliceBytes(_data, offset, bodyLen);
    }

    /**
     * @notice Extract a slice from bytes array
     * @param _data Source bytes
     * @param _start Start index
     * @param _length Number of bytes to extract
     * @return Extracted bytes
     */
    function _sliceBytes(bytes memory _data, uint256 _start, uint256 _length)
        internal
        pure
        returns (bytes memory)
    {
        require(_start + _length <= _data.length, "Slice out of bounds");

        bytes memory result = new bytes(_length);
        for (uint256 i = 0; i < _length; i++) {
            result[i] = _data[_start + i];
        }
        return result;
    }

    /**
     * @notice Decode BurnMessage from bytes
     * @param _data Encoded burn message data
     * @return burnMsg Decoded BurnMessage
     */
    function _decodeBurnMessage(bytes memory _data)
        internal
        pure
        returns (BurnMessage memory burnMsg)
    {
        uint256 offset = 0;

        // Version (4 bytes)
        bytes memory versionBytes = _sliceBytes(_data, offset, 4);
        burnMsg.version = uint32(bytes4(versionBytes));
        offset += 4;

        // Burn token (BoundedVec<u8, 64>)
        uint8 tokenLen = uint8(_data[offset]);
        offset += 1;
        burnMsg.burnToken = _sliceBytes(_data, offset, tokenLen);
        offset += tokenLen;

        // Mint recipient (BoundedVec<u8, 64>)
        uint8 recipientLen = uint8(_data[offset]);
        offset += 1;
        burnMsg.mintRecipient = _sliceBytes(_data, offset, recipientLen);
        offset += recipientLen;

        // Amount (16 bytes for u128)
        bytes memory amountBytes = _sliceBytes(_data, offset, 16);
        burnMsg.amount = uint128(bytes16(amountBytes));
    }

    /**
     * @notice Convert bytes to address
     * @param _bytes Bytes representation of address
     * @return addr Address
     */
    function _bytesToAddress(bytes memory _bytes) internal pure returns (address addr) {
        if (_bytes.length != 20) revert InvalidRecipient();

        assembly {
            addr := mload(add(_bytes, 20))
        }
    }

    /**
     * @notice Get message reception statistics
     * @return received Total messages received
     * @return minted Total EDSC minted
     */
    function getStatistics() external view returns (uint256 received, uint256 minted) {
        return (totalMessagesReceived, totalEDSCMinted);
    }
}
