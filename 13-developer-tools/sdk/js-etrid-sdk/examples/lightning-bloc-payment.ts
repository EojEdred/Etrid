/**
 * Lightning-Bloc Payment Channel Example
 *
 * Demonstrates how to:
 * 1. Open a payment channel
 * 2. Route payments through the network
 * 3. Update channel state off-chain
 * 4. Close a channel
 *
 * Lightning-Bloc enables 500K+ TPS instant payments with minimal fees.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { LightningBlocWrapper } from '../src/wrappers/LightningBlocWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid FlareChain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Initialize accounts
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  const charlie = keyring.addFromUri('//Charlie');

  console.log(`\nAlice: ${alice.address}`);
  console.log(`Bob: ${bob.address}`);
  console.log(`Charlie: ${charlie.address}`);

  // 3. Create Lightning-Bloc wrapper
  const lightning = new LightningBlocWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Open a Payment Channel');
  console.log('═══════════════════════════════════════════\n');

  try {
    // Open channel: Alice → Bob
    // Both parties deposit 100 ÉTR each
    const channelId = await lightning.openChannel(
      alice,
      bob.address,
      100n * 10n**18n, // Alice deposits 100 ÉTR
      100n * 10n**18n, // Bob deposits 100 ÉTR
      86400 * 30       // 30 days duration
    );

    console.log(`✓ Channel opened: ${channelId}`);

    // Query channel details
    const channel = await lightning.getChannel(channelId);
    console.log('\nChannel Details:');
    console.log(`  Participants: ${channel.participants[0]} ↔ ${channel.participants[1]}`);
    console.log(`  Total Balance: ${channel.totalBalance / 10n**18n} ÉTR`);
    console.log(`  Status: ${channel.status}`);
    console.log(`  Duration: ${channel.duration} seconds`);

    // Check channel balance
    const balance = await lightning.getChannelBalance(channelId);
    console.log('\nChannel Balances:');
    console.log(`  Alice: ${balance.myBalance / 10n**18n} ÉTR`);
    console.log(`  Bob: ${balance.theirBalance / 10n**18n} ÉTR`);

  } catch (error) {
    console.error('Error opening channel:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Make Instant Off-Chain Payments');
  console.log('═══════════════════════════════════════════\n');

  try {
    // Get Alice's channels
    const aliceChannels = await lightning.getMyChannels(alice.address);

    if (aliceChannels.length > 0) {
      const channelId = aliceChannels[0].channelId;

      // Alice pays Bob 10 ÉTR off-chain
      console.log('Paying 10 ÉTR from Alice to Bob...');

      // In a real implementation, this would involve:
      // 1. Creating a signed state update
      // 2. Sending to counterparty
      // 3. Both parties signing
      // 4. Updating channel state

      // For demonstration:
      const payment = {
        amount: 10n * 10n**18n,
        nonce: 1,
        recipient: bob.address,
      };

      console.log(`✓ Payment initiated: ${payment.amount / 10n**18n} ÉTR`);
      console.log('  This happens instantly off-chain!');
      console.log('  No blockchain fees, no confirmation wait');
    }
  } catch (error) {
    console.error('Error making payment:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Multi-Hop Payment Routing');
  console.log('═══════════════════════════════════════════\n');

  try {
    // Route payment: Alice → Bob → Charlie
    console.log('Finding route from Alice to Charlie...');

    const route = await lightning.routePayment(
      alice,
      charlie.address,
      5n * 10n**18n, // 5 ÉTR
      10              // Max 10 hops
    );

    console.log(`✓ Route found with ${route.hops.length} hops:`);
    route.hops.forEach((hop, i) => {
      console.log(`  ${i + 1}. ${hop.from} → ${hop.to}: ${hop.amount / 10n**18n} ÉTR (fee: ${hop.fee / 10n**18n} ÉTR)`);
    });
    console.log(`\nTotal fee: ${route.totalFee / 10n**18n} ÉTR`);
    console.log(`Total amount: ${route.totalAmount / 10n**18n} ÉTR`);
    console.log(`Estimated time: ${route.estimatedTimeMs}ms`);

  } catch (error) {
    console.error('Error routing payment:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: Estimate Routing Fees');
  console.log('═══════════════════════════════════════════\n');

  try {
    const aliceChannels = await lightning.getMyChannels(alice.address);

    if (aliceChannels.length > 0) {
      const channelId = aliceChannels[0].channelId;

      // Estimate fee for 50 ÉTR payment
      const estimate = await lightning.estimateRoutingFee(
        channelId,
        50n * 10n**18n
      );

      console.log('Fee estimate for 50 ÉTR payment:');
      console.log(`  Base fee: ${estimate.baseFee / 10n**18n} ÉTR`);
      console.log(`  Proportional fee: ${estimate.proportionalFee / 10n**18n} ÉTR`);
      console.log(`  Total fee: ${estimate.totalFee / 10n**18n} ÉTR`);
      console.log(`  Fee percentage: ${(Number(estimate.totalFee) / Number(50n * 10n**18n) * 100).toFixed(4)}%`);
    }
  } catch (error) {
    console.error('Error estimating fee:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Close Channel Cooperatively');
  console.log('═══════════════════════════════════════════\n');

  try {
    const aliceChannels = await lightning.getMyChannels(alice.address);

    if (aliceChannels.length > 0) {
      const channelId = aliceChannels[0].channelId;

      console.log(`Closing channel ${channelId}...`);

      const txHash = await lightning.closeChannel(alice, channelId);

      console.log(`✓ Channel closed cooperatively`);
      console.log(`  Transaction: ${txHash}`);
      console.log('  Final balances returned to both parties');
    }
  } catch (error) {
    console.error('Error closing channel:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Force Close (Dispute Resolution)');
  console.log('═══════════════════════════════════════════\n');

  try {
    // This example shows how to force-close if counterparty is unresponsive
    console.log('Force close scenario:');
    console.log('  Use this if counterparty is offline or uncooperative');
    console.log('  Submit latest signed state to chain');
    console.log('  Wait for challenge period (typically 24 hours)');
    console.log('  Claim funds after challenge period');

    // In real usage:
    // const txHash = await lightning.forceClose(
    //   alice,
    //   channelId,
    //   latestNonce,
    //   latestBalances,
    //   counterpartySignature
    // );

    console.log('\n  Note: Always try cooperative close first!');
    console.log('  Force close has longer settlement time');

  } catch (error) {
    console.error('Error in force close example:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Performance Comparison');
  console.log('═══════════════════════════════════════════\n');

  console.log('Lightning-Bloc Payments:');
  console.log('  ✓ Speed: Instant (< 100ms)');
  console.log('  ✓ Throughput: 500,000+ TPS');
  console.log('  ✓ Fees: ~0.001% (nearly free)');
  console.log('  ✓ Finality: Immediate off-chain');
  console.log('  ✓ Privacy: Off-chain transactions');

  console.log('\nRegular On-Chain Transfers:');
  console.log('  • Speed: 6 seconds (block time)');
  console.log('  • Throughput: 1,000 TPS (FlareChain)');
  console.log('  • Fees: ~0.01 ÉTR per transaction');
  console.log('  • Finality: 1-2 blocks');
  console.log('  • Privacy: Fully public');

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Channel Management:');
  console.log('   • Open channels with frequent trading partners');
  console.log('   • Keep channels balanced for routing');
  console.log('   • Monitor channel expiration times');

  console.log('\n2. Payment Routing:');
  console.log('   • Set reasonable max hops (5-10)');
  console.log('   • Check fee estimates before large payments');
  console.log('   • Have backup routes for critical payments');

  console.log('\n3. Security:');
  console.log('   • Always keep latest signed state backups');
  console.log('   • Monitor channels for force-close attempts');
  console.log('   • Use watchtower services for offline protection');

  console.log('\n4. Economics:');
  console.log('   • Balance channel deposits appropriately');
  console.log('   • Consider channel duration vs. deposit size');
  console.log('   • Set competitive routing fees to earn from forwards');

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
