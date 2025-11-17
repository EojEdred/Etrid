/**
 * Test Helper Utilities
 * 
 * Common utilities for SDK testing
 */

import { ApiPromise } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Create mock API instance for testing
 */
export function createMockApi(): jest.Mocked<ApiPromise> {
  return {
    query: {},
    tx: {},
    rpc: {
      system: {
        chain: jest.fn().mockResolvedValue({ toString: () => 'Ëtrid FlareChain' }),
      },
    },
    events: {},
    registry: {
      findMetaError: jest.fn(),
    },
    disconnect: jest.fn(),
  } as any;
}

/**
 * Create test accounts using development keyring
 */
export function createTestAccounts(): {
  alice: KeyringPair;
  bob: KeyringPair;
  charlie: KeyringPair;
} {
  const keyring = new Keyring({ type: 'sr25519' });
  
  return {
    alice: keyring.addFromUri('//Alice'),
    bob: keyring.addFromUri('//Bob'),
    charlie: keyring.addFromUri('//Charlie'),
  };
}

/**
 * Mock transaction result
 */
export function mockTxResult(
  status: 'InBlock' | 'Finalized' | 'Error',
  events: any[] = []
) {
  return {
    status: {
      isInBlock: status === 'InBlock',
      isFinalized: status === 'Finalized',
      asInBlock: { toString: () => '0x1234567890abcdef' },
    },
    events,
    dispatchError: status === 'Error' ? { isModule: false } : undefined,
  };
}

/**
 * Mock event data
 */
export function mockEvent(eventName: string, data: any[]) {
  return {
    event: {
      data,
      section: 'test',
      method: eventName,
    },
  };
}

/**
 * Wait for async operations
 */
export async function waitFor(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Mock promise that resolves/rejects based on condition
 */
export function mockConditionalPromise<T>(
  condition: boolean,
  resolveValue: T,
  rejectReason?: any
): Promise<T> {
  return condition
    ? Promise.resolve(resolveValue)
    : Promise.reject(rejectReason || new Error('Mock rejection'));
}

/**
 * Generate random BigInt for testing
 */
export function randomBigInt(max: bigint = 1000000n): bigint {
  return BigInt(Math.floor(Math.random() * Number(max)));
}

/**
 * Convert to chain format (string)
 */
export function toChainFormat(value: bigint): string {
  return value.toString();
}

/**
 * Mock query result with Option wrapper
 */
export function mockQueryResult<T>(value: T | null) {
  if (value === null) {
    return {
      isNone: true,
      isSome: false,
      unwrap: () => {
        throw new Error('Called unwrap on None');
      },
    };
  }
  
  return {
    isNone: false,
    isSome: true,
    unwrap: () => value,
  };
}

/**
 * Assert transaction success
 */
export function assertTxSuccess(result: any): void {
  expect(result).toBeDefined();
  expect(result.status?.isInBlock || result.status?.isFinalized).toBeTruthy();
  expect(result.dispatchError).toBeUndefined();
}

/**
 * Assert BigInt equality with tolerance
 */
export function assertBigIntNear(
  actual: bigint,
  expected: bigint,
  tolerance: bigint = 100n
): void {
  const diff = actual > expected ? actual - expected : expected - actual;
  expect(diff).toBeLessThanOrEqual(tolerance);
}
