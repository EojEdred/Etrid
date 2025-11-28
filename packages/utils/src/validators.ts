import { decodeAddress, encodeAddress } from '@polkadot/util-crypto'

/**
 * Validate Substrate/Polkadot address
 * @param address Address to validate
 * @returns true if valid
 */
export function isValidAddress(address: string): boolean {
  try {
    decodeAddress(address)
    return true
  } catch {
    return false
  }
}

/**
 * Validate Ethereum address
 * @param address Address to validate
 * @returns true if valid
 */
export function isValidEthereumAddress(address: string): boolean {
  return /^0x[a-fA-F0-9]{40}$/.test(address)
}

/**
 * Validate email address
 * @param email Email to validate
 * @returns true if valid
 */
export function isValidEmail(email: string): boolean {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)
}

/**
 * Validate URL
 * @param url URL to validate
 * @returns true if valid
 */
export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * Validate stake amount
 * @param amount Amount to validate
 * @param min Minimum allowed
 * @param max Maximum allowed
 * @returns true if valid
 */
export function isValidStakeAmount(
  amount: string | number,
  min: number = 0,
  max: number = Infinity
): boolean {
  const num = typeof amount === 'string' ? parseFloat(amount) : amount
  return !isNaN(num) && num >= min && num <= max
}
