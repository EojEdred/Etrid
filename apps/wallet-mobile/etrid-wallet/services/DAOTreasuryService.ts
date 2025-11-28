/**
 * DAO Treasury Service
 * Handles treasury management, asset tracking, and spending proposals
 */

import {
  Treasury,
  TreasuryAsset,
  TreasuryTransaction,
  TreasuryAnalytics,
  SpendProposal,
  Proposal,
} from '@/types/dao';

export class DAOTreasuryService {
  private treasuries: Map<string, Treasury> = new Map();

  /**
   * Get treasury for a DAO
   */
  async getTreasury(daoId: string): Promise<Treasury> {
    let treasury = this.treasuries.get(daoId);

    if (!treasury) {
      // Initialize treasury if not exists
      treasury = await this.initializeTreasury(daoId);
    }

    // Update asset values
    treasury = await this.updateAssetValues(treasury);

    return treasury;
  }

  /**
   * Get treasury assets
   */
  async getTreasuryAssets(daoId: string): Promise<TreasuryAsset[]> {
    const treasury = await this.getTreasury(daoId);
    return treasury.assets;
  }

  /**
   * Get treasury transactions
   */
  async getTreasuryHistory(daoId: string): Promise<TreasuryTransaction[]> {
    const treasury = await this.getTreasury(daoId);
    return treasury.transactions;
  }

  /**
   * Get treasury analytics
   */
  async getTreasuryAnalytics(
    daoId: string,
    period: '24h' | '7d' | '30d' | 'all' = '30d'
  ): Promise<TreasuryAnalytics> {
    const treasury = await this.getTreasury(daoId);
    const transactions = treasury.transactions;

    // Calculate time range
    const now = new Date();
    const startTime = this.getStartTime(now, period);

    // Filter transactions by period
    const periodTransactions = transactions.filter(
      (tx) => tx.timestamp >= startTime
    );

    // Calculate inflows and outflows
    const totalInflows = periodTransactions
      .filter((tx) => tx.type === 'inflow')
      .reduce((sum, tx) => sum + parseFloat(tx.valueUsd), 0);

    const totalOutflows = periodTransactions
      .filter((tx) => tx.type === 'outflow')
      .reduce((sum, tx) => sum + parseFloat(tx.valueUsd), 0);

    const netChange = totalInflows - totalOutflows;
    const currentValue = parseFloat(treasury.totalValue);
    const previousValue = currentValue - netChange;
    const changePercentage =
      previousValue > 0 ? (netChange / previousValue) * 100 : 0;

    // Generate time series data
    const inflowsOverTime = this.generateTimeSeriesData(
      periodTransactions.filter((tx) => tx.type === 'inflow'),
      period
    );

    const outflowsOverTime = this.generateTimeSeriesData(
      periodTransactions.filter((tx) => tx.type === 'outflow'),
      period
    );

    return {
      totalInflows: totalInflows.toString(),
      totalOutflows: totalOutflows.toString(),
      netChange: netChange.toString(),
      changePercentage,
      period,
      inflowsOverTime,
      outflowsOverTime,
    };
  }

  /**
   * Propose spending from treasury
   */
  async proposeSpend(daoId: string, spend: SpendProposal): Promise<Proposal> {
    // Validate spend proposal
    if (!spend.recipient || !spend.asset || !spend.amount) {
      throw new Error('Invalid spend proposal');
    }

    // Check if treasury has sufficient funds
    const treasury = await this.getTreasury(daoId);
    const asset = treasury.assets.find((a) => a.asset === spend.asset);

    if (!asset || parseFloat(asset.amount) < parseFloat(spend.amount)) {
      throw new Error('Insufficient treasury funds');
    }

    // Create proposal
    const proposal = {
      title: `Treasury Spend: ${spend.amount} ${spend.asset}`,
      description: `Send ${spend.amount} ${spend.asset} to ${spend.recipient}\n\nReason: ${spend.reason}`,
      type: 'treasury' as const,
      executionData: {
        action: 'transfer',
        recipient: spend.recipient,
        asset: spend.asset,
        amount: spend.amount,
      },
    };

    // TODO: Create proposal using DAOProposalService
    // const createdProposal = await daoProposalService.createProposal(daoId, proposal);
    // return createdProposal;

    return {} as Proposal; // Placeholder
  }

