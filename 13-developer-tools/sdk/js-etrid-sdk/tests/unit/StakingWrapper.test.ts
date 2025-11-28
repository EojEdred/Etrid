/**
 * StakingWrapper Unit Tests
 */

import { StakingWrapper } from '../../src/wrappers/StakingWrapper';
import {
  createMockApi,
  createTestAccounts,
  mockTxResult,
  mockEvent,
  mockQueryResult,
} from '../utils/testHelpers';

describe('StakingWrapper', () => {
  let wrapper: StakingWrapper;
  let mockApi: any;
  let accounts: ReturnType<typeof createTestAccounts>;

  beforeEach(() => {
    mockApi = createMockApi();
    wrapper = new StakingWrapper(mockApi);
    accounts = createTestAccounts();
  });

  describe('getValidatorStatus', () => {
    it('should return validator status', async () => {
      const validatorAddress = accounts.alice.address;

      mockApi.query = {
        staking: {
          validators: jest.fn().mockResolvedValue({
            isEmpty: false,
            commission: { toNumber: () => 100000000 }, // 10%
          }),
          ledger: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              total: { toBigInt: () => 1000n * 10n**18n },
              active: { toBigInt: () => 1000n * 10n**18n },
            }),
          }),
        },
        session: {
          validators: jest.fn().mockResolvedValue([validatorAddress]),
        },
      };

      const status = await wrapper.getValidatorStatus(validatorAddress);

      expect(status).toBeDefined();
      expect(status?.isValidator).toBe(true);
      expect(status?.active).toBe(true);
      expect(status?.commission).toBe(10);
    });

    it('should return null for non-validator', async () => {
      mockApi.query = {
        staking: {
          ledger: jest.fn().mockResolvedValue({ isNone: true }),
        },
      };

      const status = await wrapper.getValidatorStatus(accounts.bob.address);

      expect(status).toBeNull();
    });
  });

  describe('getStakingInfo', () => {
    it('should return staking information', async () => {
      mockApi.query = {
        staking: {
          ledger: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              total: { toBigInt: () => 500n * 10n**18n },
              active: { toBigInt: () => 500n * 10n**18n },
              unlocking: {
                toJSON: () => [],
              },
            }),
          }),
          bonded: jest.fn().mockResolvedValue(accounts.alice.address),
          nominators: jest.fn().mockResolvedValue({
            isSome: true,
            unwrap: () => ({
              targets: [accounts.bob.address],
            }),
          }),
        },
      };

      const info = await wrapper.getStakingInfo(accounts.alice.address);

      expect(info.staked).toBe(500n * 10n**18n);
      expect(info.status).toBe('bonded');
      expect(info.validator).toBe(accounts.bob.address);
    });

    it('should return idle status for non-staker', async () => {
      mockApi.query = {
        staking: {
          ledger: jest.fn().mockResolvedValue({ isNone: true }),
        },
      };

      const info = await wrapper.getStakingInfo(accounts.charlie.address);

      expect(info.staked).toBe(0n);
      expect(info.status).toBe('idle');
    });
  });

  describe('getValidators', () => {
    it('should return all active validators', async () => {
      const validators = [accounts.alice.address, accounts.bob.address];

      mockApi.query = {
        session: {
          validators: jest.fn().mockResolvedValue(validators),
        },
      };

      const result = await wrapper.getValidators();

      expect(result).toHaveLength(2);
      expect(result).toContain(accounts.alice.address);
    });
  });

  describe('bond', () => {
    it('should bond tokens for staking', async () => {
      const validator = accounts.bob.address;
      const amount = 1000n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Bonded', [accounts.alice.address, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        staking: {
          bond: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.bond(accounts.alice, validator, amount);

      expect(result.txHash).toBeDefined();
    });
  });

  describe('unbond', () => {
    it('should unbond tokens', async () => {
      const amount = 500n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Unbonded', [accounts.alice.address, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        staking: {
          unbond: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.unbond(accounts.alice, amount);

      expect(result.txHash).toBeDefined();
    });
  });

  describe('getCurrentEra', () => {
    it('should return current era number', async () => {
      mockApi.query = {
        staking: {
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
        },
      };

      const era = await wrapper.getCurrentEra();

      expect(era).toBe(100);
    });
  });

  describe('getMinimumStake', () => {
    it('should return minimum staking amount', async () => {
      mockApi.query = {
        staking: {
          minNominatorBond: jest.fn().mockResolvedValue({
            toBigInt: () => 10n * 10n**18n,
          }),
        },
      };

      const minimum = await wrapper.getMinimumStake();

      expect(minimum).toBe(10n * 10n**18n);
    });
  });

  describe('nominate', () => {
    it('should nominate validators', async () => {
      const validators = [accounts.alice.address, accounts.bob.address];

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Nominated', [accounts.charlie.address, validators]),
        ]));
      });

      mockApi.tx = {
        staking: {
          nominate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.nominate(accounts.charlie, validators);

      expect(result.txHash).toBeDefined();
    });
  });

  describe('getRewards', () => {
    it('should return staking rewards', async () => {
      mockApi.query = {
        staking: {
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
          erasRewardPoints: jest.fn().mockResolvedValue({
            individual: {
              get: () => ({ toBigInt: () => 1000n }),
            },
          }),
        },
      };

      const rewards = await wrapper.getRewards(accounts.alice.address);

      expect(rewards).toBe(1000n);
    });
  });

  describe('estimateRewards', () => {
    it('should estimate staking rewards', async () => {
      const amount = 1000n * 10n**18n;

      mockApi.query = {
        balances: {
          totalIssuance: jest.fn().mockResolvedValue({
            toBigInt: () => 1000000n * 10n**18n,
          }),
        },
      };

      const estimate = await wrapper.estimateRewards(amount);

      expect(estimate.apy).toBe(15);
      expect(estimate.yearly).toBeGreaterThan(0n);
      expect(estimate.monthly).toBeGreaterThan(0n);
      expect(estimate.daily).toBeGreaterThan(0n);
    });
  });

  describe('getNominators', () => {
    it('should return all nominators for validator', async () => {
      const validatorAddress = accounts.alice.address;

      mockApi.query = {
        staking: {
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
          erasStakers: jest.fn().mockResolvedValue({
            others: [
              {
                who: { toString: () => accounts.bob.address },
                value: { toBigInt: () => 100n * 10n**18n },
              },
              {
                who: { toString: () => accounts.charlie.address },
                value: { toBigInt: () => 200n * 10n**18n },
              },
            ],
          }),
        },
      };

      const nominators = await wrapper.getNominators(validatorAddress);

      expect(nominators).toHaveLength(2);
      expect(nominators[0].address).toBe(accounts.bob.address);
      expect(nominators[1].stake).toBe(200n * 10n**18n);
    });
  });

  describe('getCommissionHistory', () => {
    it('should return commission history', async () => {
      const validatorAddress = accounts.alice.address;

      mockApi.query = {
        staking: {
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
          erasValidatorPrefs: jest.fn().mockResolvedValue({
            commission: { toNumber: () => 100000000 }, // 10%
          }),
        },
      };

      const history = await wrapper.getCommissionHistory(validatorAddress, 5);

      expect(history).toHaveLength(5);
      expect(history[0].commission).toBe(10);
    });
  });

  describe('setCommission', () => {
    it('should update validator commission', async () => {
      const commission = 5; // 5%

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('CommissionUpdated', [accounts.alice.address, '50000000']),
        ]));
      });

      mockApi.tx = {
        staking: {
          validate: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.setCommission(accounts.alice, commission);

      expect(result.txHash).toBeDefined();
    });

    it('should fail with invalid commission', async () => {
      await expect(
        wrapper.setCommission(accounts.alice, 150)
      ).rejects.toThrow();

      await expect(
        wrapper.setCommission(accounts.alice, -10)
      ).rejects.toThrow();
    });
  });

  describe('getNetworkStats', () => {
    it('should return network staking statistics', async () => {
      mockApi.query = {
        balances: {
          totalIssuance: jest.fn().mockResolvedValue({
            toBigInt: () => 1000000n * 10n**18n,
          }),
        },
        staking: {
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
          erasTotalStake: jest.fn().mockResolvedValue({
            toBigInt: () => 500000n * 10n**18n,
          }),
        },
        session: {
          validators: jest.fn().mockResolvedValue(new Array(21)),
        },
      };

      const stats = await wrapper.getNetworkStats();

      expect(stats.totalStaked).toBe(500000n * 10n**18n);
      expect(stats.totalIssuance).toBe(1000000n * 10n**18n);
      expect(stats.stakingRate).toBe(50);
      expect(stats.activeValidators).toBe(21);
    });
  });

  describe('getValidatorDetails', () => {
    it('should return detailed validator information', async () => {
      mockApi.query = {
        staking: {
          validators: jest.fn().mockResolvedValue({
            isEmpty: false,
            commission: { toNumber: () => 100000000 },
          }),
          ledger: jest.fn().mockResolvedValue({
            isNone: false,
            unwrap: () => ({
              total: { toBigInt: () => 1000n * 10n**18n },
              active: { toBigInt: () => 1000n * 10n**18n },
            }),
          }),
          activeEra: jest.fn().mockResolvedValue({
            unwrap: () => ({
              index: { toNumber: () => 100 },
            }),
          }),
          erasStakers: jest.fn().mockResolvedValue({
            others: { length: 50 },
            total: { toBigInt: () => 5000n * 10n**18n },
          }),
        },
        session: {
          validators: jest.fn().mockResolvedValue([accounts.alice.address]),
        },
      };

      const details = await wrapper.getValidatorDetails(accounts.alice.address);

      expect(details.status?.isValidator).toBe(true);
      expect(details.nominators).toBe(50);
      expect(details.totalStake).toBe(5000n * 10n**18n);
    });
  });

  describe('getUnbondingPeriod', () => {
    it('should return unbonding period in eras', async () => {
      mockApi.consts = {
        staking: {
          bondingDuration: { toNumber: () => 28 },
        },
      };

      const period = await wrapper.getUnbondingPeriod();

      expect(period).toBe(28);
    });
  });

  describe('withdrawUnbonded', () => {
    it('should withdraw unbonded tokens', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Withdrawn', [accounts.alice.address, '500000000000000000000']),
        ]));
      });

      mockApi.tx = {
        staking: {
          withdrawUnbonded: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.withdrawUnbonded(accounts.alice, 0);

      expect(result.txHash).toBeDefined();
    });
  });

  describe('chill', () => {
    it('should stop nominating/validating', async () => {
      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('Chilled', [accounts.alice.address]),
        ]));
      });

      mockApi.tx = {
        staking: {
          chill: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.chill(accounts.alice);

      expect(result.txHash).toBeDefined();
    });
  });

  describe('bondAdditional', () => {
    it('should bond additional tokens', async () => {
      const amount = 500n * 10n**18n;

      const mockSignAndSend = jest.fn((signer, callback) => {
        callback(mockTxResult('InBlock', [
          mockEvent('BondedAdditional', [accounts.alice.address, amount.toString()]),
        ]));
      });

      mockApi.tx = {
        staking: {
          bondExtra: jest.fn(() => ({
            signAndSend: mockSignAndSend,
          })),
        },
      };

      const result = await wrapper.bondAdditional(accounts.alice, amount);

      expect(result.txHash).toBeDefined();
    });
  });
});
