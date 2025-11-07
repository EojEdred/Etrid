// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title StakingRewards
 * @notice Example contract using Ëtrid Staking Precompile (0x802)
 * @dev Uses XCM to query FlareChain validator and staking information
 */

// Interface for Ëtrid Staking Precompile
interface IEtridStaking {
    function getValidatorStake(bytes32 validatorId) external view returns (uint256 stake);
    function isValidatorActive(bytes32 validatorId) external view returns (bool active);
    function getTotalStaked() external view returns (uint256 totalStake);
    function getValidatorCount() external view returns (uint256 count);
}

contract StakingRewards {
    // Staking precompile address
    IEtridStaking private constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    event ValidatorQueried(bytes32 indexed validatorId, uint256 stake, bool active);
    event RewardCalculated(bytes32 indexed validatorId, uint256 reward);
    event NetworkStatsQueried(uint256 totalStaked, uint256 validatorCount);

    /**
     * @notice Get stake amount for a validator
     * @param validatorId Validator ID (bytes32)
     * @return stake Stake amount in native currency (18 decimals)
     */
    function getValidatorStake(string memory validatorId) public view returns (uint256 stake) {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        stake = STAKING.getValidatorStake(validatorIdBytes);
        return stake;
    }

    /**
     * @notice Check if a validator is active
     * @param validatorId Validator ID
     * @return active True if validator is active
     */
    function isValidatorActive(string memory validatorId) public view returns (bool active) {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        active = STAKING.isValidatorActive(validatorIdBytes);
        return active;
    }

    /**
     * @notice Get total amount staked across all validators
     * @return totalStake Total staked amount
     */
    function getTotalStaked() public view returns (uint256 totalStake) {
        totalStake = STAKING.getTotalStaked();
        return totalStake;
    }

    /**
     * @notice Get number of active validators
     * @return count Validator count
     */
    function getValidatorCount() public view returns (uint256 count) {
        count = STAKING.getValidatorCount();
        return count;
    }

    /**
     * @notice Query validator info and emit event
     * @param validatorId Validator ID
     */
    function queryValidator(string memory validatorId) external {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        uint256 stake = STAKING.getValidatorStake(validatorIdBytes);
        bool active = STAKING.isValidatorActive(validatorIdBytes);

        emit ValidatorQueried(validatorIdBytes, stake, active);
    }

    /**
     * @notice Query network stats and emit event
     */
    function queryNetworkStats() external {
        uint256 totalStaked = STAKING.getTotalStaked();
        uint256 validatorCount = STAKING.getValidatorCount();

        emit NetworkStatsQueried(totalStaked, validatorCount);
    }

    /**
     * @notice Calculate validator's share of total stake
     * @param validatorId Validator ID
     * @return sharePercentage Share as percentage (with 2 decimals, e.g., 1550 = 15.50%)
     */
    function getValidatorSharePercentage(string memory validatorId) public view returns (uint256 sharePercentage) {
        uint256 validatorStake = getValidatorStake(validatorId);
        uint256 totalStake = getTotalStaked();

        require(totalStake > 0, "No stake in network");

        // Calculate percentage with 2 decimal places
        sharePercentage = (validatorStake * 10000) / totalStake;
        return sharePercentage;
    }

    // Helper function to convert string to bytes32
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }

        assembly {
            result := mload(add(source, 32))
        }
    }
}

/**
 * @title DelegatedStakingPool
 * @notice Example staking pool that distributes rewards based on FlareChain validator performance
 * @dev Demonstrates advanced use of Staking precompile
 */
