/**
 * Utility formatters for Etrid SDK
 */

import { encodeAddress, decodeAddress } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';

/**
 * Format options for balance
 */
export interface BalanceFormatOptions {
  /** Number of decimal places (default: 18) */
  decimals?: number;
  /** Include currency symbol (default: true) */
  includeSymbol?: boolean;
  /** Currency symbol (default: 'ETR') */
  symbol?: string;
  /** Thousands separator (default: ',') */
  thousandsSeparator?: string;
  /** Decimal separator (default: '.') */
  decimalSeparator?: string;
  /** Compact notation for large numbers (default: false) */
  compact?: boolean;
}

/**
 * Default decimals for Etrid currencies
 */
export const DECIMALS = {
  ETR: 18,
  ETD: 18,
} as const;

/**
 * Format balance from smallest unit to human-readable string
 *
 * @example
 * ```typescript
 * formatBalance(1000000000000000000n) // "1.000000000000000000 ETR"
 * formatBalance(1000000000000000000n, { decimals: 2 }) // "1.00 ETR"
 * formatBalance(1500000000000000000n, { compact: true }) // "1.5 ETR"
 * ```
 */
export function formatBalance(
  amount: bigint,
  options: BalanceFormatOptions = {}
): string {
  const {
    decimals = DECIMALS.ETR,
    includeSymbol = true,
    symbol = 'ETR',
    thousandsSeparator = ',',
    decimalSeparator = '.',
    compact = false,
  } = options;

  // Handle negative amounts
  const isNegative = amount < 0n;
  const absAmount = isNegative ? -amount : amount;

  // Convert to decimal
  const divisor = 10n ** BigInt(decimals);
  const wholePart = absAmount / divisor;
  const fractionalPart = absAmount % divisor;

  // Format whole part with thousands separator
  const wholeStr = wholePart.toString().replace(/\B(?=(\d{3})+(?!\d))/g, thousandsSeparator);

  let result: string;

  if (compact) {
    // Compact notation - show up to 4 significant digits
    const totalStr = absAmount.toString();
    if (totalStr.length > decimals) {
      const significantDigits = 4;
      const decimalValue = Number(absAmount) / Number(divisor);
      result = decimalValue.toFixed(Math.min(significantDigits, decimals));
    } else {
      const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
      result = `${wholeStr}${decimalSeparator}${fractionalStr}`;
    }
  } else {
    // Full precision
    const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
    result = `${wholeStr}${decimalSeparator}${fractionalStr}`;
  }

  // Remove trailing zeros
  result = result.replace(/\.?0+$/, '');

  // Add negative sign if needed
  if (isNegative) {
    result = `-${result}`;
  }

  // Add currency symbol
  if (includeSymbol) {
    result = `${result} ${symbol}`;
  }

  return result;
}

/**
 * Parse balance from human-readable string to smallest unit
 *
 * @example
 * ```typescript
 * parseBalance("1.5 ETR") // 1500000000000000000n
 * parseBalance("1,000.5") // 1000500000000000000n
 * ```
 */
export function parseBalance(
  value: string,
  decimals: number = DECIMALS.ETR
): bigint {
  // Remove currency symbols and whitespace
  let cleanValue = value.replace(/[A-Z]+/g, '').trim();

  // Remove thousands separators
  cleanValue = cleanValue.replace(/,/g, '');

  // Split on decimal point
  const parts = cleanValue.split('.');
  const wholePart = parts[0] || '0';
  const fractionalPart = (parts[1] || '').padEnd(decimals, '0').slice(0, decimals);

  // Combine and convert to bigint
  const combined = wholePart + fractionalPart;
  return BigInt(combined);
}

/**
 * Format ETR balance
 */
export function formatETR(amount: bigint, options?: Omit<BalanceFormatOptions, 'symbol'>): string {
  return formatBalance(amount, { ...options, symbol: 'ETR' });
}

/**
 * Format ETD balance
 */
export function formatETD(amount: bigint, options?: Omit<BalanceFormatOptions, 'symbol'>): string {
  return formatBalance(amount, { ...options, symbol: 'ETD' });
}

/**
 * Format percentage
 */
export function formatPercentage(value: number, decimals: number = 2): string {
  return `${value.toFixed(decimals)}%`;
}

/**
 * Format address with custom prefix
 *
 * @example
 * ```typescript
 * formatAddress("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 42)
 * ```
 */
export function formatAddress(address: string, prefix: number = 42): string {
  try {
    const publicKey = decodeAddress(address);
    return encodeAddress(publicKey, prefix);
  } catch (error) {
    throw new Error(`Invalid address: ${address}`);
  }
}

/**
 * Shorten address for display
 *
 * @example
 * ```typescript
 * shortenAddress("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
 * // "5Grwv...utQY"
 * ```
 */
export function shortenAddress(
  address: string,
  prefixLength: number = 5,
  suffixLength: number = 4
): string {
  if (address.length <= prefixLength + suffixLength) {
    return address;
  }
  return `${address.slice(0, prefixLength)}...${address.slice(-suffixLength)}`;
}

/**
 * Format block number with separators
 *
 * @example
 * ```typescript
 * formatBlockNumber(1234567) // "1,234,567"
 * ```
 */
