/**
 * Phantom Wallet Adapter
 *
 * Integrates Phantom wallet for Solana + Ethereum interactions.
 * Provides bridge UI, staking, and governance voting.
 */

import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { ethers } from 'ethers';

interface PhantomConfig {
  solanaRpcUrl: string;
  ethereumRpcUrl: string;
  bridgeApiUrl: string;
}

interface BridgeRequest {
  fromChain: 'ethereum' | 'solana' | 'etrid';
  toChain: 'ethereum' | 'solana' | 'etrid';
  token: 'ETR' | 'EDSC';
  amount: string;
  destinationAddress: string;
}

class PhantomIntegration {
  private phantomWallet: PhantomWalletAdapter;
  private solanaConnection: Connection;
  private ethereumProvider: ethers.BrowserProvider | null = null;

  constructor(private config: PhantomConfig) {
    this.phantomWallet = new PhantomWalletAdapter();
    this.solanaConnection = new Connection(config.solanaRpcUrl);
  }

  /**
   * Connect Phantom wallet
   */
  async connect(): Promise<{
    solanaAddress: string;
    ethereumAddress?: string;
  }> {
    console.log('🔌 Connecting Phantom wallet...');

    try {
      await this.phantomWallet.connect();
      const solanaAddress = this.phantomWallet.publicKey?.toString() || '';

      console.log(`✅ Solana address: ${solanaAddress}`);

      // Check if Phantom also has Ethereum support
      if ((window as any).ethereum) {
        this.ethereumProvider = new ethers.BrowserProvider((window as any).ethereum);
        const accounts = await this.ethereumProvider.send('eth_requestAccounts', []);
        const ethereumAddress = accounts[0];

        console.log(`✅ Ethereum address: ${ethereumAddress}`);

        return { solanaAddress, ethereumAddress };
      }

      return { solanaAddress };

    } catch (error) {
      console.error('❌ Connection failed:', error);
      throw error;
    }
  }

  /**
   * Disconnect wallet
   */
  async disconnect() {
    await this.phantomWallet.disconnect();
    console.log('👋 Disconnected from Phantom');
  }

