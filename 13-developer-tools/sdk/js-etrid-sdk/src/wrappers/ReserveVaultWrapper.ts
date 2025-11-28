import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Vault identifier (user's account address)
 */
export type VaultId = string;

/**
 * Collateral asset identifier
 */
export type AssetId = string;

/**
 * Vault health status
 */
export enum VaultStatus {
  /** Vault is healthy with sufficient collateral */
  Healthy = 'Healthy',
  /** Vault is at risk, approaching liquidation threshold */
  AtRisk = 'AtRisk',
  /** Vault is subject to liquidation */
  Liquidatable = 'Liquidatable',
  /** Vault has been liquidated */
  Liquidated = 'Liquidated',
}

/**
 * Collateral position in vault
 */
export interface CollateralPosition {
  /** Asset identifier */
  assetId: AssetId;
  /** Amount deposited */
  amount: bigint;
  /** USD value of collateral */
  valueUSD: bigint;
  /** Collateral weight (0-10000, where 10000 = 100%) */
  weight: number;
  /** Liquidation threshold for this asset */
  liquidationThreshold: number;
}

/**
 * Vault balance and state information
 */
export interface VaultBalance {
  /** Vault owner address */
  owner: VaultId;
  /** Total collateral value in USD */
  totalCollateralUSD: bigint;
  /** Total debt value in USD */
  totalDebtUSD: bigint;
  /** Collateral ratio (percentage * 100) */
  collateralRatio: number;
  /** Minimum required collateral ratio */
  minCollateralRatio: number;
  /** Liquidation threshold */
  liquidationThreshold: number;
  /** Vault health status */
  status: VaultStatus;
  /** Individual collateral positions */
  positions: CollateralPosition[];
  /** Available to borrow (USD) */
  availableToBorrow: bigint;
  /** Available to withdraw (USD) */
  availableToWithdraw: bigint;
}

/**
 * Collateral deposit result
 */
export interface DepositResult {
  /** Transaction hash */
  txHash: string;
  /** Asset deposited */
  assetId: AssetId;
  /** Amount deposited */
  amount: bigint;
  /** New vault balance */
  newBalance: VaultBalance;
}

/**
 * Collateral withdrawal result
 */
export interface WithdrawResult {
  /** Transaction hash */
  txHash: string;
  /** Asset withdrawn */
  assetId: AssetId;
  /** Amount withdrawn */
  amount: bigint;
  /** New vault balance */
  newBalance: VaultBalance;
}

/**
 * Borrow operation result
 */
export interface BorrowResult {
  /** Transaction hash */
  txHash: string;
  /** Asset borrowed */
  assetId: AssetId;
  /** Amount borrowed */
  amount: bigint;
  /** New debt amount */
  newDebt: bigint;
  /** New collateral ratio */
  newCollateralRatio: number;
}

/**
 * Repay operation result
 */
export interface RepayResult {
  /** Transaction hash */
  txHash: string;
  /** Asset repaid */
  assetId: AssetId;
  /** Amount repaid */
  amount: bigint;
  /** Remaining debt */
  remainingDebt: bigint;
  /** New collateral ratio */
  newCollateralRatio: number;
}

/**
 * Liquidation information
 */
export interface LiquidationInfo {
  /** Vault being liquidated */
  vaultId: VaultId;
  /** Liquidator address */
  liquidator: string;
  /** Collateral assets seized */
  collateralSeized: CollateralPosition[];
  /** Debt repaid */
  debtRepaid: bigint;
  /** Liquidation penalty */
  penalty: bigint;
  /** Timestamp of liquidation */
  timestamp: number;
}

/**
 * Supported collateral asset information
 */
export interface CollateralAsset {
  /** Asset identifier */
  assetId: AssetId;
  /** Asset symbol (e.g., "ETR", "BTC", "ETH") */
  symbol: string;
  /** Asset name */
  name: string;
  /** Decimals */
  decimals: number;
  /** Maximum weight in vault (0-10000) */
  maxWeight: number;
  /** Liquidation threshold (0-10000) */
  liquidationThreshold: number;
  /** Loan-to-value ratio (0-10000) */
  ltvRatio: number;
  /** Interest rate (annual percentage, basis points) */
  interestRate: number;
  /** Current price in USD */
  priceUSD: bigint;
}

/**
 * Vault statistics
 */
export interface VaultStats {
  /** Total vaults in system */
  totalVaults: number;
  /** Total collateral value (USD) */
  totalCollateralUSD: bigint;
  /** Total debt value (USD) */
  totalDebtUSD: bigint;
  /** System-wide collateral ratio */
  systemCollateralRatio: number;
  /** Number of vaults at risk */
  vaultsAtRisk: number;
  /** Number of liquidatable vaults */
  liquidatableVaults: number;
}

/**
 * Custom errors for Reserve Vault operations
 */
export class VaultError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'VaultError';
  }
}

