// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.20;

/**
 * @title MasterChef
 * @notice Yield farming contract for FlareSwap LP tokens
 * @dev Distributes ETR rewards to LP token stakers
 */

interface IERC20 {
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
}

contract MasterChef {
    struct UserInfo {
        uint256 amount;          // How many LP tokens the user has provided
        uint256 rewardDebt;      // Reward debt (see explanation below)
    }

    struct PoolInfo {
        IERC20 lpToken;          // Address of LP token contract
        uint256 allocPoint;       // How many allocation points assigned to this pool
        uint256 lastRewardBlock;  // Last block number that reward distribution occurs
        uint256 accRewardPerShare; // Accumulated rewards per share, times 1e12
    }

    // The reward TOKEN
    IERC20 public rewardToken;
    // Reward tokens created per block
    uint256 public rewardPerBlock;
    // Bonus multiplier for early stakers
    uint256 public constant BONUS_MULTIPLIER = 1;

    // Info of each pool
    PoolInfo[] public poolInfo;
    // Info of each user that stakes LP tokens
    mapping(uint256 => mapping(address => UserInfo)) public userInfo;
    // Total allocation points. Must be the sum of all allocation points in all pools
    uint256 public totalAllocPoint = 0;
    // The block number when reward mining starts
    uint256 public startBlock;

    // Owner address for emergency functions
    address public owner;

    event Deposit(address indexed user, uint256 indexed pid, uint256 amount);
    event Withdraw(address indexed user, uint256 indexed pid, uint256 amount);
    event EmergencyWithdraw(address indexed user, uint256 indexed pid, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == owner, "MasterChef: caller is not the owner");
        _;
    }

    constructor(
        IERC20 _rewardToken,
        uint256 _rewardPerBlock,
        uint256 _startBlock
    ) {
        rewardToken = _rewardToken;
        rewardPerBlock = _rewardPerBlock;
        startBlock = _startBlock;
        owner = msg.sender;
    }

    function poolLength() external view returns (uint256) {
        return poolInfo.length;
    }

    // Add a new LP to the pool. Can only be called by the owner
    function add(uint256 _allocPoint, IERC20 _lpToken, bool _withUpdate) public onlyOwner {
        if (_withUpdate) {
            massUpdatePools();
        }
        uint256 lastRewardBlock = block.number > startBlock ? block.number : startBlock;
        totalAllocPoint = totalAllocPoint + _allocPoint;
        poolInfo.push(PoolInfo({
            lpToken: _lpToken,
            allocPoint: _allocPoint,
            lastRewardBlock: lastRewardBlock,
            accRewardPerShare: 0
        }));
    }

    // Update the given pool's allocation point. Can only be called by the owner
    function set(uint256 _pid, uint256 _allocPoint, bool _withUpdate) public onlyOwner {
        if (_withUpdate) {
            massUpdatePools();
        }
        totalAllocPoint = totalAllocPoint - poolInfo[_pid].allocPoint + _allocPoint;
        poolInfo[_pid].allocPoint = _allocPoint;
    }

    // Return reward multiplier over the given _from to _to block
    function getMultiplier(uint256 _from, uint256 _to) public pure returns (uint256) {
        return (_to - _from) * BONUS_MULTIPLIER;
    }

    // View function to see pending rewards on frontend
    function pendingReward(uint256 _pid, address _user) external view returns (uint256) {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][_user];
        uint256 accRewardPerShare = pool.accRewardPerShare;
        uint256 lpSupply = pool.lpToken.balanceOf(address(this));
        if (block.number > pool.lastRewardBlock && lpSupply != 0) {
            uint256 multiplier = getMultiplier(pool.lastRewardBlock, block.number);
            uint256 reward = multiplier * rewardPerBlock * pool.allocPoint / totalAllocPoint;
            accRewardPerShare = accRewardPerShare + (reward * 1e12 / lpSupply);
        }
        return user.amount * accRewardPerShare / 1e12 - user.rewardDebt;
    }

    // Update reward variables for all pools. Be careful of gas spending!
    function massUpdatePools() public {
        uint256 length = poolInfo.length;
        for (uint256 pid = 0; pid < length; ++pid) {
            updatePool(pid);
        }
    }

    // Update reward variables of the given pool to be up-to-date
    function updatePool(uint256 _pid) public {
        PoolInfo storage pool = poolInfo[_pid];
        if (block.number <= pool.lastRewardBlock) {
            return;
        }
        uint256 lpSupply = pool.lpToken.balanceOf(address(this));
        if (lpSupply == 0) {
            pool.lastRewardBlock = block.number;
            return;
        }
        uint256 multiplier = getMultiplier(pool.lastRewardBlock, block.number);
        uint256 reward = multiplier * rewardPerBlock * pool.allocPoint / totalAllocPoint;
        pool.accRewardPerShare = pool.accRewardPerShare + (reward * 1e12 / lpSupply);
        pool.lastRewardBlock = block.number;
    }

    // Deposit LP tokens to MasterChef for reward allocation
    function deposit(uint256 _pid, uint256 _amount) public {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];
        updatePool(_pid);
        if (user.amount > 0) {
            uint256 pending = user.amount * pool.accRewardPerShare / 1e12 - user.rewardDebt;
            if(pending > 0) {
                safeRewardTransfer(msg.sender, pending);
            }
        }
        if(_amount > 0) {
            pool.lpToken.transferFrom(address(msg.sender), address(this), _amount);
            user.amount = user.amount + _amount;
        }
        user.rewardDebt = user.amount * pool.accRewardPerShare / 1e12;
        emit Deposit(msg.sender, _pid, _amount);
    }

    // Withdraw LP tokens from MasterChef
    function withdraw(uint256 _pid, uint256 _amount) public {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];
        require(user.amount >= _amount, "withdraw: not good");
        updatePool(_pid);
        uint256 pending = user.amount * pool.accRewardPerShare / 1e12 - user.rewardDebt;
        if(pending > 0) {
            safeRewardTransfer(msg.sender, pending);
        }
        if(_amount > 0) {
            user.amount = user.amount - _amount;
            pool.lpToken.transfer(address(msg.sender), _amount);
        }
        user.rewardDebt = user.amount * pool.accRewardPerShare / 1e12;
        emit Withdraw(msg.sender, _pid, _amount);
    }

    // Withdraw without caring about rewards. EMERGENCY ONLY
    function emergencyWithdraw(uint256 _pid) public {
        PoolInfo storage pool = poolInfo[_pid];
        UserInfo storage user = userInfo[_pid][msg.sender];
        uint256 amount = user.amount;
        user.amount = 0;
        user.rewardDebt = 0;
        pool.lpToken.transfer(address(msg.sender), amount);
        emit EmergencyWithdraw(msg.sender, _pid, amount);
    }

    // Safe reward transfer function, just in case if rounding error causes pool to not have enough tokens
    function safeRewardTransfer(address _to, uint256 _amount) internal {
        uint256 rewardBal = rewardToken.balanceOf(address(this));
        if (_amount > rewardBal) {
            rewardToken.transfer(_to, rewardBal);
        } else {
            rewardToken.transfer(_to, _amount);
        }
    }

    // Update reward per block. Can only be called by the owner
    function updateRewardPerBlock(uint256 _rewardPerBlock) public onlyOwner {
        massUpdatePools();
        rewardPerBlock = _rewardPerBlock;
    }

    // Transfer ownership
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "MasterChef: new owner is the zero address");
        owner = newOwner;
    }
}
