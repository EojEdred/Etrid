// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable2Step.sol";

/**
 * @title EDSC - Ëtrid Dollar Stablecoin
 * @notice ERC-20 token representing EDSC on Ethereum
 * @dev Mintable/burnable only by authorized MessageTransmitter contract
 *
 * Key Properties:
 * - Name: Ëtrid Dollar Stablecoin
 * - Symbol: EDSC
 * - Decimals: 18 (matches Substrate implementation)
 * - Total Supply: Initially 0, minted on demand from Ëtrid chain
 * - Backing: 110-130% reserve ratio on Ëtrid FlareChain
 *
 * Security Model:
 * - Only MessageTransmitter can mint (when receiving from Ëtrid)
 * - Only MessageTransmitter can burn (when sending to Ëtrid)
 * - Owner can update MessageTransmitter address (2-step transfer)
 * - Pausable for emergency situations
 */
contract EDSC is ERC20, Ownable2Step {
    /// @notice Address of the authorized MessageTransmitter contract
    address public messageTransmitter;

    /// @notice Pause state for emergency situations
    bool public paused;

    /// @notice Emitted when the MessageTransmitter address is updated
    event MessageTransmitterUpdated(address indexed oldTransmitter, address indexed newTransmitter);

    /// @notice Emitted when the contract is paused or unpaused
    event PauseStateChanged(bool paused);

    /// @notice Emitted when tokens are minted from cross-chain transfer
    event CrossChainMint(address indexed recipient, uint256 amount, uint64 nonce);

    /// @notice Emitted when tokens are burned for cross-chain transfer
    event CrossChainBurn(address indexed sender, uint256 amount, uint64 nonce);

    error Paused();
    error NotMessageTransmitter();
    error ZeroAddress();
    error SameAddress();

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    modifier onlyMessageTransmitter() {
        if (msg.sender != messageTransmitter) revert NotMessageTransmitter();
        _;
    }

    /**
     * @notice Constructor
     * @param _owner Initial owner address (governance multisig)
     */
    constructor(address _owner) ERC20("Etrid Dollar Stablecoin", "EDSC") Ownable(_owner) {
        if (_owner == address(0)) revert ZeroAddress();
        // MessageTransmitter will be set after deployment
    }

    /**
     * @notice Set the MessageTransmitter contract address
     * @dev Only owner can call. MessageTransmitter must be deployed first.
     * @param _messageTransmitter Address of the MessageTransmitter contract
     */
    function setMessageTransmitter(address _messageTransmitter) external onlyOwner {
        if (_messageTransmitter == address(0)) revert ZeroAddress();
        if (_messageTransmitter == messageTransmitter) revert SameAddress();

        address oldTransmitter = messageTransmitter;
        messageTransmitter = _messageTransmitter;

        emit MessageTransmitterUpdated(oldTransmitter, _messageTransmitter);
    }

    /**
     * @notice Pause the contract
     * @dev Only owner can pause. Prevents all transfers, mints, and burns.
     */
    function pause() external onlyOwner {
        paused = true;
        emit PauseStateChanged(true);
    }

    /**
     * @notice Unpause the contract
     * @dev Only owner can unpause.
     */
    function unpause() external onlyOwner {
        paused = false;
        emit PauseStateChanged(false);
    }

    /**
     * @notice Mint EDSC tokens (cross-chain transfer from Ëtrid)
     * @dev Only MessageTransmitter can call
     * @param recipient Address to receive the minted tokens
     * @param amount Amount of tokens to mint (18 decimals)
     * @param nonce Unique nonce from the cross-chain message
     */
    function mint(address recipient, uint256 amount, uint64 nonce)
        external
        onlyMessageTransmitter
        whenNotPaused
    {
        if (recipient == address(0)) revert ZeroAddress();

        _mint(recipient, amount);

        emit CrossChainMint(recipient, amount, nonce);
    }

    /**
     * @notice Burn EDSC tokens (cross-chain transfer to Ëtrid)
     * @dev Only MessageTransmitter can call
     * @param sender Address whose tokens are being burned
     * @param amount Amount of tokens to burn (18 decimals)
     * @param nonce Unique nonce for the cross-chain message
     */
    function burn(address sender, uint256 amount, uint64 nonce)
        external
        onlyMessageTransmitter
        whenNotPaused
    {
        _burn(sender, amount);

        emit CrossChainBurn(sender, amount, nonce);
    }

    /**
     * @notice Override transfer to check pause state
     */
    function transfer(address to, uint256 amount)
        public
        override
        whenNotPaused
        returns (bool)
    {
        return super.transfer(to, amount);
    }

    /**
     * @notice Override transferFrom to check pause state
     */
    function transferFrom(address from, address to, uint256 amount)
        public
        override
        whenNotPaused
        returns (bool)
    {
        return super.transferFrom(from, to, amount);
    }
}
