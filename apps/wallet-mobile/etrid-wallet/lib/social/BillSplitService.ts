/**
 * Bill Split Service
 * Manages group expenses, bill splitting, and payment tracking
 */

import type {
  BillSplit,
  BillSplitInput,
  BillSplitParticipant,
  BillSplitSummary,
  BillSplitStatus,
  ApiResponse,
} from './types';
import { BillSplitError } from './types';

export class BillSplitService {
  private apiUrl: string;
  private userId: string;

  constructor(
    userId: string,
    apiUrl: string = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3001'
  ) {
    this.userId = userId;
    this.apiUrl = apiUrl;
  }

  /**
   * Create a new bill split
   */
  async createSplit(input: BillSplitInput): Promise<BillSplit> {
    // Validate input
    this.validateSplitInput(input);

    // Calculate participant amounts
    const participants = this.calculateParticipantAmounts(input);

    try {
      const response = await fetch(`${this.apiUrl}/bill-split/create`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          creatorId: this.userId,
          name: input.name,
          description: input.description,
          totalAmount: input.totalAmount,
          splitType: input.splitType,
          participants,
          notes: input.notes,
        }),
      });

      const result: ApiResponse<BillSplit> = await response.json();

      if (!result.success || !result.data) {
        throw new BillSplitError(
          result.error?.message || 'Failed to create bill split',
          result.error?.code || 'CREATE_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof BillSplitError) throw error;

      // Fallback to mock data for development
      console.warn('[BillSplitService] API not available, using mock data');
      return this.mockCreateSplit(input, participants);
    }
  }

  /**
   * Get all bill splits for the current user
   */
  async getSplits(filter?: 'pending' | 'completed'): Promise<BillSplit[]> {
    try {
      const url = filter
        ? `${this.apiUrl}/bill-split/list?userId=${this.userId}&status=${filter}`
        : `${this.apiUrl}/bill-split/list?userId=${this.userId}`;

      const response = await fetch(url);
      const result: ApiResponse<BillSplit[]> = await response.json();

      if (!result.success || !result.data) {
        throw new BillSplitError(
          result.error?.message || 'Failed to fetch bill splits',
          result.error?.code || 'FETCH_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof BillSplitError) throw error;

      // Fallback to mock data for development
      console.warn('[BillSplitService] API not available, using mock data');
      return this.mockGetSplits(filter);
    }
  }

  /**
   * Get a specific bill split by ID
   */
  async getSplit(splitId: string): Promise<BillSplit | null> {
    try {
      const response = await fetch(`${this.apiUrl}/bill-split/${splitId}`);
      const result: ApiResponse<BillSplit> = await response.json();

      if (!result.success || !result.data) {
        return null;
      }

      return result.data;
    } catch (error) {
      console.error('[BillSplitService] Failed to fetch split:', error);
      return null;
    }
  }

  /**
   * Pay your share of a bill split
   */
  async payShare(
    splitId: string,
    transactionHash: string
  ): Promise<{ split: BillSplit; txHash: string }> {
    try {
      const response = await fetch(`${this.apiUrl}/bill-split/${splitId}/pay`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          userId: this.userId,
          transactionHash,
        }),
      });

      const result: ApiResponse<{ split: BillSplit; txHash: string }> = await response.json();

      if (!result.success || !result.data) {
        throw new BillSplitError(
          result.error?.message || 'Failed to record payment',
          result.error?.code || 'PAYMENT_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof BillSplitError) throw error;
      throw new BillSplitError('Failed to record payment', 'PAYMENT_FAILED');
    }
  }

  /**
   * Send reminder to participants who haven't paid
   */
  async remindParticipants(splitId: string): Promise<void> {
    try {
      const response = await fetch(`${this.apiUrl}/bill-split/${splitId}/remind`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      });

      const result: ApiResponse<void> = await response.json();

      if (!result.success) {
        throw new BillSplitError(
          result.error?.message || 'Failed to send reminders',
          result.error?.code || 'REMINDER_FAILED'
        );
      }
    } catch (error) {
      if (error instanceof BillSplitError) throw error;
      throw new BillSplitError('Failed to send reminders', 'REMINDER_FAILED');
    }
  }

  /**
   * Cancel a bill split
   */
  async cancelSplit(splitId: string): Promise<BillSplit> {
    try {
      const response = await fetch(`${this.apiUrl}/bill-split/${splitId}/cancel`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
      });

      const result: ApiResponse<BillSplit> = await response.json();

      if (!result.success || !result.data) {
        throw new BillSplitError(
          result.error?.message || 'Failed to cancel split',
          result.error?.code || 'CANCEL_FAILED'
        );
      }

      return result.data;
    } catch (error) {
      if (error instanceof BillSplitError) throw error;
      throw new BillSplitError('Failed to cancel split', 'CANCEL_FAILED');
    }
  }

  /**
   * Get summary of all bill splits for the user
   */
  async getSummary(): Promise<BillSplitSummary> {
    const splits = await this.getSplits();

    let totalOwed = 0;
    let totalReceivable = 0;
    let pending = 0;
    let completed = 0;

    splits.forEach((split) => {
      if (split.status === 'pending' || split.status === 'partial') {
        pending++;
      } else if (split.status === 'completed') {
        completed++;
      }

      // Calculate what user owes or is owed
      const userParticipant = split.participants.find((p) => p.userId === this.userId);
      if (userParticipant) {
        const remaining = userParticipant.amountOwed - userParticipant.amountPaid;
        if (remaining > 0) {
          totalOwed += remaining;
        }
      }

      // Calculate what user is owed (if creator)
      if (split.creatorId === this.userId) {
        split.participants.forEach((p) => {
          if (p.userId !== this.userId) {
            const remaining = p.amountOwed - p.amountPaid;
            if (remaining > 0) {
              totalReceivable += remaining;
            }
          }
        });
      }
    });

    return {
      total: splits.length,
      pending,
      completed,
      totalOwed,
      totalReceivable,
    };
  }

  /**
   * Get splits where user owes money
   */
  async getOwedSplits(): Promise<BillSplit[]> {
    const splits = await this.getSplits('pending');
    return splits.filter((split) => {
      const userParticipant = split.participants.find((p) => p.userId === this.userId);
      if (!userParticipant) return false;
      return userParticipant.amountPaid < userParticipant.amountOwed;
    });
  }

  /**
   * Get splits where user is owed money
   */
  async getReceivableSplits(): Promise<BillSplit[]> {
    const splits = await this.getSplits();
    return splits.filter((split) => {
      if (split.creatorId !== this.userId) return false;
      return split.participants.some((p) => p.amountPaid < p.amountOwed);
    });
  }

  // ============================================================================
  // PRIVATE HELPERS
  // ============================================================================

  /**
   * Validate bill split input
   */
  private validateSplitInput(input: BillSplitInput): void {
    if (!input.name || input.name.trim().length === 0) {
      throw new BillSplitError('Bill name is required', 'INVALID_INPUT');
    }

    if (input.totalAmount <= 0) {
      throw new BillSplitError('Total amount must be greater than 0', 'INVALID_AMOUNT');
    }

    if (!input.participants || input.participants.length === 0) {
      throw new BillSplitError('At least one participant is required', 'NO_PARTICIPANTS');
    }

    // Validate custom split amounts
    if (input.splitType === 'custom') {
      const totalCustom = input.participants.reduce(
        (sum, p) => sum + (p.amountOwed || 0),
        0
      );
      if (Math.abs(totalCustom - input.totalAmount) > 0.01) {
        throw new BillSplitError(
          'Custom amounts must sum to total amount',
          'INVALID_SPLIT'
        );
      }
    }

    // Validate percentage split
    if (input.splitType === 'percentage') {
      const totalPercentage = input.participants.reduce(
        (sum, p) => sum + (p.percentage || 0),
        0
      );
      if (Math.abs(totalPercentage - 100) > 0.01) {
        throw new BillSplitError(
          'Percentages must sum to 100',
          'INVALID_PERCENTAGE'
        );
      }
    }
  }

  /**
   * Calculate participant amounts based on split type
   */
  private calculateParticipantAmounts(
    input: BillSplitInput
  ): Omit<BillSplitParticipant, 'id' | 'transactionHash' | 'paidAt'>[] {
    const { splitType, totalAmount, participants } = input;

    switch (splitType) {
      case 'equal':
        const equalAmount = totalAmount / participants.length;
        return participants.map((p) => ({
          userId: p.address, // Simplified, should map to actual user ID
          address: p.address,
          username: p.username,
          amountOwed: Math.round(equalAmount * 100) / 100,
          amountPaid: 0,
          status: 'pending' as const,
        }));

      case 'custom':
        return participants.map((p) => ({
          userId: p.address,
          address: p.address,
          username: p.username,
          amountOwed: p.amountOwed || 0,
          amountPaid: 0,
          status: 'pending' as const,
        }));

      case 'percentage':
        return participants.map((p) => {
          const amount = (totalAmount * (p.percentage || 0)) / 100;
          return {
            userId: p.address,
            address: p.address,
            username: p.username,
            amountOwed: Math.round(amount * 100) / 100,
            amountPaid: 0,
            status: 'pending' as const,
          };
        });

      default:
        throw new BillSplitError('Invalid split type', 'INVALID_TYPE');
    }
  }

  /**
   * Calculate split status
   */
  private calculateStatus(participants: BillSplitParticipant[]): BillSplitStatus {
    const allPaid = participants.every((p) => p.amountPaid >= p.amountOwed);
    const somePaid = participants.some((p) => p.amountPaid > 0);

    if (allPaid) return 'completed';
    if (somePaid) return 'partial';
    return 'pending';
  }

  // ============================================================================
  // MOCK DATA (for development/testing)
  // ============================================================================

  private mockCreateSplit(
    input: BillSplitInput,
    participants: Omit<BillSplitParticipant, 'id' | 'transactionHash' | 'paidAt'>[]
  ): BillSplit {
    return {
      id: `split-${Date.now()}`,
      creatorId: this.userId,
      creatorAddress: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
      name: input.name,
      description: input.description,
      totalAmount: input.totalAmount,
      splitType: input.splitType,
      status: 'pending',
      participants: participants.map((p, i) => ({
        ...p,
        id: `participant-${i}`,
      })),
      createdAt: new Date(),
      updatedAt: new Date(),
      notes: input.notes,
    };
  }

  private mockGetSplits(filter?: 'pending' | 'completed'): BillSplit[] {
    const allSplits: BillSplit[] = [
      {
        id: '1',
        creatorId: this.userId,
        creatorAddress: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        name: 'Dinner at Restaurant',
        description: 'Team dinner celebration',
        totalAmount: 150.0,
        splitType: 'equal',
        status: 'partial',
        participants: [
          {
            id: 'p1',
            userId: this.userId,
            address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
            username: 'me',
            amountOwed: 50.0,
            amountPaid: 50.0,
            status: 'paid',
            paidAt: new Date('2024-11-15'),
          },
          {
            id: 'p2',
            address: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
            username: 'alice',
            amountOwed: 50.0,
            amountPaid: 50.0,
            status: 'paid',
            paidAt: new Date('2024-11-16'),
          },
          {
            id: 'p3',
            address: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
            username: 'bob',
            amountOwed: 50.0,
            amountPaid: 0,
            status: 'pending',
          },
        ],
        createdAt: new Date('2024-11-15'),
        updatedAt: new Date('2024-11-16'),
      },
    ];

    if (filter === 'pending') {
      return allSplits.filter(
        (s) => s.status === 'pending' || s.status === 'partial'
      );
    } else if (filter === 'completed') {
      return allSplits.filter((s) => s.status === 'completed');
    }

    return allSplits;
  }
}

/**
 * Create a BillSplitService instance
 */
export const createBillSplitService = (userId: string) => new BillSplitService(userId);
