/**
 * Tests for Error Handling
 */

import { describe, it, expect } from '@jest/globals';
import {
  EtridError,
  TransactionError,
  ValidationError,
  NetworkError,
  InsufficientBalanceError,
  InvalidAddressError,
  InvalidAmountError,
  StakingError,
  GovernanceError,
  ChannelError,
  NotConnectedError,
  ErrorHelpers,
} from '../src/errors/EtridErrors';
import { ErrorCode } from '../src/types/enhanced';

describe('Error Handling', () => {
  describe('EtridError', () => {
    it('should create base error', () => {
      const error = new EtridError('Test error', ErrorCode.UNKNOWN_ERROR);

      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(EtridError);
      expect(error.message).toBe('Test error');
      expect(error.code).toBe(ErrorCode.UNKNOWN_ERROR);
      expect(error.name).toBe('EtridError');
    });

    it('should include details in error', () => {
      const details = { foo: 'bar', count: 123 };
      const error = new EtridError('Test error', ErrorCode.UNKNOWN_ERROR, details);

      expect(error.details).toEqual(details);
    });

    it('should convert to JSON', () => {
      const error = new EtridError('Test error', ErrorCode.UNKNOWN_ERROR, { foo: 'bar' });
      const json = error.toJSON();

      expect(json).toHaveProperty('name');
      expect(json).toHaveProperty('message');
      expect(json).toHaveProperty('code');
      expect(json).toHaveProperty('details');
      expect(json).toHaveProperty('stack');
    });

    it('should get user message', () => {
      const error = new EtridError('Test error', ErrorCode.UNKNOWN_ERROR);
      const message = error.getUserMessage();

      expect(message).toBe('Test error');
    });
  });

  describe('TransactionError', () => {
    it('should create transaction error', () => {
      const error = new TransactionError('Transaction failed');

      expect(error).toBeInstanceOf(TransactionError);
      expect(error).toBeInstanceOf(EtridError);
      expect(error.code).toBe(ErrorCode.TRANSACTION_FAILED);
      expect(error.name).toBe('TransactionError');
    });

    it('should get dispatch error', () => {
      const error = new TransactionError('Transaction failed');
      const dispatchError = error.getDispatchError();

      expect(dispatchError).toBeNull();
    });

    it('should get module error', () => {
      const error = new TransactionError('Transaction failed');
      const moduleError = error.getModuleError();

      expect(moduleError).toBeNull();
    });

    it('should check insufficient balance', () => {
      const error = new TransactionError('Transaction failed');
      const isInsufficientBalance = error.isInsufficientBalance();

      expect(typeof isInsufficientBalance).toBe('boolean');
    });
  });

  describe('ValidationError', () => {
    it('should create validation error', () => {
      const error = new ValidationError('Invalid value', 'amount', 123);

      expect(error).toBeInstanceOf(ValidationError);
      expect(error.field).toBe('amount');
      expect(error.details.value).toBe(123);
    });

    it('should get user message', () => {
      const error = new ValidationError('Invalid value', 'amount');
      const message = error.getUserMessage();

      expect(message).toContain('amount');
      expect(message).toContain('Invalid value');
    });
  });

  describe('NetworkError', () => {
    it('should create network error', () => {
      const error = new NetworkError('Connection failed', 'ws://localhost:9944');

      expect(error).toBeInstanceOf(NetworkError);
      expect(error.code).toBe(ErrorCode.NETWORK_ERROR);
      expect(error.endpoint).toBe('ws://localhost:9944');
    });

    it('should get user message with endpoint', () => {
      const error = new NetworkError('Connection failed', 'ws://localhost:9944');
      const message = error.getUserMessage();

      expect(message).toContain('ws://localhost:9944');
      expect(message).toContain('Connection failed');
    });

    it('should get user message without endpoint', () => {
      const error = new NetworkError('Connection failed');
      const message = error.getUserMessage();

      expect(message).toContain('Network error');
      expect(message).toContain('Connection failed');
    });
  });

  describe('InsufficientBalanceError', () => {
    it('should create insufficient balance error', () => {
      const error = new InsufficientBalanceError(1000n, 500n);

      expect(error).toBeInstanceOf(InsufficientBalanceError);
      expect(error.code).toBe(ErrorCode.INSUFFICIENT_BALANCE);
      expect(error.required).toBe(1000n);
      expect(error.available).toBe(500n);
      expect(error.currency).toBe('ETR');
    });

    it('should calculate shortage', () => {
      const error = new InsufficientBalanceError(1000n, 500n);
      const shortage = error.getShortage();

      expect(shortage).toBe(500n);
    });

    it('should get user message', () => {
      const error = new InsufficientBalanceError(1000n, 500n);
      const message = error.getUserMessage();

      expect(message).toContain('ETR');
      expect(message).toContain('500');
    });

    it('should support ETD currency', () => {
      const error = new InsufficientBalanceError(1000n, 500n, 'ETD');

      expect(error.currency).toBe('ETD');
      expect(error.getUserMessage()).toContain('ETD');
    });
  });

  describe('InvalidAddressError', () => {
    it('should create invalid address error', () => {
      const error = new InvalidAddressError('invalid-address');

      expect(error).toBeInstanceOf(InvalidAddressError);
      expect(error).toBeInstanceOf(ValidationError);
      expect(error.code).toBe(ErrorCode.INVALID_ADDRESS);
      expect(error.address).toBe('invalid-address');
      expect(error.field).toBe('address');
    });

    it('should include reason', () => {
      const error = new InvalidAddressError('invalid-address', 'Must be SS58 format');

      expect(error.message).toContain('Must be SS58 format');
    });

    it('should get user message', () => {
      const error = new InvalidAddressError('invalid-address');
      const message = error.getUserMessage();

      expect(message).toContain('Invalid address');
      expect(message).toContain('invalid-address');
    });
  });

  describe('InvalidAmountError', () => {
    it('should create invalid amount error', () => {
      const error = new InvalidAmountError(-100n);

      expect(error).toBeInstanceOf(InvalidAmountError);
      expect(error).toBeInstanceOf(ValidationError);
      expect(error.code).toBe(ErrorCode.INVALID_AMOUNT);
      expect(error.amount).toBe(-100n);
      expect(error.field).toBe('amount');
    });

    it('should include reason', () => {
      const error = new InvalidAmountError(-100n, 'Amount must be positive');

      expect(error.message).toContain('Amount must be positive');
    });

    it('should get user message', () => {
      const error = new InvalidAmountError(-100n);
      const message = error.getUserMessage();

      expect(message).toContain('Invalid amount');
      expect(message).toContain('positive');
    });
  });

  describe('StakingError', () => {
    it('should create staking error', () => {
      const error = new StakingError('Staking failed', { validator: 'xyz' });

      expect(error).toBeInstanceOf(StakingError);
      expect(error.code).toBe(ErrorCode.STAKING_ERROR);
      expect(error.details.validator).toBe('xyz');
    });
  });

  describe('GovernanceError', () => {
    it('should create governance error', () => {
      const error = new GovernanceError('Proposal not found', { proposalId: 123 });

      expect(error).toBeInstanceOf(GovernanceError);
      expect(error.code).toBe(ErrorCode.GOVERNANCE_ERROR);
      expect(error.details.proposalId).toBe(123);
    });
  });

  describe('ChannelError', () => {
    it('should create channel error', () => {
      const error = new ChannelError('Channel closed', 'channel-123');

      expect(error).toBeInstanceOf(ChannelError);
      expect(error.code).toBe(ErrorCode.CHANNEL_ERROR);
      expect(error.channelId).toBe('channel-123');
    });

    it('should get user message with channel ID', () => {
      const error = new ChannelError('Channel closed', 'channel-123');
      const message = error.getUserMessage();

      expect(message).toContain('channel-123');
      expect(message).toContain('Channel closed');
    });

    it('should get user message without channel ID', () => {
      const error = new ChannelError('Channel error');
      const message = error.getUserMessage();

      expect(message).toContain('Channel error');
    });
  });

  describe('NotConnectedError', () => {
    it('should create not connected error', () => {
      const error = new NotConnectedError();

      expect(error).toBeInstanceOf(NotConnectedError);
      expect(error.code).toBe(ErrorCode.NOT_CONNECTED);
      expect(error.message).toContain('not connected');
    });

    it('should get user message', () => {
      const error = new NotConnectedError();
      const message = error.getUserMessage();

      expect(message).toContain('Not connected');
      expect(message).toContain('connect()');
    });
  });

  describe('ErrorHelpers', () => {
    describe('isRetryable', () => {
      it('should identify retryable network errors', () => {
        const error = new NetworkError('Connection failed');
        const retryable = ErrorHelpers.isRetryable(error);

        expect(retryable).toBe(true);
      });

      it('should identify non-retryable validation errors', () => {
        const error = new ValidationError('Invalid input', 'field');
        const retryable = ErrorHelpers.isRetryable(error);

        expect(retryable).toBe(false);
      });

      it('should handle unknown errors', () => {
        const error = new Error('Unknown error');
        const retryable = ErrorHelpers.isRetryable(error);

        expect(retryable).toBe(false);
      });
    });

    describe('getCategory', () => {
      it('should categorize transaction errors', () => {
        const error = new TransactionError('Failed');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('transaction');
      });

      it('should categorize validation errors', () => {
        const error = new ValidationError('Invalid', 'field');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('validation');
      });

      it('should categorize network errors', () => {
        const error = new NetworkError('Failed');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('network');
      });

      it('should categorize staking errors', () => {
        const error = new StakingError('Failed');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('staking');
      });

      it('should categorize governance errors', () => {
        const error = new GovernanceError('Failed');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('governance');
      });

      it('should categorize channel errors', () => {
        const error = new ChannelError('Failed');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('channel');
      });

      it('should handle unknown errors', () => {
        const error = new Error('Unknown');
        const category = ErrorHelpers.getCategory(error);

        expect(category).toBe('unknown');
      });
    });

    describe('wrap', () => {
      it('should return EtridError as-is', () => {
        const originalError = new EtridError('Test', ErrorCode.UNKNOWN_ERROR);
        const wrapped = ErrorHelpers.wrap(originalError);

        expect(wrapped).toBe(originalError);
      });

      it('should wrap Error objects', () => {
        const error = new Error('Test error');
        const wrapped = ErrorHelpers.wrap(error);

        expect(wrapped).toBeInstanceOf(EtridError);
        expect(wrapped.message).toBe('Test error');
        expect(wrapped.code).toBe(ErrorCode.UNKNOWN_ERROR);
      });

      it('should wrap unknown types', () => {
        const wrapped = ErrorHelpers.wrap('string error');

        expect(wrapped).toBeInstanceOf(EtridError);
        expect(wrapped.code).toBe(ErrorCode.UNKNOWN_ERROR);
      });

      it('should use default message', () => {
        const wrapped = ErrorHelpers.wrap({}, 'Custom default');

        expect(wrapped.message).toBe('Custom default');
      });
    });
  });

  describe('Error Inheritance', () => {
    it('should maintain prototype chain', () => {
      const error = new InvalidAddressError('test');

      expect(error instanceof InvalidAddressError).toBe(true);
      expect(error instanceof ValidationError).toBe(true);
      expect(error instanceof EtridError).toBe(true);
      expect(error instanceof Error).toBe(true);
    });

    it('should have correct names', () => {
      const errors = [
        new EtridError('test', ErrorCode.UNKNOWN_ERROR),
        new TransactionError('test'),
        new ValidationError('test', 'field'),
        new NetworkError('test'),
        new InsufficientBalanceError(100n, 50n),
        new InvalidAddressError('test'),
        new InvalidAmountError(0),
        new StakingError('test'),
        new GovernanceError('test'),
        new ChannelError('test'),
        new NotConnectedError(),
      ];

      expect(errors[0].name).toBe('EtridError');
      expect(errors[1].name).toBe('TransactionError');
      expect(errors[2].name).toBe('ValidationError');
      expect(errors[3].name).toBe('NetworkError');
      expect(errors[4].name).toBe('InsufficientBalanceError');
      expect(errors[5].name).toBe('InvalidAddressError');
      expect(errors[6].name).toBe('InvalidAmountError');
      expect(errors[7].name).toBe('StakingError');
      expect(errors[8].name).toBe('GovernanceError');
      expect(errors[9].name).toBe('ChannelError');
      expect(errors[10].name).toBe('NotConnectedError');
    });
  });
});
