/**
 * Cross-Chain Bridge Example
 *
 * Demonstrates how to:
 * 1. Bridge tokens across 13 supported blockchains
 * 2. Monitor bridge transaction status
 * 3. Check bridge health and limits
 * 4. Estimate fees and confirmations
 * 5. View bridge history
 *
 * Ëtrid connects BTC, ETH, SOL, XRP, BNB, TRX, ADA, MATIC, LINK, DOGE, XLM, USDT, EDSC
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  BridgeWrapper,
  Chain,
  BridgeStatus,
} from '../src/wrappers/BridgeWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid FlareChain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Initialize account
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  console.log(`\nAccount: ${alice.address}`);

  // 3. Create Bridge wrapper
  const bridge = new BridgeWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Supported Blockchains');
  console.log('═══════════════════════════════════════════\n');

  const chains = await bridge.getSupportedChains();
  console.log(`Total chains supported: ${chains.length}\n`);

  for (const chain of chains) {
    const metadata = await bridge.getChainMetadata(chain);
    console.log(`${chain.padEnd(6)}: ${metadata.name.padEnd(20)} | ${metadata.symbol.padEnd(6)} | ${metadata.decimals} decimals`);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Estimate Bridge Fee');
  console.log('═══════════════════════════════════════════\n');

  try {
    // Estimate fee for bridging 1 BTC to Ethereum
    const fee = await bridge.estimateBridgeFee(
      Chain.BTC,
      Chain.ETH,
      100_000_000n // 1 BTC (8 decimals)
    );

    console.log('Bridge 1 BTC → ETH:');
    console.log(`  Base fee: ${Number(fee.baseFee) / 1e8} BTC`);
    console.log(`  Variable fee: ${Number(fee.variableFee) / 1e8} BTC (${fee.feePercentage / 100}%)`);
    console.log(`  Total fee: ${Number(fee.totalFee) / 1e8} BTC`);
    console.log(`  You receive: ${Number(fee.amountAfterFee) / 1e8} BTC worth of ETH`);
    console.log(`  Estimated time: ${fee.estimatedTime} seconds`);

  } catch (error) {
    console.log('⚠ Mock fee estimation (for demonstration)');
    console.log('  Typical fees: 0.1-0.5% + base fee');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Bridge Tokens');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Bridging 10 SOL → BNB Chain...');

    const bridgeTx = await bridge.bridgeTokens(
      alice,
      Chain.SOL,
      Chain.BNB,
      10n * 10n**9n, // 10 SOL (9 decimals)
      '0xYourBNBAddress1234567890abcdef...' // Recipient on BNB Chain
    );

    console.log(`\n✓ Bridge transaction initiated!`);
    console.log(`  Bridge TX ID: ${bridgeTx.id}`);
    console.log(`  Source: ${bridgeTx.sourceChain}`);
    console.log(`  Target: ${bridgeTx.targetChain}`);
    console.log(`  Amount: ${Number(bridgeTx.amount) / 1e9} SOL`);
    console.log(`  Status: ${bridgeTx.status}`);
    console.log(`  Confirmations: ${bridgeTx.confirmations}/${bridgeTx.requiredConfirmations}`);
    console.log(`  Fee: ${Number(bridgeTx.fee) / 1e9} SOL`);

    if (bridgeTx.estimatedCompletion) {
      console.log(`  ETA: ${bridgeTx.estimatedCompletion.toLocaleString()}`);
    }

  } catch (error) {
    console.log('⚠ Mock bridge transaction (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Monitor Bridge Status');
  console.log('═══════════════════════════════════════════\n');

  const mockTxHash = 'bridge_tx_123456789abcdef';

  try {
    console.log(`Monitoring transaction: ${mockTxHash}...\n`);

    // Poll for status updates
    for (let i = 0; i < 5; i++) {
      const status = await bridge.getBridgeStatus(mockTxHash);

      console.log(`[${new Date().toLocaleTimeString()}] Status update:`);
      console.log(`  Status: ${status.status}`);
      console.log(`  Confirmations: ${status.confirmations}/${status.requiredConfirmations}`);

      const progress = (status.confirmations / status.requiredConfirmations * 100).toFixed(1);
      console.log(`  Progress: ${'█'.repeat(Math.floor(Number(progress) / 5))}${'░'.repeat(20 - Math.floor(Number(progress) / 5))} ${progress}%`);

      if (status.status === BridgeStatus.Completed) {
        console.log(`  ✓ Bridge completed!`);
        break;
      }

      if (status.status === BridgeStatus.Failed) {
        console.log(`  ✗ Bridge failed!`);
        break;
      }

      console.log();
      await new Promise((resolve) => setTimeout(resolve, 2000));
    }

  } catch (error) {
    console.log('⚠ Mock status monitoring (for demonstration)');
    console.log('\nTypical status flow:');
    console.log('  Pending → Confirming → Relaying → Completed');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: Check Bridge Limits');
  console.log('═══════════════════════════════════════════\n');

  try {
    const limits = await bridge.getBridgeLimits(Chain.ETH, Chain.BTC);

    console.log('ETH → BTC Bridge Limits:');
    console.log(`  Minimum: ${Number(limits.minAmount) / 1e18} ETH`);
    console.log(`  Maximum: ${Number(limits.maxAmount) / 1e18} ETH`);
    console.log(`  Daily limit: ${Number(limits.dailyLimit) / 1e18} ETH`);
    console.log(`  Daily used: ${Number(limits.dailyUsed) / 1e18} ETH`);
    console.log(`  Available today: ${Number(limits.dailyLimit - limits.dailyUsed) / 1e18} ETH`);

  } catch (error) {
    console.log('⚠ Mock limit query (for demonstration)');
    console.log('  Example: Min 0.01 ETH, Max 100 ETH, Daily 1000 ETH');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: View Bridge History');
  console.log('═══════════════════════════════════════════\n');

  try {
    const history = await bridge.getBridgeHistory(
      alice.address,
      undefined, // All chains
      10 // Last 10 transactions
    );

    if (history.length > 0) {
      console.log(`Recent bridge transactions:\n`);

      history.forEach((tx, i) => {
        console.log(`${i + 1}. ${tx.sourceChain} → ${tx.targetChain}`);
        console.log(`   Amount: ${tx.amount}`);
        console.log(`   Status: ${tx.status}`);
        console.log(`   TX: ${tx.id}`);
        console.log();
      });
    } else {
      console.log('No bridge history yet');
    }

  } catch (error) {
    console.log('⚠ Mock history query (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Multi-Chain Strategy');
  console.log('═══════════════════════════════════════════\n');

  const routes = [
    { from: 'BTC', to: 'ETH', use: 'DeFi access', time: '~30 min', fee: '0.3%' },
    { from: 'ETH', to: 'BNB', use: 'Lower fees', time: '~5 min', fee: '0.1%' },
    { from: 'SOL', to: 'ETH', use: 'NFTs, DeFi', time: '~10 min', fee: '0.2%' },
    { from: 'XRP', to: 'USDT', use: 'Stablecoin', time: '~3 min', fee: '0.15%' },
    { from: 'ADA', to: 'MATIC', use: 'Scaling', time: '~15 min', fee: '0.25%' },
  ];

  console.log('Popular bridge routes:\n');
  routes.forEach((route, i) => {
    console.log(`${i + 1}. ${route.from} → ${route.to}`);
    console.log(`   Use case: ${route.use}`);
    console.log(`   Time: ${route.time} | Fee: ${route.fee}`);
    console.log();
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('Bridge Architecture');
  console.log('═══════════════════════════════════════════\n');

  console.log('PBC (Partition Burst Chain) Model:');
  console.log('  • Each supported chain has dedicated PBC');
  console.log('  • 13 PBCs process cross-chain transactions');
  console.log('  • Combined throughput: 70,000+ TPS');
  console.log('  • Isolated security per chain');

  console.log('\nSecurity:');
  console.log('  ✓ Multi-signature validation');
  console.log('  ✓ Time-locked transfers');
  console.log('  ✓ Fraud proof system');
  console.log('  ✓ Independent validator sets');

  console.log('\n═══════════════════════════════════════════');
  console.log('Confirmation Requirements');
  console.log('═══════════════════════════════════════════\n');

  const confirmations = [
    { chain: 'BTC', blocks: 6, time: '~60 min' },
    { chain: 'ETH', blocks: 12, time: '~3 min' },
    { chain: 'SOL', blocks: 32, time: '~15 sec' },
    { chain: 'BNB', blocks: 15, time: '~45 sec' },
    { chain: 'MATIC', blocks: 128, time: '~5 min' },
    { chain: 'XRP', blocks: 1, time: '~4 sec' },
  ];

  confirmations.forEach((conf) => {
    console.log(`${conf.chain.padEnd(6)}: ${conf.blocks.toString().padStart(3)} blocks ≈ ${conf.time}`);
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Before Bridging:');
  console.log('   • Check recipient address carefully');
  console.log('   • Verify bridge limits and fees');
  console.log('   • Ensure sufficient balance for fees');
  console.log('   • Test with small amount first');

  console.log('\n2. During Transfer:');
  console.log('   • Monitor confirmation progress');
  console.log('   • Don\'t close browser/app');
  console.log('   • Save transaction hash');
  console.log('   • Wait for full confirmations');

  console.log('\n3. After Completion:');
  console.log('   • Verify receipt on target chain');
  console.log('   • Check amounts match (minus fees)');
  console.log('   • Keep records for accounting');

  console.log('\n4. Troubleshooting:');
  console.log('   • If stuck, check bridge status');
  console.log('   • Contact support with TX hash');
  console.log('   • Don\'t attempt duplicate transfer');
  console.log('   • Bridges can take time - be patient');

  // Cleanup
  await api.disconnect();
  console.log('\n✓ Disconnected from chain');
}

// Run example
main()
  .then(() => {
    console.log('\n✅ Example completed successfully!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('\n❌ Example failed:', error);
    process.exit(1);
  });
