/**
 * Username Service
 * Manages ENS-style username registration, resolution, and pricing
 */

import type {
  UsernameRegistration,
  UsernameAvailability,
  UsernamePricing,
  UsernameResolution,
  ApiResponse,
} from './types';
import { UsernameError } from './types';

// Username validation regex: alphanumeric + dash, 1-63 characters
const USERNAME_REGEX = /^[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?$/;
const RESERVED_USERNAMES = new Set(['admin', 'etrid', 'system', 'support', 'help', 'root', 'test']);

export class UsernameService {
  private apiUrl: string;

  constructor(apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001') {
    this.apiUrl = apiUrl;
  }

  /**
   * Validate username format
   */
  validateUsername(username: string): { valid: boolean; error?: string } {
    if (!username || username.length === 0) {
      return { valid: false, error: 'Username is required' };
    }

    if (username.length < 1 || username.length > 63) {
      return { valid: false, error: 'Username must be 1-63 characters' };
    }

    if (!USERNAME_REGEX.test(username)) {
      return {
        valid: false,
        error: 'Username can only contain lowercase letters, numbers, and hyphens (cannot start or end with hyphen)',
      };
    }

    if (RESERVED_USERNAMES.has(username.toLowerCase())) {
      return { valid: false, error: 'This username is reserved' };
    }

    return { valid: true };
  }

  /**
   * Check if username is available
   */
  async checkAvailability(username: string): Promise<UsernameAvailability> {
    // Validate format first
    const validation = this.validateUsername(username);
    if (!validation.valid) {
      throw new UsernameError(validation.error!, 'INVALID_FORMAT');
    }

    try {
      const response = await fetch(`${this.apiUrl}/username/check`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username }),
      });

      const result: ApiResponse<UsernameAvailability> = await response.json();

      if (!result.success || !result.data) {
        throw new UsernameError(
          result.error?.message || 'Failed to check availability',
          result.error?.code || 'CHECK_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof UsernameError) throw error;

      // Fallback to mock data for development
      console.warn('[UsernameService] API not available, using mock data');
      return this.mockCheckAvailability(username);
    }
  }

  /**
   * Calculate username price based on length
   */
  getPrice(username: string): UsernamePricing {
    const length = username.length;
    let price: number;
    let tier: 'premium' | 'standard' | 'basic';

    if (length >= 1 && length <= 3) {
      price = 1000;
      tier = 'premium';
    } else if (length >= 4 && length <= 5) {
      price = 100;
      tier = 'standard';
    } else {
      price = 10;
      tier = 'basic';
    }

    return { username, length, price, tier };
  }

  /**
   * Register a new username
   */
  async registerUsername(
    username: string,
    address: string,
    duration: number = 365 // days
  ): Promise<UsernameRegistration> {
    // Validate username
    const validation = this.validateUsername(username);
    if (!validation.valid) {
      throw new UsernameError(validation.error!, 'INVALID_FORMAT');
    }

    // Check availability
    const availability = await this.checkAvailability(username);
    if (!availability.available) {
      throw new UsernameError('Username is not available', 'NOT_AVAILABLE');
    }

    try {
      const expiresAt = new Date();
      expiresAt.setDate(expiresAt.getDate() + duration);

      const response = await fetch(`${this.apiUrl}/username/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, address, expiresAt }),
      });

      const result: ApiResponse<UsernameRegistration> = await response.json();

      if (!result.success || !result.data) {
        throw new UsernameError(
          result.error?.message || 'Failed to register username',
          result.error?.code || 'REGISTRATION_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof UsernameError) throw error;

      // Fallback to mock data for development
      console.warn('[UsernameService] API not available, using mock registration');
      return this.mockRegisterUsername(username, address, duration);
    }
  }

  /**
   * Resolve username to address
   */
  async resolveUsername(username: string): Promise<string> {
    try {
      const response = await fetch(`${this.apiUrl}/username/resolve/${username}`);
      const result: ApiResponse<UsernameResolution> = await response.json();

      if (!result.success || !result.data) {
        throw new UsernameError(
          result.error?.message || 'Username not found',
          result.error?.code || 'NOT_FOUND'
        );
      }

      return result.data.address;
    } catch (error) {
      if (error instanceof UsernameError) throw error;

      // Fallback to mock data for development
      console.warn('[UsernameService] API not available, using mock resolution');
      return this.mockResolveUsername(username);
    }
  }

  /**
   * Reverse resolve address to username
   */
  async reverseResolve(address: string): Promise<string | null> {
    try {
      const response = await fetch(`${this.apiUrl}/username/reverse/${address}`);
      const result: ApiResponse<UsernameResolution> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data.username;
    } catch (error) {
      // Fallback to mock data for development
      console.warn('[UsernameService] API not available, using mock reverse resolution');
      return this.mockReverseResolve(address);
    }
  }

  /**
   * Get username registration details
   */
  async getRegistration(username: string): Promise<UsernameRegistration | null> {
    try {
      const response = await fetch(`${this.apiUrl}/username/${username}`);
      const result: ApiResponse<UsernameRegistration> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data;
    } catch (error) {
      console.error('[UsernameService] Failed to get registration:', error);
      return null;
    }
  }

  /**
   * Renew username registration
   */
  async renewUsername(username: string, duration: number = 365): Promise<UsernameRegistration> {
    try {
      const response = await fetch(`${this.apiUrl}/username/${username}/renew`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ duration }),
      });

      const result: ApiResponse<UsernameRegistration> = await response.json();

      if (!result.success || !result.data) {
        throw new UsernameError(
          result.error?.message || 'Failed to renew username',
          result.error?.code || 'RENEWAL_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof UsernameError) throw error;
      throw new UsernameError('Failed to renew username', 'RENEWAL_FAILED');
    }
  }

  /**
   * Generate username suggestions based on input
   */
  generateSuggestions(username: string, count: number = 5): string[] {
    const suggestions: string[] = [];
    const base = username.toLowerCase().replace(/[^a-z0-9]/g, '');

    // Add number suffixes
    for (let i = 1; i <= count && suggestions.length < count; i++) {
      suggestions.push(`${base}${i}`);
    }

    // Add random suffixes
    const suffixes = ['eth', 'wallet', 'crypto', 'defi', 'web3'];
    for (const suffix of suffixes) {
      if (suggestions.length >= count) break;
      suggestions.push(`${base}-${suffix}`);
    }

    // Add year
    if (suggestions.length < count) {
      const year = new Date().getFullYear();
      suggestions.push(`${base}${year}`);
    }

    return suggestions.slice(0, count);
  }

  // ============================================================================
  // MOCK DATA (for development/testing)
  // ============================================================================

  private mockCheckAvailability(username: string): UsernameAvailability {
    // Mock: usernames starting with 'a' are taken
    const available = !username.startsWith('a');

    return {
      available,
      username,
      suggestions: available ? undefined : this.generateSuggestions(username),
    };
  }

  private mockRegisterUsername(
    username: string,
    address: string,
    duration: number
  ): UsernameRegistration {
    const expiresAt = new Date();
    expiresAt.setDate(expiresAt.getDate() + duration);

    return {
      id: `reg-${Date.now()}`,
      username,
      address,
      expiresAt,
      createdAt: new Date(),
      transactionHash: `0x${Math.random().toString(16).slice(2, 66)}`,
    };
  }

  private mockResolveUsername(username: string): string {
    // Mock: return a fake address
    return `5${username.slice(0, 10).padEnd(10, '0')}${'0'.repeat(38)}`;
  }

  private mockReverseResolve(address: string): string | null {
    // Mock: addresses starting with '5' have usernames
    if (address.startsWith('5')) {
      return `user${address.slice(1, 5)}`;
    }
    return null;
  }
}

// Singleton instance
export const usernameService = new UsernameService();
