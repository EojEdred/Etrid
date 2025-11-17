#!/usr/bin/env node

/**
 * Ëtrid SDK Quick Start Script
 * 
 * This script helps you get started with the Ëtrid SDK quickly.
 * It creates a basic project structure with examples.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('\n🚀 Ëtrid SDK Quick Start\n');

// Check if we're in the right directory
const currentDir = process.cwd();
const packageJsonPath = path.join(currentDir, 'package.json');

if (!fs.existsSync(packageJsonPath)) {
  console.error('❌ Error: package.json not found');
  console.error('   Run this script from your project root\n');
  process.exit(1);
}

// Step 1: Install dependencies
console.log('📦 Step 1: Installing @etrid/sdk...\n');
try {
  execSync('npm install @etrid/sdk @polkadot/keyring', { stdio: 'inherit' });
  console.log('\n✅ Dependencies installed\n');
} catch (error) {
  console.error('❌ Failed to install dependencies');
  process.exit(1);
}

// Step 2: Create examples directory
console.log('📁 Step 2: Creating examples directory...\n');
const examplesDir = path.join(currentDir, 'etrid-examples');
if (!fs.existsSync(examplesDir)) {
  fs.mkdirSync(examplesDir);
}

// Step 3: Create example files
console.log('📝 Step 3: Creating example files...\n');

// Basic example
const basicExample = `/**
 * Basic Ëtrid SDK Example
 */

const { EtridClient, AccountsWrapper } = require('@etrid/sdk');
const { Keyring } = require('@polkadot/keyring');

async function main() {
  // Connect to Ëtrid node
  console.log('Connecting to Ëtrid node...');
  const client = new EtridClient('ws://127.0.0.1:9944');
  
  console.log('Connected to:', client.getChain());
  console.log('Block:', await client.getBlockNumber());
  
  // Create accounts
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  
  console.log('\\nAlice address:', alice.address);
  
  // Get balance
  const accounts = new AccountsWrapper(client.api);
  const balance = await accounts.getBalance(alice.address);
  
  console.log('Balance:', balance.free / 10n**18n, 'ÉTR');
  
  // Cleanup
  client.close();
  console.log('\\n✅ Example completed!');
}

main().catch(console.error);
`;

fs.writeFileSync(
  path.join(examplesDir, '01-basic.js'),
  basicExample
);

// Lightning-Bloc example
const lightningExample = `/**
 * Lightning-Bloc Payment Channel Example
 */

const { EtridClient, LightningBlocWrapper } = require('@etrid/sdk');
const { Keyring } = require('@polkadot/keyring');

async function main() {
  const client = new EtridClient('ws://127.0.0.1:9944');
  const lightning = new LightningBlocWrapper(client.api);
  
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  
  console.log('Opening payment channel...');
  const channel = await lightning.openChannel(
    alice,
    bob.address,
    1000n * 10n**18n  // 1000 ÉTR
  );
  
  console.log('Channel ID:', channel.channelId);
  console.log('Capacity:', channel.amount / 10n**18n, 'ÉTR');
  
  console.log('\\nSending payment...');
  await lightning.sendPayment(
    alice,
    channel.channelId,
    100n * 10n**18n  // 100 ÉTR
  );
  
  console.log('Payment sent!');
  
  console.log('\\nClosing channel...');
  await lightning.closeChannel(alice, channel.channelId);
  
  client.close();
  console.log('\\n✅ Channel closed!');
}

main().catch(console.error);
`;

fs.writeFileSync(
  path.join(examplesDir, '02-lightning-bloc.js'),
  lightningExample
);

// Staking example
const stakingExample = `/**
 * Staking Example
 */

const { EtridClient, StakingWrapper } = require('@etrid/sdk');
const { Keyring } = require('@polkadot/keyring');

async function main() {
  const client = new EtridClient('ws://127.0.0.1:9944');
  const staking = new StakingWrapper(client.api);
  
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  
  // Get validators
  console.log('Getting validators...');
  const validators = await staking.getValidators();
  console.log('Active validators:', validators.length);
  
  // Get staking info
  const info = await staking.getStakingInfo(alice.address);
  console.log('\\nStaking info:');
  console.log('  Staked:', info.staked / 10n**18n, 'ÉTR');
  console.log('  Status:', info.status);
  
  // Estimate rewards
  const estimate = await staking.estimateRewards(1000n * 10n**18n);
  console.log('\\nReward estimates for 1000 ÉTR:');
  console.log('  APY:', estimate.apy, '%');
  console.log('  Daily:', estimate.daily / 10n**18n, 'ÉTR');
  console.log('  Monthly:', estimate.monthly / 10n**18n, 'ÉTR');
  console.log('  Yearly:', estimate.yearly / 10n**18n, 'ÉTR');
  
  client.close();
  console.log('\\n✅ Example completed!');
}

main().catch(console.error);
`;

fs.writeFileSync(
  path.join(examplesDir, '03-staking.js'),
  stakingExample
);

// Create README
const readmeContent = `# Ëtrid SDK Examples

Welcome! These examples will help you get started with the Ëtrid SDK.

## Prerequisites

Make sure you have a running Ëtrid node:

\`\`\`bash
# Start local dev node
./target/release/flarechain-node --dev
\`\`\`

## Running Examples

\`\`\`bash
# Basic example
node etrid-examples/01-basic.js

# Lightning-Bloc payment channels
node etrid-examples/02-lightning-bloc.js

# Staking
node etrid-examples/03-staking.js
\`\`\`

## What's Next?

Check out the full documentation:
- [Getting Started Tutorial](../node_modules/@etrid/sdk/docs/tutorials/01-getting-started.md)
- [Advanced Features](../node_modules/@etrid/sdk/docs/tutorials/02-advanced-features.md)
- [API Documentation](https://docs.etrid.io/sdk)

## Need Help?

- Discord: https://discord.gg/etrid
- GitHub: https://github.com/etrid/etrid-protocol
- Docs: https://docs.etrid.io

Happy building! 🚀
`;

fs.writeFileSync(
  path.join(examplesDir, 'README.md'),
  readmeContent
);

console.log('✅ Created examples:');
console.log('   - etrid-examples/01-basic.js');
console.log('   - etrid-examples/02-lightning-bloc.js');
console.log('   - etrid-examples/03-staking.js');
console.log('   - etrid-examples/README.md\n');

// Step 4: Display next steps
console.log('🎉 Quick start complete!\n');
console.log('Next steps:\n');
console.log('1. Start an Ëtrid node:');
console.log('   ./target/release/flarechain-node --dev\n');
console.log('2. Run an example:');
console.log('   node etrid-examples/01-basic.js\n');
console.log('3. Explore the documentation:');
console.log('   https://docs.etrid.io/sdk\n');
console.log('Happy building! 🚀\n');
