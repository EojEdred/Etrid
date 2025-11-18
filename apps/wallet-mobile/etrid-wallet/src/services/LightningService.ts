import EtridSDKService from './EtridSDKService';
import KeychainService from './KeychainService';
import {
  LightningChannel,
  OpenChannelRequest,
  LightningPayment,
  LightningStats,
  TransactionResult,
} from '../types/defi.types';

/**
 * LightningService - Handles Lightning-Bloc instant payment operations
 * Lightning-Bloc is Ëtrid's Layer 2 payment channel solution for instant, low-fee transfers
 */
class LightningService {
  private sdk: EtridSDKService;
  private readonly DECIMALS = 12; // ETR decimals

  constructor() {
    this.sdk = EtridSDKService.getInstance();
  }

  /**
   * Get all payment channels for an address
   */
  async getChannels(address: string): Promise<LightningChannel[]> {
    try {
      await this.sdk.connect();

      const channelsData = await this.sdk.lightning?.getChannels?.(address) || [];

      return channelsData.map((channel: any) => this.formatChannel(channel));
    } catch (error) {
      console.error('Failed to get channels:', error);
      return [];
    }
  }

  /**
   * Get active payment channels
   */
  async getActiveChannels(address: string): Promise<LightningChannel[]> {
    const channels = await this.getChannels(address);
    return channels.filter(c => c.status === 'active' && c.isActive);
  }

  /**
   * Open a new payment channel
   */
  async openChannel(request: OpenChannelRequest): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      // Open channel via Lightning-Bloc wrapper
      const channelTx = await this.sdk.lightning?.openChannel?.(
        keypair,
        request.counterparty,
        BigInt(request.capacity),
        request.feeRate
      );

