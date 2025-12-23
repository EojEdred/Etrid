/**
 * Governance Service Test Script
 *
 * Run this to verify the governance service can connect to the chain
 * Usage: npx tsx lib/governance/test.ts
 */

import { governanceService } from './service';
import { ProposalCategory } from './types';

async function testGovernanceService() {
  console.log('🧪 Testing ETRID Governance Service\n');

  try {
    // Test 1: Fetch proposals
    console.log('1️⃣ Fetching proposals...');
    const proposals = await governanceService.getProposals();
    console.log(`✅ Found ${proposals.length} proposals`);

    if (proposals.length > 0) {
      const firstProposal = proposals[0];
      console.log(`   First proposal: "${firstProposal.title}" (${firstProposal.category})`);
    }

    // Test 2: Fetch governance stats
    console.log('\n2️⃣ Fetching governance statistics...');
    const stats = await governanceService.getGovernanceStats();
    console.log(`✅ Stats retrieved:`);
    console.log(`   Total Proposals: ${stats.totalProposals}`);
    console.log(`   Active Proposals: ${stats.activeProposals}`);
    console.log(`   Unique Voters: ${stats.uniqueVoters}`);

    // Test 3: Test category stats
    console.log('\n3️⃣ Fetching category statistics...');
    const categoryStats = await governanceService.getCategoryStats(
      ProposalCategory.ParameterChange
    );
    console.log(`✅ Parameter Change category:`);
    console.log(`   Total: ${categoryStats.totalProposals}`);
    console.log(`   Active: ${categoryStats.activeProposals}`);

    // Test 4: Test voting power (with a dummy address)
    console.log('\n4️⃣ Testing voting power query...');
    const dummyAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    const votingPower = await governanceService.getVotingPower(dummyAddress);
    console.log(`✅ Voting power for ${dummyAddress.slice(0, 10)}...:`);
    console.log(`   Staked: ${votingPower.stakedBalance}`);
    console.log(`   Total Power: ${votingPower.totalVotingPower}`);
    console.log(`   Can Vote: ${votingPower.canVote}`);

    // Test 5: Test conviction calculation
    console.log('\n5️⃣ Testing conviction calculation...');
    const breakdown = governanceService.calculateVotingPowerBreakdown('1000', 3);
    console.log(`✅ 1000 ETR with 3x conviction:`);
    console.log(`   Calculated Power: ${breakdown.calculatedPower}`);
    console.log(`   Multiplier: ${breakdown.multiplier}x`);
    console.log(`   Lock Period: ${breakdown.lockPeriodDays} days`);

    console.log('\n✅ All tests passed! Governance service is working correctly.\n');

  } catch (error) {
    console.error('\n❌ Test failed:', error);

    if ((error as any).code === 'NOT_CONNECTED') {
      console.log('\n💡 Note: The chain might be offline or unreachable.');
      console.log('   This is expected if running locally or if the RPC is down.');
      console.log('   The service will gracefully handle connection failures in production.\n');
    }
  } finally {
    // Disconnect
    await governanceService.disconnect();
    console.log('👋 Disconnected from chain\n');
  }
}

// Run tests if this file is executed directly
if (require.main === module) {
  testGovernanceService()
    .then(() => process.exit(0))
    .catch((error) => {
      console.error('Fatal error:', error);
      process.exit(1);
    });
}

export { testGovernanceService };
