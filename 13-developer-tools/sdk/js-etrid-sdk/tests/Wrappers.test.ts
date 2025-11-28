/**
 * Tests for SDK Wrappers (Accounts, Staking, Governance)
 */

import { describe, it, expect, beforeAll, afterAll } from '@jest/globals';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { AccountsWrapper } from '../src/wrappers/AccountsWrapper';
import { StakingWrapper } from '../src/wrappers/StakingWrapper';
import { GovernanceWrapper } from '../src/wrappers/GovernanceWrapper';
import { NotConnectedError, InvalidAddressError } from '../src/errors/EtridErrors';

describe('SDK Wrappers', () => {
  let api: ApiPromise;
  let keyring: Keyring;
  let alice: any;
  let bob: any;

  beforeAll(async () => {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    api = await ApiPromise.create({ provider });

    keyring = new Keyring({ type: 'sr25519' });
    alice = keyring.addFromUri('//Alice');
    bob = keyring.addFromUri('//Bob');
  });

  afterAll(async () => {
    await api.disconnect();
  });

  describe('AccountsWrapper', () => {
    let accounts: AccountsWrapper;

    beforeAll(() => {
      accounts = new AccountsWrapper(api);
    });

    it('should get account balance', async () => {
      const balance = await accounts.getBalance(alice.address);

      expect(balance).toBeDefined();
      expect(balance.etr).toBeGreaterThanOrEqual(0n);
      expect(balance.total).toBeGreaterThanOrEqual(0n);
      expect(balance.available).toBeGreaterThanOrEqual(0n);
    });

    it('should get free balance', async () => {
      const freeBalance = await accounts.getFreeBalance(alice.address);

      expect(freeBalance).toBeGreaterThanOrEqual(0n);
    });

    it('should get reserved balance', async () => {
      const reservedBalance = await accounts.getReservedBalance(alice.address);

      expect(reservedBalance).toBeGreaterThanOrEqual(0n);
    });

    it('should get total balance', async () => {
      const totalBalance = await accounts.getTotalBalance(alice.address);

      expect(totalBalance).toBeGreaterThanOrEqual(0n);
    });

    it('should check if account exists', async () => {
      const exists = await accounts.accountExists(alice.address);

      expect(typeof exists).toBe('boolean');
    });

    it('should get existential deposit', () => {
      const ed = accounts.getExistentialDeposit();

      expect(ed).toBeGreaterThan(0n);
    });

    it('should get account nonce', async () => {
      const nonce = await accounts.getNonce(alice.address);

      expect(typeof nonce).toBe('number');
      expect(nonce).toBeGreaterThanOrEqual(0);
    });

    it('should get account info', async () => {
      const info = await accounts.getAccountInfo(alice.address);

      expect(info).toBeDefined();
      expect(info.nonce).toBeGreaterThanOrEqual(0);
      expect(info.balance).toBeDefined();
      expect(info.balance.total).toBeGreaterThanOrEqual(0n);
    });

    it('should throw error for invalid address', async () => {
      await expect(accounts.getBalance('invalid-address')).rejects.toThrow(InvalidAddressError);
    });

    it('should validate address', async () => {
      const validAddress = alice.address;
      const balance = await accounts.getBalance(validAddress);

      expect(balance).toBeDefined();
    });
  });

  describe('StakingWrapper', () => {
    let staking: StakingWrapper;

    beforeAll(() => {
      staking = new StakingWrapper(api);
    });

    it('should get validators', async () => {
      const validators = await staking.getValidators();

      expect(Array.isArray(validators)).toBe(true);
    });

    it('should get current era', async () => {
      const era = await staking.getCurrentEra();

      expect(typeof era).toBe('number');
      expect(era).toBeGreaterThanOrEqual(0);
    });

    it('should get minimum stake', async () => {
      const minStake = await staking.getMinimumStake();

      expect(minStake).toBeGreaterThanOrEqual(0n);
    });

    it('should get unbonding period', async () => {
      const period = await staking.getUnbondingPeriod();

      expect(typeof period).toBe('number');
      expect(period).toBeGreaterThan(0);
    });

    it('should get staking info', async () => {
      const info = await staking.getStakingInfo(alice.address);

      expect(info).toBeDefined();
      expect(info.staked).toBeGreaterThanOrEqual(0n);
      expect(['bonded', 'unbonding', 'idle']).toContain(info.status);
    });

    it('should estimate rewards', async () => {
      const rewards = await staking.estimateRewards(1000000000000000000n);

      expect(rewards).toBeDefined();
      expect(rewards.daily).toBeGreaterThanOrEqual(0n);
      expect(rewards.monthly).toBeGreaterThanOrEqual(0n);
      expect(rewards.yearly).toBeGreaterThanOrEqual(0n);
      expect(rewards.apy).toBeGreaterThan(0);
    });

    it('should get validator status for non-validator', async () => {
      const status = await staking.getValidatorStatus(bob.address);

      // Bob is not a validator in test setup
      expect(status).toBeDefined();
    });

    it('should throw error for invalid address', async () => {
      await expect(staking.getStakingInfo('invalid-address')).rejects.toThrow(InvalidAddressError);
    });
  });

  describe('GovernanceWrapper', () => {
    let governance: GovernanceWrapper;

    beforeAll(() => {
      governance = new GovernanceWrapper(api);
    });

    it('should get active proposals', async () => {
      const proposals = await governance.getActiveProposals();

      expect(Array.isArray(proposals)).toBe(true);
    });

    it('should get proposal count', async () => {
      const count = await governance.getProposalCount();

      expect(typeof count).toBe('number');
      expect(count).toBeGreaterThanOrEqual(0);
    });

    it('should get voting period', async () => {
      const period = await governance.getVotingPeriod();

      expect(typeof period).toBe('number');
      expect(period).toBeGreaterThan(0);
    });

    it('should get minimum proposal stake', async () => {
      const minStake = await governance.getMinimumProposalStake();

      expect(minStake).toBeGreaterThanOrEqual(0n);
    });

    it('should get voting power', async () => {
      const power = await governance.getVotingPower(alice.address);

      expect(power).toBeGreaterThanOrEqual(0n);
    });

    it('should get proposals with pagination', async () => {
      const result = await governance.getProposals(0, 10);

      expect(result).toBeDefined();
      expect(result.items).toBeDefined();
      expect(Array.isArray(result.items)).toBe(true);
      expect(result.total).toBeGreaterThanOrEqual(0);
      expect(result.page).toBe(0);
      expect(result.limit).toBe(10);
    });

    it('should check pagination bounds', async () => {
      const result = await governance.getProposals(0, 5);

      expect(result.hasNext).toBe(result.total > 5);
      expect(result.hasPrevious).toBe(false);
    });

    it('should get proposal by id (non-existent)', async () => {
      const proposal = await governance.getProposal(999999);

      expect(proposal).toBeNull();
    });

    it('should check if user has voted (non-existent proposal)', async () => {
      const hasVoted = await governance.hasVoted(999999, alice.address);

      expect(typeof hasVoted).toBe('boolean');
    });
  });

  describe('Wrapper Error Handling', () => {
    it('should handle disconnected API in AccountsWrapper', async () => {
      const disconnectedApi = await ApiPromise.create({ provider: new WsProvider('ws://127.0.0.1:9944') });
      await disconnectedApi.disconnect();

      const accounts = new AccountsWrapper(disconnectedApi);

      await expect(accounts.getBalance(alice.address)).rejects.toThrow(NotConnectedError);
    });

    it('should validate addresses in all wrappers', async () => {
      const accounts = new AccountsWrapper(api);
      const staking = new StakingWrapper(api);
      const governance = new GovernanceWrapper(api);

      const invalidAddress = 'not-a-valid-address';

      await expect(accounts.getBalance(invalidAddress)).rejects.toThrow(InvalidAddressError);
      await expect(staking.getStakingInfo(invalidAddress)).rejects.toThrow(InvalidAddressError);
    });
  });

  describe('Wrapper Integration', () => {
    it('should get comprehensive account state', async () => {
      const accounts = new AccountsWrapper(api);
      const staking = new StakingWrapper(api);

      const [accountInfo, stakingInfo] = await Promise.all([
        accounts.getAccountInfo(alice.address),
        staking.getStakingInfo(alice.address),
      ]);

      expect(accountInfo).toBeDefined();
      expect(stakingInfo).toBeDefined();
      expect(accountInfo.balance.total).toBeGreaterThanOrEqual(0n);
      expect(stakingInfo.staked).toBeGreaterThanOrEqual(0n);
    });

    it('should get comprehensive validator data', async () => {
      const staking = new StakingWrapper(api);

      const validators = await staking.getValidators();

      if (validators.length > 0) {
        const validatorAddress = validators[0];
        const details = await staking.getValidatorDetails(validatorAddress);

        expect(details).toBeDefined();
        expect(details.totalStake).toBeGreaterThanOrEqual(0n);
        expect(details.nominators).toBeGreaterThanOrEqual(0);
        expect(details.commission).toBeGreaterThanOrEqual(0);
      }
    });
  });
});
