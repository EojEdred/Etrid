import axios from 'axios';
import config from '../config';
import logger from '../utils/logger';
import { ATMLocation } from '../types';

class ATMService {
  /**
   * Find nearby ATMs
   */
  async findNearbyATMs(params: {
    lat: number;
    lng: number;
    radius: number;
    partner: string;
  }): Promise<ATMLocation[]> {
    const locations: ATMLocation[] = [];

    try {
      // Fetch from Coinme
      if (params.partner === 'all' || params.partner === 'Coinme') {
        const coinmeLocations = await this.fetchCoinmeLocations(params);
        locations.push(...coinmeLocations);
      }

      // Fetch from Bitcoin Depot
      if (params.partner === 'all' || params.partner === 'Bitcoin Depot') {
        const depotLocations = await this.fetchBitcoinDepotLocations(params);
        locations.push(...depotLocations);
      }

      // Fetch from CoinFlip
      if (params.partner === 'all' || params.partner === 'CoinFlip') {
        const coinflipLocations = await this.fetchCoinFlipLocations(params);
        locations.push(...coinflipLocations);
      }

      // Calculate distances and sort
      locations.forEach((location) => {
        location.distance = this.calculateDistance(
          params.lat,
          params.lng,
          location.lat,
          location.lng
        );
      });

      return locations
        .filter((l) => l.distance! <= params.radius)
        .sort((a, b) => a.distance! - b.distance!);
    } catch (error: any) {
      logger.error('Error finding ATMs', { error: error.message });
      throw error;
    }
  }

  /**
   * Fetch Coinme locations
   */
  private async fetchCoinmeLocations(params: any): Promise<ATMLocation[]> {
    try {
      const response = await axios.get(`${config.atm.coinme.baseUrl}/locations`, {
        params: {
          lat: params.lat,
          lng: params.lng,
          radius: params.radius,
        },
        headers: {
          'X-API-Key': config.atm.coinme.apiKey,
        },
      });

      return response.data.locations.map((loc: any) => ({
        id: loc.id,
        partner: 'Coinme',
        name: loc.name,
        address: loc.address,
        lat: loc.latitude,
        lng: loc.longitude,
        supported_assets: ['BTC', 'ETH', 'ETR'],
      }));
    } catch (error: any) {
      logger.warn('Coinme API error', { error: error.message });
      return [];
    }
  }

  /**
   * Fetch Bitcoin Depot locations
   */
  private async fetchBitcoinDepotLocations(params: any): Promise<ATMLocation[]> {
    try {
      const response = await axios.get(`${config.atm.bitcoinDepot.baseUrl}/atms`, {
        params: {
          latitude: params.lat,
          longitude: params.lng,
          radius_miles: params.radius / 1609.34, // Convert meters to miles
        },
        headers: {
          Authorization: `Bearer ${config.atm.bitcoinDepot.apiKey}`,
        },
      });

      return response.data.atms.map((loc: any) => ({
        id: loc.id,
        partner: 'Bitcoin Depot',
        name: loc.location_name,
        address: loc.full_address,
        lat: loc.latitude,
        lng: loc.longitude,
        supported_assets: ['BTC', 'ETH'],
      }));
    } catch (error: any) {
      logger.warn('Bitcoin Depot API error', { error: error.message });
      return [];
    }
  }

  /**
   * Fetch CoinFlip locations
   */
  private async fetchCoinFlipLocations(params: any): Promise<ATMLocation[]> {
    try {
      const response = await axios.get(`${config.atm.coinflip.baseUrl}/kiosks`, {
        params: {
          lat: params.lat,
          long: params.lng,
          radius_km: params.radius / 1000,
        },
        headers: {
          'API-Key': config.atm.coinflip.apiKey,
        },
      });

      return response.data.kiosks.map((loc: any) => ({
        id: loc.kiosk_id,
        partner: 'CoinFlip',
        name: loc.name,
        address: loc.address,
        lat: loc.lat,
        lng: loc.long,
        supported_assets: ['BTC', 'ETH', 'LTC'],
      }));
    } catch (error: any) {
      logger.warn('CoinFlip API error', { error: error.message });
      return [];
    }
  }

  /**
   * Calculate distance between two coordinates (Haversine formula)
   */
  private calculateDistance(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number
  ): number {
    const R = 6371e3; // Earth radius in meters
    const φ1 = (lat1 * Math.PI) / 180;
    const φ2 = (lat2 * Math.PI) / 180;
    const Δφ = ((lat2 - lat1) * Math.PI) / 180;
    const Δλ = ((lon2 - lon1) * Math.PI) / 180;

    const a =
      Math.sin(Δφ / 2) * Math.sin(Δφ / 2) +
      Math.cos(φ1) * Math.cos(φ2) * Math.sin(Δλ / 2) * Math.sin(Δλ / 2);

    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return R * c; // Distance in meters
  }

  /**
   * Generate unique withdrawal code
   */
  generateWithdrawalCode(): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    let code = '';
    for (let i = 0; i < 12; i++) {
      code += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return code;
  }

  /**
   * Get exchange rate
   */
  async getExchangeRate(asset: string, fiat: string): Promise<number> {
    // This would fetch from a price oracle or exchange API
    // For now, returning mock rates
    const rates: Record<string, number> = {
      BTC: 45000,
      ETH: 3000,
      ETR: 2.5,
      USDT: 1.0,
    };

    return rates[asset] || 1.0;
  }

  /**
   * Submit withdrawal to partner API
   */
  async submitWithdrawal(partner: string, withdrawal: any): Promise<void> {
    try {
      switch (partner) {
        case 'Coinme':
          await this.submitToCoinme(withdrawal);
          break;
        case 'Bitcoin Depot':
          await this.submitToBitcoinDepot(withdrawal);
          break;
        case 'CoinFlip':
          await this.submitToCoinFlip(withdrawal);
          break;
      }

      logger.info('Withdrawal submitted to partner', {
        partner,
        code: withdrawal.withdrawal_code,
      });
    } catch (error: any) {
      logger.error('Error submitting withdrawal', {
        partner,
        error: error.message,
      });
      throw error;
    }
  }

  private async submitToCoinme(withdrawal: any): Promise<void> {
    await axios.post(
      `${config.atm.coinme.baseUrl}/withdrawals`,
      {
        code: withdrawal.withdrawal_code,
        amount: withdrawal.amount_usd,
        asset: withdrawal.asset,
      },
      {
        headers: {
          'X-API-Key': config.atm.coinme.apiKey,
        },
      }
    );
  }

  private async submitToBitcoinDepot(withdrawal: any): Promise<void> {
    await axios.post(
      `${config.atm.bitcoinDepot.baseUrl}/transactions`,
      {
        redemption_code: withdrawal.withdrawal_code,
        amount_usd: withdrawal.amount_usd,
        cryptocurrency: withdrawal.asset,
      },
      {
        headers: {
          Authorization: `Bearer ${config.atm.bitcoinDepot.apiKey}`,
        },
      }
    );
  }

  private async submitToCoinFlip(withdrawal: any): Promise<void> {
    await axios.post(
      `${config.atm.coinflip.baseUrl}/redemptions`,
      {
        code: withdrawal.withdrawal_code,
        amount: withdrawal.amount_usd,
        coin: withdrawal.asset,
      },
      {
        headers: {
          'API-Key': config.atm.coinflip.apiKey,
        },
      }
    );
  }
}

export default new ATMService();
