// FlareSwap Test Suite
const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("FlareSwap", function () {
  let factory, router, weth;
  let tokenA, tokenB;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    // Deploy WETH
    const WETH = await ethers.getContractFactory("WETH");
    weth = await WETH.deploy();
    await weth.deployed();

    // Deploy Factory
    const Factory = await ethers.getContractFactory("FlareSwapFactory");
    factory = await Factory.deploy(owner.address);
    await factory.deployed();

    // Deploy Router
    const Router = await ethers.getContractFactory("FlareSwapRouter");
    router = await Router.deploy(factory.address, weth.address);
    await router.deployed();

    // Deploy test tokens
    const Token = await ethers.getContractFactory("MockERC20");
    tokenA = await Token.deploy("Token A", "TKA", ethers.utils.parseEther("10000"));
    tokenB = await Token.deploy("Token B", "TKB", ethers.utils.parseEther("10000"));
    await tokenA.deployed();
    await tokenB.deployed();
  });

  describe("Factory", function () {
    it("Should create a pair", async function () {
      const tx = await factory.createPair(tokenA.address, tokenB.address);
      await tx.wait();

      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      expect(pairAddress).to.not.equal(ethers.constants.AddressZero);
    });

    it("Should not create duplicate pairs", async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      await expect(
        factory.createPair(tokenA.address, tokenB.address)
      ).to.be.revertedWith("FlareSwap: PAIR_EXISTS");
    });

    it("Should track all pairs", async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairsLength = await factory.allPairsLength();
      expect(pairsLength).to.equal(1);
    });
  });

  describe("Router - Add Liquidity", function () {
    it("Should add liquidity to a new pair", async function () {
      const amountA = ethers.utils.parseEther("100");
      const amountB = ethers.utils.parseEther("100");

      // Approve tokens
      await tokenA.approve(router.address, amountA);
      await tokenB.approve(router.address, amountB);

      // Add liquidity
      await expect(
        router.addLiquidity(
          tokenA.address,
          tokenB.address,
          amountA,
          amountB,
          0,
          0,
          owner.address,
          Math.floor(Date.now() / 1000) + 3600
        )
      ).to.emit(factory, "PairCreated");
    });

    it("Should add liquidity with ETH", async function () {
      const amountToken = ethers.utils.parseEther("100");
      const amountETH = ethers.utils.parseEther("1");

      await tokenA.approve(router.address, amountToken);

      await expect(
        router.addLiquidityETH(
          tokenA.address,
          amountToken,
          0,
          0,
          owner.address,
          Math.floor(Date.now() / 1000) + 3600,
          { value: amountETH }
        )
      ).to.not.be.reverted;
    });
  });

  describe("Router - Swaps", function () {
    beforeEach(async function () {
      // Setup: Add initial liquidity
      const amountA = ethers.utils.parseEther("1000");
      const amountB = ethers.utils.parseEther("1000");

      await tokenA.approve(router.address, amountA);
      await tokenB.approve(router.address, amountB);

      await router.addLiquidity(
        tokenA.address,
        tokenB.address,
        amountA,
        amountB,
        0,
        0,
        owner.address,
        Math.floor(Date.now() / 1000) + 3600
      );
    });

    it("Should swap exact tokens for tokens", async function () {
      const amountIn = ethers.utils.parseEther("10");
      await tokenA.transfer(addr1.address, amountIn);
      await tokenA.connect(addr1).approve(router.address, amountIn);

      const path = [tokenA.address, tokenB.address];
      await expect(
        router.connect(addr1).swapExactTokensForTokens(
          amountIn,
          0,
          path,
          addr1.address,
          Math.floor(Date.now() / 1000) + 3600
        )
      ).to.not.be.reverted;

      const balanceB = await tokenB.balanceOf(addr1.address);
      expect(balanceB).to.be.gt(0);
    });

    it("Should calculate amounts correctly", async function () {
      const amountIn = ethers.utils.parseEther("10");
      const path = [tokenA.address, tokenB.address];
      const amounts = await router.getAmountsOut(amountIn, path);

      expect(amounts.length).to.equal(2);
      expect(amounts[0]).to.equal(amountIn);
      expect(amounts[1]).to.be.gt(0);
    });
  });

  describe("Pair", function () {
    let pair;

    beforeEach(async function () {
      await factory.createPair(tokenA.address, tokenB.address);
      const pairAddress = await factory.getPair(tokenA.address, tokenB.address);
      pair = await ethers.getContractAt("FlareSwapPair", pairAddress);
    });

    it("Should have correct token addresses", async function () {
      const token0 = await pair.token0();
      const token1 = await pair.token1();

      expect([token0, token1].sort()).to.deep.equal(
        [tokenA.address, tokenB.address].sort()
      );
    });

    it("Should start with zero reserves", async function () {
      const [reserve0, reserve1] = await pair.getReserves();
      expect(reserve0).to.equal(0);
      expect(reserve1).to.equal(0);
    });
  });
});

// Mock ERC20 for testing (you'll need to create this)
// contracts/test/MockERC20.sol
