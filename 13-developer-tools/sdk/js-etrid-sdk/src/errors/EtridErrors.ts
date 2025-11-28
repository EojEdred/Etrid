/**
 * Comprehensive error handling for Etrid SDK
 */

import { SubmittableResult } from '@polkadot/api/types';
import { DispatchError } from '@polkadot/types/interfaces';
import { ErrorCode, ModuleError } from '../types/enhanced';

/**
 * Base error class for all Etrid SDK errors
 */
export class EtridError extends Error {
  constructor(
    message: string,
    public code: ErrorCode,
    public details?: any
  ) {
    super(message);
    this.name = 'EtridError';
    Object.setPrototypeOf(this, EtridError.prototype);
  }

  /**
   * Convert error to JSON for logging
   */
  toJSON(): object {
    return {
      name: this.name,
      message: this.message,
      code: this.code,
      details: this.details,
      stack: this.stack,
    };
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return this.message;
  }
}

/**
 * Transaction-specific errors
 */
export class TransactionError extends EtridError {
  constructor(message: string, public result?: SubmittableResult) {
    super(message, ErrorCode.TRANSACTION_FAILED, { result });
    this.name = 'TransactionError';
    Object.setPrototypeOf(this, TransactionError.prototype);
  }

  /**
   * Get dispatch error from result
   */
  getDispatchError(): DispatchError | null {
    return this.result?.dispatchError || null;
  }

  /**
   * Get module error information
   */
  getModuleError(): ModuleError | null {
    const err = this.getDispatchError();
    if (err?.isModule) {
      const decoded = err.asModule;
      return {
        index: decoded.index.toNumber(),
        error: decoded.error.toNumber(),
      };
    }
    return null;
  }

  /**
   * Check if error is due to insufficient balance
   */
  isInsufficientBalance(): boolean {
    const moduleErr = this.getModuleError();
    // Check common balance module errors
    return moduleErr?.index === 10 && moduleErr?.error === 2;
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    if (this.isInsufficientBalance()) {
      return 'Insufficient balance to complete transaction';
    }

    const moduleErr = this.getModuleError();
    if (moduleErr) {
      return `Transaction failed: Module ${moduleErr.index}, Error ${moduleErr.error}`;
    }

    return this.message;
  }
}

/**
 * Validation errors
 */
export class ValidationError extends EtridError {
  constructor(message: string, public field: string, value?: any) {
    super(message, ErrorCode.VALIDATION_ERROR, { field, value });
    this.name = 'ValidationError';
    Object.setPrototypeOf(this, ValidationError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return `Validation failed for ${this.field}: ${this.message}`;
  }
}

/**
 * Network/connection errors
 */
export class NetworkError extends EtridError {
  constructor(message: string, public endpoint?: string, originalError?: Error) {
    super(message, ErrorCode.NETWORK_ERROR, { endpoint, originalError });
    this.name = 'NetworkError';
    Object.setPrototypeOf(this, NetworkError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    if (this.endpoint) {
      return `Network error connecting to ${this.endpoint}: ${this.message}`;
    }
    return `Network error: ${this.message}`;
  }
}

/**
 * Insufficient balance error
 */
export class InsufficientBalanceError extends EtridError {
  constructor(
    public required: bigint,
    public available: bigint,
    public currency: 'ETR' | 'ETD' = 'ETR'
  ) {
    super(
      `Insufficient ${currency} balance. Required: ${required}, Available: ${available}`,
      ErrorCode.INSUFFICIENT_BALANCE,
      { required, available, currency }
    );
    this.name = 'InsufficientBalanceError';
    Object.setPrototypeOf(this, InsufficientBalanceError.prototype);
  }

