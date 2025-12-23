// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

/**
 * @title TronAttesterRegistry
 * @notice ETRID 9-Director Attestation Registry for TRON Network
 * @dev Adapted for TVM (TRON Virtual Machine) - uses TRC20 instead of ERC20
 *
 * TRON specifics:
 * - Uses TRX as native currency
 * - Addresses start with T (base58)
 * - Energy/Bandwidth instead of gas
 * - Compatible with Solidity ^0.8.6
 */
contract TronAttesterRegistry {
    // === Constants ===
    uint8 public constant MAX_ATTESTERS = 9;
    uint8 public constant DEFAULT_THRESHOLD = 5;

    // === State Variables ===
    address public owner;
    bytes[MAX_ATTESTERS] public attesters;
    address[MAX_ATTESTERS] public attesterAddresses;
    uint8 public activeAttesterCount;
    uint8 public threshold;
    uint256 public nonce;

    mapping(address => uint8) public attesterIndex;
    mapping(bytes32 => bool) public processedAttestations;

    // === Events ===
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
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

    // === Modifiers ===
    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    // === Constructor ===
    constructor() {
        owner = msg.sender;
        threshold = DEFAULT_THRESHOLD;
        activeAttesterCount = 0;
    }

    // === Owner Functions ===

    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }

    /**
     * @notice Register a new attester
     * @param index Attester index (0-8)
     * @param pubkey ECDSA compressed public key (33 bytes)
     */
    function registerAttester(uint8 index, bytes calldata pubkey) external onlyOwner {
        require(index < MAX_ATTESTERS, "Invalid index");
        require(pubkey.length == 33, "Invalid pubkey length");

        // Derive address from compressed public key
        address attesterAddr = _pubkeyToAddress(pubkey);
        require(attesterAddr != address(0), "Zero address");
        require(attesterIndex[attesterAddr] == 0, "Already registered");

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
        attesterIndex[attesterAddr] = index + 1;

        emit AttesterRegistered(index, attesterAddr, pubkey);
    }

    /**
     * @notice Batch register all 9 attesters
     */
    function registerAllAttesters(bytes[9] calldata pubkeys) external onlyOwner {
        for (uint8 i = 0; i < MAX_ATTESTERS; i++) {
            require(pubkeys[i].length == 33, "Invalid pubkey length");

            address attesterAddr = _pubkeyToAddress(pubkeys[i]);
            require(attesterAddr != address(0), "Zero address");

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
     */
    function updateThreshold(uint8 newThreshold) external onlyOwner {
        require(newThreshold > 0 && newThreshold <= MAX_ATTESTERS, "Invalid threshold");

        uint8 oldThreshold = threshold;
        threshold = newThreshold;

        emit ThresholdUpdated(oldThreshold, newThreshold);
    }

    /**
     * @notice Remove an attester
     */
    function removeAttester(uint8 index) external onlyOwner {
        require(index < MAX_ATTESTERS, "Invalid index");

        address attesterAddr = attesterAddresses[index];
        require(attesterAddr != address(0), "Not found");

        attesterIndex[attesterAddr] = 0;
        attesterAddresses[index] = address(0);
        delete attesters[index];
        activeAttesterCount--;

        emit AttesterRemoved(index, attesterAddr);
    }

    // === Attestation Verification ===

    /**
     * @notice Verify attestation signatures meet threshold
     */
    function verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) external returns (bool) {
        return _verifyAttestation(messageHash, signatures);
    }

    /**
     * @notice Process a bridge attestation
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
    ) external {
        // Build message hash (TRON chain ID: 728126428)
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceChain,
            destChain,
            recipient,
            amount,
            tokenAddress,
            attesterNonce,
            uint256(728126428) // TRON mainnet chain ID
        ));

        require(!processedAttestations[messageHash], "Already processed");
        require(_verifyAttestation(messageHash, signatures), "Insufficient signatures");

        processedAttestations[messageHash] = true;
        nonce++;

        emit BridgeAttestationProcessed(requestId, sourceChain, destChain, recipient, amount);
    }

    // === View Functions ===

    function getAllAttesterAddresses() external view returns (address[9] memory) {
        return attesterAddresses;
    }

    function getAttesterPubkey(uint8 index) external view returns (bytes memory) {
        require(index < MAX_ATTESTERS, "Invalid index");
        return attesters[index];
    }

    function isAttester(address addr) external view returns (bool) {
        return attesterIndex[addr] != 0;
    }

    function getRegistryInfo() external view returns (
        uint8 _activeCount,
        uint8 _threshold,
        uint256 _nonce
    ) {
        return (activeAttesterCount, threshold, nonce);
    }

    // === Internal Functions ===

    function _verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) internal returns (bool) {
        uint8 validCount = 0;
        uint256 usedMask = 0;

        bytes32 ethSignedHash = _toEthSignedMessageHash(messageHash);

        for (uint256 i = 0; i < signatures.length && validCount < threshold; i++) {
            address recovered = _recover(ethSignedHash, signatures[i]);
            uint8 idx = attesterIndex[recovered];

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

    function _toEthSignedMessageHash(bytes32 hash) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", hash));
    }

    function _recover(bytes32 hash, bytes memory signature) internal pure returns (address) {
        require(signature.length == 65, "Invalid signature length");

        bytes32 r;
        bytes32 s;
        uint8 v;

        assembly {
            r := mload(add(signature, 32))
            s := mload(add(signature, 64))
            v := byte(0, mload(add(signature, 96)))
        }

        if (v < 27) {
            v += 27;
        }

        require(v == 27 || v == 28, "Invalid signature v");

        return ecrecover(hash, v, r, s);
    }

    function _pubkeyToAddress(bytes calldata pubkey) internal pure returns (address) {
        require(pubkey.length == 33, "Invalid pubkey length");

        uint8 prefix = uint8(pubkey[0]);
        require(prefix == 0x02 || prefix == 0x03, "Invalid pubkey prefix");

        // Simplified: hash and take last 20 bytes
        bytes32 hash = keccak256(pubkey);
        return address(uint160(uint256(hash)));
    }
}
