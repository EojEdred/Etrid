// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEthereumStateProof
 * @notice Interface for verifying Ethereum mainnet state proofs on ETH-PBC
 * @dev Precompile address: 0x0000000000000000000000000000000000000804
 *
 * This precompile enables trustless verification of Ethereum mainnet state
 * without relying on external oracles. Use cases include:
 * - Cross-chain token bridges
 * - Cross-L1/L2 composability
 * - Trustless data migration
 * - Ethereum state queries from ETH-PBC
 *
 * Example Usage:
 * ```solidity
 * IEthereumStateProof proof = IEthereumStateProof(0x0000000000000000000000000000000000000804);
 *
 * // Verify mainnet state
 * bool valid = proof.verifyStateProof(stateRoot, merkleProof, key, value);
 *
 * // Get latest Ethereum block
 * (uint256 blockNum, bytes32 blockHash, bytes32 stateRoot, uint256 timestamp) = proof.getLatestEthBlock();
 * ```
 */
interface IEthereumStateProof {
    /**
     * @notice Emitted when a state proof is verified
     * @param stateRoot The Ethereum state root used for verification
     * @param key The storage key that was verified
     */
    event StateProofVerified(bytes32 indexed stateRoot, bytes32 indexed key);

    /**
     * @notice Emitted when a transaction is verified
     * @param txHash The transaction hash that was verified
     * @param blockHash The block containing the transaction
     */
    event TransactionVerified(bytes32 indexed txHash, bytes32 indexed blockHash);

    /**
     * @notice Verify a Merkle proof from Ethereum mainnet
     * @param stateRoot The Ethereum state root to verify against
     * @param proof Array of Merkle proof hashes
     * @param key The storage key to verify
     * @param value The storage value to verify
     * @return valid True if the proof is valid
     *
     * This function verifies that a specific key-value pair exists in the
     * Ethereum state tree at the given state root.
     *
     * Requirements:
     * - stateRoot must be from a verified Ethereum block
     * - proof must be a valid Merkle proof
     * - key and value must match the proof
     *
     * Use Cases:
     * - Verify mainnet contract state on ETH-PBC
     * - Trustless token bridge verification
     * - Cross-chain composability
     *
     * Example:
     * ```solidity
     * // Verify that account 0x123... has balance 100 ETH on mainnet
     * bytes32 stateRoot = 0xabc...; // From latest block
     * bytes32[] memory proof = getMainnetProof(); // Get via API
     * bytes32 key = keccak256(abi.encode(address(0x123...)));
     * bytes memory value = abi.encode(100 ether);
     *
     * bool valid = proof.verifyStateProof(stateRoot, proof, key, value);
     * require(valid, "Invalid proof");
     * ```
     */
    function verifyStateProof(
        bytes32 stateRoot,
        bytes32[] calldata proof,
        bytes32 key,
        bytes calldata value
    ) external view returns (bool valid);

