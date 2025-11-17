/**
 * GovernanceWrapper Unit Tests
 */

import { GovernanceWrapper } from '../../src/wrappers/GovernanceWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('GovernanceWrapper', () => {
  let wrapper: GovernanceWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new GovernanceWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('getActiveProposals', () => {
    it('should return all active proposals', async () => {
      const mockProposals = [
        [
          { args: [1] },
          {
            unwrap: () => ({
              proposer: { toString: () => accounts.alice.address },
              title: { toString: () => 'Proposal 1' },
              description: { toString: () => 'Description 1' },
              votesFor: { toBigInt: () => 1000n },
              votesAgainst: { toBigInt: () => 500n },
              status: { toString: () => 'active' },
              createdAt: { toNumber: () => 1000 },
              endsAt: { toNumber: () => 2000 },
            }),
          },
        ],
      ];

      mockApi.query = {
        governance: {
          proposals: {
            entries: jest.fn().mockResolvedValue(mockProposals),
          },
        },
      };

      const proposals = await wrapper.getActiveProposals();

      expect(proposals).toHaveLength(1);
      expect(proposals[0].title).toBe('Proposal 1');
      expect(proposals[0].status).toBe('active');
    });
  });

  describe('getProposal', () => {
    it('should return proposal by ID', async () => {
      const mockProposal = {
        proposer: { toString: () => accounts.alice.address },
        title: { toString: () => 'Test Proposal' },
        description: { toString: () => 'Test Description' },
        votesFor: { toBigInt: () => 1000n },
        votesAgainst: { toBigInt: () => 500n },
        status: { toString: () => 'active' },
        createdAt: { toNumber: () => 1000 },
        endsAt: { toNumber: () => 2000 },
      };

      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => mockProposal,
          }),
        },
      };

      const proposal = await wrapper.getProposal(1);

      expect(proposal).toBeDefined();
      expect(proposal?.title).toBe('Test Proposal');
    });

    it('should return null for non-existent proposal', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({ isNone: true }),
        },
      };

      const proposal = await wrapper.getProposal(999);

      expect(proposal).toBeNull();
    });
  });

  describe('vote', () => {
    it('should vote on proposal', async () => {
      const proposalId = 1;
      const approve = true;
      const stake = 1000n * 10n**18n;

      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              status: { toString: () => 'active' },
            }),
          }),
        },
      };

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Voted', [proposalId, accounts.alice.address, approve, stake.toString()]),
        ]));
      });

      mockApi.tx = {
        governance: {
          vote: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.vote(accounts.alice, proposalId, approve, stake);

      expect(result.txHash).toBeDefined();
    });

    it('should fail to vote on non-existent proposal', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({ isNone: true }),
        },
      };

      await expect(
        wrapper.vote(accounts.alice, 999, true, 1000n)
      ).rejects.toThrow();
    });

    it('should fail to vote on inactive proposal', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              status: { toString: () => 'passed' },
            }),
          }),
        },
      };

      await expect(
        wrapper.vote(accounts.alice, 1, true, 1000n)
      ).rejects.toThrow();
    });
  });

  describe('createProposal', () => {
    it('should create new proposal', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ProposalCreated', [1, accounts.alice.address]),
        ]));
      });

      mockApi.tx = {
        governance: {
          propose: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.createProposal(
        accounts.alice,
        'Test Proposal',
        'Description',
        null
      );

      expect(result.txHash).toBeDefined();
    });

    it('should fail with empty title', async () => {
      await expect(
        wrapper.createProposal(accounts.alice, '', 'Description', null)
      ).rejects.toThrow();
    });

    it('should fail with empty description', async () => {
      await expect(
        wrapper.createProposal(accounts.alice, 'Title', '', null)
      ).rejects.toThrow();
    });
  });

  describe('executeProposal', () => {
    it('should execute passed proposal', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              status: { toString: () => 'passed' },
            }),
          }),
        },
      };

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ProposalExecuted', [1]),
        ]));
      });

      mockApi.tx = {
        governance: {
          execute: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.executeProposal(accounts.alice, 1);

      expect(result.txHash).toBeDefined();
    });

    it('should fail to execute non-passed proposal', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              status: { toString: () => 'active' },
            }),
          }),
        },
      };

      await expect(
        wrapper.executeProposal(accounts.alice, 1)
      ).rejects.toThrow();
    });
  });

  describe('getProposalResults', () => {
    it('should return proposal voting results', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              votesFor: { toBigInt: () => 6000n },
              votesAgainst: { toBigInt: () => 4000n },
            }),
          }),
        },
        balances: {
          totalIssuance: jest.fn().mockResolvedValue({
            toBigInt: () => 100000n,
          }),
        },
      };

      const results = await wrapper.getProposalResults(1);

      expect(results.votesFor).toBe(6000n);
      expect(results.votesAgainst).toBe(4000n);
      expect(results.totalVotes).toBe(10000n);
      expect(results.approved).toBe(true);
      expect(results.participationRate).toBeGreaterThan(0);
    });
  });

  describe('getVotingPower', () => {
    it('should return voting power for address', async () => {
      mockApi.query = {
        system: {
          account: jest.fn().mockResolvedValue({
            data: {
              free: { toBigInt: () => 1000n * 10n**18n },
            },
          }),
        },
      };

      const power = await wrapper.getVotingPower(accounts.alice.address);

      expect(power).toBe(1000n * 10n**18n);
    });
  });

  describe('hasVoted', () => {
    it('should return true if already voted', async () => {
      mockApi.query = {
        governance: {
          votes: jest.fn().mockResolvedValue({
            isNone: false,
          }),
        },
      };

      const voted = await wrapper.hasVoted(1, accounts.alice.address);

      expect(voted).toBe(true);
    });

    it('should return false if not voted', async () => {
      mockApi.query = {
        governance: {
          votes: jest.fn().mockResolvedValue({
            isNone: true,
          }),
        },
      };

      const voted = await wrapper.hasVoted(1, accounts.alice.address);

      expect(voted).toBe(false);
    });
  });

  describe('getVotingStatistics', () => {
    it('should return voting statistics for address', async () => {
      mockApi.query = {
        governance: {
          voterHistory: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => [
              {
                approve: { isTrue: true },
                timestamp: { toNumber: () => 1000 },
                proposalId: { toNumber: () => 1 },
              },
              {
                approve: { isTrue: false },
                timestamp: { toNumber: () => 2000 },
                proposalId: { toNumber: () => 2 },
              },
            ],
          }),
          proposalCount: jest.fn().mockResolvedValue({ toNumber: () => 10 }),
        },
      };

      const stats = await wrapper.getVotingStatistics(accounts.alice.address);

      expect(stats.totalVotes).toBe(2);
      expect(stats.votesFor).toBe(1);
      expect(stats.votesAgainst).toBe(1);
      expect(stats.participationRate).toBeGreaterThan(0);
    });
  });

  describe('estimateProposalOutcome', () => {
    it('should estimate proposal outcome', async () => {
      const now = Date.now();

      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              votesFor: { toBigInt: () => 6000n },
              votesAgainst: { toBigInt: () => 4000n },
              createdAt: { toNumber: () => now - 100000 },
              endsAt: { toNumber: () => now + 100000 },
            }),
          }),
        },
        balances: {
          totalIssuance: jest.fn().mockResolvedValue({
            toBigInt: () => 100000n,
          }),
        },
      };

      const estimate = await wrapper.estimateProposalOutcome(1);

      expect(estimate.currentResult).toBe('passing');
      expect(estimate.confidence).toBeGreaterThan(0);
      expect(estimate.projectedVotesFor).toBeGreaterThanOrEqual(6000n);
    });
  });

  describe('getGovernanceStats', () => {
    it('should return governance statistics', async () => {
      const mockProposals = [
        [
          { args: [1] },
          {
            unwrap: () => ({
              proposer: { toString: () => accounts.alice.address },
              title: { toString: () => 'P1' },
              description: { toString: () => 'D1' },
              votesFor: { toBigInt: () => 1000n },
              votesAgainst: { toBigInt: () => 500n },
              status: { toString: () => 'active' },
              createdAt: { toNumber: () => 1000 },
              endsAt: { toNumber: () => 2000 },
            }),
          },
        ],
        [
          { args: [2] },
          {
            unwrap: () => ({
              proposer: { toString: () => accounts.bob.address },
              title: { toString: () => 'P2' },
              description: { toString: () => 'D2' },
              votesFor: { toBigInt: () => 2000n },
              votesAgainst: { toBigInt: () => 1000n },
              status: { toString: () => 'passed' },
              createdAt: { toNumber: () => 3000 },
              endsAt: { toNumber: () => 4000 },
            }),
          },
        ],
      ];

      mockApi.query = {
        governance: {
          proposals: {
            entries: jest.fn().mockResolvedValue(mockProposals),
          },
          votes: {
            entries: jest.fn().mockResolvedValue([
              [{ args: [1, accounts.alice.address] }, {}],
              [{ args: [1, accounts.bob.address] }, {}],
            ]),
          },
        },
        balances: {
          totalIssuance: jest.fn().mockResolvedValue({
            toBigInt: () => 100000n,
          }),
        },
      };

      const stats = await wrapper.getGovernanceStats();

      expect(stats.totalProposals).toBe(2);
      expect(stats.activeProposals).toBe(1);
      expect(stats.passedProposals).toBe(1);
    });
  });

  describe('delegateVotes', () => {
    it('should delegate voting power', async () => {
      const weight = 1000n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('VotesDelegated', [
            accounts.alice.address,
            accounts.bob.address,
            weight.toString(),
          ]),
        ]));
      });

      mockApi.tx = {
        governance: {
          delegate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.delegateVotes(
        accounts.alice,
        accounts.bob.address,
        weight
      );

      expect(result.txHash).toBeDefined();
    });
  });

  describe('undelegateVotes', () => {
    it('should undelegate voting power', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('VotesUndelegated', [
            accounts.alice.address,
            accounts.bob.address,
          ]),
        ]));
      });

      mockApi.tx = {
        governance: {
          undelegate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.undelegateVotes(
        accounts.alice,
        accounts.bob.address
      );

      expect(result.txHash).toBeDefined();
    });
  });

  describe('cancelProposal', () => {
    it('should cancel proposal by proposer', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              proposer: { toString: () => accounts.alice.address },
            }),
          }),
        },
      };

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('ProposalCancelled', [1]),
        ]));
      });

      mockApi.tx = {
        governance: {
          cancel: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.cancelProposal(accounts.alice, 1);

      expect(result.txHash).toBeDefined();
    });

    it('should fail if not proposer', async () => {
      mockApi.query = {
        governance: {
          proposals: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              proposer: { toString: () => accounts.alice.address },
            }),
          }),
        },
      };

      await expect(
        wrapper.cancelProposal(accounts.bob, 1)
      ).rejects.toThrow();
    });
  });

  describe('getVotingPeriod', () => {
    it('should return voting period', async () => {
      mockApi.consts = {
        governance: {
          votingPeriod: { toNumber: () => 100800 },
        },
      };

      const period = await wrapper.getVotingPeriod();

      expect(period).toBe(100800);
    });
  });

  describe('getMinimumProposalStake', () => {
    it('should return minimum stake for proposals', async () => {
      mockApi.consts = {
        governance: {
          minimumProposalStake: { toBigInt: () => 1000n * 10n**18n },
        },
      };

      const minimum = await wrapper.getMinimumProposalStake();

      expect(minimum).toBe(1000n * 10n**18n);
    });
  });
});
