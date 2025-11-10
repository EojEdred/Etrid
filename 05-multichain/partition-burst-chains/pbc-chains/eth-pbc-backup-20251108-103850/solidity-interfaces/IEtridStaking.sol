// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEtridStaking
 * @notice Interface for querying FlareChain validator and staking information from ETH-PBC
 * @dev Precompile address: 0x0000000000000000000000000000000000000802
 *
 * This precompile allows Solidity contracts on ETH-PBC to query real-time
 * staking and validator data from FlareChain via XCM messaging.
 *
 * All stake amounts are returned in wei (1 ETR = 1e18 wei).
 *
 * Example Usage:
 * ```solidity
 * IEtridStaking staking = IEtridStaking(0x0000000000000000000000000000000000000802);
 * uint256 stake = staking.getValidatorStake(validatorId);
 * bool isActive = staking.isValidatorActive(validatorId);
 * uint256 total = staking.getTotalStaked();
 * ```
 */
interface IEtridStaking {
    /**
     * @notice Get the total stake amount for a specific validator
     * @param validatorId The validator's unique identifier (32 bytes)
     * @return stake The total amount staked with this validator (in wei)
     *
     * Example:
     * ```solidity
     * bytes32 validator = 0x1234...;
     * uint256 stake = staking.getValidatorStake(validator);
     * // Returns: 1_000_000_000_000_000_000_000 (1000 ETR)
     * ```
     */
    function getValidatorStake(bytes32 validatorId) external view returns (uint256 stake);

    /**
     * @notice Check if a validator is currently active
     * @param validatorId The validator's unique identifier (32 bytes)
     * @return active True if validator is active, false otherwise
     *
     * A validator is considered active if they:
     * - Have sufficient stake (minimum threshold met)
     * - Are not slashed or jailed
     * - Are participating in consensus
     *
     * Example:
     * ```solidity
     * bytes32 validator = 0x1234...;
     * bool isActive = staking.isValidatorActive(validator);
     * require(isActive, "Validator is not active");
     * ```
     */
    function isValidatorActive(bytes32 validatorId) external view returns (bool active);

    /**
     * @notice Get the total amount staked across all validators
     * @return totalStaked The sum of all validator stakes (in wei)
     *
     * This represents the total network security via staked ETR.
     *
     * Example:
     * ```solidity
     * uint256 total = staking.getTotalStaked();
     * // Returns: 1_000_000_000_000_000_000_000_000 (1M ETR)
     * ```
     */
    function getTotalStaked() external view returns (uint256 totalStaked);

    /**
     * @notice Get the total number of validators in the active set
     * @return count The number of active validators
     *
     * Example:
     * ```solidity
     * uint256 count = staking.getValidatorCount();
     * // Returns: 100 (100 active validators)
     * ```
     */
    function getValidatorCount() external view returns (uint256 count);
}
