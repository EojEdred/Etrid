const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Ëtrid Bridge System", function () {
  let etrToken, edscToken, bridge;
  let owner, watchtower1, watchtower2, watchtower3, relayer, user;
  let BRIDGE_ROLE, WATCHTOWER_ROLE, AP_ROLE, ORACLE_ROLE;

  beforeEach(async function () {
    [owner, watchtower1, watchtower2, watchtower3, relayer, user] = await ethers.getSigners();

    // Deploy ETR Token
    const ETRToken = await ethers.getContractFactory("ETRToken");
    etrToken = await ETRToken.deploy(owner.address, owner.address);
    await etrToken.waitForDeployment();

    // Deploy EDSC Token
    const EDSCToken = await ethers.getContractFactory("EDSCToken");
    edscToken = await EDSCToken.deploy(owner.address, owner.address);
    await edscToken.waitForDeployment();

    // Deploy Bridge
    const EtridBridge = await ethers.getContractFactory("EtridBridge");
    bridge = await EtridBridge.deploy(
      owner.address,
      await etrToken.getAddress(),
      await edscToken.getAddress(),
      [watchtower1.address, watchtower2.address, watchtower3.address]
    );
    await bridge.waitForDeployment();

    // Setup roles
    BRIDGE_ROLE = await etrToken.BRIDGE_ROLE();
    WATCHTOWER_ROLE = await bridge.WATCHTOWER_ROLE();
    AP_ROLE = await edscToken.AP_ROLE();
    ORACLE_ROLE = await edscToken.ORACLE_ROLE();

    // Grant bridge role to bridge contract
    await etrToken.grantRole(BRIDGE_ROLE, await bridge.getAddress());
    await etrToken.revokeRole(BRIDGE_ROLE, owner.address);
    await edscToken.grantRole(BRIDGE_ROLE, await bridge.getAddress());
    await edscToken.revokeRole(BRIDGE_ROLE, owner.address);

    // Grant relayer role
    await bridge.grantRole(await bridge.RELAYER_ROLE(), relayer.address);
  });

  describe("Deployment", function () {
    it("Should set the correct token addresses", async function () {
      expect(await bridge.etrToken()).to.equal(await etrToken.getAddress());
      expect(await bridge.edscToken()).to.equal(await edscToken.getAddress());
    });

    it("Should register initial watchtowers", async function () {
      expect(await bridge.watchtowerCount()).to.equal(3);
      expect(await bridge.isWatchtower(watchtower1.address)).to.be.true;
      expect(await bridge.isWatchtower(watchtower2.address)).to.be.true;
      expect(await bridge.isWatchtower(watchtower3.address)).to.be.true;
    });

    it("Should grant correct roles", async function () {
      expect(await bridge.hasRole(WATCHTOWER_ROLE, watchtower1.address)).to.be.true;
      expect(await bridge.hasRole(await bridge.RELAYER_ROLE(), relayer.address)).to.be.true;
    });
  });

  describe("Bridge Minting (Lock on Ëtrid → Mint on Ethereum)", function () {
    it("Should mint ETR tokens with valid attestation", async function () {
      const amount = ethers.parseEther("1000");
      const etridTxHash = ethers.keccak256(ethers.toUtf8Bytes("etrid-tx-123"));
      const mintId = ethers.keccak256(ethers.toUtf8Bytes("mint-1"));
      const timestamp = Math.floor(Date.now() / 1000);

      // Create attestation
      const attestation = {
        token: await etrToken.getAddress(),
        to: user.address,
        amount: amount,
        etridTxHash: etridTxHash,
        timestamp: timestamp,
        mintId: mintId,
      };

      // Get attestation hash
      const attestationHash = await bridge.getAttestationHash(attestation);

      // Watchtowers sign
      const sig1 = await watchtower1.signMessage(ethers.getBytes(attestationHash));
      const sig2 = await watchtower2.signMessage(ethers.getBytes(attestationHash));
      const sig3 = await watchtower3.signMessage(ethers.getBytes(attestationHash));

      // Relayer submits
      await expect(
        bridge.connect(relayer).mintFromEtrid(attestation, [sig1, sig2, sig3])
      )
        .to.emit(bridge, "BridgeMinted")
        .withArgs(
          await etrToken.getAddress(),
          user.address,
          amount,
          etridTxHash,
          mintId
        );

      // Check balance
      expect(await etrToken.balanceOf(user.address)).to.equal(amount);
    });

    it("Should reject mint with insufficient signatures", async function () {
      const amount = ethers.parseEther("1000");
      const etridTxHash = ethers.keccak256(ethers.toUtf8Bytes("etrid-tx-123"));
      const mintId = ethers.keccak256(ethers.toUtf8Bytes("mint-1"));
      const timestamp = Math.floor(Date.now() / 1000);

      const attestation = {
        token: await etrToken.getAddress(),
        to: user.address,
        amount: amount,
        etridTxHash: etridTxHash,
        timestamp: timestamp,
        mintId: mintId,
      };

      const attestationHash = await bridge.getAttestationHash(attestation);
      const sig1 = await watchtower1.signMessage(ethers.getBytes(attestationHash));
      const sig2 = await watchtower2.signMessage(ethers.getBytes(attestationHash));

      // Only 2 signatures (need 3)
      await expect(
        bridge.connect(relayer).mintFromEtrid(attestation, [sig1, sig2])
      ).to.be.revertedWith("Bridge: insufficient signatures");
    });

    it("Should reject replay attacks", async function () {
      const amount = ethers.parseEther("1000");
      const etridTxHash = ethers.keccak256(ethers.toUtf8Bytes("etrid-tx-123"));
      const mintId = ethers.keccak256(ethers.toUtf8Bytes("mint-1"));
      const timestamp = Math.floor(Date.now() / 1000);

      const attestation = {
        token: await etrToken.getAddress(),
        to: user.address,
        amount: amount,
        etridTxHash: etridTxHash,
        timestamp: timestamp,
        mintId: mintId,
      };

      const attestationHash = await bridge.getAttestationHash(attestation);
      const sig1 = await watchtower1.signMessage(ethers.getBytes(attestationHash));
      const sig2 = await watchtower2.signMessage(ethers.getBytes(attestationHash));
      const sig3 = await watchtower3.signMessage(ethers.getBytes(attestationHash));

      // First mint succeeds
      await bridge.connect(relayer).mintFromEtrid(attestation, [sig1, sig2, sig3]);

      // Second mint with same mintId should fail
      await expect(
        bridge.connect(relayer).mintFromEtrid(attestation, [sig1, sig2, sig3])
      ).to.be.revertedWith("Bridge: mint already processed");
    });
  });

  describe("Token Burning (Burn on Ethereum → Release on Ëtrid)", function () {
    beforeEach(async function () {
      // Mint some tokens to user first
      const amount = ethers.parseEther("1000");
      const etridTxHash = ethers.keccak256(ethers.toUtf8Bytes("etrid-tx-123"));
      const mintId = ethers.keccak256(ethers.toUtf8Bytes("mint-1"));
      const timestamp = Math.floor(Date.now() / 1000);

      const attestation = {
        token: await etrToken.getAddress(),
        to: user.address,
        amount: amount,
        etridTxHash: etridTxHash,
        timestamp: timestamp,
        mintId: mintId,
      };

      const attestationHash = await bridge.getAttestationHash(attestation);
      const sig1 = await watchtower1.signMessage(ethers.getBytes(attestationHash));
      const sig2 = await watchtower2.signMessage(ethers.getBytes(attestationHash));
      const sig3 = await watchtower3.signMessage(ethers.getBytes(attestationHash));

      await bridge.connect(relayer).mintFromEtrid(attestation, [sig1, sig2, sig3]);
    });

    it("Should burn tokens and emit event", async function () {
      const amount = ethers.parseEther("500");
      const etridAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // SS58

      const userBalanceBefore = await etrToken.balanceOf(user.address);

      await expect(etrToken.connect(user).bridgeBurn(amount, etridAddress))
        .to.emit(etrToken, "BridgeBurn")
        .withArgs(user.address, amount, etridAddress);

      const userBalanceAfter = await etrToken.balanceOf(user.address);
      expect(userBalanceAfter).to.equal(userBalanceBefore - amount);
    });
  });

  describe("EDSC Stablecoin Features", function () {
    beforeEach(async function () {
      // Grant AP and Oracle roles
      await edscToken.addAP(owner.address);
      await edscToken.grantRole(ORACLE_ROLE, owner.address);
    });

    it("Should allow AP to mint with reserve proof", async function () {
      const amount = ethers.parseEther("100000");
      const reserveProof = ethers.keccak256(ethers.toUtf8Bytes("reserve-attestation-1"));

      await expect(edscToken.apMint(user.address, amount, reserveProof))
        .to.emit(edscToken, "APMint")
        .withArgs(owner.address, user.address, amount, reserveProof);

      expect(await edscToken.balanceOf(user.address)).to.equal(amount);
    });

    it("Should enforce daily mint limits", async function () {
      const maxDaily = ethers.parseEther("5000000"); // 5M EDSC
      const reserveProof = ethers.keccak256(ethers.toUtf8Bytes("reserve-attestation-1"));

      // This should fail (exceeds daily limit)
      await expect(
        edscToken.apMint(user.address, maxDaily + 1n, reserveProof)
      ).to.be.revertedWith("EDSC: exceeds daily limit");
    });

    it("Should allow oracle to update reserve ratio", async function () {
      const newRatio = 10500; // 105% reserves

      await expect(edscToken.updateReserveRatio(newRatio))
        .to.emit(edscToken, "ReserveRatioUpdated")
        .withArgs(10000, 10500);

      expect(await edscToken.reserveRatio()).to.equal(newRatio);
    });

    it("Should pause if reserve ratio drops below minimum", async function () {
      const lowRatio = 9500; // 95% (below 100% minimum)

      // This should auto-pause the contract
      await edscToken.updateReserveRatio(lowRatio);

      // Contract should be paused
      expect(await edscToken.paused()).to.be.true;

      // Transfers should fail
      const amount = ethers.parseEther("100");
      await expect(
        edscToken.transfer(user.address, amount)
      ).to.be.revertedWith("Pausable: paused");
    });
  });

  describe("Emergency Controls", function () {
    it("Should allow pauser to pause bridge", async function () {
      await bridge.pause();
      expect(await bridge.paused()).to.be.true;

      // Minting should fail when paused
      const amount = ethers.parseEther("1000");
      const attestation = {
        token: await etrToken.getAddress(),
        to: user.address,
        amount: amount,
        etridTxHash: ethers.keccak256(ethers.toUtf8Bytes("tx")),
        timestamp: Math.floor(Date.now() / 1000),
        mintId: ethers.keccak256(ethers.toUtf8Bytes("mint-1")),
      };

      await expect(
        bridge.connect(relayer).mintFromEtrid(attestation, [])
      ).to.be.revertedWith("Pausable: paused");
    });

    it("Should allow pauser to unpause bridge", async function () {
      await bridge.pause();
      await bridge.unpause();
      expect(await bridge.paused()).to.be.false;
    });
  });
});
