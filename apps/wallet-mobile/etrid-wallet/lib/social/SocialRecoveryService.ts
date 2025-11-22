/**
 * Social Recovery Service
 * Manages guardians, recovery processes, and account recovery mechanisms
 */

import type {
  Guardian,
  SocialRecoveryConfig,
  RecoveryProcess,
  RecoveryInitiation,
  RecoveryApproval,
  GuardianStatus,
  ApiResponse,
} from './types';
import { RecoveryError } from './types';

export class SocialRecoveryService {
  private apiUrl: string;
  private walletAddress: string;

  constructor(
    walletAddress: string,
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001'
  ) {
    this.walletAddress = walletAddress;
    this.apiUrl = apiUrl;
  }

  // ============================================================================
  // GUARDIAN MANAGEMENT
  // ============================================================================

  /**
   * Get all guardians for the wallet
   */
  async getGuardians(): Promise<Guardian[]> {
    try {
      const response = await fetch(
        `${this.apiUrl}/recovery/guardians?walletAddress=${this.walletAddress}`
      );
      const result: ApiResponse<Guardian[]> = await response.json();

      if (!result.success || !result.data) {
        throw new RecoveryError(
          result.error?.message || 'Failed to fetch guardians',
          result.error?.code || 'FETCH_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof RecoveryError) throw error;

      // Fallback to mock data for development
      console.warn('[SocialRecoveryService] API not available, using mock data');
      return this.mockGetGuardians();
    }
  }

  /**
   * Add a new guardian
   */
  async addGuardian(guardianAddress: string, guardianUsername?: string): Promise<Guardian> {
    // Validate guardian address
    if (guardianAddress === this.walletAddress) {
      throw new RecoveryError(
        'Cannot add yourself as a guardian',
        'INVALID_GUARDIAN'
      );
    }

    // Check if guardian already exists
    const existingGuardians = await this.getGuardians();
    if (existingGuardians.some((g) => g.guardianAddress === guardianAddress)) {
      throw new RecoveryError(
        'Guardian already exists',
        'GUARDIAN_EXISTS'
      );
    }

    // Check maximum guardians limit (e.g., 10)
    if (existingGuardians.length >= 10) {
      throw new RecoveryError(
        'Maximum number of guardians reached',
        'MAX_GUARDIANS'
      );
    }

    try {
      const response = await fetch(`${this.apiUrl}/recovery/guardians`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          walletAddress: this.walletAddress,
          guardianAddress,
          guardianUsername,
        }),
      });

      const result: ApiResponse<Guardian> = await response.json();

      if (!result.success || !result.data) {
        throw new RecoveryError(
          result.error?.message || 'Failed to add guardian',
          result.error?.code || 'ADD_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof RecoveryError) throw error;

      // Fallback to mock data for development
      console.warn('[SocialRecoveryService] API not available, using mock data');
      return this.mockAddGuardian(guardianAddress, guardianUsername);
    }
  }

  /**
   * Remove a guardian
   */
  async removeGuardian(guardianId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/recovery/guardians/${guardianId}`, {
        method: 'DELETE',
      });

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new RecoveryError(
          result.error?.message || 'Failed to remove guardian',
          result.error?.code || 'REMOVE_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof RecoveryError) throw error;
      throw new RecoveryError('Failed to remove guardian', 'REMOVE_FAILED');
    }
  }

  /**
   * Get recovery configuration
   */
  async getRecoveryConfig(): Promise<SocialRecoveryConfig> {
    try {
      const response = await fetch(
        `${this.apiUrl}/recovery/config?walletAddress=${this.walletAddress}`
      );
      const result: ApiResponse<SocialRecoveryConfig> = await response.json();

      if (!result.success || !result.data) {
        // Return default config if not found
        const guardians = await this.getGuardians();
        return {
          walletAddress: this.walletAddress,
          threshold: Math.ceil(guardians.length / 2), // Default: majority
          guardians,
          createdAt: new Date(),
          updatedAt: new Date(),
        };
      }

      return result.data;
    } catch (error) {
      console.error('[SocialRecoveryService] Failed to fetch config:', error);
      const guardians = await this.getGuardians();
      return {
        walletAddress: this.walletAddress,
        threshold: Math.ceil(guardians.length / 2),
        guardians,
        createdAt: new Date(),
        updatedAt: new Date(),
      };
    }
  }

  /**
   * Set recovery threshold (number of guardians required)
   */
  async setThreshold(threshold: number): Promise<void> {
    const guardians = await this.getGuardians();
    const activeGuardians = guardians.filter((g) => g.status === 'active');

    if (threshold < 1) {
      throw new RecoveryError(
        'Threshold must be at least 1',
        'INVALID_THRESHOLD'
      );
    }

    if (threshold > activeGuardians.length) {
      throw new RecoveryError(
        'Threshold cannot exceed number of active guardians',
        'INVALID_THRESHOLD'
      );
    }

    try {
      const response = await fetch(`${this.apiUrl}/recovery/config/threshold`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          walletAddress: this.walletAddress,
          threshold,
        }),
      });

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new RecoveryError(
          result.error?.message || 'Failed to set threshold',
          result.error?.code || 'THRESHOLD_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof RecoveryError) throw error;
      throw new RecoveryError('Failed to set threshold', 'THRESHOLD_FAILED');
    }
  }

  /**
   * Resend invitation to a pending guardian
   */
  async resendInvitation(guardianId: string): Promise<void> {
    try {
      const response = await fetch(
        `${this.apiUrl}/recovery/guardians/${guardianId}/invite`,
        {
          method: 'POST',
        }
      );

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new RecoveryError(
          result.error?.message || 'Failed to resend invitation',
          result.error?.code || 'INVITE_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof RecoveryError) throw error;
      throw new RecoveryError('Failed to resend invitation', 'INVITE_FAILED');
    }
  }

  // ============================================================================
  // RECOVERY PROCESS
  // ============================================================================

  /**
   * Initiate a recovery process
   */
  async initiateRecovery(newDeviceId: string, newAddress?: string): Promise<RecoveryProcess> {
    const config = await this.getRecoveryConfig();

    // Validate that we have enough guardians
    const activeGuardians = config.guardians.filter((g) => g.status === 'active');
    if (activeGuardians.length < config.threshold) {
      throw new RecoveryError(
        'Not enough active guardians for recovery',
        'INSUFFICIENT_GUARDIANS'
      );
    }

    try {
      const response = await fetch(`${this.apiUrl}/recovery/initiate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          walletAddress: this.walletAddress,
          newDeviceId,
          newAddress,
        }),
      });

      const result: ApiResponse<RecoveryProcess> = await response.json();

      if (!result.success || !result.data) {
        throw new RecoveryError(
          result.error?.message || 'Failed to initiate recovery',
          result.error?.code || 'INITIATE_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof RecoveryError) throw error;

      // Fallback to mock data for development
      console.warn('[SocialRecoveryService] API not available, using mock data');
      return this.mockInitiateRecovery(newDeviceId, newAddress, config.threshold);
    }
  }

  /**
   * Get active recovery process
   */
  async getActiveRecovery(): Promise<RecoveryProcess | null> {
    try {
      const response = await fetch(
        `${this.apiUrl}/recovery/active?walletAddress=${this.walletAddress}`
      );
      const result: ApiResponse<RecoveryProcess> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data;
    } catch (error) {
      console.error('[SocialRecoveryService] Failed to fetch active recovery:', error);
      return null;
    }
  }

  /**
   * Get recovery process by ID
   */
  async getRecovery(recoveryId: string): Promise<RecoveryProcess | null> {
    try {
      const response = await fetch(`${this.apiUrl}/recovery/${recoveryId}`);
      const result: ApiResponse<RecoveryProcess> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data;
    } catch (error) {
      console.error('[SocialRecoveryService] Failed to fetch recovery:', error);
      return null;
    }
  }

  /**
   * Approve a recovery process (called by guardian)
   */
  async approveRecovery(recoveryId: string, signature: string): Promise<RecoveryProcess> {
    try {
      const response = await fetch(`${this.apiUrl}/recovery/${recoveryId}/approve`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          guardianAddress: this.walletAddress, // In this case, current user is the guardian
          signature,
        }),
      });

      const result: ApiResponse<RecoveryProcess> = await response.json();

      if (!result.success || !result.data) {
        throw new RecoveryError(
          result.error?.message || 'Failed to approve recovery',
          result.error?.code || 'APPROVE_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof RecoveryError) throw error;
      throw new RecoveryError('Failed to approve recovery', 'APPROVE_FAILED');
    }
  }

  /**
   * Cancel an active recovery process
   */
  async cancelRecovery(recoveryId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/recovery/${recoveryId}/cancel`, {
        method: 'POST',
      });

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new RecoveryError(
          result.error?.message || 'Failed to cancel recovery',
          result.error?.code || 'CANCEL_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof RecoveryError) throw error;
      throw new RecoveryError('Failed to cancel recovery', 'CANCEL_FAILED');
    }
  }

  /**
   * Get pending recoveries where current wallet is a guardian
   */
  async getPendingApprovals(): Promise<RecoveryProcess[]> {
    try {
      const response = await fetch(
        `${this.apiUrl}/recovery/pending-approvals?guardianAddress=${this.walletAddress}`
      );
      const result: ApiResponse<RecoveryProcess[]> = await response.json();

      if (!result.success || !result.data) {
        return [];
      }

      return result.data;
    } catch (error) {
      console.error('[SocialRecoveryService] Failed to fetch pending approvals:', error);
      return [];
    }
  }

  // ============================================================================
  // UTILITIES
  // ============================================================================

  /**
   * Check if recovery is ready to execute (threshold reached)
   */
  isRecoveryReady(recovery: RecoveryProcess): boolean {
    return recovery.approvals.length >= recovery.requiredApprovals;
  }

  /**
   * Check if recovery time delay has passed
   */
  hasDelayPassed(recovery: RecoveryProcess): boolean {
    if (!recovery.completesAt) return false;
    return new Date() >= new Date(recovery.completesAt);
  }

  /**
   * Get recommended threshold for number of guardians
   */
  getRecommendedThreshold(guardianCount: number): number {
    if (guardianCount <= 2) return guardianCount; // All must approve
    if (guardianCount === 3) return 2; // 2 of 3
    if (guardianCount === 4) return 3; // 3 of 4
    if (guardianCount === 5) return 3; // 3 of 5
    return Math.ceil(guardianCount * 0.6); // 60% for larger groups
  }

  // ============================================================================
  // MOCK DATA (for development/testing)
  // ============================================================================

  private mockGetGuardians(): Guardian[] {
    return [
      {
        id: '1',
        walletAddress: this.walletAddress,
        guardianAddress: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        guardianUsername: 'alice',
        status: 'active',
        addedAt: new Date('2024-10-01'),
        activatedAt: new Date('2024-10-02'),
      },
      {
        id: '2',
        walletAddress: this.walletAddress,
        guardianAddress: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
        guardianUsername: 'bob',
        status: 'active',
        addedAt: new Date('2024-10-01'),
        activatedAt: new Date('2024-10-03'),
      },
      {
        id: '3',
        walletAddress: this.walletAddress,
        guardianAddress: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
        guardianUsername: 'charlie',
        status: 'pending',
        addedAt: new Date('2024-11-15'),
        invitationSentAt: new Date('2024-11-15'),
      },
    ];
  }

  private mockAddGuardian(guardianAddress: string, guardianUsername?: string): Guardian {
    return {
      id: `guardian-${Date.now()}`,
      walletAddress: this.walletAddress,
      guardianAddress,
      guardianUsername,
      status: 'pending',
      addedAt: new Date(),
      invitationSentAt: new Date(),
    };
  }

  private mockInitiateRecovery(
    newDeviceId: string,
    newAddress: string | undefined,
    threshold: number
  ): RecoveryProcess {
    const completesAt = new Date();
    completesAt.setHours(completesAt.getHours() + 48); // 48 hour delay

    return {
      id: `recovery-${Date.now()}`,
      walletAddress: this.walletAddress,
      newDeviceId,
      newAddress,
      status: 'initiated',
      requiredApprovals: threshold,
      approvals: [],
      initiatedAt: new Date(),
      completesAt,
    };
  }
}

/**
 * Create a SocialRecoveryService instance
 */
export const createSocialRecoveryService = (walletAddress: string) =>
  new SocialRecoveryService(walletAddress);
