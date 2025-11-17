// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridStateProof
 * @dev Interface for State Proof Verification Precompile (0x806)
 *
 * Verifies Merkle proofs from Ethereum mainnet without external oracles.
 * Enables trustless cross-L1/L2 composability.
 *
 * Address: 0x0000000000000000000000000000000000000806
 *
 * @notice Novel Feature: Built-in Ethereum state verification
 *         Allows ETH PBC contracts to trustlessly verify Ethereum state
 */
interface IEtridStateProof {
    /**
     * @notice Latest Ethereum block information
     */
    struct EthBlockInfo {
        uint256 blockNumber;
        bytes32 blockHash;
        bytes32 stateRoot;
        uint256 timestamp;
    }

    /**
     * @notice Verify a state proof from Ethereum mainnet
     * @param stateRoot Ethereum state root to verify against
     * @param proof Merkle proof array
     * @param key Storage key to verify
     * @param value Expected storage value
     * @return valid true if proof is valid
     *
     * @dev Example: Verify USDC balance on Ethereum
     *   bytes32 stateRoot = stateProof.getLatestEthBlock().stateRoot;
     *   bytes32[] memory proof = ...; // Merkle proof from Ethereum
     *   bytes32 key = keccak256(abi.encode(userAddress, balanceSlot));
     *   bytes memory value = abi.encode(expectedBalance);
     *
     *   bool valid = stateProof.verifyStateProof(
     *       stateRoot,
     *       proof,
     *       key,
     *       value
     *   );
     *   require(valid, "Invalid state proof");
     *
     * @dev Use case: Trustless cross-chain liquidity verification
     */
    function verifyStateProof(
        bytes32 stateRoot,
        bytes32[] calldata proof,
        bytes32 key,
        bytes calldata value
    ) external view returns (bool valid);

    /**
     * @notice Get the latest verified Ethereum block
     * @return blockNumber Latest Ethereum block number
     * @return blockHash Block hash
     * @return stateRoot State root
     * @return timestamp Block timestamp
     *
     * @dev Example:
     *   (uint256 blockNum,
     *    bytes32 blockHash,
     *    bytes32 stateRoot,
     *    uint256 timestamp) = stateProof.getLatestEthBlock();
     *
     *   require(block.timestamp - timestamp < 1 hours, "Stale Ethereum data");
     *
     * @dev Block headers are relayed from Ethereum via:
     *   - Ethereum light client running on FlareChain
     *   - Validator consensus on block finality
     *   - XCM messages to ETH PBC
     */
    function getLatestEthBlock()
        external
        view
        returns (
            uint256 blockNumber,
            bytes32 blockHash,
            bytes32 stateRoot,
            uint256 timestamp
        );

    /**
     * @notice Verify an Ethereum transaction inclusion proof
     * @param txHash Transaction hash to verify
     * @param blockHash Block containing the transaction
     * @param rlpEncodedTx RLP-encoded transaction
     * @param proof Merkle proof of transaction inclusion
     * @return valid true if transaction is included in block
     *
     * @dev Example: Verify cross-chain deposit
     *   bytes32 txHash = 0x...; // Ethereum deposit tx
     *   bytes32 blockHash = 0x...; // Containing block
     *   bytes memory rlpTx = ...; // RLP-encoded transaction
     *   bytes32[] memory proof = ...; // Merkle proof
     *
     *   bool valid = stateProof.verifyTransaction(
     *       txHash,
     *       blockHash,
     *       rlpTx,
     *       proof
     *   );
     *   require(valid, "Invalid transaction proof");
     */
    function verifyTransaction(
        bytes32 txHash,
        bytes32 blockHash,
        bytes calldata rlpEncodedTx,
        bytes32[] calldata proof
    ) external view returns (bool valid);

    /**
     * @notice Emitted when a new Ethereum block is verified
     * @param blockNumber Ethereum block number
     * @param blockHash Block hash
     * @param stateRoot State root
     */
    event EthBlockVerified(
        uint256 indexed blockNumber,
        bytes32 indexed blockHash,
        bytes32 stateRoot
    );
}
