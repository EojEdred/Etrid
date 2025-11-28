const hre = require("hardhat");
const fs = require("fs");

async function main() {
  console.log("Authorizing TokenMessenger to burn EDSC...\n");

  // Load deployment
  const files = fs.readdirSync(".").filter(f => f.startsWith("deployment-localhost"));
  const latestFile = files.sort().reverse()[0];
  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));

  const edscAddress = deployment.contracts.EDSC;
  const tokenMessengerAddress = deployment.contracts.TokenMessenger;

  console.log("EDSC Token:", edscAddress);
  console.log("TokenMessenger:", tokenMessengerAddress);
  console.log("");

  // Get EDSC contract
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = EDSC.attach(edscAddress);

  // Set TokenMessenger as authorized
  const tx = await edsc.setMessageTransmitter(tokenMessengerAddress);
  await tx.wait();

  console.log("✓ TokenMessenger authorized to burn/mint EDSC");
  console.log("");

  // Verify
  const authorizedAddress = await edsc.messageTransmitter();
  console.log("Verification:");
  console.log("  Authorized address:", authorizedAddress);
  console.log("  TokenMessenger:", tokenMessengerAddress);
  console.log("  Match:", authorizedAddress === tokenMessengerAddress ? "✓" : "✗");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
