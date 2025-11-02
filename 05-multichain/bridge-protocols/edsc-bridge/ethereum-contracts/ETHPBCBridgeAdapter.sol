// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ETHPBCBridgeAdapter
 * @notice Bridge adapter for ETH PBC to transfer MasterChef rewards cross-chain via EDSC
 * @dev Integrates with EDSC TokenMessenger for cross-chain token transfers
 *
 * Architecture:
 * - ETH PBC (EVM chain) <-> EDSC Bridge <-> FlareChain (Substrate chain)
 * - Users stake LP tokens on ETH PBC via MasterChef
 * - Rewards (ETR tokens) can be claimed on ETH PBC or bridged to FlareChain
 * - Uses EDSC burn-and-mint mechanism with M-of-N attestation (3-of-5)
 */
contract ETHPBCBridgeAdapter is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    // ============ State Variables ============

    /// @notice EDSC token address (stablecoin on ETH PBC)
    address public immutable edscToken;

    /// @notice ETR reward token address on ETH PBC
    address public immutable etrToken;

    /// @notice EDSC TokenMessenger contract for cross-chain transfers
    address public tokenMessenger;

    /// @notice MasterChef contract address
    address public masterChef;

    /// @notice FlareChain domain identifier for EDSC
    uint32 public constant FLARECHAIN_DOMAIN = 1;

    /// @notice Mapping of user => nonce for bridge operations
    mapping(address => uint256) public userNonce;

    /// @notice Mapping of transfer hash => completion status
    mapping(bytes32 => bool) public completedTransfers;

    // ============ Events ============

    event RewardBridged(
        address indexed user,
        uint256 amount,
        bytes32 indexed transferId,
        uint256 nonce
    );

    event RewardReceived(
        address indexed user,
        uint256 amount,
        bytes32 indexed transferId
    );

    event TokenMessengerUpdated(address indexed newMessenger);
    event MasterChefUpdated(address indexed newMasterChef);

    // ============ Errors ============

    error ZeroAddress();
    error ZeroAmount();
    error InsufficientBalance();
    error TransferFailed();
    error InvalidMessenger();
    error AlreadyCompleted();
    error Unauthorized();

    // ============ Constructor ============

    constructor(
        address _edscToken,
        address _etrToken,
        address _tokenMessenger,
        address _masterChef
    ) {
        if (_edscToken == address(0) || _etrToken == address(0)) revert ZeroAddress();
        if (_tokenMessenger == address(0) || _masterChef == address(0)) revert ZeroAddress();

        edscToken = _edscToken;
        etrToken = _etrToken;
        tokenMessenger = _tokenMessenger;
        masterChef = _masterChef;
    }

    // ============ Public Functions ============

    /**
     * @notice Bridge MasterChef rewards to FlareChain
     * @param amount Amount of ETR tokens to bridge
     * @param destinationAddress Recipient address on FlareChain (32 bytes)
     * @return transferId Unique identifier for the cross-chain transfer
     */
    function bridgeRewards(
        uint256 amount,
        bytes32 destinationAddress
    ) external nonReentrant returns (bytes32 transferId) {
        if (amount == 0) revert ZeroAmount();
        if (destinationAddress == bytes32(0)) revert ZeroAddress();

        // Get current nonce and increment
        uint256 nonce = userNonce[msg.sender]++;

        // Generate transfer ID
        transferId = keccak256(
            abi.encodePacked(
                msg.sender,
                amount,
                destinationAddress,
                nonce,
                block.timestamp
            )
        );

        // Transfer ETR tokens from user to this contract
        IERC20(etrToken).safeTransferFrom(msg.sender, address(this), amount);

        // Approve TokenMessenger to burn EDSC equivalent
        // Note: In production, this would involve an ETR->EDSC swap via DEX
        // For now, assuming 1:1 ratio for simplicity
        IERC20(edscToken).safeApprove(tokenMessenger, amount);

        // Initiate cross-chain transfer via EDSC TokenMessenger
        // TokenMessenger will burn EDSC on ETH PBC and mint on FlareChain
        (bool success, ) = tokenMessenger.call(
            abi.encodeWithSignature(
                "depositForBurn(uint256,uint32,bytes32,address)",
                amount,
                FLARECHAIN_DOMAIN,
                destinationAddress,
                edscToken
            )
        );

        if (!success) revert TransferFailed();

        emit RewardBridged(msg.sender, amount, transferId, nonce);

        return transferId;
    }

    /**
     * @notice Receive bridged rewards from FlareChain
     * @param recipient Address to receive tokens on ETH PBC
     * @param amount Amount of tokens to receive
     * @param transferId Unique identifier from source chain
     * @param attestation Multi-sig attestation from EDSC oracle network
     */
    function receiveRewards(
        address recipient,
        uint256 amount,
        bytes32 transferId,
        bytes calldata attestation
    ) external nonReentrant {
        if (recipient == address(0)) revert ZeroAddress();
        if (amount == 0) revert ZeroAmount();
        if (completedTransfers[transferId]) revert AlreadyCompleted();

        // Verify attestation signature (3-of-5 multisig)
        if (!_verifyAttestation(transferId, amount, recipient, attestation)) {
            revert Unauthorized();
        }

        // Mark transfer as completed
        completedTransfers[transferId] = true;

        // Mint EDSC tokens to recipient
        // Note: In production, TokenMessenger handles minting
        (bool success, ) = tokenMessenger.call(
            abi.encodeWithSignature(
                "mintTokens(address,uint256)",
                recipient,
                amount
            )
        );

        if (!success) revert TransferFailed();

        emit RewardReceived(recipient, amount, transferId);
    }

    /**
     * @notice Harvest MasterChef rewards and optionally bridge them
     * @param poolId Pool ID in MasterChef
     * @param bridgeToFlareChain Whether to bridge rewards to FlareChain
     * @param destinationAddress Recipient on FlareChain (if bridging)
     */
    function harvestAndBridge(
        uint256 poolId,
        bool bridgeToFlareChain,
        bytes32 destinationAddress
    ) external nonReentrant returns (bytes32 transferId) {
        // Call MasterChef.harvest on behalf of user
        (bool success, bytes memory data) = masterChef.call(
            abi.encodeWithSignature(
                "harvest(uint256,address)",
                poolId,
                msg.sender
            )
        );

        if (!success) revert TransferFailed();

        // If bridging, initiate cross-chain transfer
        if (bridgeToFlareChain) {
            uint256 balance = IERC20(etrToken).balanceOf(msg.sender);
            if (balance == 0) revert InsufficientBalance();

            // User must approve this contract first
            transferId = bridgeRewards(balance, destinationAddress);
        }

        return transferId;
    }

    // ============ Admin Functions ============

    /**
     * @notice Update TokenMessenger address
     * @param newMessenger New TokenMessenger contract address
     */
    function setTokenMessenger(address newMessenger) external onlyOwner {
        if (newMessenger == address(0)) revert ZeroAddress();
        tokenMessenger = newMessenger;
        emit TokenMessengerUpdated(newMessenger);
    }

    /**
     * @notice Update MasterChef address
     * @param newMasterChef New MasterChef contract address
     */
    function setMasterChef(address newMasterChef) external onlyOwner {
        if (newMasterChef == address(0)) revert ZeroAddress();
        masterChef = newMasterChef;
        emit MasterChefUpdated(newMasterChef);
    }

    // ============ Internal Functions ============

    /**
     * @notice Verify EDSC oracle network attestation (3-of-5 multisig)
     * @param transferId Transfer identifier
     * @param amount Amount being transferred
     * @param recipient Recipient address
     * @param attestation Packed signature data
     * @return valid Whether attestation is valid
     */
    function _verifyAttestation(
        bytes32 transferId,
        uint256 amount,
        address recipient,
        bytes calldata attestation
    ) internal view returns (bool valid) {
        // Message hash for signature verification
        bytes32 messageHash = keccak256(
            abi.encodePacked(
                transferId,
                amount,
                recipient,
                FLARECHAIN_DOMAIN,
                block.chainid
            )
        );

        bytes32 ethSignedMessageHash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)
        );

        // Extract signatures (3 required from 5 possible oracles)
        // Attestation format: [r1][s1][v1][r2][s2][v2][r3][s3][v3]
        if (attestation.length != 195) return false; // 65 bytes * 3 signatures

        uint8 validSignatures = 0;
        address[] memory signers = new address[](3);

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

            // Check if signer is a registered oracle
            if (_isRegisteredOracle(signer)) {
                // Check for duplicate signers
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

        // Require 3 valid signatures
        return validSignatures >= 3;
    }

    /**
     * @notice Check if address is a registered EDSC oracle
     * @param oracle Address to check
     * @return Whether address is registered oracle
     */
    function _isRegisteredOracle(address oracle) internal view returns (bool) {
        // Query TokenMessenger for oracle registry
        (bool success, bytes memory data) = tokenMessenger.staticcall(
            abi.encodeWithSignature("isOracle(address)", oracle)
        );

        if (!success || data.length == 0) return false;
        return abi.decode(data, (bool));
    }

    // ============ View Functions ============

    /**
     * @notice Get user's current nonce
     * @param user User address
     * @return Current nonce value
     */
    function getNonce(address user) external view returns (uint256) {
        return userNonce[user];
    }

    /**
     * @notice Check if transfer has been completed
     * @param transferId Transfer identifier
     * @return Whether transfer is completed
     */
    function isTransferCompleted(bytes32 transferId) external view returns (bool) {
        return completedTransfers[transferId];
    }
}
