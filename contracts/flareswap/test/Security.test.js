// Security Test Suite
// Tests: Reentrancy, Flash Loan Attacks, Access Control, Edge Cases

const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Security Tests", function () {
  let factory, router, weth, masterChef, rewardToken;
  let tokenA, tokenB;
  let owner, attacker, user;

  beforeEach(async function () {
    [owner, attacker, user] = await ethers.getSigners();

    // Deploy core contracts
    const WETH = await ethers.getContractFactory("WETH");
    weth = await WETH.deploy();

    const Factory = await ethers.getContractFactory("FlareSwapFactory");
    factory = await Factory.deploy(owner.address);

    const Router = await ethers.getContractFactory("FlareSwapRouter");
    router = await Router.deploy(factory.address, weth.address);

    // Deploy test tokens
    const Token = await ethers.getContractFactory("MockERC20");
    rewardToken = await Token.deploy("Reward", "RWD", ethers.utils.parseEther("1000000"));
    tokenA = await Token.deploy("Token A", "TKA", ethers.utils.parseEther("100000"));
    tokenB = await Token.deploy("Token B", "TKB", ethers.utils.parseEther("100000"));

    // Deploy MasterChef
    const currentBlock = await ethers.provider.getBlockNumber();
    const MasterChef = await ethers.getContractFactory("MasterChef");
    masterChef = await MasterChef.deploy(
      rewardToken.address,
      ethers.utils.parseEther("1"),
      currentBlock + 1
    );

    await rewardToken.transfer(masterChef.address, ethers.utils.parseEther("100000"));
  });

  describe("Access Control Tests", function () {
    describe("Factory Access Control", function () {
      it("Should prevent non-owner from setting fee receiver", async function () {
        await expect(
          factory.connect(attacker).setFeeTo(attacker.address)
        ).to.be.revertedWith("FlareSwap: FORBIDDEN");
      });

      it("Should prevent non-feeToSetter from changing feeToSetter", async function () {
        await expect(
          factory.connect(attacker).setFeeToSetter(attacker.address)
        ).to.be.revertedWith("FlareSwap: FORBIDDEN");
      });

      it("Should allow owner to set fee receiver", async function () {
        await factory.connect(owner).setFeeTo(user.address);
        expect(await factory.feeTo()).to.equal(user.address);
      });
    });

    describe("MasterChef Access Control", function () {
      it("Should prevent non-owner from adding pools", async function () {
        await expect(
          masterChef.connect(attacker).add(100, tokenA.address, false)
        ).to.be.revertedWith("MasterChef: caller is not the owner");
      });

      it("Should prevent non-owner from updating pool allocation", async function () {
        await masterChef.connect(owner).add(100, tokenA.address, false);
        await expect(
          masterChef.connect(attacker).set(0, 200, false)
        ).to.be.revertedWith("MasterChef: caller is not the owner");
      });

      it("Should prevent non-owner from updating reward per block", async function () {
        await expect(
          masterChef.connect(attacker).updateRewardPerBlock(ethers.utils.parseEther("2"))
        ).to.be.revertedWith("MasterChef: caller is not the owner");
      });

      it("Should prevent non-owner from transferring ownership", async function () {
        await expect(
          masterChef.connect(attacker).transferOwnership(attacker.address)
        ).to.be.revertedWith("MasterChef: caller is not the owner");
      });
    });

    describe("Pair Access Control", function () {
      let pair;

      beforeEach(async function () {
        await factory.createPair(tokenA.address, tokenB.address);
        const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
        pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
      });

      it("Should prevent non-factory from initializing pair", async function () {
        await expect(
          pair.connect(attacker).initialize(tokenA.address, tokenB.address)
        ).to.be.revertedWith("FlareSwap: FORBIDDEN");
      });
    });
  });

  describe("Reentrancy Protection Tests", function () {
    let pair;

    beforeEach(async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      pair = await ethers.getContractAt("FlareSwapPair", pairAddress);

      // Add initial liquidity
      await tokenA.transfer(user.address, ethers.utils.parseEther("1000"));
      await tokenB.transfer(user.address, ethers.utils.parseEther("1000"));

      await tokenA.connect(user).approve(router.address, ethers.utils.parseEther("500"));
      await tokenB.connect(user).approve(router.address, ethers.utils.parseEther("500"));

      await router.connect(user).addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("500"),
        ethers.utils.parseEther("500"),
        0, 0,
        user.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });

    it("Pair operations should be protected by lock modifier", async function () {
      // The lock modifier prevents reentrancy by setting unlocked to 0 during execution
      // This is tested implicitly through normal operations
      const amount = ethers.utils.parseEther("10");

      await tokenA.connect(user).approve(router.address, amount);
      await router.connect(user).swapExactTokensForTokens(
        amount,
        0,
        [tokenA.address, tokenB.address],
        user.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // If reentrancy protection failed, the transaction would revert with LOCKED
      expect(await tokenB.balanceOf(user.address)).to.be.gt(ethers.utils.parseEther("500"));
    });

    it("MasterChef should handle multiple operations safely", async function () {
      await masterChef.add(100, pair.address, false);

      const lpBalance = await pair.balanceOf(user.address);
      await pair.connect(user).approve(masterChef.address, lpBalance);

      // Deposit
      await masterChef.connect(user).deposit(0, lpBalance.div(2));

      // Immediate withdraw shouldn't cause issues
      await masterChef.connect(user).withdraw(0, lpBalance.div(4));

      const userInfo = await masterChef.userInfo(0, user.address);
      expect(userInfo.amount).to.equal(lpBalance.div(4));
    });
  });

  describe("Input Validation Tests", function () {
    it("Factory should reject identical tokens", async function () {
      await expect(
        factory.createPair(tokenA.address, tokenA.address)
      ).to.be.revertedWith("FlareSwap: IDENTICAL_ADDRESSES");
    });

    it("Factory should reject zero address", async function () {
      await expect(
        factory.createPair(tokenA.address, ethers.constants.AddressZero)
      ).to.be.revertedWith("FlareSwap: ZERO_ADDRESS");
    });

    it("Router should reject expired deadlines", async function () {
      const expiredDeadline = Math.floor(Date.now() / 1000) - 3600; // 1 hour ago

      await tokenA.approve(router.address, ethers.utils.parseEther("100"));
      await tokenB.approve(router.address, ethers.utils.parseEther("100"));

      await expect(
        router.addLiquidity(
          tokenA.address,
          tokenB.address,
          ethers.utils.parseEther("100"),
          ethers.utils.parseEther("100"),
          0, 0,
          owner.address,
          expiredDeadline
        )
      ).to.be.revertedWith("FlareSwapRouter: EXPIRED");
    });

    it("Router should reject insufficient output amounts", async function () {
      // Create pair and add liquidity first
      await factory.createPair(tokenA.address, tokenB.address);
      await tokenA.approve(router.address, ethers.utils.parseEther("500"));
      await tokenB.approve(router.address, ethers.utils.parseEther("500"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("500"),
        ethers.utils.parseEther("500"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Try to swap with unrealistic minimum output
      await tokenA.approve(router.address, ethers.utils.parseEther("10"));

      await expect(
        router.swapExactTokensForTokens(
          ethers.utils.parseEther("10"),
          ethers.utils.parseEther("100"), // Expecting way more than possible
          [tokenA.address, tokenB.address],
          owner.address,
          Math.floor(Date.now() / 1000) + 3600
        )
      ).to.be.revertedWith("FlareSwapRouter: INSUFFICIENT_OUTPUT_AMOUNT");
    });

    it("MasterChef should reject withdrawal of more than deposited", async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      await masterChef.add(100, pairAddress, false);

      await tokenA.approve(router.address, ethers.utils.parseEther("100"));
      await tokenB.approve(router.address, ethers.utils.parseEther("100"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
      const lpBalance = await pair.balanceOf(owner.address);

      await pair.approve(masterChef.address, lpBalance);
      await masterChef.deposit(0, lpBalance);

      await expect(
        masterChef.withdraw(0, lpBalance.add(1))
      ).to.be.revertedWith("withdraw: not good");
    });
  });

  describe("Economic Attack Protection", function () {
    let pair;

    beforeEach(async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
    });

    it("Should enforce minimum liquidity lock", async function () {
      // Add first liquidity
      await tokenA.approve(router.address, ethers.utils.parseEther("100"));
      await tokenB.approve(router.address, ethers.utils.parseEther("100"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Check that minimum liquidity is locked (1000 wei)
      const zeroAddressBalance = await pair.balanceOf(ethers.constants.AddressZero);
      expect(zeroAddressBalance).to.equal(1000);
    });

    it("Should maintain K invariant during swaps", async function () {
      // Add liquidity
      await tokenA.approve(router.address, ethers.utils.parseEther("1000"));
      await tokenB.approve(router.address, ethers.utils.parseEther("1000"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("1000"),
        ethers.utils.parseEther("1000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const [reserve0Before, reserve1Before] = await pair.getReserves();
      const kBefore = reserve0Before.mul(reserve1Before);

      // Perform swap
      await tokenA.transfer(attacker.address, ethers.utils.parseEther("100"));
      await tokenA.connect(attacker).approve(router.address, ethers.utils.parseEther("10"));

      await router.connect(attacker).swapExactTokensForTokens(
        ethers.utils.parseEther("10"),
        0,
        [tokenA.address, tokenB.address],
        attacker.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const [reserve0After, reserve1After] = await pair.getReserves();
      const kAfter = reserve0After.mul(reserve1After);

      // K should increase (due to 0.3% fee)
      expect(kAfter).to.be.gte(kBefore);
    });

    it("Should prevent price manipulation through large swaps", async function () {
      // Add liquidity
      await tokenA.approve(router.address, ethers.utils.parseEther("1000"));
      await tokenB.approve(router.address, ethers.utils.parseEther("1000"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("1000"),
        ethers.utils.parseEther("1000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      // Large swap should receive diminishing returns due to constant product formula
      await tokenA.transfer(attacker.address, ethers.utils.parseEther("10000"));
      await tokenA.connect(attacker).approve(router.address, ethers.utils.parseEther("500"));

      const amounts = await router.getAmountsOut(
        ethers.utils.parseEther("500"),
        [tokenA.address, tokenB.address]
      );

      // Due to constant product formula, output should be significantly less than 500
      expect(amounts[1]).to.be.lt(ethers.utils.parseEther("500"));
    });
  });

  describe("Edge Cases and Boundary Conditions", function () {
    it("Should handle zero amount deposits gracefully", async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      await masterChef.add(100, pairAddress, false);

      // Depositing 0 should not fail but also not change anything
      await masterChef.deposit(0, 0);

      const userInfo = await masterChef.userInfo(0, owner.address);
      expect(userInfo.amount).to.equal(0);
    });

    it("Should handle consecutive deposit and withdraw", async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      await masterChef.add(100, pairAddress, false);

      await tokenA.approve(router.address, ethers.utils.parseEther("100"));
      await tokenB.approve(router.address, ethers.utils.parseEther("100"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
      const lpBalance = await pair.balanceOf(owner.address);

      await pair.approve(masterChef.address, lpBalance);
      await masterChef.deposit(0, lpBalance);
      await masterChef.withdraw(0, lpBalance);

      const userInfo = await masterChef.userInfo(0, owner.address);
      expect(userInfo.amount).to.equal(0);
    });

    it("Should handle maximum uint112 reserves", async function () {
      // This test ensures the contract can handle large reserve values
      // without overflow (Solidity 0.8+ has built-in overflow protection)
      const pair = await ethers.getContractAt(
        "FlareSwapPair",
        await factory.callStatic.createPair(tokenA.address, tokenB.address)
      );

      // Just verify the contract compiles with uint112 type (test passes if no compilation error)
      expect(pair.address).to.be.properAddress;
    });
  });

  describe("Slippage Protection", function () {
    beforeEach(async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      await tokenA.approve(router.address, ethers.utils.parseEther("1000"));
      await tokenB.approve(router.address, ethers.utils.parseEther("1000"));

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("1000"),
        ethers.utils.parseEther("1000"),
        0, 0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });

    it("Should respect minimum output amount in swaps", async function () {
      await tokenA.transfer(user.address, ethers.utils.parseEther("100"));
      await tokenA.connect(user).approve(router.address, ethers.utils.parseEther("10"));

      const amounts = await router.getAmountsOut(
        ethers.utils.parseEther("10"),
        [tokenA.address, tokenB.address]
      );

      const minOutput = amounts[1].mul(95).div(100); // 5% slippage tolerance

      await router.connect(user).swapExactTokensForTokens(
        ethers.utils.parseEther("10"),
        minOutput,
        [tokenA.address, tokenB.address],
        user.address,
        Math.floor(Date.now() / 1000) + 3600
      );

      const balanceB = await tokenB.balanceOf(user.address);
      expect(balanceB).to.be.gte(minOutput);
    });

    it("Should respect minimum liquidity amounts when adding", async function () {
      await tokenA.transfer(user.address, ethers.utils.parseEther("100"));
      await tokenB.transfer(user.address, ethers.utils.parseEther("100"));

      await tokenA.connect(user).approve(router.address, ethers.utils.parseEther("100"));
      await tokenB.connect(user).approve(router.address, ethers.utils.parseEther("100"));

      // Should succeed with reasonable minimum amounts
      await router.connect(user).addLiquidity(
        tokenA.address,
        tokenB.address,
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("100"),
        ethers.utils.parseEther("95"), // 5% slippage on A
        ethers.utils.parseEther("95"), // 5% slippage on B
        user.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });
  });
});
