// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridStaking
 * @dev Interface for Etrid Staking Precompile (0x802)
 *
 * Query FlareChain validator and staking information from ETH PBC.
 * Provides transparency into network security and validator performance.
 *
 * Address: 0x0000000000000000000000000000000000000802
 *
 * @notice Read-only queries - staking operations must be done on FlareChain
 */
interface IEtridStaking {
    /**
     * @notice Get the stake amount for a specific validator
     * @param validatorId Validator identifier (bytes32)
     * @return stake Stake amount in wei
     *
     * @dev Example:
     *   bytes32 validatorId = bytes32(uint256(uint160(validatorAddress)));
     *   uint256 stake = staking.getValidatorStake(validatorId);
     *   require(stake >= 1000 ether, "Validator has insufficient stake");
     */
    function getValidatorStake(bytes32 validatorId)
        external
        view
        returns (uint256 stake);

    /**
     * @notice Check if a validator is currently active
     * @param validatorId Validator identifier
     * @return active true if validator is active and producing blocks
     *
     * @dev Example:
     *   bool isActive = staking.isValidatorActive(validatorId);
     *   require(isActive, "Cannot delegate to inactive validator");
     */
    function isValidatorActive(bytes32 validatorId)
        external
        view
        returns (bool active);

    /**
     * @notice Get total amount staked across all validators
     * @return totalStaked Total network stake in wei
     *
     * @dev Example:
     *   uint256 total = staking.getTotalStaked();
     *   uint256 myStake = staking.getValidatorStake(myValidatorId);
     *   uint256 myShare = (myStake * 10000) / total; // Share in basis points
     */
    function getTotalStaked() external view returns (uint256 totalStaked);

    /**
     * @notice Get total number of validators
     * @return count Number of validators (active + inactive)
     *
     * @dev Example:
     *   uint256 validatorCount = staking.getValidatorCount();
     *   require(validatorCount >= 100, "Network needs more validators");
     */
    function getValidatorCount() external view returns (uint256 count);
}
