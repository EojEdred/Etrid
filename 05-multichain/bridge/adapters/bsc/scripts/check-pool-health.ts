import { ethers } from "hardhat";

/**
 * Check Pool Health
 *
 * Comprehensive health check for MasterChef pools:
 * - Reward distribution status
 * - Contract balance sufficiency
 * - Pool configuration validity
 * - Reward calculation accuracy
 *
 * Usage:
 *   npx hardhat run scripts/check-pool-health.ts --network bscTestnet
 *   npx hardhat run scripts/check-pool-health.ts --network bscMainnet
 */

interface HealthCheck {
  name: string;
  status: "✅" | "⚠️" | "❌";
  message: string;
  critical: boolean;
}

const checks: HealthCheck[] = [];

function addCheck(name: string, passed: boolean, message: string, critical: boolean = false) {
  checks.push({
    name,
    status: passed ? "✅" : critical ? "❌" : "⚠️",
    message,
    critical,
  });
}

async function main() {
  console.log("\n🏥 MASTERCHEF POOL HEALTH CHECK\n");
  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Get network info
  const network = await ethers.provider.getNetwork();
  const isMainnet = network.chainId === 56n;
  const isTestnet = network.chainId === 97n;

  console.log(`📍 Network: ${isMainnet ? "BSC Mainnet" : isTestnet ? "BSC Testnet" : "Unknown"}`);
  console.log(`   Chain ID: ${network.chainId}`);

  const blockNumber = await ethers.provider.getBlockNumber();
  console.log(`   Block: ${blockNumber}\n`);

  // Get addresses
  const etrAddressKey = isMainnet ? "ETR_TOKEN_ADDRESS_MAINNET" : "ETR_TOKEN_ADDRESS_TESTNET";
  const masterChefAddressKey = isMainnet ? "MASTERCHEF_ADDRESS_MAINNET" : "MASTERCHEF_ADDRESS_TESTNET";

  const etrAddress = process.env[etrAddressKey];
  const masterChefAddress = process.env[masterChefAddressKey];

  if (!etrAddress || !masterChefAddress) {
    console.log("❌ ERROR: Contract addresses not found in .env\n");
    process.exit(1);
  }

  console.log("📄 Contracts:");
  console.log(`   ÉTR Token:  ${etrAddress}`);
  console.log(`   MasterChef: ${masterChefAddress}\n`);

  // Get contracts
  const etr = await ethers.getContractAt("EtridToken", etrAddress);
  const masterChef = await ethers.getContractAt("MasterChef", masterChefAddress);

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("🔍 Running health checks...\n");

  // ===== 1. Basic Configuration =====
  console.log("1️⃣  Basic Configuration\n");

  try {
    const rewardToken = await masterChef.rewardToken();
    const correctRewardToken = rewardToken.toLowerCase() === etrAddress.toLowerCase();

    addCheck(
      "Correct reward token",
      correctRewardToken,
      correctRewardToken ? `ÉTR (${etrAddress})` : `Wrong token: ${rewardToken}`,
      true
    );
  } catch (error) {
    addCheck("Reward token check", false, `Error: ${error}`, true);
  }

  try {
    const isPaused = await masterChef.paused();
    addCheck(
      "Contract not paused",
      !isPaused,
      isPaused ? "MasterChef is paused!" : "Active",
      !isTestnet
    );
  } catch (error) {
    addCheck("Pause status check", false, `Error: ${error}`, false);
  }

  try {
    const poolLength = await masterChef.poolLength();
    addCheck(
      "Pools configured",
      poolLength > 0n,
      poolLength > 0n ? `${poolLength} pool(s)` : "No pools added",
      !isTestnet
    );
  } catch (error) {
    addCheck("Pool count check", false, `Error: ${error}`, true);
  }

  // ===== 2. Reward Balance =====
  console.log("\n2️⃣  Reward Balance\n");

  try {
    const masterChefBalance = await etr.balanceOf(masterChefAddress);
    const rewardPerBlock = await masterChef.rewardPerBlock();

    const blocksPerDay = 28800;
    const dailyRewards = rewardPerBlock * BigInt(blocksPerDay);

    const daysRemaining = masterChefBalance > 0n
      ? Number(masterChefBalance / dailyRewards)
      : 0;

    const hasSufficientBalance = daysRemaining >= 7; // At least 7 days

    addCheck(
      "Sufficient reward balance",
      hasSufficientBalance,
      `${ethers.formatEther(masterChefBalance)} ÉTR (${daysRemaining} days remaining)`,
      daysRemaining < 3
    );

    if (daysRemaining < 30 && isMainnet) {
      addCheck(
        "Adequate reserves",
        false,
        `Only ${daysRemaining} days remaining - consider topping up`,
        false
      );
    }
  } catch (error) {
    addCheck("Balance check", false, `Error: ${error}`, true);
  }

  // ===== 3. Pool Health =====
  console.log("\n3️⃣  Pool Health\n");

  try {
    const poolLength = await masterChef.poolLength();
    const totalAllocPoint = await masterChef.totalAllocPoint();

    addCheck(
      "Total allocation configured",
      totalAllocPoint > 0n,
      `${totalAllocPoint} allocation points`,
      true
    );

    for (let i = 0; i < Number(poolLength); i++) {
      const poolInfo = await masterChef.poolInfo(i);

      // Check LP token is valid
      let lpValid = false;
      try {
        const lpToken = await ethers.getContractAt("IERC20", poolInfo.lpToken);
        const totalSupply = await lpToken.totalSupply();
        lpValid = totalSupply > 0n;
      } catch (error) {
        lpValid = false;
      }

      addCheck(
        `Pool ${i} LP token valid`,
        lpValid,
        lpValid ? `Valid (${poolInfo.lpToken})` : `Invalid or no supply`,
        true
      );

      // Check allocation is reasonable
      const hasAllocation = poolInfo.allocPoint > 0n;
      addCheck(
        `Pool ${i} has allocation`,
        hasAllocation,
        hasAllocation ? `${poolInfo.allocPoint} points` : "No allocation",
        false
      );

      // Check last reward block is reasonable
      const blocksBehind = blockNumber - Number(poolInfo.lastRewardBlock);
      const isUpToDate = blocksBehind < 100; // Less than 5 minutes behind

      addCheck(
        `Pool ${i} rewards up to date`,
        isUpToDate,
        isUpToDate
          ? `Last updated ${blocksBehind} blocks ago`
          : `${blocksBehind} blocks behind - may need update`,
        false
      );
    }
  } catch (error) {
    addCheck("Pool health check", false, `Error: ${error}`, true);
  }

  // ===== 4. Ownership & Security =====
  console.log(`\n4️⃣  Ownership & Security\n`);

  try {
    const owner = await masterChef.owner();
    const code = await ethers.provider.getCode(owner);
    const isMultisig = code !== "0x";

    if (isMainnet) {
      addCheck(
        "Multi-sig ownership",
        isMultisig,
        isMultisig ? `Multi-sig: ${owner}` : `EOA: ${owner} (should be multi-sig!)`,
        true
      );
    } else {
      addCheck(
        "Owner set",
        true,
        `Owner: ${owner}`,
        false
      );
    }
  } catch (error) {
    addCheck("Ownership check", false, `Error: ${error}`, false);
  }

  try {
    const etrOwner = await etr.owner();
    const masterChefOwner = await masterChef.owner();

    const sameOwner = etrOwner.toLowerCase() === masterChefOwner.toLowerCase();

    addCheck(
      "Consistent ownership",
      sameOwner,
      sameOwner
        ? `Both owned by ${etrOwner}`
        : `Different owners: ÉTR (${etrOwner}) vs MasterChef (${masterChefOwner})`,
      false
    );
  } catch (error) {
    addCheck("Ownership consistency", false, `Error: ${error}`, false);
  }

  // ===== 5. Emission Schedule =====
  console.log(`\n5️⃣  Emission Schedule\n`);

  try {
    const rewardPerBlock = await masterChef.rewardPerBlock();
    const rewardPerBlockFloat = parseFloat(ethers.formatEther(rewardPerBlock));

    // Expected rates for 6-month program
    const expectedRates = [2.89, 4.05, 4.63];
    const isExpectedRate = expectedRates.some(
      rate => Math.abs(rate - rewardPerBlockFloat) < 0.01
    );

    addCheck(
      "Emission rate within expected range",
      isExpectedRate || !isMainnet,
      `${rewardPerBlockFloat} ÉTR/block`,
      false
    );

    // Calculate projected monthly emissions
    const blocksPerMonth = 28800 * 30;
    const monthlyEmissions = rewardPerBlock * BigInt(blocksPerMonth);

    addCheck(
      "Monthly emissions calculated",
      true,
      `~${ethers.formatEther(monthlyEmissions)} ÉTR/month`,
      false
    );
  } catch (error) {
    addCheck("Emission schedule check", false, `Error: ${error}`, false);
  }

  // ===== SUMMARY =====
  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log("📊 HEALTH CHECK RESULTS\n");

  let criticalIssues = 0;
  let warnings = 0;
  let passed = 0;

  checks.forEach((check) => {
    console.log(`${check.status} ${check.name}`);
    console.log(`   ${check.message}`);

    if (check.status === "✅") {
      passed++;
    } else if (check.status === "❌") {
      criticalIssues++;
    } else {
      warnings++;
    }
  });

  console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
  console.log(`Total Checks: ${checks.length}`);
  console.log(`✅ Passed: ${passed}`);
  console.log(`⚠️  Warnings: ${warnings}`);
  console.log(`❌ Critical Issues: ${criticalIssues}`);
  console.log();

  if (criticalIssues === 0) {
    console.log("✅ ALL CRITICAL CHECKS PASSED!\n");
    if (warnings > 0) {
      console.log(`⚠️  You have ${warnings} warning(s). Review them but system is operational.\n`);
    } else {
      console.log("🎉 Perfect health - all systems operational!\n");
    }
  } else {
    console.log(`❌ FOUND ${criticalIssues} CRITICAL ISSUE(S)!\n`);
    console.log("⚠️  Address critical issues immediately!\n");
  }

  console.log("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

  // Export report
  const fs = require("fs");
  const path = require("path");

  const reportData = {
    timestamp: new Date().toISOString(),
    network: isMainnet ? "mainnet" : "testnet",
    chainId: Number(network.chainId),
    blockNumber,
    contracts: {
      etrToken: etrAddress,
      masterChef: masterChefAddress,
    },
    summary: {
      total: checks.length,
      passed,
      warnings,
      criticalIssues,
      healthy: criticalIssues === 0,
    },
    checks: checks.map(c => ({
      name: c.name,
      status: c.status,
      message: c.message,
      critical: c.critical,
    })),
  };

  const outputPath = path.join(__dirname, `../health-report-${Date.now()}.json`);
  fs.writeFileSync(outputPath, JSON.stringify(reportData, null, 2));

  console.log(`📝 Report saved to: ${path.basename(outputPath)}\n`);

  // Exit with error code if critical issues found
  if (criticalIssues > 0) {
    process.exit(1);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("\n❌ Health check failed:");
    console.error(error);
    process.exit(1);
  });
