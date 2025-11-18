/**
 * ATM Service - Handles ATM location lookup and cash withdrawal operations
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import {
  ATMLocation,
  ATMFilter,
  WithdrawalRequest,
  WithdrawalResponse,
  Withdrawal,
  Coordinates,
} from '../types/atm.types';

const API_BASE_URL = process.env.EXPO_PUBLIC_API_URL || 'https://api.etrid.io';

class ATMService {
  private baseUrl = `${API_BASE_URL}/api/v1`;

  /**
   * Get ATM locations within a radius
   */
  async getATMLocations(
    coordinates: Coordinates,
    radius: number = 10, // miles
    filter?: ATMFilter
  ): Promise<ATMLocation[]> {
    try {
      const params = new URLSearchParams({
        lat: coordinates.latitude.toString(),
        lng: coordinates.longitude.toString(),
        radius: radius.toString(),
      });

      if (filter?.partner) {
        params.append('partner', filter.partner);
      }
      if (filter?.minRating) {
        params.append('minRating', filter.minRating.toString());
      }
      if (filter?.is24Hours) {
        params.append('is24Hours', 'true');
      }

      const response = await fetch(`${this.baseUrl}/atm/locations?${params}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch ATM locations: ${response.statusText}`);
      }

      const data = await response.json();
      return data.locations || [];
    } catch (error) {
      console.error('Error fetching ATM locations:', error);
      throw error;
    }
  }

  /**
   * Get details for a specific ATM
   */
  async getATMDetails(atmId: string): Promise<ATMLocation> {
    try {
      const response = await fetch(`${this.baseUrl}/atm/locations/${atmId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch ATM details: ${response.statusText}`);
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Error fetching ATM details:', error);
      throw error;
    }
  }

  /**
   * Create a cash withdrawal request
   */
  async createWithdrawal(
    request: WithdrawalRequest,
    authToken: string
  ): Promise<WithdrawalResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/atm/withdraw`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authToken}`,
        },
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to create withdrawal');
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Error creating withdrawal:', error);
      throw error;
    }
  }

  /**
   * Get withdrawal status by code
   */
  async getWithdrawalStatus(
    code: string,
    authToken: string
  ): Promise<Withdrawal> {
    try {
      const response = await fetch(`${this.baseUrl}/atm/withdraw/${code}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authToken}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch withdrawal status: ${response.statusText}`);
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('Error fetching withdrawal status:', error);
      throw error;
    }
  }

  /**
   * Cancel a pending withdrawal
   */
  async cancelWithdrawal(code: string, authToken: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/atm/withdraw/${code}/cancel`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${authToken}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to cancel withdrawal: ${response.statusText}`);
      }
    } catch (error) {
      console.error('Error canceling withdrawal:', error);
      throw error;
    }
  }

  /**
   * Get user's withdrawal history
   */
  async getWithdrawalHistory(
    userAddress: string,
    authToken: string,
    limit: number = 50
  ): Promise<Withdrawal[]> {
    try {
      const response = await fetch(
        `${this.baseUrl}/atm/withdrawals?user=${userAddress}&limit=${limit}`,
        {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${authToken}`,
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Failed to fetch withdrawal history: ${response.statusText}`);
      }

      const data = await response.json();
      return data.withdrawals || [];
    } catch (error) {
      console.error('Error fetching withdrawal history:', error);
      throw error;
    }
  }

  /**
   * Search ATMs by address or zip code
   */
  async searchATMs(query: string): Promise<ATMLocation[]> {
    try {
      const response = await fetch(
        `${this.baseUrl}/atm/search?q=${encodeURIComponent(query)}`,
        {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
          },
        }
      );

      if (!response.ok) {
        throw new Error(`Failed to search ATMs: ${response.statusText}`);
      }

      const data = await response.json();
      return data.locations || [];
    } catch (error) {
      console.error('Error searching ATMs:', error);
      throw error;
    }
  }

  /**
   * Calculate withdrawal fee for an amount
   */
  calculateFee(amount: number, feePercentage: number): number {
    return (amount * feePercentage) / 100;
  }

  /**
   * Calculate total withdrawal cost
   */
  calculateTotal(amount: number, feePercentage: number): number {
    return amount + this.calculateFee(amount, feePercentage);
  }

  /**
   * Check if withdrawal amount is within ATM limits
   */
  isWithinLimits(amount: number, atm: ATMLocation): boolean {
    return amount > 0 && amount <= atm.dailyLimit;
  }

  /**
   * Format withdrawal code for display (XXXX-XXXX)
   */
  formatCode(code: string): string {
    if (code.length === 8) {
      return `${code.slice(0, 4)}-${code.slice(4)}`;
    }
    return code;
  }

  /**
   * Calculate distance between two coordinates (Haversine formula)
   */
  calculateDistance(
    lat1: number,
    lng1: number,
    lat2: number,
    lng2: number
  ): number {
    const R = 3959; // Earth's radius in miles
    const dLat = this.toRad(lat2 - lat1);
    const dLng = this.toRad(lng2 - lng1);

    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.toRad(lat1)) *
        Math.cos(this.toRad(lat2)) *
        Math.sin(dLng / 2) *
        Math.sin(dLng / 2);

    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  private toRad(degrees: number): number {
    return degrees * (Math.PI / 180);
  }

  /**
   * Sort ATMs by distance from user location
   */
  sortByDistance(
    atms: ATMLocation[],
    userLocation: Coordinates
  ): ATMLocation[] {
    return atms
      .map((atm) => ({
        ...atm,
        distance: this.calculateDistance(
          userLocation.latitude,
          userLocation.longitude,
          atm.lat,
          atm.lng
        ),
      }))
      .sort((a, b) => (a.distance || 0) - (b.distance || 0));
  }

  /**
   * Filter ATMs by criteria
   */
  filterATMs(atms: ATMLocation[], filter: ATMFilter): ATMLocation[] {
    return atms.filter((atm) => {
      if (filter.partner && atm.partner !== filter.partner) {
        return false;
      }
      if (filter.minRating && atm.rating < filter.minRating) {
        return false;
      }
      if (filter.is24Hours && !atm.is24Hours) {
        return false;
      }
      if (filter.maxDistance && atm.distance && atm.distance > filter.maxDistance) {
        return false;
      }
      return true;
    });
  }
}

export default new ATMService();
