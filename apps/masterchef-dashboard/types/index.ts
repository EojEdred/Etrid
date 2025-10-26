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
  aprPercent?: number | null;
  dailyRewards: string;
  monthlyRewards: string;
}

export interface EmissionsData {
  perBlock: string;
  perDay: string;
  perMonth: string;
  perYear: string;
}

export interface BalanceData {
  masterChefETR: string;
  daysRemaining: number;
}

export interface PriceData {
  bnb: number;
  etr: number;
}

export interface OverviewData {
  totalPools: number;
  totalAllocPoint: string;
  rewardPerBlock: string;
  totalStakedLP: string;
  totalTVLUSD: number | null;
}

export interface ContractsData {
  etrToken: string;
  masterChef: string;
}

export interface MetricsData {
  timestamp: string;
  network: string;
  chainId: number;
  blockNumber: number;
  contracts: ContractsData;
  masterchef: {
    totalPools: number;
    totalAllocPoint: string;
    rewardPerBlock: string;
    paused: boolean;
    owner: string;
  };
  emissions: EmissionsData;
  balance: BalanceData;
  pools: PoolData[];
  prices?: PriceData;
  overview: OverviewData;
}
