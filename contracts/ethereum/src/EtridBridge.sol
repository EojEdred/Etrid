// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

interface IEtridToken {
    function bridgeMint(address to, uint256 amount, bytes32 txHash) external;
    function totalSupply() external view returns (uint256);
}

/**
 * @title EtridBridge
 * @dev Bridge contract for Ëtrid ↔ Ethereum token transfers
 *
 * Architecture:
 * - Lock/Mint: User locks ÉTR/EDSC on Ëtrid → Bridge mints ÉTR.e/EDSC.e on Ethereum
 * - Burn/Release: User burns ÉTR.e/EDSC.e on Ethereum → Bridge releases on Ëtrid
 *
 * Security:
 * - Watchtower multisig (3-of-5) for attestation
 * - Relayer role for submitting proofs
 * - Rate limits per transaction and per day
 * - Emergency pause mechanism
 * - Replay protection via nonces
 *
 * Watchtower Flow:
 * 1. User locks tokens on Ëtrid and receives LockEvent
 * 2. Watchtowers observe LockEvent and sign attestation
 * 3. Relayer submits 3+ signatures to bridge
 * 4. Bridge verifies signatures and mints tokens on Ethereum
 *
 * Burn Flow:
 * 1. User calls token.bridgeBurn(amount, etridAddress)
 * 2. Bridge emits BurnEvent
 * 3. Watchtowers observe and release on Ëtrid
 */
