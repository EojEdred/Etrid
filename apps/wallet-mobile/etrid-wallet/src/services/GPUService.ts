import EtridSDKService from './EtridSDKService';
import KeychainService from './KeychainService';

export interface GPUSpec {
  id?: string;
  model: string;
  vram: number; // GB
  computeUnits: number;
  clockSpeed: number; // MHz
  pricePerHour: string; // in ËDSC tokens
  provider: string;
  reputation: 'Bronze' | 'Silver' | 'Gold' | 'Platinum';
  availability: '24/7' | 'Business Hours' | 'Custom';
  uptime: number; // percentage
  location: string;
  attestation: {
    tpmQuote: string;
    benchmarkScore: number;
    verified: boolean;
  };
  available: boolean;
  minRentalHours: number;
  maxRentalHours: number;
}

export interface GPURental {
  rentalId: string;
  gpuId: string;
  gpuModel: string;
  startTime: number;
  endTime: number;
  durationHours: number;
  pricePerHour: string;
  totalCost: string;
  status: 'active' | 'expired' | 'cancelled';
  sshCredentials?: {
    host: string;
    port: number;
    username: string;
    password: string;
  };
  usageMetrics?: {
    cpuUsage: number;
    gpuUsage: number;
    memoryUsage: number;
    networkIn: number;
    networkOut: number;
  };
}

export interface GPUSearchFilters {
  model?: string;
  minVRAM?: number;
  maxVRAM?: number;
  minPrice?: number;
  maxPrice?: number;
  location?: string;
  reputation?: 'Bronze' | 'Silver' | 'Gold' | 'Platinum';
  availability?: '24/7' | 'Business Hours' | 'Custom';
  sortBy?: 'price' | 'compute' | 'reputation' | 'vram';
  sortOrder?: 'asc' | 'desc';
}

/**
 * Service for GPU marketplace operations
 * Uses GPURegistryWrapper and GPUNFTWrapper from Ëtrid SDK
 */
class GPUService {
  private sdk: EtridSDKService;

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
   * Search GPUs with filters
   */
  public async searchGPUs(filters: GPUSearchFilters): Promise<GPUSpec[]> {
    await this.ensureConnected();

    try {
      const gpus = await this.sdk.gpuRegistry.searchGpus({
        minVRAM: filters.minVRAM,
        maxPrice: filters.maxPrice,
        model: filters.model,
      });

      // Apply additional filters
      let filtered = gpus;

      if (filters.maxVRAM) {
        filtered = filtered.filter((gpu: any) => gpu.vram <= filters.maxVRAM!);
      }

      if (filters.location) {
        filtered = filtered.filter((gpu: any) => gpu.location === filters.location);
      }

      if (filters.reputation) {
        filtered = filtered.filter((gpu: any) => gpu.reputation === filters.reputation);
      }

      if (filters.availability) {
        filtered = filtered.filter((gpu: any) => gpu.availability === filters.availability);
      }

      // Sort
      if (filters.sortBy) {
        filtered.sort((a: any, b: any) => {
          let comparison = 0;
          switch (filters.sortBy) {
            case 'price':
              comparison = parseFloat(a.pricePerHour) - parseFloat(b.pricePerHour);
              break;
            case 'compute':
              comparison = b.computeUnits - a.computeUnits;
              break;
            case 'reputation':
              const repRank = { Bronze: 1, Silver: 2, Gold: 3, Platinum: 4 };
              comparison = repRank[b.reputation] - repRank[a.reputation];
              break;
            case 'vram':
              comparison = b.vram - a.vram;
              break;
          }
          return filters.sortOrder === 'desc' ? -comparison : comparison;
        });
      }

      return filtered.map(this.mapGPUFromChain);
    } catch (error) {
      console.error('Failed to search GPUs:', error);
      throw error;
    }
  }

  /**
   * Get GPU details by ID
   */
  public async getGPUDetails(gpuId: string): Promise<GPUSpec> {
    await this.ensureConnected();

    try {
      const gpu = await this.sdk.gpuRegistry.getGpuDetails(gpuId);
      return this.mapGPUFromChain({ id: gpuId, ...gpu });
    } catch (error) {
      console.error('Failed to get GPU details:', error);
      throw error;
    }
  }

  /**
   * Rent GPU
   */
  public async rentGPU(gpuId: string, durationHours: number): Promise<GPURental> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      const rental = await this.sdk.gpuNFT.rentGpu(keypair, gpuId, durationHours);

      // Generate SSH credentials (simulated - in production, this comes from the provider)
      const sshCredentials = {
        host: `gpu-${gpuId}.etrid.io`,
        port: 22,
        username: 'etrid',
        password: this.generateRandomPassword(),
      };

