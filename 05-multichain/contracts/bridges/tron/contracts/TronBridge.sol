// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

/**
 * @title TronBridge
 * @notice ETRID Cross-Chain Bridge for TRON Network with 5-of-9 Director Attestation
 * @dev Adapted for TVM (TRON Virtual Machine)
 *
 * TRON Specifics:
 * - Uses TRX as native currency
 * - Addresses are base58 encoded (T...)
 * - Uses Energy/Bandwidth instead of gas
 * - TRC20 tokens instead of ERC20
 *
 * Bridge Flow:
 * 1. User locks TRX/TRC20 tokens
 * 2. Directors observe and sign
 * 3. 5-of-9 attestation collected
 * 4. Tokens minted/unlocked on destination
 */

interface ITRC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
    function approve(address spender, uint256 amount) external returns (bool);
}

interface ITronAttesterRegistry {
    function verifyAttestation(bytes32 messageHash, bytes[] calldata signatures) external returns (bool);
    function threshold() external view returns (uint8);
}

interface IMintableBurnable {
    function mint(address to, uint256 amount) external;
    function burnFrom(address from, uint256 amount) external;
}

contract TronBridge {
    // ============ Constants ============

    uint32 public constant TRON_DOMAIN_ID = 195; // TRON mainnet domain
    uint256 public constant MIN_LOCK_AMOUNT = 1_000_000; // 1 TRX (6 decimals)

    // ============ State Variables ============

    address public owner;
    address public attesterRegistry;
    uint16 public feeBps; // Fee in basis points (100 = 1%)
    address public feeRecipient;
    bool public paused;

    mapping(address => bool) public supportedTokens;
    mapping(address => uint256) public userNonces;
    mapping(bytes32 => bool) public processedRequests;
    mapping(address => uint256) public dailyLimits;
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

    event TrxLocked(
        bytes32 indexed requestId,
        address indexed sender,
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

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event AttesterRegistryUpdated(address indexed newRegistry);
    event TokenSupported(address indexed token);
    event FeesUpdated(uint16 newFeeBps, address feeRecipient);
    event Paused();
    event Unpaused();

    // ============ Modifiers ============

    modifier onlyOwner() {
        require(msg.sender == owner, "Not owner");
        _;
    }

    modifier whenNotPaused() {
        require(!paused, "Bridge paused");
        _;
    }

    // ============ Constructor ============

    constructor(address _attesterRegistry, address _feeRecipient) {
        require(_attesterRegistry != address(0), "Zero registry");
        require(_feeRecipient != address(0), "Zero fee recipient");

        owner = msg.sender;
        attesterRegistry = _attesterRegistry;
        feeRecipient = _feeRecipient;
        feeBps = 10; // 0.1% default fee
    }

    // ============ Lock Functions ============

    /**
     * @notice Lock TRC20 tokens to bridge to ETRID PBC
     */
    function lockTokens(
        address token,
        uint256 amount,
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external whenNotPaused returns (bytes32 requestId) {
        require(token != address(0), "Zero token");
        require(amount >= MIN_LOCK_AMOUNT, "Amount too small");
        require(destinationAddress != bytes32(0), "Zero destination");
        require(supportedTokens[token], "Token not supported");
        require(destinationDomain != TRON_DOMAIN_ID, "Same domain");

        // Check daily limit
        _checkDailyLimit(token, amount);

        // Calculate fee
        uint256 fee = (amount * feeBps) / 10000;
        uint256 netAmount = amount - fee;

        // Get nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            token,
            amount,
            destinationAddress,
            destinationDomain,
            TRON_DOMAIN_ID,
            nonce,
            block.timestamp
        ));

        // Transfer tokens
        require(ITRC20(token).transferFrom(msg.sender, address(this), amount), "Transfer failed");

        // Transfer fee
        if (fee > 0) {
            require(ITRC20(token).transfer(feeRecipient, fee), "Fee transfer failed");
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
     * @notice Lock native TRX to bridge
     */
    function lockTrx(
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external payable whenNotPaused returns (bytes32 requestId) {
        require(msg.value >= MIN_LOCK_AMOUNT, "Amount too small");
        require(destinationAddress != bytes32(0), "Zero destination");
        require(destinationDomain != TRON_DOMAIN_ID, "Same domain");

        // Calculate fee
        uint256 fee = (msg.value * feeBps) / 10000;
        uint256 netAmount = msg.value - fee;

        // Get nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            address(0), // Native TRX
            msg.value,
            destinationAddress,
            destinationDomain,
            TRON_DOMAIN_ID,
            nonce,
            block.timestamp
        ));

        // Transfer fee
        if (fee > 0) {
            payable(feeRecipient).transfer(fee);
        }

        emit TrxLocked(
            requestId,
            msg.sender,
            netAmount,
            fee,
            destinationAddress,
            destinationDomain,
            nonce
        );

        return requestId;
    }

    // ============ Unlock Functions ============

    /**
     * @notice Unlock TRC20 tokens after attestation
     */
    function unlockTokens(
        bytes32 requestId,
        uint32 sourceDomain,
        address recipient,
        address token,
        uint256 amount,
        bytes[] calldata signatures
    ) external whenNotPaused {
        require(recipient != address(0), "Zero recipient");
        require(amount > 0, "Zero amount");
        require(!processedRequests[requestId], "Already processed");

        // Build message hash
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceDomain,
            TRON_DOMAIN_ID,
            recipient,
            token,
            amount
        ));

        // Verify attestation
        require(
            ITronAttesterRegistry(attesterRegistry).verifyAttestation(messageHash, signatures),
            "Attestation failed"
        );

        // Mark as processed
        processedRequests[requestId] = true;

        // Transfer tokens
        if (token == address(0)) {
            // Native TRX
            payable(recipient).transfer(amount);
        } else {
            require(ITRC20(token).transfer(recipient, amount), "Transfer failed");
        }

        emit TokensUnlocked(requestId, recipient, token, amount, sourceDomain);
    }

    /**
     * @notice Mint wrapped tokens
     */
    function mintTokens(
        bytes32 requestId,
        uint32 sourceDomain,
        address recipient,
        address wrappedToken,
        uint256 amount,
        bytes[] calldata signatures
    ) external whenNotPaused {
        require(recipient != address(0), "Zero recipient");
        require(amount > 0, "Zero amount");
        require(!processedRequests[requestId], "Already processed");

        // Build message hash
        bytes32 messageHash = keccak256(abi.encodePacked(
            requestId,
            sourceDomain,
            TRON_DOMAIN_ID,
            recipient,
            wrappedToken,
            amount,
            "MINT"
        ));

        // Verify attestation
        require(
            ITronAttesterRegistry(attesterRegistry).verifyAttestation(messageHash, signatures),
            "Attestation failed"
        );

        // Mark as processed
        processedRequests[requestId] = true;

        // Mint tokens
        IMintableBurnable(wrappedToken).mint(recipient, amount);

        emit TokensMinted(requestId, recipient, wrappedToken, amount);
    }

    /**
     * @notice Burn wrapped tokens to bridge back
     */
    function burnTokens(
        address wrappedToken,
        uint256 amount,
        bytes32 destinationAddress,
        uint32 destinationDomain
    ) external whenNotPaused returns (bytes32 requestId) {
        require(amount > 0, "Zero amount");
        require(destinationAddress != bytes32(0), "Zero destination");
        require(destinationDomain != TRON_DOMAIN_ID, "Same domain");

        // Get nonce
        uint256 nonce = userNonces[msg.sender]++;

        // Generate request ID
        requestId = keccak256(abi.encodePacked(
            msg.sender,
            wrappedToken,
            amount,
            destinationAddress,
            destinationDomain,
            TRON_DOMAIN_ID,
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

    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Zero address");
        address oldOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(oldOwner, newOwner);
    }

    function setAttesterRegistry(address newRegistry) external onlyOwner {
        require(newRegistry != address(0), "Zero address");
        attesterRegistry = newRegistry;
        emit AttesterRegistryUpdated(newRegistry);
    }

    function addSupportedToken(address token) external onlyOwner {
        require(token != address(0), "Zero address");
        supportedTokens[token] = true;
        emit TokenSupported(token);
    }

    function setFees(uint16 newFeeBps, address newFeeRecipient) external onlyOwner {
        require(newFeeBps <= 500, "Fee too high"); // Max 5%
        require(newFeeRecipient != address(0), "Zero address");
        feeBps = newFeeBps;
        feeRecipient = newFeeRecipient;
        emit FeesUpdated(newFeeBps, newFeeRecipient);
    }

    function setDailyLimit(address token, uint256 limit) external onlyOwner {
        dailyLimits[token] = limit;
    }

    function pause() external onlyOwner {
        paused = true;
        emit Paused();
    }

    function unpause() external onlyOwner {
        paused = false;
        emit Unpaused();
    }

    function emergencyWithdraw(address token, uint256 amount, address recipient) external onlyOwner {
        if (token == address(0)) {
            payable(recipient).transfer(amount);
        } else {
            ITRC20(token).transfer(recipient, amount);
        }
    }

    // ============ View Functions ============

    function getNonce(address user) external view returns (uint256) {
        return userNonces[user];
    }

    function isProcessed(bytes32 requestId) external view returns (bool) {
        return processedRequests[requestId];
    }

    function getRemainingDailyQuota(address token) external view returns (uint256) {
        if (dailyLimits[token] == 0) return type(uint256).max;
        uint256 today = block.timestamp / 1 days;
        return dailyLimits[token] - dailyUsage[token][today];
    }

    // ============ Internal Functions ============

    function _checkDailyLimit(address token, uint256 amount) internal {
        if (dailyLimits[token] > 0) {
            uint256 today = block.timestamp / 1 days;
            require(
                dailyUsage[token][today] + amount <= dailyLimits[token],
                "Daily limit exceeded"
            );
            dailyUsage[token][today] += amount;
        }
    }

    // ============ Receive ============

    receive() external payable {}
}
