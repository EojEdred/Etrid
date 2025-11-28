// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable2Step.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./EDSC.sol";

/**
 * @title EDSCTokenMessenger
 * @notice Burns EDSC on Ethereum to send to Ëtrid
 * @dev Users burn EDSC here, attesters sign, relayers deliver to Ëtrid
 *
 * Message Flow (Ethereum → Ëtrid):
 * 1. User approves EDSC tokens to this contract
 * 2. User calls burnAndSend() with destination details
 * 3. Contract burns EDSC and emits MessageSent event
 * 4. Off-chain attesters monitor and sign the message
 * 5. Relayer delivers signed message to Ëtrid MessageTransmitter
 * 6. Ëtrid MessageTransmitter verifies and mints EDSC
 *
 * Security Model:
 * - Rate limiting (per-tx and daily limits)
 * - Nonce-based message ordering
 * - Domain validation
 * - Pausable for emergencies
 * - Owner can update limits via governance
 */
contract EDSCTokenMessenger is Ownable2Step {
    /// @notice EDSC token contract
    EDSC public immutable edscToken;

    /// @notice This chain's domain ID
    uint32 public constant LOCAL_DOMAIN = 0; // Ethereum

    /// @notice Ëtrid domain ID
    uint32 public constant ETRID_DOMAIN = 2;

    /// @notice Maximum burn amount per transaction (1M EDSC)
    uint256 public maxBurnAmount = 1_000_000 ether;

    /// @notice Maximum daily burn volume (10M EDSC)
    uint256 public dailyBurnLimit = 10_000_000 ether;

    /// @notice Blocks per day (assuming 12 second blocks)
    uint256 public constant BLOCKS_PER_DAY = 7200; // ~24 hours

    /// @notice Current nonce for message ordering
    uint64 public nonce;

    /// @notice Daily burn tracking
    uint256 public dailyBurnVolume;
    uint256 public dailyBurnResetBlock;

    /// @notice Pause state
    bool public paused;

    /// @notice Statistics
    uint256 public totalMessagesSent;
    uint256 public totalEDSCBurned;

    /// @notice Outbound message structure
    struct OutboundMessage {
        uint32 destinationDomain;
        address sender;
        bytes recipient;        // Ëtrid account (32 bytes)
        uint256 amount;
        uint64 nonce;
        uint256 timestamp;
    }

    /// @notice Mapping of nonce to outbound message
    mapping(uint64 => OutboundMessage) public outboundMessages;

    // Events
    event MessageSent(
        uint32 indexed destinationDomain,
        uint64 indexed nonce,
        address indexed sender,
        bytes recipient,
        uint256 amount
    );

    event BurnLimitUpdated(uint256 maxBurnAmount, uint256 dailyBurnLimit);
    event PauseStateChanged(bool paused);

    // Errors
    error Paused();
    error InvalidDomain();
    error InvalidRecipient();
    error InvalidAmount();
    error AmountExceedsMaxBurn();
    error DailyLimitExceeded();
    error InsufficientAllowance();
    error BurnFailed();

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /**
     * @notice Constructor
     * @param _owner Initial owner (governance multisig)
     * @param _edscToken EDSC token contract address
     */
    constructor(address _owner, address _edscToken) Ownable(_owner) {
        if (_owner == address(0)) revert InvalidRecipient();
        if (_edscToken == address(0)) revert InvalidRecipient();

        edscToken = EDSC(_edscToken);
        dailyBurnResetBlock = block.number;
    }

    /**
     * @notice Burn EDSC and send to Ëtrid
     * @param _recipient Ëtrid account (32 bytes SS58 address)
     * @param _amount Amount of EDSC to burn (18 decimals)
     */
    function burnAndSend(bytes calldata _recipient, uint256 _amount)
        external
        whenNotPaused
    {
        return _burnAndSendTo(ETRID_DOMAIN, _recipient, _amount);
    }

    /**
     * @notice Burn EDSC and send to specific domain
     * @param _destinationDomain Destination domain ID
     * @param _recipient Recipient address on destination chain
     * @param _amount Amount of EDSC to burn
     */
    function burnAndSendTo(
        uint32 _destinationDomain,
        bytes calldata _recipient,
        uint256 _amount
    ) external whenNotPaused {
        return _burnAndSendTo(_destinationDomain, _recipient, _amount);
    }

    /**
     * @notice Internal function to burn and send
     */
    function _burnAndSendTo(
        uint32 _destinationDomain,
        bytes calldata _recipient,
        uint256 _amount
    ) internal {
        // Validate inputs
        if (_destinationDomain != ETRID_DOMAIN) revert InvalidDomain();
        if (_recipient.length != 32) revert InvalidRecipient();
        if (_amount == 0) revert InvalidAmount();
        if (_amount > maxBurnAmount) revert AmountExceedsMaxBurn();

        // Check and update daily limit
        _checkAndUpdateDailyLimit(_amount);

        // Get next nonce
        uint64 currentNonce = nonce;
        nonce++;

        // Burn EDSC from sender
        edscToken.burn(msg.sender, _amount, currentNonce);

        // Store outbound message
        outboundMessages[currentNonce] = OutboundMessage({
            destinationDomain: _destinationDomain,
            sender: msg.sender,
            recipient: _recipient,
            amount: _amount,
            nonce: currentNonce,
            timestamp: block.timestamp
        });

        // Update statistics
        totalMessagesSent++;
        totalEDSCBurned += _amount;

        emit MessageSent(
            _destinationDomain,
            currentNonce,
            msg.sender,
            _recipient,
            _amount
        );
    }

    /**
     * @notice Check and update daily burn limit
     * @param _amount Amount being burned
     */
    function _checkAndUpdateDailyLimit(uint256 _amount) internal {
        // Reset daily volume if day has passed
        if (block.number >= dailyBurnResetBlock + BLOCKS_PER_DAY) {
            dailyBurnVolume = 0;
            dailyBurnResetBlock = block.number;
        }

        // Check if adding amount exceeds daily limit
        uint256 newVolume = dailyBurnVolume + _amount;
        if (newVolume > dailyBurnLimit) revert DailyLimitExceeded();

        // Update daily volume
        dailyBurnVolume = newVolume;
    }

    /**
     * @notice Update burn limits (governance only)
     * @param _maxBurnAmount New max burn per transaction
     * @param _dailyBurnLimit New daily burn limit
     */
    function updateBurnLimits(uint256 _maxBurnAmount, uint256 _dailyBurnLimit)
        external
        onlyOwner
    {
        if (_maxBurnAmount == 0) revert InvalidAmount();
        if (_dailyBurnLimit < _maxBurnAmount) revert InvalidAmount();

        maxBurnAmount = _maxBurnAmount;
        dailyBurnLimit = _dailyBurnLimit;

        emit BurnLimitUpdated(_maxBurnAmount, _dailyBurnLimit);
    }

    /**
     * @notice Pause the messenger
     */
    function pause() external onlyOwner {
        paused = true;
        emit PauseStateChanged(true);
    }

    /**
     * @notice Unpause the messenger
     */
    function unpause() external onlyOwner {
        paused = false;
        emit PauseStateChanged(false);
    }

    /**
     * @notice Get daily burn status
     * @return volume Current daily burn volume
     * @return limit Daily burn limit
     * @return resetBlock Block when daily volume resets
     * @return blocksUntilReset Blocks until reset
     */
    function getDailyBurnStatus()
        external
        view
        returns (
            uint256 volume,
            uint256 limit,
            uint256 resetBlock,
            uint256 blocksUntilReset
        )
    {
        volume = dailyBurnVolume;
        limit = dailyBurnLimit;
        resetBlock = dailyBurnResetBlock + BLOCKS_PER_DAY;

        if (block.number >= resetBlock) {
            blocksUntilReset = 0;
        } else {
            blocksUntilReset = resetBlock - block.number;
        }
    }

    /**
     * @notice Get message statistics
     * @return sent Total messages sent
     * @return burned Total EDSC burned
     */
    function getStatistics() external view returns (uint256 sent, uint256 burned) {
        return (totalMessagesSent, totalEDSCBurned);
    }

    /**
     * @notice Get outbound message details
     * @param _nonce Message nonce
     * @return message OutboundMessage struct
     */
    function getMessage(uint64 _nonce) external view returns (OutboundMessage memory) {
        return outboundMessages[_nonce];
    }
}
