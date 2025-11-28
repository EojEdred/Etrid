/**
 * Metadata Service
 * Handles transaction metadata scrubbing
 */

import { TransactionMetadata } from '../types/privacy';

export class MetadataService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Scrub metadata from a transaction
   */
  async scrubTransaction(tx: any): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/metadata/scrub`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(tx),
      });

      if (!response.ok) {
        throw new Error('Failed to scrub transaction metadata');
      }

      return await response.json();
    } catch (error) {
      console.error('Error scrubbing transaction metadata:', error);
      throw error;
    }
  }

  /**
   * Get privacy score for an address
   */
  async getPrivacyScore(address: string): Promise<number> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/metadata/score/${address}`);

      if (!response.ok) {
        throw new Error('Failed to get privacy score');
      }

      const data = await response.json();
      return data.score;
    } catch (error) {
      console.error('Error getting privacy score:', error);
      throw error;
    }
  }

  /**
   * Enable metadata scrubbing
   */
  async enableScrubbing(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/metadata/enable`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to enable metadata scrubbing');
      }
    } catch (error) {
      console.error('Error enabling metadata scrubbing:', error);
      throw error;
    }
  }

  /**
   * Disable metadata scrubbing
   */
  async disableScrubbing(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/metadata/disable`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to disable metadata scrubbing');
      }
    } catch (error) {
      console.error('Error disabling metadata scrubbing:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const metadataService = new MetadataService();
