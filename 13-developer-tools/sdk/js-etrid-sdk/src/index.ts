/**
 * Ëtrid SDK for JavaScript/TypeScript
 *
 * Provides a comprehensive interface to interact with Ëtrid Protocol blockchain.
 *
 * @example
 * ```typescript
 * import { EtridClient, Account } from '@etrid/sdk';
 *
 * const client = new EtridClient('ws://localhost:9944');
 * await client.connect();
 *
 * const account = Account.fromMnemonic('word1 word2 ...');
 * const balance = await client.query.balance(account.address);
 * console.log(`Balance: ${balance.free} ETR`);
 * ```
 *
 * @packageDocumentation
 */

export { EtridClient } from './client';
export { Account } from './account';
export * from './types';

// Export builders
export { TransactionBuilder } from './builders/TransactionBuilder';

// Export wrappers
export { AccountsWrapper } from './wrappers/AccountsWrapper';
export { StakingWrapper } from './wrappers/StakingWrapper';
export { GovernanceWrapper } from './wrappers/GovernanceWrapper';

// Export Layer 3 wrappers
export {
  LightningBlocWrapper,
  ChannelId,
  Channel,
  ChannelState,
  ChannelStatus,
  PaymentRoute,
  ChannelBalance,
  RoutingFeeEstimate,
  ChannelUpdateResult,
  ChannelError as LightningBlocChannelError,
  InsufficientBalanceError as LightningBlocInsufficientBalanceError,
  ChannelNotFoundError,
  InvalidChannelStateError,
  RouteNotFoundError,
} from './wrappers/LightningBlocWrapper';

export {
  DistributionPayWrapper,
  DistributionCategory,
  PendingRewards,
  DistributionSchedule,
  ClaimHistory,
  DistributionEstimate,
  DistributionError,
  NotEligibleError,
  ClaimTooEarlyError,
  NoRewardsError,
} from './wrappers/DistributionPayWrapper';

export {
  EtwasmVMWrapper,
  VMw,
  CodeHash,
  ContractAddress,
  CodeUploadResult,
  InstantiateParams,
  ContractDeployment,
  CallResult,
  GasEstimate,
  ContractInfo,
  GAS_CONSTANTS,
  EtwasmError,
  ContractNotFoundError,
  InsufficientGasError,
  ContractExecutionError,
} from './wrappers/EtwasmVMWrapper';

export {
  AIDidWrapper,
  AIDID,
  AIType,
  AIProfile,
  Capabilities,
  Restrictions,
  SafetyProfile,
  Task,
  Reputation,
  InferenceMetadata,
  Permission,
  AIRegistrationResult,
  AISearchResult,
  AIDidError,
  AINotFoundError,
  InvalidProfileError,
  PermissionDeniedError,
} from './wrappers/AIDidWrapper';

export {
  BridgeWrapper,
  Chain,
  BridgeTxHash,
  BridgeStatus,
  BridgeTransaction,
  BridgeFee,
  PBCInfo,
  BridgeLimits,
  ChainMetadata,
  BridgeError,
  UnsupportedChainError,
  BridgeLimitError,
  BridgeNotAvailableError,
} from './wrappers/BridgeWrapper';

export {
  OracleWrapper,
  PricePair,
  OracleSource,
  Price,
  PriceData,
  TWAPData,
  OracleSourceInfo,
  OracleSourceStatus,
  PriceUpdateEvent,
  HistoricalPrice,
  PriceUpdateCallback,
  PriceSubscription,
  TWAPParams,
  AggregationMethod,
  PRICE_PRECISION,
  toPrice,
  fromPrice,
  OracleError,
  PriceNotFoundError,
  InsufficientDataError,
  OracleSourceError,
} from './wrappers/OracleWrapper';

export {
  ReserveVaultWrapper,
  VaultId,
  AssetId,
  VaultStatus,
  CollateralPosition,
  VaultBalance,
  DepositResult,
  WithdrawResult,
  BorrowResult,
  RepayResult,
  LiquidationInfo,
  CollateralAsset,
  VaultStats,
  VaultError,
  InsufficientCollateralError,
  CollateralRatioError,
  LiquidationError,
  UnsupportedAssetError,
} from './wrappers/ReserveVaultWrapper';

// Export errors
export {
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
} from './errors/EtridErrors';

// Export enhanced types
export * from './types/enhanced';

// Export formatters
export * from './utils/formatters';

export const VERSION = '0.1.0';
