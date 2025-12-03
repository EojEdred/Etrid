/**
 * ETRID Governance Example Component
 *
 * Demonstrates how to use the governance service in a React component
 * This is a reference implementation - adapt as needed for your UI
 */

'use client';

import { useState } from 'react';
import {
  useActiveProposals,
  useVotingPower,
  useCastVote,
  useGovernanceStats,
  ProposalCategory,
  ConvictionLevel,
  VoteType,
  CATEGORY_LABELS,
  STATUS_LABELS,
  CONVICTION_CONFIGS,
} from './index';

interface GovernanceExampleProps {
  account: string | null;
}

export function GovernanceExample({ account }: GovernanceExampleProps) {
  const { proposals, loading: proposalsLoading } = useActiveProposals();
  const { votingPower, loading: powerLoading } = useVotingPower(account);
  const { stats } = useGovernanceStats();
  const { castVote, voting } = useCastVote();

  const [selectedProposal, setSelectedProposal] = useState<number | null>(null);
  const [selectedConviction, setSelectedConviction] = useState(ConvictionLevel.Locked1x);

  const handleVote = async (proposalId: number, voteType: VoteType) => {
    if (!account) {
      alert('Please connect your wallet');
      return;
    }

    try {
      // In a real app, you'd get the signer from Polkadot extension
      const { web3FromAddress } = await import('@polkadot/extension-dapp');
      const injector = await web3FromAddress(account);

      const result = await castVote(
        {
          proposalId,
          voteType,
          conviction: selectedConviction,
        },
        injector.signer
      );

      if (result.success) {
        alert(`Vote submitted! Transaction: ${result.txHash}`);
      } else {
        alert(`Vote failed: ${result.error?.message}`);
      }
    } catch (error) {
      console.error('Error voting:', error);
      alert(`Error: ${error}`);
    }
  };

  if (!account) {
    return (
      <div className="p-6 bg-gray-100 rounded-lg">
        <p className="text-center text-gray-600">
          Please connect your wallet to participate in governance
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Governance Stats */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-2xl font-bold mb-4">Governance Overview</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <p className="text-sm text-gray-600">Total Proposals</p>
            <p className="text-2xl font-bold">{stats?.totalProposals || 0}</p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Active Proposals</p>
            <p className="text-2xl font-bold">{stats?.activeProposals || 0}</p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Unique Voters</p>
            <p className="text-2xl font-bold">{stats?.uniqueVoters || 0}</p>
          </div>
          <div>
            <p className="text-sm text-gray-600">Participation Rate</p>
            <p className="text-2xl font-bold">
              {stats?.participationRate.toFixed(1) || 0}%
            </p>
          </div>
        </div>
      </div>

      {/* Your Voting Power */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-2xl font-bold mb-4">Your Voting Power</h2>
        {powerLoading ? (
          <p>Loading...</p>
        ) : (
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            <div>
              <p className="text-sm text-gray-600">Staked Balance</p>
              <p className="text-xl font-semibold">
                {votingPower?.stakedBalance || '0'} ETR
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600">Total Voting Power</p>
              <p className="text-xl font-semibold">
                {votingPower?.totalVotingPower || '0'}
              </p>
            </div>
            <div>
              <p className="text-sm text-gray-600">Active Votes</p>
              <p className="text-xl font-semibold">
                {votingPower?.activeVotes || 0}
              </p>
            </div>
          </div>
        )}
      </div>

      {/* Conviction Selector */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h3 className="text-lg font-semibold mb-3">Select Conviction Level</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-2">
          {CONVICTION_CONFIGS.map((config) => (
            <button
              key={config.level}
              onClick={() => setSelectedConviction(config.level)}
              className={`p-3 rounded border text-sm ${
                selectedConviction === config.level
                  ? 'border-blue-500 bg-blue-50'
                  : 'border-gray-300'
              }`}
            >
              <div className="font-semibold">{config.multiplier}x</div>
              <div className="text-xs text-gray-600">
                {config.lockPeriodDays} days
              </div>
            </button>
          ))}
        </div>
        <p className="text-sm text-gray-600 mt-2">
          Higher conviction = more voting power, but longer lock period
        </p>
      </div>

      {/* Active Proposals */}
      <div className="bg-white p-6 rounded-lg shadow">
        <h2 className="text-2xl font-bold mb-4">Active Proposals</h2>
        {proposalsLoading ? (
          <p>Loading proposals...</p>
        ) : proposals.length === 0 ? (
          <p className="text-gray-600">No active proposals</p>
        ) : (
          <div className="space-y-4">
            {proposals.map((proposal) => (
              <div
                key={proposal.id}
                className="border rounded-lg p-4 hover:shadow-md transition"
              >
                <div className="flex justify-between items-start mb-2">
                  <h3 className="text-xl font-semibold">{proposal.title}</h3>
                  <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                    {CATEGORY_LABELS[proposal.category]}
                  </span>
                </div>

                <p className="text-gray-700 mb-3">{proposal.description}</p>

                <div className="grid grid-cols-3 gap-2 mb-3 text-sm">
                  <div>
                    <p className="text-gray-600">Votes For</p>
                    <p className="font-semibold">{proposal.votesFor}</p>
                  </div>
                  <div>
                    <p className="text-gray-600">Votes Against</p>
                    <p className="font-semibold">{proposal.votesAgainst}</p>
                  </div>
                  <div>
                    <p className="text-gray-600">Abstain</p>
                    <p className="font-semibold">{proposal.votesAbstain}</p>
                  </div>
                </div>

                <div className="flex gap-2">
                  <button
                    onClick={() => handleVote(proposal.id, VoteType.Aye)}
                    disabled={voting}
                    className="flex-1 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50"
                  >
                    {voting ? 'Voting...' : 'Vote Yes'}
                  </button>
                  <button
                    onClick={() => handleVote(proposal.id, VoteType.Nay)}
                    disabled={voting}
                    className="flex-1 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 disabled:opacity-50"
                  >
                    {voting ? 'Voting...' : 'Vote No'}
                  </button>
                  <button
                    onClick={() => handleVote(proposal.id, VoteType.Abstain)}
                    disabled={voting}
                    className="flex-1 px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 disabled:opacity-50"
                  >
                    Abstain
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

// Example usage in a page component:
/*
'use client';

import { GovernanceExample } from '@/lib/governance/example';
import { useWallet } from '@/lib/polkadot/useWallet';

export default function GovernancePage() {
  const { address } = useWallet();

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">ETRID Governance</h1>
      <GovernanceExample account={address} />
    </div>
  );
}
*/
