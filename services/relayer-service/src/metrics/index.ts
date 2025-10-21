/**
 * Prometheus Metrics for Relayer Service
 *
 * Exposes metrics for monitoring relay operations, performance, and balances.
 */

import { Registry, Counter, Gauge, Histogram, collectDefaultMetrics } from 'prom-client';

// Create a Registry which registers the metrics
export const register = new Registry();

// Add default metrics (CPU, memory, etc.)
collectDefaultMetrics({
  register,
  prefix: 'relayer_',
});

// Service health
export const serviceUp = new Gauge({
  name: 'relayer_service_up',
  help: 'Service is running (1 = up, 0 = down)',
  registers: [register],
});

// Initialize as up
serviceUp.set(1);

// Messages relayed successfully
export const messagesRelayed = new Counter({
  name: 'relayer_messages_relayed_total',
  help: 'Total messages successfully relayed',
  labelNames: ['destination'],
  registers: [register],
});

// Relay attempts (including failures)
export const relayAttempts = new Counter({
  name: 'relayer_relay_attempts_total',
  help: 'Total relay attempts',
  labelNames: ['destination'],
  registers: [register],
});

// Relay failures
export const relayFailures = new Counter({
  name: 'relayer_relay_failures_total',
  help: 'Total relay failures',
  labelNames: ['destination', 'reason'],
  registers: [register],
});

// Relay duration (time from fetch to successful relay)
export const relayDuration = new Histogram({
  name: 'relayer_relay_duration_seconds',
  help: 'Time to relay a message',
  labelNames: ['destination'],
  buckets: [1, 5, 10, 30, 60, 120, 300, 600],
  registers: [register],
});

// Balances
export const ethBalance = new Gauge({
  name: 'relayer_balance_eth',
  help: 'ETH balance of relayer account',
  registers: [register],
});

export const edscBalance = new Gauge({
  name: 'relayer_balance_edsc',
  help: 'EDSC balance of relayer account',
  registers: [register],
});

// Attestations fetched
export const attestationsFetched = new Counter({
  name: 'relayer_attestations_fetched_total',
  help: 'Total attestations fetched from attestation services',
  registers: [register],
});

// Ready attestations available
export const attestationsReady = new Gauge({
  name: 'relayer_attestations_ready_current',
  help: 'Number of attestations currently ready to relay',
  registers: [register],
});

// Duplicate message attempts (already relayed)
export const duplicateAttempts = new Counter({
  name: 'relayer_duplicate_message_attempts_total',
  help: 'Number of times relayer attempted to relay already-relayed messages',
  registers: [register],
});

// API request metrics (for metrics endpoint)
export const apiRequests = new Counter({
  name: 'relayer_api_requests_total',
  help: 'Total API requests received',
  labelNames: ['endpoint', 'method', 'status'],
  registers: [register],
});

// Fetcher metrics
export const fetcherErrors = new Counter({
  name: 'relayer_fetcher_errors_total',
  help: 'Total errors from attestation fetcher',
  labelNames: ['service_url'],
  registers: [register],
});

export const fetcherPollDuration = new Histogram({
  name: 'relayer_fetcher_poll_duration_seconds',
  help: 'Time to poll all attestation services',
  buckets: [0.5, 1, 2, 5, 10],
  registers: [register],
});

// Last successful relay timestamp
export const lastRelayTimestamp = new Gauge({
  name: 'relayer_last_relay_timestamp',
  help: 'Timestamp of last successful relay',
  labelNames: ['destination'],
  registers: [register],
});

// Gas price (Ethereum)
export const ethereumGasPrice = new Gauge({
  name: 'relayer_ethereum_gas_price_gwei',
  help: 'Current Ethereum gas price in gwei',
  registers: [register],
});

/**
 * Record a successful relay
 */
export function recordSuccessfulRelay(destination: string, duration: number): void {
  messagesRelayed.inc({ destination });
  relayAttempts.inc({ destination });
  relayDuration.observe({ destination }, duration);
  lastRelayTimestamp.set({ destination }, Date.now() / 1000);
}

/**
 * Record a failed relay
 */
export function recordFailedRelay(destination: string, reason: string): void {
  relayAttempts.inc({ destination });
  relayFailures.inc({ destination, reason });
}

/**
 * Update balance metrics
 */
export function updateBalances(eth: number, edsc: number): void {
  ethBalance.set(eth);
  edscBalance.set(edsc);
}

/**
 * Record an API request
 */
export function recordApiRequest(
  endpoint: string,
  method: string,
  status: number
): void {
  apiRequests.inc({ endpoint, method, status: status.toString() });
}

/**
 * Get metrics as string (for /metrics endpoint)
 */
export async function getMetrics(): Promise<string> {
  return register.metrics();
}

/**
 * Reset all metrics (useful for testing)
 */
export function resetMetrics(): void {
  register.resetMetrics();
}
