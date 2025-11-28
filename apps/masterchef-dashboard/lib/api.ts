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
  // Try to fetch from file first, fallback to embedded data
  try {
    const response = await fetch('/metrics.json');

    if (response.ok) {
      const data: MetricsData = await response.json();

      // Validate data
      if (data && data.pools && Array.isArray(data.pools)) {
        return data;
      }
    }
  } catch (error) {
    console.warn('Could not fetch metrics.json, using embedded data:', error);
  }

  // Fallback to embedded mock data
  return getMockMetrics();
}

// Embedded mock data for standalone deployment
function getMockMetrics(): MetricsData {
  return {
    "timestamp": new Date().toISOString(),
    "network": "bsc",
    "chainId": 56,
    "blockNumber": 34567890,
    "contracts": {
      "etrToken": "0x1234567890123456789012345678901234567890",
      "masterChef": "0x0987654321098765432109876543210987654321"
    },
    "masterchef": {
      "totalPools": 6,
      "totalAllocPoint": "1000",
      "rewardPerBlock": "10.5",
      "paused": false,
      "owner": "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
    },
    "emissions": {
      "perBlock": "10.5",
      "perDay": "302400",
      "perMonth": "9072000",
      "perYear": "108864000"
    },
    "balance": {
      "masterChefETR": "50000000",
      "daysRemaining": 165
    },
    "pools": [
      {
        "poolId": 0,
        "lpToken": "0x1111111111111111111111111111111111111111",
        "lpSymbol": "ÉTR-BNB LP",
        "lpName": "ÉTR-BNB PancakeSwap LP",
        "totalStaked": "125000.50",
        "allocPoint": 250,
        "rewardShare": 0.25,
        "lpPrice": 12.5,
        "tvlUSD": 1562506.25,
        "aprPercent": 85.4,
        "dailyRewards": "75600",
        "monthlyRewards": "2268000"
      },
      {
        "poolId": 1,
        "lpToken": "0x2222222222222222222222222222222222222222",
        "lpSymbol": "ÉTR-BUSD LP",
        "lpName": "ÉTR-BUSD PancakeSwap LP",
        "totalStaked": "98450.75",
        "allocPoint": 200,
        "rewardShare": 0.20,
        "lpPrice": 10.2,
        "tvlUSD": 1004197.65,
        "aprPercent": 72.3,
        "dailyRewards": "60480",
        "monthlyRewards": "1814400"
      },
      {
        "poolId": 2,
        "lpToken": "0x3333333333333333333333333333333333333333",
        "lpSymbol": "ÉTR-ETH LP",
        "lpName": "ÉTR-ETH PancakeSwap LP",
        "totalStaked": "54230.25",
        "allocPoint": 150,
        "rewardShare": 0.15,
        "lpPrice": 18.7,
        "tvlUSD": 1014085.68,
        "aprPercent": 68.9,
        "dailyRewards": "45360",
        "monthlyRewards": "1360800"
      },
      {
        "poolId": 3,
        "lpToken": "0x4444444444444444444444444444444444444444",
        "lpSymbol": "ÉTR-CAKE LP",
        "lpName": "ÉTR-CAKE PancakeSwap LP",
        "totalStaked": "76543.80",
        "allocPoint": 150,
        "rewardShare": 0.15,
        "lpPrice": 8.4,
        "tvlUSD": 642967.92,
        "aprPercent": 59.2,
        "dailyRewards": "45360",
        "monthlyRewards": "1360800"
      },
      {
        "poolId": 4,
        "lpToken": "0x5555555555555555555555555555555555555555",
        "lpSymbol": "ÉTR-USDT LP",
        "lpName": "ÉTR-USDT PancakeSwap LP",
        "totalStaked": "112890.40",
        "allocPoint": 150,
        "rewardShare": 0.15,
        "lpPrice": 9.8,
        "tvlUSD": 1106325.92,
        "aprPercent": 54.1,
        "dailyRewards": "45360",
        "monthlyRewards": "1360800"
      },
      {
        "poolId": 5,
        "lpToken": "0x6666666666666666666666666666666666666666",
        "lpSymbol": "ÉTR Single Stake",
        "lpName": "ÉTR Single Asset Staking",
        "totalStaked": "2450000.00",
        "allocPoint": 100,
        "rewardShare": 0.10,
        "lpPrice": null,
        "tvlUSD": 24500000.00,
        "aprPercent": 18.5,
        "dailyRewards": "30240",
        "monthlyRewards": "907200"
      }
    ],
    "prices": {
      "bnb": 320.5,
      "etr": 10.0
    },
    "overview": {
      "totalPools": 6,
      "totalAllocPoint": "1000",
      "rewardPerBlock": "10.5",
      "totalStakedLP": "2917115.70",
      "totalTVLUSD": 29831083.42
    }
  };
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
