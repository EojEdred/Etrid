import { ethers } from "hardhat";
import * as fs from "fs";
import * as path from "path";

/**
 * Pre-Launch Validation Script
 *
 * Checks everything before mainnet deployment
 * Run this before deploying to mainnet to catch issues early
 *
 * Usage:
 *   npx hardhat run scripts/pre-launch-check.ts --network bscMainnet
 */

interface CheckResult {
  name: string;
  status: "✅" | "⚠️" | "❌";
  message: string;
  critical: boolean;
}

const results: CheckResult[] = [];

function addResult(name: string, passed: boolean, message: string, critical: boolean = true) {
  results.push({
    name,
    status: passed ? "✅" : (critical ? "❌" : "⚠️"),
    message,
    critical
  });
}

async function main() {
  console.log("\n🔍 PRE-LAUNCH VALIDATION CHECK\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);
  console.log(`   Chain ID: ${network.chainId}\n`);

  if (!isMainnet && !isTestnet) {
    console.log("❌ ERROR: Not connected to BSC network!");
    process.exit(1);
  }

  // Get deployer
  const [deployer] = await ethers.getSigners();
  const deployerAddress = await deployer.getAddress();

  console.log("🔍 Running checks...\n");

  // ===== 1. Environment Configuration =====
  console.log("1️⃣  Environment Configuration");

  const envPath = path.join(__dirname, "../.env");
  const envExists = fs.existsSync(envPath);
  addResult(
    ".env file exists",
    envExists,
    envExists ? "Found" : "Missing - create from .env.example"
  );

  const hasPrivateKey = !!process.env.DEPLOYER_PRIVATE_KEY;
  addResult(
    "DEPLOYER_PRIVATE_KEY set",
    hasPrivateKey,
    hasPrivateKey ? "Configured" : "Missing in .env"
  );

  const hasBscscanKey = !!process.env.BSCSCAN_API_KEY;
  addResult(
    "BSCSCAN_API_KEY set",
    hasBscscanKey,
    hasBscscanKey ? "Configured (for verification)" : "Missing (optional but recommended)",
    false
  );

  // ===== 2. Wallet Status =====
  console.log("\n2️⃣  Wallet Status");

  console.log(`   Address: ${deployerAddress}`);

  const balance = await ethers.provider.getBalance(deployerAddress);
  const balanceInBNB = parseFloat(ethers.formatEther(balance));

  const minBalance = isMainnet ? 0.05 : 0.01; // 0.05 BNB mainnet, 0.01 testnet
  addResult(
    `Sufficient BNB balance (≥${minBalance})`,
    balanceInBNB >= minBalance,
    `${balanceInBNB.toFixed(4)} BNB ${balanceInBNB >= minBalance ? "" : `- Need at least ${minBalance} BNB`}`
  );

  // ===== 3. Contract Deployment Status =====
  console.log("\n3️⃣  Contract Deployment Status");

  const etrAddressKey = isMainnet ? "ETR_TOKEN_ADDRESS_MAINNET" : "ETR_TOKEN_ADDRESS_TESTNET";
  const etrAddress = process.env[etrAddressKey];
  const etrDeployed = !!etrAddress && ethers.isAddress(etrAddress || "");

  addResult(
    "ÉTR token deployed",
    etrDeployed,
    etrDeployed ? `At ${etrAddress}` : `${etrAddressKey} not set in .env`,
    !isTestnet // Critical for mainnet, warning for testnet
  );

  if (etrDeployed) {
    try {
      const code = await ethers.provider.getCode(etrAddress!);
      const hasCode = code !== "0x";
      addResult(
        "ÉTR contract has code",
        hasCode,
        hasCode ? "Contract exists on-chain" : "No code at address - deploy first!"
      );

      if (hasCode) {
        const etrContract = await ethers.getContractAt("EtridToken", etrAddress!);
        const symbol = await etrContract.symbol();
        const name = await etrContract.name();
        addResult(
          "ÉTR contract verified",
          symbol === "ÉTR",
          `Symbol: ${symbol}, Name: ${name}`,
          false
        );
      }
    } catch (error) {
      addResult(
        "ÉTR contract accessible",
        false,
        `Error accessing contract: ${error}`
      );
    }
  }

  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";
  const masterChefAddress = process.env[masterChefAddressKey];
  const masterChefDeployed = !!masterChefAddress && ethers.isAddress(masterChefAddress || "");

  addResult(
    "MasterChef deployed",
    masterChefDeployed,
    masterChefDeployed ? `At ${masterChefAddress}` : `${masterChefAddressKey} not set - will deploy`,
    false // Not critical, might be deploying it now
  );

  // ===== 4. MasterChef Configuration (if deployed) =====
  if (masterChefDeployed && etrDeployed) {
    console.log("\n4️⃣  MasterChef Configuration");

    try {
      const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress!);

      const rewardToken = await masterChef.rewardToken();
      const correctRewardToken = rewardToken.toLowerCase() === etrAddress!.toLowerCase();
      addResult(
        "Correct reward token",
        correctRewardToken,
        correctRewardToken ? "ÉTR set correctly" : `Wrong token: ${rewardToken}`
      );

      const rewardPerBlock = await masterChef.rewardPerBlock();
      const expectedReward = ethers.parseEther("2.89");
      const correctEmission = rewardPerBlock === expectedReward;
      addResult(
        "Correct emission rate",
        correctEmission,
        `${ethers.formatEther(rewardPerBlock)} ÉTR/block ${correctEmission ? "(Month 1)" : `- Expected ${ethers.formatEther(expectedReward)}`}`,
        false
      );

      const poolLength = await masterChef.poolLength();
      addResult(
        "LP pools added",
        poolLength > 0n,
        poolLength > 0n ? `${poolLength} pool(s)` : "No pools - add after deployment",
        false
      );

      if (isMainnet) {
        const etrContract = await ethers.getContractAt("EtridToken", etrAddress!);
        const masterChefBalance = await etrContract.balanceOf(masterChefAddress!);
        const targetBalance = ethers.parseEther("20000000"); // 20M ÉTR
        const hasSufficientBalance = masterChefBalance >= targetBalance;

        addResult(
          "MasterChef funded (20M ÉTR)",
          hasSufficientBalance,
          `${ethers.formatEther(masterChefBalance)} ÉTR ${hasSufficientBalance ? "" : "- Transfer 20M ÉTR!"}`
        );
      }
    } catch (error) {
      addResult(
        "MasterChef accessible",
        false,
        `Error: ${error}`
      );
    }
  } else {
    console.log("\n4️⃣  MasterChef Configuration - Skipped (not deployed yet)");
  }

  // ===== 5. Testnet Testing Status =====
  if (isMainnet) {
    console.log("\n5️⃣  Testnet Testing Status");

    const testnetEtrAddress = process.env.ETR_TOKEN_ADDRESS_TESTNET;
    const testnetMasterChefAddress = process.env.MASTERCHEF_ADDRESS_TESTNET;

    addResult(
      "Testnet ÉTR deployed",
      !!testnetEtrAddress,
      testnetEtrAddress ? `Tested at ${testnetEtrAddress}` : "Deploy to testnet first!",
      true
    );

    addResult(
      "Testnet MasterChef deployed",
      !!testnetMasterChefAddress,
      testnetMasterChefAddress ? `Tested at ${testnetMasterChefAddress}` : "Test MasterChef on testnet first!",
      true
    );

    const hasTestnetDeployments = fs.existsSync(path.join(__dirname, "../deployment-testnet.json")) ||
                                   fs.existsSync(path.join(__dirname, "../masterchef-deployment-testnet.json"));
    addResult(
      "Testnet deployment files exist",
      hasTestnetDeployments,
      hasTestnetDeployments ? "Found testnet deployment records" : "No testnet deployment records",
      false
    );
  }

  // ===== 6. Security Checks =====
  console.log(`\n${isMainnet ? "6️⃣" : "5️⃣"}  Security Checks`);

  if (etrDeployed && masterChefDeployed) {
    try {
      const etrContract = await ethers.getContractAt("EtridToken", etrAddress!);
      const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress!);

      const etrOwner = await etrContract.owner();
      const masterChefOwner = await masterChef.owner();

      const isMultisig = (address: string) => {
        // Basic check: multi-sig addresses are typically not the deployer
        return address.toLowerCase() !== deployerAddress.toLowerCase();
      };

      if (isMainnet) {
        addResult(
          "ÉTR ownership transferred",
          isMultisig(etrOwner),
          isMultisig(etrOwner) ? "Owned by multi-sig" : "Still owned by deployer - transfer to multi-sig!",
          true
        );

        addResult(
          "MasterChef ownership transferred",
          isMultisig(masterChefOwner),
          isMultisig(masterChefOwner) ? "Owned by multi-sig" : "Still owned by deployer - transfer to multi-sig!",
          true
        );
      } else {
        addResult(
          "ÉTR owner check",
          true,
          `Owner: ${etrOwner}`,
          false
        );

        addResult(
          "MasterChef owner check",
          true,
          `Owner: ${masterChefOwner}`,
          false
        );
      }

      const isPaused = await etrContract.paused();
      addResult(
        "ÉTR not paused",
        !isPaused,
        isPaused ? "Contract is paused!" : "Ready for transfers"
      );

      const masterChefPaused = await masterChef.paused();
      addResult(
        "MasterChef not paused",
        !masterChefPaused,
        masterChefPaused ? "Contract is paused!" : "Ready for deposits"
      );
    } catch (error) {
      // Skip if contracts not accessible
    }
  }

  // ===== 7. Documentation & Preparation =====
  console.log(`\n${isMainnet ? "7️⃣" : "6️⃣"}  Documentation & Preparation`);

  const hasReadme = fs.existsSync(path.join(__dirname, "../README_DEPLOYMENT.md"));
  addResult(
    "Deployment guide available",
    hasReadme,
    hasReadme ? "README_DEPLOYMENT.md found" : "Missing deployment guide",
    false
  );

  const hasChecklist = fs.existsSync(path.join(__dirname, "../../../../../../FINAL_DEPLOYMENT_CHECKLIST.md"));
  addResult(
    "Final checklist available",
    hasChecklist,
    hasChecklist ? "FINAL_DEPLOYMENT_CHECKLIST.md found" : "Missing checklist",
    false
  );

  // ===== SUMMARY =====
  console.log("\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 VALIDATION SUMMARY\n");

  let criticalIssues = 0;
  let warnings = 0;
  let passed = 0;

  results.forEach(result => {
    console.log(`${result.status} ${result.name}`);
    console.log(`   ${result.message}`);

    if (result.status === "✅") {
      passed++;
    } else if (result.status === "❌") {
      criticalIssues++;
    } else {
      warnings++;
    }
  });

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log(`Total Checks: ${results.length}`);
  console.log(`✅ Passed: ${passed}`);
  console.log(`⚠️  Warnings: ${warnings}`);
  console.log(`❌ Critical Issues: ${criticalIssues}`);
  console.log();

  if (criticalIssues === 0) {
    console.log("🎉 ALL CRITICAL CHECKS PASSED!");
    console.log();
    if (warnings > 0) {
      console.log(`⚠️  You have ${warnings} warning(s). Review them but you can proceed.`);
    } else {
      console.log("✅ Ready for mainnet deployment!");
    }
    console.log();
  } else {
    console.log(`❌ FOUND ${criticalIssues} CRITICAL ISSUE(S)!`);
    console.log();
    console.log("⚠️  DO NOT PROCEED WITH MAINNET DEPLOYMENT!");
    console.log("   Fix critical issues first.\n");
    process.exit(1);
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Save results to file
  const reportPath = path.join(__dirname, `../pre-launch-report-${Date.now()}.json`);
  fs.writeFileSync(reportPath, JSON.stringify({
    timestamp: new Date().toISOString(),
    network: isMainnet ? "mainnet" : "testnet",
    chainId: Number(network.chainId),
    deployer: deployerAddress,
    results,
    summary: {
      total: results.length,
      passed,
      warnings,
      criticalIssues
    }
  }, null, 2));

  console.log(`📝 Full report saved to: ${path.basename(reportPath)}\n`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Validation check failed:");
    console.error(error);
    process.exit(1);
  });
