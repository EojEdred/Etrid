// Test script for XCM-enabled precompiles
// Run with: node test-xcm-precompiles.js

const { ethers } = require("hardhat");

// Precompile addresses
const ORACLE_ADDRESS = "0x0000000000000000000000000000000000000800";
const GOVERNANCE_ADDRESS = "0x0000000000000000000000000000000000000801";
const STAKING_ADDRESS = "0x0000000000000000000000000000000000000802";

// ABIs
const ORACLE_ABI = [
    "function getPriceInETH(bytes32 symbol) external view returns (uint256)",
    "function getPrice(bytes32 symbol, bytes32 quote) external view returns (uint256)",
    "function getLastUpdate(bytes32 symbol) external view returns (uint256)"
];

const GOVERNANCE_ABI = [
    "function submitProposal(string memory title, string memory description) external returns (uint256)",
    "function voteOnProposal(uint256 proposalId, bool support) external",
    "function getProposalStatus(uint256 proposalId) external view returns (uint8)"
];

const STAKING_ABI = [
    "function getValidatorStake(bytes32 validatorId) external view returns (uint256)",
    "function isValidatorActive(bytes32 validatorId) external view returns (bool)",
    "function getTotalStaked() external view returns (uint256)",
    "function getValidatorCount() external view returns (uint256)"
];

async function main() {
    console.log("🧪 Testing XCM-enabled Custom Precompiles\n");

    // Get signer
    const [signer] = await ethers.getSigners();
    console.log("Using account:", signer.address);
    console.log();

    // Test Oracle Precompile
    console.log("═══ Testing Oracle Precompile (0x800) ═══");
    const oracle = new ethers.Contract(ORACLE_ADDRESS, ORACLE_ABI, signer);

    try {
        console.log("Querying BTC price in ETH...");
        const btcPrice = await oracle.getPriceInETH(
            ethers.utils.formatBytes32String("BTC")
        );
        console.log("✅ BTC price:", ethers.utils.formatEther(btcPrice), "ETH");

        console.log("Querying ETH price in USD...");
        const ethPrice = await oracle.getPrice(
            ethers.utils.formatBytes32String("ETH"),
            ethers.utils.formatBytes32String("USD")
        );
        console.log("✅ ETH price: $", ethers.utils.formatEther(ethPrice));

        console.log("Checking last update...");
        const lastUpdate = await oracle.getLastUpdate(
            ethers.utils.formatBytes32String("BTC")
        );
        const date = new Date(lastUpdate.toNumber() * 1000);
        console.log("✅ Last update:", date.toISOString());
    } catch (error) {
        console.error("❌ Oracle test failed:", error.message);
    }
    console.log();

    // Test Governance Precompile
    console.log("═══ Testing Governance Precompile (0x801) ═══");
    const governance = new ethers.Contract(GOVERNANCE_ADDRESS, GOVERNANCE_ABI, signer);

    try {
        console.log("Submitting test proposal...");
        const tx = await governance.submitProposal(
            "Test Proposal",
            "This is a test proposal submitted via XCM precompile"
        );
        const receipt = await tx.wait();
        console.log("✅ Proposal submitted, tx:", receipt.transactionHash);

        // Extract proposal ID from events or return value
        const proposalId = 42; // Mock ID

        console.log("Checking proposal status...");
        const status = await governance.getProposalStatus(proposalId);
        const statusNames = ["Pending", "Active", "Passed", "Failed"];
        console.log("✅ Proposal status:", statusNames[status]);

        console.log("Voting on proposal...");
        const voteTx = await governance.voteOnProposal(proposalId, true);
        await voteTx.wait();
        console.log("✅ Vote cast: YES");
    } catch (error) {
        console.error("❌ Governance test failed:", error.message);
    }
    console.log();

    // Test Staking Precompile
    console.log("═══ Testing Staking Precompile (0x802) ═══");
    const staking = new ethers.Contract(STAKING_ADDRESS, STAKING_ABI, signer);

    try {
        console.log("Querying total staked...");
        const totalStaked = await staking.getTotalStaked();
        console.log("✅ Total staked:", ethers.utils.formatEther(totalStaked), "ETR");

        console.log("Querying validator count...");
        const count = await staking.getValidatorCount();
        console.log("✅ Validator count:", count.toString());

        // Test specific validator
        const testValidatorId = ethers.utils.formatBytes32String("validator1");
        console.log("Checking if validator is active...");
        const isActive = await staking.isValidatorActive(testValidatorId);
        console.log("✅ Validator active:", isActive);

        if (isActive) {
            console.log("Querying validator stake...");
            const stake = await staking.getValidatorStake(testValidatorId);
            console.log("✅ Validator stake:", ethers.utils.formatEther(stake), "ETR");
        }
    } catch (error) {
        console.error("❌ Staking test failed:", error.message);
    }
    console.log();

    console.log("🎉 All precompile tests completed!");
    console.log();
    console.log("Note: In production mode, these queries will trigger XCM messages");
    console.log("to FlareChain and wait for responses. Monitor XCM message delivery");
    console.log("in the relay chain explorer.");
}

main()
    .then(() => process.exit(0))
    .catch(error => {
        console.error(error);
        process.exit(1);
    });
