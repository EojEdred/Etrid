export interface ValidatorInfo {
  address: string;
  stash: string;
  controller: string;
  sessionKeys: string;
  commission: number;
  totalStake: bigint;
  ownStake: bigint;
  nominatorCount: number;
  isActive: boolean;
  isElected: boolean;
  isBlocking: boolean;
  eraPoints: number;
  lastBlockProduced: number;
  uptime: number;
}

export interface Nominator {
  address: string;
  stake: bigint;
  active: boolean;
  since: number;
  lastReward: bigint;
}

export interface Reward {
  era: number;
  amount: bigint;
  timestamp: number;
  nominators: number;
  commission: number;
}

export interface PerformanceMetrics {
  blocksProduced: number;
  missedBlocks: number;
  uptime: number;
  averageBlockTime: number;
  eraPoints: number;
  rank: number;
  totalValidators: number;
}

export interface Alert {
  id: string;
  type: 'warning' | 'error' | 'info' | 'success';
  title: string;
  message: string;
  timestamp: number;
  read: boolean;
  action?: {
    label: string;
    href: string;
  };
}

export interface ChartData {
  timestamp: number;
  value: number;
  label?: string;
}

export interface ValidatorSettings {
  commission: number;
  paymentDestination: 'staked' | 'stash' | 'controller' | 'account';
  paymentAccount?: string;
  autoCompound: boolean;
  alertsEnabled: boolean;
  emailNotifications: boolean;
  discordNotifications: boolean;
}

export interface SessionInfo {
  currentSession: number;
  currentEra: number;
  sessionProgress: number;
  eraProgress: number;
  sessionLength: number;
  eraLength: number;
  timeToNextSession: number;
  timeToNextEra: number;
}

export interface NetworkStats {
  totalIssuance: bigint;
  totalStaked: bigint;
  stakingRate: number;
  activeValidators: number;
  waitingValidators: number;
  minNominatorBond: bigint;
  minValidatorBond: bigint;
  inflationRate: number;
}
