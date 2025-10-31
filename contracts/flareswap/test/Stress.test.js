// Stress Test Suite
// Tests: High volume, large pools, edge cases, concurrent operations

const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Stress Tests", function () {
  let factory, router, weth, masterChef, rewardToken;
  let owner, users;

  // Increase timeout for stress tests
  this.timeout(120000);

  before(async function () {
    const signers = await ethers.getSigners();
    owner = signers[0];
    users = signers.slice(1, 11); // 10 test users

    // Deploy core contracts
    const WETH = await ethers.getContractFactory("WETH");
    weth = await WETH.deploy();

    const Factory = await ethers.getContractFactory("FlareSwapFactory");
    factory = await Factory.deploy(owner.address);

    const Router = await ethers.getContractFactory("FlareSwapRouter");
    router = await Router.deploy(factory.address, weth.address);

    const Token = await ethers.getContractFactory("MockERC20");
    rewardToken = await Token.deploy("Reward", "RWD", ethers.utils.parseEther("100000000"));

    const currentBlock = await ethers.provider.getBlockNumber();
    const MasterChef = await ethers.getContractFactory("MasterChef");
    masterChef = await MasterChef.deploy(
      rewardToken.address,
      ethers.utils.parseEther("10"),
      currentBlock + 1
    );

    await rewardToken.transfer(masterChef.address, ethers.utils.parseEther("50000000"));
  });

  describe("High Volume Trading", function () {
    let tokenA, tokenB, pair;

    before(async function () {
      const Token = await ethers.getContractFactory("MockERC20");
      tokenA = await Token.deploy("Token A", "TKA", ethers.utils.parseEther("10000000"));
      tokenB = await Token.deploy("Token B", "TKB", ethers.utils.parseEther("10000000"));

      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      pair = await ethers.getContractAt("FlareSwapPair", pairAddress);

      // Add large initial liquidity
      await tokenA.approve(router.address, ethers.utils.parseEther("1000000"));
      await tokenB.approve(router.address, ethers.utils.parseEther("1000000"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("1000000"),
        ethers.utils.parseEther("1000000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });

    it("Should handle 100 consecutive swaps", async function () {
      const swapAmount = ethers.utils.parseEther("100");

      for (let i = 0; i < users.length; i++) {
        await tokenA.transfer(users[i].address, swapAmount.mul(10));
      }

      let successfulSwaps = 0;

      for (let i = 0; i < 100; i++) {
        const user = users[i % users.length];
        await tokenA.connect(user).approve(router.address, swapAmount);

        await router.connect(user).swapExactTokensForTokens(
          swapAmount,
          0,
          [tokenA.address, tokenB.address],
          user.address,
          Math.floor(Date.now() / 1000) + 3600
        );

        successfulSwaps++;
      }

      expect(successfulSwaps).to.equal(100);
      console.log("      ✓ Completed 100 swaps successfully");
    });

    it("Should handle multiple simultaneous liquidity operations", async function () {
      const promises = [];

      for (let i = 0; i < 5; i++) {
        const user = users[i];
        const amount = ethers.utils.parseEther("1000");

        await tokenA.transfer(user.address, amount);
        await tokenB.transfer(user.address, amount);
        await tokenA.connect(user).approve(router.address, amount);
        await tokenB.connect(user).approve(router.address, amount);

        promises.push(
          router.connect(user).addLiquidity(
            tokenA.address,
            tokenB.address,
            amount,
            amount,
            0, 0,
            user.address,
            Math.floor(Date.now() / 1000) + 3600
          )
        );
      }

      await Promise.all(promises);

      const totalSupply = await pair.totalSupply();
      expect(totalSupply).to.be.gt(ethers.utils.parseEther("1000000"));
      console.log("      ✓ 5 users added liquidity simultaneously");
    });
  });

  describe("Large Pool Operations", function () {
    let tokenX, tokenY, pair;

    before(async function () {
      const Token = await ethers.getContractFactory("MockERC20");
      tokenX = await Token.deploy("Token X", "TKX", ethers.utils.parseEther("1000000000")); // 1B tokens
      tokenY = await Token.deploy("Token Y", "TKY", ethers.utils.parseEther("1000000000"));

      await factory.createPair(tokenX.address, tokenY.address);
      const pairAddress = await factory.getPair(tokenX.address, tokenY.address);
      pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
    });

    it("Should handle very large liquidity addition", async function () {
      const largeAmount = ethers.utils.parseEther("10000000"); // 10M tokens

      await tokenX.approve(router.address, largeAmount);
      await tokenY.approve(router.address, largeAmount);

      await router.addLiquidity(
        tokenX.address,
        tokenY.address,
        largeAmount,
        largeAmount,
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const [reserve0, reserve1] = await pair.getReserves();
      expect(reserve0).to.equal(largeAmount);
      expect(reserve1).to.equal(largeAmount);
      console.log("      ✓ Added", ethers.utils.formatEther(largeAmount), "tokens liquidity");
    });

    it("Should handle very large swap", async function () {
      const largeSwap = ethers.utils.parseEther("100000"); // 100K tokens

      await tokenX.approve(router.address, largeSwap);

      const balanceBefore = await tokenY.balanceOf(owner.address);

      await router.swapExactTokensForTokens(
        largeSwap,
        0,
        [tokenX.address, tokenY.address],
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const balanceAfter = await tokenY.balanceOf(owner.address);
      const received = balanceAfter.sub(balanceBefore);

      expect(received).to.be.gt(0);
      console.log("      ✓ Swapped", ethers.utils.formatEther(largeSwap), "tokens");
    });
  });

  describe("MasterChef Stress Tests", function () {
    let lpToken;

    before(async function () {
      const Token = await ethers.getContractFactory("MockERC20");
      const tokenM = await Token.deploy("Token M", "TKM", ethers.utils.parseEther("10000000"));
      const tokenN = await Token.deploy("Token N", "TKN", ethers.utils.parseEther("10000000"));

      await factory.createPair(tokenM.address, tokenN.address);
      const pairAddress = await factory.getPair(tokenM.address, tokenN.address);
      lpToken = await ethers.getContractAt("FlareSwapPair", pairAddress);

      // Add pool to MasterChef
      await masterChef.add(1000, lpToken.address, false);

      // Create LP tokens for users
      await tokenM.approve(router.address, ethers.utils.parseEther("1000000"));
      await tokenN.approve(router.address, ethers.utils.parseEther("1000000"));

      await router.addLiquidity(
        tokenM.address,
        tokenN.address,
        ethers.utils.parseEther("1000000"),
        ethers.utils.parseEther("1000000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Distribute LP tokens to users
      const lpBalance = await lpToken.balanceOf(owner.address);
      const amountPerUser = lpBalance.div(users.length);

      for (const user of users) {
        await lpToken.transfer(user.address, amountPerUser);
      }
    });

    it("Should handle 10 users staking simultaneously", async function () {
      for (const user of users) {
        const balance = await lpToken.balanceOf(user.address);
        await lpToken.connect(user).approve(masterChef.address, balance);
        await masterChef.connect(user).deposit(0, balance);
      }

      // Verify all users have staked
      for (const user of users) {
        const userInfo = await masterChef.userInfo(0, user.address);
        expect(userInfo.amount).to.be.gt(0);
      }

      console.log("      ✓ 10 users staked LP tokens successfully");
    });

    it("Should correctly distribute rewards to multiple stakers", async function () {
      // Mine some blocks
      for (let i = 0; i < 50; i++) {
        await ethers.provider.send("evm_mine");
      }

      const rewards = [];
      for (const user of users) {
        const pending = await masterChef.pendingReward(0, user.address);
        rewards.push(pending);
      }

      // All users should have similar rewards (since they staked equal amounts)
      const avgReward = rewards.reduce((a, b) => a.add(b), ethers.BigNumber.from(0)).div(rewards.length);

      for (const reward of rewards) {
        // Each user's reward should be within 60% of average (accounting for timing and order)
        const ratio = reward.mul(100).div(avgReward);
        expect(ratio).to.be.gte(40).and.lte(160);
      }

      console.log("      ✓ Rewards distributed proportionally to all users");
    });

    it("Should handle rapid deposit/withdraw cycles", async function () {
      const user = users[0];
      const userInfo = await masterChef.userInfo(0, user.address);
      const halfAmount = userInfo.amount.div(2);

      for (let i = 0; i < 10; i++) {
        await masterChef.connect(user).withdraw(0, halfAmount);
        await lpToken.connect(user).approve(masterChef.address, halfAmount);
        await masterChef.connect(user).deposit(0, halfAmount);
      }

      const finalInfo = await masterChef.userInfo(0, user.address);
      expect(finalInfo.amount).to.be.gte(halfAmount);
      console.log("      ✓ Completed 10 deposit/withdraw cycles");
    });
  });

  describe("Edge Cases and Limits", function () {
    it("Should handle small amount swaps", async function () {
      const Token = await ethers.getContractFactory("MockERC20");
      const token1 = await Token.deploy("Token 1", "TK1", ethers.utils.parseEther("1000000"));
      const token2 = await Token.deploy("Token 2", "TK2", ethers.utils.parseEther("1000000"));

      await factory.createPair(token1.address, token2.address);

      await token1.approve(router.address, ethers.utils.parseEther("10000"));
      await token2.approve(router.address, ethers.utils.parseEther("10000"));

      await router.addLiquidity(
        token1.address,
        token2.address,
        ethers.utils.parseEther("10000"),
        ethers.utils.parseEther("10000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Try swapping a very small amount (1000 wei = 0.000000000000001 tokens)
      const smallAmount = ethers.BigNumber.from("1000");
      await token1.approve(router.address, smallAmount);
      await router.swapExactTokensForTokens(
        smallAmount,
        0,
        [token1.address, token2.address],
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      console.log("      ✓ Successfully swapped small amount (1000 wei)");
    });

    it("Should handle many pools in MasterChef", async function () {
      const Token = await ethers.getContractFactory("MockERC20");

      // Add 20 pools
      for (let i = 0; i < 20; i++) {
        const tokenP1 = await Token.deploy(`Pool Token ${i}A`, `PT${i}A`, ethers.utils.parseEther("1000000"));
        const tokenP2 = await Token.deploy(`Pool Token ${i}B`, `PT${i}B`, ethers.utils.parseEther("1000000"));

        await factory.createPair(tokenP1.address, tokenP2.address);
        const pairAddress = await factory.getPair(tokenP1.address, tokenP2.address);

        await masterChef.add(100, pairAddress, false);
      }

      const poolLength = await masterChef.poolLength();
      expect(poolLength).to.be.gte(21); // 1 original + 20 new

      console.log("      ✓ Successfully created and added 20 pools");
    });

    it("Should handle massUpdatePools with many pools", async function () {
      await masterChef.massUpdatePools();
      console.log("      ✓ massUpdatePools executed successfully");
    });
  });

  describe("Gas Optimization Verification", function () {
    let tokenG1, tokenG2;

    before(async function () {
      const Token = await ethers.getContractFactory("MockERC20");
      tokenG1 = await Token.deploy("Gas Token 1", "GT1", ethers.utils.parseEther("1000000"));
      tokenG2 = await Token.deploy("Gas Token 2", "GT2", ethers.utils.parseEther("1000000"));

      await factory.createPair(tokenG1.address, tokenG2.address);

      await tokenG1.approve(router.address, ethers.utils.parseEther("100000"));
      await tokenG2.approve(router.address, ethers.utils.parseEther("100000"));

      await router.addLiquidity(
        tokenG1.address,
        tokenG2.address,
        ethers.utils.parseEther("100000"),
        ethers.utils.parseEther("100000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });

    it("Should execute swap with reasonable gas", async function () {
      await tokenG1.approve(router.address, ethers.utils.parseEther("100"));

      const tx = await router.swapExactTokensForTokens(
        ethers.utils.parseEther("100"),
        0,
        [tokenG1.address, tokenG2.address],
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const receipt = await tx.wait();
      console.log("      ✓ Swap gas used:", receipt.gasUsed.toString());
      expect(receipt.gasUsed.toNumber()).to.be.lt(200000); // Should be less than 200k gas
    });

    it("Should execute addLiquidity with reasonable gas", async function () {
      await tokenG1.approve(router.address, ethers.utils.parseEther("1000"));
      await tokenG2.approve(router.address, ethers.utils.parseEther("1000"));

      const tx = await router.addLiquidity(
        tokenG1.address,
        tokenG2.address,
        ethers.utils.parseEther("1000"),
        ethers.utils.parseEther("1000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const receipt = await tx.wait();
      console.log("      ✓ Add liquidity gas used:", receipt.gasUsed.toString());
      expect(receipt.gasUsed.toNumber()).to.be.lt(300000); // Should be less than 300k gas
    });
  });
});
