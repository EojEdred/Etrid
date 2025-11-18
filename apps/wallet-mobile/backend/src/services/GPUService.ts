import axios from 'axios';
import config from '../config';
import logger from '../utils/logger';
import { GPUSearchResult } from '../types';

class GPUService {
  /**
   * Search available GPUs
   */
  async searchGPUs(params: {
    min_vram?: number;
    max_price?: number;
    gpu_type?: string;
    provider?: string;
  }): Promise<GPUSearchResult[]> {
    const gpus: GPUSearchResult[] = [];

    try {
      // Fetch from Vast.ai
      if (params.provider === 'all' || params.provider === 'Vast.ai') {
        const vastGPUs = await this.searchVastAI(params);
        gpus.push(...vastGPUs);
      }

      // Fetch from RunPod
      if (params.provider === 'all' || params.provider === 'RunPod') {
        const runpodGPUs = await this.searchRunPod(params);
        gpus.push(...runpodGPUs);
      }

      // Filter and sort
      let filtered = gpus;

      if (params.min_vram) {
        filtered = filtered.filter((g) => g.vram_gb >= params.min_vram!);
      }

      if (params.max_price) {
        filtered = filtered.filter(
          (g) => parseFloat(g.price_per_hour) <= params.max_price!
        );
      }

      if (params.gpu_type) {
        filtered = filtered.filter((g) =>
          g.name.toLowerCase().includes(params.gpu_type!.toLowerCase())
        );
      }

      return filtered.sort((a, b) => parseFloat(a.price_per_hour) - parseFloat(b.price_per_hour));
    } catch (error: any) {
      logger.error('Error searching GPUs', { error: error.message });
      throw error;
    }
  }

  /**
   * Search Vast.ai
   */
  private async searchVastAI(params: any): Promise<GPUSearchResult[]> {
    try {
      const response = await axios.get('https://console.vast.ai/api/v0/bundles/', {
        headers: {
          Authorization: `Bearer ${config.gpu.vastAi.apiKey}`,
        },
      });

      return response.data.offers
        .filter((offer: any) => offer.verification === 'verified')
        .map((offer: any) => ({
          id: offer.id.toString(),
          name: offer.gpu_name,
          provider: 'Vast.ai',
          vram_gb: offer.gpu_ram,
          gpu_count: offer.num_gpus,
          cpu_cores: offer.cpu_cores,
          ram_gb: offer.ram,
          disk_gb: offer.disk_space,
          price_per_hour: offer.dph_total.toString(),
          availability: true,
        }));
    } catch (error: any) {
      logger.warn('Vast.ai API error', { error: error.message });
      return [];
    }
  }

  /**
   * Search RunPod
   */
  private async searchRunPod(params: any): Promise<GPUSearchResult[]> {
    try {
      const response = await axios.get('https://api.runpod.io/graphql', {
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${config.gpu.runpod.apiKey}`,
        },
        data: {
          query: `
            query {
              gpuTypes {
                id
                displayName
                memoryInGb
                secureCloud
                communityCloud
                lowestPrice {
                  gpuName
                  minimumBidPrice
                  uninterruptablePrice
                }
              }
            }
          `,
        },
      });

      return response.data.data.gpuTypes.map((gpu: any) => ({
        id: gpu.id,
        name: gpu.displayName,
        provider: 'RunPod',
        vram_gb: gpu.memoryInGb,
        gpu_count: 1,
        cpu_cores: 8,
        ram_gb: 32,
        disk_gb: 100,
        price_per_hour: gpu.lowestPrice?.uninterruptablePrice || '0.5',
        availability: gpu.communityCloud || gpu.secureCloud,
      }));
    } catch (error: any) {
      logger.warn('RunPod API error', { error: error.message });
      return [];
    }
  }

  /**
   * Get GPU details
   */
  async getGPUDetails(gpuId: string): Promise<GPUSearchResult | null> {
    // Try to fetch details from provider
    // For now, return mock data
    return {
      id: gpuId,
      name: 'NVIDIA RTX 4090',
      provider: 'Vast.ai',
      vram_gb: 24,
      gpu_count: 1,
      cpu_cores: 16,
      ram_gb: 64,
      disk_gb: 500,
      price_per_hour: '1.2',
      availability: true,
    };
  }

  /**
   * Provision GPU instance
   */
  async provisionGPU(
    provider: string,
    gpuId: string,
    durationHours: number
  ): Promise<any> {
    try {
      if (provider === 'Vast.ai') {
        return await this.provisionVastAI(gpuId, durationHours);
      } else if (provider === 'RunPod') {
        return await this.provisionRunPod(gpuId, durationHours);
      }

      throw new Error('Unsupported provider');
    } catch (error: any) {
      logger.error('Error provisioning GPU', {
        provider,
        gpuId,
        error: error.message,
      });
      throw error;
    }
  }

  /**
   * Provision Vast.ai instance
   */
  private async provisionVastAI(gpuId: string, durationHours: number): Promise<any> {
    const response = await axios.post(
      `https://console.vast.ai/api/v0/asks/${gpuId}/`,
      {
        client_id: 'etrid-wallet',
        image: 'pytorch/pytorch:latest',
        disk: 50,
        label: `etrid-gpu-${Date.now()}`,
      },
      {
        headers: {
          Authorization: `Bearer ${config.gpu.vastAi.apiKey}`,
        },
      }
    );

    const instance = response.data.new_contract;

    return {
      instance_id: instance.id.toString(),
      ssh_host: instance.ssh_host,
      ssh_port: instance.ssh_port,
      ssh_username: 'root',
      ssh_password: instance.ssh_password,
      status: 'provisioning',
    };
  }

  /**
   * Provision RunPod instance
   */
  private async provisionRunPod(gpuId: string, durationHours: number): Promise<any> {
    const response = await axios.post(
      'https://api.runpod.io/graphql',
      {
        query: `
          mutation {
            podFindAndDeployOnDemand(
              input: {
                cloudType: COMMUNITY
                gpuTypeId: "${gpuId}"
                name: "etrid-gpu-${Date.now()}"
                imageName: "runpod/pytorch:latest"
                dockerArgs: ""
                ports: "22/tcp"
              }
            ) {
              id
              imageName
              podType
            }
          }
        `,
      },
      {
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${config.gpu.runpod.apiKey}`,
        },
      }
    );

    const pod = response.data.data.podFindAndDeployOnDemand;

    return {
      instance_id: pod.id,
      ssh_host: `${pod.id}.runpod.io`,
      ssh_port: 22,
      ssh_username: 'root',
      ssh_password: Math.random().toString(36).substring(2, 15),
      status: 'provisioning',
    };
  }

  /**
   * Terminate instance
   */
  async terminateInstance(provider: string, instanceId: string): Promise<void> {
    try {
      if (provider === 'Vast.ai') {
        await axios.delete(
          `https://console.vast.ai/api/v0/instances/${instanceId}/`,
          {
            headers: {
              Authorization: `Bearer ${config.gpu.vastAi.apiKey}`,
            },
          }
        );
      } else if (provider === 'RunPod') {
        await axios.post(
          'https://api.runpod.io/graphql',
          {
            query: `
              mutation {
                podTerminate(input: { podId: "${instanceId}" })
              }
            `,
          },
          {
            headers: {
              'Content-Type': 'application/json',
              Authorization: `Bearer ${config.gpu.runpod.apiKey}`,
            },
          }
        );
      }

      logger.info('Instance terminated', { provider, instanceId });
    } catch (error: any) {
      logger.error('Error terminating instance', {
        provider,
        instanceId,
        error: error.message,
      });
      throw error;
    }
  }
}

export default new GPUService();
