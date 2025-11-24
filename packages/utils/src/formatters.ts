/**
 * Format blockchain address to short form
 * @param address Full blockchain address
 * @returns Shortened address (0x1234...5678)
 */
export function formatAddress(address: string): string {
  if (!address) return ''
  if (address.length < 12) return address
  return `${address.slice(0, 6)}...${address.slice(-4)}`
}

/**
 * Format balance with proper decimals
 * @param balance Raw balance string
 * @param decimals Number of decimal places (default: 12 for ÉTR)
 * @returns Formatted balance
 */
export function formatBalance(balance: string | number, decimals: number = 12): string {
  const num = typeof balance === 'string' ? parseFloat(balance) : balance
  const adjusted = num / Math.pow(10, decimals)
  return adjusted.toLocaleString(undefined, {
    maximumFractionDigits: 2,
    minimumFractionDigits: 0
  })
}

/**
 * Format large numbers with K, M, B suffixes
 * @param num Number to format
 * @returns Formatted string (1.2K, 3.4M, etc.)
 */
export function formatLargeNumber(num: number): string {
  if (num >= 1_000_000_000) {
    return (num / 1_000_000_000).toFixed(1) + 'B'
  }
  if (num >= 1_000_000) {
    return (num / 1_000_000).toFixed(1) + 'M'
  }
  if (num >= 1_000) {
    return (num / 1_000).toFixed(1) + 'K'
  }
  return num.toLocaleString()
}

/**
 * Format timestamp to relative time (e.g., "2 hours ago")
 * @param timestamp Date object or ISO string
 * @returns Relative time string
 */
export function formatTimeAgo(timestamp: Date | string): string {
  const date = typeof timestamp === 'string' ? new Date(timestamp) : timestamp
  const now = new Date()
  const seconds = Math.floor((now.getTime() - date.getTime()) / 1000)

  if (seconds < 60) return 'just now'
  if (seconds < 3600) return `${Math.floor(seconds / 60)} minutes ago`
  if (seconds < 86400) return `${Math.floor(seconds / 3600)} hours ago`
  if (seconds < 604800) return `${Math.floor(seconds / 86400)} days ago`
  if (seconds < 2592000) return `${Math.floor(seconds / 604800)} weeks ago`
  return date.toLocaleDateString()
}

/**
 * Format percentage with proper rounding
 * @param value Percentage value (0-100 or 0-1)
 * @param isDecimal Whether value is 0-1 (true) or 0-100 (false)
 * @returns Formatted percentage string
 */
export function formatPercentage(value: number, isDecimal: boolean = false): string {
  const percent = isDecimal ? value * 100 : value
  return `${percent.toFixed(2)}%`
}

/**
 * Format duration in seconds to human readable
 * @param seconds Duration in seconds
 * @returns Formatted duration (e.g., "2h 30m")
 */
export function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m`

  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`
}
