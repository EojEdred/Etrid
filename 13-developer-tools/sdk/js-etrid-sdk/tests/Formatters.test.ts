/**
 * Tests for Formatter Utilities
 */

import { describe, it, expect } from '@jest/globals';
import {
  formatBalance,
  parseBalance,
  formatETR,
  formatETD,
  formatPercentage,
  formatAddress,
  shortenAddress,
  formatBlockNumber,
  formatTimestamp,
  formatDuration,
  formatBlockTime,
  formatHash,
  formatCompact,
  formatAPY,
  parseCompact,
  formatFee,
  formatRelativeTime,
  validateAndFormatAddress,
  DECIMALS,
} from '../src/utils/formatters';

describe('Formatters', () => {
  describe('Balance Formatting', () => {
    it('should format balance with default options', () => {
      const result = formatBalance(1000000000000000000n);

      expect(result).toContain('1');
      expect(result).toContain('ETR');
    });

    it('should format balance with custom decimals', () => {
      const result = formatBalance(1500000000000000000n, { decimals: 2 });

      expect(result).toBe('1.50 ETR');
    });

    it('should format balance in compact mode', () => {
      const result = formatBalance(1500000000000000000n, { compact: true, decimals: 18 });

      expect(result).toContain('1.5');
    });

    it('should format balance without symbol', () => {
      const result = formatBalance(1000000000000000000n, { includeSymbol: false });

      expect(result).not.toContain('ETR');
    });

    it('should format balance with custom symbol', () => {
      const result = formatBalance(1000000000000000000n, { symbol: 'TEST' });

      expect(result).toContain('TEST');
    });

    it('should handle negative balances', () => {
      const result = formatBalance(-1000000000000000000n);

      expect(result).toContain('-');
    });

    it('should format zero balance', () => {
      const result = formatBalance(0n);

      expect(result).toContain('0');
    });

    it('should format large balance with thousands separator', () => {
      const result = formatBalance(1234567000000000000000000n, { decimals: 18 });

      expect(result).toContain(',');
    });
  });

  describe('Balance Parsing', () => {
    it('should parse balance string', () => {
      const result = parseBalance('1.5 ETR');

      expect(result).toBe(1500000000000000000n);
    });

    it('should parse balance with thousands separator', () => {
      const result = parseBalance('1,000.5');

      expect(result).toBe(1000500000000000000n);
    });

    it('should parse integer balance', () => {
      const result = parseBalance('100');

      expect(result).toBe(100000000000000000000n);
    });

    it('should handle decimal-only input', () => {
      const result = parseBalance('0.5');

      expect(result).toBe(500000000000000000n);
    });
  });

  describe('Currency-Specific Formatters', () => {
    it('should format ETR', () => {
      const result = formatETR(1000000000000000000n);

      expect(result).toContain('ETR');
    });

    it('should format ETD', () => {
      const result = formatETD(1000000000000000000n);

      expect(result).toContain('ETD');
    });

    it('should format fee', () => {
      const result = formatFee(1000000000000000n);

      expect(result).toContain('ETR');
    });
  });

  describe('Percentage Formatting', () => {
    it('should format percentage with default decimals', () => {
      const result = formatPercentage(15.5);

      expect(result).toBe('15.50%');
    });

    it('should format percentage with custom decimals', () => {
      const result = formatPercentage(15.567, 3);

      expect(result).toBe('15.567%');
    });

    it('should format APY', () => {
      const result = formatAPY(12.5);

      expect(result).toBe('12.50%');
    });
  });

  describe('Address Formatting', () => {
    const validAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

    it('should format address with default prefix', () => {
      const result = formatAddress(validAddress);

      expect(result).toBeTruthy();
      expect(typeof result).toBe('string');
    });

    it('should format address with custom prefix', () => {
      const result = formatAddress(validAddress, 0);

      expect(result).toBeTruthy();
    });

    it('should shorten address', () => {
      const result = shortenAddress(validAddress);

      expect(result).toContain('...');
      expect(result.length).toBeLessThan(validAddress.length);
    });

    it('should shorten address with custom lengths', () => {
      const result = shortenAddress(validAddress, 8, 8);

      expect(result).toContain('...');
    });

    it('should not shorten short addresses', () => {
      const shortAddr = '12345';
      const result = shortenAddress(shortAddr);

      expect(result).toBe(shortAddr);
    });

    it('should throw error for invalid address', () => {
      expect(() => formatAddress('invalid')).toThrow();
    });

    it('should validate and format address', () => {
      const result = validateAndFormatAddress(validAddress);

      expect(result.valid).toBe(true);
      expect(result.formatted).toBeDefined();
      expect(result.error).toBeUndefined();
    });

    it('should validate invalid address', () => {
      const result = validateAndFormatAddress('invalid');

      expect(result.valid).toBe(false);
      expect(result.formatted).toBeUndefined();
      expect(result.error).toBeDefined();
    });
  });

  describe('Block Number Formatting', () => {
    it('should format block number with separators', () => {
      const result = formatBlockNumber(1234567);

      expect(result).toBe('1,234,567');
    });

    it('should format small block number', () => {
      const result = formatBlockNumber(123);

      expect(result).toBe('123');
    });
  });

  describe('Timestamp Formatting', () => {
    it('should format timestamp with time', () => {
      const result = formatTimestamp(1640000000000);

      expect(result).toContain('-');
      expect(result).toContain(':');
    });

    it('should format timestamp without time', () => {
      const result = formatTimestamp(1640000000000, false);

      expect(result).toContain('-');
      expect(result).not.toContain(':');
    });

    it('should format recent timestamp', () => {
      const now = Date.now();
      const result = formatTimestamp(now);

      expect(result).toBeTruthy();
    });
  });

  describe('Duration Formatting', () => {
    it('should format duration in minutes', () => {
      const result = formatDuration(10); // 10 blocks = 1 minute

      expect(result).toContain('minute');
    });

    it('should format duration in hours', () => {
      const result = formatDuration(600); // 600 blocks = 1 hour

      expect(result).toContain('hour');
    });

    it('should format duration in days', () => {
      const result = formatDuration(14400); // 14400 blocks = 1 day

      expect(result).toContain('day');
    });

    it('should format complex duration', () => {
      const result = formatDuration(15000); // 1 day, 1 hour

      expect(result).toContain('day');
      expect(result).toContain('hour');
    });

    it('should handle very short duration', () => {
      const result = formatDuration(1);

      expect(result).toContain('less than a minute');
    });

    it('should format block time', () => {
      const result = formatBlockTime(100);

      expect(result).toBeTruthy();
    });
  });

  describe('Hash Formatting', () => {
    it('should format hash', () => {
      const hash = '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef';
      const result = formatHash(hash);

      expect(result).toContain('0x');
      expect(result).toContain('...');
    });

    it('should add 0x prefix if missing', () => {
      const hash = '1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef';
      const result = formatHash(hash);

      expect(result).toContain('0x');
    });

    it('should not shorten short hash', () => {
      const hash = '0x1234';
      const result = formatHash(hash);

      expect(result).toBe(hash);
    });
  });

  describe('Compact Notation', () => {
    it('should format thousands', () => {
      const result = formatCompact(1500);

      expect(result).toBe('1.5K');
    });

    it('should format millions', () => {
      const result = formatCompact(2500000);

      expect(result).toBe('2.5M');
    });

    it('should format billions', () => {
      const result = formatCompact(3500000000);

      expect(result).toBe('3.5B');
    });

    it('should format small numbers without notation', () => {
      const result = formatCompact(500);

      expect(result).toBe('500');
    });

    it('should format bigint values', () => {
      const result = formatCompact(1500n);

      expect(result).toBe('1.5K');
    });

    it('should parse compact notation K', () => {
      const result = parseCompact('1.5K');

      expect(result).toBe(1500);
    });

    it('should parse compact notation M', () => {
      const result = parseCompact('2M');

      expect(result).toBe(2000000);
    });

    it('should parse compact notation B', () => {
      const result = parseCompact('3.5B');

      expect(result).toBe(3500000000);
    });

    it('should parse plain numbers', () => {
      const result = parseCompact('500');

      expect(result).toBe(500);
    });

    it('should throw error for invalid compact notation', () => {
      expect(() => parseCompact('invalid')).toThrow();
    });
  });

  describe('Relative Time', () => {
    it('should format just now', () => {
      const result = formatRelativeTime(Date.now() - 30000);

      expect(result).toBe('just now');
    });

    it('should format minutes ago', () => {
      const result = formatRelativeTime(Date.now() - 120000);

      expect(result).toContain('minute');
      expect(result).toContain('ago');
    });

    it('should format hours ago', () => {
      const result = formatRelativeTime(Date.now() - 3600000);

      expect(result).toContain('hour');
      expect(result).toContain('ago');
    });

    it('should format days ago', () => {
      const result = formatRelativeTime(Date.now() - 86400000);

      expect(result).toContain('day');
      expect(result).toContain('ago');
    });

    it('should format months ago', () => {
      const result = formatRelativeTime(Date.now() - 2592000000);

      expect(result).toContain('month');
      expect(result).toContain('ago');
    });

    it('should format years ago', () => {
      const result = formatRelativeTime(Date.now() - 31536000000);

      expect(result).toContain('year');
      expect(result).toContain('ago');
    });
  });

  describe('Constants', () => {
    it('should have correct ETR decimals', () => {
      expect(DECIMALS.ETR).toBe(18);
    });

    it('should have correct ETD decimals', () => {
      expect(DECIMALS.ETD).toBe(18);
    });
  });

  describe('Edge Cases', () => {
    it('should handle very large balances', () => {
      const result = formatBalance(123456789012345678901234567890n);

      expect(result).toBeTruthy();
    });

    it('should handle fractional balances', () => {
      const result = formatBalance(123456789012345678n);

      expect(result).toBeTruthy();
    });

    it('should parse balance round-trip', () => {
      const original = '1.5 ETR';
      const parsed = parseBalance(original);
      const formatted = formatBalance(parsed, { decimals: 1 });

      expect(formatted).toContain('1.5');
    });
  });
});
