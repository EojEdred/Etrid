import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

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
 * Basis points (0-10000 representing 0%-100%)
 */
export type BasisPoints = number;

/**
 * GPU hardware specifications
 */
export interface GpuSpecs {
  /** GPU model name (e.g., "RTX 4090", "A100") */
  model: string;
  /** VRAM in gigabytes */
  vramGb: number;
  /** Compute units (CUDA cores, stream processors, etc.) */
  computeUnits: number;
  /** Clock speed in MHz */
  clockSpeedMhz: number;
  /** Power consumption in watts */
  tdpWatts: number;
}

/**
 * Hardware attestation proof
 */
export interface HardwareAttestation {
  /** TPM quote proving hardware authenticity (max 256 bytes) */
  tpmQuote: Uint8Array;
  /** Benchmark score proving performance */
  benchmarkScore: number;
  /** Attestation timestamp */
  timestamp: Timestamp;
}

/**
 * GPU provider reputation metrics
 */
export interface Reputation {
  /** Total jobs completed successfully */
  jobsCompleted: number;
  /** Total jobs failed */
  jobsFailed: number;
  /** Uptime percentage in basis points (0-10000) */
  uptimeBps: BasisPoints;
  /** Average rating (0-50000 = 0.0-5.0 stars, scaled by 10000) */
  rating: number;
  /** Total ratings received */
  ratingCount: number;
}

/**
 * Helper functions for Reputation
 */
export const ReputationHelpers = {
  /** Get uptime as percentage (0-100) */
  uptimePercent(rep: Reputation): number {
    return rep.uptimeBps / 100;
  },

  /** Get rating as stars (0.0-5.0) */
  ratingStars(rep: Reputation): number {
    return rep.rating / 10000;
  },

  /** Get job success rate as percentage */
  successRate(rep: Reputation): number {
    const total = rep.jobsCompleted + rep.jobsFailed;
    return total === 0 ? 0 : (rep.jobsCompleted / total) * 100;
  },
};

/**
 * GPU node status
 */
export enum GpuStatus {
  /** GPU is online and accepting jobs */
  Active = 'Active',
  /** GPU is temporarily paused (manual) */
  Paused = 'Paused',
  /** GPU is offline (detected by off-chain worker) */
  Offline = 'Offline',
  /** GPU is slashed for misbehavior */
  Slashed = 'Slashed',
}

/**
 * GPU availability schedule
 */
export type AvailabilitySchedule =
  | { type: 'AlwaysOn' }
  | { type: 'BusinessHours' }
  | { type: 'Custom'; schedule: Uint8Array }; // 21 bytes = 168 bits (weekly schedule)

/**
 * Helper to create availability schedules
 */
export const AvailabilitySchedule = {
  alwaysOn(): AvailabilitySchedule {
    return { type: 'AlwaysOn' };
  },

  businessHours(): AvailabilitySchedule {
    return { type: 'BusinessHours' };
  },

  custom(schedule: Uint8Array): AvailabilitySchedule {
    if (schedule.length !== 21) {
      throw new Error('Custom schedule must be exactly 21 bytes (168 bits)');
    }
    return { type: 'Custom', schedule };
  },
};

/**
 * Complete GPU node information
 */
export interface GpuNode {
  /** Provider account address */
  provider: string;
  /** GPU specifications */
  specs: GpuSpecs;
  /** Hardware attestation */
  attestation: HardwareAttestation;
  /** Staked amount in ËDSC */
  stake: Balance;
  /** Current status */
  status: GpuStatus;
  /** Reputation metrics */
  reputation: Reputation;
  /** Availability schedule */
  schedule: AvailabilitySchedule;
  /** Registration timestamp */
  registeredAt: Timestamp;
  /** Last heartbeat timestamp */
  lastHeartbeat: Timestamp;
}

/**
 * Provider earnings information
 */
export interface ProviderEarnings {
  /** Total earned in ËDSC */
  totalEarned: Balance;
  /** Pending payout amount */
  pendingPayout: Balance;
  /** Last payout timestamp */
  lastPayout: Timestamp;
  /** Earnings history */
  earningsHistory: EarningsRecord[];
}

/**
 * Individual earnings record
 */