    /**
     * @notice Get the latest verified Ethereum block header
     * @return blockNumber The block number
     * @return blockHash The block hash
     * @return stateRoot The state root
     * @return timestamp The block timestamp
     *
     * Returns information about the latest Ethereum mainnet block that
     * has been verified and stored on ETH-PBC. This is used as the
     * canonical reference for state proof verification.
     *
     * Update Frequency:
     * - Typically updated every ~12 seconds (Ethereum block time)
     * - May lag slightly due to finality requirements
     * - Guaranteed to be finalized (100+ confirmations)
     *
     * Example:
     * ```solidity
     * (uint256 blockNum, bytes32 blockHash, bytes32 stateRoot, uint256 timestamp)
     *     = proof.getLatestEthBlock();
     *
     * require(block.timestamp - timestamp < 1 hours, "Data too old");
     *
     * // Use stateRoot for verification
     * bool valid = proof.verifyStateProof(stateRoot, ...);
     * ```
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
     * @param txHash The transaction hash to verify
     * @param blockHash The block hash containing the transaction
     * @param rlpEncodedTx The RLP-encoded transaction data
     * @param proof Merkle proof for transaction inclusion
     * @return valid True if the transaction is included in the block
     *
     * Verifies that a specific transaction was included in an Ethereum
     * block and that the block is part of the canonical chain.
     *
     * Requirements:
     * - blockHash must be from a verified Ethereum block
     * - rlpEncodedTx must match the txHash
     * - proof must be a valid inclusion proof
     *
     * Use Cases:
     * - Cross-chain event verification
     * - Token deposit confirmation
     * - Cross-L1/L2 message passing
     *
     * Example:
     * ```solidity
     * // Verify mainnet deposit transaction
     * bytes32 txHash = 0xdef...;
     * bytes32 blockHash = 0x456...;
     * bytes memory rlpTx = getRLPTransaction(txHash);
     * bytes32[] memory proof = getInclusionProof(txHash);
     *
     * bool valid = proof.verifyTransaction(txHash, blockHash, rlpTx, proof);
     * require(valid, "Transaction not verified");
     *
     * // Process deposit...
     * ```
     */
    function verifyTransaction(
        bytes32 txHash,
        bytes32 blockHash,
        bytes calldata rlpEncodedTx,
        bytes32[] calldata proof
    ) external view returns (bool valid);
}

/**
 * @title Advanced Usage Examples
 * @dev These examples show common patterns for state proof verification
 */
contract StateProofExamples {
    IEthereumStateProof constant PROOF = IEthereumStateProof(0x0000000000000000000000000000000000000804);

    /**
     * @notice Example: Verify mainnet ERC20 balance
     */
    function verifyMainnetBalance(
        address token,
        address account,
        uint256 expectedBalance,
        bytes32[] calldata proof
    ) external view returns (bool) {
        // Get latest Ethereum state root
        (, , bytes32 stateRoot, ) = PROOF.getLatestEthBlock();

        // Calculate storage slot for ERC20 balance
        // balances[account] is typically at slot keccak256(account, 0)
        bytes32 key = keccak256(abi.encode(account, uint256(0)));

        // Encode expected balance
        bytes memory value = abi.encode(expectedBalance);

        // Verify proof
        return PROOF.verifyStateProof(stateRoot, proof, key, value);
    }

    /**
     * @notice Example: Cross-chain token bridge
     */
    function processMainnetDeposit(
        bytes32 txHash,
        bytes32 blockHash,
        bytes calldata rlpTx,
        bytes32[] calldata proof,
        address recipient,
        uint256 amount
    ) external {
        // Verify transaction was included on mainnet
        bool valid = PROOF.verifyTransaction(txHash, blockHash, rlpTx, proof);
        require(valid, "Invalid deposit proof");

        // Decode and validate transaction data
        // (In production, parse rlpTx to extract deposit details)

        // Mint tokens on ETH-PBC
        // mint(recipient, amount);
    }

    /**
     * @notice Example: Verify contract code hash
     */
    function verifyContractCode(
        address mainnetContract,
        bytes32 expectedCodeHash,
        bytes32[] calldata proof
    ) external view returns (bool) {
        (, , bytes32 stateRoot, ) = PROOF.getLatestEthBlock();

        // Account code hash is stored in account state
        bytes32 key = keccak256(abi.encode(mainnetContract));
        bytes memory value = abi.encode(expectedCodeHash);

        return PROOF.verifyStateProof(stateRoot, proof, key, value);
    }

    /**
     * @notice Example: Time-locked verification
     */
    function verifyRecentState(
        bytes32 stateRoot,
        bytes32[] calldata proof,
        bytes32 key,
        bytes calldata value,
        uint256 maxAge
    ) external view returns (bool) {
        // Get latest block info
        (, , , uint256 timestamp) = PROOF.getLatestEthBlock();

        // Ensure data is recent
        require(block.timestamp - timestamp <= maxAge, "Data too old");

        // Verify proof
        return PROOF.verifyStateProof(stateRoot, proof, key, value);
    }
}
