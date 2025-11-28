// Type definitions for the Lightning Network landing page

export interface Chain {
  name: string;
  symbol: string;
  status: 'Live' | 'Coming Soon';
}

export interface Feature {
  icon: React.ComponentType;
  title: string;
  description: string;
}

export interface Statistic {
  value: string | number;
  label: string;
  format?: 'currency' | 'number' | 'percentage';
}

export interface RoadmapItem {
  quarter: string;
  features: string;
  completed?: boolean;
}

export interface Invoice {
  id: string;
  amount: string;
  description: string;
  qrData: string;
  expiresAt: number;
}
