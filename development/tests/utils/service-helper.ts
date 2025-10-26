import axios, { AxiosInstance } from 'axios';

/**
 * Utilities for interacting with attestation and relayer services
 */

export interface Attestation {
  messageHash: string;
  message: string;
  signatures: string[];
  signatureCount: number;
  thresholdMet: boolean;
  status: 'pending' | 'ready' | 'relayed' | 'expired';
}

export interface ServiceHealth {
  status: 'healthy' | 'degraded' | 'unhealthy';
  uptime: number;
  [key: string]: any;
}

export class AttestationServiceHelper {
  private client: AxiosInstance;

  constructor(private baseUrl: string) {
    this.client = axios.create({
      baseURL: baseUrl,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  /**
   * Check service health
   */
  async checkHealth(): Promise<ServiceHealth> {
    const response = await this.client.get<ServiceHealth>('/health');
    return response.data;
  }

  /**
   * Wait for service to be healthy
   */
  async waitForHealthy(timeoutMs: number = 30000): Promise<void> {
    const startTime = Date.now();

    while (Date.now() - startTime < timeoutMs) {
      try {
        const health = await this.checkHealth();
        if (health.status === 'healthy') {
          console.log('Attestation service is healthy');
          return;
        }
      } catch (error) {
        // Service not ready yet
      }

      await new Promise((resolve) => setTimeout(resolve, 1000));
    }

    throw new Error('Attestation service did not become healthy in time');
  }

  /**
   * Get attestation by message hash
   */
  async getAttestation(messageHash: string): Promise<Attestation | null> {
    try {
      const response = await this.client.get<Attestation>(`/attestation/${messageHash}`);
      return response.data;
    } catch (error: any) {
      if (error?.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  /**
   * Get attestation by domain and nonce
   */
  async getAttestationByNonce(
    sourceDomain: number,
    nonce: bigint
  ): Promise<Attestation | null> {
    try {
      const response = await this.client.get<Attestation>(
        `/attestation/${sourceDomain}/${nonce.toString()}`
      );
      return response.data;
    } catch (error: any) {
      if (error?.response?.status === 404) {
        return null;
      }
      throw error;
    }
  }

  /**
   * Get all ready attestations
   */
  async getReadyAttestations(): Promise<Attestation[]> {
    const response = await this.client.get<{ count: number; attestations: Attestation[] }>(
      '/attestations/ready'
    );
    return response.data.attestations;
  }

  /**
   * Wait for attestation to be ready
   */
  async waitForAttestation(
    messageHash: string,
    timeoutMs: number = 60000
  ): Promise<Attestation> {
    const startTime = Date.now();

    while (Date.now() - startTime < timeoutMs) {
      const attestation = await this.getAttestation(messageHash);

      if (attestation && attestation.thresholdMet) {
        console.log(
          `Attestation ready: ${messageHash} (${attestation.signatureCount} signatures)`
        );
        return attestation;
      }

      await new Promise((resolve) => setTimeout(resolve, 2000));
    }

    throw new Error(`Attestation not ready in time: ${messageHash}`);
  }

  /**
   * Wait for attestation by nonce to be ready
   */
  async waitForAttestationByNonce(
    sourceDomain: number,
    nonce: bigint,
    timeoutMs: number = 60000
  ): Promise<Attestation> {
    const startTime = Date.now();

    while (Date.now() - startTime < timeoutMs) {
      const attestation = await this.getAttestationByNonce(sourceDomain, nonce);

      if (attestation && attestation.thresholdMet) {
        console.log(
          `Attestation ready: domain=${sourceDomain} nonce=${nonce} (${attestation.signatureCount} signatures)`
        );
        return attestation;
      }

      await new Promise((resolve) => setTimeout(resolve, 2000));
    }

    throw new Error(
      `Attestation not ready in time: domain=${sourceDomain} nonce=${nonce}`
    );
  }

  /**
   * Get service statistics
   */
  async getStats(): Promise<any> {
    const response = await this.client.get('/stats');
    return response.data;
  }

  /**
   * Get monitor status
   */
  async getStatus(): Promise<any> {
    const response = await this.client.get('/status');
    return response.data;
  }
}

export class RelayerServiceHelper {
  constructor(private baseUrl?: string) {
    // Relayer service may not have an API endpoint in basic setup
  }

  /**
   * Check if relayer service is running (basic check)
   */
  async isRunning(): Promise<boolean> {
    if (!this.baseUrl) {
      // No API endpoint, assume running if no error
      return true;
    }

    try {
      await axios.get(`${this.baseUrl}/health`, { timeout: 5000 });
      return true;
    } catch (error) {
      return false;
    }
  }

  /**
   * Wait for message to be relayed (by checking destination chain)
   */
  async waitForRelay(
    checkFunction: () => Promise<boolean>,
    timeoutMs: number = 120000
  ): Promise<void> {
    const startTime = Date.now();

    while (Date.now() - startTime < timeoutMs) {
      const isRelayed = await checkFunction();

      if (isRelayed) {
        console.log('Message relayed successfully');
        return;
      }

      await new Promise((resolve) => setTimeout(resolve, 3000));
    }

    throw new Error('Message not relayed in time');
  }
}

/**
 * Wait for a condition to be true
 */
export async function waitFor(
  condition: () => Promise<boolean>,
  timeoutMs: number = 60000,
  intervalMs: number = 2000,
  description: string = 'condition'
): Promise<void> {
  const startTime = Date.now();

  while (Date.now() - startTime < timeoutMs) {
    if (await condition()) {
      console.log(`✓ ${description}`);
      return;
    }

    await new Promise((resolve) => setTimeout(resolve, intervalMs));
  }

  throw new Error(`Timeout waiting for ${description}`);
}

/**
 * Sleep for a duration
 */
export async function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
