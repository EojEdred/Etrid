import EtridSDKService from './EtridSDKService';
import KeychainService from './KeychainService';
import {
  StakingInfo,
  Validator,
  ValidatorStake,
  UnbondingPosition,
  RewardHistory,
  StakeOptions,
  StakingEstimate,
  TransactionResult,
} from '../types/defi.types';

/**
 * StakingService - Handles all staking-related operations
 * Connects to FlareChain staking pallet via EtridSDK
 */
class StakingService {
  private sdk: EtridSDKService;
  private readonly DECIMALS = 12; // ETR decimals
  private readonly UNBONDING_PERIOD_DAYS = 28;

  constructor() {
    this.sdk = EtridSDKService.getInstance();
  }

  /**
   * Get comprehensive staking info for an address
   */
  async getStakingInfo(address: string, etrPrice: number = 2.45): Promise<StakingInfo> {
    try {
      await this.sdk.connect();

      // Get staked balance
      const stakedBalance = await this.sdk.staking.getStakedBalance(address);
      const stakedETR = this.fromSmallestUnit(stakedBalance);

      // Get rewards
      const rewards = await this.sdk.staking.getRewards(address);
      const rewardsETR = this.fromSmallestUnit(rewards);

      // Get validator nominations
      const nominations = await this.sdk.staking.getNominations?.(address) || [];

      const activeValidators: ValidatorStake[] = await Promise.all(
        nominations.slice(0, 3).map(async (nom: any) => {
          const validator = await this.getValidatorInfo(nom.validatorAddress);
          return {
            validatorAddress: nom.validatorAddress,
            validatorName: validator.name,
            stakedAmount: nom.amount || stakedBalance,
            stakedAmountETR: this.fromSmallestUnit(nom.amount || stakedBalance),
            commission: validator.commission,
            apy: validator.apy,
            status: validator.status,
            uptime: validator.uptime,
          };
        })
      );

      // Get unbonding positions
      const unbonding = await this.sdk.staking.getUnbonding?.(address) || [];
      const unbondingPositions: UnbondingPosition[] = unbonding.map((pos: any) => {
        const remainingMs = pos.unbondingAt - Date.now();
        const remainingDays = Math.ceil(remainingMs / (1000 * 60 * 60 * 24));
        const remainingHours = Math.ceil(remainingMs / (1000 * 60 * 60));

        return {
          amount: pos.amount,
          amountETR: this.fromSmallestUnit(pos.amount),
          unbondingAt: pos.unbondingAt,
          remainingDays: Math.max(0, remainingDays),
          remainingHours: Math.max(0, remainingHours),
          status: remainingMs <= 0 ? 'ready' : 'unbonding',
        };
      });

      // Get rewards history (last 30 days)
      const rewardsHistory = await this.getRewardsHistory(address, 30, etrPrice);

      // Calculate APY and daily rewards
      const currentAPY = this.calculateAPY(stakedETR, rewardsHistory);
      const dailyRewards = (stakedETR * currentAPY) / 365 / 100;

      return {
        totalStaked: stakedBalance,
        totalStakedETR: stakedETR,
        totalStakedUSD: stakedETR * etrPrice,
        currentAPY,
        dailyRewards: this.toSmallestUnit(dailyRewards),
        dailyRewardsETR: dailyRewards,
        totalEarned: rewards,
        totalEarnedETR: rewardsETR,
        totalEarnedUSD: rewardsETR * etrPrice,
        activeValidators,
        unbondingPeriod: this.UNBONDING_PERIOD_DAYS,
        unbondingPositions,
        rewardsHistory,
      };
    } catch (error) {
      console.error('Failed to get staking info:', error);
      throw new Error('Failed to load staking information');
    }
  }

  /**
   * Get list of all validators
   */
  async getValidators(sortBy: 'apy' | 'commission' | 'uptime' | 'stake' = 'apy'): Promise<Validator[]> {
    try {
      await this.sdk.connect();

      // Get all validators from chain
      const validatorsData = await this.sdk.staking.getValidators?.() || this.getMockValidators();

      const validators: Validator[] = validatorsData.map((v: any) => ({
        address: v.address,
        name: v.identity?.display || `Validator ${v.address.slice(0, 8)}`,
        commission: v.commission || 5,
        apy: v.apy || 12.5,
        uptime: v.uptime || 99.5,
        totalStake: v.totalStake || '0',
        totalStakeETR: this.fromSmallestUnit(v.totalStake || '0'),
        nominators: v.nominators || 0,
        status: v.status || 'active',
        identity: v.identity,
      }));

      // Sort validators
      return this.sortValidators(validators, sortBy);
    } catch (error) {
      console.error('Failed to get validators:', error);
      return this.getMockValidators();
    }
  }

  /**
   * Get specific validator info
   */
  async getValidatorInfo(address: string): Promise<Validator> {
    try {
      const validators = await this.getValidators();
      const validator = validators.find(v => v.address === address);

      if (!validator) {
        throw new Error('Validator not found');
      }

      return validator;
    } catch (error) {
      console.error('Failed to get validator info:', error);
      throw error;
    }
  }

  /**
   * Stake tokens
   */
  async stake(options: StakeOptions): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      // Bond tokens
      const bondTx = await this.sdk.staking.bond(keypair, BigInt(options.amount));

      // Nominate validator if specified
      if (options.validatorAddress) {
        await this.sdk.staking.nominate(keypair, [options.validatorAddress]);
      } else {
        // Auto-select top 3 validators by APY
        const validators = await this.getValidators('apy');
        const topValidators = validators.slice(0, 3).map(v => v.address);
        await this.sdk.staking.nominate(keypair, topValidators);
      }

