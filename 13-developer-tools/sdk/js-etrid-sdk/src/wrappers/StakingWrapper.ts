/**
 * Type-safe wrapper for Staking pallet
 */

import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { ValidatorStatus, StakingInfo, TransactionResult } from '../types/enhanced';
import { TransactionBuilder } from '../builders/TransactionBuilder';
import { NotConnectedError, InvalidAddressError, StakingError } from '../errors/EtridErrors';
import { decodeAddress } from '@polkadot/util-crypto';

/**
 * Staking wrapper for staking operations
 */
export class StakingWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Ensure API is connected
   */
  private ensureConnected(): void {
    if (!this.api.isConnected) {
      throw new NotConnectedError();
    }
  }

  /**
   * Validate address format
   */
  private validateAddress(address: string): void {
    try {
      decodeAddress(address);
    } catch (error) {
      throw new InvalidAddressError(address, 'Invalid SS58 address format');
    }
  }

  /**
   * Get validator status
   */
  async getValidatorStatus(address: string): Promise<ValidatorStatus | null> {
    this.ensureConnected();
    this.validateAddress(address);

    try {
      // Query validator preferences
      const prefs = await this.api.query.staking.validators(address);

      // Check if address is in validator set
      const validators = await this.api.query.session.validators();
      const isActive = validators.some((v: any) => v.toString() === address);

      // Get total stake
      const staking = await this.api.query.staking.ledger(address);

      if (staking.isNone) {
        return null;
      }

      const ledger = staking.unwrap();

      return {
        isValidator: !prefs.isEmpty,
        stake: ledger.total.toBigInt(),
        commission: prefs.commission.toNumber() / 10000000, // Convert from per-billion
        active: isActive,
        selfStake: ledger.active.toBigInt(),
      };
    } catch (error) {
      throw new StakingError('Failed to get validator status', { address, error });
    }
  }

  /**
   * Get staking information for an account
   */
  async getStakingInfo(address: string): Promise<StakingInfo> {
    this.ensureConnected();
    this.validateAddress(address);

    try {
      const ledger = await this.api.query.staking.ledger(address);

      if (ledger.isNone) {
        return {
          staked: 0n,
          rewards: 0n,
          status: 'idle',
        };
      }

      const unwrapped = ledger.unwrap();
      const controller = await this.api.query.staking.bonded(address);

      // Get validator if nominated
      const nominators = await this.api.query.staking.nominators(address);
      const validator = nominators.isSome ? nominators.unwrap().targets[0]?.toString() : undefined;

      // Check if unbonding
      const unlocking = unwrapped.unlocking.toJSON() as any[];
      const isUnbonding = unlocking.length > 0;
      const unbondingPeriod = isUnbonding ? unlocking[0]?.era : undefined;

      return {
        staked: unwrapped.total.toBigInt(),
        validator,
        rewards: 0n, // Would need to query rewards separately
        unbondingPeriod,
        status: isUnbonding ? 'unbonding' : unwrapped.active.toBigInt() > 0n ? 'bonded' : 'idle',
      };
    } catch (error) {
      throw new StakingError('Failed to get staking info', { address, error });
    }
  }

  /**
   * Get all validators
   */
  async getValidators(): Promise<string[]> {
    this.ensureConnected();

    try {
      const validators = await this.api.query.session.validators();
      return validators.map((v: any) => v.toString());
    } catch (error) {
      throw new StakingError('Failed to get validators', { error });
    }
  }

  /**
   * Get validator details
   */
  async getValidatorDetails(address: string): Promise<{
    status: ValidatorStatus | null;
    nominators: number;
    totalStake: bigint;
    commission: number;
  }> {
    this.ensureConnected();
    this.validateAddress(address);

    const status = await this.getValidatorStatus(address);

    if (!status) {
      return {
        status: null,
        nominators: 0,
        totalStake: 0n,
        commission: 0,
      };
    }

    // Get nominator count
    const exposure = await this.api.query.staking.erasStakers(
      await this.getCurrentEra(),
      address
    );

    return {
      status,
      nominators: exposure.others.length,
      totalStake: exposure.total.toBigInt(),
      commission: status.commission,
    };
  }

  /**
   * Get current era
   */
  async getCurrentEra(): Promise<number> {
    this.ensureConnected();

    const activeEra = await this.api.query.staking.activeEra();
    return activeEra.unwrap().index.toNumber();
  }

  /**
   * Get minimum staking amount
   */
  async getMinimumStake(): Promise<bigint> {
    this.ensureConnected();

    const minNominatorBond = await this.api.query.staking.minNominatorBond();
    return minNominatorBond.toBigInt();
  }

  /**
   * Bond tokens for staking
   */
  async bond(
    from: KeyringPair,
    validator: string,
    amount: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();
    this.validateAddress(validator);

    return new TransactionBuilder(this.api)
      .stake(validator, amount)
      .submit(from);
  }

  /**
   * Bond additional tokens
   */
  async bondAdditional(
    from: KeyringPair,
    amount: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();

    return new TransactionBuilder(this.api)
      .bondAdditional(amount)
      .submit(from);
  }

  /**
   * Unbond tokens
   */
  async unbond(
    from: KeyringPair,
    amount: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();

    return new TransactionBuilder(this.api)
      .unbond(amount)
      .submit(from);
  }

  /**
   * Nominate validators
   */
  async nominate(
    from: KeyringPair,
    validators: string[]
  ): Promise<TransactionResult> {
    this.ensureConnected();

    // Validate all addresses
    validators.forEach(v => this.validateAddress(v));

    return new TransactionBuilder(this.api)
      .nominate(validators)
      .submit(from);
  }

  /**
   * Withdraw unbonded tokens
   */
  async withdrawUnbonded(
    from: KeyringPair,
    numSlashingSpans: number = 0
  ): Promise<TransactionResult> {
    this.ensureConnected();

    const tx = this.api.tx.staking.withdrawUnbonded(numSlashingSpans);
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Chill (stop nominating/validating)
   */
  async chill(from: KeyringPair): Promise<TransactionResult> {
    this.ensureConnected();

    const tx = this.api.tx.staking.chill();
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Get staking rewards for an address
   */
  async getRewards(address: string, era?: number): Promise<bigint> {
    this.ensureConnected();
    this.validateAddress(address);

    try {
      const currentEra = era ?? await this.getCurrentEra();
      const points = await this.api.query.staking.erasRewardPoints(currentEra);

      // This is simplified - actual reward calculation is more complex
      const individual = (points.individual as any).get(address);
      return individual ? individual.toBigInt() : 0n;
    } catch (error) {
      throw new StakingError('Failed to get rewards', { address, era, error });
    }
  }

  /**
   * Get unbonding period in eras
   */
  async getUnbondingPeriod(): Promise<number> {
    this.ensureConnected();

    const bondingDuration = await this.api.consts.staking.bondingDuration;
    return bondingDuration.toNumber();
  }

  /**
   * Estimate staking rewards for an amount
   */
  async estimateRewards(amount: bigint): Promise<{
    daily: bigint;
    monthly: bigint;
    yearly: bigint;
    apy: number;
  }> {
    this.ensureConnected();

    try {
      // Get total staked
      const totalIssuance = await this.api.query.balances.totalIssuance();
      const totalStaked = totalIssuance.toBigInt() / 2n; // Simplified assumption

      // Estimate APY (simplified - actual calculation is more complex)
      const apy = 15.0; // 15% APY as example
      const yearlyRewards = (amount * BigInt(Math.floor(apy * 100))) / 10000n;
      const monthlyRewards = yearlyRewards / 12n;
      const dailyRewards = yearlyRewards / 365n;

      return {
        daily: dailyRewards,
        monthly: monthlyRewards,
        yearly: yearlyRewards,
        apy,
      };
    } catch (error) {
      throw new StakingError('Failed to estimate rewards', { amount, error });
    }
  }

  /**
   * Get all nominators for a validator
   */
  async getNominators(validatorAddress: string): Promise<Array<{
    address: string;
    stake: bigint;
  }>> {
    this.ensureConnected();
    this.validateAddress(validatorAddress);

    try {
      const currentEra = await this.getCurrentEra();
      const exposure = await this.api.query.staking.erasStakers(currentEra, validatorAddress);

      return exposure.others.map((nominator: any) => ({
        address: nominator.who.toString(),
        stake: nominator.value.toBigInt(),
      }));
    } catch (error) {
      throw new StakingError('Failed to get nominators', { validatorAddress, error });
    }
  }

  /**
   * Get validator commission history
   */
  async getCommissionHistory(
    validatorAddress: string,
    eras: number = 10
  ): Promise<Array<{ era: number; commission: number }>> {
    this.ensureConnected();
    this.validateAddress(validatorAddress);

    try {
      const currentEra = await this.getCurrentEra();
      const history: Array<{ era: number; commission: number }> = [];

      for (let i = 0; i < eras; i++) {
        const era = currentEra - i;
        if (era < 0) break;

        const prefs = await this.api.query.staking.erasValidatorPrefs(era, validatorAddress);
        history.push({
          era,
          commission: prefs.commission.toNumber() / 10000000,
        });
      }

      return history;
    } catch (error) {
      throw new StakingError('Failed to get commission history', { validatorAddress, error });
    }
  }

  /**
   * Set validator commission
   */
  async setCommission(
    from: KeyringPair,
    commission: number
  ): Promise<TransactionResult> {
    this.ensureConnected();

    if (commission < 0 || commission > 100) {
      throw new StakingError('Commission must be between 0 and 100');
    }

    const commissionInPerbill = Math.floor(commission * 10000000); // Convert to per-billion
    const tx = this.api.tx.staking.validate({ commission: commissionInPerbill });
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Get total network staking stats
   */
  async getNetworkStats(): Promise<{
    totalStaked: bigint;
    totalIssuance: bigint;
    stakingRate: number;
    activeValidators: number;
    waitingValidators: number;
  }> {
    this.ensureConnected();

    try {
      const totalIssuance = await this.api.query.balances.totalIssuance();
      const activeEra = await this.api.query.staking.activeEra();
      const currentEra = activeEra.unwrap().index.toNumber();

      // Get total staked
      const erasTotalStake = await this.api.query.staking.erasTotalStake(currentEra);
      const totalStaked = erasTotalStake.toBigInt();

      // Get validator counts
      const validators = await this.api.query.session.validators();
      const activeValidators = validators.length;

      // Calculate staking rate
      const stakingRate = Number((totalStaked * 10000n) / totalIssuance.toBigInt()) / 100;

      return {
        totalStaked,
        totalIssuance: totalIssuance.toBigInt(),
        stakingRate,
        activeValidators,
        waitingValidators: 0, // Would need additional queries
      };
    } catch (error) {
      throw new StakingError('Failed to get network stats', { error });
    }
  }
}