  /**
   * Bridge tokens between chains
   */
  async bridgeTokens(request: BridgeRequest): Promise<string> {
    console.log(`🌉 Bridging ${request.amount} ${request.token} from ${request.fromChain} to ${request.toChain}...`);

    try {
      // Step 1: Lock/burn tokens on source chain
      let sourceTxHash: string;

      if (request.fromChain === 'solana') {
        sourceTxHash = await this.burnOnSolana(request.token, request.amount);
      } else if (request.fromChain === 'ethereum') {
        sourceTxHash = await this.burnOnEthereum(request.token, request.amount, request.destinationAddress);
      } else {
        throw new Error('Ëtrid to external bridge not yet implemented');
      }

      console.log(`✅ Source tx: ${sourceTxHash}`);

      // Step 2: Submit bridge request to relayer
      const response = await fetch(`${this.config.bridgeApiUrl}/bridge`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ...request,
          sourceTxHash
        })
      });

      if (!response.ok) {
        throw new Error(`Bridge API error: ${response.statusText}`);
      }

      const data = await response.json();
      console.log(`✅ Bridge request submitted: ${data.requestId}`);

      // Step 3: Poll for completion
      return await this.waitForBridgeCompletion(data.requestId);

    } catch (error) {
      console.error('❌ Bridge failed:', error);
      throw error;
    }
  }

  /**
   * Burn tokens on Solana
   */
  private async burnOnSolana(token: string, amount: string): Promise<string> {
    if (!this.phantomWallet.publicKey) {
      throw new Error('Wallet not connected');
    }

    // This would use @solana/spl-token to burn
    // Simplified for demo
    const transaction = new Transaction();
    // Add burn instruction here

    const signature = await this.phantomWallet.sendTransaction(
      transaction,
      this.solanaConnection
    );

    await this.solanaConnection.confirmTransaction(signature);
    return signature;
  }

  /**
   * Burn tokens on Ethereum
   */
  private async burnOnEthereum(
    token: string,
    amount: string,
    etridAddress: string
  ): Promise<string> {
    if (!this.ethereumProvider) {
      throw new Error('Ethereum not available in Phantom');
    }

    const signer = await this.ethereumProvider.getSigner();

    // Load token contract
    const tokenAddress = token === 'ETR'
      ? process.env.NEXT_PUBLIC_ETR_TOKEN_ADDRESS
      : process.env.NEXT_PUBLIC_EDSC_TOKEN_ADDRESS;

    const tokenAbi = [
      "function bridgeBurn(uint256 amount, string calldata etridAddress) external"
    ];

    const tokenContract = new ethers.Contract(tokenAddress!, tokenAbi, signer);

    const tx = await tokenContract.bridgeBurn(
      ethers.parseEther(amount),
      etridAddress
    );

    const receipt = await tx.wait();
    return receipt.hash;
  }

  /**
   * Wait for bridge completion
   */
  private async waitForBridgeCompletion(requestId: string): Promise<string> {
    console.log('⏳ Waiting for bridge completion...');

    let attempts = 0;
    const maxAttempts = 60; // 5 minutes (5s intervals)

    while (attempts < maxAttempts) {
      const response = await fetch(`${this.config.bridgeApiUrl}/bridge/${requestId}`);

      if (!response.ok) {
        throw new Error(`Bridge status check failed: ${response.statusText}`);
      }

      const data = await response.json();

      if (data.status === 'completed') {
        console.log(`✅ Bridge completed! Destination tx: ${data.destinationTxHash}`);
        return data.destinationTxHash;
      }

      if (data.status === 'failed') {
        throw new Error(`Bridge failed: ${data.error}`);
      }

      // Wait 5 seconds before next poll
      await new Promise(resolve => setTimeout(resolve, 5000));
      attempts++;
    }

    throw new Error('Bridge timeout');
  }

  /**
   * Stake ÉTR tokens
   */
  async stakeETR(amount: string): Promise<string> {
    console.log(`🥩 Staking ${amount} ÉTR...`);

    // This would interact with staking contract
    // Simplified for demo
    if (!this.ethereumProvider) {
      throw new Error('Ethereum not available');
    }

    const signer = await this.ethereumProvider.getSigner();

    const stakingAbi = [
      "function stake(uint256 amount) external"
    ];

    const stakingContract = new ethers.Contract(
      process.env.NEXT_PUBLIC_STAKING_ADDRESS!,
      stakingAbi,
      signer
    );

    const tx = await stakingContract.stake(ethers.parseEther(amount));
    const receipt = await tx.wait();

    console.log(`✅ Staked! TX: ${receipt.hash}`);
    return receipt.hash;
  }

  /**
   * Vote on governance proposal
   */
  async vote(proposalId: string, supportVote: boolean): Promise<string> {
    console.log(`🗳️  Voting ${supportVote ? 'YES' : 'NO'} on proposal ${proposalId}...`);

    if (!this.ethereumProvider) {
      throw new Error('Ethereum not available');
    }

    const signer = await this.ethereumProvider.getSigner();

    const governanceAbi = [
      "function castVote(uint256 proposalId, uint8 support) external"
    ];

    const governanceContract = new ethers.Contract(
      process.env.NEXT_PUBLIC_GOVERNANCE_ADDRESS!,
      governanceAbi,
      signer
    );

    const tx = await governanceContract.castVote(proposalId, supportVote ? 1 : 0);
    const receipt = await tx.wait();

    console.log(`✅ Vote cast! TX: ${receipt.hash}`);
    return receipt.hash;
  }

  /**
   * Get wallet balances
   */
  async getBalances(): Promise<{
    solana?: { ETR: string; EDSC: string };
    ethereum?: { ETR: string; EDSC: string };
  }> {
    const balances: any = {};

    // Solana balances
    if (this.phantomWallet.publicKey) {
      // This would query SPL token accounts
      balances.solana = {
        ETR: '0',
        EDSC: '0'
      };
    }

    // Ethereum balances
    if (this.ethereumProvider) {
      const signer = await this.ethereumProvider.getSigner();
      const address = await signer.getAddress();

      const tokenAbi = ["function balanceOf(address) view returns (uint256)"];

      const etrContract = new ethers.Contract(
        process.env.NEXT_PUBLIC_ETR_TOKEN_ADDRESS!,
        tokenAbi,
        this.ethereumProvider
      );

      const edscContract = new ethers.Contract(
        process.env.NEXT_PUBLIC_EDSC_TOKEN_ADDRESS!,
        tokenAbi,
        this.ethereumProvider
      );

      const etrBalance = await etrContract.balanceOf(address);
      const edscBalance = await edscContract.balanceOf(address);

      balances.ethereum = {
        ETR: ethers.formatEther(etrBalance),
        EDSC: ethers.formatEther(edscBalance)
      };
    }

    return balances;
  }
}

export default PhantomIntegration;
