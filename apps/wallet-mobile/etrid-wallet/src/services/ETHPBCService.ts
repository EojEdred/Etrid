import EtridSDKService from './EtridSDKService';
import KeychainService from './KeychainService';

export type PrecompileAddress =
  | '0x800' // Oracle
  | '0x801' // Governance
  | '0x802' // Staking
  | '0x803' // ETH Wrap
  | '0x804' // Bridge
  | '0x805' // Token Registry
  | '0x806'; // State Proof

export interface PrecompileCall {
  address: PrecompileAddress;
  name: string;
  description: string;
  parameters: {
    name: string;
    type: string;
    description: string;
  }[];
}

export interface ETHBalance {
  eth: string;
  weth: string;
  ethUSD: string;
  wethUSD: string;
}

/**
 * Service for ETH PBC (Ethereum L2) operations
 * Uses ETHPBCPrecompileWrapper from Ëtrid SDK
 */
class ETHPBCService {
  private sdk: EtridSDKService;

  private precompiles: Record<PrecompileAddress, PrecompileCall> = {
    '0x800': {
      address: '0x800',
      name: 'Oracle',
      description: 'Get price feeds from oracle',
      parameters: [
        {
          name: 'asset',
          type: 'string',
          description: 'Asset symbol (ETH, BTC, EDSC, etc.)',
        },
      ],
    },
    '0x801': {
      address: '0x801',
      name: 'Governance',
      description: 'Vote on governance proposals',
      parameters: [
        {
          name: 'proposalId',
          type: 'number',
          description: 'Proposal ID',
        },
        {
          name: 'vote',
          type: 'boolean',
          description: 'true for Yes, false for No',
        },
      ],
    },
    '0x802': {
      address: '0x802',
      name: 'Staking',
      description: 'Stake tokens for rewards',
      parameters: [
        {
          name: 'amount',
          type: 'string',
          description: 'Amount to stake (in wei)',
        },
      ],
    },
    '0x803': {
      address: '0x803',
      name: 'ETH Wrap',
      description: 'Wrap/unwrap ETH to wETH',
      parameters: [
        {
          name: 'action',
          type: 'string',
          description: 'wrap or unwrap',
        },
        {
          name: 'amount',
          type: 'string',
          description: 'Amount (in wei)',
        },
      ],
    },
    '0x804': {
      address: '0x804',
      name: 'Bridge',
      description: 'Cross-chain bridge operations',
      parameters: [
        {
          name: 'targetChain',
          type: 'string',
          description: 'Target chain ID',
        },
        {
          name: 'amount',
          type: 'string',
          description: 'Amount to bridge',
        },
      ],
    },
    '0x805': {
      address: '0x805',
      name: 'Token Registry',
      description: 'Query registered tokens',
      parameters: [
        {
          name: 'tokenAddress',
          type: 'string',
          description: 'Token contract address',
        },
      ],
    },
    '0x806': {
      address: '0x806',
      name: 'State Proof',
      description: 'Verify Ethereum state proofs',
      parameters: [
        {
          name: 'blockHash',
          type: 'string',
          description: 'Ethereum block hash',
        },
        {
          name: 'proof',
          type: 'string',
          description: 'Merkle proof',
        },
      ],
    },
  };

  constructor() {
    this.sdk = EtridSDKService.getInstance();
  }

  /**
   * Initialize SDK connection
   */
  private async ensureConnected(): Promise<void> {
    if (!this.sdk.isConnected()) {
      await this.sdk.connect();
    }
  }

  /**
   * Get ETH PBC balance
   */
  public async getETHBalance(): Promise<ETHBalance> {
    await this.ensureConnected();

    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        throw new Error('No wallet found');
      }

