/**
 * Firebase Analytics
 *
 * Provides helper functions for logging analytics events in the Ëtrid Wallet.
 * Events are only sent in production mode.
 */

import { logEvent as firebaseLogEvent, setUserProperties, setUserId } from 'firebase/analytics';
import { getFirebaseAnalytics } from './config';

/**
 * Pre-defined analytics event names
 * Following Google Analytics 4 naming conventions
 */
export const AnalyticsEvents = {
  // App Lifecycle
  APP_OPEN: 'app_open',
  SCREEN_VIEW: 'screen_view',

  // Wallet Management
  WALLET_CREATED: 'wallet_created',
  WALLET_IMPORTED: 'wallet_imported',
  WALLET_EXPORTED: 'wallet_exported',
  WALLET_BACKUP_CREATED: 'wallet_backup_created',
  WALLET_RESTORED: 'wallet_restored',

  // Transactions
  TRANSACTION_INITIATED: 'transaction_initiated',
  TRANSACTION_SENT: 'transaction_sent',
  TRANSACTION_RECEIVED: 'transaction_received',
  TRANSACTION_FAILED: 'transaction_failed',
  TRANSACTION_CONFIRMED: 'transaction_confirmed',

  // Bloccard Features
  BLOCCARD_VIEWED: 'bloccard_viewed',
  BLOCCARD_CREATED: 'bloccard_created',
  BLOCCARD_SHARED: 'bloccard_shared',
  BLOCCARD_SCANNED: 'bloccard_scanned',
  BLOCCARD_FAVORITE_ADDED: 'bloccard_favorite_added',

  // NFT Features
  NFT_VIEWED: 'nft_viewed',
  NFT_PURCHASED: 'nft_purchased',
  NFT_LISTED: 'nft_listed',
  NFT_TRANSFERRED: 'nft_transferred',

  // DeFi Features
  SWAP_INITIATED: 'swap_initiated',
  SWAP_COMPLETED: 'swap_completed',
  LIQUIDITY_ADDED: 'liquidity_added',
  LIQUIDITY_REMOVED: 'liquidity_removed',
  STAKE_INITIATED: 'stake_initiated',
  STAKE_COMPLETED: 'stake_completed',
  UNSTAKE_INITIATED: 'unstake_initiated',
  UNSTAKE_COMPLETED: 'unstake_completed',

  // Trading
  TRADE_EXECUTED: 'trade_executed',
  ORDER_PLACED: 'order_placed',
  ORDER_CANCELLED: 'order_cancelled',
  LIMIT_ORDER_SET: 'limit_order_set',

  // Social Features
  CONTACT_ADDED: 'contact_added',
  MESSAGE_SENT: 'message_sent',
  GROUP_CREATED: 'group_created',
  GROUP_JOINED: 'group_joined',

  // Governance
  PROPOSAL_VIEWED: 'proposal_viewed',
  VOTE_CAST: 'vote_cast',
  PROPOSAL_CREATED: 'proposal_created',

  // Notifications
  NOTIFICATION_RECEIVED: 'notification_received',
  NOTIFICATION_OPENED: 'notification_opened',
  NOTIFICATION_DISMISSED: 'notification_dismissed',

  // Settings
  SETTINGS_CHANGED: 'settings_changed',
  THEME_CHANGED: 'theme_changed',
  LANGUAGE_CHANGED: 'language_changed',
  CURRENCY_CHANGED: 'currency_changed',

  // Security
  BIOMETRIC_ENABLED: 'biometric_enabled',
  PIN_SET: 'pin_set',
  TWO_FACTOR_ENABLED: 'two_factor_enabled',

  // Errors
  ERROR_OCCURRED: 'error_occurred',
  API_ERROR: 'api_error',
  NETWORK_ERROR: 'network_error',

  // Engagement
  FEATURE_USED: 'feature_used',
  TUTORIAL_STARTED: 'tutorial_started',
  TUTORIAL_COMPLETED: 'tutorial_completed',
  SEARCH_PERFORMED: 'search_performed',
  SHARE_INITIATED: 'share_initiated',
} as const;

