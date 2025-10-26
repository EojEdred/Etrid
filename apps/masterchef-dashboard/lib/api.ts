import type { MetricsData } from '../types';

/**
 * Fetch metrics from the exported JSON file
 *
 * In production, this should point to:
 * - A static file served from your web server
 * - An API endpoint that returns the latest metrics
 * - A CDN URL where metrics are uploaded
 */
export async function fetchMetrics(): Promise<MetricsData> {
  // Option 1: Fetch from static file (development)
  // const response = await fetch('/api/metrics');

  // Option 2: Fetch from your server
  // const response = await fetch('https://your-domain.com/api/metrics');

  // Option 3: Fetch latest metrics file (example)
  // This assumes you're copying metrics-*.json to /public/metrics.json
  const response = await fetch('/metrics.json');

  if (!response.ok) {
    throw new Error(`Failed to fetch metrics: ${response.statusText}`);
  }

  const data: MetricsData = await response.json();

  // Validate data
  if (!data || !data.pools || !Array.isArray(data.pools)) {
    throw new Error('Invalid metrics data format');
  }

  return data;
}

/**
 * Fetch metrics with error handling and retries
 */
export async function fetchMetricsWithRetry(retries = 3): Promise<MetricsData> {
  let lastError: Error | null = null;

  for (let i = 0; i < retries; i++) {
    try {
      return await fetchMetrics();
    } catch (error) {
      lastError = error as Error;
      console.error(`Fetch attempt ${i + 1} failed:`, error);

      // Wait before retrying (exponential backoff)
      if (i < retries - 1) {
        await new Promise(resolve => setTimeout(resolve, 1000 * Math.pow(2, i)));
      }
    }
  }

  throw lastError || new Error('Failed to fetch metrics after retries');
}

/**
 * Create an API route handler for Next.js
 * Save this as app/api/metrics/route.ts
 */
export const metricsApiHandler = `
import { NextResponse } from 'next/server';
import { readFileSync } from 'fs';
import { join } from 'path';

export async function GET() {
  try {
    // Read the latest metrics file
    // Adjust path based on where you store exported metrics
    const metricsPath = join(process.cwd(), '../../../05-multichain/bridge/adapters/bsc/metrics-latest.json');
    const data = readFileSync(metricsPath, 'utf-8');
    const metrics = JSON.parse(data);

    return NextResponse.json(metrics, {
      headers: {
        'Cache-Control': 'public, s-maxage=60, stale-while-revalidate=120',
      },
    });
  } catch (error) {
    console.error('Failed to fetch metrics:', error);
    return NextResponse.json(
      { error: 'Failed to load metrics' },
      { status: 500 }
    );
  }
}
`;
