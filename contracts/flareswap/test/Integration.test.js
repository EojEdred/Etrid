// Integration Test Suite - Full User Journey
// Tests: Wallet → DEX (Swap, Add Liquidity) → Staking

const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Integration Tests - Full User Journey", function () {
  let factory, router, weth, masterChef, etrToken;
  let tokenA, tokenB;
  let owner, alice, bob;
  let pairAB;

  before(async function () {
    [owner, alice, bob] = await ethers.getSigners();

    // Deploy core DEX contracts
    const WETH = await ethers.getContractFactory("WETH");
    weth = await WETH.deploy();

    const Factory = await ethers.getContractFactory("FlareSwapFactory");
    factory = await Factory.deploy(owner.address);

    const Router = await ethers.getContractFactory("FlareSwapRouter");
    router = await Router.deploy(factory.address, weth.address);

    // Deploy test tokens
    const Token = await ethers.getContractFactory("MockERC20");
    etrToken = await Token.deploy("Etrid Token", "ETR", ethers.utils.parseEther("1000000"));
    tokenA = await Token.deploy("Token A", "TKA", ethers.utils.parseEther("100000"));
    tokenB = await Token.deploy("Token B", "TKB", ethers.utils.parseEther("100000"));

    // Deploy MasterChef
    const currentBlock = await ethers.provider.getBlockNumber();
    const MasterChef = await ethers.getContractFactory("MasterChef");
    masterChef = await MasterChef.deploy(
      etrToken.address,
      ethers.utils.parseEther("1"), // 1 ETR per block
      currentBlock + 1
    );

    // Fund MasterChef with rewards
    await etrToken.transfer(masterChef.address, ethers.utils.parseEther("100000"));

    // Create trading pair
    await factory.createPair(tokenA.address, tokenB.address);
    pairAB = await factory.getPair(tokenA.address, tokenB.address);

    // Add staking pool for the pair
    await masterChef.add(100, pairAB, false);

    // Distribute tokens to users
    await tokenA.transfer(alice.address, ethers.utils.parseEther("1000"));
    await tokenB.transfer(alice.address, ethers.utils.parseEther("1000"));
    await tokenA.transfer(bob.address, ethers.utils.parseEther("1000"));
    await tokenB.transfer(bob.address, ethers.utils.parseEther("1000"));
  });

  describe("Scenario 1: Complete User Journey", function () {
    it("Step 1: Alice adds liquidity to the pool", async function () {
      const amountA = ethers.utils.parseEther("100");
      const amountB = ethers.utils.parseEther("100");

      await tokenA.connect(alice).approve(router.address, amountA);
      await tokenB.connect(alice).approve(router.address, amountB);

      const tx = await router.connect(alice).addLiquidity(
        tokenA.address,
        tokenB.address,
        amountA,
        amountB,
        0,
        0,
        alice.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const pair = await ethers.getContractAt("FlareSwapPair", pairAB);
      const lpBalance = await pair.balanceOf(alice.address);
      expect(lpBalance).to.be.gt(0);
      console.log("      ✓ Alice received LP tokens:", ethers.utils.formatEther(lpBalance));
    });

    it("Step 2: Alice stakes LP tokens in MasterChef", async function () {
      const pair = await ethers.getContractAt("FlareSwapPair", pairAB);
      const lpBalance = await pair.balanceOf(alice.address);

      await pair.connect(alice).approve(masterChef.address, lpBalance);
      await masterChef.connect(alice).deposit(0, lpBalance);

      const userInfo = await masterChef.userInfo(0, alice.address);
      expect(userInfo.amount).to.equal(lpBalance);
      console.log("      ✓ Alice staked LP tokens:", ethers.utils.formatEther(lpBalance));
    });

    it("Step 3: Bob swaps tokens", async function () {
      const amountIn = ethers.utils.parseEther("10");
      await tokenA.connect(bob).approve(router.address, amountIn);

      const path = [tokenA.address, tokenB.address];
      const amountsBefore = await router.getAmountsOut(amountIn, path);

      await router.connect(bob).swapExactTokensForTokens(
        amountIn,
        0,
        path,
        bob.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const bobBalanceB = await tokenB.balanceOf(bob.address);
      expect(bobBalanceB).to.be.gt(ethers.utils.parseEther("1000"));
      console.log("      ✓ Bob swapped 10 TKA for", ethers.utils.formatEther(bobBalanceB.sub(ethers.utils.parseEther("1000"))), "TKB");
    });

    it("Step 4: Alice accumulates staking rewards", async function () {
      // Mine some blocks
      for (let i = 0; i < 10; i++) {
        await ethers.provider.send("evm_mine");
      }

      const pendingRewards = await masterChef.pendingReward(0, alice.address);
      expect(pendingRewards).to.be.gt(0);
      console.log("      ✓ Alice has pending rewards:", ethers.utils.formatEther(pendingRewards), "ETR");
    });

    it("Step 5: Alice withdraws LP tokens and claims rewards", async function () {
      const userInfoBefore = await masterChef.userInfo(0, alice.address);
      const etrBalanceBefore = await etrToken.balanceOf(alice.address);

      await masterChef.connect(alice).withdraw(0, userInfoBefore.amount);

      const etrBalanceAfter = await etrToken.balanceOf(alice.address);
      const rewards = etrBalanceAfter.sub(etrBalanceBefore);

      expect(rewards).to.be.gt(0);
      console.log("      ✓ Alice withdrew LP tokens and received", ethers.utils.formatEther(rewards), "ETR rewards");
    });

    it("Step 6: Alice removes liquidity from the pool", async function () {
      const pair = await ethers.getContractAt("FlareSwapPair", pairAB);
      const lpBalance = await pair.balanceOf(alice.address);

      await pair.connect(alice).approve(router.address, lpBalance);

      const tokenABefore = await tokenA.balanceOf(alice.address);
      const tokenBBefore = await tokenB.balanceOf(alice.address);

      await router.connect(alice).removeLiquidity(
        tokenA.address,
        tokenB.address,
        lpBalance,
        0,
        0,
        alice.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const tokenAAfter = await tokenA.balanceOf(alice.address);
      const tokenBAfter = await tokenB.balanceOf(alice.address);

      expect(tokenAAfter).to.be.gt(tokenABefore);
      expect(tokenBAfter).to.be.gt(tokenBBefore);
      console.log("      ✓ Alice removed liquidity and received tokens back");
    });
  });

  describe("Scenario 2: Multi-User Staking Competition", function () {
    let pairCD, tokenC, tokenD;

    before(async function () {
      // Deploy new tokens
      const Token = await ethers.getContractFactory("MockERC20");
      tokenC = await Token.deploy("Token C", "TKC", ethers.utils.parseEther("100000"));
      tokenD = await Token.deploy("Token D", "TKD", ethers.utils.parseEther("100000"));

      // Create pair
      await factory.createPair(tokenC.address, tokenD.address);
      pairCD = await factory.getPair(tokenC.address, tokenD.address);

      // Add staking pool
      await masterChef.add(200, pairCD, false);

      // Distribute tokens
      await tokenC.transfer(alice.address, ethers.utils.parseEther("1000"));
      await tokenD.transfer(alice.address, ethers.utils.parseEther("1000"));
      await tokenC.transfer(bob.address, ethers.utils.parseEther("1000"));
      await tokenD.transfer(bob.address, ethers.utils.parseEther("1000"));
    });

    it("Alice and Bob both add liquidity", async function () {
      // Alice adds liquidity
      await tokenC.connect(alice).approve(router.address, ethers.utils.parseEther("100"));
      await tokenD.connect(alice).approve(router.address, ethers.utils.parseEther("100"));
      await router.connect(alice).addLiquidity(
        tokenC.address,
        tokenD.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        alice.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Bob adds liquidity
      await tokenC.connect(bob).approve(router.address, ethers.utils.parseEther("100"));
      await tokenD.connect(bob).approve(router.address, ethers.utils.parseEther("100"));
      await router.connect(bob).addLiquidity(
        tokenC.address,
        tokenD.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        bob.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const pair = await ethers.getContractAt("FlareSwapPair", pairCD);
      const aliceLp = await pair.balanceOf(alice.address);
      const bobLp = await pair.balanceOf(bob.address);

      expect(aliceLp).to.be.gt(0);
      expect(bobLp).to.be.gt(0);
    });

    it("Both users stake their LP tokens", async function () {
      const pair = await ethers.getContractAt("FlareSwapPair", pairCD);

      // Alice stakes
      const aliceLp = await pair.balanceOf(alice.address);
      await pair.connect(alice).approve(masterChef.address, aliceLp);
      await masterChef.connect(alice).deposit(1, aliceLp);

      // Bob stakes
      const bobLp = await pair.balanceOf(bob.address);
      await pair.connect(bob).approve(masterChef.address, bobLp);
      await masterChef.connect(bob).deposit(1, bobLp);

      const aliceInfo = await masterChef.userInfo(1, alice.address);
      const bobInfo = await masterChef.userInfo(1, bob.address);

      expect(aliceInfo.amount).to.equal(aliceLp);
      expect(bobInfo.amount).to.equal(bobLp);
    });

    it("Rewards are distributed proportionally", async function () {
      // Mine blocks
      for (let i = 0; i < 20; i++) {
        await ethers.provider.send("evm_mine");
      }

      const aliceRewards = await masterChef.pendingReward(1, alice.address);
      const bobRewards = await masterChef.pendingReward(1, bob.address);

      expect(aliceRewards).to.be.gt(0);
      expect(bobRewards).to.be.gt(0);

      // Since they staked equal amounts, rewards should be approximately equal
      // Note: Due to block timing, there might be slight variations
      const ratio = aliceRewards.mul(100).div(bobRewards);
      expect(ratio).to.be.gte(80).and.lte(125); // Within 25% of each other (accounting for block timing)
    });
  });

  describe("Scenario 3: ETH Trading and Staking", function () {
    let etrWethPair;

    before(async function () {
      // Create ETR/WETH pair
      await factory.createPair(etrToken.address, weth.address);
      etrWethPair = await factory.getPair(etrToken.address, weth.address);

      // Add staking pool
      await masterChef.add(300, etrWethPair, false);

      // Transfer ETR to Alice
      await etrToken.transfer(alice.address, ethers.utils.parseEther("1000"));
    });

    it("Alice adds liquidity with ETH", async function () {
      const amountToken = ethers.utils.parseEther("100");
      const amountETH = ethers.utils.parseEther("1");

      await etrToken.connect(alice).approve(router.address, amountToken);

      await router.connect(alice).addLiquidityETH(
        etrToken.address,
        amountToken,
        0, 0,
        alice.address,
        Math.floor(Date.now() / 1000) + 3600,
        { value: amountETH }
      );

      const pair = await ethers.getContractAt("FlareSwapPair", etrWethPair);
      const lpBalance = await pair.balanceOf(alice.address);
      expect(lpBalance).to.be.gt(0);
    });

    it("Alice stakes ETR/WETH LP tokens", async function () {
      const pair = await ethers.getContractAt("FlareSwapPair", etrWethPair);
      const lpBalance = await pair.balanceOf(alice.address);

      await pair.connect(alice).approve(masterChef.address, lpBalance);
      await masterChef.connect(alice).deposit(2, lpBalance);

      const userInfo = await masterChef.userInfo(2, alice.address);
      expect(userInfo.amount).to.equal(lpBalance);
    });

    it("Bob swaps ETH for ETR", async function () {
      const amountETH = ethers.utils.parseEther("0.1");
      const path = [weth.address, etrToken.address];

      const etrBalanceBefore = await etrToken.balanceOf(bob.address);

      await router.connect(bob).swapExactETHForTokens(
        0,
        path,
        bob.address,
        Math.floor(Date.now() / 1000) + 3600,
        { value: amountETH }
      );

      const etrBalanceAfter = await etrToken.balanceOf(bob.address);
      expect(etrBalanceAfter).to.be.gt(etrBalanceBefore);
    });
  });

  describe("Scenario 4: Emergency Situations", function () {
    let emergencyPair, emergencyTokenA, emergencyTokenB;

    before(async function () {
      // Create emergency test pair
      const Token = await ethers.getContractFactory("MockERC20");
      emergencyTokenA = await Token.deploy("Emergency A", "EMA", ethers.utils.parseEther("10000"));
      emergencyTokenB = await Token.deploy("Emergency B", "EMB", ethers.utils.parseEther("10000"));

      await factory.createPair(emergencyTokenA.address, emergencyTokenB.address);
      emergencyPair = await factory.getPair(emergencyTokenA.address, emergencyTokenB.address);

      await masterChef.add(50, emergencyPair, false);

      // Setup Alice's position
      await emergencyTokenA.transfer(alice.address, ethers.utils.parseEther("200"));
      await emergencyTokenB.transfer(alice.address, ethers.utils.parseEther("200"));

      await emergencyTokenA.connect(alice).approve(router.address, ethers.utils.parseEther("100"));
      await emergencyTokenB.connect(alice).approve(router.address, ethers.utils.parseEther("100"));

      await router.connect(alice).addLiquidity(
        emergencyTokenA.address,
        emergencyTokenB.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        alice.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const pair = await ethers.getContractAt("FlareSwapPair", emergencyPair);
      const lpBalance = await pair.balanceOf(alice.address);
      await pair.connect(alice).approve(masterChef.address, lpBalance);
      await masterChef.connect(alice).deposit(3, lpBalance);
    });

    it("Alice can emergency withdraw without rewards", async function () {
      const userInfoBefore = await masterChef.userInfo(3, alice.address);
      const pair = await ethers.getContractAt("FlareSwapPair", emergencyPair);
      const lpBalanceBefore = await pair.balanceOf(alice.address);

      await masterChef.connect(alice).emergencyWithdraw(3);

      const lpBalanceAfter = await pair.balanceOf(alice.address);
      const userInfoAfter = await masterChef.userInfo(3, alice.address);

      expect(lpBalanceAfter.sub(lpBalanceBefore)).to.equal(userInfoBefore.amount);
      expect(userInfoAfter.amount).to.equal(0);
    });
  });
});
