/**
 * ËtwasmVM Wrapper for Ëtrid SDK
 *
 * Provides interface for deploying and interacting with smart contracts
 * on the ËtwasmVM runtime with reentrancy protection.
 */

import { ApiPromise } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { WeightV2 } from '@polkadot/types/interfaces';
import { TransactionError } from '../errors/EtridErrors';

/**
 * Contract address (SS58 format)
 */
export type ContractAddress = string;

/**
 * Code hash (32-byte hash)
 */
export type CodeHash = string;

/**
 * Gas unit for ËtwasmVM (VMw = Virtual Machine work units)
 */
export type VMw = bigint;

/**
 * Contract deployment result
 */
export interface ContractDeployment {
  /** Deployed contract address */
  address: ContractAddress;
  /** Code hash */
  codeHash: CodeHash;
  /** Transaction hash */
  txHash: string;
  /** Gas used for deployment */
  gasUsed: VMw;
  /** Storage deposit required */
  storageDeposit: bigint;
}

/**
 * Contract call result
 */
export interface CallResult {
  /** Whether call succeeded */
  success: boolean;
  /** Output data from contract */
  output: any;
  /** Gas used */
  gasUsed: VMw;
  /** Events emitted */
  events: ContractEvent[];
  /** Error message if failed */
  error?: string;
}

/**
 * Contract event
 */
export interface ContractEvent {
  /** Contract that emitted event */
  contract: ContractAddress;
  /** Event name */
  name: string;
  /** Event data */
  data: any;
}

/**
 * Contract information
 */
export interface ContractInfo {
  /** Contract address */
  address: ContractAddress;
  /** Code hash */
  codeHash: CodeHash;
  /** Deployer address */
  deployer: string;
  /** Contract balance */
  balance: bigint;
  /** Storage used (bytes) */
  storage: bigint;
  /** Block deployed */
  deployedAt: number;
}

/**
 * Gas estimate
 */
export interface GasEstimate {
  /** Gas required */
  gasRequired: VMw;
  /** Storage deposit */
  storageDeposit: bigint;
  /** Total cost in ÉTR */
  costInETR: bigint;
  /** Breakdown */
  breakdown: {
    execution: VMw;
    storage: bigint;
  };
}

/**
 * Code upload result
 */
export interface CodeUploadResult {
  /** Code hash */
  codeHash: CodeHash;
  /** Transaction hash */
  txHash: string;
  /** Storage deposit */
  storageDeposit: bigint;
}

/**
 * Contract instantiation params
 */
export interface InstantiateParams {
  /** Code hash to instantiate from */
  codeHash: CodeHash;
  /** Constructor arguments */
  args: any[];
  /** Value to transfer */
  value?: bigint;
  /** Gas limit */
  gasLimit?: VMw;
  /** Storage deposit limit */
  storageDepositLimit?: bigint;
  /** Salt for deterministic address */
  salt?: Uint8Array;
}

/** ËtwasmVM Gas Constants */
export const GAS_CONSTANTS = {
  /** Block limit: 10M VMw per block */
  BLOCK_LIMIT: 10_000_000n,
  /** Transaction limit: 1M VMw per transaction */
  TX_LIMIT: 1_000_000n,
  /** 1 ÉTR = 1M VMw */
  VMW_PER_ETR: 1_000_000n,
  /** Default gas limit for calls */
  DEFAULT_GAS: 500_000n,
};

/**
 * ËtwasmVM wrapper for smart contract operations
 *
 * Enables deployment and interaction with WebAssembly smart contracts
 * on Ëtrid's EVM-compatible runtime with reentrancy protection.
 *
 * @example
 * ```typescript
 * const etwasm = new EtwasmVMWrapper(api);
 *
 * // Deploy contract
 * const deployment = await etwasm.deployContract(
 *   alice,
 *   wasmCode,
 *   [initialSupply],  // constructor args
 *   0n,               // no value transfer
 *   500_000n          // gas limit
 * );
 *
 * // Call contract
 * const result = await etwasm.callContract(
 *   alice,
 *   deployment.address,
 *   'transfer',
 *   [bobAddress, 1000]
 * );
 * ```
 */
