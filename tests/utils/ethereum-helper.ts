import { ethers } from 'ethers';

/**
 * Ethereum testing utilities
 */

export interface EthereumContracts {
  edsc: ethers.Contract;
  attesterRegistry: ethers.Contract;
  messageTransmitter: ethers.Contract;
  tokenMessenger: ethers.Contract;
}

export class EthereumHelper {
  provider: ethers.JsonRpcProvider;
  wallet: ethers.Wallet;
  contracts?: EthereumContracts;

  constructor(rpcUrl: string, privateKey: string) {
    this.provider = new ethers.JsonRpcProvider(rpcUrl);
    this.wallet = new ethers.Wallet(privateKey, this.provider);
  }

  /**
   * Connect to deployed contracts
   */
  async connectContracts(addresses: {
    edsc: string;
    attesterRegistry: string;
    messageTransmitter: string;
    tokenMessenger: string;
  }): Promise<void> {
    // EDSC Token ABI (minimal for testing)
    const edscAbi = [
      'function balanceOf(address account) external view returns (uint256)',
      'function approve(address spender, uint256 amount) external returns (bool)',
      'function mint(address to, uint256 amount) external',
      'function burn(address from, uint256 amount) external',
      'event Transfer(address indexed from, address indexed to, uint256 value)',
    ];

    // AttesterRegistry ABI
    const attesterRegistryAbi = [
      'function addAttester(address attester) external',
      'function removeAttester(address attester) external',
      'function setThreshold(uint32 domain, uint256 threshold) external',
      'function getThreshold(uint32 domain) external view returns (uint256)',
      'function isAttester(address account) external view returns (bool)',
      'function getAttesterCount() external view returns (uint256)',
    ];

    // MessageTransmitter ABI
    const messageTransmitterAbi = [
      'function receiveMessage(bytes calldata message, bytes[] calldata signatures) external returns (bool)',
      'function isMessageReceived(bytes32 messageHash) external view returns (bool)',
      'function usedNonces(uint32 sourceDomain, uint64 nonce) external view returns (bool)',
      'event MessageReceived(bytes32 indexed messageHash, uint32 indexed sourceDomain, uint64 indexed nonce, address recipient, uint256 amount)',
    ];

    // TokenMessenger ABI
    const tokenMessengerAbi = [
      'function burnAndSend(bytes calldata recipient, uint256 amount) external returns (uint64)',
      'function getOutboundMessage(uint64 nonce) external view returns (bytes memory)',
      'function setRateLimit(uint256 maxAmount, uint256 windowSize) external',
      'event MessageSent(uint32 indexed destinationDomain, uint64 indexed nonce, address indexed sender, bytes recipient, uint256 amount)',
    ];

    this.contracts = {
      edsc: new ethers.Contract(addresses.edsc, edscAbi, this.wallet),
      attesterRegistry: new ethers.Contract(
        addresses.attesterRegistry,
        attesterRegistryAbi,
        this.wallet
      ),
      messageTransmitter: new ethers.Contract(
        addresses.messageTransmitter,
        messageTransmitterAbi,
        this.wallet
      ),
      tokenMessenger: new ethers.Contract(
        addresses.tokenMessenger,
        tokenMessengerAbi,
        this.wallet
      ),
    };
  }

  /**
   * Get EDSC balance
   */
  async getBalance(address: string): Promise<bigint> {
    if (!this.contracts) throw new Error('Contracts not connected');
    return await this.contracts.edsc.balanceOf(address);
  }

  /**
   * Mint EDSC tokens (for testing)
   */
  async mintEDSC(to: string, amount: bigint): Promise<void> {
    if (!this.contracts) throw new Error('Contracts not connected');
    const tx = await this.contracts.edsc.mint(to, amount);
    await tx.wait();
  }

  /**
   * Approve TokenMessenger to spend EDSC
   */
  async approveTokenMessenger(amount: bigint): Promise<void> {
    if (!this.contracts) throw new Error('Contracts not connected');
    const tx = await this.contracts.edsc.approve(
      await this.contracts.tokenMessenger.getAddress(),
      amount
    );
    await tx.wait();
  }

  /**
   * Burn EDSC and send to Ëtrid
   */
  async burnAndSend(
    recipientSubstrateAddress: string,
    amount: bigint
  ): Promise<{ nonce: bigint; txHash: string }> {
    if (!this.contracts) throw new Error('Contracts not connected');

    // Encode recipient as bytes (Substrate address)
    const recipientBytes = ethers.toUtf8Bytes(recipientSubstrateAddress.padEnd(32, '\0'));

    const tx = await this.contracts.tokenMessenger.burnAndSend(recipientBytes, amount);
    const receipt = await tx.wait();

    // Find MessageSent event
    const event = receipt?.logs
      .map((log: any) => {
        try {
          return this.contracts!.tokenMessenger.interface.parseLog(log);
        } catch {
          return null;
        }
      })
      .find((e: any) => e?.name === 'MessageSent');

    if (!event) {
      throw new Error('MessageSent event not found');
    }

    return {
      nonce: event.args.nonce,
      txHash: receipt!.hash,
    };
  }

  /**
   * Receive message from Ëtrid
   */
  async receiveMessage(
    message: Uint8Array,
    signatures: string[]
  ): Promise<{ success: boolean; txHash: string }> {
    if (!this.contracts) throw new Error('Contracts not connected');

    const tx = await this.contracts.messageTransmitter.receiveMessage(message, signatures);
    const receipt = await tx.wait();

    return {
      success: receipt?.status === 1,
      txHash: receipt!.hash,
    };
  }

  /**
   * Check if message already received
   */
  async isMessageReceived(messageHash: string): Promise<boolean> {
    if (!this.contracts) throw new Error('Contracts not connected');
    return await this.contracts.messageTransmitter.isMessageReceived(messageHash);
  }

  /**
   * Add attester to registry
   */
  async addAttester(attesterAddress: string): Promise<void> {
    if (!this.contracts) throw new Error('Contracts not connected');
    const tx = await this.contracts.attesterRegistry.addAttester(attesterAddress);
    await tx.wait();
  }

  /**
   * Set signature threshold
   */
  async setThreshold(domain: number, threshold: number): Promise<void> {
    if (!this.contracts) throw new Error('Contracts not connected');
    const tx = await this.contracts.attesterRegistry.setThreshold(domain, threshold);
    await tx.wait();
  }

  /**
   * Get current block number
   */
  async getBlockNumber(): Promise<number> {
    return await this.provider.getBlockNumber();
  }

  /**
   * Wait for blocks
   */
  async waitBlocks(count: number): Promise<void> {
    const startBlock = await this.getBlockNumber();
    while ((await this.getBlockNumber()) < startBlock + count) {
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }

  /**
   * Get ETH balance
   */
  async getEthBalance(address: string): Promise<bigint> {
    return await this.provider.getBalance(address);
  }

  /**
   * Send ETH
   */
  async sendEth(to: string, amount: bigint): Promise<string> {
    const tx = await this.wallet.sendTransaction({
      to,
      value: amount,
    });
    const receipt = await tx.wait();
    return receipt!.hash;
  }

  /**
   * Disconnect
   */
  disconnect(): void {
    this.provider.destroy();
  }
}
