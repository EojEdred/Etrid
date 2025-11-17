/**
 * DistributionPayWrapper Unit Tests
 */

import {
  DistributionPayWrapper,
  DistributionCategory,
} from '../../src/wrappers/DistributionPayWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('DistributionPayWrapper', () => {
  let wrapper: DistributionPayWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new DistributionPayWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('claimReward', () => {
    it('should claim reward successfully', async () => {
      const amount = 100n * 10n**18n;
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('RewardClaimed', [
            accounts.alice.address,
            DistributionCategory.Stakers,
            amount.toString(),
          ]),
        ]));
      });

      mockApi.tx = {
        distributionPay: {
          claimReward: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const txHash = await wrapper.claimReward(
        accounts.alice,
        DistributionCategory.Stakers
      );

      expect(txHash).toBeDefined();
      expect(mockApi.tx.distributionPay.claimReward).toHaveBeenCalledWith(
        DistributionCategory.Stakers
      );
    });

    it('should fail if not eligible', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 1 },
          },
        });
      });

      mockApi.tx = {
        distributionPay: {
          claimReward: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'distributionPay',
        name: 'NotEligible',
        docs: ['Account not eligible for this category'],
      }));

      await expect(
        wrapper.claimReward(accounts.alice, DistributionCategory.Directors)
      ).rejects.toThrow();
    });

    it('should fail if claimed too early', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback({
          ...mockTxResult('Error'),
          dispatchError: {
            isModule: true,
            asModule: { index: 1, error: 2 },
          },
        });
      });

      mockApi.tx = {
        distributionPay: {
          claimReward: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      mockApi.registry.findMetaError = jest.fn(() => ({
        section: 'distributionPay',
        name: 'ClaimTooEarly',
        docs: ['Must wait 24 hours between claims'],
      }));

      await expect(
        wrapper.claimReward(accounts.alice, DistributionCategory.Stakers)
      ).rejects.toThrow();
    });
  });

  describe('getPendingRewards', () => {
    it('should return pending amounts by category', async () => {
      mockApi.query = {
        distributionPay: {
          pendingRewards: jest.fn((address, category) => {
            const amounts: Record<string, string> = {
              [DistributionCategory.Stakers]: (50n * 10n**18n).toString(),
              [DistributionCategory.Voters]: (10n * 10n**18n).toString(),
              [DistributionCategory.FlareNodes]: '0',
              [DistributionCategory.ValidityNodes]: '0',
              [DistributionCategory.Directors]: '0',
            };
            return Promise.resolve(amounts[category] || '0');
          }),
        },
      };

      const pending = await wrapper.getPendingRewards(accounts.alice.address);

      expect(pending.total).toBe(60n * 10n**18n);
      expect(pending.byCategory[DistributionCategory.Stakers]).toBe(50n * 10n**18n);
      expect(pending.byCategory[DistributionCategory.Voters]).toBe(10n * 10n**18n);
    });

    it('should return zero for ineligible users', async () => {
      mockApi.query = {
        distributionPay: {
          pendingRewards: jest.fn(() => Promise.resolve('0')),
        },
      };

      const pending = await wrapper.getPendingRewards(accounts.bob.address);

      expect(pending.total).toBe(0n);
      Object.values(pending.byCategory).forEach((amount) => {
        expect(amount).toBe(0n);
      });
    });
  });

  describe('getDistributionSchedule', () => {
    it('should return distribution configuration', async () => {
      const mockSchedule = {
        totalDailyDistribution: (27397n * 10n**18n).toString(),
        distributionPeriod: '86400',
        nextDistribution: Date.now() + 3600000,
        categoryAllocations: {
          [DistributionCategory.Voters]: (2740n * 10n**18n).toString(),
          [DistributionCategory.FlareNodes]: (4110n * 10n**18n).toString(),
          [DistributionCategory.ValidityNodes]: (4110n * 10n**18n).toString(),
          [DistributionCategory.Stakers]: (10959n * 10n**18n).toString(),
          [DistributionCategory.Directors]: (5479n * 10n**18n).toString(),
        },
      };

      mockApi.query = {
        distributionPay: {
          distributionSchedule: jest.fn().mockResolvedValue(mockSchedule),
        },
      };

      const schedule = await wrapper.getDistributionSchedule();

      expect(schedule.totalDailyDistribution).toBe(27397n * 10n**18n);
      expect(schedule.distributionPeriod).toBe(86400);
      expect(schedule.categoryAllocations[DistributionCategory.Stakers]).toBe(
        10959n * 10n**18n
      );
    });
  });

  describe('isEligible', () => {
    it('should check voter eligibility', async () => {
      mockApi.query = {
        distributionPay: {
          eligibility: jest.fn().mockResolvedValue(true),
        },
      };

      const eligible = await wrapper.isEligible(
        accounts.alice.address,
        DistributionCategory.Voters
      );

      expect(eligible).toBe(true);
    });

    it('should return false for ineligible category', async () => {
      mockApi.query = {
        distributionPay: {
          eligibility: jest.fn().mockResolvedValue(false),
        },
      };

      const eligible = await wrapper.isEligible(
        accounts.bob.address,
        DistributionCategory.Directors
      );

      expect(eligible).toBe(false);
    });
  });

  describe('getEligibleCategories', () => {
    it('should return all eligible categories', async () => {
      mockApi.query = {
        distributionPay: {
          eligibility: jest.fn((address, category) => {
            const eligible = [
              DistributionCategory.Stakers,
              DistributionCategory.Voters,
            ];
            return Promise.resolve(eligible.includes(category));
          }),
        },
      };

      const categories = await wrapper.getEligibleCategories(accounts.alice.address);

      expect(categories).toHaveLength(2);
      expect(categories).toContain(DistributionCategory.Stakers);
      expect(categories).toContain(DistributionCategory.Voters);
    });

    it('should return empty array if none eligible', async () => {
      mockApi.query = {
        distributionPay: {
          eligibility: jest.fn().mockResolvedValue(false),
        },
      };

      const categories = await wrapper.getEligibleCategories(accounts.charlie.address);

      expect(categories).toHaveLength(0);
    });
  });

  describe('estimateNextDistribution', () => {
    it('should calculate estimated amount', async () => {
      const mockEstimate = {
        estimatedAmount: (100n * 10n**18n).toString(),
        nextDistribution: Date.now() + 3600000,
        yourShare: '1000', // 10%
        totalRecipients: '10',
      };

      mockApi.query = {
        distributionPay: {
          estimates: jest.fn().mockResolvedValue(mockEstimate),
        },
      };

      const estimate = await wrapper.estimateNextDistribution(
        accounts.alice.address,
        DistributionCategory.Stakers
      );

      expect(estimate.estimatedAmount).toBe(100n * 10n**18n);
      expect(estimate.yourShare).toBe(1000);
      expect(estimate.totalRecipients).toBe(10);
    });
  });

  describe('getClaimHistory', () => {
    it('should return claim history', async () => {
      const mockHistory = {
        claims: [
          {
            category: DistributionCategory.Stakers,
            amount: (50n * 10n**18n).toString(),
            timestamp: Date.now() - 86400000,
            txHash: '0xabc123',
          },
          {
            category: DistributionCategory.Voters,
            amount: (10n * 10n**18n).toString(),
            timestamp: Date.now() - 172800000,
            txHash: '0xdef456',
          },
        ],
        totalClaims: 2,
        totalClaimed: (60n * 10n**18n).toString(),
      };

      mockApi.query = {
        distributionPay: {
          claimHistory: jest.fn().mockResolvedValue(mockHistory),
        },
      };

      const history = await wrapper.getClaimHistory(accounts.alice.address, 10);

      expect(history.claims).toHaveLength(2);
      expect(history.totalClaimed).toBe(60n * 10n**18n);
      expect(history.claims[0].category).toBe(DistributionCategory.Stakers);
    });

    it('should return empty history for new user', async () => {
      mockApi.query = {
        distributionPay: {
          claimHistory: jest.fn().mockResolvedValue({
            claims: [],
            totalClaims: 0,
            totalClaimed: '0',
          }),
        },
      };

      const history = await wrapper.getClaimHistory(accounts.charlie.address, 10);

      expect(history.claims).toHaveLength(0);
      expect(history.totalClaimed).toBe(0n);
    });
  });
});
