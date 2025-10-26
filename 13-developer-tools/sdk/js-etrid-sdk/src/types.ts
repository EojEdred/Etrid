/**
 * Common types used throughout the SDK
 */

/**
 * Account balance information
 */
export interface Balance {
  /** Free balance */
  free: bigint;
  /** Reserved balance */
  reserved: bigint;
  /** Frozen balance */
  frozen: bigint;
}

/**
 * Block information
 */
export interface Block {
  /** Block number */
  number: number;
  /** Block hash */
  hash: string;
  /** Parent hash */
  parentHash: string;
  /** State root */
  stateRoot: string;
}

/**
 * Transaction hash
 */
export type TxHash = string;

/**
 * Account address (SS58 encoded)
 */
export type Address = string;