export class InsufficientCollateralError extends VaultError {
  constructor(required: bigint, available: bigint) {
    super(`Insufficient collateral: required ${required}, available ${available}`);
    this.name = 'InsufficientCollateralError';
  }
}

export class CollateralRatioError extends VaultError {
  constructor(current: number, minimum: number) {
    super(`Collateral ratio too low: ${current}% < ${minimum}%`);
    this.name = 'CollateralRatioError';
  }
}

export class LiquidationError extends VaultError {
  constructor(message: string) {
    super(message);
    this.name = 'LiquidationError';
  }
}

export class UnsupportedAssetError extends VaultError {
  constructor(assetId: AssetId) {
    super(`Asset not supported as collateral: ${assetId}`);
    this.name = 'UnsupportedAssetError';
  }
}

/**
 * ReserveVaultWrapper
 *
 * Wrapper for the pallet-reserve-vault module.
 *
 * Manages collateralized vaults for DeFi lending/borrowing on Ëtrid.
 * Users can deposit multiple assets as collateral and borrow against them,
 * with automatic liquidation if collateral ratios fall below thresholds.
 *
 * **Features:**
 * - Multi-asset collateral support
 * - Over-collateralized lending
 * - Automatic liquidation protection
 * - Real-time collateral ratio monitoring
 * - Interest accrual on borrowed assets
 * - Flash loan support (future)
 *
 * **Key Concepts:**
 * - **Collateral Ratio**: Total collateral value / Total debt value * 100
 * - **LTV (Loan-to-Value)**: Maximum borrowable amount per unit collateral
 * - **Liquidation Threshold**: Minimum collateral ratio before liquidation
 * - **Health Factor**: Distance from liquidation (>1 = safe, <1 = at risk)
 *
 * @example
 * ```typescript
 * import { ApiPromise } from '@polkadot/api';
 * import { Keyring } from '@polkadot/keyring';
 * import { ReserveVaultWrapper } from '@etrid/sdk';
 *
 * const api = await ApiPromise.create({ provider });
 * const keyring = new Keyring({ type: 'sr25519' });
 * const alice = keyring.addFromUri('//Alice');
 * const vault = new ReserveVaultWrapper(api);
 *
 * // Deposit collateral
 * const deposit = await vault.depositCollateral(
 *   alice,
 *   'ETR',
 *   1000n * 10n**18n // 1000 ETR
 * );
 * console.log(`Deposited, new balance: $${deposit.newBalance.totalCollateralUSD}`);
 *
 * // Check vault health
 * const balance = await vault.getVaultBalance(alice.address);
 * console.log(`Collateral ratio: ${balance.collateralRatio / 100}%`);
 * console.log(`Status: ${balance.status}`);
 *
 * // Borrow against collateral
 * const borrow = await vault.borrow(
 *   alice,
 *   'USDT',
 *   500n * 10n**6n // 500 USDT
 * );
 * console.log(`Borrowed 500 USDT, new ratio: ${borrow.newCollateralRatio / 100}%`);
 *
 * // Monitor collateral ratio
 * const ratio = await vault.getCollateralRatio(alice.address);
 * if (ratio.status === 'AtRisk') {
 *   console.warn('Warning: Vault at risk of liquidation!');
 * }
 * ```
 */
