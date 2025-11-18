/**
 * ATM Types - Type definitions for ATM integration
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

export interface ATMLocation {
  id: string;
  partner: 'Coinme' | 'Bitcoin Depot' | 'CoinFlip';
  name: string;
  address: string;
  city: string;
  state: string;
  zip: string;
  lat: number;
  lng: number;
  hours: string;
  is24Hours: boolean;
  fee: number; // Percentage (7-12)
  dailyLimit: number; // USD
  rating: number; // 0-5 stars
  reviewCount: number;
  distance?: number; // Calculated distance in miles
  features: ATMFeature[];
}

export type ATMFeature =
  | 'buyBitcoin'
  | 'sellBitcoin'
  | 'cashWithdrawal'
  | 'billPayment'
  | 'phoneTopup';

export interface ATMFilter {
  partner?: 'Coinme' | 'Bitcoin Depot' | 'CoinFlip';
  maxDistance?: number; // miles
  minRating?: number;
  is24Hours?: boolean;
  features?: ATMFeature[];
}

export interface WithdrawalRequest {
  user: string; // User's address
  amount: number; // USD amount
  asset: 'ETR' | 'BTC' | 'ETH' | 'USDT';
  atmPartner: string;
  atmLocationId: string;
}

export interface WithdrawalResponse {
  withdrawalCode: string; // "8472-3951"
  expiresAt: string; // ISO timestamp
  fee: number; // Fee in USD
  feePercentage: number;
  total: number; // Total cost in USD
  assetAmount: number; // Amount of crypto deducted
  txHash: string; // Blockchain transaction hash
  atmLocation: ATMLocation;
  status: WithdrawalStatus;
}

export type WithdrawalStatus =
  | 'pending'
  | 'confirmed'
  | 'processing'
  | 'completed'
  | 'expired'
  | 'cancelled'
  | 'failed';

export interface Withdrawal {
  id: string;
  code: string;
  amount: number; // USD
  asset: 'ETR' | 'BTC' | 'ETH' | 'USDT';
  assetAmount: number;
  fee: number;
  total: number;
  atmLocation: ATMLocation;
  status: WithdrawalStatus;
  createdAt: string;
  expiresAt: string;
  completedAt?: string;
  txHash: string;
  qrCode?: string;
}

export interface ATMStats {
  totalLocations: number;
  nearestDistance: number;
  averageFee: number;
  totalWithdrawals: number;
  totalVolume: number; // USD
}

export interface Coordinates {
  latitude: number;
  longitude: number;
}

export interface MapRegion {
  latitude: number;
  longitude: number;
  latitudeDelta: number;
  longitudeDelta: number;
}
