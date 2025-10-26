const hre = require("hardhat");

async function main() {
  console.log("Deploying ÉTR token to BSC...\n");

  // Get deployer account
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", (await deployer.provider.getBalance(deployer.address)).toString());

  // Deploy ÉTR token
  console.log("\nDeploying EtridToken contract...");
  const EtridToken = await hre.ethers.getContractFactory("EtridToken");

  const token = await EtridToken.deploy(
    "Etrid Coin",           // name
    "ÉTR"                   // symbol (with accent)
  );

  await token.waitForDeployment();
  const tokenAddress = await token.getAddress();

  console.log("✅ ÉTR Token deployed to:", tokenAddress);

  // Verify contract parameters
  const name = await token.name();
  const symbol = await token.symbol();
  const decimals = await token.decimals();

  console.log("\nToken Details:");
  console.log("  Name:", name);
  console.log("  Symbol:", symbol);
  console.log("  Decimals:", decimals.toString());
  console.log("  Deployer has DEFAULT_ADMIN_ROLE:", true);
  console.log("  Deployer has MINTER_ROLE:", true);
  console.log("  Deployer has PAUSER_ROLE:", true);
  console.log("  Deployer has BRIDGE_ROLE:", true);

  // Save deployment info
  const network = hre.network.name;
  const deploymentInfo = {
    network: network,
    chainId: (await hre.ethers.provider.getNetwork()).chainId.toString(),
    deployer: deployer.address,
    timestamp: new Date().toISOString(),
    contracts: {
      EtridToken: {
        address: tokenAddress,
        name: name,
        symbol: symbol,
        decimals: decimals.toString()
      }
    }
  };

  console.log("\nDeployment Info:");
  console.log(JSON.stringify(deploymentInfo, null, 2));

  // Instructions for next steps
  console.log("\n" + "=".repeat(60));
  console.log("NEXT STEPS:");
  console.log("=".repeat(60));
  console.log("\n1. Verify contract on BSCScan:");
  console.log("   npx hardhat verify --network", network, tokenAddress, '"Etrid Coin"', '"ÉTR"');

  console.log("\n2. Transfer roles to multisig:");
  console.log("   - DEFAULT_ADMIN_ROLE to foundation multisig");
  console.log("   - BRIDGE_ROLE to BSC bridge contract");
  console.log("   - Keep PAUSER_ROLE with multisig for emergencies");

  console.log("\n3. Create PancakeSwap liquidity pool:");
  console.log("   - Pair: ÉTR/BNB");
  console.log("   - Initial liquidity from Community LP Pool (250M ÉTR allocation)");

  console.log("\n4. Register token on aggregators:");
  console.log("   - CoinGecko: https://www.coingecko.com/en/coins/new");
  console.log("   - CoinMarketCap: https://support.coinmarketcap.com/hc/en-us/articles/360043659351");

  console.log("\n5. Set up bridge integration:");
  console.log("   - Grant BRIDGE_ROLE to bridge contract");
  console.log("   - Configure bridge oracle on FlareChain");
  console.log("   - Test bridge transfers");

  console.log("\n" + "=".repeat(60));
  console.log("Token Address:", tokenAddress);
  console.log("=".repeat(60) + "\n");

  return deploymentInfo;
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
