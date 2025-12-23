// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title AttesterRegistry
 * @notice ETRID 9-Director Attestation Registry for cross-chain bridge operations
 * @dev Stores attester public keys and verifies attestations with 5-of-9 threshold
 */
contract AttesterRegistry is Ownable, ReentrancyGuard {
    using ECDSA for bytes32;

    // === Constants ===
    uint8 public constant MAX_ATTESTERS = 9;
    uint8 public constant DEFAULT_THRESHOLD = 5;

    // === State Variables ===

    /// @notice Attester public keys (ECDSA compressed, 33 bytes)
    bytes[MAX_ATTESTERS] public attesters;

    /// @notice Derived Ethereum addresses for each attester
    address[MAX_ATTESTERS] public attesterAddresses;

    /// @notice Number of active attesters
    uint8 public activeAttesterCount;

    /// @notice Required threshold for valid attestation
    uint8 public threshold;

    /// @notice Nonce for replay protection
    uint256 public nonce;

    /// @notice Mapping of attester address to index (1-indexed, 0 = not attester)
    mapping(address => uint8) public attesterIndex;

    /// @notice Processed attestation hashes (prevent replay)
    mapping(bytes32 => bool) public processedAttestations;

    // === Events ===

    event AttesterRegistered(uint8 indexed index, address indexed attesterAddress, bytes pubkey);
    event AttesterUpdated(uint8 indexed index, address indexed oldAddress, address indexed newAddress);
    event AttesterRemoved(uint8 indexed index, address indexed attesterAddress);
    event ThresholdUpdated(uint8 oldThreshold, uint8 newThreshold);
    event AttestationVerified(bytes32 indexed messageHash, uint8 validSignatures, bool passed);
    event BridgeAttestationProcessed(
        bytes32 indexed requestId,
        uint32 sourceChain,
        uint32 destChain,
        address recipient,
        uint256 amount
    );

    // === Errors ===

    error InvalidThreshold();
    error InvalidAttesterIndex();
    error AttesterAlreadyExists();
    error AttesterNotFound();
    error InsufficientSignatures();
    error InvalidSignature();
    error AttestationAlreadyProcessed();
    error InvalidPubkeyLength();
    error ZeroAddress();

    // === Constructor ===

    constructor(address initialOwner) Ownable(initialOwner) {
        threshold = DEFAULT_THRESHOLD;
        activeAttesterCount = 0;
    }

    // === Admin Functions ===

    /**
     * @notice Register a new attester
     * @param index Attester index (0-8)
     * @param pubkey ECDSA compressed public key (33 bytes)
     */
    function registerAttester(uint8 index, bytes calldata pubkey) external onlyOwner {
        if (index >= MAX_ATTESTERS) revert InvalidAttesterIndex();
        if (pubkey.length != 33) revert InvalidPubkeyLength();

        // Derive Ethereum address from compressed public key
        address attesterAddr = _pubkeyToAddress(pubkey);
        if (attesterAddr == address(0)) revert ZeroAddress();

        // Check if already registered elsewhere
        if (attesterIndex[attesterAddr] != 0) revert AttesterAlreadyExists();

        // Remove old attester if exists
        address oldAddr = attesterAddresses[index];
        if (oldAddr != address(0)) {
            attesterIndex[oldAddr] = 0;
        } else {
            activeAttesterCount++;
        }

        // Register new attester
        attesters[index] = pubkey;
        attesterAddresses[index] = attesterAddr;
        attesterIndex[attesterAddr] = index + 1; // 1-indexed

        emit AttesterRegistered(index, attesterAddr, pubkey);
    }

    /**
     * @notice Batch register all 9 attesters
     * @param pubkeys Array of 9 ECDSA compressed public keys
     */
    function registerAllAttesters(bytes[9] calldata pubkeys) external onlyOwner {
        for (uint8 i = 0; i < MAX_ATTESTERS; i++) {
            if (pubkeys[i].length != 33) revert InvalidPubkeyLength();

            address attesterAddr = _pubkeyToAddress(pubkeys[i]);
            if (attesterAddr == address(0)) revert ZeroAddress();

            // Remove old mapping if exists
            address oldAddr = attesterAddresses[i];
            if (oldAddr != address(0)) {
                attesterIndex[oldAddr] = 0;
            }

            attesters[i] = pubkeys[i];
            attesterAddresses[i] = attesterAddr;
            attesterIndex[attesterAddr] = i + 1;

            emit AttesterRegistered(i, attesterAddr, pubkeys[i]);
        }

        activeAttesterCount = MAX_ATTESTERS;
    }

    /**
     * @notice Update attestation threshold
     * @param newThreshold New required signatures (1-9)
     */
    function updateThreshold(uint8 newThreshold) external onlyOwner {
        if (newThreshold == 0 || newThreshold > MAX_ATTESTERS) revert InvalidThreshold();

        uint8 oldThreshold = threshold;
        threshold = newThreshold;

        emit ThresholdUpdated(oldThreshold, newThreshold);
    }

    /**
     * @notice Remove an attester
     * @param index Attester index to remove
     */
    function removeAttester(uint8 index) external onlyOwner {
        if (index >= MAX_ATTESTERS) revert InvalidAttesterIndex();

        address attesterAddr = attesterAddresses[index];
        if (attesterAddr == address(0)) revert AttesterNotFound();

        attesterIndex[attesterAddr] = 0;
        attesterAddresses[index] = address(0);
        delete attesters[index];
        activeAttesterCount--;

        emit AttesterRemoved(index, attesterAddr);
    }

    // === Attestation Verification ===

    /**
     * @notice Verify attestation signatures meet threshold
     * @param messageHash Hash of the message to verify
     * @param signatures Array of signatures from attesters
     * @return valid True if enough valid signatures provided
     */
    function verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) external view returns (bool valid) {
        return _verifyAttestation(messageHash, signatures);
    }

    /**
     * @notice Process a bridge attestation
     * @param requestId Unique bridge request ID
     * @param sourceChain Source chain ID
     * @param destChain Destination chain ID
     * @param recipient Recipient address
     * @param amount Amount to bridge
     * @param tokenAddress Token address (0x0 for native)
     * @param attesterNonce Attestation nonce
     * @param signatures Array of attester signatures
     */
    function processBridgeAttestation(
        bytes32 requestId,
        uint32 sourceChain,
        uint32 destChain,
        address recipient,
        uint256 amount,
        address tokenAddress,
        uint256 attesterNonce,
        bytes[] calldata signatures
    ) external nonReentrant {
        // Build message hash
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceChain,
            destChain,
            recipient,
            amount,
            tokenAddress,
            attesterNonce,
            block.chainid
        ));

        // Check not already processed
        if (processedAttestations[messageHash]) revert AttestationAlreadyProcessed();

        // Verify signatures
        if (!_verifyAttestation(messageHash, signatures)) revert InsufficientSignatures();

        // Mark as processed
        processedAttestations[messageHash] = true;
        nonce++;

        emit BridgeAttestationProcessed(requestId, sourceChain, destChain, recipient, amount);
    }

    // === View Functions ===

    /**
     * @notice Get all attester addresses
     * @return Array of attester Ethereum addresses
     */
    function getAllAttesterAddresses() external view returns (address[9] memory) {
        return attesterAddresses;
    }

    /**
     * @notice Get attester public key by index
     * @param index Attester index
     * @return ECDSA compressed public key
     */
    function getAttesterPubkey(uint8 index) external view returns (bytes memory) {
        if (index >= MAX_ATTESTERS) revert InvalidAttesterIndex();
        return attesters[index];
    }

    /**
     * @notice Check if address is an attester
     * @param addr Address to check
     * @return True if address is a registered attester
     */
    function isAttester(address addr) external view returns (bool) {
        return attesterIndex[addr] != 0;
    }

    /**
     * @notice Get registry info
     */
    function getRegistryInfo() external view returns (
        uint8 _activeCount,
        uint8 _threshold,
        uint256 _nonce
    ) {
        return (activeAttesterCount, threshold, nonce);
    }

    // === Internal Functions ===

    /**
     * @dev Internal attestation verification
     */
    function _verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) internal view returns (bool) {
        uint8 validCount = 0;
        uint256 usedMask = 0; // Bitmask of used attesters

        bytes32 ethSignedHash = messageHash.toEthSignedMessageHash();

        for (uint256 i = 0; i < signatures.length && validCount < threshold; i++) {
            address recovered = ethSignedHash.recover(signatures[i]);
            uint8 idx = attesterIndex[recovered];

            // Check valid attester and not already used
            if (idx != 0) {
                uint256 mask = 1 << (idx - 1);
                if ((usedMask & mask) == 0) {
                    usedMask |= mask;
                    validCount++;
                }
            }
        }

        emit AttestationVerified(messageHash, validCount, validCount >= threshold);

        return validCount >= threshold;
    }

    /**
     * @dev Convert compressed ECDSA public key to Ethereum address
     * @param pubkey 33-byte compressed public key
     * @return Ethereum address
     */
    function _pubkeyToAddress(bytes calldata pubkey) internal pure returns (address) {
        require(pubkey.length == 33, "Invalid pubkey length");

        // Get prefix and x-coordinate
        uint8 prefix = uint8(pubkey[0]);
        require(prefix == 0x02 || prefix == 0x03, "Invalid pubkey prefix");

        // For now, we'll use a simplified approach:
        // Hash the compressed pubkey and take last 20 bytes
        // In production, you'd decompress and hash the uncompressed form
        bytes32 hash = keccak256(pubkey);
        return address(uint160(uint256(hash)));
    }
}
