/**
 * Timelock Service
 * Manages time-delayed transactions
 */

import { TimelockedTransaction } from '../types/security';

export class TimelockService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Set timelock delay
   */
  async setDelay(hours: number): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/delay`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ hours }),
      });

      if (!response.ok) {
        throw new Error('Failed to set timelock delay');
      }
    } catch (error) {
      console.error('Error setting timelock delay:', error);
      throw error;
    }
  }

  /**
   * Get all pending timelocked transactions
   */
  async getPendingTransactions(): Promise<TimelockedTransaction[]> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/pending`);

      if (!response.ok) {
        throw new Error('Failed to fetch pending transactions');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching pending transactions:', error);
      throw error;
    }
  }

  /**
   * Cancel a timelocked transaction
   */
  async cancelTimelock(txId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/${txId}/cancel`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to cancel timelock');
      }
    } catch (error) {
      console.error('Error cancelling timelock:', error);
      throw error;
    }
  }

  /**
   * Execute an unlocked transaction
   */
  async executeTransaction(txId: string): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/${txId}/execute`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to execute transaction');
      }

      return await response.json();
    } catch (error) {
      console.error('Error executing transaction:', error);
      throw error;
    }
  }

  /**
   * Add an address to timelock exceptions
   */
  async addException(address: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/exceptions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ address }),
      });

      if (!response.ok) {
        throw new Error('Failed to add timelock exception');
      }
    } catch (error) {
      console.error('Error adding timelock exception:', error);
      throw error;
    }
  }

  /**
   * Remove an address from timelock exceptions
   */
  async removeException(address: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/timelock/exceptions/${address}`, {
        method: 'DELETE',
      });

      if (!response.ok) {
        throw new Error('Failed to remove timelock exception');
      }
    } catch (error) {
      console.error('Error removing timelock exception:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const timelockService = new TimelockService();
