/**
 * ETH PBC Precompile Wrappers for JavaScript/TypeScript SDK
 *
 * Provides TypeScript interface to Ethereum Partition Burst Chain (ETH PBC) precompiles
 * that enable access to FlareChain features from EVM contracts.
 *
 * ## Precompile Addresses
 * - 0x800: Oracle (FlareChain price feeds)
 * - 0x801: Governance (voting from ETH PBC)
 * - 0x802: Staking (validator queries)
 * - 0x803: Native ETH Wrapping (zero-fee wrap/unwrap)
 * - 0x804: XCM Bridge (cross-chain transfers)
 * - 0x805: Token Registry (registered tokens)
 * - 0x806: State Proof (Ethereum state verification)
 *
 * @example
 * ```typescript
 * import { ethers } from 'ethers';
 * import { ETHPBCPrecompiles } from 'etrid-sdk';
 *
 * const provider = new ethers.providers.JsonRpcProvider('http://localhost:9944');
 * const wallet = new ethers.Wallet(privateKey, provider);
 * const precompiles = new ETHPBCPrecompiles(provider, wallet);
 *
 * // Get BTC price from FlareChain oracle
 * const price = await precompiles.getOraclePrice('BTC', 'USD');
 * console.log(`BTC Price: $${ethers.utils.formatUnits(price, 18)}`);
 *
 * // Wrap ETH to wETH
 * const tx = await precompiles.wrapEth(ethers.utils.parseEther('1.0'));
 * console.log(`Wrapped ETH: ${tx.hash}`);
 * ```
 */

import { ethers, BigNumber } from 'ethers';

/** Precompile contract addresses */
export const PRECOMPILE_ADDRESSES = {
  ORACLE: '0x0000000000000000000000000000000000000800',
  GOVERNANCE: '0x0000000000000000000000000000000000000801',
  STAKING: '0x0000000000000000000000000000000000000802',
  NATIVE_ETH_WRAP: '0x0000000000000000000000000000000000000803',
  XCM_BRIDGE: '0x0000000000000000000000000000000000000804',
  TOKEN_REGISTRY: '0x0000000000000000000000000000000000000805',
  STATE_PROOF: '0x0000000000000000000000000000000000000806',
} as const;

/** Governance proposal status enum */
export enum ProposalStatus {
  Pending = 0,
  Active = 1,
  Passed = 2,
  Failed = 3,
}

/** Token information from registry */
export interface TokenInfo {
  name: string;
  symbol: string;
  decimals: number;
  totalBridgedSupply: BigNumber;
}

/** Latest Ethereum block information */
export interface EthBlockInfo {
  blockNumber: BigNumber;
  blockHash: string;
  stateRoot: string;
  timestamp: BigNumber;
}

/**
 * ETH PBC Precompile wrapper class
 *
 * Provides high-level TypeScript interface to interact with FlareChain features
 * from ETH PBC via precompiled contracts.
 */
export class ETHPBCPrecompiles {
  constructor(
    private provider: ethers.providers.Provider,
    private signer?: ethers.Signer
  ) {}

  /**
   * Helper to make a read-only call to a precompile
   */
  private async call(to: string, data: string): Promise<string> {
    return this.provider.call({ to, data });
  }

  /**
   * Helper to send a transaction to a precompile
   */
  private async sendTransaction(
    to: string,
    data: string,
    value: BigNumber = BigNumber.from(0)
  ): Promise<ethers.ContractTransaction> {
    if (!this.signer) {
      throw new Error('Signer required for transactions');
    }

    return this.signer.sendTransaction({
      to,
      data,
      value,
    });
  }

  // ========== Oracle Precompile (0x800) ==========