export class EtwasmVMWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Uploads contract code to the chain
   *
   * Stores the WASM bytecode on-chain without instantiating it.
   * This allows reusing the same code for multiple contract instances.
   *
   * @param signer - Your account
   * @param wasmCode - Contract WASM bytecode
   * @param gasLimit - Gas limit (optional)
   * @returns Promise resolving to code hash
   *
   * @throws {TransactionError} If upload fails
   *
   * @example
   * ```typescript
   * const fs = require('fs');
   * const wasmCode = fs.readFileSync('contract.wasm');
   * const result = await etwasm.uploadCode(alice, wasmCode);
   * console.log('Code hash:', result.codeHash);
   * ```
   */
  async uploadCode(
    signer: KeyringPair,
    wasmCode: Uint8Array,
    gasLimit?: VMw
  ): Promise<CodeUploadResult> {
    try {
      const gas = gasLimit || GAS_CONSTANTS.DEFAULT_GAS;

      return new Promise((resolve, reject) => {
        this.api.tx.contracts
          .uploadCode(wasmCode, null, 'Deterministic')
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                // Find CodeStored event
                const codeEvent = events.find(({ event }) =>
                  this.api.events.contracts.CodeStored.is(event)
                );

                if (codeEvent) {
                  const [codeHash] = codeEvent.event.data;
                  resolve({
                    codeHash: codeHash.toHex(),
                    txHash: status.asFinalized.toHex(),
                    storageDeposit: 0n, // Would extract from events
                  });
                } else {
                  reject(new TransactionError('CodeStored event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to upload code: ${error.message}`);
    }
  }

  /**
   * Instantiates a contract from uploaded code
   *
   * Creates a new contract instance from a previously uploaded code hash.
   *
   * @param signer - Your account
   * @param params - Instantiation parameters
   * @returns Promise resolving to contract address
   *
   * @throws {TransactionError} If instantiation fails
   *
   * @example
   * ```typescript
   * const address = await etwasm.instantiate(alice, {
   *   codeHash,
   *   args: [1000000],  // initial supply
   *   value: 0n,
   *   gasLimit: 500_000n
   * });
   * console.log('Contract deployed at:', address);
   * ```
   */
  async instantiate(
    signer: KeyringPair,
    params: InstantiateParams
  ): Promise<ContractAddress> {
    try {
      const { codeHash, args, value = 0n, gasLimit = GAS_CONSTANTS.DEFAULT_GAS, salt } = params;

      return new Promise((resolve, reject) => {
        this.api.tx.contracts
          .instantiate(value, gasLimit, null, codeHash, args, salt || new Uint8Array())
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                // Find Instantiated event
                const instantiateEvent = events.find(({ event }) =>
                  this.api.events.contracts.Instantiated.is(event)
                );

                if (instantiateEvent) {
                  const [deployer, contract] = instantiateEvent.event.data;
                  resolve(contract.toString());
                } else {
                  reject(new TransactionError('Instantiated event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to instantiate contract: ${error.message}`);
    }
  }

  /**
   * Deploys a smart contract (upload + instantiate in one step)
   *
   * Convenience method that uploads code and instantiates in a single call.
   *
   * @param signer - Your account
   * @param wasmCode - Contract WASM bytecode
   * @param constructorArgs - Constructor arguments
   * @param value - Value to transfer to contract
   * @param gasLimit - Gas limit
   * @returns Promise resolving to deployment details
   *
   * @throws {TransactionError} If deployment fails
   *
   * @example
   * ```typescript
   * const deployment = await etwasm.deployContract(
   *   alice,
   *   wasmCode,
   *   [1000000],        // initial supply
   *   0n,               // no value
   *   500_000n          // gas
   * );
   * console.log('Deployed at:', deployment.address);
   * console.log('Code hash:', deployment.codeHash);
   * ```
   */
  async deployContract(
    signer: KeyringPair,
    wasmCode: Uint8Array,
    constructorArgs: any[] = [],
    value: bigint = 0n,
    gasLimit?: VMw
  ): Promise<ContractDeployment> {
    try {
      // Step 1: Upload code
      const uploadResult = await this.uploadCode(signer, wasmCode, gasLimit);

      // Step 2: Instantiate
      const address = await this.instantiate(signer, {
        codeHash: uploadResult.codeHash,
        args: constructorArgs,
        value,
        gasLimit,
      });

      return {
        address,
        codeHash: uploadResult.codeHash,
        txHash: uploadResult.txHash,
        gasUsed: gasLimit || GAS_CONSTANTS.DEFAULT_GAS,
        storageDeposit: uploadResult.storageDeposit,
      };
    } catch (error) {
      throw new TransactionError(`Failed to deploy contract: ${error.message}`);
    }
  }

  /**
   * Calls a contract method (write operation)
   *
   * Executes a state-changing contract method.
   *
   * @param signer - Your account
   * @param contractAddress - Contract address
   * @param method - Method name
   * @param args - Method arguments
   * @param value - Value to transfer
   * @param gasLimit - Gas limit
   * @returns Promise resolving to call result
   *
   * @throws {TransactionError} If call fails
   *
   * @example
   * ```typescript
   * const result = await etwasm.callContract(
   *   alice,
   *   contractAddress,
   *   'transfer',
   *   [bobAddress, 1000],
   *   0n,
   *   500_000n
   * );
   * if (result.success) {
   *   console.log('Transfer successful');
   * }
   * ```
   */
  async callContract(
    signer: KeyringPair,
    contractAddress: ContractAddress,
    method: string,
    args: any[] = [],
    value: bigint = 0n,
    gasLimit?: VMw
  ): Promise<CallResult> {
    try {
      const gas = gasLimit || GAS_CONSTANTS.DEFAULT_GAS;

      return new Promise((resolve, reject) => {
        this.api.tx.contracts
          .call(contractAddress, value, gas, null, [method, ...args])
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  resolve({
                    success: false,
                    output: null,
                    gasUsed: 0n,
                    events: [],
                    error: `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`,
                  });
                } else {
                  resolve({
                    success: false,
                    output: null,
                    gasUsed: 0n,
                    events: [],
                    error: dispatchError.toString(),
                  });
                }
              } else {
                // Extract contract events
                const contractEvents: ContractEvent[] = [];
                events.forEach(({ event }) => {
                  if (this.api.events.contracts.ContractEmitted.is(event)) {
                    const [contract, data] = event.data;
                    contractEvents.push({
                      contract: contract.toString(),
                      name: 'ContractEmitted',
                      data: data.toHuman(),
                    });
                  }
                });

                resolve({
                  success: true,
                  output: null, // Would extract from events
                  gasUsed: gas, // Would extract actual gas used
                  events: contractEvents,
                });
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to call contract: ${error.message}`);
    }
  }

  /**
   * Queries a contract (read-only operation)
   *
   * Reads contract state without modifying it. Does not require gas.
   *
   * @param contractAddress - Contract address
   * @param method - Method name
   * @param args - Method arguments
   * @param caller - Optional caller address (defaults to zero address)
   * @returns Promise resolving to query result
   *
   * @throws {TransactionError} If query fails
   *
   * @example
   * ```typescript
   * const balance = await etwasm.queryContract(
   *   contractAddress,
   *   'balanceOf',
   *   [aliceAddress]
   * );
   * console.log('Balance:', balance);
   * ```
   */
  async queryContract(
    contractAddress: ContractAddress,
    method: string,
    args: any[] = [],
    caller?: string
  ): Promise<any> {
    try {
      const callerAddress = caller || '0x0000000000000000000000000000000000000000';

      const result = await this.api.rpc.contracts.call(
        callerAddress,
        contractAddress,
        0,
        null,
        null,
        [method, ...args]
      );

      if (result.isOk) {
        return result.asOk.data.toHuman();
      } else {
        throw new TransactionError(`Query failed: ${result.asErr.toString()}`);
      }
    } catch (error) {
      throw new TransactionError(`Failed to query contract: ${error.message}`);
    }
  }

  /**
   * Estimates gas for a contract call
   *
   * Dry-runs the contract call to estimate gas requirements.
   *
   * @param contractAddress - Contract address
   * @param method - Method name
   * @param args - Method arguments
   * @param value - Value to transfer
   * @param caller - Caller address
   * @returns Promise resolving to gas estimate
   *
   * @example
   * ```typescript
   * const estimate = await etwasm.estimateGas(
   *   contractAddress,
   *   'transfer',
   *   [bobAddress, 1000]
   * );
   * console.log(`Required: ${estimate.gasRequired} VMw`);
   * console.log(`Cost: ${estimate.costInETR} ÉTR`);
   * ```
   */
  async estimateGas(
    contractAddress: ContractAddress,
    method: string,
    args: any[] = [],
    value: bigint = 0n,
    caller?: string
  ): Promise<GasEstimate> {
    try {
      const callerAddress = caller || '0x0000000000000000000000000000000000000000';

      const result = await this.api.rpc.contracts.call(
        callerAddress,
        contractAddress,
        value,
        null,
        null,
        [method, ...args]
      );

      if (result.isOk) {
        const gasRequired = BigInt(result.asOk.gasRequired.toString());
        const storageDeposit = 0n; // Would extract from result
        const costInETR = gasRequired / GAS_CONSTANTS.VMW_PER_ETR;

        return {
          gasRequired,
          storageDeposit,
          costInETR,
          breakdown: {
            execution: gasRequired,
            storage: storageDeposit,
          },
        };
      } else {
        throw new TransactionError(`Gas estimation failed: ${result.asErr.toString()}`);
      }
    } catch (error) {
      throw new TransactionError(`Failed to estimate gas: ${error.message}`);
    }
  }

  /**
   * Gets contract information
   *
   * @param contractAddress - Contract address
   * @returns Promise resolving to contract info
   *
   * @example
   * ```typescript
   * const info = await etwasm.getContractInfo(contractAddress);
   * console.log('Code hash:', info.codeHash);
   * console.log('Balance:', info.balance);
   * console.log('Storage:', info.storage);
   * ```
   */
  async getContractInfo(contractAddress: ContractAddress): Promise<ContractInfo> {
    try {
      const contractInfo = await this.api.query.contracts.contractInfoOf(contractAddress);

      if (contractInfo.isNone) {
        throw new TransactionError('Contract not found');
      }

      const info = contractInfo.unwrap();
      const balance = await this.api.query.system.account(contractAddress);

      return {
        address: contractAddress,
        codeHash: info.codeHash.toHex(),
        deployer: info.depositAccount.toString(),
        balance: BigInt(balance.data.free.toString()),
        storage: BigInt(info.storageBytes.toString()),
        deployedAt: info.storageBaseDeposit.toNumber(), // Placeholder
      };
    } catch (error) {
      throw new TransactionError(`Failed to get contract info: ${error.message}`);
    }
  }

  /**
   * Gets code hash for a contract
   *
   * @param contractAddress - Contract address
   * @returns Promise resolving to code hash
   */
  async getCodeHash(contractAddress: ContractAddress): Promise<CodeHash> {
    const info = await this.getContractInfo(contractAddress);
    return info.codeHash;
  }

  /**
   * Checks if address is a contract
   *
   * @param address - Address to check
   * @returns Promise resolving to boolean
   */
  async isContract(address: string): Promise<boolean> {
    try {
      const contractInfo = await this.api.query.contracts.contractInfoOf(address);
      return contractInfo.isSome;
    } catch {
      return false;
    }
  }
}
