/**
 * ETRID Governance Module
 *
 * Complete governance functionality for the ETRID web wallet
 * Exports service, types, and React hooks
 */

// Export service
export { governanceService, GovernanceService } from './service';

// Export all types
export * from './types';

// Export all hooks
export * from './hooks';

// Re-export commonly used items for convenience
export { default as governance } from './service';
