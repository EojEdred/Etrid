/**
 * Ëtrid AI Compute Network - JavaScript/TypeScript SDK
 *
 * Install:
 *   npm install @etrid/ai-compute
 *
 * Usage:
 *   import { AICompute } from '@etrid/ai-compute';
 *
 *   const client = new AICompute({ apiKey: 'your_telegram_user_id' });
 *
 *   const result = await client.run({
 *     model: 'gpt-4',
 *     prompt: 'Write a haiku about blockchain',
 *     maxTokens: 50
 *   });
 *
 *   console.log(result.output);
 */

import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { blake2AsHex } from '@polkadot/util-crypto';

export interface AIComputeConfig {
  apiKey?: string;
  wsUrl?: string;
  autoPayment?: boolean;
}

export interface JobParams {
  model: string;
  prompt: string;
  maxTokens?: number;
  priority?: 'economy' | 'standard' | 'premium';
  payment?: number;
}

export interface JobResult {
  jobId: number;
  output: string;
  cost: number;  // ËDSC
  computeTime: number;  // seconds
  gpuModel: string;
  status: string;
}

export interface ModelInfo {
  id: number;
  name: string;
  version: string;
  aidid: string;
  royalty: number;  // percentage
}

export interface NetworkStats {
  activeGpus: number;
  totalJobs: number;
  network: string;
  status: string;
}

export class AICompute {
  private apiKey?: string;
  private wsUrl: string;
  private autoPayment: boolean;
  private api?: ApiPromise;
  private account?: KeyringPair;

  constructor(config: AIComputeConfig = {}) {
    this.apiKey = config.apiKey;
    this.wsUrl = config.wsUrl || 'wss://ai-compute-pbc.etrid.network';
    this.autoPayment = config.autoPayment ?? true;
  }

  /**
   * Connect to AI-Compute-PBC blockchain
   */
  async connect(): Promise<void> {
    const wsProvider = new WsProvider(this.wsUrl);
    this.api = await ApiPromise.create({ provider: wsProvider });

    // Create account from API key
    if (this.apiKey) {
      const keyring = new Keyring({ type: 'sr25519' });
      // For demo: derive account from API key
      // In production: use proper key management (Telegram Wallet integration)
      this.account = keyring.addFromUri(`//${this.apiKey}`);
    }
  }

  /**
   * Run AI inference job
   */
  async run(params: JobParams): Promise<JobResult> {
    if (!this.api) {
      await this.connect();
    }

    const {
      model,
      prompt,
      maxTokens = 100,
      priority = 'standard',
      payment,
    } = params;

    // 1. Lookup model ID
    const modelId = await this.lookupModel(model);

    // 2. Calculate payment
    const cost = payment ?? this.estimateCost(model, maxTokens, priority);

    // 3. Submit job
    const jobId = await this.submitJob({
      modelId,
      inputData: prompt,
      maxTokens,
      payment: cost,
      priority,
    });

    // 4. Wait for result
    const result = await this.waitForResult(jobId);

    return result;
  }

  /**
   * Lookup model ID from registry
   */
  private async lookupModel(modelName: string): Promise<number> {
    if (!this.api) throw new Error('Not connected');

    const result = await this.api.query.modelRegistry.modelByName(
      Buffer.from(modelName, 'utf-8')
    );

    const ids = result.toJSON() as number[];
    if (ids && ids.length > 0) {
      return ids[0];  // Return first version
    } else {
      throw new Error(`Model '${modelName}' not found in registry`);
    }
  }

  /**
   * Estimate job cost in ËDSC
   */
  private estimateCost(
    model: string,
    maxTokens: number,
    priority: string
  ): number {
    // Base prices (ËDSC per 1K tokens)
    const basePrices: Record<string, number> = {
      'gpt-4': 0.03,
      'gpt-3.5': 0.001,
      'claude': 0.025,
      'stable-diffusion': 0.01,
      'whisper': 0.006,
    };

    // Priority multipliers
    const priorityMultipliers: Record<string, number> = {
      'economy': 1.0,
      'standard': 1.5,
      'premium': 3.0,
    };

    const basePrice = basePrices[model] || 0.01;
    const multiplier = priorityMultipliers[priority] || 1.5;

    const cost = (basePrice * maxTokens / 1000) * multiplier;
    return parseFloat(cost.toFixed(6));
  }