export function formatBlockNumber(blockNumber: number): string {
  return blockNumber.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

/**
 * Format timestamp to human-readable date
 *
 * @example
 * ```typescript
 * formatTimestamp(1640000000000) // "2021-12-20 12:26:40"
 * ```
 */
export function formatTimestamp(timestamp: number, includeTime: boolean = true): string {
  const date = new Date(timestamp);

  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');

  if (!includeTime) {
    return `${year}-${month}-${day}`;
  }

  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');

  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}

/**
 * Format duration from blocks
 * Assumes 6 second block time
 *
 * @example
 * ```typescript
 * formatDuration(600) // "1 hour"
 * formatDuration(28800) // "2 days"
 * ```
 */
export function formatDuration(blocks: number, blockTime: number = 6): string {
  const seconds = blocks * blockTime;

  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  const parts: string[] = [];

  if (days > 0) {
    parts.push(`${days} day${days !== 1 ? 's' : ''}`);
  }
  if (hours > 0) {
    parts.push(`${hours} hour${hours !== 1 ? 's' : ''}`);
  }
  if (minutes > 0 && days === 0) {
    parts.push(`${minutes} minute${minutes !== 1 ? 's' : ''}`);
  }

  if (parts.length === 0) {
    return 'less than a minute';
  }

  return parts.join(', ');
}

/**
 * Format block time (blocks to human readable time)
 *
 * @example
 * ```typescript
 * formatBlockTime(100) // "10 minutes"
 * ```
 */
export function formatBlockTime(blocks: number, blockTime: number = 6): string {
  return formatDuration(blocks, blockTime);
}

/**
 * Format hash (transaction or block)
 *
 * @example
 * ```typescript
 * formatHash("0x1234...abcd") // "0x1234...abcd"
 * formatHash("0x1234567890abcdef", 6, 4) // "0x123456...cdef"
 * ```
 */
export function formatHash(
  hash: string,
  prefixLength: number = 10,
  suffixLength: number = 8
): string {
  if (!hash.startsWith('0x')) {
    hash = `0x${hash}`;
  }

  if (hash.length <= prefixLength + suffixLength) {
    return hash;
  }

  return `${hash.slice(0, prefixLength)}...${hash.slice(-suffixLength)}`;
}

/**
 * Convert hex to bytes
 */
export function hexToBytes(hex: string): Uint8Array {
  if (hex.startsWith('0x')) {
    hex = hex.slice(2);
  }

  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < hex.length; i += 2) {
    bytes[i / 2] = parseInt(hex.slice(i, i + 2), 16);
  }

  return bytes;
}

/**
 * Convert bytes to hex
 */
export function bytesToHex(bytes: Uint8Array): string {
  return u8aToHex(bytes);
}

/**
 * Format large numbers with compact notation
 *
 * @example
 * ```typescript
 * formatCompact(1000) // "1K"
 * formatCompact(1500000) // "1.5M"
 * formatCompact(1000000000) // "1B"
 * ```
 */
export function formatCompact(value: number | bigint): string {
  const num = typeof value === 'bigint' ? Number(value) : value;

  if (num >= 1e9) {
    return `${(num / 1e9).toFixed(1)}B`;
  }
  if (num >= 1e6) {
    return `${(num / 1e6).toFixed(1)}M`;
  }
  if (num >= 1e3) {
    return `${(num / 1e3).toFixed(1)}K`;
  }

  return num.toString();
}

/**
 * Format APY/APR
 */
export function formatAPY(apy: number, decimals: number = 2): string {
  return `${apy.toFixed(decimals)}%`;
}

/**
 * Parse compact notation to number
 *
 * @example
 * ```typescript
 * parseCompact("1.5K") // 1500
 * parseCompact("2M") // 2000000
 * ```
 */
export function parseCompact(value: string): number {
  const match = value.match(/^([\d.]+)([KMB]?)$/i);

  if (!match) {
    throw new Error(`Invalid compact notation: ${value}`);
  }

  const num = parseFloat(match[1]);
  const suffix = match[2].toUpperCase();

  switch (suffix) {
    case 'K':
      return num * 1e3;
    case 'M':
      return num * 1e6;
    case 'B':
      return num * 1e9;
    default:
      return num;
  }
}

/**
 * Format transaction fee
 */
export function formatFee(fee: bigint, options?: Omit<BalanceFormatOptions, 'symbol'>): string {
  return formatBalance(fee, { ...options, symbol: 'ETR', compact: true });
}

/**
 * Format relative time
 *
 * @example
 * ```typescript
 * formatRelativeTime(Date.now() - 60000) // "1 minute ago"
 * ```
 */
export function formatRelativeTime(timestamp: number): string {
  const now = Date.now();
  const diff = now - timestamp;
  const seconds = Math.floor(diff / 1000);

  if (seconds < 60) {
    return 'just now';
  }

  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) {
    return `${minutes} minute${minutes !== 1 ? 's' : ''} ago`;
  }

  const hours = Math.floor(minutes / 60);
  if (hours < 24) {
    return `${hours} hour${hours !== 1 ? 's' : ''} ago`;
  }

  const days = Math.floor(hours / 24);
  if (days < 30) {
    return `${days} day${days !== 1 ? 's' : ''} ago`;
  }

  const months = Math.floor(days / 30);
  if (months < 12) {
    return `${months} month${months !== 1 ? 's' : ''} ago`;
  }

  const years = Math.floor(months / 12);
  return `${years} year${years !== 1 ? 's' : ''} ago`;
}

/**
 * Validate and format address
 */
export function validateAndFormatAddress(address: string, prefix: number = 42): {
  valid: boolean;
  formatted?: string;
  error?: string;
} {
  try {
    const formatted = formatAddress(address, prefix);
    return { valid: true, formatted };
  } catch (error) {
    return {
      valid: false,
      error: error instanceof Error ? error.message : 'Invalid address',
    };
  }
}
