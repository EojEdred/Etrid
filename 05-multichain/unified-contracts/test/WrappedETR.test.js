const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("WrappedETR", function () {
  let wrappedETR;
  let owner;
  let bridge;
  let user1;
  let user2;

  beforeEach(async function () {
    [owner, bridge, user1, user2] = await ethers.getSigners();

    const WrappedETR = await ethers.getContractFactory("WrappedETR");
    wrappedETR = await WrappedETR.deploy(owner.address);
    await wrappedETR.waitForDeployment();
  });

  describe("Deployment", function () {
    it("Should set the correct name and symbol", async function () {
      expect(await wrappedETR.name()).to.equal("Wrapped ETR");
      expect(await wrappedETR.symbol()).to.equal("wETR");
    });

    it("Should set the correct decimals", async function () {
      expect(await wrappedETR.decimals()).to.equal(18);
    });

    it("Should grant admin role to deployer", async function () {
      const DEFAULT_ADMIN_ROLE = await wrappedETR.DEFAULT_ADMIN_ROLE();
      expect(await wrappedETR.hasRole(DEFAULT_ADMIN_ROLE, owner.address)).to.be.true;
    });

    it("Should have zero initial supply", async function () {
      expect(await wrappedETR.totalSupply()).to.equal(0);
    });
  });

  describe("Bridge Minting", function () {
    beforeEach(async function () {
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      await wrappedETR.grantRole(MINTER_ROLE, bridge.address);
    });

    it("Should allow bridge to mint tokens", async function () {
      const amount = ethers.parseEther("100");
      const txHash = ethers.id("test-tx-hash");

      await expect(
        wrappedETR.connect(bridge).bridgeMint(user1.address, amount, txHash)
      )
        .to.emit(wrappedETR, "TokensBridgedIn")
        .withArgs(user1.address, amount, txHash);

      expect(await wrappedETR.balanceOf(user1.address)).to.equal(amount);
    });

    it("Should not allow non-bridge to mint", async function () {
      const amount = ethers.parseEther("100");
      const txHash = ethers.id("test-tx-hash");

      await expect(
        wrappedETR.connect(user1).bridgeMint(user1.address, amount, txHash)
      ).to.be.reverted;
    });

    it("Should not exceed max supply", async function () {
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      await wrappedETR.grantRole(MINTER_ROLE, bridge.address);

      const maxSupply = await wrappedETR.MAX_SUPPLY();
      const txHash = ethers.id("test-tx-hash");

      // Mint max supply
      await wrappedETR.connect(bridge).bridgeMint(user1.address, maxSupply, txHash);

      // Try to mint more
      await expect(
        wrappedETR.connect(bridge).bridgeMint(user2.address, 1, txHash)
      ).to.be.revertedWithCustomError(wrappedETR, "MaxSupplyExceeded");
    });
  });

  describe("Bridge Burning", function () {
    beforeEach(async function () {
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      const BURNER_ROLE = await wrappedETR.BURNER_ROLE();
      await wrappedETR.grantRole(MINTER_ROLE, bridge.address);
      await wrappedETR.grantRole(BURNER_ROLE, bridge.address);

      // Mint some tokens first
      const amount = ethers.parseEther("100");
      const txHash = ethers.id("test-tx-hash");
      await wrappedETR.connect(bridge).bridgeMint(user1.address, amount, txHash);
    });

    it("Should allow bridge to burn tokens", async function () {
      const amount = ethers.parseEther("50");
      const destinationAccount = ethers.id("substrate-account");

      await expect(
        wrappedETR.connect(bridge).bridgeBurn(user1.address, amount, destinationAccount)
      )
        .to.emit(wrappedETR, "TokensBridgedOut")
        .withArgs(user1.address, amount, destinationAccount);

      expect(await wrappedETR.balanceOf(user1.address)).to.equal(
        ethers.parseEther("50")
      );
    });

    it("Should not allow non-bridge to burn", async function () {
      const amount = ethers.parseEther("50");
      const destinationAccount = ethers.id("substrate-account");

      await expect(
        wrappedETR.connect(user1).bridgeBurn(user1.address, amount, destinationAccount)
      ).to.be.reverted;
    });
  });

  describe("Pausable", function () {
    it("Should allow pauser to pause", async function () {
      const PAUSER_ROLE = await wrappedETR.PAUSER_ROLE();
      expect(await wrappedETR.hasRole(PAUSER_ROLE, owner.address)).to.be.true;

      await wrappedETR.pause();
      expect(await wrappedETR.paused()).to.be.true;
    });

    it("Should prevent transfers when paused", async function () {
      // Mint tokens
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      await wrappedETR.grantRole(MINTER_ROLE, bridge.address);
      const amount = ethers.parseEther("100");
      const txHash = ethers.id("test-tx-hash");
      await wrappedETR.connect(bridge).bridgeMint(user1.address, amount, txHash);

      // Pause
      await wrappedETR.pause();

      // Try to transfer
      await expect(
        wrappedETR.connect(user1).transfer(user2.address, amount)
      ).to.be.reverted;
    });

    it("Should allow unpause", async function () {
      await wrappedETR.pause();
      await wrappedETR.unpause();
      expect(await wrappedETR.paused()).to.be.false;
    });
  });

  describe("Access Control", function () {
    it("Should allow admin to grant roles", async function () {
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      await wrappedETR.grantRole(MINTER_ROLE, bridge.address);
      expect(await wrappedETR.hasRole(MINTER_ROLE, bridge.address)).to.be.true;
    });

    it("Should not allow non-admin to grant roles", async function () {
      const MINTER_ROLE = await wrappedETR.MINTER_ROLE();
      await expect(
        wrappedETR.connect(user1).grantRole(MINTER_ROLE, user2.address)
      ).to.be.reverted;
    });
  });
});
