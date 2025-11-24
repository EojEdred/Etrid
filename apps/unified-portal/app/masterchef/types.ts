export interface PoolData {
  poolId: number;
  lpToken: string;
  lpSymbol: string;
  lpName: string;
  totalStaked: string;
  allocPoint: number;
  rewardShare: number;
  lpPrice: number | null;
  tvlUSD: number | null;
  aprPercent: number | null;
  dailyRewards: string;
  monthlyRewards: string;
}

export interface EmissionsData {
  perBlock: string;
  perDay: string;
  perMonth: string;
  perYear: string;
}

export interface MetricsData {
  timestamp: string;
  network: string;
  chainId: number;
  blockNumber: number;
  contracts: {
    etrToken: string;
    masterChef: string;
  };
  masterchef: {
    totalPools: number;
    totalAllocPoint: string;
    rewardPerBlock: string;
    paused: boolean;
    owner: string;
  };
  emissions: EmissionsData;
  balance: {
    masterChefETR: string;
    daysRemaining: number;
  };
  pools: PoolData[];
  prices: {
    bnb: number;
    etr: number;
  };
  overview: {
    totalPools: number;
    totalAllocPoint: string;
    rewardPerBlock: string;
    totalStakedLP: string;
    totalTVLUSD: number;
  };
}
