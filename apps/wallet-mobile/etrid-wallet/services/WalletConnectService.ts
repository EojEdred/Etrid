/**
 * WalletConnect Service
 * Handles WalletConnect v2 protocol for connecting with dApps
 */

import {
  WalletConnectSession,
  WalletConnectProposal,
  TransactionRequest,
} from '@/types/dapp';

export class WalletConnectService {
  private sessions: Map<string, WalletConnectSession> = new Map();
  private proposals: Map<string, WalletConnectProposal> = new Map();
  private client: any = null;

  /**
   * Initialize WalletConnect client
   */
  async initialize(): Promise<void> {
    // TODO: Initialize WalletConnect v2 client
    // This would use @walletconnect/sign-client
    /*
    this.client = await SignClient.init({
      projectId: process.env.WALLETCONNECT_PROJECT_ID,
      metadata: {
        name: 'Ëtrid Wallet',
        description: 'Ëtrid Mobile Wallet',
        url: 'https://etrid.io',
        icons: ['https://etrid.io/icon.png']
      }
    });

    // Register event listeners
    this.client.on('session_proposal', this.handleSessionProposal);
    this.client.on('session_request', this.handleSessionRequest);
    this.client.on('session_delete', this.handleSessionDelete);
    */
  }

  /**
   * Pair with dApp using URI (from QR code or deep link)
   */
  async pair(uri: string): Promise<void> {
    try {
      // Parse WalletConnect URI
      if (!uri.startsWith('wc:')) {
        throw new Error('Invalid WalletConnect URI');
      }

      // Pair with the dApp
      // await this.client.pair({ uri });

      // The session_proposal event will be triggered
      console.log('Pairing initiated with URI:', uri);
    } catch (error) {
      console.error('Failed to pair:', error);
      throw error;
    }
  }

