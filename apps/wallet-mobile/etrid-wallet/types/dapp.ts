/**
 * DApp Browser Type Definitions
 */

export interface DApp {
  id: string;
  name: string;
  url: string;
  category: DAppCategory;
  description: string;
  iconUrl: string;
  rating: number;
  userCount?: number;
  isFeatured?: boolean;
  isTrending?: boolean;
}

export type DAppCategory =
  | 'DeFi'
  | 'NFT'
  | 'Gaming'
  | 'Social'
  | 'Tools'
  | 'Other';

export interface Bookmark {
  id: string;
  userId: string;
  url: string;
  title: string;
  faviconUrl?: string;
  folder?: string;
  createdAt: Date;
}

export interface ConnectedDApp {
  id: string;
  url: string;
  name: string;
  iconUrl?: string;
  permissions: DAppPermission[];
  connectedAt: Date;
  lastActiveAt: Date;
}

export type DAppPermissionType =
  | 'read_balance'
  | 'sign_transaction'
  | 'access_address'
  | 'send_transaction';

export interface DAppPermission {
  type: DAppPermissionType;
  grantedAt: Date;
  autoApprove?: boolean;
}

export interface DAppRequest {
  id: string;
  dAppUrl: string;
  method: string;
  params: any[];
  timestamp: Date;
}

export interface WalletConnectSession {
  id: string;
  topic: string;
  dAppUrl: string;
  dAppName: string;
  dAppIcon?: string;
  permissions: string[];
  chains: string[];
  createdAt: Date;
  expiresAt: Date;
}

export interface WalletConnectProposal {
  id: string;
  proposer: {
    name: string;
    description: string;
    url: string;
    icons: string[];
  };
  permissions: {
    chains: string[];
    methods: string[];
    events: string[];
  };
  expiresAt: Date;
}

export interface TransactionRequest {
  from: string;
  to: string;
  value?: string;
  data?: string;
  gas?: string;
  gasPrice?: string;
  nonce?: number;
}

export interface TransactionSimulation {
  success: boolean;
  gasEstimate: string;
  balanceChanges: {
    asset: string;
    amount: string;
    direction: 'in' | 'out';
  }[];
  warnings: string[];
}

export interface BrowserTab {
  id: string;
  url: string;
  title: string;
  faviconUrl?: string;
  isActive: boolean;
}

export interface BrowserHistory {
  id: string;
  url: string;
  title: string;
  visitedAt: Date;
}

export interface Web3Provider {
  isConnected: boolean;
  chainId?: string;
  selectedAddress?: string;
  request: (args: { method: string; params?: any[] }) => Promise<any>;
  on: (event: string, handler: (...args: any[]) => void) => void;
  removeListener: (event: string, handler: (...args: any[]) => void) => void;
}
