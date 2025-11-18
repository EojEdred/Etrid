import { mnemonicValidate } from '@polkadot/util-crypto';
import { decodeAddress, encodeAddress } from '@polkadot/keyring';
import { hexToU8a, isHex } from '@polkadot/util';
import { VALIDATION_RULES } from './constants';
import BigNumber from 'bignumber.js';

/**
 * Validate Substrate address
 */
export const isValidAddress = (address: string): boolean => {
  try {
    if (!address || address.length === 0) {
      return false;
    }

    // Try to decode the address
    const decoded = decodeAddress(address);

    // Check if decoded successfully
    if (decoded.length !== 32) {
      return false;
    }

    // Try to re-encode to verify
    const reEncoded = encodeAddress(decoded);

    return true;
  } catch (error) {
    return false;
  }
};

/**
 * Validate mnemonic phrase
 */
export const isValidMnemonic = (mnemonic: string): boolean => {
  try {
    if (!mnemonic || mnemonic.trim().length === 0) {
      return false;
    }

    const trimmed = mnemonic.trim();
    const words = trimmed.split(/\s+/);

    // Check word count (should be 12, 15, 18, 21, or 24)
    if (![12, 15, 18, 21, 24].includes(words.length)) {
      return false;
    }

    // Validate using polkadot utility
    return mnemonicValidate(trimmed);
  } catch (error) {
    return false;
  }
};

/**
 * Validate transaction amount
 */
export const isValidAmount = (
  amount: string | number,
  options: {
    min?: string | number;
    max?: string | number;
    allowZero?: boolean;
  } = {}
): { valid: boolean; error?: string } => {
  try {
    const { min = VALIDATION_RULES.MIN_TRANSACTION_AMOUNT, max = VALIDATION_RULES.MAX_TRANSACTION_AMOUNT, allowZero = false } = options;

    const amountStr = typeof amount === 'number' ? amount.toString() : amount;

    // Check if empty
    if (!amountStr || amountStr.trim().length === 0) {
      return { valid: false, error: 'Amount is required' };
    }

    // Check if valid number
    const amountNum = parseFloat(amountStr);
    if (isNaN(amountNum)) {
      return { valid: false, error: 'Invalid amount' };
    }

    // Check if negative
    if (amountNum < 0) {
      return { valid: false, error: 'Amount cannot be negative' };
    }

    // Check if zero
    if (amountNum === 0 && !allowZero) {
      return { valid: false, error: 'Amount must be greater than zero' };
    }

    // Check minimum
    const minNum = typeof min === 'number' ? min : parseFloat(min);
    if (amountNum < minNum) {
      return { valid: false, error: `Amount must be at least ${min}` };
    }

    // Check maximum
    const maxNum = typeof max === 'number' ? max : parseFloat(max);
    if (amountNum > maxNum) {
      return { valid: false, error: `Amount cannot exceed ${max}` };
    }

    return { valid: true };
  } catch (error) {
    return { valid: false, error: 'Invalid amount format' };
  }
};

/**
 * Validate sufficient balance
 */
export const hasSufficientBalance = (
  balance: string | number,
  amount: string | number,
  fee: string | number = 0
): { sufficient: boolean; error?: string } => {
  try {
    const balanceBN = new BigNumber(balance);
    const amountBN = new BigNumber(amount);
    const feeBN = new BigNumber(fee);

    const total = amountBN.plus(feeBN);

    if (balanceBN.isLessThan(total)) {
      const shortfall = total.minus(balanceBN);
      return {
        sufficient: false,
        error: `Insufficient balance. You need ${shortfall.toString()} more.`,
      };
    }

    return { sufficient: true };
  } catch (error) {
    return { sufficient: false, error: 'Error checking balance' };
  }
};

/**
 * Validate email
 */
export const isValidEmail = (email: string): boolean => {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
};

/**
 * Validate password strength
 */