      return {
        rentalId: rental.rentalId,
        gpuId: rental.gpuId,
        gpuModel: 'RTX 4090', // Would come from GPU details
        startTime: Date.now(),
        endTime: Date.now() + durationHours * 3600 * 1000,
        durationHours: rental.duration,
        pricePerHour: '2.5',
        totalCost: (2.5 * durationHours).toString(),
        status: 'active',
        sshCredentials,
      };
    } catch (error) {
      console.error('Failed to rent GPU:', error);
      throw error;
    }
  }

  /**
   * Get my GPU rentals
   */
  public async getMyRentals(): Promise<GPURental[]> {
    await this.ensureConnected();

    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        throw new Error('No wallet found');
      }

      const rentals = await this.sdk.gpuNFT.getMyRentals(address);

      return rentals.map((rental: any) => ({
        rentalId: rental.rentalId,
        gpuId: rental.gpuId,
        gpuModel: rental.gpuModel || 'Unknown',
        startTime: rental.startTime,
        endTime: rental.endTime,
        durationHours: rental.duration,
        pricePerHour: rental.pricePerHour,
        totalCost: rental.totalCost,
        status: this.getRentalStatus(rental.endTime),
        sshCredentials: rental.sshCredentials,
        usageMetrics: rental.usageMetrics,
      }));
    } catch (error) {
      console.error('Failed to get rentals:', error);
      throw error;
    }
  }

  /**
   * Extend rental
   */
  public async extendRental(rentalId: string, additionalHours: number): Promise<void> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.gpuNFT.extendRental(keypair, rentalId, additionalHours);
    } catch (error) {
      console.error('Failed to extend rental:', error);
      throw error;
    }
  }

  /**
   * End rental
   */
  public async endRental(rentalId: string): Promise<void> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.gpuNFT.endRental(keypair, rentalId);
    } catch (error) {
      console.error('Failed to end rental:', error);
      throw error;
    }
  }

  /**
   * Register GPU (for providers)
   */
  public async registerGPU(gpuSpec: Omit<GPUSpec, 'id'>): Promise<string> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      const gpuId = await this.sdk.gpuRegistry.registerGpu(keypair, {
        model: gpuSpec.model,
        vram: gpuSpec.vram,
        computeUnits: gpuSpec.computeUnits,
        pricePerHour: gpuSpec.pricePerHour,
        availability: gpuSpec.availability,
        attestation: gpuSpec.attestation,
      });

      return gpuId;
    } catch (error) {
      console.error('Failed to register GPU:', error);
      throw error;
    }
  }

  /**
   * Update GPU availability
   */
  public async updateGPUAvailability(gpuId: string, available: boolean): Promise<void> {
    await this.ensureConnected();

    try {
      const keypair = await KeychainService.loadKeypair();
      if (!keypair) {
        throw new Error('No wallet found');
      }

      await this.sdk.gpuRegistry.updateGpuAvailability(keypair, gpuId, available);
    } catch (error) {
      console.error('Failed to update GPU availability:', error);
      throw error;
    }
  }

  /**
   * Get rental usage metrics (simulated)
   */
  public async getRentalMetrics(rentalId: string): Promise<any> {
    // In production, this would query real-time metrics from the GPU provider
    return {
      cpuUsage: Math.random() * 100,
      gpuUsage: Math.random() * 100,
      memoryUsage: Math.random() * 100,
      networkIn: Math.random() * 1000,
      networkOut: Math.random() * 1000,
    };
  }

  // Helper methods
  private mapGPUFromChain(gpu: any): GPUSpec {
    return {
      id: gpu.id,
      model: gpu.model,
      vram: gpu.vram,
      computeUnits: gpu.computeUnits,
      clockSpeed: gpu.clockSpeed || 2400,
      pricePerHour: gpu.pricePerHour,
      provider: gpu.provider || 'Unknown',
      reputation: gpu.reputation || 'Bronze',
      availability: gpu.availability || '24/7',
      uptime: gpu.uptime || 99.5,
      location: gpu.location || 'US-East',
      attestation: gpu.attestation || {
        tpmQuote: '',
        benchmarkScore: 0,
        verified: false,
      },
      available: gpu.available !== false,
      minRentalHours: gpu.minRentalHours || 1,
      maxRentalHours: gpu.maxRentalHours || 720,
    };
  }

  private getRentalStatus(endTime: number): 'active' | 'expired' | 'cancelled' {
    if (endTime > Date.now()) {
      return 'active';
    }
    return 'expired';
  }

  private generateRandomPassword(): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
    let password = '';
    for (let i = 0; i < 16; i++) {
      password += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return password;
  }
}

export default new GPUService();
