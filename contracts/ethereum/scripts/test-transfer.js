const hre = require("hardhat");
const fs = require("fs");

async function main() {
  console.log("🚀 Testing EDSC Cross-Chain Bridge");
  console.log("=" .repeat(60), "\n");

  // Load deployment
  const files = fs.readdirSync(".").filter(f => f.startsWith("deployment-localhost"));
  const latestFile = files.sort().reverse()[0];
  const deployment = JSON.parse(fs.readFileSync(latestFile, "utf8"));

  const edscAddress = deployment.contracts.EDSC;
  const tokenMessengerAddress = deployment.contracts.TokenMessenger;

  console.log("Contract Addresses:");
  console.log("  EDSC Token:", edscAddress);
  console.log("  Token Messenger:", tokenMessengerAddress);
  console.log("");

  // Get contracts
  const EDSC = await hre.ethers.getContractFactory("EDSC");
  const edsc = EDSC.attach(edscAddress);

  const TokenMessenger = await hre.ethers.getContractFactory("EDSCTokenMessenger");
  const messenger = TokenMessenger.attach(tokenMessengerAddress);

  const [sender] = await hre.ethers.getSigners();
  console.log("Sender Account:", sender.address);
  console.log("");

  // 1. Mint EDSC to sender (using testMint for local testing)
  console.log("Step 1: Minting 1000 EDSC to sender...");
  const mintTx = await edsc.testMint(sender.address, hre.ethers.parseUnits("1000", 18));
  await mintTx.wait();
  console.log("  ✓ Minted 1000 EDSC");

  // Check balance
  const balance = await edsc.balanceOf(sender.address);
  console.log("  Current Balance:", hre.ethers.formatUnits(balance, 18), "EDSC\n");

  // 2. Approve TokenMessenger
  console.log("Step 2: Approving TokenMessenger to spend 100 EDSC...");
  const approveTx = await edsc.approve(
    tokenMessengerAddress,
    hre.ethers.parseUnits("100", 18)
  );
  await approveTx.wait();
  console.log("  ✓ Approved\n");

  // 3. Initiate cross-chain transfer
  console.log("Step 3: Initiating cross-chain transfer...");
  console.log("  Amount: 100 EDSC");
  console.log("  From: Ethereum (domain 0)");
  console.log("  To: Ëtrid Substrate (domain 2)");

  // 32-byte recipient address (Substrate account)
  const recipientSubstrate = "0x1234567890123456789012345678901234567890123456789012345678901234";
  const amount = hre.ethers.parseUnits("100", 18);
  const destinationDomain = 2; // Ëtrid

  console.log("  Recipient:", recipientSubstrate);
  console.log("");

  const transferTx = await messenger.burnAndSendTo(
    destinationDomain,
    recipientSubstrate,
    amount
  );
  console.log("  ⏳ Waiting for transaction confirmation...");

  const receipt = await transferTx.wait();
  console.log("  ✓ Transaction confirmed!");
  console.log("  TX Hash:", receipt.hash);
  console.log("  Block:", receipt.blockNumber);
  console.log("");

  // Find MessageSent event
  const event = receipt.logs.find(log => {
    try {
      const parsed = messenger.interface.parseLog(log);
      return parsed && parsed.name === "MessageSent";
    } catch {
      return false;
    }
  });

  if (event) {
    const parsed = messenger.interface.parseLog(event);
    console.log("📨 Message Sent Event:");
    console.log("  Nonce:", parsed.args.nonce.toString());
    console.log("  Amount:", hre.ethers.formatUnits(parsed.args.amount, 18), "EDSC");
    console.log("  Destination Domain:", parsed.args.destinationDomain.toString());
    console.log("");
  }

  // Check new balance
  const newBalance = await edsc.balanceOf(sender.address);
  console.log("Final Balance:", hre.ethers.formatUnits(newBalance, 18), "EDSC");
  console.log("");

  console.log("=" .repeat(60));
  console.log("✅ Transfer initiated successfully!");
  console.log("=" .repeat(60));
  console.log("");
  console.log("Next Steps:");
  console.log("1. Attestation service should detect the burn event");
  console.log("2. After collecting signatures, message becomes 'ready'");
  console.log("3. Relayer will automatically relay to Substrate");
  console.log("");
  console.log("Monitor progress:");
  console.log("  Attestation service: http://localhost:3000/stats");
  console.log("  Relayer service: http://localhost:3001/health");
  console.log("");
  console.log("Expected timeline:");
  console.log("  - Attestation detection: ~5 seconds");
  console.log("  - Signature collection: ~10-30 seconds (3-of-5)");
  console.log("  - Relay to Substrate: ~10 seconds");
  console.log("  - Total: ~30-60 seconds");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