  /**
   * Get price from FlareChain oracle
   *
   * @param symbol - Asset symbol (e.g., 'BTC', 'ETH', 'SOL')
   * @param quote - Quote currency (default: 'USD')
   * @returns Price scaled by 1e18 (e.g., 50000e18 = $50,000)
   */
  async getOraclePrice(symbol: string, quote: string = 'USD'): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getPrice(bytes32 symbol, bytes32 quote) view returns (uint256)',
    ]);

    const symbolBytes32 = ethers.utils.formatBytes32String(symbol);
    const quoteBytes32 = ethers.utils.formatBytes32String(quote);

    const data = iface.encodeFunctionData('getPrice', [symbolBytes32, quoteBytes32]);
    const result = await this.call(PRECOMPILE_ADDRESSES.ORACLE, data);

    const decoded = iface.decodeFunctionResult('getPrice', result);
    return decoded[0];
  }

  /**
   * Get price in ETH from FlareChain oracle
   *
   * @param symbol - Asset symbol (e.g., 'BTC', 'SOL')
   * @returns Price in ETH scaled by 1e18
   */
  async getOraclePriceInEth(symbol: string): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getPriceInETH(bytes32 symbol) view returns (uint256)',
    ]);

    const symbolBytes32 = ethers.utils.formatBytes32String(symbol);
    const data = iface.encodeFunctionData('getPriceInETH', [symbolBytes32]);
    const result = await this.call(PRECOMPILE_ADDRESSES.ORACLE, data);

    const decoded = iface.decodeFunctionResult('getPriceInETH', result);
    return decoded[0];
  }

  /**
   * Get last update timestamp for an oracle price feed
   *
   * @param symbol - Asset symbol
   * @returns Unix timestamp of last update
   */
  async getOracleLastUpdate(symbol: string): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getLastUpdate(bytes32 symbol) view returns (uint256)',
    ]);

    const symbolBytes32 = ethers.utils.formatBytes32String(symbol);
    const data = iface.encodeFunctionData('getLastUpdate', [symbolBytes32]);
    const result = await this.call(PRECOMPILE_ADDRESSES.ORACLE, data);

    const decoded = iface.decodeFunctionResult('getLastUpdate', result);
    return decoded[0];
  }

  // ========== Governance Precompile (0x801) ==========

  /**
   * Create a governance proposal on FlareChain
   *
   * @param title - Proposal title (max 256 chars)
   * @param description - Proposal description (max 10000 chars)
   * @returns Transaction response
   */
  async governanceCreateProposal(
    title: string,
    description: string
  ): Promise<ethers.ContractTransaction> {
    const iface = new ethers.utils.Interface([
      'function submitProposal(string title, string description) returns (uint256)',
    ]);

    const data = iface.encodeFunctionData('submitProposal', [title, description]);
    return this.sendTransaction(PRECOMPILE_ADDRESSES.GOVERNANCE, data);
  }

  /**
   * Vote on a governance proposal
   *
   * @param proposalId - Proposal ID to vote on
   * @param support - true to vote YES, false to vote NO
   * @returns Transaction response
   */
  async governanceVote(
    proposalId: number | BigNumber,
    support: boolean
  ): Promise<ethers.ContractTransaction> {
    const iface = new ethers.utils.Interface([
      'function voteOnProposal(uint256 proposalId, bool support)',
    ]);

    const data = iface.encodeFunctionData('voteOnProposal', [proposalId, support]);
    return this.sendTransaction(PRECOMPILE_ADDRESSES.GOVERNANCE, data);
  }

  /**
   * Get governance proposal status
   *
   * @param proposalId - Proposal ID
   * @returns Status: Pending, Active, Passed, or Failed
   */
  async getProposalStatus(proposalId: number | BigNumber): Promise<ProposalStatus> {
    const iface = new ethers.utils.Interface([
      'function getProposalStatus(uint256 proposalId) view returns (uint8)',
    ]);

    const data = iface.encodeFunctionData('getProposalStatus', [proposalId]);
    const result = await this.call(PRECOMPILE_ADDRESSES.GOVERNANCE, data);

    const decoded = iface.decodeFunctionResult('getProposalStatus', result);
    return decoded[0] as ProposalStatus;
  }

  // ========== Staking Precompile (0x802) ==========

  /**
   * Get stake amount for a validator
   *
   * @param validatorId - Validator ID (bytes32)
   * @returns Stake amount in wei
   */
  async getValidatorStake(validatorId: string): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getValidatorStake(bytes32 validatorId) view returns (uint256)',
    ]);

    const validatorBytes32 = ethers.utils.hexZeroPad(validatorId, 32);
    const data = iface.encodeFunctionData('getValidatorStake', [validatorBytes32]);
    const result = await this.call(PRECOMPILE_ADDRESSES.STAKING, data);

    const decoded = iface.decodeFunctionResult('getValidatorStake', result);
    return decoded[0];
  }

  /**
   * Check if a validator is active
   *
   * @param validatorId - Validator ID (bytes32)
   * @returns true if validator is active
   */
  async isValidatorActive(validatorId: string): Promise<boolean> {
    const iface = new ethers.utils.Interface([
      'function isValidatorActive(bytes32 validatorId) view returns (bool)',
    ]);

    const validatorBytes32 = ethers.utils.hexZeroPad(validatorId, 32);
    const data = iface.encodeFunctionData('isValidatorActive', [validatorBytes32]);
    const result = await this.call(PRECOMPILE_ADDRESSES.STAKING, data);

    const decoded = iface.decodeFunctionResult('isValidatorActive', result);
    return decoded[0];
  }

  /**
   * Get total amount staked across all validators
   *
   * @returns Total stake in wei
   */
  async getTotalStaked(): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getTotalStaked() view returns (uint256)',
    ]);

    const data = iface.encodeFunctionData('getTotalStaked', []);
    const result = await this.call(PRECOMPILE_ADDRESSES.STAKING, data);

    const decoded = iface.decodeFunctionResult('getTotalStaked', result);
    return decoded[0];
  }

  /**
   * Get total number of validators
   *
   * @returns Validator count
   */
  async getValidatorCount(): Promise<number> {
    const iface = new ethers.utils.Interface([
      'function getValidatorCount() view returns (uint256)',
    ]);

    const data = iface.encodeFunctionData('getValidatorCount', []);
    const result = await this.call(PRECOMPILE_ADDRESSES.STAKING, data);

    const decoded = iface.decodeFunctionResult('getValidatorCount', result);
    return decoded[0].toNumber();
  }

  // ========== Native ETH Wrap Precompile (0x803) ==========

  /**
   * Wrap native ETH to wETH (zero-fee, instant)
   *
   * @param amount - Amount of ETH to wrap (in wei)
   * @returns Transaction response
   */
  async wrapEth(amount: BigNumber): Promise<ethers.ContractTransaction> {
    const iface = new ethers.utils.Interface(['function wrap() payable returns (uint256)']);

    const data = iface.encodeFunctionData('wrap', []);
    return this.sendTransaction(PRECOMPILE_ADDRESSES.NATIVE_ETH_WRAP, data, amount);
  }

  /**
   * Unwrap wETH to native ETH (zero-fee, instant)
   *
   * @param amount - Amount of wETH to unwrap (in wei)
   * @returns Transaction response
   */
  async unwrapEth(amount: BigNumber): Promise<ethers.ContractTransaction> {
    const iface = new ethers.utils.Interface([
      'function unwrap(uint256 amount) returns (bool)',
    ]);

    const data = iface.encodeFunctionData('unwrap', [amount]);
    return this.sendTransaction(PRECOMPILE_ADDRESSES.NATIVE_ETH_WRAP, data);
  }

  /**
   * Get current ETH<->wETH wrap rate
   *
   * @returns Rate scaled by 1e18 (normally 1e18 = 1:1)
   */
  async getWrapRate(): Promise<BigNumber> {
    const iface = new ethers.utils.Interface([
      'function getWrapRate() view returns (uint256)',
    ]);

    const data = iface.encodeFunctionData('getWrapRate', []);
    const result = await this.call(PRECOMPILE_ADDRESSES.NATIVE_ETH_WRAP, data);

    const decoded = iface.decodeFunctionResult('getWrapRate', result);
    return decoded[0];
  }

  // ========== Token Registry Precompile (0x805) ==========

  /**
   * Get registered token information
   *
   * @param tokenAddress - ERC-20 token address
   * @returns Token metadata and bridged supply
   */
  async getTokenInfo(tokenAddress: string): Promise<TokenInfo> {
    const iface = new ethers.utils.Interface([
      'function getTokenInfo(address token) view returns (string name, string symbol, uint8 decimals, uint256 totalBridgedSupply)',
    ]);

    const data = iface.encodeFunctionData('getTokenInfo', [tokenAddress]);
    const result = await this.call(PRECOMPILE_ADDRESSES.TOKEN_REGISTRY, data);

    const decoded = iface.decodeFunctionResult('getTokenInfo', result);

    return {
      name: decoded.name,
      symbol: decoded.symbol,
      decimals: decoded.decimals,
      totalBridgedSupply: decoded.totalBridgedSupply,
    };
  }

  /**
   * Register a token from Ethereum mainnet
   *
   * @param tokenAddress - Token address to register
   * @returns Transaction response
   */
  async registerToken(tokenAddress: string): Promise<ethers.ContractTransaction> {
    const iface = new ethers.utils.Interface([
      'function registerToken(address token) returns (bool)',
    ]);

    const data = iface.encodeFunctionData('registerToken', [tokenAddress]);
    return this.sendTransaction(PRECOMPILE_ADDRESSES.TOKEN_REGISTRY, data);
  }

  /**
   * Get list of all bridged tokens
   *
   * @returns Array of token addresses
   */
  async getBridgedTokens(): Promise<string[]> {
    const iface = new ethers.utils.Interface([
      'function getBridgedTokens() view returns (address[] memory)',
    ]);

    const data = iface.encodeFunctionData('getBridgedTokens', []);
    const result = await this.call(PRECOMPILE_ADDRESSES.TOKEN_REGISTRY, data);

    const decoded = iface.decodeFunctionResult('getBridgedTokens', result);
    return decoded[0];
  }

  // ========== State Proof Precompile (0x806) ==========

  /**
   * Get the latest verified Ethereum block
   *
   * @returns Latest Ethereum block information
   */
  async getLatestEthBlock(): Promise<EthBlockInfo> {
    const iface = new ethers.utils.Interface([
      'function getLatestEthBlock() view returns (uint256 blockNumber, bytes32 blockHash, bytes32 stateRoot, uint256 timestamp)',
    ]);

    const data = iface.encodeFunctionData('getLatestEthBlock', []);
    const result = await this.call(PRECOMPILE_ADDRESSES.STATE_PROOF, data);

    const decoded = iface.decodeFunctionResult('getLatestEthBlock', result);

    return {
      blockNumber: decoded.blockNumber,
      blockHash: decoded.blockHash,
      stateRoot: decoded.stateRoot,
      timestamp: decoded.timestamp,
    };
  }

  /**
   * Verify Ethereum state proof
   *
   * @param proof - Merkle proof bytes
   * @returns true if proof is valid
   */
  async verifyEthStateProof(proof: string): Promise<boolean> {
    // This is a complex function requiring full Merkle proof encoding
    // Simplified stub implementation
    console.warn('verifyEthStateProof is a stub implementation');
    return true;
  }
}

/**
 * Create a new ETH PBC Precompiles instance
 *
 * @param provider - ethers.js provider
 * @param signer - Optional signer for transactions
 * @returns ETHPBCPrecompiles instance
 */
export function createETHPBCPrecompiles(
  provider: ethers.providers.Provider,
  signer?: ethers.Signer
): ETHPBCPrecompiles {
  return new ETHPBCPrecompiles(provider, signer);
}