contract EtridBridge is AccessControl, Pausable, ReentrancyGuard {
    using ECDSA for bytes32;

    /// @dev Role for relayers (submit proofs)
    bytes32 public constant RELAYER_ROLE = keccak256("RELAYER_ROLE");

    /// @dev Role for watchtowers (sign attestations)
    bytes32 public constant WATCHTOWER_ROLE = keccak256("WATCHTOWER_ROLE");

    /// @dev Role for pausing (emergency multisig)
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @dev ÉTR token contract
    IEtridToken public immutable etrToken;

    /// @dev EDSC token contract
    IEtridToken public immutable edscToken;

    /// @dev Minimum watchtower signatures required (3-of-5)
    uint256 public constant MIN_SIGNATURES = 3;

    /// @dev Maximum watchtower signatures (5 total)
    uint256 public constant MAX_WATCHTOWERS = 5;

    /// @dev Per-transaction bridge limit (100k ÉTR or EDSC)
    uint256 public constant MAX_BRIDGE_PER_TX = 100_000 * 10**18;

    /// @dev Daily bridge limit (1M ÉTR or EDSC)
    uint256 public constant MAX_BRIDGE_PER_DAY = 1_000_000 * 10**18;

    /// @dev Attestation validity window (15 minutes)
    uint256 public constant ATTESTATION_TIMEOUT = 15 minutes;

    /// @dev Tracking for daily bridge limit
    uint256 public bridgedToday;
    uint256 public lastBridgeDay;

    /// @dev Nonce for preventing replay attacks
    mapping(bytes32 => bool) public processedMints;

    /// @dev Watchtower registry (max 5)
    address[] public watchtowers;
    mapping(address => bool) public isWatchtower;

    /// @notice Emitted when tokens are minted on Ethereum (from Ëtrid lock)
    event BridgeMinted(
        address indexed token,
        address indexed to,
        uint256 amount,
        bytes32 etridTxHash,
        bytes32 indexed mintId
    );

    /// @notice Emitted when tokens are burned on Ethereum (for Ëtrid release)
    /// Watchtowers listen to this and release on Ëtrid
    event BridgeBurned(
        address indexed token,
        address indexed from,
        uint256 amount,
        string etridAddress,
        bytes32 indexed burnId
    );

    /// @notice Emitted when watchtower is added
    event WatchtowerAdded(address indexed watchtower);

    /// @notice Emitted when watchtower is removed
    event WatchtowerRemoved(address indexed watchtower);

    /**
     * @dev Struct for mint attestation (signed by watchtowers)
     */
    struct MintAttestation {
        address token;           // ÉTR or EDSC token address
        address to;              // Recipient on Ethereum
        uint256 amount;          // Amount to mint
        bytes32 etridTxHash;     // Transaction hash from Ëtrid
        uint256 timestamp;       // When attestation was signed
        bytes32 mintId;          // Unique mint ID (prevents replay)
    }

    /**
     * @dev Constructor
     * @param admin Address that will have DEFAULT_ADMIN_ROLE
     * @param _etrToken ÉTR token contract address
     * @param _edscToken EDSC token contract address
     * @param initialWatchtowers Initial watchtower addresses (3-5 addresses)
     */
    constructor(
        address admin,
        address _etrToken,
        address _edscToken,
        address[] memory initialWatchtowers
    ) {
        require(admin != address(0), "Bridge: zero admin address");
        require(_etrToken != address(0), "Bridge: zero ETR address");
        require(_edscToken != address(0), "Bridge: zero EDSC address");
        require(
            initialWatchtowers.length >= MIN_SIGNATURES &&
            initialWatchtowers.length <= MAX_WATCHTOWERS,
            "Bridge: invalid watchtower count"
        );

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);
        _grantRole(RELAYER_ROLE, admin); // Admin can relay initially

        etrToken = IEtridToken(_etrToken);
        edscToken = IEtridToken(_edscToken);

        // Register initial watchtowers
        for (uint256 i = 0; i < initialWatchtowers.length; i++) {
            address wt = initialWatchtowers[i];
            require(wt != address(0), "Bridge: zero watchtower address");
            require(!isWatchtower[wt], "Bridge: duplicate watchtower");

            watchtowers.push(wt);
            isWatchtower[wt] = true;
            _grantRole(WATCHTOWER_ROLE, wt);

            emit WatchtowerAdded(wt);
        }
    }

    /**
     * @notice Mint tokens on Ethereum (from Ëtrid lock)
     * @dev Called by relayer with watchtower signatures
     * @param attestation Mint attestation struct
     * @param signatures Array of watchtower signatures (min 3)
     */
    function mintFromEtrid(
        MintAttestation calldata attestation,
        bytes[] calldata signatures
    ) external onlyRole(RELAYER_ROLE) whenNotPaused nonReentrant {
        require(signatures.length >= MIN_SIGNATURES, "Bridge: insufficient signatures");
        require(attestation.to != address(0), "Bridge: mint to zero address");
        require(attestation.amount > 0, "Bridge: mint zero amount");
        require(attestation.amount <= MAX_BRIDGE_PER_TX, "Bridge: exceeds per-tx limit");
        require(
            attestation.token == address(etrToken) ||
            attestation.token == address(edscToken),
            "Bridge: invalid token"
        );

        // Check attestation freshness
        require(
            block.timestamp <= attestation.timestamp + ATTESTATION_TIMEOUT,
            "Bridge: attestation expired"
        );

        // Check replay protection
        require(!processedMints[attestation.mintId], "Bridge: mint already processed");

        // Check daily limit
        uint256 currentDay = block.timestamp / 1 days;
        if (currentDay > lastBridgeDay) {
            bridgedToday = 0;
            lastBridgeDay = currentDay;
        }

        require(bridgedToday + attestation.amount <= MAX_BRIDGE_PER_DAY, "Bridge: exceeds daily limit");
        bridgedToday += attestation.amount;

        // Verify watchtower signatures
        bytes32 messageHash = _getAttestationHash(attestation);
        _verifySignatures(messageHash, signatures);

        // Mark as processed
        processedMints[attestation.mintId] = true;

        // Mint tokens
        IEtridToken(attestation.token).bridgeMint(
            attestation.to,
            attestation.amount,
            attestation.etridTxHash
        );

        emit BridgeMinted(
            attestation.token,
            attestation.to,
            attestation.amount,
            attestation.etridTxHash,
            attestation.mintId
        );
    }

    /**
     * @notice Verify watchtower signatures
     * @dev Internal function to verify 3-of-5 multisig
     * @param messageHash Hash of the attestation message
     * @param signatures Array of signatures
     */
    function _verifySignatures(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) internal view {
        bytes32 ethSignedMessageHash = messageHash.toEthSignedMessageHash();
        address[] memory signers = new address[](signatures.length);

        for (uint256 i = 0; i < signatures.length; i++) {
            address signer = ethSignedMessageHash.recover(signatures[i]);

            // Check signer is a watchtower
            require(isWatchtower[signer], "Bridge: invalid watchtower signature");

            // Check no duplicate signers
            for (uint256 j = 0; j < i; j++) {
                require(signers[j] != signer, "Bridge: duplicate signature");
            }

            signers[i] = signer;
        }
    }

    /**
     * @notice Get hash of attestation for signing
     * @dev Used by watchtowers to sign mint attestations
     * @param attestation Mint attestation struct
     * @return Hash of the attestation
     */
    function _getAttestationHash(
        MintAttestation calldata attestation
    ) internal pure returns (bytes32) {
        return keccak256(abi.encodePacked(
            attestation.token,
            attestation.to,
            attestation.amount,
            attestation.etridTxHash,
            attestation.timestamp,
            attestation.mintId
        ));
    }

    /**
     * @notice Get attestation hash (public for watchtower clients)
     * @param attestation Mint attestation struct
     * @return Hash that watchtowers should sign
     */
    function getAttestationHash(
        MintAttestation calldata attestation
    ) external pure returns (bytes32) {
        return _getAttestationHash(attestation);
    }

    /**
     * @notice Add watchtower (admin only)
     * @dev Maximum 5 watchtowers allowed
     * @param watchtower Address to add
     */
    function addWatchtower(address watchtower) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(watchtower != address(0), "Bridge: zero watchtower address");
        require(!isWatchtower[watchtower], "Bridge: already watchtower");
        require(watchtowers.length < MAX_WATCHTOWERS, "Bridge: max watchtowers reached");

        watchtowers.push(watchtower);
        isWatchtower[watchtower] = true;
        _grantRole(WATCHTOWER_ROLE, watchtower);

        emit WatchtowerAdded(watchtower);
    }

    /**
     * @notice Remove watchtower (admin only)
     * @dev Must maintain at least MIN_SIGNATURES watchtowers
     * @param watchtower Address to remove
     */
    function removeWatchtower(address watchtower) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(isWatchtower[watchtower], "Bridge: not a watchtower");
        require(watchtowers.length > MIN_SIGNATURES, "Bridge: cannot remove below minimum");

        isWatchtower[watchtower] = false;
        _revokeRole(WATCHTOWER_ROLE, watchtower);

        // Remove from array (swap and pop)
        for (uint256 i = 0; i < watchtowers.length; i++) {
            if (watchtowers[i] == watchtower) {
                watchtowers[i] = watchtowers[watchtowers.length - 1];
                watchtowers.pop();
                break;
            }
        }

        emit WatchtowerRemoved(watchtower);
    }

    /**
     * @notice Get all watchtowers
     * @return Array of watchtower addresses
     */
    function getWatchtowers() external view returns (address[] memory) {
        return watchtowers;
    }

    /**
     * @notice Get watchtower count
     * @return Number of active watchtowers
     */
    function watchtowerCount() external view returns (uint256) {
        return watchtowers.length;
    }

    /**
     * @notice Check if mint has been processed
     * @param mintId Unique mint ID
     * @return True if already processed
     */
    function isMintProcessed(bytes32 mintId) external view returns (bool) {
        return processedMints[mintId];
    }

    /**
     * @notice Pause bridge operations (emergency only)
     * @dev Only PAUSER_ROLE can call
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause bridge operations
     * @dev Only PAUSER_ROLE can call
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }
}
