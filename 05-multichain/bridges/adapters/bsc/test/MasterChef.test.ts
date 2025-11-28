import { expect } from "chai";
import { ethers } from "hardhat";
import { MasterChef, EtridToken } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("MasterChef", function () {
  let masterChef: MasterChef;
  let rewardToken: EtridToken;
  let lpToken: EtridToken; // Using ERC20 as mock LP token
  let owner: SignerWithAddress;
  let user1: SignerWithAddress;
  let user2: SignerWithAddress;

  const REWARD_PER_BLOCK = ethers.parseEther("10"); // 10 ÉTR per block
  const INITIAL_SUPPLY = ethers.parseEther("1000000"); // 1M ÉTR for rewards
  const LP_AMOUNT = ethers.parseEther("100"); // 100 LP tokens

  beforeEach(async function () {
    [owner, user1, user2] = await ethers.getSigners();

    // Deploy reward token (ÉTR)
    const EtridToken = await ethers.getContractFactory("EtridToken");
    rewardToken = await EtridToken.deploy("Etrid Coin", "ÉTR");
    await rewardToken.waitForDeployment();

    // Deploy mock LP token
    lpToken = await EtridToken.deploy("LP Token", "LP");
    await lpToken.waitForDeployment();

    // Get current block number
    const currentBlock = await ethers.provider.getBlockNumber();
    const startBlock = currentBlock + 10;

    // Deploy MasterChef
    const MasterChef = await ethers.getContractFactory("MasterChef");
    masterChef = await MasterChef.deploy(
      await rewardToken.getAddress(),
      REWARD_PER_BLOCK,
      startBlock
    );
    await masterChef.waitForDeployment();

    // Mint reward tokens to MasterChef
    const MINTER_ROLE = await rewardToken.MINTER_ROLE();
    const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_mint"));
    await rewardToken.bridgeMint(await masterChef.getAddress(), INITIAL_SUPPLY, txHash);

    // Mint LP tokens to users
    await lpToken.bridgeMint(user1.address, LP_AMOUNT, txHash);
    await lpToken.bridgeMint(user2.address, LP_AMOUNT, txHash);
  });

  describe("Deployment", function () {
    it("Should set the correct reward token", async function () {
      expect(await masterChef.rewardToken()).to.equal(await rewardToken.getAddress());
    });

    it("Should set the correct reward per block", async function () {
      expect(await masterChef.rewardPerBlock()).to.equal(REWARD_PER_BLOCK);
    });

    it("Should start with zero pools", async function () {
      expect(await masterChef.poolLength()).to.equal(0);
    });

    it("Should set the correct start block", async function () {
      const currentBlock = await ethers.provider.getBlockNumber();
      const startBlock = await masterChef.startBlock();
      expect(startBlock).to.be.greaterThan(currentBlock);
    });
  });

  describe("Pool Management", function () {
    it("Should allow owner to add pool", async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);

      expect(await masterChef.poolLength()).to.equal(1);

      const poolInfo = await masterChef.poolInfo(0);
      expect(poolInfo.lpToken).to.equal(await lpToken.getAddress());
      expect(poolInfo.allocPoint).to.equal(1000);
    });

    it("Should not allow non-owner to add pool", async function () {
      await expect(
        masterChef.connect(user1).add(1000, await lpToken.getAddress(), false)
      ).to.be.reverted;
    });

    it("Should not allow adding same LP token twice", async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);

      await expect(
        masterChef.add(500, await lpToken.getAddress(), false)
      ).to.be.revertedWith("MasterChef: LP token already added");
    });

    it("Should allow owner to update pool allocation", async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);
      await masterChef.set(0, 2000, false);

      const poolInfo = await masterChef.poolInfo(0);
      expect(poolInfo.allocPoint).to.equal(2000);
    });

    it("Should update total allocation points correctly", async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);
      expect(await masterChef.totalAllocPoint()).to.equal(1000);

      await masterChef.set(0, 2000, false);
      expect(await masterChef.totalAllocPoint()).to.equal(2000);
    });
  });

  describe("Deposits and Withdrawals", function () {
    beforeEach(async function () {
      // Add LP pool
      await masterChef.add(1000, await lpToken.getAddress(), false);

      // Approve MasterChef to spend LP tokens
      await lpToken.connect(user1).approve(await masterChef.getAddress(), ethers.MaxUint256);
      await lpToken.connect(user2).approve(await masterChef.getAddress(), ethers.MaxUint256);

      // Mine blocks to reach start block
      const startBlock = await masterChef.startBlock();
      const currentBlock = await ethers.provider.getBlockNumber();
      for (let i = 0; i < startBlock - currentBlock; i++) {
        await ethers.provider.send("evm_mine", []);
      }
    });

    it("Should allow users to deposit LP tokens", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      const userInfo = await masterChef.userInfo(0, user1.address);
      expect(userInfo.amount).to.equal(depositAmount);

      const poolInfo = await masterChef.poolInfo(0);
      expect(poolInfo.totalStaked).to.equal(depositAmount);
    });

    it("Should allow users to withdraw LP tokens", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);
      await masterChef.connect(user1).withdraw(0, depositAmount);

      const userInfo = await masterChef.userInfo(0, user1.address);
      expect(userInfo.amount).to.equal(0);
    });

    it("Should not allow withdrawing more than deposited", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      await expect(
        masterChef.connect(user1).withdraw(0, ethers.parseEther("20"))
      ).to.be.revertedWith("MasterChef: withdraw amount exceeds balance");
    });

    it("Should not allow deposits when paused", async function () {
      await masterChef.pause();

      await expect(
        masterChef.connect(user1).deposit(0, ethers.parseEther("10"))
      ).to.be.revertedWith("Pausable: paused");
    });

    it("Should allow withdrawals even when paused", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);
      await masterChef.pause();

      // Withdraw should still work when paused
      await expect(
        masterChef.connect(user1).withdraw(0, depositAmount)
      ).to.not.be.reverted;
    });
  });

  describe("Reward Distribution", function () {
    beforeEach(async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);
      await lpToken.connect(user1).approve(await masterChef.getAddress(), ethers.MaxUint256);

      const startBlock = await masterChef.startBlock();
      const currentBlock = await ethers.provider.getBlockNumber();
      for (let i = 0; i < startBlock - currentBlock; i++) {
        await ethers.provider.send("evm_mine", []);
      }
    });

    it("Should accumulate rewards over time", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      // Mine 10 blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const pending = await masterChef.pendingReward(0, user1.address);

      // Should have earned rewards (approximately 10 blocks * 10 ÉTR)
      expect(pending).to.be.greaterThan(0);
      expect(pending).to.be.closeTo(
        REWARD_PER_BLOCK * 10n,
        ethers.parseEther("50") // Allow some variance
      );
    });

    it("Should allow users to harvest rewards", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      // Mine blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const balanceBefore = await rewardToken.balanceOf(user1.address);
      await masterChef.connect(user1).harvest(0);
      const balanceAfter = await rewardToken.balanceOf(user1.address);

      expect(balanceAfter).to.be.greaterThan(balanceBefore);
    });

    it("Should split rewards proportionally between users", async function () {
      const depositAmount = ethers.parseEther("10");

      // Both users deposit same amount
      await lpToken.connect(user2).approve(await masterChef.getAddress(), ethers.MaxUint256);
      await masterChef.connect(user1).deposit(0, depositAmount);
      await masterChef.connect(user2).deposit(0, depositAmount);

      // Mine blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const pending1 = await masterChef.pendingReward(0, user1.address);
      const pending2 = await masterChef.pendingReward(0, user2.address);

      // Rewards should be approximately equal (allowing for rounding)
      expect(pending1).to.be.closeTo(pending2, ethers.parseEther("5"));
    });

    it("Should auto-harvest on deposit", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      // Mine blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const pendingBefore = await masterChef.pendingReward(0, user1.address);

      // Second deposit should auto-harvest
      await masterChef.connect(user1).deposit(0, depositAmount);

      const userInfo = await masterChef.userInfo(0, user1.address);
      // Pending rewards should be accumulated
      expect(userInfo.pendingRewards).to.be.greaterThan(0);
    });

    it("Should auto-harvest on withdraw", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      // Mine blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const userInfo = await masterChef.userInfo(0, user1.address);
      await masterChef.connect(user1).withdraw(0, depositAmount / 2n);

      const userInfoAfter = await masterChef.userInfo(0, user1.address);
      expect(userInfoAfter.pendingRewards).to.be.greaterThan(0);
    });
  });

  describe("Emergency Withdraw", function () {
    beforeEach(async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);
      await lpToken.connect(user1).approve(await masterChef.getAddress(), ethers.MaxUint256);

      const startBlock = await masterChef.startBlock();
      const currentBlock = await ethers.provider.getBlockNumber();
      for (let i = 0; i < startBlock - currentBlock; i++) {
        await ethers.provider.send("evm_mine", []);
      }
    });

    it("Should allow emergency withdraw", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      const lpBalanceBefore = await lpToken.balanceOf(user1.address);
      await masterChef.connect(user1).emergencyWithdraw(0);
      const lpBalanceAfter = await lpToken.balanceOf(user1.address);

      expect(lpBalanceAfter - lpBalanceBefore).to.equal(depositAmount);

      const userInfo = await masterChef.userInfo(0, user1.address);
      expect(userInfo.amount).to.equal(0);
      expect(userInfo.pendingRewards).to.equal(0);
    });

    it("Should forfeit rewards on emergency withdraw", async function () {
      const depositAmount = ethers.parseEther("10");

      await masterChef.connect(user1).deposit(0, depositAmount);

      // Mine blocks to accumulate rewards
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      const rewardBalanceBefore = await rewardToken.balanceOf(user1.address);
      await masterChef.connect(user1).emergencyWithdraw(0);
      const rewardBalanceAfter = await rewardToken.balanceOf(user1.address);

      // No rewards should be received
      expect(rewardBalanceAfter).to.equal(rewardBalanceBefore);
    });
  });

  describe("Emission Rate Updates", function () {
    it("Should allow owner to update emission rate", async function () {
      const newRate = ethers.parseEther("20");

      await masterChef.updateRewardPerBlock(newRate);

      expect(await masterChef.rewardPerBlock()).to.equal(newRate);
    });

    it("Should not allow non-owner to update emission rate", async function () {
      const newRate = ethers.parseEther("20");

      await expect(
        masterChef.connect(user1).updateRewardPerBlock(newRate)
      ).to.be.reverted;
    });

    it("Should update all pools before changing rate", async function () {
      await masterChef.add(1000, await lpToken.getAddress(), false);
      await lpToken.connect(user1).approve(await masterChef.getAddress(), ethers.MaxUint256);

      const startBlock = await masterChef.startBlock();
      const currentBlock = await ethers.provider.getBlockNumber();
      for (let i = 0; i < startBlock - currentBlock; i++) {
        await ethers.provider.send("evm_mine", []);
      }

      await masterChef.connect(user1).deposit(0, ethers.parseEther("10"));

      const poolInfoBefore = await masterChef.poolInfo(0);

      await masterChef.updateRewardPerBlock(ethers.parseEther("20"));

      const poolInfoAfter = await masterChef.poolInfo(0);

      // lastRewardBlock should be updated
      expect(poolInfoAfter.lastRewardBlock).to.be.greaterThan(poolInfoBefore.lastRewardBlock);
    });
  });

  describe("Pausable", function () {
    it("Should allow owner to pause", async function () {
      await masterChef.pause();
      expect(await masterChef.paused()).to.be.true;
    });

    it("Should allow owner to unpause", async function () {
      await masterChef.pause();
      await masterChef.unpause();
      expect(await masterChef.paused()).to.be.false;
    });

    it("Should not allow non-owner to pause", async function () {
      await expect(masterChef.connect(user1).pause()).to.be.reverted;
    });
  });
});
