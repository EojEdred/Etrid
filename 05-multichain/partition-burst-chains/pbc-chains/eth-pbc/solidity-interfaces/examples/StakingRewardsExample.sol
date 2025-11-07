// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "../IEtridStaking.sol";

/**
 * @title Staking Rewards Example
 * @notice Demonstrates using Ëtrid Staking precompile for reward calculations
 * @dev This shows how to build a staking rewards system based on FlareChain validator data
 */
contract StakingRewardsExample {
    IEtridStaking private constant STAKING =
        IEtridStaking(0x0000000000000000000000000000000000000802);

    struct UserStake {
        bytes32 validator;      // Validator they delegated to
        uint256 amount;         // Amount staked
        uint256 lastClaimTime;  // Last time rewards were claimed
    }

    mapping(address => UserStake) public userStakes;
    uint256 public constant REWARD_RATE = 1e16; // 1% per year = 1e16 per second

    event Staked(address indexed user, bytes32 indexed validator, uint256 amount);
    event RewardsClaimed(address indexed user, uint256 amount);

    /**
     * @notice Stake tokens with a FlareChain validator
     * @param validator The validator ID to delegate to
     * @param amount Amount to stake
     */
    function stake(bytes32 validator, uint256 amount) external {
        // Check validator is active on FlareChain
        require(STAKING.isValidatorActive(validator), "Validator not active");

        // In production, transfer tokens from user here
        // token.transferFrom(msg.sender, address(this), amount);

        userStakes[msg.sender] = UserStake({
            validator: validator,
            amount: amount,
            lastClaimTime: block.timestamp
        });

        emit Staked(msg.sender, validator, amount);
    }

    /**
     * @notice Calculate pending rewards for a user
     * @param user Address to check rewards for
     * @return rewards Amount of pending rewards
     */
    function pendingRewards(address user) public view returns (uint256 rewards) {
        UserStake memory userStake = userStakes[user];
        if (userStake.amount == 0) return 0;

        // Calculate time-based rewards
        uint256 timePassed = block.timestamp - userStake.lastClaimTime;
        uint256 baseReward = (userStake.amount * REWARD_RATE * timePassed) / 1e18;

        // Get validator's total stake from FlareChain
        uint256 validatorStake = STAKING.getValidatorStake(userStake.validator);

        // Bonus: If validator has high stake, give bonus rewards
        if (validatorStake > 100_000 ether) {
            baseReward = (baseReward * 110) / 100; // 10% bonus
        }

        return baseReward;
    }

    /**
     * @notice Claim accumulated rewards
     */
    function claimRewards() external {
        uint256 rewards = pendingRewards(msg.sender);
        require(rewards > 0, "No rewards to claim");

        // Update last claim time
        userStakes[msg.sender].lastClaimTime = block.timestamp;

        // In production, transfer rewards to user
        // rewardToken.transfer(msg.sender, rewards);

        emit RewardsClaimed(msg.sender, rewards);
    }

    /**
     * @notice Get network staking statistics
     * @return totalStaked Total ETR staked across all validators
     * @return validatorCount Number of active validators
     * @return averageStake Average stake per validator
     */
    function getNetworkStats()
        external
        view
        returns (
            uint256 totalStaked,
            uint256 validatorCount,
            uint256 averageStake
        )
    {
        totalStaked = STAKING.getTotalStaked();
        validatorCount = STAKING.getValidatorCount();
        averageStake = validatorCount > 0 ? totalStaked / validatorCount : 0;

        return (totalStaked, validatorCount, averageStake);
    }

    /**
     * @notice Check if a validator is suitable for delegation
     * @param validator Validator ID to check
     * @return suitable True if validator meets criteria
     */
    function isValidatorSuitable(bytes32 validator)
        external
        view
        returns (bool suitable)
    {
        // Check validator is active
        if (!STAKING.isValidatorActive(validator)) {
            return false;
        }

        // Check validator has minimum stake (shows commitment)
        uint256 stake = STAKING.getValidatorStake(validator);
        if (stake < 1000 ether) {
            return false;
        }

        return true;
    }
}