  /**
   * Submit job to blockchain
   */
  private async submitJob(params: {
    modelId: number;
    inputData: string;
    maxTokens: number;
    payment: number;
    priority: string;
  }): Promise<number> {
    if (!this.api || !this.account) throw new Error('Not connected');

    const { modelId, inputData, maxTokens, payment, priority } = params;

    // Hash input data
    const inputHash = blake2AsHex(inputData, 256);

    // Create extrinsic
    const tx = this.api.tx.jobMarketplace.submitJob(
      modelId,
      'LLM',  // Model type (auto-detect in production)
      inputHash,
      'json',  // Output format
      maxTokens * 2,  // Max compute time (rough estimate)
      BigInt(Math.floor(payment * 10**18)),  // Convert to smallest unit
      priority.charAt(0).toUpperCase() + priority.slice(1)  // Capitalize
    );

    // Sign and send
    return new Promise((resolve, reject) => {
      tx.signAndSend(this.account!, ({ events = [], status }) => {
        if (status.isInBlock || status.isFinalized) {
          // Extract job ID from events
          const jobSubmittedEvent = events.find(
            ({ event }) =>
              event.section === 'jobMarketplace' &&
              event.method === 'JobSubmitted'
          );

          if (jobSubmittedEvent) {
            const jobId = jobSubmittedEvent.event.data[0].toNumber();
            resolve(jobId);
          } else {
            reject(new Error('Job ID not found in events'));
          }
        }
      }).catch(reject);
    });
  }

  /**
   * Wait for job completion
   */
  private async waitForResult(
    jobId: number,
    timeout: number = 300000  // 5 minutes
  ): Promise<JobResult> {
    if (!this.api) throw new Error('Not connected');

    const startTime = Date.now();

    while (true) {
      // Query job status
      const job = await this.api.query.jobMarketplace.jobs(jobId);
      const jobData = job.toJSON() as any;

      if (jobData) {
        const status = jobData.status;

        if (status === 'Completed') {
          // Fetch result from off-chain storage
          const resultHash = jobData.resultHash;
          const output = await this.fetchResultData(resultHash);

          return {
            jobId,
            output,
            cost: parseFloat(jobData.payment) / 10**18,
            computeTime: jobData.completedAt - jobData.submittedAt,
            gpuModel: 'RTX 4090',  // TODO: Fetch from GPU registry
            status: 'Completed',
          };
        } else if (status === 'Failed') {
          throw new Error(`Job ${jobId} failed`);
        }
      }

      // Check timeout
      if (Date.now() - startTime > timeout) {
        throw new Error(`Job ${jobId} timed out after ${timeout}ms`);
      }

      // Wait 2 seconds before next poll
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
  }

  /**
   * Fetch result data from IPFS/Arweave
   */
  private async fetchResultData(resultHash: string): Promise<string> {
    // TODO: Implement IPFS fetch
    // For demo, return placeholder
    return 'This is the AI model output (fetched from IPFS)';
  }

  /**
   * List available AI models
   */
  async listModels(): Promise<ModelInfo[]> {
    if (!this.api) await this.connect();

    const total = await this.api!.query.modelRegistry.totalModels();
    const totalModels = total.toNumber();

    const models: ModelInfo[] = [];

    for (let i = 0; i < totalModels; i++) {
      const model = await this.api!.query.modelRegistry.models(i);
      const modelData = model.toJSON() as any;

      if (modelData) {
        models.push({
          id: i,
          name: Buffer.from(modelData.name, 'hex').toString('utf-8'),
          version: Buffer.from(modelData.version, 'hex').toString('utf-8'),
          aidid: Buffer.from(modelData.aidid, 'hex').toString('utf-8'),
          royalty: modelData.royaltyBps / 100,  // Convert to %
        });
      }
    }

    return models;
  }

  /**
   * Get network statistics
   */
  async getGpuStats(): Promise<NetworkStats> {
    if (!this.api) await this.connect();

    const activeGpus = await this.api!.query.gpuRegistry.activeGpuCount();
    const totalJobs = await this.api!.query.jobMarketplace.totalJobs();

    return {
      activeGpus: activeGpus.toNumber(),
      totalJobs: totalJobs.toNumber(),
      network: 'AI-Compute-PBC',
      status: 'online',
    };
  }

  /**
   * Disconnect from blockchain
   */
  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
    }
  }
}

// Example usage
async function example() {
  const client = new AICompute({ apiKey: 'your_telegram_user_id' });

  // Run GPT-4 inference
  const result = await client.run({
    model: 'gpt-4',
    prompt: 'Write a haiku about decentralized AI',
    maxTokens: 50,
    priority: 'standard',
  });

  console.log(`Output: ${result.output}`);
  console.log(`Cost: $${result.cost} ËDSC`);
  console.log(`Compute Time: ${result.computeTime}s`);
  console.log(`GPU: ${result.gpuModel}`);

  // List available models
  const models = await client.listModels();
  console.log(`\nAvailable models: ${models.length}`);
  models.slice(0, 5).forEach(model => {
    console.log(`  - ${model.name} (${model.version}) - ${model.aidid}`);
  });

  // Get network stats
  const stats = await client.getGpuStats();
  console.log(`\nNetwork Stats:`);
  console.log(`  Active GPUs: ${stats.activeGpus}`);
  console.log(`  Total Jobs: ${stats.totalJobs}`);

  await client.disconnect();
}

// Export for Node.js and browser
export default AICompute;