  /**
   * Execute treasury transfer (called when proposal is executed)
   */
  async executeTreasuryTransfer(
    daoId: string,
    recipient: string,
    asset: string,
    amount: string
  ): Promise<TreasuryTransaction> {
    const treasury = await this.getTreasury(daoId);

    // Find asset
    const assetIndex = treasury.assets.findIndex((a) => a.asset === asset);
    if (assetIndex === -1) {
      throw new Error('Asset not found in treasury');
    }

    const treasuryAsset = treasury.assets[assetIndex];

    // Check balance
    if (parseFloat(treasuryAsset.amount) < parseFloat(amount)) {
      throw new Error('Insufficient balance');
    }

    // Update asset balance
    treasuryAsset.amount = (
      parseFloat(treasuryAsset.amount) - parseFloat(amount)
    ).toString();
    treasuryAsset.valueUsd = (
      parseFloat(treasuryAsset.valueUsd) -
      parseFloat(amount) * this.getAssetPrice(asset)
    ).toString();

    // Recalculate percentages
    const totalValue = treasury.assets.reduce(
      (sum, a) => sum + parseFloat(a.valueUsd),
      0
    );
    treasury.assets.forEach((a) => {
      a.percentage = (parseFloat(a.valueUsd) / totalValue) * 100;
    });

    treasury.totalValue = totalValue.toString();

    // Create transaction record
    const transaction: TreasuryTransaction = {
      id: Date.now().toString(),
      type: 'outflow',
      asset,
      amount,
      valueUsd: (parseFloat(amount) * this.getAssetPrice(asset)).toString(),
      to: recipient,
      description: 'Treasury spend proposal executed',
      timestamp: new Date(),
      txHash: '0x' + Math.random().toString(16).substring(2),
    };

    treasury.transactions.unshift(transaction);

    this.treasuries.set(daoId, treasury);
    await this.saveTreasuries();

    // TODO: Execute actual on-chain transfer
    // await this.transferOnChain(daoId, recipient, asset, amount);

    return transaction;
  }

  /**
   * Add funds to treasury (e.g., from donations)
   */
  async addTreasuryFunds(
    daoId: string,
    asset: string,
    amount: string,
    from?: string,
    description?: string
  ): Promise<TreasuryTransaction> {
    const treasury = await this.getTreasury(daoId);

    // Find or create asset
    let assetIndex = treasury.assets.findIndex((a) => a.asset === asset);

    if (assetIndex === -1) {
      // Add new asset
      treasury.assets.push({
        asset,
        symbol: asset,
        amount: '0',
        valueUsd: '0',
        percentage: 0,
      });
      assetIndex = treasury.assets.length - 1;
    }

    const treasuryAsset = treasury.assets[assetIndex];

    // Update asset balance
    treasuryAsset.amount = (
      parseFloat(treasuryAsset.amount) + parseFloat(amount)
    ).toString();
    treasuryAsset.valueUsd = (
      parseFloat(treasuryAsset.valueUsd) +
      parseFloat(amount) * this.getAssetPrice(asset)
    ).toString();

    // Recalculate percentages
    const totalValue = treasury.assets.reduce(
      (sum, a) => sum + parseFloat(a.valueUsd),
      0
    );
    treasury.assets.forEach((a) => {
      a.percentage = (parseFloat(a.valueUsd) / totalValue) * 100;
    });

    treasury.totalValue = totalValue.toString();

    // Create transaction record
    const transaction: TreasuryTransaction = {
      id: Date.now().toString(),
      type: 'inflow',
      asset,
      amount,
      valueUsd: (parseFloat(amount) * this.getAssetPrice(asset)).toString(),
      from,
      description: description || 'Treasury deposit',
      timestamp: new Date(),
      txHash: '0x' + Math.random().toString(16).substring(2),
    };

    treasury.transactions.unshift(transaction);

    this.treasuries.set(daoId, treasury);
    await this.saveTreasuries();

    return transaction;
  }

  /**
   * Initialize treasury for a new DAO
   */
  private async initializeTreasury(daoId: string): Promise<Treasury> {
    const treasury: Treasury = {
      daoId,
      totalValue: '0',
      assets: [],
      transactions: [],
      analytics: {
        totalInflows: '0',
        totalOutflows: '0',
        netChange: '0',
        changePercentage: 0,
        period: '30d',
        inflowsOverTime: [],
        outflowsOverTime: [],
      },
    };

    this.treasuries.set(daoId, treasury);
    await this.saveTreasuries();

    return treasury;
  }

