/**
 * Governance API Functions
 * Handles on-chain voting and proposal interactions for Ëtrid Consensus Day
 */

import { ApiPromise } from '@polkadot/api';
import { createApi } from './api';
import type { ChainId } from './chains';

export interface ProposalVote {
  proposalId: number;
  vote: 'aye' | 'nay' | 'abstain';
  votingPower: bigint;
  voter: string;
}

/**
 * Submit a vote for a governance proposal using the consensus-day-governance pallet
 */
export async function submitVote(
  chainId: ChainId,
  voterAddress: string,
  proposalId: number,
  vote: 'aye' | 'nay' | 'abstain',
  signer: any
): Promise<string> {
  const api = await createApi(chainId);

  // Map vote type to pallet enum
  const voteType = vote === 'aye' ? 'Aye' : vote === 'nay' ? 'Nay' : 'Abstain';

  // Call the governance pallet vote extrinsic
  // api.tx.consensusDayGovernance.vote(proposalId, voteType)

  return new Promise((resolve, reject) => {
    // Try to call pallet if it exists, otherwise fall back to remark
    const tx = api.tx.consensusDayGovernance
      ? api.tx.consensusDayGovernance.vote(proposalId, voteType)
      : api.tx.system.remark(
          JSON.stringify({
            type: 'governance_vote',
            proposalId,
            vote: voteType,
            timestamp: Date.now()
          })
        );

    tx.signAndSend(
      voterAddress,
      { signer },
      ({ status, txHash }) => {
        if (status.isInBlock) {
          console.log(`[Governance] Vote transaction in block: ${txHash.toHex()}`);
          resolve(txHash.toHex());
        } else if (status.isFinalized) {
          console.log(`[Governance] Vote finalized`);
        } else if (status.isInvalid) {
          reject(new Error('Vote transaction invalid'));
        }
      }
    ).catch(reject);
  });
}

/**
 * Get voting power for an address based on ASF formula
 * Formula: √(stake × coinage)
 */
export async function getVotingPower(
  chainId: ChainId,
  address: string,
  stakeAmount: bigint,
  coinageDays: number = 180 // Default coinage
): Promise<number> {
  // Calculate voting power using Ascending Scale of Finality
  const stake = Number(stakeAmount) / 1e12; // Convert from plancks to ÉTR
  const votingPower = Math.sqrt(stake * coinageDays);

  return Math.floor(votingPower);
}

/**
 * Check if an address has voted on a proposal
 * In production, query the governance pallet storage
 */
export async function hasVoted(
  chainId: ChainId,
  address: string,
  proposalId: number
): Promise<boolean> {
  const api = await createApi(chainId);

  // In production:
  // const vote = await api.query.governance.votingOf(address, proposalId);
  // return vote.isSome;

  // For now, return false (not voted)
  return false;
}

/**
 * Get all proposals from chain
 * In production, query the governance pallet
 */
export async function getProposals(chainId: ChainId): Promise<any[]> {
  const api = await createApi(chainId);

  // In production:
  // const proposals = await api.query.governance.proposals.entries();
  // return proposals.map(([key, proposal]) => ({...}));

  // For now, return empty array (use mock data in UI)
  return [];
}

/**
 * Get vote tally for a proposal
 */
export async function getVoteTally(
  chainId: ChainId,
  proposalId: number
): Promise<{ ayes: bigint; nays: bigint; abstain: bigint }> {
  const api = await createApi(chainId);

  // In production:
  // const tally = await api.query.governance.proposalVotes(proposalId);

  // For now, return zero tallies
  return {
    ayes: BigInt(0),
    nays: BigInt(0),
    abstain: BigInt(0)
  };
}
