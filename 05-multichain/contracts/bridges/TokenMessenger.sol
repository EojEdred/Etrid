// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/**
 * @title TokenMessenger
 * @notice Cross-chain token messaging protocol with M-of-N attestation
 * @dev Burn-and-mint bridge mechanism with oracle attestation (3-of-5)
 *
 * Architecture:
 * 1. User burns tokens on source chain
 * 2. Oracle network (5 nodes) observes burn event
 * 3. 3-of-5 oracles sign attestation
 * 4. User submits attestation to destination chain
 * 5. Tokens minted on destination chain
 *
 * Security:
 * - 3-of-5 multisig required
 * - Nonce tracking prevents replay
 * - Rate limiting per address
 * - Emergency pause
 */
interface IMintableBurnable {
    function mint(address to, uint256 amount) external;
    function burnFrom(address from, uint256 amount) external;
}

contract TokenMessenger is AccessControl, ReentrancyGuard, Pausable {
    bytes32 public constant ORACLE_ROLE = keccak256("ORACLE_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    /// @notice Domain identifier for this chain
    uint32 public immutable localDomain;

    /// @notice Supported token addresses
    mapping(address => bool) public supportedTokens;

    /// @notice Oracle addresses (5 total)
    address[5] public oracles;

    /// @notice Number of active oracles
    uint8 public oracleCount;

    /// @notice Required signatures (3-of-5)
    uint8 public constant REQUIRED_SIGNATURES = 3;

    /// @notice Nonce for each user
    mapping(address => uint256) public nonces;

    /// @notice Completed message hashes (prevent replay)
    mapping(bytes32 => bool) public completedMessages;

    /// @notice User rate limiting (max per hour)
    mapping(address => mapping(uint256 => uint256)) public hourlyTransfers;
    uint256 public constant MAX_HOURLY_TRANSFER = 100_000 * 10**6; // 100k EDSC

    struct Message {
        uint32 sourceDomain;
        uint32 destinationDomain;
        uint64 nonce;
        address sender;
        address recipient;
        address token;
        uint256 amount;
        bytes32 messageHash;
    }

    event TokensBurned(
        address indexed sender,
        uint32 indexed destinationDomain,
        bytes32 indexed recipient,
        address token,
        uint256 amount,
        uint64 nonce
    );

    event TokensMinted(
        address indexed recipient,
        uint32 indexed sourceDomain,
        address token,
        uint256 amount,
        bytes32 messageHash
    );

    event OracleAdded(address indexed oracle);
    event OracleRemoved(address indexed oracle);
    event TokenSupported(address indexed token);

    error InvalidSignature();
    error InsufficientSignatures();
    error MessageAlreadyProcessed();
    error UnsupportedToken();
    error ZeroAddress();
    error ZeroAmount();
    error InvalidDomain();
    error RateLimitExceeded();
    error DuplicateSigner();

    constructor(uint32 _localDomain, address admin) {
        if (admin == address(0)) revert ZeroAddress();
        if (_localDomain == 0) revert InvalidDomain();

        localDomain = _localDomain;

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(PAUSER_ROLE, admin);
    }

    /**
     * @notice Burn tokens to send cross-chain
     * @param amount Amount to burn
     * @param destinationDomain Target chain domain ID
     * @param recipient Recipient address on destination chain (as bytes32)
     * @param token Token address to burn
     * @return nonce Message nonce
     */
    function depositForBurn(
        uint256 amount,
        uint32 destinationDomain,
        bytes32 recipient,
        address token
    ) external nonReentrant whenNotPaused returns (uint64) {
        if (amount == 0) revert ZeroAmount();
        if (recipient == bytes32(0)) revert ZeroAddress();
        if (!supportedTokens[token]) revert UnsupportedToken();
        if (destinationDomain == localDomain) revert InvalidDomain();

        // Rate limiting
        uint256 currentHour = block.timestamp / 1 hours;
        hourlyTransfers[msg.sender][currentHour] += amount;
        if (hourlyTransfers[msg.sender][currentHour] > MAX_HOURLY_TRANSFER) {
            revert RateLimitExceeded();
        }

        // Burn tokens
        IMintableBurnable(token).burnFrom(msg.sender, amount);

        // Increment nonce
        uint64 nonce = uint64(nonces[msg.sender]++);

        emit TokensBurned(
            msg.sender,
            destinationDomain,
            recipient,
            token,
            amount,
            nonce
        );

        return nonce;
    }

    /**
     * @notice Mint tokens after receiving attestation
     * @param message Cross-chain message struct
     * @param attestation Packed signature data from oracles
     */
    function receiveMessage(
        Message calldata message,
        bytes calldata attestation
    ) external nonReentrant whenNotPaused {
        if (message.recipient == address(0)) revert ZeroAddress();
        if (message.amount == 0) revert ZeroAmount();
        if (message.destinationDomain != localDomain) revert InvalidDomain();
        if (!supportedTokens[message.token]) revert UnsupportedToken();
        if (completedMessages[message.messageHash]) revert MessageAlreadyProcessed();

        // Verify attestation
        if (!_verifyAttestation(message, attestation)) {
            revert InsufficientSignatures();
        }

        // Mark as completed
        completedMessages[message.messageHash] = true;

        // Mint tokens
        IMintableBurnable(message.token).mint(message.recipient, message.amount);

        emit TokensMinted(
            message.recipient,
            message.sourceDomain,
            message.token,
            message.amount,
            message.messageHash
        );
    }

    /**
     * @notice Verify oracle attestation (3-of-5 multisig)
     * @param message Message to verify
     * @param attestation Signature data
     * @return valid Whether attestation is valid
     */
    function _verifyAttestation(
        Message calldata message,
        bytes calldata attestation
    ) internal view returns (bool) {
        // Reconstruct message hash
        bytes32 computedHash = keccak256(abi.encode(
            message.sourceDomain,
            message.destinationDomain,
            message.nonce,
            message.sender,
            message.recipient,
            message.token,
            message.amount
        ));

        if (computedHash != message.messageHash) return false;

        bytes32 ethSignedMessageHash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n32", computedHash)
        );

        // Require 195 bytes (3 signatures * 65 bytes)
        if (attestation.length != 195) return false;

        uint8 validSignatures = 0;
        address[3] memory signers;

        for (uint8 i = 0; i < 3; i++) {
            uint256 offset = i * 65;
            bytes32 r;
            bytes32 s;
            uint8 v;

            assembly {
                r := calldataload(add(attestation.offset, offset))
                s := calldataload(add(attestation.offset, add(offset, 32)))
                v := byte(0, calldataload(add(attestation.offset, add(offset, 64))))
            }

            address signer = ecrecover(ethSignedMessageHash, v, r, s);

            // Check if signer is an oracle
            if (_isOracle(signer)) {
                // Check for duplicates
                bool duplicate = false;
                for (uint8 j = 0; j < i; j++) {
                    if (signers[j] == signer) {
                        duplicate = true;
                        break;
                    }
                }

                if (!duplicate) {
                    signers[i] = signer;
                    validSignatures++;
                }
            }
        }

        return validSignatures >= REQUIRED_SIGNATURES;
    }

    /**
     * @notice Check if address is an oracle
     * @param addr Address to check
     * @return Whether address is oracle
     */
    function _isOracle(address addr) internal view returns (bool) {
        for (uint8 i = 0; i < oracleCount; i++) {
            if (oracles[i] == addr) return true;
        }
        return false;
    }

    /**
     * @notice Check if address is an oracle (external)
     * @param addr Address to check
     * @return Whether address is oracle
     */
    function isOracle(address addr) external view returns (bool) {
        return _isOracle(addr);
    }

    /**
     * @notice Add oracle address
     * @param oracle Oracle address to add
     */
    function addOracle(address oracle) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (oracle == address(0)) revert ZeroAddress();
        require(oracleCount < 5, "Max oracles reached");

        oracles[oracleCount] = oracle;
        oracleCount++;

        _grantRole(ORACLE_ROLE, oracle);
        emit OracleAdded(oracle);
    }

    /**
     * @notice Add supported token
     * @param token Token address
     */
    function addSupportedToken(address token) external onlyRole(DEFAULT_ADMIN_ROLE) {
        if (token == address(0)) revert ZeroAddress();
        supportedTokens[token] = true;
        emit TokenSupported(token);
    }

    /**
     * @notice Pause all operations
     */
    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    /**
     * @notice Unpause operations
     */
    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }
}
