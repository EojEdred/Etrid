/**
 * Security Service
 * Handles enhanced security features
 */

import {
  SecuritySettings,
  SecurityScore,
  SecurityEvent,
  PanicModeStatus,
  GuardianApproval,
} from '../types/security';

export class SecurityService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Calculate security score based on enabled features
   */
  async getSecurityScore(): Promise<SecurityScore> {
    try {
      const response = await fetch(`${this.baseUrl}/security/score`);

      if (!response.ok) {
        throw new Error('Failed to fetch security score');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching security score:', error);
      throw error;
    }
  }

  /**
   * Get current security settings
   */
  async getSecuritySettings(): Promise<SecuritySettings> {
    try {
      const response = await fetch(`${this.baseUrl}/security/settings`);

      if (!response.ok) {
        throw new Error('Failed to fetch security settings');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching security settings:', error);
      throw error;
    }
  }

  /**
   * Update security settings
   */
  async updateSettings(settings: Partial<SecuritySettings>): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/settings`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(settings),
      });

      if (!response.ok) {
        throw new Error('Failed to update security settings');
      }
    } catch (error) {
      console.error('Error updating security settings:', error);
      throw error;
    }
  }

  /**
   * Activate panic mode
   */
  async activatePanicMode(reason?: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/panic/activate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ reason }),
      });

      if (!response.ok) {
        throw new Error('Failed to activate panic mode');
      }
    } catch (error) {
      console.error('Error activating panic mode:', error);
      throw error;
    }
  }

  /**
   * Deactivate panic mode (requires guardian approvals)
   */
  async deactivatePanicMode(guardianApprovals: GuardianApproval[]): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/panic/deactivate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ guardianApprovals }),
      });

      if (!response.ok) {
        throw new Error('Failed to deactivate panic mode');
      }
    } catch (error) {
      console.error('Error deactivating panic mode:', error);
      throw error;
    }
  }

  /**
   * Get panic mode status
   */
  async getPanicModeStatus(): Promise<PanicModeStatus> {
    try {
      const response = await fetch(`${this.baseUrl}/security/panic/status`);

      if (!response.ok) {
        throw new Error('Failed to fetch panic mode status');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching panic mode status:', error);
      throw error;
    }
  }

  /**
   * Set duress PIN (shows fake balance when used)
   */
  async setDuressPin(pin: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/duress-pin`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ pin }),
      });

      if (!response.ok) {
        throw new Error('Failed to set duress PIN');
      }
    } catch (error) {
      console.error('Error setting duress PIN:', error);
      throw error;
    }
  }

  /**
   * Add a guardian
   */
  async addGuardian(guardianAddress: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/guardians`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ guardian: guardianAddress }),
      });

      if (!response.ok) {
        throw new Error('Failed to add guardian');
      }
    } catch (error) {
      console.error('Error adding guardian:', error);
      throw error;
    }
  }

  /**
   * Remove a guardian
   */
  async removeGuardian(guardianAddress: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/guardians/${guardianAddress}`, {
        method: 'DELETE',
      });

      if (!response.ok) {
        throw new Error('Failed to remove guardian');
      }
    } catch (error) {
      console.error('Error removing guardian:', error);
      throw error;
    }
  }

  /**
   * Get security events
   */
  async getSecurityEvents(limit: number = 50): Promise<SecurityEvent[]> {
    try {
      const response = await fetch(`${this.baseUrl}/security/events?limit=${limit}`);

      if (!response.ok) {
        throw new Error('Failed to fetch security events');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching security events:', error);
      throw error;
    }
  }

  /**
   * Log a security event
   */
  async logSecurityEvent(
    type: SecurityEvent['type'],
    description: string,
    severity: SecurityEvent['severity'],
    metadata?: Record<string, any>
  ): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/events`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ type, description, severity, metadata }),
      });

      if (!response.ok) {
        throw new Error('Failed to log security event');
      }
    } catch (error) {
      console.error('Error logging security event:', error);
      throw error;
    }
  }

  /**
   * Enable/disable biometrics
   */
  async setBiometrics(enabled: boolean): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/biometrics`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled }),
      });

      if (!response.ok) {
        throw new Error('Failed to update biometrics setting');
      }
    } catch (error) {
      console.error('Error updating biometrics:', error);
      throw error;
    }
  }

  /**
   * Enable/disable 2FA
   */
  async setTwoFactor(enabled: boolean): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/security/2fa`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled }),
      });

      if (!response.ok) {
        throw new Error('Failed to update 2FA setting');
      }
    } catch (error) {
      console.error('Error updating 2FA:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const securityService = new SecurityService();