  /**
   * Approve session proposal
   */
  async approveSession(proposalId: string): Promise<WalletConnectSession> {
    const proposal = this.proposals.get(proposalId);
    if (!proposal) {
      throw new Error('Proposal not found');
    }

    try {
      // Get user's account and chain
      const account = await this.getCurrentAccount();
      const chainId = await this.getCurrentChainId();

      // Approve the session
      /*
      const { topic } = await this.client.approve({
        id: Number(proposalId),
        namespaces: {
          eip155: {
            accounts: [`eip155:${chainId}:${account}`],
            methods: proposal.permissions.methods,
            events: proposal.permissions.events,
          }
        }
      });
      */

      // Create session object
      const session: WalletConnectSession = {
        id: proposalId,
        topic: 'mock-topic', // Would be from client.approve()
        dAppUrl: proposal.proposer.url,
        dAppName: proposal.proposer.name,
        dAppIcon: proposal.proposer.icons[0],
        permissions: proposal.permissions.methods,
        chains: proposal.permissions.chains,
        createdAt: new Date(),
        expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000), // 7 days
      };

      this.sessions.set(session.id, session);
      this.proposals.delete(proposalId);

      // Persist to storage
      await this.saveSessions();

      return session;
    } catch (error) {
      console.error('Failed to approve session:', error);
      throw error;
    }
  }

  /**
   * Reject session proposal
   */
  async rejectSession(proposalId: string): Promise<void> {
    const proposal = this.proposals.get(proposalId);
    if (!proposal) {
      throw new Error('Proposal not found');
    }

    try {
      // Reject the session
      /*
      await this.client.reject({
        id: Number(proposalId),
        reason: {
          code: 5000,
          message: 'User rejected'
        }
      });
      */

      this.proposals.delete(proposalId);
    } catch (error) {
      console.error('Failed to reject session:', error);
      throw error;
    }
  }

  /**
   * Sign transaction for WalletConnect session
   */
  async signTransaction(
    sessionId: string,
    transaction: TransactionRequest
  ): Promise<string> {
    const session = this.sessions.get(sessionId);
    if (!session) {
      throw new Error('Session not found');
    }

    // Request user approval
    const approved = await this.requestTransactionApproval(session, transaction);
    if (!approved) {
      throw new Error('User rejected transaction');
    }

    // Sign transaction
    const signedTx = await this.signTransactionInternal(transaction);
    return signedTx;
  }

  /**
   * Sign message for WalletConnect session
   */
  async signMessage(sessionId: string, message: string): Promise<string> {
    const session = this.sessions.get(sessionId);
    if (!session) {
      throw new Error('Session not found');
    }

    // Request user approval
    const approved = await this.requestSignatureApproval(session, message);
    if (!approved) {
      throw new Error('User rejected signature');
    }

    // Sign message
    const signature = await this.signMessageInternal(message);
    return signature;
  }

  /**
   * Get all active sessions
   */
  async getSessions(): Promise<WalletConnectSession[]> {
    // Load from storage if not in memory
    if (this.sessions.size === 0) {
      await this.loadSessions();
    }

    return Array.from(this.sessions.values());
  }

  /**
   * Get pending proposals
   */
  async getProposals(): Promise<WalletConnectProposal[]> {
    return Array.from(this.proposals.values());
  }

  /**
   * Disconnect session
   */
  async disconnect(sessionId: string): Promise<void> {
    const session = this.sessions.get(sessionId);
    if (!session) {
      throw new Error('Session not found');
    }

    try {
      // Disconnect the session
      /*
      await this.client.disconnect({
        topic: session.topic,
        reason: {
          code: 6000,
          message: 'User disconnected'
        }
      });
      */

      this.sessions.delete(sessionId);
      await this.saveSessions();
    } catch (error) {
      console.error('Failed to disconnect session:', error);
      throw error;
    }
  }

  /**
   * Handle incoming session proposal
   */
  private handleSessionProposal = (proposal: any): void => {
    const wcProposal: WalletConnectProposal = {
      id: proposal.id.toString(),
      proposer: {
        name: proposal.params.proposer.metadata.name,
        description: proposal.params.proposer.metadata.description,
        url: proposal.params.proposer.metadata.url,
        icons: proposal.params.proposer.metadata.icons,
      },
      permissions: {
        chains: proposal.params.requiredNamespaces.eip155?.chains || [],
        methods: proposal.params.requiredNamespaces.eip155?.methods || [],
        events: proposal.params.requiredNamespaces.eip155?.events || [],
      },
      expiresAt: new Date(proposal.params.expiryTimestamp * 1000),
    };

    this.proposals.set(wcProposal.id, wcProposal);

    // Notify UI (would emit event or use callback)
    console.log('New session proposal:', wcProposal);
  };

  /**
   * Handle incoming session request (transaction, signature, etc.)
   */
  private handleSessionRequest = async (request: any): Promise<void> => {
    const { topic, params } = request;
    const session = Array.from(this.sessions.values()).find(
      (s) => s.topic === topic
    );

    if (!session) {
      console.error('Session not found for request:', topic);
      return;
    }

    const { request: sessionRequest } = params;

    try {
      let result: any;

      switch (sessionRequest.method) {
        case 'eth_sendTransaction':
          result = await this.signTransaction(
            session.id,
            sessionRequest.params[0]
          );
          break;

        case 'personal_sign':
          result = await this.signMessage(session.id, sessionRequest.params[0]);
          break;

        case 'eth_signTypedData':
        case 'eth_signTypedData_v4':
          result = await this.signTypedData(
            session.id,
            sessionRequest.params[1]
          );
          break;

        default:
          throw new Error(`Unsupported method: ${sessionRequest.method}`);
      }

      // Send response
      /*
      await this.client.respond({
        topic,
        response: {
          id: request.id,
          jsonrpc: '2.0',
          result
        }
      });
      */
    } catch (error: any) {
      // Send error response
      /*
      await this.client.respond({
        topic,
        response: {
          id: request.id,
          jsonrpc: '2.0',
          error: {
            code: -32000,
            message: error.message
          }
        }
      });
      */
    }
  };

  /**
   * Handle session deletion
   */
  private handleSessionDelete = (event: any): void => {
    const { topic } = event;
    const session = Array.from(this.sessions.values()).find(
      (s) => s.topic === topic
    );

    if (session) {
      this.sessions.delete(session.id);
      this.saveSessions();
      console.log('Session deleted:', session.id);
    }
  };

  /**
   * Sign typed data (EIP-712)
   */
  private async signTypedData(sessionId: string, typedData: string): Promise<string> {
    const session = this.sessions.get(sessionId);
    if (!session) {
      throw new Error('Session not found');
    }

    const approved = await this.requestSignatureApproval(session, typedData);
    if (!approved) {
      throw new Error('User rejected signature');
    }

    const signature = await this.signTypedDataInternal(typedData);
    return signature;
  }

  // Storage methods
  private async saveSessions(): Promise<void> {
    const sessionsArray = Array.from(this.sessions.values());
    // TODO: Persist to AsyncStorage or SecureStore
    // await AsyncStorage.setItem('wc_sessions', JSON.stringify(sessionsArray));
  }

  private async loadSessions(): Promise<void> {
    // TODO: Load from AsyncStorage or SecureStore
    // const sessionsJson = await AsyncStorage.getItem('wc_sessions');
    // if (sessionsJson) {
    //   const sessions = JSON.parse(sessionsJson);
    //   sessions.forEach((session: WalletConnectSession) => {
    //     this.sessions.set(session.id, session);
    //   });
    // }
  }

  // Mock methods that would be implemented with actual wallet integration
  private async getCurrentAccount(): Promise<string> {
    // TODO: Get from wallet service
    return '0x0000000000000000000000000000000000000000';
  }

  private async getCurrentChainId(): Promise<string> {
    // TODO: Get from wallet service
    return '1';
  }

  private async requestTransactionApproval(
    session: WalletConnectSession,
    transaction: TransactionRequest
  ): Promise<boolean> {
    // TODO: Show approval modal in UI
    return true;
  }

  private async requestSignatureApproval(
    session: WalletConnectSession,
    data: string
  ): Promise<boolean> {
    // TODO: Show approval modal in UI
    return true;
  }

  private async signTransactionInternal(transaction: TransactionRequest): Promise<string> {
    // TODO: Implement actual transaction signing
    return '0x0000000000000000000000000000000000000000000000000000000000000000';
  }

  private async signMessageInternal(message: string): Promise<string> {
    // TODO: Implement actual message signing
    return '0x0000000000000000000000000000000000000000000000000000000000000000';
  }

  private async signTypedDataInternal(typedData: string): Promise<string> {
    // TODO: Implement actual typed data signing
    return '0x0000000000000000000000000000000000000000000000000000000000000000';
  }
}

export const walletConnectService = new WalletConnectService();
