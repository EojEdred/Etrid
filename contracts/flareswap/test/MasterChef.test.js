// MasterChef Test Suite
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MasterChef", function () {
  let masterChef, rewardToken, lpToken1, lpToken2;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    // Deploy reward token (ETR)
    const Token = await ethers.getContractFactory("MockERC20");
    rewardToken = await Token.deploy("Etrid Token", "ETR", ethers.utils.parseEther("1000000"));
    await rewardToken.deployed();

    // Deploy LP tokens
    lpToken1 = await Token.deploy("LP Token 1", "LP1", ethers.utils.parseEther("10000"));
    lpToken2 = await Token.deploy("LP Token 2", "LP2", ethers.utils.parseEther("10000"));
    await lpToken1.deployed();
    await lpToken2.deployed();

    // Deploy MasterChef
    const currentBlock = await ethers.provider.getBlockNumber();
    const rewardPerBlock = ethers.utils.parseEther("10"); // 10 ETR per block

    const MasterChef = await ethers.getContractFactory("MasterChef");
    masterChef = await MasterChef.deploy(
      rewardToken.address,
      rewardPerBlock,
      currentBlock + 10 // start in 10 blocks
    );
    await masterChef.deployed();

    // Transfer reward tokens to MasterChef
    await rewardToken.transfer(masterChef.address, ethers.utils.parseEther("100000"));
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await masterChef.owner()).to.equal(owner.address);
    });

    it("Should set the right reward token", async function () {
      expect(await masterChef.rewardToken()).to.equal(rewardToken.address);
    });

    it("Should set the right reward per block", async function () {
      const expected = ethers.utils.parseEther("10");
      expect(await masterChef.rewardPerBlock()).to.equal(expected);
    });
  });

  describe("Pool Management", function () {
    it("Should add a new pool", async function () {
      await masterChef.add(100, lpToken1.address, false);
      expect(await masterChef.poolLength()).to.equal(1);

      const pool = await masterChef.poolInfo(0);
      expect(pool.lpToken).to.equal(lpToken1.address);
      expect(pool.allocPoint).to.equal(100);
    });

    it("Should add multiple pools", async function () {
      await masterChef.add(100, lpToken1.address, false);
      await masterChef.add(200, lpToken2.address, false);

      expect(await masterChef.poolLength()).to.equal(2);
      expect(await masterChef.totalAllocPoint()).to.equal(300);
    });

    it("Should update pool allocation", async function () {
      await masterChef.add(100, lpToken1.address, false);
      await masterChef.set(0, 200, false);

      const pool = await masterChef.poolInfo(0);
      expect(pool.allocPoint).to.equal(200);
      expect(await masterChef.totalAllocPoint()).to.equal(200);
    });

    it("Should only allow owner to add pools", async function () {
      await expect(
        masterChef.connect(addr1).add(100, lpToken1.address, false)
      ).to.be.revertedWith("MasterChef: caller is not the owner");
    });
  });

  describe("Staking", function () {
    beforeEach(async function () {
      // Add pool
      await masterChef.add(100, lpToken1.address, false);

      // Transfer LP tokens to users
      await lpToken1.transfer(addr1.address, ethers.utils.parseEther("100"));
      await lpToken1.transfer(addr2.address, ethers.utils.parseEther("100"));
    });

    it("Should allow users to deposit LP tokens", async function () {
      const depositAmount = ethers.utils.parseEther("50");

      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);

      const userInfo = await masterChef.userInfo(0, addr1.address);
      expect(userInfo.amount).to.equal(depositAmount);
    });

    it("Should allow users to withdraw LP tokens", async function () {
      const depositAmount = ethers.utils.parseEther("50");
      const withdrawAmount = ethers.utils.parseEther("30");

      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);
      await masterChef.connect(addr1).withdraw(0, withdrawAmount);

      const userInfo = await masterChef.userInfo(0, addr1.address);
      expect(userInfo.amount).to.equal(depositAmount.sub(withdrawAmount));
    });

    it("Should not allow withdrawal of more than deposited", async function () {
      const depositAmount = ethers.utils.parseEther("50");
      const withdrawAmount = ethers.utils.parseEther("60");

      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);

      await expect(
        masterChef.connect(addr1).withdraw(0, withdrawAmount)
      ).to.be.revertedWith("withdraw: not good");
    });
  });

  describe("Rewards", function () {
    beforeEach(async function () {
      // Add pool
      const currentBlock = await ethers.provider.getBlockNumber();
      const MasterChef = await ethers.getContractFactory("MasterChef");
      masterChef = await MasterChef.deploy(
        rewardToken.address,
        ethers.utils.parseEther("10"),
        currentBlock + 1 // start in next block
      );
      await masterChef.deployed();
      await rewardToken.transfer(masterChef.address, ethers.utils.parseEther("100000"));

      await masterChef.add(100, lpToken1.address, false);

      // Transfer LP tokens to users
      await lpToken1.transfer(addr1.address, ethers.utils.parseEther("100"));
    });

    it("Should accumulate rewards over time", async function () {
      const depositAmount = ethers.utils.parseEther("50");

      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);

      // Mine some blocks
      for (let i = 0; i < 5; i++) {
        await ethers.provider.send("evm_mine");
      }

      const pending = await masterChef.pendingReward(0, addr1.address);
      expect(pending).to.be.gt(0);
    });

    it("Should distribute rewards on withdrawal", async function () {
      const depositAmount = ethers.utils.parseEther("50");

      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);

      // Mine some blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine");
      }

      const balanceBefore = await rewardToken.balanceOf(addr1.address);
      await masterChef.connect(addr1).withdraw(0, depositAmount);
      const balanceAfter = await rewardToken.balanceOf(addr1.address);

      expect(balanceAfter).to.be.gt(balanceBefore);
    });
  });

  describe("Emergency Withdraw", function () {
    it("Should allow emergency withdrawal without rewards", async function () {
      await masterChef.add(100, lpToken1.address, false);
      await lpToken1.transfer(addr1.address, ethers.utils.parseEther("100"));

      const depositAmount = ethers.utils.parseEther("50");
      await lpToken1.connect(addr1).approve(masterChef.address, depositAmount);
      await masterChef.connect(addr1).deposit(0, depositAmount);

      const lpBalanceBefore = await lpToken1.balanceOf(addr1.address);
      await masterChef.connect(addr1).emergencyWithdraw(0);
      const lpBalanceAfter = await lpToken1.balanceOf(addr1.address);

      expect(lpBalanceAfter.sub(lpBalanceBefore)).to.equal(depositAmount);

      const userInfo = await masterChef.userInfo(0, addr1.address);
      expect(userInfo.amount).to.equal(0);
    });
  });

  describe("Owner Functions", function () {
    it("Should allow owner to update reward per block", async function () {
      const newRewardPerBlock = ethers.utils.parseEther("20");
      await masterChef.updateRewardPerBlock(newRewardPerBlock);
      expect(await masterChef.rewardPerBlock()).to.equal(newRewardPerBlock);
    });

    it("Should not allow non-owner to update reward per block", async function () {
      const newRewardPerBlock = ethers.utils.parseEther("20");
      await expect(
        masterChef.connect(addr1).updateRewardPerBlock(newRewardPerBlock)
      ).to.be.revertedWith("MasterChef: caller is not the owner");
    });

    it("Should allow owner to transfer ownership", async function () {
      await masterChef.transferOwnership(addr1.address);
      expect(await masterChef.owner()).to.equal(addr1.address);
    });
  });
});
