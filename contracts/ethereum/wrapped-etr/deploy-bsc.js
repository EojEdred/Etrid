const hre = require("hardhat");

async function main() {
  console.log("Starting WrappedETR deployment to BSC...");
  console.log("Network:", hre.network.name);

  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);

  const balance = await deployer.getBalance();
  console.log("Account balance:", hre.ethers.utils.formatEther(balance), "BNB");

  // Deploy WrappedETR
  console.log("\nDeploying WrappedETR...");
  const WrappedETR = await hre.ethers.getContractFactory("WrappedETR");

  // Admin address - change this to your desired admin address
  // For production, use a multisig wallet
  const adminAddress = deployer.address;
  console.log("Admin address:", adminAddress);

  const wrappedETR = await WrappedETR.deploy(adminAddress);
  await wrappedETR.deployed();

  console.log("\n=================================");
  console.log("WrappedETR deployed to:", wrappedETR.address);
  console.log("=================================");

  // Wait for a few block confirmations
  console.log("\nWaiting for 5 block confirmations...");
  await wrappedETR.deployTransaction.wait(5);

  console.log("\nDeployment complete!");
  console.log("\nContract details:");
  console.log("- Address:", wrappedETR.address);
  console.log("- Admin:", adminAddress);
  console.log("- Name:", await wrappedETR.name());
  console.log("- Symbol:", await wrappedETR.symbol());
  console.log("- Decimals:", await wrappedETR.decimals());

  // Verify contract on BscScan
  if (hre.network.name !== "hardhat" && hre.network.name !== "localhost") {
    console.log("\n=================================");
    console.log("Verifying contract on BscScan...");
    console.log("=================================");

    try {
      await hre.run("verify:verify", {
        address: wrappedETR.address,
        constructorArguments: [adminAddress],
      });
      console.log("Contract verified successfully!");
    } catch (error) {
      if (error.message.includes("Already Verified")) {
        console.log("Contract already verified!");
      } else {
        console.error("Error verifying contract:", error);
        console.log("\nYou can verify manually with:");
        console.log(`npx hardhat verify --network ${hre.network.name} ${wrappedETR.address} ${adminAddress}`);
      }
    }
  }

  console.log("\n=================================");
  console.log("NEXT STEPS:");
  console.log("=================================");
  console.log("1. Save the contract address:", wrappedETR.address);
  console.log("2. Add bridge address using: addBridge(bridgeAddress)");
  console.log("3. Test minting with the bridge");
  console.log("4. Consider transferring admin to a multisig wallet");
  console.log("\nExample commands:");
  console.log(`npx hardhat console --network ${hre.network.name}`);
  console.log(`const token = await ethers.getContractAt("WrappedETR", "${wrappedETR.address}")`);
  console.log(`await token.addBridge("BRIDGE_ADDRESS_HERE")`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
