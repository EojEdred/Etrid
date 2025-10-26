import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}

export function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US').format(num);
}

export function formatPercentage(value: number): string {
  return `${value.toFixed(2)}%`;
}

export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(2)}s`;
  if (ms < 3600000) return `${(ms / 60000).toFixed(2)}m`;
  return `${(ms / 3600000).toFixed(2)}h`;
}

export function truncateAddress(address: string, length: number = 8): string {
  if (address.length <= length * 2) return address;
  return `${address.slice(0, length)}...${address.slice(-length)}`;
}

export function getStatusColor(status: string): string {
  const colors: Record<string, string> = {
    active: 'text-green-500',
    inactive: 'text-gray-500',
    disputed: 'text-yellow-500',
    closed: 'text-red-500',
  };
  return colors[status] || 'text-gray-500';
}

export function getSeverityColor(severity: string): string {
  const colors: Record<string, string> = {
    low: 'text-blue-500 bg-blue-50 dark:bg-blue-950',
    medium: 'text-yellow-500 bg-yellow-50 dark:bg-yellow-950',
    high: 'text-orange-500 bg-orange-50 dark:bg-orange-950',
    critical: 'text-red-500 bg-red-50 dark:bg-red-950',
  };
  return colors[severity] || 'text-gray-500';
}

export function calculateReputationScore(metrics: {
  fraudDetections: number;
  falsePositives: number;
  uptime: number;
  successfulInterventions: number;
}): number {
  const fraudAccuracy = metrics.fraudDetections > 0
    ? ((metrics.fraudDetections - metrics.falsePositives) / metrics.fraudDetections) * 100
    : 0;
  const uptimeScore = metrics.uptime;
  const interventionScore = Math.min(metrics.successfulInterventions * 2, 100);

  return Math.round((fraudAccuracy * 0.4 + uptimeScore * 0.4 + interventionScore * 0.2));
}

export function timeAgo(date: Date): string {
  const seconds = Math.floor((new Date().getTime() - date.getTime()) / 1000);

  const intervals: Record<string, number> = {
    year: 31536000,
    month: 2592000,
    week: 604800,
    day: 86400,
    hour: 3600,
    minute: 60,
    second: 1,
  };

  for (const [unit, secondsInUnit] of Object.entries(intervals)) {
    const interval = Math.floor(seconds / secondsInUnit);
    if (interval >= 1) {
      return `${interval} ${unit}${interval !== 1 ? 's' : ''} ago`;
    }
  }

  return 'just now';
}
