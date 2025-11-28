/**
 * Multi-Signature Wallet Service
 * Handles all multi-sig wallet operations
 */

import {
  MultiSigWallet,
  MultiSigParams,
  Proposal,
  TransactionInput,
  Signature,
  MultiSigStats,
} from '../types/multisig';

export class MultiSigService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * Create a new multi-sig wallet
   */
  async createWallet(params: MultiSigParams): Promise<MultiSigWallet> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(params),
      });

      if (!response.ok) {
        throw new Error('Failed to create multi-sig wallet');
      }

      return await response.json();
    } catch (error) {
      console.error('Error creating multi-sig wallet:', error);
      throw error;
    }
  }

  /**
   * Get all multi-sig wallets for the current user
   */
  async getWallets(): Promise<MultiSigWallet[]> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/list`);

      if (!response.ok) {
        throw new Error('Failed to fetch multi-sig wallets');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching multi-sig wallets:', error);
      throw error;
    }
  }

  /**
   * Get a specific multi-sig wallet
   */
  async getWallet(walletId: string): Promise<MultiSigWallet> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}`);

      if (!response.ok) {
        throw new Error('Failed to fetch multi-sig wallet');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching multi-sig wallet:', error);
      throw error;
    }
  }

  /**
   * Propose a new transaction
   */
  async proposeTransaction(
    walletId: string,
    tx: TransactionInput
  ): Promise<Proposal> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}/propose`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(tx),
      });

      if (!response.ok) {
        throw new Error('Failed to propose transaction');
      }

      return await response.json();
    } catch (error) {
      console.error('Error proposing transaction:', error);
      throw error;
    }
  }

  /**
   * Sign a pending transaction
   */
  async signTransaction(proposalId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/proposal/${proposalId}/sign`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to sign transaction');
      }
    } catch (error) {
      console.error('Error signing transaction:', error);
      throw error;
    }
  }

  /**
   * Reject a pending transaction
   */
  async rejectTransaction(proposalId: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/proposal/${proposalId}/reject`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to reject transaction');
      }
    } catch (error) {
      console.error('Error rejecting transaction:', error);
      throw error;
    }
  }

  /**
   * Execute a fully signed transaction
   */
  async executeTransaction(proposalId: string): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/proposal/${proposalId}/execute`, {
        method: 'POST',
      });

      if (!response.ok) {
        throw new Error('Failed to execute transaction');
      }

      return await response.json();
    } catch (error) {
      console.error('Error executing transaction:', error);
      throw error;
    }
  }

  /**
   * Add a new signer to the wallet
   */
  async addSigner(walletId: string, signer: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}/signers`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ signer }),
      });

      if (!response.ok) {
        throw new Error('Failed to add signer');
      }
    } catch (error) {
      console.error('Error adding signer:', error);
      throw error;
    }
  }

  /**
   * Remove a signer from the wallet
   */
  async removeSigner(walletId: string, signer: string): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}/signers/${signer}`, {
        method: 'DELETE',
      });

      if (!response.ok) {
        throw new Error('Failed to remove signer');
      }
    } catch (error) {
      console.error('Error removing signer:', error);
      throw error;
    }
  }

  /**
   * Change the signing threshold
   */
  async changeThreshold(walletId: string, newThreshold: number): Promise<void> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}/threshold`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ threshold: newThreshold }),
      });

      if (!response.ok) {
        throw new Error('Failed to change threshold');
      }
    } catch (error) {
      console.error('Error changing threshold:', error);
      throw error;
    }
  }

  /**
   * Get pending approvals for a wallet
   */
  async getPendingApprovals(walletId: string): Promise<Proposal[]> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/${walletId}/pending`);

      if (!response.ok) {
        throw new Error('Failed to fetch pending approvals');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching pending approvals:', error);
      throw error;
    }
  }

  /**
   * Get all pending approvals for the current user
   */
  async getAllPendingApprovals(): Promise<Proposal[]> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/pending/all`);

      if (!response.ok) {
        throw new Error('Failed to fetch pending approvals');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching pending approvals:', error);
      throw error;
    }
  }

  /**
   * Get multi-sig statistics
   */
  async getStats(): Promise<MultiSigStats> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/stats`);

      if (!response.ok) {
        throw new Error('Failed to fetch stats');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching stats:', error);
      throw error;
    }
  }

  /**
   * Get proposal details
   */
  async getProposal(proposalId: string): Promise<Proposal> {
    try {
      const response = await fetch(`${this.baseUrl}/multisig/proposal/${proposalId}`);

      if (!response.ok) {
        throw new Error('Failed to fetch proposal');
      }

      return await response.json();
    } catch (error) {
      console.error('Error fetching proposal:', error);
      throw error;
    }
  }
}

// Export singleton instance
export const multiSigService = new MultiSigService();
