/**
 * Privacy Service
 * Handles privacy features like stealth addresses and coin mixing
 */

import {
  PrivacySettings,
  StealthAddress,
  MixingSession,
  MixingStatus,
  PrivacyScore,
} from '../types/privacy';

export class PrivacyService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Get privacy settings
   */
  async getPrivacySettings(): Promise<PrivacySettings> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/settings`);

      if (!response.ok) {
        throw new Error('Failed to fetch privacy settings');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching privacy settings:', error);
      throw error;
    }
  }

  /**
   * Update privacy settings
   */
  async updateSettings(settings: Partial<PrivacySettings>): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/settings`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(settings),
      });

      if (!response.ok) {
        throw new Error('Failed to update privacy settings');
      }
    } catch (error) {
      console.error('Error updating privacy settings:', error);
      throw error;
    }
  }

  /**
   * Generate a new stealth address
   */
  async generateStealthAddress(label?: string): Promise<StealthAddress> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/stealth/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ label }),
      });

      if (!response.ok) {
        throw new Error('Failed to generate stealth address');
      }

      return await response.json();
    } catch (error) {
      console.error('Error generating stealth address:', error);
      throw error;
    }
  }

  /**
   * Get all stealth addresses
   */
  async getStealthAddresses(): Promise<StealthAddress[]> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/stealth/list`);

      if (!response.ok) {
        throw new Error('Failed to fetch stealth addresses');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching stealth addresses:', error);
      throw error;
    }
  }

  /**
   * Start a coin mixing session
   */
  async mixCoins(amount: string, rounds: number): Promise<MixingSession> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/mix/start`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ amount, rounds }),
      });

      if (!response.ok) {
        throw new Error('Failed to start mixing session');
      }

      return await response.json();
    } catch (error) {
      console.error('Error starting mixing session:', error);
      throw error;
    }
  }

  /**
   * Get mixing session status
   */
  async getMixingStatus(sessionId: string): Promise<MixingStatus> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/mix/${sessionId}/status`);

      if (!response.ok) {
        throw new Error('Failed to fetch mixing status');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching mixing status:', error);
      throw error;
    }
  }

  /**
   * Get all mixing sessions
   */
  async getMixingSessions(): Promise<MixingSession[]> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/mix/sessions`);

      if (!response.ok) {
        throw new Error('Failed to fetch mixing sessions');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching mixing sessions:', error);
      throw error;
    }
  }

  /**
   * Enable Tor routing
   */
  async enableTor(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/tor/enable`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to enable Tor');
      }
    } catch (error) {
      console.error('Error enabling Tor:', error);
      throw error;
    }
  }

  /**
   * Disable Tor routing
   */
  async disableTor(): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/tor/disable`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to disable Tor');
      }
    } catch (error) {
      console.error('Error disabling Tor:', error);
      throw error;
    }
  }

  /**
   * Get privacy score
   */
  async getPrivacyScore(): Promise<PrivacyScore> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/score`);

      if (!response.ok) {
        throw new Error('Failed to fetch privacy score');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching privacy score:', error);
      throw error;
    }
  }

  /**
   * Set privacy level
   */
  async setPrivacyLevel(level: 'low' | 'medium' | 'high'): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/level`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ level }),
      });

      if (!response.ok) {
        throw new Error('Failed to set privacy level');
      }
    } catch (error) {
      console.error('Error setting privacy level:', error);
      throw error;
    }
  }

  /**
   * Cancel a mixing session
   */
  async cancelMixing(sessionId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/privacy/mix/${sessionId}/cancel`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to cancel mixing session');
      }
    } catch (error) {
      console.error('Error cancelling mixing session:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const privacyService = new PrivacyService();
