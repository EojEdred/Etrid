import { TOKEN_DECIMALS } from './constants';
import BigNumber from 'bignumber.js';

/**
 * Format large numbers with proper notation (K, M, B)
 */
export const formatNumber = (value: number | string, decimals: number = 2): string => {
  const num = typeof value === 'string' ? parseFloat(value) : value;

  if (isNaN(num)) return '0';

  if (num >= 1e9) {
    return `${(num / 1e9).toFixed(decimals)}B`;
  } else if (num >= 1e6) {
    return `${(num / 1e6).toFixed(decimals)}M`;
  } else if (num >= 1e3) {
    return `${(num / 1e3).toFixed(decimals)}K`;
  }

  return num.toFixed(decimals);
};

/**
 * Format currency with proper decimals
 */
export const formatCurrency = (
  amount: number | string,
  currency: string = 'USD',
  decimals?: number
): string => {
  const num = typeof amount === 'string' ? parseFloat(amount) : amount;

  if (isNaN(num)) return '0.00';

  const finalDecimals = decimals ?? (num < 1 ? 4 : 2);

  if (currency === 'USD') {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: finalDecimals,
      maximumFractionDigits: finalDecimals,
    }).format(num);
  }

  return `${num.toFixed(finalDecimals)} ${currency}`;
};

/**
 * Format crypto amount with proper decimals
 */
export const formatCryptoAmount = (
  amount: string | number,
  symbol: string,
  options: {
    showSymbol?: boolean;
    decimals?: number;
    compact?: boolean;
  } = {}
): string => {
  const { showSymbol = true, decimals, compact = false } = options;

  const num = typeof amount === 'string' ? parseFloat(amount) : amount;

  if (isNaN(num)) return showSymbol ? `0 ${symbol}` : '0';

  // Determine decimals based on amount size
  let finalDecimals = decimals;
  if (finalDecimals === undefined) {
    if (num >= 1000) finalDecimals = 2;
    else if (num >= 1) finalDecimals = 4;
    else if (num >= 0.01) finalDecimals = 6;
    else finalDecimals = 8;
  }

  const formatted = compact ? formatNumber(num, finalDecimals) : num.toFixed(finalDecimals);

  return showSymbol ? `${formatted} ${symbol}` : formatted;
};

/**
 * Format blockchain amount (with decimals) to human-readable
 */
export const formatTokenAmount = (
  amount: string,
  tokenSymbol: keyof typeof TOKEN_DECIMALS,
  options: {
    showSymbol?: boolean;
    decimals?: number;
  } = {}
): string => {
  const { showSymbol = true, decimals = 4 } = options;

  try {
    const tokenDecimals = TOKEN_DECIMALS[tokenSymbol] || 12;
    const bn = new BigNumber(amount);
    const divisor = new BigNumber(10).pow(tokenDecimals);
    const humanReadable = bn.dividedBy(divisor).toNumber();

    return formatCryptoAmount(humanReadable, tokenSymbol, { showSymbol, decimals });
  } catch (error) {
    console.error('Error formatting token amount:', error);
    return showSymbol ? `0 ${tokenSymbol}` : '0';
  }
};

/**
 * Convert human-readable amount to blockchain format
 */
export const toTokenAmount = (
  amount: string | number,
  tokenSymbol: keyof typeof TOKEN_DECIMALS
): string => {
  try {
    const tokenDecimals = TOKEN_DECIMALS[tokenSymbol] || 12;
    const bn = new BigNumber(amount);
    const multiplier = new BigNumber(10).pow(tokenDecimals);
    return bn.multipliedBy(multiplier).toFixed(0);
  } catch (error) {
    console.error('Error converting to token amount:', error);
    return '0';
  }
};

/**
 * Format address for display (shorten)
 */
export const formatAddress = (
  address: string,
  startChars: number = 6,
  endChars: number = 4
): string => {
  if (!address) return '';
  if (address.length <= startChars + endChars) return address;
  return `${address.slice(0, startChars)}...${address.slice(-endChars)}`;
};

/**
 * Format date/time for display
 */
export const formatDate = (timestamp: number | Date, format: 'short' | 'long' | 'relative' = 'short'): string => {
  const date = typeof timestamp === 'number' ? new Date(timestamp) : timestamp;

  if (format === 'relative') {
    return formatRelativeTime(date);
  }

  const options: Intl.DateTimeFormatOptions =
    format === 'long'
      ? {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit',
        }
      : {
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit',
        };

  return new Intl.DateTimeFormat('en-US', options).format(date);
};

/**
 * Format relative time (e.g., "2 hours ago")
 */
export const formatRelativeTime = (date: Date): string => {
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);
  const diffWeek = Math.floor(diffDay / 7);
  const diffMonth = Math.floor(diffDay / 30);
  const diffYear = Math.floor(diffDay / 365);

  if (diffSec < 60) return 'Just now';
  if (diffMin < 60) return `${diffMin}m ago`;
  if (diffHour < 24) return `${diffHour}h ago`;
  if (diffDay < 7) return `${diffDay}d ago`;
  if (diffWeek < 4) return `${diffWeek}w ago`;
  if (diffMonth < 12) return `${diffMonth}mo ago`;
  return `${diffYear}y ago`;
};

/**
 * Format time duration
 */
export const formatDuration = (milliseconds: number): string => {
  const seconds = Math.floor(milliseconds / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) return `${days}d ${hours % 24}h`;
  if (hours > 0) return `${hours}h ${minutes % 60}m`;
  if (minutes > 0) return `${minutes}m ${seconds % 60}s`;
  return `${seconds}s`;
};

/**
 * Format percentage
 */
export const formatPercentage = (value: number, decimals: number = 2, showSign: boolean = true): string => {
  const sign = showSign && value > 0 ? '+' : '';
  return `${sign}${value.toFixed(decimals)}%`;
};

/**
 * Format transaction hash
 */
export const formatTransactionHash = (hash: string): string => {
  return formatAddress(hash, 10, 8);
};

/**
 * Format file size
 */
export const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
};

/**
 * Capitalize first letter
 */
export const capitalize = (str: string): string => {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
};

/**
 * Format phone number
 */
export const formatPhoneNumber = (phone: string): string => {
  const cleaned = phone.replace(/\D/g, '');
  const match = cleaned.match(/^(\d{3})(\d{3})(\d{4})$/);
  if (match) {
    return `(${match[1]}) ${match[2]}-${match[3]}`;
  }
  return phone;
};

/**
 * Format balance with appropriate precision
 */
export const formatBalance = (balance: string | number, currency: string = 'ETR'): string => {
  const num = typeof balance === 'string' ? parseFloat(balance) : balance;

  if (isNaN(num)) return `0.00 ${currency}`;

  // Show more decimals for small amounts
  const decimals = num < 0.01 ? 8 : num < 1 ? 6 : num < 1000 ? 4 : 2;

  return `${num.toFixed(decimals)} ${currency}`;
};

/**
 * Parse user input to number
 */
export const parseUserInput = (input: string): number => {
  // Remove all non-numeric characters except decimal point
  const cleaned = input.replace(/[^0-9.]/g, '');

  // Handle multiple decimal points
  const parts = cleaned.split('.');
  const formatted = parts.length > 1 ? `${parts[0]}.${parts.slice(1).join('')}` : cleaned;

  return parseFloat(formatted) || 0;
};

/**
 * Format USD value with appropriate notation
 */
export const formatUSDValue = (value: number, compact: boolean = false): string => {
  if (compact && value >= 1000) {
    return formatCurrency(value, 'USD', 0);
  }
  return formatCurrency(value, 'USD');
};
