/**
 * Multi-Signature Wallet Types
 * Defines types for multi-sig wallets, proposals, and signatures
 */

export interface MultiSigWallet {
  id: string;
  name: string;
  address: string;
  threshold: number; // M in M-of-N
  totalSigners: number; // N in M-of-N
  signers: string[];
  balance: string;
  purpose: 'personal' | 'business' | 'dao' | 'couples';
  createdAt: Date;
  pendingApprovals: number;
}

export interface MultiSigParams {
  name: string;
  purpose: 'personal' | 'business' | 'dao' | 'couples';
  signers: string[];
  threshold: number;
}

export interface Signer {
  address: string;
  username?: string;
  addedAt: Date;
  isOwner: boolean;
}

export interface Proposal {
  id: string;
  walletId: string;
  proposer: string;
  proposerUsername?: string;
  type: 'transfer' | 'contract_call' | 'add_signer' | 'remove_signer' | 'change_threshold';
  transactionData: TransactionInput;
  signatures: Signature[];
  signaturesCollected: number;
  threshold: number;
  status: 'pending' | 'approved' | 'executed' | 'rejected' | 'expired';
  createdAt: Date;
  expiresAt: Date;
  executedAt?: Date;
}

export interface TransactionInput {
  to?: string;
  amount?: string;
  data?: string;
  description?: string;
}

export interface Signature {
  signer: string;
  signature: string;
  signedAt: Date;
  status: 'approved' | 'rejected';
}

export interface MultiSigStats {
  totalWallets: number;
  totalBalance: string;
  pendingApprovals: number;
  walletsNeedingAction: number;
}

export interface SignerWithStatus extends Signer {
  hasSigned: boolean;
  status: 'pending' | 'approved' | 'rejected';
}

export interface WalletConfig {
  name: string;
  description: string;
  recommendedThreshold: string;
  example: string;
  minSigners: number;
  maxSigners: number;
}

export const WALLET_CONFIGS: Record<string, WalletConfig> = {
  personal: {
    name: 'Personal',
    description: 'You + trusted contact',
    recommendedThreshold: '2-of-2',
    example: 'You + partner for savings',
    minSigners: 2,
    maxSigners: 3,
  },
  couples: {
    name: 'Couples',
    description: 'Joint account for couples',
    recommendedThreshold: '1-of-2 or 2-of-2',
    example: 'Married couples, shared expenses',
    minSigners: 2,
    maxSigners: 2,
  },
  business: {
    name: 'Business',
    description: 'Company treasury',
    recommendedThreshold: '3-of-5',
    example: 'Board members approval',
    minSigners: 3,
    maxSigners: 9,
  },
  dao: {
    name: 'DAO',
    description: 'Decentralized organization',
    recommendedThreshold: '5-of-9 or 7-of-13',
    example: 'Community governance',
    minSigners: 5,
    maxSigners: 20,
  },
};