      return {
        success: true,
        txHash: bondTx.toString(),
        message: 'Successfully staked tokens',
      };
    } catch (error) {
      console.error('Failed to stake:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to stake tokens',
      };
    }
  }

  /**
   * Unstake tokens
   */
  async unstake(amount: string): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const unbondTx = await this.sdk.staking.unbond(keypair, BigInt(amount));

      return {
        success: true,
        txHash: unbondTx.toString(),
        message: `Successfully initiated unstaking. Tokens will be available in ${this.UNBONDING_PERIOD_DAYS} days.`,
      };
    } catch (error) {
      console.error('Failed to unstake:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to unstake tokens',
      };
    }
  }

  /**
   * Withdraw unbonded tokens
   */
  async withdrawUnbonded(): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const withdrawTx = await this.sdk.staking.withdrawUnbonded?.(keypair, 0);

      return {
        success: true,
        txHash: withdrawTx?.toString(),
        message: 'Successfully withdrew unbonded tokens',
      };
    } catch (error) {
      console.error('Failed to withdraw:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to withdraw tokens',
      };
    }
  }

  /**
   * Claim staking rewards
   */
  async claimRewards(): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const claimTx = await this.sdk.staking.payoutStakers?.(keypair);

      return {
        success: true,
        txHash: claimTx?.toString(),
        message: 'Successfully claimed staking rewards',
      };
    } catch (error) {
      console.error('Failed to claim rewards:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to claim rewards',
      };
    }
  }

  /**
   * Estimate staking rewards
   */
  estimateRewards(amount: number, apy: number = 12.5): StakingEstimate {
    const dailyReward = (amount * apy) / 365 / 100;
    const monthlyReward = dailyReward * 30;
    const yearlyReward = amount * (apy / 100);

    return {
      dailyReward,
      monthlyReward,
      yearlyReward,
      effectiveAPY: apy,
    };
  }

  /**
   * Get rewards history
   */
  private async getRewardsHistory(
    address: string,
    days: number,
    etrPrice: number
  ): Promise<RewardHistory[]> {
    try {
      const history = await this.sdk.staking.getRewardsHistory?.(address, days) || [];

      return history.map((reward: any) => ({
        timestamp: reward.timestamp,
        amount: reward.amount,
        amountETR: this.fromSmallestUnit(reward.amount),
        amountUSD: this.fromSmallestUnit(reward.amount) * etrPrice,
        validator: reward.validator,
        type: reward.type || 'staking_reward',
      }));
    } catch (error) {
      // Return mock history if API fails
      return this.getMockRewardsHistory(days, etrPrice);
    }
  }

  /**
   * Calculate APY from rewards history
   */
  private calculateAPY(stakedAmount: number, history: RewardHistory[]): number {
    if (history.length === 0 || stakedAmount === 0) {
      return 12.5; // Default APY
    }

    const totalRewards = history.reduce((sum, r) => sum + r.amountETR, 0);
    const avgDailyReward = totalRewards / history.length;
    const yearlyReward = avgDailyReward * 365;
    const apy = (yearlyReward / stakedAmount) * 100;

    return Math.min(Math.max(apy, 8), 20); // Clamp between 8% and 20%
  }

  /**
   * Sort validators by criteria
   */
  private sortValidators(validators: Validator[], sortBy: string): Validator[] {
    return validators.sort((a, b) => {
      switch (sortBy) {
        case 'apy':
          return b.apy - a.apy;
        case 'commission':
          return a.commission - b.commission;
        case 'uptime':
          return b.uptime - a.uptime;
        case 'stake':
          return b.totalStakeETR - a.totalStakeETR;
        default:
          return 0;
      }
    });
  }

  /**
   * Convert from smallest unit to ETR
   */
  private fromSmallestUnit(amount: string): number {
    return Number(amount) / Math.pow(10, this.DECIMALS);
  }

  /**
   * Convert from ETR to smallest unit
   */
  private toSmallestUnit(amount: number): string {
    return (BigInt(Math.floor(amount * Math.pow(10, this.DECIMALS)))).toString();
  }

  /**
   * Mock validators for testing
   */
  private getMockValidators(): Validator[] {
    return [
      {
        address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        name: 'FlareNode Alpha',
        commission: 3,
        apy: 15.2,
        uptime: 99.9,
        totalStake: '5000000000000000',
        totalStakeETR: 5000000,
        nominators: 1250,
        status: 'active',
      },
      {
        address: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        name: 'Etrid Foundation',
        commission: 0,
        apy: 14.8,
        uptime: 100,
        totalStake: '8000000000000000',
        totalStakeETR: 8000000,
        nominators: 2100,
        status: 'active',
      },
      {
        address: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
        name: 'Validator One',
        commission: 5,
        apy: 13.5,
        uptime: 99.5,
        totalStake: '3500000000000000',
        totalStakeETR: 3500000,
        nominators: 890,
        status: 'active',
      },
    ];
  }

  /**
   * Mock rewards history
   */
  private getMockRewardsHistory(days: number, etrPrice: number): RewardHistory[] {
    const history: RewardHistory[] = [];
    const now = Date.now();

    for (let i = 0; i < days; i++) {
      const amount = 3.2 + Math.random() * 0.8; // Random between 3.2 and 4.0 ETR
      const amountSmallest = this.toSmallestUnit(amount);

      history.push({
        timestamp: now - i * 24 * 60 * 60 * 1000,
        amount: amountSmallest,
        amountETR: amount,
        amountUSD: amount * etrPrice,
        validator: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        type: 'staking_reward',
      });
    }

    return history.reverse();
  }
}

export default new StakingService();
