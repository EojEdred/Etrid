import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import { cryptoWaitReady } from '@polkadot/util-crypto';

/**
 * Substrate testing utilities
 */

export class SubstrateHelper {
  api: ApiPromise | null = null;
  keyring: Keyring | null = null;
  account: KeyringPair | null = null;

  constructor(private wsUrl: string, private accountUri: string = '//Alice') {}

  /**
   * Connect to Substrate node
   */
  async connect(): Promise<void> {
    const provider = new WsProvider(this.wsUrl);
    this.api = await ApiPromise.create({ provider });

    await cryptoWaitReady();
    this.keyring = new Keyring({ type: 'sr25519' });
    this.account = this.keyring.addFromUri(this.accountUri);

    console.log(`Connected to Substrate: ${await this.getChainName()}`);
    console.log(`Account: ${this.account.address}`);
  }

  /**
   * Get chain name
   */
  async getChainName(): Promise<string> {
    if (!this.api) throw new Error('Not connected');
    const chain = await this.api.rpc.system.chain();
    return chain.toString();
  }

  /**
   * Get EDSC balance
   */
  async getBalance(address: string): Promise<bigint> {
    if (!this.api) throw new Error('Not connected');

    // Query balance from assets pallet (EDSC asset)
    const balance = await this.api.query.assets?.account(0, address); // Asset ID 0 = EDSC

    if (balance && balance.isSome) {
      const accountData: any = balance.unwrap();
      return BigInt(accountData.balance.toString());
    }

    return 0n;
  }

  /**
   * Mint EDSC tokens (for testing)
   */
  async mintEDSC(to: string, amount: bigint): Promise<string> {
    if (!this.api || !this.account) throw new Error('Not connected');

    return new Promise((resolve, reject) => {
      this.api!.tx.assets
        ?.mint(0, to, amount.toString()) // Asset ID 0 = EDSC
        .signAndSend(this.account!, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              resolve(status.asInBlock.toString());
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Burn EDSC and send to Ethereum
   */
  async burnAndSend(
    recipientEthereumAddress: string,
    amount: bigint,
    destinationDomain: number = 0
  ): Promise<{ nonce: bigint; blockHash: string }> {
    if (!this.api || !this.account) throw new Error('Not connected');

    // Encode recipient as bytes (Ethereum address is 20 bytes)
    const recipientBytes = this.encodeEthereumAddress(recipientEthereumAddress);

    return new Promise((resolve, reject) => {
      let nonce: bigint | null = null;

      this.api!.tx.tokenMessenger
        ?.burnAndSend(destinationDomain, recipientBytes, amount.toString())
        .signAndSend(this.account!, ({ status, events, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              // Find BurnMessageSent event
              for (const record of events) {
                const { event } = record;
                if (
                  event.section === 'tokenMessenger' &&
                  event.method === 'BurnMessageSent'
                ) {
                  const [, eventNonce] = event.data;
                  nonce = BigInt(eventNonce.toString());
                  break;
                }
              }

              if (nonce === null) {
                reject(new Error('BurnMessageSent event not found'));
              } else {
                resolve({
                  nonce,
                  blockHash: status.asInBlock.toString(),
                });
              }
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Receive message from Ethereum
   */
  async receiveMessage(
    message: Uint8Array,
    signatures: Uint8Array[]
  ): Promise<{ success: boolean; blockHash: string }> {
    if (!this.api || !this.account) throw new Error('Not connected');

    // Convert Uint8Array to Vec<u8> format for Substrate
    const messageVec = Array.from(message);
    const signatureVecs = signatures.map((sig) => Array.from(sig));

    return new Promise((resolve, reject) => {
      this.api!.tx.attestation
        ?.receiveMessage(messageVec, signatureVecs)
        .signAndSend(this.account!, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              resolve({
                success: true,
                blockHash: status.asInBlock.toString(),
              });
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Check if message already received
   */
  async isMessageReceived(messageHash: string): Promise<boolean> {
    if (!this.api) throw new Error('Not connected');

    const isReceived = await this.api.query.attestation?.receivedMessages(messageHash);
    return isReceived ? isReceived.toJSON() === true : false;
  }

  /**
   * Add attester to registry
   */
  async addAttester(attesterAddress: string): Promise<string> {
    if (!this.api || !this.account) throw new Error('Not connected');

    return new Promise((resolve, reject) => {
      this.api!.tx.attestation
        ?.addAttester(attesterAddress)
        .signAndSend(this.account!, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              resolve(status.asInBlock.toString());
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Set signature threshold
   */
  async setThreshold(domain: number, threshold: number): Promise<string> {
    if (!this.api || !this.account) throw new Error('Not connected');

    return new Promise((resolve, reject) => {
      this.api!.tx.attestation
        ?.setThreshold(domain, threshold)
        .signAndSend(this.account!, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              resolve(status.asInBlock.toString());
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Get current block number
   */
  async getBlockNumber(): Promise<number> {
    if (!this.api) throw new Error('Not connected');
    const header = await this.api.rpc.chain.getHeader();
    return header.number.toNumber();
  }

  /**
   * Wait for blocks
   */
  async waitBlocks(count: number): Promise<void> {
    if (!this.api) throw new Error('Not connected');

    const startBlock = await this.getBlockNumber();
    while ((await this.getBlockNumber()) < startBlock + count) {
      await new Promise((resolve) => setTimeout(resolve, 2000));
    }
  }

  /**
   * Get free balance (native token)
   */
  async getFreeBalance(address: string): Promise<bigint> {
    if (!this.api) throw new Error('Not connected');

    const accountInfo: any = await this.api.query.system.account(address);
    return BigInt(accountInfo.data.free.toString());
  }

  /**
   * Transfer native tokens
   */
  async transfer(to: string, amount: bigint): Promise<string> {
    if (!this.api || !this.account) throw new Error('Not connected');

    return new Promise((resolve, reject) => {
      this.api!.tx.balances
        .transfer(to, amount.toString())
        .signAndSend(this.account!, ({ status, dispatchError }) => {
          if (status.isInBlock) {
            if (dispatchError) {
              const error = this.decodeError(dispatchError);
              reject(new Error(error));
            } else {
              resolve(status.asInBlock.toString());
            }
          }
        })
        .catch(reject);
    });
  }

  /**
   * Encode Ethereum address as bytes
   */
  private encodeEthereumAddress(address: string): Uint8Array {
    // Remove 0x prefix if present
    const cleanAddress = address.startsWith('0x') ? address.slice(2) : address;

    // Convert hex to bytes
    const bytes = new Uint8Array(20);
    for (let i = 0; i < 20; i++) {
      bytes[i] = parseInt(cleanAddress.slice(i * 2, i * 2 + 2), 16);
    }

    return bytes;
  }

  /**
   * Decode dispatch error
   */
  private decodeError(dispatchError: any): string {
    if (!this.api) return 'Unknown error';

    if (dispatchError.isModule) {
      const decoded = this.api.registry.findMetaError(dispatchError.asModule);
      return `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`;
    } else {
      return dispatchError.toString();
    }
  }

  /**
   * Disconnect
   */
  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.api = null;
    }
  }
}