  /**
   * Update asset values with current prices
   */
  private async updateAssetValues(treasury: Treasury): Promise<Treasury> {
    let totalValue = 0;

    for (const asset of treasury.assets) {
      const price = this.getAssetPrice(asset.asset);
      asset.valueUsd = (parseFloat(asset.amount) * price).toString();
      totalValue += parseFloat(asset.valueUsd);
    }

    // Update percentages
    treasury.assets.forEach((asset) => {
      asset.percentage =
        totalValue > 0 ? (parseFloat(asset.valueUsd) / totalValue) * 100 : 0;
    });

    treasury.totalValue = totalValue.toString();

    return treasury;
  }

  /**
   * Get asset price in USD (mock - would fetch from price oracle)
   */
  private getAssetPrice(asset: string): number {
    // TODO: Fetch real prices from oracle
    const mockPrices: { [key: string]: number } = {
      ETH: 2000,
      ETRID: 10,
      USDC: 1,
      USDT: 1,
      DAI: 1,
    };

    return mockPrices[asset] || 1;
  }

  /**
   * Get start time for analytics period
   */
  private getStartTime(now: Date, period: '24h' | '7d' | '30d' | 'all'): Date {
    const startTime = new Date(now);

    switch (period) {
      case '24h':
        startTime.setHours(startTime.getHours() - 24);
        break;
      case '7d':
        startTime.setDate(startTime.getDate() - 7);
        break;
      case '30d':
        startTime.setDate(startTime.getDate() - 30);
        break;
      case 'all':
        startTime.setFullYear(2020); // Beginning of time
        break;
    }

    return startTime;
  }

  /**
   * Generate time series data for charts
   */
  private generateTimeSeriesData(
    transactions: TreasuryTransaction[],
    period: '24h' | '7d' | '30d' | 'all'
  ): { timestamp: Date; value: number }[] {
    // TODO: Implement proper time series aggregation
    const data: { timestamp: Date; value: number }[] = [];

    // Group transactions by time buckets
    const buckets = this.getTimeBuckets(period);

    for (const bucket of buckets) {
      const bucketTransactions = transactions.filter(
        (tx) =>
          tx.timestamp >= bucket.start && tx.timestamp < bucket.end
      );

      const value = bucketTransactions.reduce(
        (sum, tx) => sum + parseFloat(tx.valueUsd),
        0
      );

      data.push({
        timestamp: bucket.start,
        value,
      });
    }

    return data;
  }

  /**
   * Get time buckets for period
   */
  private getTimeBuckets(
    period: '24h' | '7d' | '30d' | 'all'
  ): { start: Date; end: Date }[] {
    const now = new Date();
    const buckets: { start: Date; end: Date }[] = [];

    let bucketCount: number;
    let bucketSize: number; // milliseconds

    switch (period) {
      case '24h':
        bucketCount = 24;
        bucketSize = 60 * 60 * 1000; // 1 hour
        break;
      case '7d':
        bucketCount = 7;
        bucketSize = 24 * 60 * 60 * 1000; // 1 day
        break;
      case '30d':
        bucketCount = 30;
        bucketSize = 24 * 60 * 60 * 1000; // 1 day
        break;
      case 'all':
        bucketCount = 12;
        bucketSize = 30 * 24 * 60 * 60 * 1000; // 1 month
        break;
    }

    for (let i = 0; i < bucketCount; i++) {
      const end = new Date(now.getTime() - i * bucketSize);
      const start = new Date(end.getTime() - bucketSize);
      buckets.push({ start, end });
    }

    return buckets.reverse();
  }

  // Storage methods
  private async saveTreasuries(): Promise<void> {
    const treasuriesArray = Array.from(this.treasuries.values());
    // TODO: Persist to AsyncStorage or API
    // await AsyncStorage.setItem('dao_treasuries', JSON.stringify(treasuriesArray));
  }

  private async loadTreasuries(): Promise<void> {
    // TODO: Load from AsyncStorage or API
    // const treasuriesJson = await AsyncStorage.getItem('dao_treasuries');
    // if (treasuriesJson) {
    //   const treasuries = JSON.parse(treasuriesJson);
    //   treasuries.forEach((treasury: Treasury) => {
    //     this.treasuries.set(treasury.daoId, treasury);
    //   });
    // }
  }
}

export const daoTreasuryService = new DAOTreasuryService();