  /**
   * Get shortage amount
   */
  getShortage(): bigint {
    return this.required - this.available;
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return `Insufficient ${this.currency} balance. You need ${this.getShortage()} more.`;
  }
}

/**
 * Invalid address error
 */
export class InvalidAddressError extends ValidationError {
  constructor(public address: string, reason?: string) {
    super(
      reason || 'Invalid address format',
      'address',
      address
    );
    this.code = ErrorCode.INVALID_ADDRESS;
    this.name = 'InvalidAddressError';
    Object.setPrototypeOf(this, InvalidAddressError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return `Invalid address: ${this.address}`;
  }
}

/**
 * Invalid amount error
 */
export class InvalidAmountError extends ValidationError {
  constructor(public amount: any, reason?: string) {
    super(
      reason || 'Invalid amount',
      'amount',
      amount
    );
    this.code = ErrorCode.INVALID_AMOUNT;
    this.name = 'InvalidAmountError';
    Object.setPrototypeOf(this, InvalidAmountError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return `Invalid amount: ${this.amount}. Amount must be positive.`;
  }
}

/**
 * Staking-specific errors
 */
export class StakingError extends EtridError {
  constructor(message: string, details?: any) {
    super(message, ErrorCode.STAKING_ERROR, details);
    this.name = 'StakingError';
    Object.setPrototypeOf(this, StakingError.prototype);
  }
}

/**
 * Governance-specific errors
 */
export class GovernanceError extends EtridError {
  constructor(message: string, details?: any) {
    super(message, ErrorCode.GOVERNANCE_ERROR, details);
    this.name = 'GovernanceError';
    Object.setPrototypeOf(this, GovernanceError.prototype);
  }
}

/**
 * Channel-specific errors
 */
export class ChannelError extends EtridError {
  constructor(message: string, public channelId?: string, details?: any) {
    super(message, ErrorCode.CHANNEL_ERROR, { channelId, ...details });
    this.name = 'ChannelError';
    Object.setPrototypeOf(this, ChannelError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    if (this.channelId) {
      return `Channel error (${this.channelId}): ${this.message}`;
    }
    return `Channel error: ${this.message}`;
  }
}

/**
 * Not connected error
 */
export class NotConnectedError extends EtridError {
  constructor() {
    super('Client is not connected to the blockchain', ErrorCode.NOT_CONNECTED);
    this.name = 'NotConnectedError';
    Object.setPrototypeOf(this, NotConnectedError.prototype);
  }

  /**
   * Get user-friendly error message
   */
  getUserMessage(): string {
    return 'Not connected to blockchain. Please call connect() first.';
  }
}

/**
 * Error helper functions
 */
export class ErrorHelpers {
  /**
   * Check if error is retryable
   */
  static isRetryable(error: Error): boolean {
    if (error instanceof NetworkError) {
      return true;
    }
    if (error instanceof EtridError) {
      return [
        ErrorCode.NETWORK_ERROR,
        ErrorCode.CONNECTION_FAILED,
        ErrorCode.RPC_ERROR,
      ].includes(error.code);
    }
    return false;
  }

  /**
   * Get error category for analytics
   */
  static getCategory(error: Error): string {
    if (error instanceof TransactionError) return 'transaction';
    if (error instanceof ValidationError) return 'validation';
    if (error instanceof NetworkError) return 'network';
    if (error instanceof StakingError) return 'staking';
    if (error instanceof GovernanceError) return 'governance';
    if (error instanceof ChannelError) return 'channel';
    return 'unknown';
  }

  /**
   * Wrap unknown errors
   */
  static wrap(error: unknown, defaultMessage = 'An unexpected error occurred'): EtridError {
    if (error instanceof EtridError) {
      return error;
    }

    if (error instanceof Error) {
      return new EtridError(error.message, ErrorCode.UNKNOWN_ERROR, { originalError: error });
    }

    return new EtridError(defaultMessage, ErrorCode.UNKNOWN_ERROR, { error });
  }

  /**
   * Create error from dispatch error
   */
  static fromDispatchError(dispatchError: DispatchError, result?: SubmittableResult): TransactionError {
    if (dispatchError.isModule) {
      const decoded = dispatchError.asModule;
      return new TransactionError(
        `Module error: ${decoded.index.toNumber()}::${decoded.error.toNumber()}`,
        result
      );
    }

    if (dispatchError.isBadOrigin) {
      return new TransactionError('Bad origin', result);
    }

    if (dispatchError.isCannotLookup) {
      return new TransactionError('Cannot lookup', result);
    }

    return new TransactionError('Transaction dispatch error', result);
  }
}

/**
 * Export all error classes and helpers
 */
export {
  ErrorCode,
};
