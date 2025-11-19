/**
 * Whitelist Service
 * Manages whitelisted addresses
 */

import { WhitelistedAddress } from '../types/security';

export class WhitelistService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Add an address to the whitelist
   */
  async addAddress(address: string, label?: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/whitelist`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ address, label }),
      });

      if (!response.ok) {
        throw new Error('Failed to add address to whitelist');
      }
    } catch (error) {
      console.error('Error adding address to whitelist:', error);
      throw error;
    }
  }

  /**
   * Remove an address from the whitelist
   */
  async removeAddress(address: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/whitelist/${address}`, {
        method: 'DELETE',
      });

      if (!response.ok) {
        throw new Error('Failed to remove address from whitelist');
      }
    } catch (error) {
      console.error('Error removing address from whitelist:', error);
      throw error;
    }
  }

  /**
   * Get all whitelisted addresses
   */
  async getWhitelist(): Promise<WhitelistedAddress[]> {
    try {
      const response = await fetch(`${this.baseUrl}/security/whitelist`);

      if (!response.ok) {
        throw new Error('Failed to fetch whitelist');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching whitelist:', error);
      throw error;
    }
  }

  /**
   * Check if an address is whitelisted
   */
  async isWhitelisted(address: string): Promise<boolean> {
    try {
      const response = await fetch(`${this.baseUrl}/security/whitelist/check/${address}`);

      if (!response.ok) {
        throw new Error('Failed to check whitelist status');
      }

      const data = await response.json();
      return data.isWhitelisted;
    } catch (error) {
      console.error('Error checking whitelist status:', error);
      throw error;
    }
  }

  /**
   * Update label for a whitelisted address
   */
  async updateLabel(address: string, label: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/whitelist/${address}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ label }),
      });

      if (!response.ok) {
        throw new Error('Failed to update whitelist label');
      }
    } catch (error) {
      console.error('Error updating whitelist label:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const whitelistService = new WhitelistService();
