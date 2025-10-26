// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @title MasterChef
 * @dev ÉTR LP Rewards Distribution Contract
 *
 * Features:
 * - Multi-pool LP token staking
 * - Time-based ÉTR emission rewards
 * - Per-block reward calculation
 * - Emergency withdraw function
 * - Pausable for emergency situations
 *
 * Based on PancakeSwap MasterChef V2
 * Adapted for Ëtrid Protocol Community LP Rewards Program
 */
contract MasterChef is Ownable, ReentrancyGuard, Pausable {
    using SafeERC20 for IERC20;

    // Info of each user
    struct UserInfo {
        uint256 amount;         // How many LP tokens the user has provided
        uint256 rewardDebt;     // Reward debt (see explanation below)
        uint256 pendingRewards; // Rewards not yet harvested
    }

    // Info of each pool
    struct PoolInfo {
        IERC20 lpToken;           // Address of LP token contract
        uint256 allocPoint;       // Allocation points assigned to this pool (weight)
        uint256 lastRewardBlock;  // Last block number that ÉTR distribution occurred
        uint256 accRewardPerShare; // Accumulated ÉTR per share, times 1e12
        uint256 totalStaked;      // Total LP tokens staked in this pool
    }

    // The ÉTR reward token
    IERC20 public rewardToken;

    // ÉTR tokens rewarded per block
    uint256 public rewardPerBlock;

    // Info of each pool
    PoolInfo[] public poolInfo;

    // Info of each user that stakes LP tokens
    mapping(uint256 => mapping(address => UserInfo)) public userInfo;

    // Total allocation points (must be the sum of all allocation points in all pools)
    uint256 public totalAllocPoint = 0;

    // The block number when ÉTR mining starts
    uint256 public startBlock;

    // Events
    event Deposit(address indexed user, uint256 indexed pid, uint256 amount);
    event Withdraw(address indexed user, uint256 indexed pid, uint256 amount);
    event EmergencyWithdraw(address indexed user, uint256 indexed pid, uint256 amount);
    event Harvest(address indexed user, uint256 indexed pid, uint256 amount);
    event PoolAdded(uint256 indexed pid, address lpToken, uint256 allocPoint);
    event PoolUpdated(uint256 indexed pid, uint256 allocPoint);
    event RewardPerBlockUpdated(uint256 oldRate, uint256 newRate);

    /**
     * @dev Constructor
     * @param _rewardToken The ÉTR token address
     * @param _rewardPerBlock Initial ÉTR per block (can be updated)
     * @param _startBlock Block number when rewards start
     */
    constructor(
        IERC20 _rewardToken,
        uint256 _rewardPerBlock,
        uint256 _startBlock
    ) {
        rewardToken = _rewardToken;
        rewardPerBlock = _rewardPerBlock;
        startBlock = _startBlock;
    }

    /**
     * @dev Returns number of pools
     */
    function poolLength() external view returns (uint256) {
        return poolInfo.length;
    }

    /**
     * @dev Add a new LP pool (only owner)
     * @param _allocPoint Allocation points for this pool
     * @param _lpToken LP token address
     * @param _withUpdate Whether to update all pools before adding
     */
    function add(
        uint256 _allocPoint,
        IERC20 _lpToken,
        bool _withUpdate
    ) external onlyOwner {
        // Check if pool already exists
        for (uint256 i = 0; i < poolInfo.length; i++) {
            require(poolInfo[i].lpToken != _lpToken, "MasterChef: LP token already added");
        }

        if (_withUpdate) {
            massUpdatePools();
        }

        uint256 lastRewardBlock = block.number > startBlock ? block.number : startBlock;
        totalAllocPoint += _allocPoint;

        poolInfo.push(
            PoolInfo({
                lpToken: _lpToken,
                allocPoint: _allocPoint,
                lastRewardBlock: lastRewardBlock,
                accRewardPerShare: 0,
                totalStaked: 0
            })
        );

        emit PoolAdded(poolInfo.length - 1, address(_lpToken), _allocPoint);
    }

    /**
     * @dev Update allocation points of a pool (only owner)
     * @param _pid Pool ID
     * @param _allocPoint New allocation points
     * @param _withUpdate Whether to update all pools before changing
     */
    function set(
        uint256 _pid,
        uint256 _allocPoint,
        bool _withUpdate
    ) external onlyOwner {
        if (_withUpdate) {
            massUpdatePools();
        }

        totalAllocPoint = totalAllocPoint - poolInfo[_pid].allocPoint + _allocPoint;
        poolInfo[_pid].allocPoint = _allocPoint;

        emit PoolUpdated(_pid, _allocPoint);
    }

    /**
     * @dev Update reward per block (only owner)
     * @param _rewardPerBlock New reward per block
     */
    function updateRewardPerBlock(uint256 _rewardPerBlock) external onlyOwner {
        massUpdatePools(); // Update all pools before changing rate
        uint256 oldRate = rewardPerBlock;
        rewardPerBlock = _rewardPerBlock;
        emit RewardPerBlockUpdated(oldRate, _rewardPerBlock);
    }

    /**
     * @dev View function to see pending ÉTR rewards for a user
     * @param _pid Pool ID
     * @param _user User address
     * @return Pending ÉTR rewards
     */
    function pendingReward(uint256 _pid, address _user) external view returns (uint256) {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][_user];

        uint256 accRewardPerShare = pool.accRewardPerShare;
        uint256 lpSupply = pool.totalStaked;

        if (block.number > pool.lastRewardBlock && lpSupply != 0) {
            uint256 blocks = block.number - pool.lastRewardBlock;
            uint256 reward = (blocks * rewardPerBlock * pool.allocPoint) / totalAllocPoint;
            accRewardPerShare += (reward * 1e12) / lpSupply;
        }

        uint256 pending = (user.amount * accRewardPerShare) / 1e12 - user.rewardDebt;
        return pending + user.pendingRewards;
    }

    /**
     * @dev Update reward variables for all pools
     */
    function massUpdatePools() public {
        uint256 length = poolInfo.length;
        for (uint256 pid = 0; pid < length; ++pid) {
            updatePool(pid);
        }
    }

    /**
     * @dev Update reward variables of the given pool
     * @param _pid Pool ID
     */
    function updatePool(uint256 _pid) public {
        PoolInfo storage pool = poolInfo[_pid];

        if (block.number <= pool.lastRewardBlock) {
            return;
        }

        uint256 lpSupply = pool.totalStaked;
        if (lpSupply == 0 || pool.allocPoint == 0) {
            pool.lastRewardBlock = block.number;
            return;
        }

        uint256 blocks = block.number - pool.lastRewardBlock;
        uint256 reward = (blocks * rewardPerBlock * pool.allocPoint) / totalAllocPoint;

        pool.accRewardPerShare += (reward * 1e12) / lpSupply;
        pool.lastRewardBlock = block.number;
    }

    /**
     * @dev Deposit LP tokens to earn ÉTR rewards
     * @param _pid Pool ID
     * @param _amount Amount of LP tokens to deposit
     */
    function deposit(uint256 _pid, uint256 _amount) external nonReentrant whenNotPaused {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];

        updatePool(_pid);

        // Harvest existing rewards
        if (user.amount > 0) {
            uint256 pending = (user.amount * pool.accRewardPerShare) / 1e12 - user.rewardDebt;
            if (pending > 0) {
                user.pendingRewards += pending;
            }
        }

        // Transfer LP tokens from user
        if (_amount > 0) {
            uint256 balanceBefore = pool.lpToken.balanceOf(address(this));
            pool.lpToken.safeTransferFrom(msg.sender, address(this), _amount);
            uint256 balanceAfter = pool.lpToken.balanceOf(address(this));
            uint256 actualAmount = balanceAfter - balanceBefore;

            user.amount += actualAmount;
            pool.totalStaked += actualAmount;
        }

        user.rewardDebt = (user.amount * pool.accRewardPerShare) / 1e12;
        emit Deposit(msg.sender, _pid, _amount);
    }

    /**
     * @dev Withdraw LP tokens
     * @param _pid Pool ID
     * @param _amount Amount of LP tokens to withdraw
     */
    function withdraw(uint256 _pid, uint256 _amount) external nonReentrant {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];

        require(user.amount >= _amount, "MasterChef: withdraw amount exceeds balance");

        updatePool(_pid);

        // Harvest existing rewards
        uint256 pending = (user.amount * pool.accRewardPerShare) / 1e12 - user.rewardDebt;
        if (pending > 0) {
            user.pendingRewards += pending;
        }

        // Withdraw LP tokens
        if (_amount > 0) {
            user.amount -= _amount;
            pool.totalStaked -= _amount;
            pool.lpToken.safeTransfer(msg.sender, _amount);
        }

        user.rewardDebt = (user.amount * pool.accRewardPerShare) / 1e12;
        emit Withdraw(msg.sender, _pid, _amount);
    }

    /**
     * @dev Harvest ÉTR rewards without withdrawing LP tokens
     * @param _pid Pool ID
     */
    function harvest(uint256 _pid) external nonReentrant {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];

        updatePool(_pid);

        uint256 pending = (user.amount * pool.accRewardPerShare) / 1e12 - user.rewardDebt;
        pending += user.pendingRewards;

        if (pending > 0) {
            user.pendingRewards = 0;
            safeRewardTransfer(msg.sender, pending);
            emit Harvest(msg.sender, _pid, pending);
        }

        user.rewardDebt = (user.amount * pool.accRewardPerShare) / 1e12;
    }

    /**
     * @dev Emergency withdraw LP tokens WITHOUT claiming rewards
     * @param _pid Pool ID
     */
    function emergencyWithdraw(uint256 _pid) external nonReentrant {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];

        uint256 amount = user.amount;
        user.amount = 0;
        user.rewardDebt = 0;
        user.pendingRewards = 0;
        pool.totalStaked -= amount;

        pool.lpToken.safeTransfer(msg.sender, amount);
        emit EmergencyWithdraw(msg.sender, _pid, amount);
    }

    /**
     * @dev Safe ÉTR transfer function (handles insufficient balance)
     * @param _to Recipient address
     * @param _amount Amount to transfer
     */
    function safeRewardTransfer(address _to, uint256 _amount) internal {
        uint256 rewardBalance = rewardToken.balanceOf(address(this));

        if (_amount > rewardBalance) {
            rewardToken.safeTransfer(_to, rewardBalance);
        } else {
            rewardToken.safeTransfer(_to, _amount);
        }
    }

    /**
     * @dev Pause deposits (emergency only)
     */
    function pause() external onlyOwner {
        _pause();
    }

    /**
     * @dev Unpause deposits
     */
    function unpause() external onlyOwner {
        _unpause();
    }

    /**
     * @dev Emergency function to recover tokens sent by mistake
     * @param _token Token address
     * @param _amount Amount to recover
     */
    function emergencyRewardWithdraw(address _token, uint256 _amount) external onlyOwner {
        require(_token != address(rewardToken) || block.number > startBlock + 10000000,
            "MasterChef: cannot withdraw reward token during active rewards");

        IERC20(_token).safeTransfer(msg.sender, _amount);
    }
}