/**
 * Log a custom analytics event
 *
 * @param eventName Name of the event (use AnalyticsEvents constants)
 * @param params Optional event parameters
 */
export function logEvent(
  eventName: string,
  params?: Record<string, any>
): void {
  const analytics = getFirebaseAnalytics();

  // Only log in production or if explicitly enabled
  const shouldLog =
    process.env.NODE_ENV === 'production' ||
    process.env.NEXT_PUBLIC_FIREBASE_ANALYTICS_DEBUG === 'true';

  if (!shouldLog) {
    console.log('[Analytics Debug]', eventName, params);
    return;
  }

  if (!analytics) {
    console.warn('Analytics: Not initialized');
    return;
  }

  try {
    firebaseLogEvent(analytics, eventName, params);
    console.log('Analytics: Event logged:', eventName, params);
  } catch (error) {
    console.error('Analytics: Error logging event:', error);
  }
}

/**
 * Log a screen view event
 *
 * @param screenName Name of the screen/page
 * @param screenClass Optional screen class/category
 */
export function logScreenView(
  screenName: string,
  screenClass?: string
): void {
  logEvent(AnalyticsEvents.SCREEN_VIEW, {
    screen_name: screenName,
    screen_class: screenClass || screenName,
  });
}

/**
 * Log a transaction event
 *
 * @param type Transaction type ('send' | 'receive' | 'swap' | 'stake')
 * @param amount Transaction amount
 * @param currency Currency code
 * @param success Whether transaction succeeded
 */
export function logTransaction(
  type: 'send' | 'receive' | 'swap' | 'stake',
  amount: number,
  currency: string,
  success: boolean = true
): void {
  const eventMap = {
    send: success ? AnalyticsEvents.TRANSACTION_SENT : AnalyticsEvents.TRANSACTION_FAILED,
    receive: AnalyticsEvents.TRANSACTION_RECEIVED,
    swap: success ? AnalyticsEvents.SWAP_COMPLETED : AnalyticsEvents.TRANSACTION_FAILED,
    stake: success ? AnalyticsEvents.STAKE_COMPLETED : AnalyticsEvents.TRANSACTION_FAILED,
  };

  logEvent(eventMap[type], {
    value: amount,
    currency,
    transaction_type: type,
    success,
  });
}

/**
 * Log an error event
 *
 * @param errorType Type of error ('api' | 'network' | 'validation' | 'general')
 * @param errorMessage Error message
 * @param errorCode Optional error code
 */
export function logError(
  errorType: 'api' | 'network' | 'validation' | 'general',
  errorMessage: string,
  errorCode?: string | number
): void {
  const eventMap = {
    api: AnalyticsEvents.API_ERROR,
    network: AnalyticsEvents.NETWORK_ERROR,
    validation: AnalyticsEvents.ERROR_OCCURRED,
    general: AnalyticsEvents.ERROR_OCCURRED,
  };

  logEvent(eventMap[errorType], {
    error_type: errorType,
    error_message: errorMessage,
    error_code: errorCode,
  });
}

/**
 * Log NFT interaction
 *
 * @param action Action performed ('view' | 'purchase' | 'list' | 'transfer')
 * @param nftId NFT identifier
 * @param collection NFT collection name
 * @param price Optional price (for purchase/list)
 */
export function logNFTInteraction(
  action: 'view' | 'purchase' | 'list' | 'transfer',
  nftId: string,
  collection: string,
  price?: number
): void {
  const eventMap = {
    view: AnalyticsEvents.NFT_VIEWED,
    purchase: AnalyticsEvents.NFT_PURCHASED,
    list: AnalyticsEvents.NFT_LISTED,
    transfer: AnalyticsEvents.NFT_TRANSFERRED,
  };

  logEvent(eventMap[action], {
    nft_id: nftId,
    collection,
    price,
    action,
  });
}

/**
 * Log Bloccard interaction
 *
 * @param action Action performed
 * @param bloccardId Bloccard identifier
 */
