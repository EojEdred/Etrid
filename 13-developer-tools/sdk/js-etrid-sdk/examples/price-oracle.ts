/**
 * Oracle Price Feed Example
 *
 * Demonstrates how to:
 * 1. Get current prices from decentralized oracles
 * 2. Calculate time-weighted average prices (TWAP)
 * 3. Subscribe to real-time price updates
 * 4. Query historical price data
 * 5. Monitor oracle source health
 *
 * Ëtrid aggregates prices from multiple oracle sources for reliability.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import {
  OracleWrapper,
  toPrice,
  fromPrice,
  PRICE_PRECISION,
} from '../src/wrappers/OracleWrapper';

async function main() {
  // 1. Connect to Ëtrid node
  console.log('Connecting to Ëtrid FlareChain...');
  const provider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider });
  console.log('✓ Connected to chain:', (await api.rpc.system.chain()).toString());

  // 2. Create Oracle wrapper
  const oracle = new OracleWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Oracle Price System');
  console.log('═══════════════════════════════════════════\n');

  console.log('Price Precision:');
  console.log(`  • 18 decimals (${PRICE_PRECISION.toString()} precision)`);
  console.log(`  • Example: $42.50 = ${toPrice(42.50).toString()}`);

  console.log('\nHelper Functions:');
  console.log(`  • toPrice(42.50) → ${toPrice(42.50).toString()}`);
  console.log(`  • fromPrice(${toPrice(42.50).toString()}) → $${fromPrice(toPrice(42.50))}`);

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Get Current Prices');
  console.log('═══════════════════════════════════════════\n');

  const pairs = ['ETR/USD', 'BTC/USD', 'ETH/USD', 'SOL/USD'];

  for (const pair of pairs) {
    try {
      const priceData = await oracle.getPrice(pair);

      console.log(`${pair}:`);
      console.log(`  Price: $${priceData.priceFormatted.toFixed(2)}`);
      console.log(`  Confidence: ${priceData.confidence / 100}%`);
      console.log(`  Sources: ${priceData.sources}`);
      console.log(`  Deviation: ${priceData.deviation / 100}%`);
      console.log(`  Updated: ${new Date(priceData.timestamp).toLocaleString()}`);
      console.log();

    } catch (error) {
      console.log(`⚠ Mock price for ${pair}: $${Math.random() * 1000 + 50}`);
      console.log();
    }
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Calculate TWAP (24h)');
  console.log('═══════════════════════════════════════════\n');

  try {
    const now = Date.now();
    const dayAgo = now - 24 * 60 * 60 * 1000;

    const twap = await oracle.getTWAP({
      pair: 'ETR/USD',
      startTime: dayAgo,
      endTime: now,
      minDataPoints: 100,
    });

    console.log('ETR/USD 24-hour TWAP:');
    console.log(`  TWAP: $${twap.twapFormatted.toFixed(2)}`);
    console.log(`  Data points: ${twap.dataPoints}`);
    console.log(`  Min price: $${fromPrice(twap.minPrice).toFixed(2)}`);
    console.log(`  Max price: $${fromPrice(twap.maxPrice).toFixed(2)}`);
    console.log(`  Std deviation: ${twap.stdDeviation.toFixed(4)}`);
    console.log(`  Period: ${new Date(twap.startTime).toLocaleDateString()} - ${new Date(twap.endTime).toLocaleDateString()}`);

    const volatility = ((fromPrice(twap.maxPrice) - fromPrice(twap.minPrice)) / twap.twapFormatted * 100);
    console.log(`  Volatility: ${volatility.toFixed(2)}%`);

  } catch (error) {
    console.log('⚠ Mock TWAP calculation (for demonstration)');
    console.log('  TWAP: $52.35, Min: $49.20, Max: $55.80, Volatility: 12.6%');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Real-Time Price Subscription');
  console.log('═══════════════════════════════════════════\n');

  try {
    console.log('Subscribing to ETR/USD price updates...\n');

    const subscription = await oracle.subscribePriceUpdates(
      'ETR/USD',
      (event) => {
        const change = event.changePercent >= 0 ? '+' : '';
        const arrow = event.changePercent >= 0 ? '↑' : '↓';

        console.log(`[${new Date(event.timestamp).toLocaleTimeString()}] Price Update:`);
        console.log(`  New: $${fromPrice(event.price).toFixed(2)}`);
        console.log(`  Previous: $${fromPrice(event.previousPrice).toFixed(2)}`);
        console.log(`  Change: ${arrow} ${change}${event.changePercent.toFixed(2)}%`);
        console.log(`  Sources: ${event.sources.join(', ')}`);
        console.log();
      }
    );

    // Simulate receiving updates for 10 seconds
    console.log('Listening for 10 seconds...\n');
    await new Promise((resolve) => setTimeout(resolve, 10000));

    subscription.unsubscribe();
    console.log('✓ Unsubscribed from price updates');

  } catch (error) {
    console.log('⚠ Mock price subscription (for demonstration)');
    console.log('\nExample updates:');
    console.log('  [10:30:15] $52.30 → $52.45 (↑ +0.29%)');
    console.log('  [10:30:45] $52.45 → $52.38 (↓ -0.13%)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: Oracle Source Health');
  console.log('═══════════════════════════════════════════\n');

  try {
    const sources = await oracle.getPriceSources('ETR/USD');

    console.log(`Found ${sources.length} oracle sources:\n`);

    sources.forEach((source, i) => {
      const successRate = source.successfulUpdates / (source.successfulUpdates + source.failedUpdates) * 100;

      console.log(`${i + 1}. ${source.name}`);
      console.log(`   Status: ${source.status}`);
      console.log(`   Uptime: ${source.uptime / 100}%`);
      console.log(`   Weight: ${source.weight / 100}%`);
      console.log(`   Success rate: ${successRate.toFixed(2)}%`);
      console.log(`   Last update: ${new Date(source.lastUpdate).toLocaleString()}`);
      console.log();
    });

  } catch (error) {
    console.log('⚠ Mock source health (for demonstration)');
    console.log('\nExample sources:');
    console.log('  1. Chainlink (Active, 99.8% uptime, 40% weight)');
    console.log('  2. Band Protocol (Active, 99.5% uptime, 30% weight)');
    console.log('  3. DIA (Active, 98.9% uptime, 30% weight)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Historical Price Analysis');
  console.log('═══════════════════════════════════════════\n');

  try {
    const now = Date.now();
    const weekAgo = now - 7 * 24 * 60 * 60 * 1000;

    const history = await oracle.getHistoricalPrices(
      'ETR/USD',
      weekAgo,
      now,
      24 * 60 * 60 * 1000 // Daily intervals
    );

    console.log('ETR/USD Price History (Last 7 Days):\n');

    history.forEach((point) => {
      const date = new Date(point.timestamp).toLocaleDateString();
      console.log(`  ${date}: $${fromPrice(point.price).toFixed(2)}`);
    });

    if (history.length >= 2) {
      const firstPrice = fromPrice(history[0].price);
      const lastPrice = fromPrice(history[history.length - 1].price);
      const change = ((lastPrice - firstPrice) / firstPrice * 100);
      const arrow = change >= 0 ? '↑' : '↓';

      console.log(`\nWeekly change: ${arrow} ${change >= 0 ? '+' : ''}${change.toFixed(2)}%`);
    }

  } catch (error) {
    console.log('⚠ Mock historical data (for demonstration)');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Multi-Pair Dashboard');
  console.log('═══════════════════════════════════════════\n');

  const dashboard = [
    { pair: 'ETR/USD', price: 52.35, change24h: 2.3 },
    { pair: 'BTC/USD', price: 67250.00, change24h: -1.8 },
    { pair: 'ETH/USD', price: 3420.50, change24h: 3.5 },
    { pair: 'SOL/USD', price: 98.75, change24h: 5.2 },
    { pair: 'BNB/USD', price: 342.80, change24h: 1.1 },
  ];

  console.log('Price Dashboard:\n');
  console.log('Pair          Price         24h Change');
  console.log('─'.repeat(45));

  dashboard.forEach((item) => {
    const arrow = item.change24h >= 0 ? '↑' : '↓';
    const sign = item.change24h >= 0 ? '+' : '';
    const color = item.change24h >= 0 ? '' : '';

    console.log(
      `${item.pair.padEnd(12)}  $${item.price.toFixed(2).padStart(10)}    ${arrow} ${sign}${item.change24h.toFixed(2)}%`
    );
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('DeFi Use Cases');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Lending Protocols:');
  console.log('   • Determine collateral values');
  console.log('   • Calculate liquidation thresholds');
  console.log('   • Use TWAP to prevent flash loan attacks');

  console.log('\n2. DEX (Decentralized Exchanges):');
  console.log('   • Price discovery for swaps');
  console.log('   • Slippage calculations');
  console.log('   • Arbitrage detection');

  console.log('\n3. Stablecoins:');
  console.log('   • Peg maintenance');
  console.log('   • Collateral ratio monitoring');
  console.log('   • Redemption pricing');

  console.log('\n4. Options & Derivatives:');
  console.log('   • Strike price determination');
  console.log('   • Settlement pricing');
  console.log('   • Volatility calculations');

  console.log('\n═══════════════════════════════════════════');
  console.log('Oracle Security & Reliability');
  console.log('═══════════════════════════════════════════\n');

  console.log('Multi-Source Aggregation:');
  console.log('  ✓ Multiple independent oracle providers');
  console.log('  ✓ Weighted average based on reliability');
  console.log('  ✓ Outlier detection and removal');
  console.log('  ✓ Minimum source threshold required');

  console.log('\nManipulation Protection:');
  console.log('  ✓ TWAP for time-weighted prices');
  console.log('  ✓ Deviation alerts');
  console.log('  ✓ Source health monitoring');
  console.log('  ✓ Confidence scoring');

  console.log('\nFailover & Redundancy:');
  console.log('  ✓ Automatic source failover');
  console.log('  ✓ Degraded mode operation');
  console.log('  ✓ Historical data backup');
  console.log('  ✓ Manual override capability');

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Price Fetching:');
  console.log('   • Check confidence levels');
  console.log('   • Verify sufficient oracle sources');
  console.log('   • Monitor for stale prices');
  console.log('   • Use TWAP for critical operations');

  console.log('\n2. DeFi Integration:');
  console.log('   • Set appropriate deviation thresholds');
  console.log('   • Implement circuit breakers');
  console.log('   • Cache prices with TTL');
  console.log('   • Handle oracle failures gracefully');

  console.log('\n3. Risk Management:');
  console.log('   • Don\'t rely on single oracle source');
  console.log('   • Monitor price volatility');
  console.log('   • Set reasonable update intervals');
  console.log('   • Test with historical data');

  console.log('\n4. Cost Optimization:');
  console.log('   • Use queries (free) for reads');
  console.log('   • Batch price requests');
  console.log('   • Subscribe only when needed');
  console.log('   • Cache aggressively with invalidation');

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