export class ReserveVaultWrapper {
  /**
   * Creates a ReserveVaultWrapper instance
   * @param api - Connected Polkadot.js API instance
   */
  constructor(private api: ApiPromise) {}

  /**
   * Deposit collateral into vault
   *
   * Deposits an asset as collateral, increasing the vault's borrowing capacity.
   *
   * @param signer - Account keypair for signing transaction
   * @param assetId - Asset identifier to deposit
   * @param amount - Amount to deposit (in smallest unit)
   * @returns Deposit result with new vault balance
   * @throws {UnsupportedAssetError} If asset is not supported as collateral
   * @throws {VaultError} If deposit fails
   *
   * @example
   * ```typescript
   * // Deposit 1000 ETR
   * const result = await vault.depositCollateral(
   *   signer,
   *   'ETR',
   *   1000n * 10n**18n
   * );
   *
   * console.log(`Deposited ${result.amount} of ${result.assetId}`);
   * console.log(`New collateral: $${result.newBalance.totalCollateralUSD}`);
   * console.log(`Can borrow up to: $${result.newBalance.availableToBorrow}`);
   * ```
   */
  async depositCollateral(
    signer: KeyringPair,
    assetId: AssetId,
    amount: bigint
  ): Promise<DepositResult> {
    // Verify asset is supported
    const isSupported = await this.isAssetSupported(assetId);
    if (!isSupported) {
      throw new UnsupportedAssetError(assetId);
    }

    return new Promise((resolve, reject) => {
      this.api.tx.reserveVault
        .depositCollateral(assetId, amount.toString())
        .signAndSend(signer, ({ status, events, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = this.api.registry.findMetaError(dispatchError.asModule);
              reject(new VaultError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
            } else {
              reject(new VaultError(dispatchError.toString()));
            }
            return;
          }

          if (status.isInBlock || status.isFinalized) {
            const depositEvent = events.find(({ event }) =>
              this.api.events.reserveVault.CollateralDeposited.is(event)
            );

            if (depositEvent) {
              const [owner, asset, depositedAmount] = depositEvent.event.data;

              // Get new vault balance
              this.getVaultBalance(signer.address)
                .then(newBalance => {
                  resolve({
                    txHash: status.asInBlock.toString(),
                    assetId: asset.toString(),
                    amount: BigInt(depositedAmount.toString()),
                    newBalance,
                  });
                })
                .catch(reject);
            } else {
              reject(new VaultError('Deposit event not found in transaction'));
            }
          }
        });
    });
  }

  /**
   * Withdraw collateral from vault
   *
   * Withdraws collateral, reducing borrowing capacity. Withdrawal will fail
   * if it would bring the collateral ratio below the minimum threshold.
   *
   * @param signer - Account keypair for signing transaction
   * @param assetId - Asset identifier to withdraw
   * @param amount - Amount to withdraw (in smallest unit)
   * @returns Withdrawal result with new vault balance
   * @throws {InsufficientCollateralError} If withdrawal would violate collateral ratio
   * @throws {VaultError} If withdrawal fails
   *
   * @example
   * ```typescript
   * // Withdraw 100 ETR
   * const result = await vault.withdrawCollateral(
   *   signer,
   *   'ETR',
   *   100n * 10n**18n
   * );
   *
   * console.log(`Withdrew ${result.amount} of ${result.assetId}`);
   * console.log(`New collateral ratio: ${result.newBalance.collateralRatio / 100}%`);
   * ```
   */
  async withdrawCollateral(
    signer: KeyringPair,
    assetId: AssetId,
    amount: bigint
  ): Promise<WithdrawResult> {
    return new Promise((resolve, reject) => {
      this.api.tx.reserveVault
        .withdrawCollateral(assetId, amount.toString())
        .signAndSend(signer, ({ status, events, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = this.api.registry.findMetaError(dispatchError.asModule);

              // Check for specific errors
              if (decoded.name === 'InsufficientCollateral') {
                reject(new InsufficientCollateralError(amount, 0n)); // Amount from error
              } else if (decoded.name === 'CollateralRatioTooLow') {
                reject(new CollateralRatioError(0, 0)); // Ratios from error
              } else {
                reject(new VaultError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
              }
            } else {
              reject(new VaultError(dispatchError.toString()));
            }
            return;
          }

          if (status.isInBlock || status.isFinalized) {
            const withdrawEvent = events.find(({ event }) =>
              this.api.events.reserveVault.CollateralWithdrawn.is(event)
            );

            if (withdrawEvent) {
              const [owner, asset, withdrawnAmount] = withdrawEvent.event.data;

              this.getVaultBalance(signer.address)
                .then(newBalance => {
                  resolve({
                    txHash: status.asInBlock.toString(),
                    assetId: asset.toString(),
                    amount: BigInt(withdrawnAmount.toString()),
                    newBalance,
                  });
                })
                .catch(reject);
            } else {
              reject(new VaultError('Withdrawal event not found in transaction'));
            }
          }
        });
    });
  }