export const isValidPassword = (
  password: string
): { valid: boolean; strength: 'weak' | 'medium' | 'strong'; errors: string[] } => {
  const errors: string[] = [];
  let strength: 'weak' | 'medium' | 'strong' = 'weak';

  if (password.length < VALIDATION_RULES.MIN_PASSWORD_LENGTH) {
    errors.push(`Password must be at least ${VALIDATION_RULES.MIN_PASSWORD_LENGTH} characters`);
  }

  if (!/[a-z]/.test(password)) {
    errors.push('Password must contain lowercase letters');
  }

  if (!/[A-Z]/.test(password)) {
    errors.push('Password must contain uppercase letters');
  }

  if (!/[0-9]/.test(password)) {
    errors.push('Password must contain numbers');
  }

  if (!/[^a-zA-Z0-9]/.test(password)) {
    errors.push('Password must contain special characters');
  }

  // Calculate strength
  if (errors.length === 0) {
    strength = password.length >= 12 ? 'strong' : 'medium';
  } else if (errors.length <= 2) {
    strength = 'medium';
  }

  return {
    valid: errors.length === 0,
    strength,
    errors,
  };
};

/**
 * Validate phone number
 */
export const isValidPhoneNumber = (phone: string): boolean => {
  const phoneRegex = /^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$/;
  return phoneRegex.test(phone);
};

/**
 * Validate URL
 */
export const isValidURL = (url: string): boolean => {
  try {
    new URL(url);
    return true;
  } catch (error) {
    return false;
  }
};

/**
 * Validate hex string
 */
export const isValidHexString = (hex: string): boolean => {
  return isHex(hex);
};

/**
 * Validate transaction hash
 */
export const isValidTransactionHash = (hash: string): boolean => {
  return isValidHexString(hash) && hash.length === 66; // 0x + 64 chars
};

/**
 * Validate numeric input
 */
export const isNumericInput = (input: string): boolean => {
  // Allow numbers and single decimal point
  return /^\d*\.?\d*$/.test(input);
};

/**
 * Validate alphanumeric
 */
export const isAlphanumeric = (input: string): boolean => {
  return /^[a-zA-Z0-9]+$/.test(input);
};

/**
 * Validate name (letters, spaces, hyphens)
 */
export const isValidName = (name: string): boolean => {
  return /^[a-zA-Z\s\-']+$/.test(name) && name.trim().length > 0;
};

/**
 * Sanitize input (remove potentially dangerous characters)
 */
export const sanitizeInput = (input: string): string => {
  return input.replace(/[<>\"']/g, '');
};

/**
 * Validate JSON string
 */
export const isValidJSON = (str: string): boolean => {
  try {
    JSON.parse(str);
    return true;
  } catch (error) {
    return false;
  }
};

/**
 * Check if input is empty or whitespace
 */
export const isEmpty = (input: string): boolean => {
  return !input || input.trim().length === 0;
};

/**
 * Validate date range
 */
export const isValidDateRange = (startDate: Date, endDate: Date): boolean => {
  return startDate < endDate;
};

/**
 * Validate BIP39 word
 */
export const isValidBIP39Word = (word: string): boolean => {
  // This is a simplified check - in production, check against BIP39 wordlist
  return /^[a-z]{3,8}$/.test(word.toLowerCase());
};

/**
 * Validate recovery phrase word count
 */
export const isValidWordCount = (words: string[]): boolean => {
  return [12, 15, 18, 21, 24].includes(words.length);
};

/**
 * Comprehensive form validation
 */
export const validateForm = (
  fields: { [key: string]: any },
  rules: { [key: string]: (value: any) => { valid: boolean; error?: string } }
): { valid: boolean; errors: { [key: string]: string } } => {
  const errors: { [key: string]: string } = {};

  for (const [field, rule] of Object.entries(rules)) {
    const value = fields[field];
    const result = rule(value);

    if (!result.valid && result.error) {
      errors[field] = result.error;
    }
  }

  return {
    valid: Object.keys(errors).length === 0,
    errors,
  };
};
