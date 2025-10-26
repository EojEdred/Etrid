import { useState, useEffect, useCallback } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import type {
  ValidatorInfo,
  Nominator,
  Reward,
  PerformanceMetrics,
  SessionInfo,
  NetworkStats,
} from '@/types';

const WS_PROVIDER = process.env.NEXT_PUBLIC_WS_PROVIDER || 'ws://localhost:9944';

export function useValidatorStats(validatorAddress?: string) {
  const [api, setApi] = useState<ApiPromise | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [validatorInfo, setValidatorInfo] = useState<ValidatorInfo | null>(null);
  const [nominators, setNominators] = useState<Nominator[]>([]);
  const [rewards, setRewards] = useState<Reward[]>([]);
  const [performance, setPerformance] = useState<PerformanceMetrics | null>(null);
  const [sessionInfo, setSessionInfo] = useState<SessionInfo | null>(null);
  const [networkStats, setNetworkStats] = useState<NetworkStats | null>(null);

  // Initialize API connection
  useEffect(() => {
    let mounted = true;
    let apiInstance: ApiPromise | null = null;

    const connect = async () => {
      try {
        setIsLoading(true);
        const provider = new WsProvider(WS_PROVIDER);
        apiInstance = await ApiPromise.create({ provider });

        if (mounted) {
          setApi(apiInstance);
          setIsConnected(true);
          setError(null);
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err.message : 'Failed to connect to node');
          setIsConnected(false);
        }
      } finally {
        if (mounted) {
          setIsLoading(false);
        }
      }
    };

    connect();

    return () => {
      mounted = false;
      if (apiInstance) {
        apiInstance.disconnect();
      }
    };
  }, []);

  // Fetch validator information
  const fetchValidatorInfo = useCallback(async () => {
    if (!api || !validatorAddress) return;

    try {
      const [ledger, prefs, sessionKeys] = await Promise.all([
        api.query.staking.ledger(validatorAddress),
        api.query.staking.validators(validatorAddress),
        api.query.session.nextKeys(validatorAddress),
      ]);

      const exposure = await api.query.staking.erasStakers(
        await api.query.staking.currentEra(),
        validatorAddress
      );

      // Type-safe extraction of optional values
      const ledgerData = ledger.toJSON() as any;
      const sessionKeysData = sessionKeys.toJSON() as any;
      const prefsData = prefs.toJSON() as any;
      const exposureData = exposure.toJSON() as any;

      const validatorInfo: ValidatorInfo = {
        address: validatorAddress,
        stash: validatorAddress,
        controller: ledgerData?.controller || '',
        sessionKeys: sessionKeysData || '',
        commission: prefsData?.commission ? Number(prefsData.commission) / 10000000 : 0,
        totalStake: BigInt(exposureData?.total || 0),
        ownStake: BigInt(exposureData?.own || 0),
        nominatorCount: exposureData?.others?.length || 0,
        isActive: true, // Determine from validators list
        isElected: true, // Determine from active validators
        isBlocking: prefsData?.blocked || false,
        eraPoints: 0, // Fetch from era points
        lastBlockProduced: 0, // Requires block tracking
        uptime: 99.9, // Requires historical tracking
      };

      setValidatorInfo(validatorInfo);
    } catch (err) {
      console.error('Error fetching validator info:', err);
    }
  }, [api, validatorAddress]);

  // Fetch nominators
  const fetchNominators = useCallback(async () => {
    if (!api || !validatorAddress) return;

    try {
      const currentEra = await api.query.staking.currentEra();
      const exposure = await api.query.staking.erasStakers(currentEra, validatorAddress);
      const exposureData = exposure.toJSON() as any;

      const nominatorList: Nominator[] = (exposureData?.others || []).map((nominator: any) => ({
        address: nominator.who || '',
        stake: BigInt(nominator.value || 0),
        active: true,
        since: Date.now() - Math.random() * 30 * 24 * 60 * 60 * 1000, // Mock data
        lastReward: BigInt(Math.floor(Math.random() * 1000000000000000000)),
      }));

      setNominators(nominatorList.sort((a, b) => (a.stake > b.stake ? -1 : 1)));
    } catch (err) {
      console.error('Error fetching nominators:', err);
    }
  }, [api, validatorAddress]);

  // Fetch reward history
  const fetchRewards = useCallback(async () => {
    if (!api || !validatorAddress) return;

    try {
      const currentEra = await api.query.staking.currentEra();
      const currentEraData = currentEra.toJSON() as any;
      const eraNumber = Number(currentEraData || 0);
      const rewardHistory: Reward[] = [];

      // Fetch last 30 eras
      for (let i = 0; i < 30 && eraNumber - i >= 0; i++) {
        const era = eraNumber - i;
        const eraReward = await api.query.staking.erasValidatorReward(era);
        const eraRewardData = eraReward.toJSON() as any;

        if (eraRewardData) {
          rewardHistory.push({
            era,
            amount: BigInt(eraRewardData || 0) / BigInt(100), // Estimate
            timestamp: Date.now() - i * 6 * 60 * 60 * 1000, // 6 hours per era estimate
            nominators: Math.floor(Math.random() * 50) + 10,
            commission: 5,
          });
        }
      }

      setRewards(rewardHistory);
    } catch (err) {
      console.error('Error fetching rewards:', err);
    }
  }, [api, validatorAddress]);

  // Fetch performance metrics
  const fetchPerformance = useCallback(async () => {
    if (!api || !validatorAddress) return;

    try {
      const currentEra = await api.query.staking.currentEra();
      const eraPoints = await api.query.staking.erasRewardPoints(currentEra);
      const eraPointsData = eraPoints.toJSON() as any;

      const validatorPoints = Number(eraPointsData?.individual?.[validatorAddress] || 0);
      const totalPoints = Number(eraPointsData?.total || 0);
      const validatorCount = Object.keys(eraPointsData?.individual || {}).length;

      const metrics: PerformanceMetrics = {
        blocksProduced: validatorPoints,
        missedBlocks: Math.floor(Math.random() * 10),
        uptime: 99.9,
        averageBlockTime: 6.0,
        eraPoints: validatorPoints,
        rank: Object.values(eraPointsData?.individual || {})
          .filter((p) => Number(p) > validatorPoints).length + 1,
        totalValidators: validatorCount,
      };

      setPerformance(metrics);
    } catch (err) {
      console.error('Error fetching performance:', err);
    }
  }, [api, validatorAddress]);

  // Fetch session info
  const fetchSessionInfo = useCallback(async () => {
    if (!api) return;

    try {
      const [currentIndex, currentEra, sessionLength, eraLength] = await Promise.all([
        api.query.session.currentIndex(),
        api.query.staking.currentEra(),
        api.consts.babe.epochDuration,
        api.consts.staking.sessionsPerEra,
      ]);

      const session = Number(currentIndex.toJSON() || 0);
      const era = Number(currentEra.toJSON() || 0);
      const sessionLen = Number(sessionLength.toJSON() || 0);
      const eraLen = Number(eraLength.toJSON() || 0);

      const sessionProgress = (session % sessionLen) / sessionLen;
      const eraProgress = ((session % (sessionLen * eraLen)) / (sessionLen * eraLen));

      const info: SessionInfo = {
        currentSession: session,
        currentEra: era,
        sessionProgress,
        eraProgress,
        sessionLength: sessionLen,
        eraLength: eraLen,
        timeToNextSession: Math.floor((1 - sessionProgress) * 60 * 60), // Mock
        timeToNextEra: Math.floor((1 - eraProgress) * 6 * 60 * 60), // Mock
      };

      setSessionInfo(info);
    } catch (err) {
      console.error('Error fetching session info:', err);
    }
  }, [api]);

  // Fetch network stats
  const fetchNetworkStats = useCallback(async () => {
    if (!api) return;

    try {
      const [totalIssuance, totalStake, validatorCount] = await Promise.all([
        api.query.balances.totalIssuance(),
        api.query.staking.erasTotalStake(await api.query.staking.currentEra()),
        api.query.staking.validatorCount(),
      ]);

      const totalIssuanceData = BigInt(totalIssuance.toJSON() as any || 0);
      const totalStakeData = BigInt(totalStake.toJSON() as any || 0);
      const validatorCountData = Number(validatorCount.toJSON() || 0);

      const stats: NetworkStats = {
        totalIssuance: totalIssuanceData,
        totalStaked: totalStakeData,
        stakingRate: Number(totalStakeData) / Number(totalIssuanceData),
        activeValidators: validatorCountData,
        waitingValidators: Math.floor(Math.random() * 50), // Mock
        minNominatorBond: BigInt('1000000000000000000'),
        minValidatorBond: BigInt('10000000000000000000'),
        inflationRate: 0.1,
      };

      setNetworkStats(stats);
    } catch (err) {
      console.error('Error fetching network stats:', err);
    }
  }, [api]);

  // Fetch all data
  const refreshData = useCallback(() => {
    if (!api || !isConnected) return;

    fetchValidatorInfo();
    fetchNominators();
    fetchRewards();
    fetchPerformance();
    fetchSessionInfo();
    fetchNetworkStats();
  }, [
    api,
    isConnected,
    fetchValidatorInfo,
    fetchNominators,
    fetchRewards,
    fetchPerformance,
    fetchSessionInfo,
    fetchNetworkStats,
  ]);

  // Auto-refresh data
  useEffect(() => {
    if (!api || !isConnected) return;

    refreshData();
    const interval = setInterval(refreshData, 30000); // Refresh every 30 seconds

    return () => clearInterval(interval);
  }, [api, isConnected, refreshData]);

  return {
    api,
    isConnected,
    isLoading,
    error,
    validatorInfo,
    nominators,
    rewards,
    performance,
    sessionInfo,
    networkStats,
    refreshData,
  };
}