  /**
   * Get vault balance and state
   *
   * Retrieves comprehensive information about a vault including all
   * collateral positions, debt, and health metrics.
   *
   * @param vaultId - Vault owner's address
   * @returns Vault balance and state information
   * @throws {VaultError} If vault query fails
   *
   * @example
   * ```typescript
   * const balance = await vault.getVaultBalance(userAddress);
   *
   * console.log(`Total Collateral: $${balance.totalCollateralUSD}`);
   * console.log(`Total Debt: $${balance.totalDebtUSD}`);
   * console.log(`Collateral Ratio: ${balance.collateralRatio / 100}%`);
   * console.log(`Status: ${balance.status}`);
   *
   * console.log('Positions:');
   * balance.positions.forEach(pos => {
   *   console.log(`  ${pos.assetId}: ${pos.amount} ($${pos.valueUSD})`);
   * });
   *
   * if (balance.status === 'AtRisk') {
   *   console.warn('Warning: Add collateral or repay debt!');
   * }
   * ```
   */
  async getVaultBalance(vaultId: VaultId): Promise<VaultBalance> {
    try {
      const vaultData = await this.api.query.reserveVault.vaults(vaultId);

      if (vaultData.isNone) {
        // Return empty vault for new users
        return {
          owner: vaultId,
          totalCollateralUSD: 0n,
          totalDebtUSD: 0n,
          collateralRatio: 0,
          minCollateralRatio: 15000, // 150%
          liquidationThreshold: 12000, // 120%
          status: VaultStatus.Healthy,
          positions: [],
          availableToBorrow: 0n,
          availableToWithdraw: 0n,
        };
      }

      const vault = vaultData.unwrap();
      const totalCollateralUSD = BigInt(vault.totalCollateral.toString());
      const totalDebtUSD = BigInt(vault.totalDebt.toString());
      const minCollateralRatio = Number(vault.minCollateralRatio.toString());
      const liquidationThreshold = Number(vault.liquidationThreshold.toString());

      // Calculate collateral ratio
      let collateralRatio = 0;
      if (totalDebtUSD > 0n) {
        collateralRatio = Number((totalCollateralUSD * 10000n) / totalDebtUSD);
      }

      // Determine vault status
      let status = VaultStatus.Healthy;
      if (collateralRatio > 0 && collateralRatio < liquidationThreshold) {
        status = VaultStatus.Liquidatable;
      } else if (collateralRatio > 0 && collateralRatio < minCollateralRatio) {
        status = VaultStatus.AtRisk;
      }

      // Get individual positions
      const positions: CollateralPosition[] = [];
      const positionsData = vault.positions;
      for (const posData of positionsData) {
        positions.push({
          assetId: posData.assetId.toString(),
          amount: BigInt(posData.amount.toString()),
          valueUSD: BigInt(posData.valueUSD.toString()),
          weight: Number(posData.weight.toString()),
          liquidationThreshold: Number(posData.liquidationThreshold.toString()),
        });
      }

      // Calculate available to borrow and withdraw
      const availableToBorrow = this.calculateAvailableToBorrow(
        totalCollateralUSD,
        totalDebtUSD,
        minCollateralRatio
      );
      const availableToWithdraw = this.calculateAvailableToWithdraw(
        totalCollateralUSD,
        totalDebtUSD,
        minCollateralRatio
      );

      return {
        owner: vaultId,
        totalCollateralUSD,
        totalDebtUSD,
        collateralRatio,
        minCollateralRatio,
        liquidationThreshold,
        status,
        positions,
        availableToBorrow,
        availableToWithdraw,
      };
    } catch (error) {
      throw new VaultError(`Failed to get vault balance: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Get collateral ratio for a vault
   *
   * Quick method to check vault health without fetching full balance.
   *
   * @param vaultId - Vault owner's address
   * @returns Collateral ratio and status
   *
   * @example
   * ```typescript
   * const ratio = await vault.getCollateralRatio(userAddress);
   * console.log(`Ratio: ${ratio.collateralRatio / 100}%`);
   * console.log(`Status: ${ratio.status}`);
   * ```
   */
  async getCollateralRatio(vaultId: VaultId): Promise<{
    collateralRatio: number;
    status: VaultStatus;
  }> {
    const balance = await this.getVaultBalance(vaultId);
    return {
      collateralRatio: balance.collateralRatio,
      status: balance.status,
    };
  }

  /**
   * Borrow assets against collateral
   *
   * Borrows an asset against deposited collateral, increasing vault debt.
   *
   * @param signer - Account keypair for signing transaction
   * @param assetId - Asset to borrow
   * @param amount - Amount to borrow
   * @returns Borrow result with updated debt
   * @throws {InsufficientCollateralError} If borrow would violate collateral ratio
   *
   * @example
   * ```typescript
   * const result = await vault.borrow(
   *   signer,
   *   'USDT',
   *   1000n * 10n**6n // Borrow 1000 USDT
   * );
   * console.log(`New debt: ${result.newDebt}`);
   * console.log(`New ratio: ${result.newCollateralRatio / 100}%`);
   * ```
   */
  async borrow(
    signer: KeyringPair,
    assetId: AssetId,
    amount: bigint
  ): Promise<BorrowResult> {
    return new Promise((resolve, reject) => {
      this.api.tx.reserveVault
        .borrow(assetId, amount.toString())
        .signAndSend(signer, ({ status, events, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = this.api.registry.findMetaError(dispatchError.asModule);
              reject(new VaultError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
            } else {
              reject(new VaultError(dispatchError.toString()));
            }
            return;
          }

          if (status.isInBlock || status.isFinalized) {
            const borrowEvent = events.find(({ event }) =>
              this.api.events.reserveVault.AssetBorrowed.is(event)
            );

            if (borrowEvent) {
              const [owner, asset, borrowedAmount, newDebt] = borrowEvent.event.data;

              this.getVaultBalance(signer.address)
                .then(balance => {
                  resolve({
                    txHash: status.asInBlock.toString(),
                    assetId: asset.toString(),
                    amount: BigInt(borrowedAmount.toString()),
                    newDebt: BigInt(newDebt.toString()),
                    newCollateralRatio: balance.collateralRatio,
                  });
                })
                .catch(reject);
            } else {
              reject(new VaultError('Borrow event not found in transaction'));
            }
          }
        });
    });
  }

  /**
   * Repay borrowed assets
   *
   * Repays borrowed assets, reducing vault debt.
   *
   * @param signer - Account keypair for signing transaction
   * @param assetId - Asset to repay
   * @param amount - Amount to repay
   * @returns Repay result with updated debt
   *
   * @example
   * ```typescript
   * const result = await vault.repay(
   *   signer,
   *   'USDT',
   *   500n * 10n**6n // Repay 500 USDT
   * );
   * console.log(`Remaining debt: ${result.remainingDebt}`);
   * ```
   */
  async repay(
    signer: KeyringPair,
    assetId: AssetId,
    amount: bigint
  ): Promise<RepayResult> {
    return new Promise((resolve, reject) => {
      this.api.tx.reserveVault
        .repay(assetId, amount.toString())
        .signAndSend(signer, ({ status, events, dispatchError }) => {
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = this.api.registry.findMetaError(dispatchError.asModule);
              reject(new VaultError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
            } else {
              reject(new VaultError(dispatchError.toString()));
            }
            return;
          }

          if (status.isInBlock || status.isFinalized) {
            const repayEvent = events.find(({ event }) =>
              this.api.events.reserveVault.DebtRepaid.is(event)
            );

            if (repayEvent) {
              const [owner, asset, repaidAmount, remainingDebt] = repayEvent.event.data;

              this.getVaultBalance(signer.address)
                .then(balance => {
                  resolve({
                    txHash: status.asInBlock.toString(),
                    assetId: asset.toString(),
                    amount: BigInt(repaidAmount.toString()),
                    remainingDebt: BigInt(remainingDebt.toString()),
                    newCollateralRatio: balance.collateralRatio,
                  });
                })
                .catch(reject);
            } else {
              reject(new VaultError('Repay event not found in transaction'));
            }
          }
        });
    });
  }

  /**
   * Get supported collateral assets
   *
   * @returns Array of supported collateral assets with metadata
   *
   * @example
   * ```typescript
   * const assets = await vault.getSupportedAssets();
   * assets.forEach(asset => {
   *   console.log(`${asset.symbol}: LTV ${asset.ltvRatio / 100}%, threshold ${asset.liquidationThreshold / 100}%`);
   * });
   * ```
   */
  async getSupportedAssets(): Promise<CollateralAsset[]> {
    try {
      const assetsData = await this.api.query.reserveVault.supportedAssets();
      const assets: CollateralAsset[] = [];

      for (const assetData of assetsData) {
        const asset = assetData as any;
        assets.push({
          assetId: asset.assetId.toString(),
          symbol: asset.symbol.toString(),
          name: asset.name.toString(),
          decimals: Number(asset.decimals.toString()),
          maxWeight: Number(asset.maxWeight.toString()),
          liquidationThreshold: Number(asset.liquidationThreshold.toString()),
          ltvRatio: Number(asset.ltvRatio.toString()),
          interestRate: Number(asset.interestRate.toString()),
          priceUSD: BigInt(asset.priceUSD.toString()),
        });
      }

      return assets;
    } catch (error) {
      throw new VaultError(`Failed to get supported assets: ${error instanceof Error ? error.message : String(error)}`);
    }
  }

  /**
   * Check if an asset is supported as collateral
   *
   * @param assetId - Asset identifier
   * @returns True if asset is supported
   */
  async isAssetSupported(assetId: AssetId): Promise<boolean> {
    try {
      const assetInfo = await this.api.query.reserveVault.assets(assetId);
      return assetInfo.isSome;
    } catch (error) {
      return false;
    }
  }

  /**
   * Calculate available amount to borrow
   * @private
   */
  private calculateAvailableToBorrow(
    collateralUSD: bigint,
    debtUSD: bigint,
    minRatio: number
  ): bigint {
    if (collateralUSD === 0n) return 0n;
    const maxDebt = (collateralUSD * 10000n) / BigInt(minRatio);
    if (maxDebt <= debtUSD) return 0n;
    return maxDebt - debtUSD;
  }

  /**
   * Calculate available amount to withdraw
   * @private
   */
  private calculateAvailableToWithdraw(
    collateralUSD: bigint,
    debtUSD: bigint,
    minRatio: number
  ): bigint {
    if (debtUSD === 0n) return collateralUSD;
    const requiredCollateral = (debtUSD * BigInt(minRatio)) / 10000n;
    if (requiredCollateral >= collateralUSD) return 0n;
    return collateralUSD - requiredCollateral;
  }
}
