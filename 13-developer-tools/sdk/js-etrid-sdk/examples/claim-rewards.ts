/**
 * Distribution Pay Rewards Example
 *
 * Demonstrates how to:
 * 1. Check eligibility for daily rewards
 * 2. Query pending rewards across all categories
 * 3. Claim rewards from different categories
 * 4. View claim history
 * 5. Estimate future distributions
 *
 * Ëtrid distributes 27,397 ÉTR daily (10M ÉTR/year) to network participants.
 */

import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  DistributionPayWrapper,
  DistributionCategory,
} from '../src/wrappers/DistributionPayWrapper';

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

  // 3. Create DistributionPay wrapper
  const distribution = new DistributionPayWrapper(api);

  console.log('\n═══════════════════════════════════════════');
  console.log('Daily Distribution Schedule');
  console.log('═══════════════════════════════════════════\n');

  // Get distribution schedule
  const schedule = await distribution.getDistributionSchedule();

  console.log(`Total Daily Distribution: ${schedule.totalDailyDistribution / 10n**18n} ÉTR`);
  console.log(`Annual Distribution: ${schedule.totalDailyDistribution * 365n / 10n**18n} ÉTR`);
  console.log(`Distribution Period: Every ${schedule.distributionPeriod} seconds`);
  console.log(`Next Distribution: ${new Date(schedule.nextDistribution).toLocaleString()}`);

  console.log('\nCategory Breakdown:');
  Object.entries(schedule.categoryAllocations).forEach(([category, amount]) => {
    const etr = amount / 10n**18n;
    const percentage = (Number(amount) / Number(schedule.totalDailyDistribution) * 100).toFixed(1);
    console.log(`  ${category.padEnd(15)}: ${etr.toString().padStart(6)} ÉTR/day (${percentage}%)`);
  });

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 1: Check Eligibility');
  console.log('═══════════════════════════════════════════\n');

  // Check eligibility for each category
  const categories = Object.values(DistributionCategory);

  for (const category of categories) {
    const eligible = await distribution.isEligible(alice.address, category);
    const status = eligible ? '✓ Eligible' : '✗ Not eligible';
    console.log(`  ${category.padEnd(15)}: ${status}`);
  }

  // Get all eligible categories at once
  const eligibleCategories = await distribution.getEligibleCategories(alice.address);
  console.log(`\nTotal eligible categories: ${eligibleCategories.length}`);
  if (eligibleCategories.length > 0) {
    console.log('Eligible for:', eligibleCategories.join(', '));
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 2: Check Pending Rewards');
  console.log('═══════════════════════════════════════════\n');

  const pending = await distribution.getPendingRewards(alice.address);

  console.log('Pending Rewards by Category:');
  Object.entries(pending.byCategory).forEach(([category, amount]) => {
    if (amount > 0n) {
      console.log(`  ${category.padEnd(15)}: ${amount / 10n**18n} ÉTR`);
    }
  });

  console.log(`\nTotal Pending: ${pending.total / 10n**18n} ÉTR`);

  if (pending.total > 0n) {
    const usdValue = pending.total * 50n / 10n**18n; // Assuming 1 ÉTR = $50
    console.log(`Estimated Value: ~$${usdValue}`);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 3: Claim Rewards');
  console.log('═══════════════════════════════════════════\n');

  try {
    // Claim from all eligible categories
    if (eligibleCategories.length > 0) {
      for (const category of eligibleCategories) {
        const categoryPending = pending.byCategory[category];

        if (categoryPending > 0n) {
          console.log(`Claiming from ${category}...`);

          const txHash = await distribution.claimReward(alice, category);

          console.log(`  ✓ Claimed: ${categoryPending / 10n**18n} ÉTR`);
          console.log(`  Transaction: ${txHash}`);
        } else {
          console.log(`  No rewards pending in ${category}`);
        }
      }
    } else {
      console.log('⚠ No eligible categories for claiming');
      console.log('\nHow to become eligible:');
      console.log('  • Voters: Participate in governance votes');
      console.log('  • FlareNodes: Run a FlareChain validator');
      console.log('  • ValidityNodes: Run a validity node');
      console.log('  • Stakers: Stake ÉTR tokens');
      console.log('  • Directors: Become a foundation director');
    }
  } catch (error) {
    console.error('Error claiming rewards:', error);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 4: View Claim History');
  console.log('═══════════════════════════════════════════\n');

  const history = await distribution.getClaimHistory(alice.address, 10);

  if (history.claims.length > 0) {
    console.log(`Recent claims (showing ${history.claims.length} of ${history.totalClaims}):\n`);

    history.claims.forEach((claim, i) => {
      const date = new Date(claim.timestamp).toLocaleDateString();
      const time = new Date(claim.timestamp).toLocaleTimeString();
      console.log(`${i + 1}. ${date} ${time}`);
      console.log(`   Category: ${claim.category}`);
      console.log(`   Amount: ${claim.amount / 10n**18n} ÉTR`);
      console.log(`   Transaction: ${claim.txHash}`);
      console.log();
    });

    console.log(`Total claimed (all time): ${history.totalClaimed / 10n**18n} ÉTR`);
  } else {
    console.log('No claim history yet');
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 5: Estimate Future Distributions');
  console.log('═══════════════════════════════════════════\n');

  if (eligibleCategories.length > 0) {
    const category = eligibleCategories[0];

    const estimate = await distribution.estimateNextDistribution(
      alice.address,
      category
    );

    console.log(`Estimate for ${category}:\n`);
    console.log(`  Estimated Amount: ${estimate.estimatedAmount / 10n**18n} ÉTR`);
    console.log(`  Next Distribution: ${new Date(estimate.nextDistribution).toLocaleString()}`);
    console.log(`  Your Share: ${estimate.yourShare}%`);
    console.log(`  Total Recipients: ${estimate.totalRecipients}`);

    // Calculate monthly and yearly projections
    const daily = estimate.estimatedAmount;
    const monthly = daily * 30n;
    const yearly = daily * 365n;

    console.log('\nProjections:');
    console.log(`  Daily: ${daily / 10n**18n} ÉTR`);
    console.log(`  Monthly: ${monthly / 10n**18n} ÉTR`);
    console.log(`  Yearly: ${yearly / 10n**18n} ÉTR`);

    // USD estimates (assuming $50 per ÉTR)
    const price = 50n;
    console.log('\nEstimated Value (at $50/ÉTR):');
    console.log(`  Daily: $${daily * price / 10n**18n}`);
    console.log(`  Monthly: $${monthly * price / 10n**18n}`);
    console.log(`  Yearly: $${yearly * price / 10n**18n}`);
  }

  console.log('\n═══════════════════════════════════════════');
  console.log('Example 6: Multi-Category Strategy');
  console.log('═══════════════════════════════════════════\n');

  console.log('Maximizing rewards across categories:\n');

  const strategies = [
    {
      category: 'Voters',
      allocation: '2,740 ÉTR/day',
      requirement: 'Vote on governance proposals',
      minStake: '100 ÉTR',
      effort: 'Low',
      roi: 'Medium',
    },
    {
      category: 'Stakers',
      allocation: '10,959 ÉTR/day',
      requirement: 'Stake ÉTR tokens',
      minStake: '1,000 ÉTR',
      effort: 'Low',
      roi: 'Medium-High',
    },
    {
      category: 'FlareNodes',
      allocation: '4,110 ÉTR/day',
      requirement: 'Run validator node',
      minStake: '10,000 ÉTR',
      effort: 'High',
      roi: 'High',
    },
    {
      category: 'ValidityNodes',
      allocation: '4,110 ÉTR/day',
      requirement: 'Run validity node',
      minStake: '5,000 ÉTR',
      effort: 'Medium',
      roi: 'High',
    },
    {
      category: 'Directors',
      allocation: '5,479 ÉTR/day',
      requirement: 'Foundation director',
      minStake: 'Elected position',
      effort: 'Very High',
      roi: 'Very High',
    },
  ];

  strategies.forEach((strategy) => {
    console.log(`${strategy.category}:`);
    console.log(`  Daily Pool: ${strategy.allocation}`);
    console.log(`  Requirement: ${strategy.requirement}`);
    console.log(`  Min Stake: ${strategy.minStake}`);
    console.log(`  Effort: ${strategy.effort}`);
    console.log(`  ROI: ${strategy.roi}`);
    console.log();
  });

  console.log('Recommended Strategy:');
  console.log('  1. Start with Staking (largest pool, low effort)');
  console.log('  2. Add Voting (easy participation)');
  console.log('  3. Consider running ValidityNode (medium commitment)');
  console.log('  4. Advanced: Run FlareNode validator (high returns)');

  console.log('\n═══════════════════════════════════════════');
  console.log('Distribution Pay Economics');
  console.log('═══════════════════════════════════════════\n');

  console.log('Token Distribution Model:');
  console.log('  • Daily: 27,397 ÉTR');
  console.log('  • Monthly: ~821,910 ÉTR');
  console.log('  • Yearly: 10,000,000 ÉTR');
  console.log('  • Inflation Rate: Decreasing (capped supply)');

  console.log('\nValue Propositions:');
  console.log('  ✓ Predictable daily income for participants');
  console.log('  ✓ Incentivizes network security (validators)');
  console.log('  ✓ Rewards governance participation');
  console.log('  ✓ Encourages long-term holding (staking)');
  console.log('  ✓ Sustainable economic model');

  console.log('\nKey Benefits:');
  console.log('  • Passive income from staking');
  console.log('  • Compound by claiming and re-staking');
  console.log('  • Multiple income streams possible');
  console.log('  • On-chain transparency');
  console.log('  • Automated distribution (no manual claims needed in future)');

  console.log('\n═══════════════════════════════════════════');
  console.log('Best Practices');
  console.log('═══════════════════════════════════════════\n');

  console.log('1. Claiming Strategy:');
  console.log('   • Claim daily to maximize compounding');
  console.log('   • Gas fees are minimal compared to rewards');
  console.log('   • Auto-claim coming in future update');

  console.log('\n2. Maximize Earnings:');
  console.log('   • Participate in multiple categories');
  console.log('   • Re-stake claimed rewards');
  console.log('   • Vote consistently in governance');
  console.log('   • Run infrastructure if possible');

  console.log('\n3. Tax Considerations:');
  console.log('   • Track all claims for tax reporting');
  console.log('   • Claim history provides complete records');
  console.log('   • Consider jurisdiction-specific rules');

  console.log('\n4. Risk Management:');
  console.log('   • Diversify across categories');
  console.log('   • Don\'t overcommit to validator infrastructure');
  console.log('   • Balance rewards vs. lockup periods');

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
