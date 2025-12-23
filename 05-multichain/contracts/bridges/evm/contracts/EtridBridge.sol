// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";

/**
 * @title EtridBridge
 * @notice Cross-chain bridge with 5-of-9 Director attestation
 * @dev Integrates with AttesterRegistry for signature verification
 *
 * Bridge Flow:
 * 1. User locks tokens on source chain (this contract)
 * 2. Directors observe lock event and sign attestation
 * 3. 5-of-9 signatures collected by relayer
 * 4. User/relayer claims on destination chain with attestation
 * 5. Wrapped tokens minted on destination
 *
 * Supported Directions:
 * - EVM -> ETRID PBC: Lock native tokens, mint wrapped on PBC
 * - ETRID PBC -> EVM: Burn wrapped on PBC, unlock native on EVM
 */
interface IAttesterRegistry {
    function verifyAttestation(
        bytes32 messageHash,
        bytes[] calldata signatures
    ) external returns (bool);

    function threshold() external view returns (uint8);
    function activeAttesterCount() external view returns (uint8);
}

interface IMintableBurnable {
    function mint(address to, uint256 amount) external;
    function burnFrom(address from, uint256 amount) external;
}

contract EtridBridge is ReentrancyGuard, Ownable, Pausable {
    using SafeERC20 for IERC20;

    // ============ Constants ============

    /// @notice Primearc Core Chain domain ID
    uint32 public constant PRIMEARC_CORE_DOMAIN = 1;

    /// @notice Minimum lock amount (prevent dust)
    uint256 public constant MIN_LOCK_AMOUNT = 1000; // 0.001 with 6 decimals

    // ============ State Variables ============

    /// @notice AttesterRegistry contract for 5-of-9 verification
    IAttesterRegistry public attesterRegistry;

    /// @notice Domain ID for this chain
    uint32 public immutable localDomain;

    /// @notice Mapping: token address => is supported
    mapping(address => bool) public supportedTokens;

    /// @notice Mapping: token address => wrapped token on PBC
    mapping(address => bytes32) public wrappedTokens;

    /// @notice Mapping: source token => destination token (for claiming)
    mapping(address => address) public tokenMappings;

    /// @notice User nonces for replay protection
    mapping(address => uint256) public userNonces;

    /// @notice Processed bridge requests (prevent double-claim)
    mapping(bytes32 => bool) public processedRequests;

    /// @notice Bridge fee in basis points (100 = 1%)
    uint16 public bridgeFeesBps;

    /// @notice Fee recipient address
    address public feeRecipient;

    /// @notice Daily limit per token
    mapping(address => uint256) public dailyLimits;

    /// @notice Daily usage tracking
    mapping(address => mapping(uint256 => uint256)) public dailyUsage;

    // ============ Events ============

    event TokensLocked(
        bytes32 indexed requestId,
        address indexed sender,
        address token,
        uint256 amount,
        uint256 fee,
        bytes32 destinationAddress,
        uint32 destinationDomain,
        uint256 nonce
    );

    event TokensUnlocked(
        bytes32 indexed requestId,
        address indexed recipient,
        address token,
        uint256 amount,
        uint32 sourceDomain
    );

    event TokensMinted(
        bytes32 indexed requestId,
        address indexed recipient,
        address token,
        uint256 amount
    );

    event TokensBurned(
        bytes32 indexed requestId,
        address indexed sender,
        address token,
        uint256 amount,
        bytes32 destinationAddress
    );

    event AttesterRegistryUpdated(address indexed newRegistry);
    event TokenSupported(address indexed token, bytes32 wrappedToken);
    event FeeUpdated(uint16 newFeeBps, address feeRecipient);
    event DailyLimitUpdated(address indexed token, uint256 limit);

    // ============ Errors ============

    error ZeroAddress();
    error ZeroAmount();
    error AmountTooSmall();
    error UnsupportedToken();
    error InvalidDomain();
    error RequestAlreadyProcessed();
    error AttestationFailed();
    error DailyLimitExceeded();
    error InvalidFee();
    error TransferFailed();

    // ============ Constructor ============

    constructor(
        uint32 _localDomain,
        address _attesterRegistry,
        address _feeRecipient
    ) Ownable(msg.sender) {
        if (_localDomain == 0) revert InvalidDomain();
        if (_attesterRegistry == address(0)) revert ZeroAddress();
        if (_feeRecipient == address(0)) revert ZeroAddress();

        localDomain = _localDomain;
        attesterRegistry = IAttesterRegistry(_attesterRegistry);
        feeRecipient = _feeRecipient;
        bridgeFeesBps = 10; // 0.1% default fee
    }

    // ============ Lock & Bridge (EVM -> PBC) ============

    /**
     * @notice Lock tokens to bridge to ETRID PBC
     * @param token Token address to lock
     * @param amount Amount to lock
     * @param destinationAddress Recipient on PBC (32 bytes SS58)
     * @param destinationDomain Target PBC domain ID
     * @return requestId Unique bridge request ID
     */
    function lockTokens(
        address token,
        uint256 amount,
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external nonReentrant whenNotPaused returns (bytes32 requestId) {
        if (token == address(0)) revert ZeroAddress();
        if (amount < MIN_LOCK_AMOUNT) revert AmountTooSmall();
        if (destinationAddress == bytes32(0)) revert ZeroAddress();
        if (!supportedTokens[token]) revert UnsupportedToken();
        if (destinationDomain == localDomain) revert InvalidDomain();

        // Check daily limit
        uint256 today = block.timestamp / 1 days;
        if (dailyLimits[token] > 0) {
            if (dailyUsage[token][today] + amount > dailyLimits[token]) {
                revert DailyLimitExceeded();
            }
            dailyUsage[token][today] += amount;
        }

        // Calculate fee
        uint256 fee = (amount * bridgeFeesBps) / 10000;
        uint256 netAmount = amount - fee;

        // Get user nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            token,
            amount,
            destinationAddress,
            destinationDomain,
            localDomain,
            nonce,
            block.timestamp
        ));

        // Transfer tokens to this contract
        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);

        // Transfer fee to recipient
        if (fee > 0) {
            IERC20(token).safeTransfer(feeRecipient, fee);
        }

        emit TokensLocked(
            requestId,
            msg.sender,
            token,
            netAmount,
            fee,
            destinationAddress,
            destinationDomain,
            nonce
        );

        return requestId;
    }

    /**
     * @notice Lock native ETH/BNB/MATIC to bridge
     * @param destinationAddress Recipient on PBC (32 bytes SS58)
     * @param destinationDomain Target PBC domain ID
     * @return requestId Unique bridge request ID
     */
    function lockNative(
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external payable nonReentrant whenNotPaused returns (bytes32 requestId) {
        if (msg.value < MIN_LOCK_AMOUNT) revert AmountTooSmall();
        if (destinationAddress == bytes32(0)) revert ZeroAddress();
        if (destinationDomain == localDomain) revert InvalidDomain();

        // Calculate fee
        uint256 fee = (msg.value * bridgeFeesBps) / 10000;
        uint256 netAmount = msg.value - fee;

        // Get user nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            address(0), // Native token
            msg.value,
            destinationAddress,
            destinationDomain,
            localDomain,
            nonce,
            block.timestamp
        ));

        // Transfer fee
        if (fee > 0) {
            (bool success, ) = feeRecipient.call{value: fee}("");
            if (!success) revert TransferFailed();
        }

        emit TokensLocked(
            requestId,
            msg.sender,
            address(0),
            netAmount,
            fee,
            destinationAddress,
            destinationDomain,
            nonce
        );

        return requestId;
    }

    // ============ Claim & Unlock (PBC -> EVM) ============

    /**
     * @notice Unlock tokens after receiving attestation from Directors
     * @param requestId Original bridge request ID
     * @param sourceDomain Source chain domain ID
     * @param recipient Recipient address on this chain
     * @param token Token to unlock
     * @param amount Amount to unlock
     * @param signatures Array of Director signatures (5+ of 9)
     */
    function unlockTokens(
        bytes32 requestId,
        uint32 sourceDomain,
        address recipient,
        address token,
        uint256 amount,
        bytes[] calldata signatures
    ) external nonReentrant whenNotPaused {
        if (recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (processedRequests[requestId]) revert RequestAlreadyProcessed();

        // Build message hash for attestation verification
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceDomain,
            localDomain,
            recipient,
            token,
            amount
        ));

        // Verify 5-of-9 Director attestation
        if (!attesterRegistry.verifyAttestation(messageHash, signatures)) {
            revert AttestationFailed();
        }

        // Mark as processed
        processedRequests[requestId] = true;

        // Transfer tokens to recipient
        if (token == address(0)) {
            // Native token
            (bool success, ) = recipient.call{value: amount}("");
            if (!success) revert TransferFailed();
        } else {
            IERC20(token).safeTransfer(recipient, amount);
        }

        emit TokensUnlocked(requestId, recipient, token, amount, sourceDomain);
    }

    /**
     * @notice Mint wrapped tokens (for tokens not locked on this chain)
     * @param requestId Original bridge request ID
     * @param sourceDomain Source chain domain ID
     * @param recipient Recipient address
     * @param wrappedToken Wrapped token to mint
     * @param amount Amount to mint
     * @param signatures Director signatures
     */
    function mintTokens(
        bytes32 requestId,
        uint32 sourceDomain,
        address recipient,
        address wrappedToken,
        uint256 amount,
        bytes[] calldata signatures
    ) external nonReentrant whenNotPaused {
        if (recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (processedRequests[requestId]) revert RequestAlreadyProcessed();

        // Build message hash
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceDomain,
            localDomain,
            recipient,
            wrappedToken,
            amount,
            "MINT"
        ));

        // Verify attestation
        if (!attesterRegistry.verifyAttestation(messageHash, signatures)) {
            revert AttestationFailed();
        }

        // Mark as processed
        processedRequests[requestId] = true;

        // Mint wrapped tokens
        IMintableBurnable(wrappedToken).mint(recipient, amount);

        emit TokensMinted(requestId, recipient, wrappedToken, amount);
    }

    /**
     * @notice Burn wrapped tokens to bridge back to original chain
     * @param wrappedToken Wrapped token to burn
     * @param amount Amount to burn
     * @param destinationAddress Recipient on destination chain
     * @param destinationDomain Target chain domain ID
     * @return requestId Bridge request ID
     */
    function burnTokens(
        address wrappedToken,
        uint256 amount,
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external nonReentrant whenNotPaused returns (bytes32 requestId) {
        if (amount == 0) revert ZeroAmount();
        if (destinationAddress == bytes32(0)) revert ZeroAddress();
        if (destinationDomain == localDomain) revert InvalidDomain();

        // Get user nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            wrappedToken,
            amount,
            destinationAddress,
            destinationDomain,
            localDomain,
            nonce,
            block.timestamp
        ));

        // Burn tokens
        IMintableBurnable(wrappedToken).burnFrom(msg.sender, amount);

        emit TokensBurned(
            requestId,
            msg.sender,
            wrappedToken,
            amount,
            destinationAddress
        );

        return requestId;
    }

    // ============ Admin Functions ============

    /**
     * @notice Update AttesterRegistry address
     */
    function setAttesterRegistry(address newRegistry) external onlyOwner {
        if (newRegistry == address(0)) revert ZeroAddress();
        attesterRegistry = IAttesterRegistry(newRegistry);
        emit AttesterRegistryUpdated(newRegistry);
    }

    /**
     * @notice Add supported token
     */
    function addSupportedToken(
        address token,
        bytes32 wrappedToken
    ) external onlyOwner {
        if (token == address(0)) revert ZeroAddress();
        supportedTokens[token] = true;
        wrappedTokens[token] = wrappedToken;
        emit TokenSupported(token, wrappedToken);
    }

    /**
     * @notice Set token mapping (source -> destination)
     */
    function setTokenMapping(
        address sourceToken,
        address destToken
    ) external onlyOwner {
        tokenMappings[sourceToken] = destToken;
    }

    /**
     * @notice Update bridge fees
     */
    function setFees(
        uint16 newFeeBps,
        address newFeeRecipient
    ) external onlyOwner {
        if (newFeeBps > 500) revert InvalidFee(); // Max 5%
        if (newFeeRecipient == address(0)) revert ZeroAddress();
        bridgeFeesBps = newFeeBps;
        feeRecipient = newFeeRecipient;
        emit FeeUpdated(newFeeBps, newFeeRecipient);
    }

    /**
     * @notice Set daily limit for token
     */
    function setDailyLimit(address token, uint256 limit) external onlyOwner {
        dailyLimits[token] = limit;
        emit DailyLimitUpdated(token, limit);
    }

    /**
     * @notice Pause bridge operations
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @notice Unpause bridge operations
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    /**
     * @notice Emergency withdraw (admin only)
     */
    function emergencyWithdraw(
        address token,
        uint256 amount,
        address recipient
    ) external onlyOwner {
        if (token == address(0)) {
            (bool success, ) = recipient.call{value: amount}("");
            if (!success) revert TransferFailed();
        } else {
            IERC20(token).safeTransfer(recipient, amount);
        }
    }

    // ============ View Functions ============

    /**
     * @notice Get user nonce
     */
    function getNonce(address user) external view returns (uint256) {
        return userNonces[user];
    }

    /**
     * @notice Check if request is processed
     */
    function isProcessed(bytes32 requestId) external view returns (bool) {
        return processedRequests[requestId];
    }

    /**
     * @notice Get remaining daily quota
     */
    function getRemainingDailyQuota(address token) external view returns (uint256) {
        if (dailyLimits[token] == 0) return type(uint256).max;
        uint256 today = block.timestamp / 1 days;
        return dailyLimits[token] - dailyUsage[token][today];
    }

    /**
     * @notice Receive native tokens
     */
    receive() external payable {}
}
