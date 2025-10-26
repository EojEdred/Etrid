import { expect } from "chai";
import { ethers } from "hardhat";
import { EtridToken } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";

describe("EtridToken", function () {
  let etr: EtridToken;
  let owner: SignerWithAddress;
  let bridge: SignerWithAddress;
  let user1: SignerWithAddress;
  let user2: SignerWithAddress;

  const TOKEN_NAME = "Etrid Coin Test";
  const TOKEN_SYMBOL = "ÉTR";
  const MINT_AMOUNT = ethers.parseEther("1000");
  const TRANSFER_AMOUNT = ethers.parseEther("100");

  beforeEach(async function () {
    [owner, bridge, user1, user2] = await ethers.getSigners();

    const EtridToken = await ethers.getContractFactory("EtridToken");
    etr = await EtridToken.deploy(TOKEN_NAME, TOKEN_SYMBOL);
    await etr.waitForDeployment();

    // Grant bridge role to bridge signer
    const BRIDGE_ROLE = await etr.BRIDGE_ROLE();
    await etr.grantRole(BRIDGE_ROLE, bridge.address);
  });

  describe("Deployment", function () {
    it("Should set the correct name and symbol", async function () {
      expect(await etr.name()).to.equal(TOKEN_NAME);
      expect(await etr.symbol()).to.equal(TOKEN_SYMBOL);
    });

    it("Should have 18 decimals", async function () {
      expect(await etr.decimals()).to.equal(18);
    });

    it("Should start with zero total supply", async function () {
      expect(await etr.totalSupply()).to.equal(0);
    });

    it("Should grant all roles to deployer", async function () {
      const DEFAULT_ADMIN_ROLE = await etr.DEFAULT_ADMIN_ROLE();
      const MINTER_ROLE = await etr.MINTER_ROLE();
      const PAUSER_ROLE = await etr.PAUSER_ROLE();
      const BRIDGE_ROLE = await etr.BRIDGE_ROLE();

      expect(await etr.hasRole(DEFAULT_ADMIN_ROLE, owner.address)).to.be.true;
      expect(await etr.hasRole(MINTER_ROLE, owner.address)).to.be.true;
      expect(await etr.hasRole(PAUSER_ROLE, owner.address)).to.be.true;
      expect(await etr.hasRole(BRIDGE_ROLE, owner.address)).to.be.true;
    });
  });

  describe("Bridge Minting", function () {
    it("Should allow bridge to mint tokens", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_tx_hash"));

      await expect(
        etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash)
      )
        .to.emit(etr, "BridgeMint")
        .withArgs(user1.address, MINT_AMOUNT, txHash);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT);
      expect(await etr.totalSupply()).to.equal(MINT_AMOUNT);
    });

    it("Should not allow non-bridge to mint", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_tx_hash"));

      await expect(
        etr.connect(user1).bridgeMint(user2.address, MINT_AMOUNT, txHash)
      ).to.be.reverted;
    });

    it("Should not allow minting to zero address", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_tx_hash"));

      await expect(
        etr.connect(bridge).bridgeMint(ethers.ZeroAddress, MINT_AMOUNT, txHash)
      ).to.be.revertedWith("EtridToken: mint to zero address");
    });

    it("Should not allow minting zero amount", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_tx_hash"));

      await expect(
        etr.connect(bridge).bridgeMint(user1.address, 0, txHash)
      ).to.be.revertedWith("EtridToken: mint amount must be positive");
    });

    it("Should not allow minting when paused", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("test_tx_hash"));

      await etr.connect(owner).pause();

      await expect(
        etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash)
      ).to.be.revertedWith("Pausable: paused");
    });
  });

  describe("Bridge Burning", function () {
    beforeEach(async function () {
      // Mint tokens to user1 first
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("setup_mint"));
      await etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash);
    });

    it("Should allow users to burn tokens for bridging", async function () {
      const burnAmount = ethers.parseEther("500");
      const etridAddress = "etrid1abc123...";

      await expect(
        etr.connect(user1).bridgeBurn(burnAmount, etridAddress)
      )
        .to.emit(etr, "BridgeBurn")
        .withArgs(user1.address, burnAmount, etridAddress);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT - burnAmount);
      expect(await etr.totalSupply()).to.equal(MINT_AMOUNT - burnAmount);
    });

    it("Should not allow burning more than balance", async function () {
      const burnAmount = MINT_AMOUNT + ethers.parseEther("1");
      const etridAddress = "etrid1abc123...";

      await expect(
        etr.connect(user1).bridgeBurn(burnAmount, etridAddress)
      ).to.be.reverted;
    });

    it("Should not allow burning zero amount", async function () {
      const etridAddress = "etrid1abc123...";

      await expect(
        etr.connect(user1).bridgeBurn(0, etridAddress)
      ).to.be.revertedWith("EtridToken: burn amount must be positive");
    });

    it("Should not allow burning with empty Etrid address", async function () {
      const burnAmount = ethers.parseEther("500");

      await expect(
        etr.connect(user1).bridgeBurn(burnAmount, "")
      ).to.be.revertedWith("EtridToken: invalid Etrid address");
    });

    it("Should not allow burning when paused", async function () {
      const burnAmount = ethers.parseEther("500");
      const etridAddress = "etrid1abc123...";

      await etr.connect(owner).pause();

      await expect(
        etr.connect(user1).bridgeBurn(burnAmount, etridAddress)
      ).to.be.revertedWith("Pausable: paused");
    });
  });

  describe("Standard ERC20 Transfers", function () {
    beforeEach(async function () {
      // Mint tokens to user1 first
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("setup_mint"));
      await etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash);
    });

    it("Should allow token transfers", async function () {
      await etr.connect(user1).transfer(user2.address, TRANSFER_AMOUNT);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT - TRANSFER_AMOUNT);
      expect(await etr.balanceOf(user2.address)).to.equal(TRANSFER_AMOUNT);
    });

    it("Should not allow transfers when paused", async function () {
      await etr.connect(owner).pause();

      await expect(
        etr.connect(user1).transfer(user2.address, TRANSFER_AMOUNT)
      ).to.be.revertedWith("Pausable: paused");
    });

    it("Should allow token approvals and transferFrom", async function () {
      await etr.connect(user1).approve(user2.address, TRANSFER_AMOUNT);

      expect(await etr.allowance(user1.address, user2.address)).to.equal(TRANSFER_AMOUNT);

      await etr.connect(user2).transferFrom(user1.address, user2.address, TRANSFER_AMOUNT);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT - TRANSFER_AMOUNT);
      expect(await etr.balanceOf(user2.address)).to.equal(TRANSFER_AMOUNT);
    });
  });

  describe("Pausable", function () {
    it("Should allow pauser to pause", async function () {
      await etr.connect(owner).pause();
      expect(await etr.paused()).to.be.true;
    });

    it("Should allow pauser to unpause", async function () {
      await etr.connect(owner).pause();
      await etr.connect(owner).unpause();
      expect(await etr.paused()).to.be.false;
    });

    it("Should not allow non-pauser to pause", async function () {
      await expect(etr.connect(user1).pause()).to.be.reverted;
    });

    it("Should block all transfers when paused", async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("setup_mint"));
      await etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash);

      await etr.connect(owner).pause();

      await expect(
        etr.connect(user1).transfer(user2.address, TRANSFER_AMOUNT)
      ).to.be.revertedWith("Pausable: paused");
    });
  });

  describe("Access Control", function () {
    it("Should allow admin to grant roles", async function () {
      const MINTER_ROLE = await etr.MINTER_ROLE();

      await etr.connect(owner).grantRole(MINTER_ROLE, user1.address);

      expect(await etr.hasRole(MINTER_ROLE, user1.address)).to.be.true;
    });

    it("Should allow admin to revoke roles", async function () {
      const BRIDGE_ROLE = await etr.BRIDGE_ROLE();

      await etr.connect(owner).revokeRole(BRIDGE_ROLE, bridge.address);

      expect(await etr.hasRole(BRIDGE_ROLE, bridge.address)).to.be.false;
    });

    it("Should not allow non-admin to grant roles", async function () {
      const MINTER_ROLE = await etr.MINTER_ROLE();

      await expect(
        etr.connect(user1).grantRole(MINTER_ROLE, user2.address)
      ).to.be.reverted;
    });
  });

  describe("Burnable (Standard ERC20Burnable)", function () {
    beforeEach(async function () {
      const txHash = ethers.keccak256(ethers.toUtf8Bytes("setup_mint"));
      await etr.connect(bridge).bridgeMint(user1.address, MINT_AMOUNT, txHash);
    });

    it("Should allow users to burn their own tokens", async function () {
      const burnAmount = ethers.parseEther("500");

      await etr.connect(user1).burn(burnAmount);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT - burnAmount);
      expect(await etr.totalSupply()).to.equal(MINT_AMOUNT - burnAmount);
    });

    it("Should allow users to burn tokens from allowance", async function () {
      const burnAmount = ethers.parseEther("500");

      await etr.connect(user1).approve(user2.address, burnAmount);
      await etr.connect(user2).burnFrom(user1.address, burnAmount);

      expect(await etr.balanceOf(user1.address)).to.equal(MINT_AMOUNT - burnAmount);
      expect(await etr.totalSupply()).to.equal(MINT_AMOUNT - burnAmount);
    });
  });
});