      return {
        success: true,
        txHash: channelTx?.toString(),
        message: 'Payment channel opening. This may take a few moments.',
      };
    } catch (error) {
      console.error('Failed to open channel:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to open payment channel',
      };
    }
  }

  /**
   * Close a payment channel
   */
  async closeChannel(channelId: string, force: boolean = false): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const closeTx = await this.sdk.lightning?.closeChannel?.(keypair, channelId, force);

      return {
        success: true,
        txHash: closeTx?.toString(),
        message: force
          ? 'Force closing channel. Funds will be available after settlement period.'
          : 'Closing channel cooperatively. Funds will be available shortly.',
      };
    } catch (error) {
      console.error('Failed to close channel:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to close payment channel',
      };
    }
  }

  /**
   * Send instant payment through Lightning-Bloc
   */
  async sendPayment(
    channelId: string,
    recipient: string,
    amount: string
  ): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      // Send instant payment through channel
      const paymentTx = await this.sdk.lightning?.sendPayment?.(
        keypair,
        channelId,
        recipient,
        BigInt(amount)
      );

      return {
        success: true,
        txHash: paymentTx?.toString(),
        message: 'Instant payment sent successfully',
      };
    } catch (error) {
      console.error('Failed to send payment:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to send instant payment',
      };
    }
  }

  /**
   * Get payment history for channels
   */
  async getPaymentHistory(address: string): Promise<LightningPayment[]> {
    try {
      await this.sdk.connect();

      const paymentsData = await this.sdk.lightning?.getPaymentHistory?.(address) || [];

      return paymentsData.map((payment: any) => ({
        id: payment.id,
        channelId: payment.channelId,
        amount: payment.amount,
        amountETR: this.fromSmallestUnit(payment.amount),
        recipient: payment.recipient,
        recipientName: payment.recipientName,
        timestamp: payment.timestamp,
        status: payment.status,
        txHash: payment.txHash,
        fee: payment.fee || '0',
        feeETR: this.fromSmallestUnit(payment.fee || '0'),
        type: payment.type,
      }));
    } catch (error) {
      console.error('Failed to get payment history:', error);
      return [];
    }
  }

  /**
   * Get Lightning-Bloc statistics
   */
  async getStats(address: string): Promise<LightningStats> {
    try {
      const channels = await this.getChannels(address);
      const payments = await this.getPaymentHistory(address);

      const activeChannels = channels.filter(c => c.status === 'active');
      const totalCapacity = channels.reduce((sum, c) => sum + BigInt(c.capacity), BigInt(0));

      const sentPayments = payments.filter(p => p.type === 'sent');
      const receivedPayments = payments.filter(p => p.type === 'received');

      const totalSent = sentPayments.reduce((sum, p) => sum + BigInt(p.amount), BigInt(0));
      const totalReceived = receivedPayments.reduce((sum, p) => sum + BigInt(p.amount), BigInt(0));

      const totalFees = payments.reduce((sum, p) => sum + Number(p.feeETR), 0);
      const totalAmount = sentPayments.reduce((sum, p) => sum + p.amountETR, 0);
      const averageFee = totalAmount > 0 ? (totalFees / totalAmount) * 100 : 0;

      return {
        totalChannels: channels.length,
        activeChannels: activeChannels.length,
        totalCapacity: totalCapacity.toString(),
        totalCapacityETR: this.fromSmallestUnit(totalCapacity.toString()),
        totalSent: totalSent.toString(),
        totalSentETR: this.fromSmallestUnit(totalSent.toString()),
        totalReceived: totalReceived.toString(),
        totalReceivedETR: this.fromSmallestUnit(totalReceived.toString()),
        averageFee,
      };
    } catch (error) {
      console.error('Failed to get Lightning stats:', error);
      return {
        totalChannels: 0,
        activeChannels: 0,
        totalCapacity: '0',
        totalCapacityETR: 0,
        totalSent: '0',
        totalSentETR: 0,
        totalReceived: '0',
        totalReceivedETR: 0,
        averageFee: 0,
      };
    }
  }

  /**
   * Estimate fees for opening a channel
   */
  estimateOpenChannelFee(capacity: string): { openingFee: number; closingFee: number } {
    const capacityETR = this.fromSmallestUnit(capacity);

    // Opening fee: 0.1% of capacity
    const openingFee = capacityETR * 0.001;

    // Closing fee: Fixed 0.01 ETR
    const closingFee = 0.01;

    return { openingFee, closingFee };
  }

  /**
   * Estimate payment fee
   */
  estimatePaymentFee(amount: string): number {
    const amountETR = this.fromSmallestUnit(amount);

    // Lightning payments: 0.01% fee
    return amountETR * 0.0001;
  }

  /**
   * Find suitable channel for payment
   */
  async findChannelForPayment(address: string, amount: string): Promise<LightningChannel | null> {
    try {
      const channels = await this.getActiveChannels(address);
      const amountBigInt = BigInt(amount);

      // Find channel with sufficient balance
      const suitableChannel = channels.find(
        c => BigInt(c.localBalance) >= amountBigInt && c.status === 'active'
      );

      return suitableChannel || null;
    } catch (error) {
      console.error('Failed to find suitable channel:', error);
      return null;
    }
  }

  /**
   * Check if address has active channels
   */
  async hasActiveChannels(address: string): Promise<boolean> {
    const channels = await this.getActiveChannels(address);
    return channels.length > 0;
  }

  /**
   * Format channel data
   */
  private formatChannel(data: any): LightningChannel {
    const localBalanceETR = this.fromSmallestUnit(data.localBalance || '0');
    const remoteBalanceETR = this.fromSmallestUnit(data.remoteBalance || '0');
    const capacityETR = localBalanceETR + remoteBalanceETR;

    return {
      id: data.id,
      channelId: data.channelId,
      counterparty: data.counterparty,
      counterpartyName: data.counterpartyName,
      capacity: this.toSmallestUnit(capacityETR),
      capacityETR,
      localBalance: data.localBalance || '0',
      localBalanceETR,
      remoteBalance: data.remoteBalance || '0',
      remoteBalanceETR,
      status: data.status,
      openedAt: data.openedAt,
      closedAt: data.closedAt,
      txHash: data.txHash,
      isActive: data.status === 'active' && localBalanceETR > 0,
    };
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
    return BigInt(Math.floor(amount * Math.pow(10, this.DECIMALS))).toString();
  }
}

export default new LightningService();
