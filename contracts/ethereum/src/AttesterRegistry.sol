// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/MessageHashUtils.sol";

/**
 * @title AttesterRegistry
 * @notice Manages registered attesters and verifies M-of-N threshold signatures
 * @dev Follows Circle CCTP attestation model
 *
 * Key Features:
 * - Register/remove attesters (governance controlled)
 * - Enable/disable attesters without removing
 * - Verify M-of-N threshold signatures (e.g., 3-of-5)
 * - Prevent signature reuse via nonce tracking
 * - Support message hash verification
 *
 * Security Model:
 * - Only owner (governance) can modify attesters
 * - Attesters cannot remove themselves
 * - Threshold is configurable per domain
 * - Signature verification uses ECDSA recovery
 */
contract AttesterRegistry is Ownable2Step {
    using ECDSA for bytes32;

    /// @notice Attester information
    struct AttesterInfo {
        address attesterAddress;  // Ethereum address of attester
        bool enabled;             // Active status
        uint256 registeredAt;     // Block number when registered
        uint256 messagesSigned;   // Statistics counter
    }

    /// @notice Threshold configuration per domain
    struct ThresholdConfig {
        uint32 minSignatures;     // M in M-of-N
        uint32 totalAttesters;    // N in M-of-N
        bool enabled;             // Configuration active
    }

    /// @notice Mapping of attester address to info
    mapping(address => AttesterInfo) public attesters;

    /// @notice Array of all attester addresses (for enumeration)
    address[] public attesterList;

    /// @notice Count of enabled attesters
    uint256 public enabledAttesterCount;

    /// @notice Threshold configuration per domain (domain => config)
    mapping(uint32 => ThresholdConfig) public thresholdConfigs;

    /// @notice Global threshold configuration (used when domain-specific not set)
    ThresholdConfig public globalThreshold;

    /// @notice Track used nonces per domain to prevent replay attacks
    /// domain => nonce => used
    mapping(uint32 => mapping(uint64 => bool)) public usedNonces;

    /// @notice Pause state
    bool public paused;

    // Events
    event AttesterRegistered(address indexed attester);
    event AttesterRemoved(address indexed attester);
    event AttesterEnabled(address indexed attester);
    event AttesterDisabled(address indexed attester);
    event ThresholdConfigured(uint32 indexed domain, uint32 minSignatures, uint32 totalAttesters);
    event NonceUsed(uint32 indexed domain, uint64 indexed nonce);
    event PauseStateChanged(bool paused);

    // Errors
    error Paused();
    error AttesterAlreadyRegistered();
    error AttesterNotRegistered();
    error AttesterAlreadyEnabled();
    error AttesterAlreadyDisabled();
    error ZeroAddress();
    error InvalidThreshold();
    error InsufficientSignatures();
    error InvalidSignature();
    error DuplicateSignature();
    error NonceAlreadyUsed();
    error AttesterNotEnabled();

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /**
     * @notice Constructor
     * @param _owner Initial owner (governance multisig)
     * @param _minSignatures Initial global threshold (M)
     * @param _totalAttesters Initial total attesters (N)
     */
    constructor(address _owner, uint32 _minSignatures, uint32 _totalAttesters) Ownable(_owner) {
        if (_owner == address(0)) revert ZeroAddress();
        if (_minSignatures == 0 || _minSignatures > _totalAttesters) revert InvalidThreshold();

        globalThreshold = ThresholdConfig({
            minSignatures: _minSignatures,
            totalAttesters: _totalAttesters,
            enabled: true
        });

        emit ThresholdConfigured(0, _minSignatures, _totalAttesters);
    }

    /**
     * @notice Register a new attester
     * @param _attester Address of the attester to register
     */
    function registerAttester(address _attester) external onlyOwner {
        if (_attester == address(0)) revert ZeroAddress();
        if (attesters[_attester].attesterAddress != address(0)) revert AttesterAlreadyRegistered();

        attesters[_attester] = AttesterInfo({
            attesterAddress: _attester,
            enabled: true,
            registeredAt: block.number,
            messagesSigned: 0
        });

        attesterList.push(_attester);
        enabledAttesterCount++;

        emit AttesterRegistered(_attester);
    }

    /**
     * @notice Remove an attester
     * @param _attester Address of the attester to remove
     */
    function removeAttester(address _attester) external onlyOwner {
        if (attesters[_attester].attesterAddress == address(0)) revert AttesterNotRegistered();

        if (attesters[_attester].enabled) {
            enabledAttesterCount--;
        }

        delete attesters[_attester];

        // Remove from attesterList
        for (uint256 i = 0; i < attesterList.length; i++) {
            if (attesterList[i] == _attester) {
                attesterList[i] = attesterList[attesterList.length - 1];
                attesterList.pop();
                break;
            }
        }

        emit AttesterRemoved(_attester);
    }

    /**
     * @notice Enable a disabled attester
     * @param _attester Address of the attester to enable
     */
    function enableAttester(address _attester) external onlyOwner {
        if (attesters[_attester].attesterAddress == address(0)) revert AttesterNotRegistered();
        if (attesters[_attester].enabled) revert AttesterAlreadyEnabled();

        attesters[_attester].enabled = true;
        enabledAttesterCount++;

        emit AttesterEnabled(_attester);
    }

    /**
     * @notice Disable an attester
     * @param _attester Address of the attester to disable
     */
    function disableAttester(address _attester) external onlyOwner {
        if (attesters[_attester].attesterAddress == address(0)) revert AttesterNotRegistered();
        if (!attesters[_attester].enabled) revert AttesterAlreadyDisabled();

        attesters[_attester].enabled = false;
        enabledAttesterCount--;

        emit AttesterDisabled(_attester);
    }

    /**
     * @notice Configure threshold for a domain
     * @param _domain Domain ID (0 for global)
     * @param _minSignatures Minimum signatures required (M)
     * @param _totalAttesters Total attesters (N)
     */
    function configureThreshold(uint32 _domain, uint32 _minSignatures, uint32 _totalAttesters)
        external
        onlyOwner
    {
        if (_minSignatures == 0 || _minSignatures > _totalAttesters) revert InvalidThreshold();

        ThresholdConfig memory config = ThresholdConfig({
            minSignatures: _minSignatures,
            totalAttesters: _totalAttesters,
            enabled: true
        });

        if (_domain == 0) {
            globalThreshold = config;
        } else {
            thresholdConfigs[_domain] = config;
        }

        emit ThresholdConfigured(_domain, _minSignatures, _totalAttesters);
    }

    /**
     * @notice Pause the registry
     */
    function pause() external onlyOwner {
        paused = true;
        emit PauseStateChanged(true);
    }

    /**
     * @notice Unpause the registry
     */
    function unpause() external onlyOwner {
        paused = false;
        emit PauseStateChanged(false);
    }

    /**
     * @notice Verify M-of-N signatures for a message
     * @param _messageHash Hash of the message to verify
     * @param _signatures Array of signatures (65 bytes each)
     * @param _domain Domain ID for threshold lookup
     * @param _nonce Unique nonce to prevent replay
     * @return bool True if verification passes
     */
    function verifySignatures(
        bytes32 _messageHash,
        bytes[] calldata _signatures,
        uint32 _domain,
        uint64 _nonce
    ) external whenNotPaused returns (bool) {
        // Check nonce not already used
        if (usedNonces[_domain][_nonce]) revert NonceAlreadyUsed();

        // Get threshold for domain
        ThresholdConfig memory threshold = thresholdConfigs[_domain].enabled
            ? thresholdConfigs[_domain]
            : globalThreshold;

        // Check minimum signatures met
        if (_signatures.length < threshold.minSignatures) revert InsufficientSignatures();

        // Verify each signature
        address[] memory signers = new address[](_signatures.length);

        for (uint256 i = 0; i < _signatures.length; i++) {
            // Recover signer address from signature
            address signer = ECDSA.recover(MessageHashUtils.toEthSignedMessageHash(_messageHash), _signatures[i]);

            // Check attester is registered and enabled
            if (attesters[signer].attesterAddress == address(0)) revert InvalidSignature();
            if (!attesters[signer].enabled) revert AttesterNotEnabled();

            // Check for duplicate signers
            for (uint256 j = 0; j < i; j++) {
                if (signers[j] == signer) revert DuplicateSignature();
            }

            signers[i] = signer;

            // Update statistics
            attesters[signer].messagesSigned++;
        }

        // Mark nonce as used
        usedNonces[_domain][_nonce] = true;
        emit NonceUsed(_domain, _nonce);

        return true;
    }

    /**
     * @notice Get threshold for a domain
     * @param _domain Domain ID
     * @return minSignatures Minimum signatures required
     * @return totalAttesters Total attesters
     */
    function getThreshold(uint32 _domain)
        external
        view
        returns (uint32 minSignatures, uint32 totalAttesters)
    {
        ThresholdConfig memory threshold = thresholdConfigs[_domain].enabled
            ? thresholdConfigs[_domain]
            : globalThreshold;

        return (threshold.minSignatures, threshold.totalAttesters);
    }

    /**
     * @notice Get total number of attesters
     * @return count Total attesters registered
     */
    function getAttesterCount() external view returns (uint256) {
        return attesterList.length;
    }

    /**
     * @notice Check if an address is an enabled attester
     * @param _attester Address to check
     * @return bool True if enabled attester
     */
    function isEnabledAttester(address _attester) external view returns (bool) {
        return attesters[_attester].enabled;
    }
}
