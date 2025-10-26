import { formatDistanceToNow, format as dateFormat } from 'date-fns';

const DECIMALS = parseInt(process.env.NEXT_PUBLIC_CHAIN_DECIMALS || '18');
const TOKEN = process.env.NEXT_PUBLIC_CHAIN_TOKEN || 'ETRID';

export function formatBalance(balance: bigint, decimals: number = DECIMALS): string {
  const divisor = BigInt(10 ** decimals);
  const integerPart = balance / divisor;
  const fractionalPart = balance % divisor;

  const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
  const trimmedFractional = fractionalStr.replace(/0+$/, '').slice(0, 4);

  if (trimmedFractional) {
    return `${integerPart.toLocaleString()}.${trimmedFractional}`;
  }
  return integerPart.toLocaleString();
}

export function formatTokenAmount(balance: bigint, showSymbol: boolean = true): string {
  const formatted = formatBalance(balance);
  return showSymbol ? `${formatted} ${TOKEN}` : formatted;
}

export function formatPercentage(value: number, decimals: number = 2): string {
  return `${(value * 100).toFixed(decimals)}%`;
}

export function formatCommission(commission: number): string {
  // Commission is stored as parts per billion (0-1000000000)
  return formatPercentage(commission / 1000000000, 2);
}

export function formatAddress(address: string, length: number = 8): string {
  if (address.length <= length * 2) return address;
  return `${address.slice(0, length)}...${address.slice(-length)}`;
}

export function formatTimeAgo(timestamp: number): string {
  return formatDistanceToNow(new Date(timestamp), { addSuffix: true });
}

export function formatDateTime(timestamp: number): string {
  return dateFormat(new Date(timestamp), 'MMM dd, yyyy HH:mm:ss');
}

export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  }
  return `${secs}s`;
}

export function formatNumber(num: number, decimals: number = 0): string {
  return num.toLocaleString(undefined, {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  });
}

export function formatLargeNumber(num: number): string {
  if (num >= 1e9) return `${(num / 1e9).toFixed(2)}B`;
  if (num >= 1e6) return `${(num / 1e6).toFixed(2)}M`;
  if (num >= 1e3) return `${(num / 1e3).toFixed(2)}K`;
  return num.toString();
}

export function calculateAPY(
  eraReward: bigint,
  totalStake: bigint,
  erasPerDay: number = 4
): number {
  if (totalStake === 0n) return 0;

  const dailyReturn = Number(eraReward) / Number(totalStake) * erasPerDay;
  const annualReturn = dailyReturn * 365;

  return annualReturn;
}