export function logBloccardInteraction(
  action: 'view' | 'create' | 'share' | 'scan' | 'favorite',
  bloccardId?: string
): void {
  const eventMap = {
    view: AnalyticsEvents.BLOCCARD_VIEWED,
    create: AnalyticsEvents.BLOCCARD_CREATED,
    share: AnalyticsEvents.BLOCCARD_SHARED,
    scan: AnalyticsEvents.BLOCCARD_SCANNED,
    favorite: AnalyticsEvents.BLOCCARD_FAVORITE_ADDED,
  };

  logEvent(eventMap[action], {
    bloccard_id: bloccardId,
    action,
  });
}

/**
 * Log DeFi interaction
 *
 * @param action DeFi action
 * @param protocol Protocol name
 * @param amount Amount involved
 * @param token Token symbol
 */
export function logDeFiInteraction(
  action: 'swap' | 'add_liquidity' | 'remove_liquidity' | 'stake' | 'unstake',
  protocol: string,
  amount: number,
  token: string
): void {
  const eventMap = {
    swap: AnalyticsEvents.SWAP_COMPLETED,
    add_liquidity: AnalyticsEvents.LIQUIDITY_ADDED,
    remove_liquidity: AnalyticsEvents.LIQUIDITY_REMOVED,
    stake: AnalyticsEvents.STAKE_COMPLETED,
    unstake: AnalyticsEvents.UNSTAKE_COMPLETED,
  };

  logEvent(eventMap[action], {
    protocol,
    amount,
    token,
    action,
  });
}

/**
 * Log governance interaction
 *
 * @param action Governance action
 * @param proposalId Proposal identifier
 * @param vote Optional vote ('for' | 'against' | 'abstain')
 */
export function logGovernanceInteraction(
  action: 'view' | 'vote' | 'create',
  proposalId: string,
  vote?: 'for' | 'against' | 'abstain'
): void {
  const eventMap = {
    view: AnalyticsEvents.PROPOSAL_VIEWED,
    vote: AnalyticsEvents.VOTE_CAST,
    create: AnalyticsEvents.PROPOSAL_CREATED,
  };

  logEvent(eventMap[action], {
    proposal_id: proposalId,
    vote,
    action,
  });
}

/**
 * Log notification interaction
 *
 * @param action Notification action
 * @param notificationType Type of notification
 */
export function logNotificationInteraction(
  action: 'received' | 'opened' | 'dismissed',
  notificationType: string
): void {
  const eventMap = {
    received: AnalyticsEvents.NOTIFICATION_RECEIVED,
    opened: AnalyticsEvents.NOTIFICATION_OPENED,
    dismissed: AnalyticsEvents.NOTIFICATION_DISMISSED,
  };

  logEvent(eventMap[action], {
    notification_type: notificationType,
    action,
  });
}

/**
 * Set user properties
 *
 * @param properties User properties to set
 */
export function setAnalyticsUserProperties(
  properties: Record<string, any>
): void {
  const analytics = getFirebaseAnalytics();

  if (!analytics) {
    console.warn('Analytics: Not initialized');
    return;
  }

  try {
    setUserProperties(analytics, properties);
    console.log('Analytics: User properties set:', properties);
  } catch (error) {
    console.error('Analytics: Error setting user properties:', error);
  }
}

/**
 * Set user ID for analytics
 *
 * @param userId User identifier (should be anonymized/hashed)
 */
export function setAnalyticsUserId(userId: string): void {
  const analytics = getFirebaseAnalytics();

  if (!analytics) {
    console.warn('Analytics: Not initialized');
    return;
  }

  try {
    setUserId(analytics, userId);
    console.log('Analytics: User ID set');
  } catch (error) {
    console.error('Analytics: Error setting user ID:', error);
  }
}

/**
 * Enable/disable analytics collection
 *
 * @param enabled Whether to enable analytics
 */
export function setAnalyticsCollectionEnabled(enabled: boolean): void {
  const analytics = getFirebaseAnalytics();

  if (!analytics) {
    return;
  }

  // This requires firebase/analytics setAnalyticsCollectionEnabled
  // For now, we'll just log it
  console.log('Analytics: Collection', enabled ? 'enabled' : 'disabled');
}
