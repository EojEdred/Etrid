/**
 * Transaction Signing Utilities
 *
 * Handles transaction signing using either:
 * 1. Built-in wallet (keyring)
 * 2. Polkadot.js browser extension
 */

import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import { getCurrentAccount } from './keyring';

// Try to get web3FromAddress - it may not be available in all contexts
let web3FromAddress: any = null;
try {
  web3FromAddress = require('@polkadot/extension-dapp').web3FromAddress;
} catch {
  // Extension not available
}

export interface TxResult {
  txHash: string;
  blockHash?: string;
}

export type SignerSource = 'builtin' | 'extension';

/**
 * Get the signer for a transaction
 * Prefers built-in wallet, falls back to extension
 */
export async function getSigner(address: string): Promise<{
  source: SignerSource;
  signer?: any;
  pair?: KeyringPair;
}> {
  // First, check if we have a built-in wallet unlocked
  const builtinAccount = getCurrentAccount();

  if (builtinAccount && builtinAccount.address === address) {
    return {
      source: 'builtin',
      pair: builtinAccount.pair,
    };
  }

  // Fall back to extension
  if (web3FromAddress) {
    try {
      const injector = await web3FromAddress(address);
      return {
        source: 'extension',
        signer: injector.signer,
      };
    } catch (error) {
      throw new Error('No wallet available. Please unlock your wallet or install Polkadot.js extension.');
    }
  }

  throw new Error('No wallet available. Please create or unlock your wallet.');
}

/**
 * Sign and send a transaction
 */
export async function signAndSendTx(
  api: ApiPromise,
  tx: any,
  address: string
): Promise<TxResult> {
  const { source, signer, pair } = await getSigner(address);

  return new Promise(async (resolve, reject) => {
    try {
      const signOptions = source === 'builtin' ? {} : { signer };
      const signerAccount = source === 'builtin' ? pair : address;

      const unsub = await tx.signAndSend(
        signerAccount,
        signOptions,
        ({ status, txHash, dispatchError }: any) => {
          console.log(`[ETRID] Transaction status: ${status.type}`);

          if (status.isInBlock) {
            console.log(`[ETRID] Included in block: ${status.asInBlock.toHex()}`);
          }

          if (status.isFinalized) {
            if (dispatchError) {
              let errorMsg = 'Transaction failed';

              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(dispatchError.asModule);
                errorMsg = `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`;
              }

              unsub();
              reject(new Error(errorMsg));
            } else {
              unsub();
              resolve({
                txHash: txHash.toHex(),
                blockHash: status.asFinalized.toHex(),
              });
            }
          }

          if (status.isError) {
            unsub();
            reject(new Error('Transaction failed'));
          }
        }
      );
    } catch (error) {
      reject(error);
    }
  });
}
