import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * NFT ID type
 */
export type NftId = number;

/**
 * GPU ID type
 */
export type GpuId = number;

/**
 * Balance type (in ËDSC smallest units)
 */
export type Balance = bigint;

/**
 * Timestamp (Unix timestamp in seconds)
 */
export type Timestamp = number;

/**
 * GPU NFT metadata
 */
export interface GpuNFT {
  /** NFT ID */
  nftId: NftId;
  /** Current owner address */
  owner: string;
  /** Associated GPU ID */
  gpuId: GpuId;
  /** Reputation score at mint time (0-10000) */
  reputationSnapshot: number;
  /** Total earnings accumulated */
  totalEarnings: Balance;
  /** Whether listed for sale */
  isListed: boolean;
  /** Sale price if listed */
  listPrice: Balance;
  /** Mint timestamp */
  mintedAt: Timestamp;
}

/**
 * Helper functions for GpuNFT
 */
export const GpuNFTHelpers = {
  /** Get reputation as score out of 100 */
  reputationScore(nft: GpuNFT): number {
    return nft.reputationSnapshot / 100;
  },
};

/**
 * NFT ownership history record
 */
export interface OwnershipRecord {
  /** Previous owner address */
  previousOwner: string;
  /** New owner address */
  newOwner: string;
  /** Sale price */
  price: Balance;
  /** Transfer timestamp */
  timestamp: Timestamp;
}

/**
 * GPU rental terms configuration
 */
export interface RentalTerms {
  /** Price per hour in ËDSC smallest units */
  hourlyRate: Balance;
  /** Minimum rental duration in hours */
  minimumHours: number;
  /** Maximum rental duration in hours */
  maximumHours: number;
  /** Security deposit required */
  depositRequired: Balance;
  /** Auto-renew after expiration */
  autoRenew: boolean;
}

/**
 * Helper functions for RentalTerms
 */
export const RentalTermsHelpers = {
  /** Calculate total cost for given duration */
  calculateCost(terms: RentalTerms, hours: number): Balance {
    const computeCost = terms.hourlyRate * BigInt(hours);
    return computeCost + terms.depositRequired;
  },
};

/**
 * Active rental information
 */
export interface RentalInfo {
  /** Rental ID */
  rentalId: number;
  /** Renter address */
  renter: string;
  /** NFT ID being rented */
  nftId: NftId;
  /** Rental start time */
  startTime: Timestamp;
  /** Rental end time */
  endTime: Timestamp;
  /** Hourly rate paid */
  hourlyRate: Balance;
  /** Deposit held */
  deposit: Balance;
  /** Whether rental is active */
  isActive: boolean;
}

/**
 * GPU NFT errors
 */
export class GPUNFTError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'GPUNFTError';
  }
}

export class NFTNotFoundError extends GPUNFTError {
  constructor(nftId: NftId) {
    super(`NFT ${nftId} not found`);
    this.name = 'NFTNotFoundError';
  }
}

export class NotNFTOwnerError extends GPUNFTError {
  constructor(nftId: NftId) {
    super(`Not the owner of NFT ${nftId}`);
    this.name = 'NotNFTOwnerError';
  }
}

export class NFTNotListedError extends GPUNFTError {
  constructor(nftId: NftId) {
    super(`NFT ${nftId} is not listed for sale`);
    this.name = 'NFTNotListedError';
  }
}

/**
 * GPUNFTWrapper
 *
 * Wrapper for pallet-gpu-nft - Tradeable GPU certificates.
 *
 * Provides NFT-based GPU ownership, marketplace trading, and rental capabilities.
 *
 * **Features:**
 * - Mint GPUs as tradeable NFTs
 * - Transfer GPU ownership
 * - List/buy GPUs on marketplace
 * - Track ownership history and provenance
 * - Configure rental terms and pricing
 * - Rent GPUs for compute jobs
 *
 * @example
 * ```typescript
 * import { ApiPromise } from '@polkadot/api';
 * import { GPUNFTWrapper, RentalTerms } from '@etrid/sdk';
 *
 * const api = await ApiPromise.create({ provider });
 * const gpuNft = new GPUNFTWrapper(api);
 *
 * // Mint GPU as NFT
 * const nftId = await gpuNft.mintGpuNft(keypair, gpuId);
 *
 * // List for sale
 * await gpuNft.listForSale(keypair, nftId, 1000n * 10n**18n);
 *
 * // Configure rental
 * const terms: RentalTerms = {
 *   hourlyRate: 10n * 10n**18n,
 *   minimumHours: 1,
 *   maximumHours: 720,
 *   depositRequired: 100n * 10n**18n,
 *   autoRenew: false
 * };
 * await gpuNft.setRentalTerms(keypair, nftId, terms);
 *
 * // Rent GPU
 * const rentalId = await gpuNft.rentGpu(keypair, nftId, 24);
 * ```
 */
