const hre = require("hardhat");

async function main() {
  console.log("Checking ETR balance on Polygon...\n");

  const tokenAddress = "0x1A065196152C2A70e54AC06D3a3433e3D8606eF3";
  const walletAddress = "0x36F94145F89F572d55a66743904E29d5FDC22497";

  const token = await hre.ethers.getContractAt("EtridPoly", tokenAddress);

  const name = await token.name();
  const symbol = await token.symbol();
  const decimals = await token.decimals();
  const totalSupply = await token.totalSupply();
  const balance = await token.balanceOf(walletAddress);
  const owner = await token.owner();

  console.log("Token Info:");
  console.log("  Name:", name);
  console.log("  Symbol:", symbol);
  console.log("  Decimals:", decimals);
  console.log("  Total Supply:", hre.ethers.formatEther(totalSupply), "ETR");
  console.log("\nWallet:", walletAddress);
  console.log("  Balance:", hre.ethers.formatEther(balance), "ETR");
  console.log("\nContract Owner:", owner);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