contract DelegatedStakingPool {
    IEtridStaking private constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    string[] public trackedValidators;
    mapping(address => uint256) public userShares;
    uint256 public totalShares;
    uint256 public accumulatedRewards;

    event ValidatorAdded(string indexed validatorId);
    event UserDeposited(address indexed user, uint256 amount, uint256 shares);
    event UserWithdrew(address indexed user, uint256 amount, uint256 shares);
    event RewardsDistributed(uint256 amount);

    /**
     * @notice Add a validator to track
     * @param validatorId Validator ID to track
     */
    function addValidator(string memory validatorId) external {
        // Verify validator is active
        require(isValidatorActive(validatorId), "Validator not active");

        trackedValidators.push(validatorId);
        emit ValidatorAdded(validatorId);
    }

    /**
     * @notice Get total stake across all tracked validators
     * @return totalValidatorStake Total stake amount
     */
    function getTotalValidatorStake() public view returns (uint256 totalValidatorStake) {
        for (uint256 i = 0; i < trackedValidators.length; i++) {
            bytes32 validatorIdBytes = stringToBytes32(trackedValidators[i]);
            totalValidatorStake += STAKING.getValidatorStake(validatorIdBytes);
        }
        return totalValidatorStake;
    }

    /**
     * @notice Calculate average stake per validator
     * @return averageStake Average stake amount
     */
    function getAverageValidatorStake() public view returns (uint256 averageStake) {
        require(trackedValidators.length > 0, "No validators tracked");
        uint256 totalStake = getTotalValidatorStake();
        averageStake = totalStake / trackedValidators.length;
        return averageStake;
    }

    /**
     * @notice Get pool's share of network stake
     * @return sharePercentage Share as percentage (2 decimals)
     */
    function getPoolNetworkShare() public view returns (uint256 sharePercentage) {
        uint256 poolStake = getTotalValidatorStake();
        uint256 networkStake = STAKING.getTotalStaked();

        require(networkStake > 0, "No network stake");

        sharePercentage = (poolStake * 10000) / networkStake;
        return sharePercentage;
    }

    /**
     * @notice Simulate deposit (no actual token transfer)
     * @param amount Amount to deposit
     */
    function deposit(uint256 amount) external {
        require(amount > 0, "Invalid amount");

        uint256 shares;
        if (totalShares == 0) {
            shares = amount;
        } else {
            // Calculate shares based on current pool value
            uint256 poolValue = getTotalValidatorStake();
            shares = (amount * totalShares) / poolValue;
        }

        userShares[msg.sender] += shares;
        totalShares += shares;

        emit UserDeposited(msg.sender, amount, shares);
    }

    /**
     * @notice Calculate user's withdrawable amount
     * @param user User address
     * @return amount Withdrawable amount
     */
    function calculateWithdrawableAmount(address user) public view returns (uint256 amount) {
        require(totalShares > 0, "No shares exist");

        uint256 poolValue = getTotalValidatorStake() + accumulatedRewards;
        uint256 userShareAmount = userShares[user];

        amount = (poolValue * userShareAmount) / totalShares;
        return amount;
    }

    /**
     * @notice Check if a validator is active
     * @param validatorId Validator ID
     * @return active True if validator is active
     */
    function isValidatorActive(string memory validatorId) internal view returns (bool active) {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        return STAKING.isValidatorActive(validatorIdBytes);
    }

    /**
     * @notice Get count of active validators in pool
     * @return activeCount Number of active validators
     */
    function getActiveValidatorCount() public view returns (uint256 activeCount) {
        for (uint256 i = 0; i < trackedValidators.length; i++) {
            if (isValidatorActive(trackedValidators[i])) {
                activeCount++;
            }
        }
        return activeCount;
    }

    /**
     * @notice Get list of tracked validators
     * @return validators Array of validator IDs
     */
    function getTrackedValidators() external view returns (string[] memory validators) {
        return trackedValidators;
    }

    // Helper function
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }
        assembly {
            result := mload(add(source, 32))
        }
    }
}

/**
 * @title ValidatorPerformanceTracker
 * @notice Tracks and compares validator performance metrics
 * @dev Uses Staking precompile to monitor validator statistics
 */
contract ValidatorPerformanceTracker {
    IEtridStaking private constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    struct ValidatorSnapshot {
        uint256 stake;
        bool active;
        uint256 timestamp;
    }

    mapping(bytes32 => ValidatorSnapshot[]) public validatorHistory;

    event SnapshotTaken(bytes32 indexed validatorId, uint256 stake, bool active, uint256 timestamp);

    /**
     * @notice Take a snapshot of validator's current state
     * @param validatorId Validator ID
     */
    function takeSnapshot(string memory validatorId) external {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);

        uint256 stake = STAKING.getValidatorStake(validatorIdBytes);
        bool active = STAKING.isValidatorActive(validatorIdBytes);

        ValidatorSnapshot memory snapshot = ValidatorSnapshot({
            stake: stake,
            active: active,
            timestamp: block.timestamp
        });

        validatorHistory[validatorIdBytes].push(snapshot);

        emit SnapshotTaken(validatorIdBytes, stake, active, block.timestamp);
    }

    /**
     * @notice Get snapshot count for a validator
     * @param validatorId Validator ID
     * @return count Number of snapshots
     */
    function getSnapshotCount(string memory validatorId) external view returns (uint256 count) {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        return validatorHistory[validatorIdBytes].length;
    }

    /**
     * @notice Get a specific snapshot
     * @param validatorId Validator ID
     * @param index Snapshot index
     * @return snapshot Snapshot data
     */
    function getSnapshot(string memory validatorId, uint256 index) external view returns (
        uint256 stake,
        bool active,
        uint256 timestamp
    ) {
        bytes32 validatorIdBytes = stringToBytes32(validatorId);
        require(index < validatorHistory[validatorIdBytes].length, "Invalid index");

        ValidatorSnapshot memory snapshot = validatorHistory[validatorIdBytes][index];
        return (snapshot.stake, snapshot.active, snapshot.timestamp);
    }

    // Helper function
    function stringToBytes32(string memory source) internal pure returns (bytes32 result) {
        bytes memory tempEmptyStringTest = bytes(source);
        if (tempEmptyStringTest.length == 0) {
            return 0x0;
        }
        assembly {
            result := mload(add(source, 32))
        }
    }
}