export class GPUNFTWrapper {
  private readonly pallet = 'gpuNft';

  /**
   * Create GPU NFT wrapper
   * @param api - Connected Polkadot.js API instance
   */
  constructor(private api: ApiPromise) {}

  /**
   * Mint GPU as NFT
   *
   * Creates an NFT representing ownership of a GPU.
   *
   * @param keypair - Owner's keypair
   * @param gpuId - GPU ID to mint as NFT
   * @returns NFT ID of newly minted certificate
   */
  async mintGpuNft(keypair: KeyringPair, gpuId: GpuId): Promise<NftId> {
    try {
      const tx = this.api.tx[this.pallet].mintNft(gpuId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ events, status }) => {
          if (status.isInBlock || status.isFinalized) {
            events.forEach(({ event }) => {
              if (this.api.events[this.pallet].NFTMinted.is(event)) {
                const nftId = (event.data[0] as any).toNumber();
                resolve(nftId);
              }
            });

            reject(new GPUNFTError('NFT minting failed: no event found'));
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to mint GPU NFT: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Transfer GPU NFT to another account
   *
   * @param keypair - Current owner's keypair
   * @param nftId - NFT ID to transfer
   * @param to - Recipient address
   * @returns True if successful
   */
  async transferGpuNft(keypair: KeyringPair, nftId: NftId, to: string): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].transferNft(nftId, to);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to transfer NFT: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * List GPU NFT for sale on marketplace
   *
   * @param keypair - Owner's keypair
   * @param nftId - NFT ID to list
   * @param price - Sale price in ËDSC
   * @returns True if successful
   */
  async listForSale(keypair: KeyringPair, nftId: NftId, price: Balance): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].listNft(nftId, price.toString());

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to list NFT: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Purchase GPU NFT from marketplace
   *
   * @param keypair - Buyer's keypair
   * @param nftId - NFT ID to purchase
   * @returns True if successful
   */
  async buyGpuNft(keypair: KeyringPair, nftId: NftId): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].buyNft(nftId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      if (error instanceof Error && error.message.includes('NotListed')) {
        throw new NFTNotListedError(nftId);
      }
      throw new GPUNFTError(`Failed to buy NFT: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Query NFT metadata and details
   *
   * @param nftId - NFT ID
   * @returns Complete NFT information
   */
  async getNftMetadata(nftId: NftId): Promise<GpuNFT> {
    try {
      const result = await this.api.query[this.pallet].gpuNFTs(nftId);

      if (result.isNone) {
        throw new NFTNotFoundError(nftId);
      }

      const data = result.unwrap();
      return {
        nftId,
        owner: data.owner.toString(),
        gpuId: data.gpuId.toNumber(),
        reputationSnapshot: data.reputationSnapshot.toNumber(),
        totalEarnings: BigInt(data.totalEarnings.toString()),
        isListed: data.isListed.valueOf(),
        listPrice: BigInt(data.listPrice.toString()),
        mintedAt: data.mintedAt.toNumber(),
      };
    } catch (error) {
      if (error instanceof NFTNotFoundError) {
        throw error;
      }
      throw new GPUNFTError(`Failed to query NFT metadata: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Track NFT ownership provenance
   *
   * @param nftId - NFT ID
   * @returns List of ownership records (chronological)
   */
  async getOwnershipHistory(nftId: NftId): Promise<OwnershipRecord[]> {
    try {
      const result = await this.api.query[this.pallet].ownershipHistory(nftId);

      if (result.isNone) {
        return [];
      }

      const records = result.unwrap();
      return records.map((record: any) => ({
        previousOwner: record.previousOwner.toString(),
        newOwner: record.newOwner.toString(),
        price: BigInt(record.price.toString()),
        timestamp: record.timestamp.toNumber(),
      }));
    } catch (error) {
      throw new GPUNFTError(`Failed to query ownership history: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Configure GPU rental pricing and terms
   *
   * @param keypair - Owner's keypair
   * @param nftId - NFT ID
   * @param terms - Rental terms configuration
   * @returns True if successful
   */
  async setRentalTerms(keypair: KeyringPair, nftId: NftId, terms: RentalTerms): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].setRentalTerms(
        nftId,
        terms.hourlyRate.toString(),
        terms.minimumHours,
        terms.maximumHours,
        terms.depositRequired.toString(),
        terms.autoRenew
      );

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to set rental terms: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Rent GPU for compute jobs
   *
   * @param keypair - Renter's keypair
   * @param nftId - NFT ID to rent
   * @param durationHours - Rental duration in hours
   * @returns Rental ID
   */
  async rentGpu(keypair: KeyringPair, nftId: NftId, durationHours: number): Promise<number> {
    try {
      const tx = this.api.tx[this.pallet].rentGpu(nftId, durationHours);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ events, status }) => {
          if (status.isInBlock || status.isFinalized) {
            events.forEach(({ event }) => {
              if (this.api.events[this.pallet].GPURented.is(event)) {
                const rentalId = (event.data[0] as any).toNumber();
                resolve(rentalId);
              }
            });

            reject(new GPUNFTError('GPU rental failed: no event found'));
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to rent GPU: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get all NFTs currently listed for sale
   *
   * @param limit - Maximum results to return
   * @returns List of listed NFTs
   */
  async getListedNfts(limit: number = 100): Promise<GpuNFT[]> {
    try {
      const nextId = await this.api.query[this.pallet].nextNftId();
      const listed: GpuNFT[] = [];

      for (let nftId = 0; nftId < nextId.toNumber() && listed.length < limit; nftId++) {
        try {
          const nft = await this.getNftMetadata(nftId);
          if (nft.isListed) {
            listed.push(nft);
          }
        } catch (error) {
          // Skip NFTs that don't exist
          continue;
        }
      }

      return listed;
    } catch (error) {
      throw new GPUNFTError(`Failed to get listed NFTs: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get active rental information
   *
   * @param rentalId - Rental ID
   * @returns Rental information
   */
  async getRentalInfo(rentalId: number): Promise<RentalInfo> {
    try {
      const result = await this.api.query[this.pallet].activeRentals(rentalId);

      if (result.isNone) {
        throw new GPUNFTError(`Rental ${rentalId} not found`);
      }

      const data = result.unwrap();
      return {
        rentalId,
        renter: data.renter.toString(),
        nftId: data.nftId.toNumber(),
        startTime: data.startTime.toNumber(),
        endTime: data.endTime.toNumber(),
        hourlyRate: BigInt(data.hourlyRate.toString()),
        deposit: BigInt(data.deposit.toString()),
        isActive: data.isActive.valueOf(),
      };
    } catch (error) {
      throw new GPUNFTError(`Failed to get rental info: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Cancel active rental (owner only)
   *
   * @param keypair - Owner's keypair
   * @param rentalId - Rental ID to cancel
   * @returns True if successful
   */
  async cancelRental(keypair: KeyringPair, rentalId: number): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].cancelRental(rentalId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to cancel rental: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Delist NFT from marketplace
   *
   * @param keypair - Owner's keypair
   * @param nftId - NFT ID to delist
   * @returns True if successful
   */
  async delistNft(keypair: KeyringPair, nftId: NftId): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].delistNft(nftId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPUNFTError(`Failed to delist NFT: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get NFTs owned by account
   *
   * @param owner - Owner address
   * @returns List of NFT IDs owned by the account
   */
  async getOwnedNfts(owner: string): Promise<NftId[]> {
    try {
      const result = await this.api.query[this.pallet].ownedNfts(owner);

      if (result.isNone) {
        return [];
      }

      return result.unwrap().map((nftId: any) => nftId.toNumber());
    } catch (error) {
      throw new GPUNFTError(`Failed to get owned NFTs: ${error instanceof Error ? error.message : String(error)}`);
    }
  }
}