      // In production, query actual ETH and wETH balances
      // For now, return mock data
      return {
        eth: '1.5',
        weth: '0.5',
        ethUSD: '4500.00',
        wethUSD: '1500.00',
      };
    } catch (error) {
      console.error('Failed to get ETH balance:', error);
      throw error;
    }
  }

  /**
   * Wrap ETH to wETH
   */
  public async wrapETH(amount: string): Promise<string> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.ethPBCPrecompile.wrapETH(keypair, amount);
      return `Wrapped ${amount} ETH to wETH`;
    } catch (error) {
      console.error('Failed to wrap ETH:', error);
      throw error;
    }
  }

  /**
   * Unwrap wETH to ETH
   */
  public async unwrapETH(amount: string): Promise<string> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.ethPBCPrecompile.unwrapETH(keypair, amount);
      return `Unwrapped ${amount} wETH to ETH`;
    } catch (error) {
      console.error('Failed to unwrap ETH:', error);
      throw error;
    }
  }

  /**
   * Get available precompiles
   */
  public getPrecompiles(): PrecompileCall[] {
    return Object.values(this.precompiles);
  }

  /**
   * Get precompile details
   */
  public getPrecompile(address: PrecompileAddress): PrecompileCall {
    return this.precompiles[address];
  }

  /**
   * Call precompile - Oracle (0x800)
   */
  public async getOraclePrice(asset: string): Promise<string> {
    await this.ensureConnected();

    try {
      const price = await this.sdk.ethPBCPrecompile.getOraclePrice(asset);
      return price;
    } catch (error) {
      console.error('Failed to get oracle price:', error);
      throw error;
    }
  }

  /**
   * Call precompile - Governance (0x801)
   */
  public async voteOnProposal(proposalId: number, vote: boolean): Promise<void> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.ethPBCPrecompile.voteOnProposal(keypair, proposalId, vote);
    } catch (error) {
      console.error('Failed to vote on proposal:', error);
      throw error;
    }
  }

  /**
   * Call precompile - Staking (0x802)
   */
  public async stakeTokens(amount: string): Promise<void> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.ethPBCPrecompile.stakeTokens(keypair, amount);
    } catch (error) {
      console.error('Failed to stake tokens:', error);
      throw error;
    }
  }

  /**
   * Call precompile - Bridge (0x804)
   */
  public async bridgeToChain(targetChain: string, amount: string): Promise<string> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      // Encode bridge call
      const data = this.encodeBridgeCall(targetChain, amount);
      const result = await this.sdk.ethPBCPrecompile.callPrecompile(
        keypair,
        '0x0000000000000000000000000000000000000804',
        data
      );

      return result;
    } catch (error) {
      console.error('Failed to bridge to chain:', error);
      throw error;
    }
  }

  /**
   * Call precompile - Token Registry (0x805)
   */
  public async queryToken(tokenAddress: string): Promise<any> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      const data = '0x' + tokenAddress.slice(2);
      const result = await this.sdk.ethPBCPrecompile.callPrecompile(
        keypair,
        '0x0000000000000000000000000000000000000805',
        data
      );

      return result;
    } catch (error) {
      console.error('Failed to query token:', error);
      throw error;
    }
  }

  /**
   * Call precompile - State Proof (0x806)
   */
  public async verifyStateProof(blockHash: string, proof: string): Promise<boolean> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      const data = '0x' + blockHash.slice(2) + proof.slice(2);
      const result = await this.sdk.ethPBCPrecompile.callPrecompile(
        keypair,
        '0x0000000000000000000000000000000000000806',
        data
      );

      return result === '0x01';
    } catch (error) {
      console.error('Failed to verify state proof:', error);
      throw error;
    }
  }

  /**
   * Generic precompile call
   */
  public async callPrecompile(
    address: PrecompileAddress,
    parameters: any[]
  ): Promise<any> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      const precompile = this.precompiles[address];
      const data = this.encodePrecompileCall(precompile, parameters);

      const fullAddress = `0x0000000000000000000000000000000000000${address.slice(2)}`;
      const result = await this.sdk.ethPBCPrecompile.callPrecompile(
        keypair,
        fullAddress,
        data
      );

      return result;
    } catch (error) {
      console.error('Failed to call precompile:', error);
      throw error;
    }
  }

  /**
   * Get precompile call history
   */
  public async getPrecompileCallHistory(): Promise<any[]> {
    // In production, query on-chain events
    // For now, return empty array
    return [];
  }

  // Helper methods
  private encodeBridgeCall(targetChain: string, amount: string): string {
    // Simple encoding: chainId (32 bytes) + amount (32 bytes)
    const chainIdHex = parseInt(targetChain).toString(16).padStart(64, '0');
    const amountHex = BigInt(amount).toString(16).padStart(64, '0');
    return '0x' + chainIdHex + amountHex;
  }

  private encodePrecompileCall(precompile: PrecompileCall, parameters: any[]): string {
    // Simple encoding based on parameter types
    let encoded = '0x';

    parameters.forEach((param, index) => {
      const paramDef = precompile.parameters[index];

      if (paramDef.type === 'number') {
        encoded += param.toString(16).padStart(64, '0');
      } else if (paramDef.type === 'string') {
        encoded += Buffer.from(param).toString('hex').padStart(64, '0');
      } else if (paramDef.type === 'boolean') {
        encoded += param ? '01' : '00';
      }
    });

    return encoded;
  }
}

export default new ETHPBCService();