export interface EarningsRecord {
  /** Amount earned */
  amount: Balance;
  /** Job ID or rental ID */
  sourceId: number;
  /** Timestamp */
  timestamp: Timestamp;
}

/**
 * GPU search filters
 */
export interface GpuSearchFilters {
  /** Minimum VRAM in GB */
  minVramGb?: number;
  /** Minimum compute units */
  minComputeUnits?: number;
  /** GPU status filter */
  status?: GpuStatus;
  /** Minimum rating (0.0-5.0) */
  minRating?: number;
  /** Minimum uptime percentage (0-100) */
  minUptime?: number;
  /** Maximum results */
  limit?: number;
}

/**
 * GPU Registry errors
 */
export class GPURegistryError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'GPURegistryError';
  }
}

export class GPUNotFoundError extends GPURegistryError {
  constructor(gpuId: GpuId) {
    super(`GPU ${gpuId} not found`);
    this.name = 'GPUNotFoundError';
  }
}

export class InsufficientStakeError extends GPURegistryError {
  constructor(stake: Balance, required: Balance) {
    super(`Insufficient stake: ${stake}, required: ${required}`);
    this.name = 'InsufficientStakeError';
  }
}

/**
 * GPURegistryWrapper
 *
 * Wrapper for pallet-gpu-registry - GPU provider registration and management.
 *
 * **Features:**
 * - GPU node registration with hardware attestation
 * - Staking mechanism (providers stake ËDSC)
 * - Reputation tracking (uptime, job success rate, user ratings)
 * - Hardware verification (prevent fake/virtualized GPUs)
 * - Scheduled availability (24/7, business hours, custom schedules)
 * - Provider earnings tracking
 *
 * @example
 * ```typescript
 * import { ApiPromise } from '@polkadot/api';
 * import { GPURegistryWrapper, GpuSpecs, HardwareAttestation } from '@etrid/sdk';
 *
 * const api = await ApiPromise.create({ provider });
 * const registry = new GPURegistryWrapper(api);
 *
 * // Register GPU
 * const specs: GpuSpecs = {
 *   model: 'RTX 4090',
 *   vramGb: 24,
 *   computeUnits: 16384,
 *   clockSpeedMhz: 2520,
 *   tdpWatts: 450
 * };
 *
 * const attestation: HardwareAttestation = {
 *   tpmQuote: new Uint8Array(256),
 *   benchmarkScore: 98500,
 *   timestamp: Math.floor(Date.now() / 1000)
 * };
 *
 * const gpuId = await registry.registerGpu(
 *   keypair,
 *   specs,
 *   attestation,
 *   100n * 10n**18n, // 100 ËDSC
 *   AvailabilitySchedule.alwaysOn()
 * );
 *
 * // Query GPU details
 * const gpu = await registry.getGpuSpecs(gpuId);
 * console.log(`GPU: ${gpu.specs.model}, ${gpu.specs.vramGb}GB VRAM`);
 *
 * // Check reputation
 * const rep = await registry.getReputation(gpuId);
 * console.log(`Rating: ${ReputationHelpers.ratingStars(rep)}/5.0`);
 * console.log(`Uptime: ${ReputationHelpers.uptimePercent(rep)}%`);
 *
 * // Search for GPUs
 * const results = await registry.searchGpus({
 *   minVramGb: 16,
 *   minComputeUnits: 10000,
 *   status: GpuStatus.Active,
 *   limit: 10
 * });
 * ```
 */
export class GPURegistryWrapper {
  private readonly pallet = 'gpuRegistry';

  /**
   * Create GPU Registry wrapper
   * @param api - Connected Polkadot.js API instance
   */
  constructor(private api: ApiPromise) {}

