import EtridSDKService from './EtridSDKService';

export interface Transaction {
  id: string;
  hash: string;
  type: 'sent' | 'received' | 'staked' | 'unstaked' | 'reward' | 'bridge';
  amount: string;
  currency: string;
  from: string;
  to: string;
  timestamp: number;
  status: 'pending' | 'confirmed' | 'failed';
  blockNumber?: number;
  fee?: string;
  memo?: string;
}

/**
 * Service for fetching and managing transaction history
 */
class TransactionService {
  private sdk: EtridSDKService;

  constructor() {
    this.sdk = EtridSDKService.getInstance();
  }

  /**
   * Fetch transaction history for an address
   */
  public async getTransactionHistory(
    address: string,
    limit: number = 50,
    offset: number = 0
  ): Promise<Transaction[]> {
    try {
      // In production, this would query the blockchain or indexer
      // For now, return mock data
      return this.getMockTransactions(address, limit, offset);
    } catch (error) {
      console.error('Error fetching transaction history:', error);
      throw new Error('Failed to fetch transaction history');
    }
  }

  /**
   * Get transactions by type
   */
  public async getTransactionsByType(
    address: string,
    type: Transaction['type'],
    limit: number = 50
  ): Promise<Transaction[]> {
    try {
      const allTransactions = await this.getTransactionHistory(address, limit);
      return allTransactions.filter(tx => tx.type === type);
    } catch (error) {
      console.error('Error fetching transactions by type:', error);
      throw new Error('Failed to fetch transactions');
    }
  }

  /**
   * Get pending transactions
   */
  public async getPendingTransactions(address: string): Promise<Transaction[]> {
    try {
      const allTransactions = await this.getTransactionHistory(address, 20);
      return allTransactions.filter(tx => tx.status === 'pending');
    } catch (error) {
      console.error('Error fetching pending transactions:', error);
      return [];
    }
  }

  /**
   * Get transaction details by hash
   */
  public async getTransactionDetails(hash: string): Promise<Transaction | null> {
    try {
      // In production, query blockchain for transaction details
      const mockTx = this.getMockTransactions('', 1, 0)[0];
      return { ...mockTx, hash };
    } catch (error) {
      console.error('Error fetching transaction details:', error);
      return null;
    }
  }

  /**
   * Format transaction for display
   */
  public formatTransaction(tx: Transaction, currentAddress: string): {
    title: string;
    subtitle: string;
    amount: string;
    isPositive: boolean;
  } {
    const isReceived = tx.to.toLowerCase() === currentAddress.toLowerCase();
    const isPositive = isReceived || tx.type === 'reward';

    let title = '';
    let subtitle = '';

    switch (tx.type) {
      case 'sent':
        title = 'Sent';
        subtitle = `To ${this.shortenAddress(tx.to)}`;
        break;
      case 'received':
        title = 'Received';
        subtitle = `From ${this.shortenAddress(tx.from)}`;
        break;
      case 'staked':
        title = 'Staked';
        subtitle = 'Locked for rewards';
        break;
      case 'unstaked':
        title = 'Unstaked';
        subtitle = 'Released from staking';
        break;
      case 'reward':
        title = 'Staking Reward';
        subtitle = 'Earned from staking';
        break;
      case 'bridge':
        title = 'Bridge Transfer';
        subtitle = 'Cross-chain transfer';
        break;
    }

    return {
      title,
      subtitle,
      amount: `${isPositive ? '+' : '-'}${tx.amount} ${tx.currency}`,
      isPositive,
    };
  }

  /**
   * Shorten address for display
   */
  private shortenAddress(address: string): string {
    if (address.length <= 13) return address;
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  }

  /**
   * Mock transaction data for development
   */
  private getMockTransactions(address: string, limit: number, offset: number): Transaction[] {
    const now = Date.now();
    const mockTransactions: Transaction[] = [
      {
        id: '1',
        hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef',
        type: 'received',
        amount: '50.00',
        currency: 'ETR',
        from: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        to: address,
        timestamp: now - 3600000, // 1 hour ago
        status: 'confirmed',
        blockNumber: 1234567,
        fee: '0.001',
      },
      {
        id: '2',
        hash: '0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890',
        type: 'sent',
        amount: '25.50',
        currency: 'ETR',
        from: address,
        to: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        timestamp: now - 7200000, // 2 hours ago
        status: 'confirmed',
        blockNumber: 1234500,
        fee: '0.001',
      },
      {
        id: '3',
        hash: '0x7890abcdef1234567890abcdef1234567890abcdef1234567890abcdef123456',
        type: 'staked',
        amount: '100.00',
        currency: 'ETR',
        from: address,
        to: address,
        timestamp: now - 86400000, // 1 day ago
        status: 'confirmed',
        blockNumber: 1230000,
        fee: '0.001',
      },
      {
        id: '4',
        hash: '0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234',
        type: 'reward',
        amount: '2.35',
        currency: 'ETR',
        from: 'Staking Pool',
        to: address,
        timestamp: now - 172800000, // 2 days ago
        status: 'confirmed',
        blockNumber: 1225000,
      },
      {
        id: '5',
        hash: '0x34567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12',
        type: 'received',
        amount: '15.75',
        currency: 'ETR',
        from: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
        to: address,
        timestamp: now - 259200000, // 3 days ago
        status: 'confirmed',
        blockNumber: 1220000,
        fee: '0.001',
      },
      {
        id: '6',
        hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcd12',
        type: 'bridge',
        amount: '0.05',
        currency: 'BTC',
        from: 'Bitcoin Network',
        to: address,
        timestamp: now - 345600000, // 4 days ago
        status: 'confirmed',
        blockNumber: 1215000,
        fee: '0.0001',
        memo: 'Bridge from BTC',
      },
      {
        id: '7',
        hash: '0x90abcdef1234567890abcdef1234567890abcdef1234567890abcdef12345678',
        type: 'sent',
        amount: '5.00',
        currency: 'ETR',
        from: address,
        to: '5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw',
        timestamp: now - 432000000, // 5 days ago
        status: 'confirmed',
        blockNumber: 1210000,
        fee: '0.001',
      },
      {
        id: '8',
        hash: '0xdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab',
        type: 'received',
        amount: '30.00',
        currency: 'ETR',
        from: '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
        to: address,
        timestamp: now - 518400000, // 6 days ago
        status: 'confirmed',
        blockNumber: 1205000,
        fee: '0.001',
      },
    ];

    return mockTransactions.slice(offset, offset + limit);
  }

  /**
   * Calculate total sent amount
   */
  public async getTotalSent(address: string, currency: string = 'ETR'): Promise<number> {
    try {
      const transactions = await this.getTransactionsByType(address, 'sent');
      return transactions
        .filter(tx => tx.currency === currency)
        .reduce((sum, tx) => sum + parseFloat(tx.amount), 0);
    } catch (error) {
      console.error('Error calculating total sent:', error);
      return 0;
    }
  }

  /**
   * Calculate total received amount
   */
  public async getTotalReceived(address: string, currency: string = 'ETR'): Promise<number> {
    try {
      const transactions = await this.getTransactionsByType(address, 'received');
      return transactions
        .filter(tx => tx.currency === currency)
        .reduce((sum, tx) => sum + parseFloat(tx.amount), 0);
    } catch (error) {
      console.error('Error calculating total received:', error);
      return 0;
    }
  }
}

export default new TransactionService();