  /**
   * Register a new GPU node
   *
   * Registers GPU with hardware specifications, attestation proof, and stake.
   * Provider must stake minimum amount of ËDSC tokens.
   *
   * @param keypair - Provider's keypair for signing
   * @param specs - GPU hardware specifications
   * @param attestation - Hardware attestation proof
   * @param stake - Amount to stake (must be >= MinimumStake)
   * @param schedule - Availability schedule
   * @returns GPU ID assigned to the registered GPU
   *
   * @example
   * ```typescript
   * const gpuId = await registry.registerGpu(
   *   keypair,
   *   specs,
   *   attestation,
   *   100n * 10n**18n, // 100 ËDSC
   *   AvailabilitySchedule.alwaysOn()
   * );
   * ```
   */
  async registerGpu(
    keypair: KeyringPair,
    specs: GpuSpecs,
    attestation: HardwareAttestation,
    stake: Balance,
    schedule: AvailabilitySchedule
  ): Promise<GpuId> {
    try {
      const tx = this.api.tx[this.pallet].registerGpu(
        specs,
        attestation,
        stake.toString(),
        schedule
      );

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ events, status }) => {
          if (status.isInBlock || status.isFinalized) {
            events.forEach(({ event }) => {
              if (this.api.events[this.pallet].GpuRegistered.is(event)) {
                const gpuId = (event.data[0] as any).toNumber();
                resolve(gpuId);
              }
            });

            // If no GpuRegistered event found
            reject(new GPURegistryError('GPU registration failed: no event found'));
          }
        }).catch(reject);
      });
    } catch (error) {
      if (error instanceof Error && error.message.includes('InsufficientStake')) {
        throw new InsufficientStakeError(stake, 0n); // Actual minimum would come from chain
      }
      throw new GPURegistryError(`Failed to register GPU: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Unregister GPU and withdraw stake
   *
   * @param keypair - Provider's keypair
   * @param gpuId - GPU ID to unregister
   * @returns True if successful
   */
  async unregisterGpu(keypair: KeyringPair, gpuId: GpuId): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].unregisterGpu(gpuId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      if (error instanceof Error && error.message.includes('GpuNotFound')) {
        throw new GPUNotFoundError(gpuId);
      }
      throw new GPURegistryError(`Failed to unregister GPU: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Update GPU availability schedule
   *
   * @param keypair - Provider's keypair
   * @param gpuId - GPU ID
   * @param schedule - New availability schedule
   * @returns True if successful
   */
  async updateAvailability(
    keypair: KeyringPair,
    gpuId: GpuId,
    schedule: AvailabilitySchedule
  ): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].updateAvailability(gpuId, schedule);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPURegistryError(`Failed to update availability: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Query GPU hardware details
   *
   * @param gpuId - GPU ID
   * @returns Complete GPU node information
   */
  async getGpuSpecs(gpuId: GpuId): Promise<GpuNode> {
    try {
      const result = await this.api.query[this.pallet].gpuNodes(gpuId);

      if (result.isNone) {
        throw new GPUNotFoundError(gpuId);
      }

      const data = result.unwrap();
      return this.parseGpuNode(data);
    } catch (error) {
      if (error instanceof GPUNotFoundError) {
        throw error;
      }
      throw new GPURegistryError(`Failed to query GPU specs: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get provider reputation metrics
   *
   * @param gpuId - GPU ID
   * @returns Reputation metrics
   */
  async getReputation(gpuId: GpuId): Promise<Reputation> {
    const gpu = await this.getGpuSpecs(gpuId);
    return gpu.reputation;
  }

  /**
   * Search for GPUs matching criteria
   *
   * @param filters - Search filters
   * @returns List of matching GPU nodes
   */
  async searchGpus(filters: GpuSearchFilters = {}): Promise<GpuNode[]> {
    try {
      const nextId = await this.api.query[this.pallet].nextGpuId();
      const limit = filters.limit || 100;
      const results: GpuNode[] = [];

      for (let gpuId = 0; gpuId < nextId.toNumber() && results.length < limit; gpuId++) {
        try {
          const gpu = await this.getGpuSpecs(gpuId);

          // Apply filters
          if (filters.minVramGb && gpu.specs.vramGb < filters.minVramGb) continue;
          if (filters.minComputeUnits && gpu.specs.computeUnits < filters.minComputeUnits) continue;
          if (filters.status && gpu.status !== filters.status) continue;
          if (filters.minRating && ReputationHelpers.ratingStars(gpu.reputation) < filters.minRating) continue;
          if (filters.minUptime && ReputationHelpers.uptimePercent(gpu.reputation) < filters.minUptime) continue;

          results.push(gpu);
        } catch (error) {
          // Skip GPUs that don't exist
          continue;
        }
      }

      return results;
    } catch (error) {
      throw new GPURegistryError(`Failed to search GPUs: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Report GPU online status (heartbeat)
   *
   * @param keypair - Provider's keypair
   * @param gpuId - GPU ID
   * @returns True if successful
   */
  async reportUptime(keypair: KeyringPair, gpuId: GpuId): Promise<boolean> {
    try {
      const tx = this.api.tx[this.pallet].reportHeartbeat(gpuId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ status }) => {
          if (status.isInBlock || status.isFinalized) {
            resolve(true);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPURegistryError(`Failed to report uptime: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Query provider earnings history
   *
   * @param provider - Provider account address
   * @returns Earnings data
   */
  async getProviderEarnings(provider: string): Promise<ProviderEarnings> {
    // This would query a separate earnings tracking storage
    // For now, returning placeholder structure
    return {
      totalEarned: 0n,
      pendingPayout: 0n,
      lastPayout: 0,
      earningsHistory: [],
    };
  }

  /**
   * Penalize provider for poor performance (validator/governance only)
   *
   * @param keypair - Validator/governance keypair
   * @param gpuId - GPU ID to slash
   * @returns Amount slashed
   */
  async slashProvider(keypair: KeyringPair, gpuId: GpuId): Promise<Balance> {
    try {
      const tx = this.api.tx[this.pallet].slashProvider(gpuId);

      return new Promise((resolve, reject) => {
        tx.signAndSend(keypair, ({ events, status }) => {
          if (status.isInBlock || status.isFinalized) {
            events.forEach(({ event }) => {
              if (this.api.events[this.pallet].ProviderSlashed.is(event)) {
                const slashAmount = BigInt((event.data[1] as any).toString());
                resolve(slashAmount);
              }
            });

            resolve(0n);
          }
        }).catch(reject);
      });
    } catch (error) {
      throw new GPURegistryError(`Failed to slash provider: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Parse GPU node data from chain
   * @private
   */
  private parseGpuNode(data: any): GpuNode {
    return {
      provider: data.provider.toString(),
      specs: {
        model: data.specs.model.toUtf8(),
        vramGb: data.specs.vramGb.toNumber(),
        computeUnits: data.specs.computeUnits.toNumber(),
        clockSpeedMhz: data.specs.clockSpeedMhz.toNumber(),
        tdpWatts: data.specs.tdpWatts.toNumber(),
      },
      attestation: {
        tpmQuote: data.attestation.tpmQuote.toU8a(),
        benchmarkScore: data.attestation.benchmarkScore.toNumber(),
        timestamp: data.attestation.timestamp.toNumber(),
      },
      stake: BigInt(data.stake.toString()),
      status: this.parseStatus(data.status),
      reputation: {
        jobsCompleted: data.reputation.jobsCompleted.toNumber(),
        jobsFailed: data.reputation.jobsFailed.toNumber(),
        uptimeBps: data.reputation.uptimeBps.toNumber(),
        rating: data.reputation.rating.toNumber(),
        ratingCount: data.reputation.ratingCount.toNumber(),
      },
      schedule: this.parseSchedule(data.schedule),
      registeredAt: data.registeredAt.toNumber(),
      lastHeartbeat: data.lastHeartbeat.toNumber(),
    };
  }

  /**
   * Parse GPU status from chain
   * @private
   */
  private parseStatus(status: any): GpuStatus {
    const statusStr = status.toString();
    if (statusStr === 'Active') return GpuStatus.Active;
    if (statusStr === 'Paused') return GpuStatus.Paused;
    if (statusStr === 'Offline') return GpuStatus.Offline;
    if (statusStr === 'Slashed') return GpuStatus.Slashed;
    return GpuStatus.Offline;
  }

  /**
   * Parse availability schedule from chain
   * @private
   */
  private parseSchedule(schedule: any): AvailabilitySchedule {
    if (schedule.isAlwaysOn) return { type: 'AlwaysOn' };
    if (schedule.isBusinessHours) return { type: 'BusinessHours' };
    if (schedule.isCustom) {
      return { type: 'Custom', schedule: schedule.asCustom.toU8a() };
    }
    return { type: 'AlwaysOn' };
  }
}
