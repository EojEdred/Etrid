import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  RefreshControl,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * ConsensusScreen - Matches iOS Swift GovernanceView exactly
 *
 * Tabs:
 * - Proposals - Active governance proposals
 * - Directors - Elected board directors
 * - Staking - Stake ETR for voting power
 * - Rewards - Participation rewards
 *
 * Calendar-based phases:
 * - Registration: Jan 1 - Nov 30
 * - Voting: Dec 1 (Consensus Day)
 * - Execution: Dec 2 - 30
 * - Distribution: Dec 31
 */

type ConsensusTab = 'proposals' | 'directors' | 'staking' | 'rewards';

const TABS: {key: ConsensusTab; label: string}[] = [
  {key: 'proposals', label: 'Proposals'},
  {key: 'directors', label: 'Directors'},
  {key: 'staking', label: 'Staking'},
  {key: 'rewards', label: 'Rewards'},
];

// Sample data matching iOS Swift app
const SAMPLE_PROPOSALS = [
  {
    id: '1',
    title: 'Increase Validator Rewards by 5%',
    description: 'Proposal to increase validator staking rewards to encourage more network participation.',
    category: 'parameterChange',
    status: 'active',
    votesFor: 1250,
    votesAgainst: 340,
    approvalPercentage: 78.6,
    timeRemaining: '23h 45m',
    hasVoted: false,
  },
  {
    id: '2',
    title: 'Fund Community Development Grant',
    description: 'Allocate 50,000 ETR for developer grants and ecosystem growth.',
    category: 'budgetAllocation',
    status: 'active',
    votesFor: 890,
    votesAgainst: 210,
    approvalPercentage: 80.9,
    timeRemaining: '23h 45m',
    hasVoted: true,
  },
];

const SAMPLE_DIRECTORS = [
  {
    id: '1',
    name: 'Alice Chen',
    address: '0x1234...5678',
    manifesto: 'Committed to transparent governance and sustainable growth.',
    votesReceived: 12500,
    votingPowerReceived: 45000,
    status: 'elected',
  },
  {
    id: '2',
    name: 'Bob Smith',
    address: '0x8765...4321',
    manifesto: 'Focus on technical excellence and community engagement.',
    votesReceived: 11200,
    votingPowerReceived: 42000,
    status: 'elected',
  },
];

const SAMPLE_REWARDS = [
  {
    id: '1',
    type: 'voting',
    amount: 25.5,
    consensusDayYear: 2024,
    isClaimed: false,
  },
  {
    id: '2',
    type: 'staking',
    amount: 150.25,
    consensusDayYear: 2024,
    isClaimed: false,
  },
];

export default function ConsensusScreen({navigation}: any) {
  const [selectedTab, setSelectedTab] = useState<ConsensusTab>('proposals');
  const [refreshing, setRefreshing] = useState(false);
  const [currentPhase, setCurrentPhase] = useState<string>('registration');

  useEffect(() => {
    // Determine current phase based on date (matching iOS Swift logic)
    const now = new Date();
    const month = now.getMonth() + 1;
    const day = now.getDate();

    if (month === 12 && day === 1) {
      setCurrentPhase('voting');
    } else if (month === 12 && day >= 2 && day <= 30) {
      setCurrentPhase('execution');
    } else if (month === 12 && day === 31) {
      setCurrentPhase('distribution');
    } else {
      setCurrentPhase('registration');
    }
  }, []);

  const onRefresh = async () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 1000);
  };

  const getDaysUntilConsensusDay = () => {
    const now = new Date();
    const year = now.getFullYear();
    const consensusDay = new Date(year, 11, 1); // Dec 1
    if (now > consensusDay) {
      consensusDay.setFullYear(year + 1);
    }
    const diff = Math.ceil((consensusDay.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
    return diff;
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Phase Banner - Matches iOS Swift CalendarPhaseBanner */}
      <PhaseBanner phase={currentPhase} daysUntil={getDaysUntilConsensusDay()} />

      {/* Voting Power Card */}
      <VotingPowerCard />

      {/* Tab Selector */}
      <ScrollView
        horizontal
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.tabsContainer}>
        {TABS.map(tab => (
          <TouchableOpacity
            key={tab.key}
            style={[
              styles.tabChip,
              selectedTab === tab.key && styles.tabChipSelected,
            ]}
            onPress={() => setSelectedTab(tab.key)}>
            <Text
              style={[
                styles.tabChipText,
                selectedTab === tab.key && styles.tabChipTextSelected,
              ]}>
              {tab.label}
            </Text>
          </TouchableOpacity>
        ))}
      </ScrollView>

      {/* Tab Content */}
      <ScrollView
        style={styles.contentScrollView}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }>
        {selectedTab === 'proposals' && (
          <ProposalsTab
            proposals={SAMPLE_PROPOSALS}
            onProposalPress={(proposal: any) =>
              navigation.navigate('ProposalDetail', {proposal})
            }
            onSubmitPress={() => navigation.navigate('SubmitProposal')}
          />
        )}
        {selectedTab === 'directors' && (
          <DirectorsTab directors={SAMPLE_DIRECTORS} />
        )}
        {selectedTab === 'staking' && (
          <StakingTab onStakePress={() => navigation.navigate('Staking')} />
        )}
        {selectedTab === 'rewards' && <RewardsTab rewards={SAMPLE_REWARDS} />}
      </ScrollView>
    </LinearGradient>
  );
}

function PhaseBanner({phase, daysUntil}: {phase: string; daysUntil: number}) {
  const getPhaseInfo = () => {
    const year = new Date().getFullYear();
    switch (phase) {
      case 'voting':
        return {
          title: `Consensus Day ${year}`,
          subtitle: 'Voting Phase',
          icon: 'check-circle',
          countdown: 'TODAY',
          countdownSub: 'Vote now!',
          gradient: [colors.warning, colors.error],
        };
      case 'registration':
        return {
          title: 'Registration Period',
          subtitle: 'Registration Phase',
          icon: 'edit-3',
          countdown: `${daysUntil} days`,
          countdownSub: 'until Consensus Day',
          gradient: [colors.primary, colors.secondary],
        };
      case 'execution':
        return {
          title: 'Execution Period',
          subtitle: 'Execution Phase',
          icon: 'play-circle',
          countdown: `${31 - new Date().getDate()} days`,
          countdownSub: 'until completion',
          gradient: ['#9333EA', '#3B82F6'],
        };
      case 'distribution':
        return {
          title: 'Distribution Day',
          subtitle: 'Distribution Phase',
          icon: 'gift',
          countdown: 'FINAL DAY',
          countdownSub: 'of the year',
          gradient: [colors.success, '#14B8A6'],
        };
      default:
        return {
          title: 'Governance',
          subtitle: 'Inactive',
          icon: 'activity',
          countdown: '--',
          countdownSub: '',
          gradient: [colors.textSecondary, colors.textMuted],
        };
    }
  };

  const info = getPhaseInfo();

  return (
    <LinearGradient
      colors={info.gradient as any}
      start={{x: 0, y: 0}}
      end={{x: 1, y: 0}}
      style={styles.phaseBanner}>
      <View style={styles.phaseBannerLeft}>
        <Icon name={info.icon} size={24} color="#FFFFFF" />
        <View style={styles.phaseBannerText}>
          <Text style={styles.phaseBannerTitle}>{info.title}</Text>
          <Text style={styles.phaseBannerSubtitle}>{info.subtitle}</Text>
        </View>
      </View>
      <View style={styles.phaseBannerRight}>
        <Text style={styles.phaseCountdown}>{info.countdown}</Text>
        <Text style={styles.phaseCountdownSub}>{info.countdownSub}</Text>
      </View>
    </LinearGradient>
  );
}

function VotingPowerCard() {
  return (
    <View style={styles.votingPowerCard}>
      <View style={styles.votingPowerMain}>
        <View>
          <Text style={styles.votingPowerLabel}>Your Voting Power</Text>
          <Text style={styles.votingPowerValue}>1,250.00</Text>
        </View>
        <View style={styles.votingPowerMultipliers}>
          <View style={styles.multiplierRow}>
            <Icon name="clock" size={12} color={colors.success} />
            <Text style={styles.multiplierText}>+15%</Text>
          </View>
          <View style={styles.multiplierRow}>
            <Icon name="star" size={12} color={colors.warning} />
            <Text style={styles.multiplierTextWarning}>+10%</Text>
          </View>
        </View>
      </View>
      <View style={styles.votingPowerStats}>
        <View style={styles.statItem}>
          <Text style={styles.statValue}>5,000 ETR</Text>
          <Text style={styles.statLabel}>Staked</Text>
        </View>
        <View style={styles.statItem}>
          <Text style={styles.statValue}>3</Text>
          <Text style={styles.statLabel}>Proposals Voted</Text>
        </View>
        <View style={styles.statItem}>
          <Text style={styles.statValue}>175.75 ETR</Text>
          <Text style={styles.statLabel}>Rewards</Text>
        </View>
      </View>
    </View>
  );
}

function ProposalsTab({
  proposals,
  onProposalPress,
  onSubmitPress,
}: {
  proposals: any[];
  onProposalPress: (proposal: any) => void;
  onSubmitPress: () => void;
}) {
  if (proposals.length === 0) {
    return (
      <View style={styles.emptyState}>
        <Icon name="file-text" size={48} color={colors.textSecondary} />
        <Text style={styles.emptyStateTitle}>No Active Proposals</Text>
        <Text style={styles.emptyStateText}>
          Proposals will appear here during Consensus Day
        </Text>
      </View>
    );
  }

  return (
    <View style={styles.tabContent}>
      <TouchableOpacity style={styles.submitProposalButton} onPress={onSubmitPress}>
        <Icon name="plus-circle" size={20} color="#FFFFFF" />
        <Text style={styles.submitProposalText}>Submit Proposal</Text>
      </TouchableOpacity>
      {proposals.map(proposal => (
        <ProposalCard
          key={proposal.id}
          proposal={proposal}
          onPress={() => onProposalPress(proposal)}
        />
      ))}
    </View>
  );
}

function ProposalCard({
  proposal,
  onPress,
}: {
  proposal: any;
  onPress: () => void;
}) {
  const getCategoryIcon = () => {
    switch (proposal.category) {
      case 'parameterChange':
        return 'settings';
      case 'budgetAllocation':
        return 'dollar-sign';
      default:
        return 'file-text';
    }
  };

  return (
    <TouchableOpacity style={styles.proposalCard} onPress={onPress}>
      <View style={styles.proposalHeader}>
        <View style={styles.proposalCategoryBadge}>
          <Icon name={getCategoryIcon()} size={12} color={colors.primary} />
          <Text style={styles.proposalCategoryText}>
            {proposal.category === 'parameterChange' ? 'Parameter' : 'Budget'}
          </Text>
        </View>
        <View style={styles.proposalStatusRow}>
          <Text style={styles.proposalStatusText}>{proposal.status}</Text>
          {proposal.hasVoted && (
            <Icon name="check-circle" size={16} color={colors.success} />
          )}
        </View>
      </View>
      <Text style={styles.proposalTitle}>{proposal.title}</Text>
      <Text style={styles.proposalDescription} numberOfLines={2}>
        {proposal.description}
      </Text>
      <View style={styles.proposalVotes}>
        <View style={styles.voteRow}>
          <Text style={styles.voteFor}>For: {proposal.votesFor}</Text>
          <Text style={styles.voteAgainst}>Against: {proposal.votesAgainst}</Text>
        </View>
        <View style={styles.progressBar}>
          <View
            style={[
              styles.progressFill,
              {width: `${proposal.approvalPercentage}%`},
            ]}
          />
        </View>
        <Text style={styles.approvalText}>
          {proposal.approvalPercentage.toFixed(1)}% approval
        </Text>
      </View>
      <Text style={styles.timeRemaining}>{proposal.timeRemaining} remaining</Text>
    </TouchableOpacity>
  );
}

function DirectorsTab({directors}: {directors: any[]}) {
  return (
    <View style={styles.tabContent}>
      {directors.map(director => (
        <View key={director.id} style={styles.directorCard}>
          <View style={styles.directorHeader}>
            <View style={styles.directorAvatar}>
              <Text style={styles.directorAvatarText}>
                {director.name.charAt(0)}
              </Text>
            </View>
            <View style={styles.directorInfo}>
              <Text style={styles.directorName}>{director.name}</Text>
              <Text style={styles.directorAddress}>{director.address}</Text>
            </View>
            {director.status === 'elected' && (
              <Icon name="award" size={20} color={colors.success} />
            )}
          </View>
          <Text style={styles.directorManifesto}>{director.manifesto}</Text>
          <View style={styles.directorStats}>
            <View style={styles.directorStatRow}>
              <Icon name="users" size={14} color={colors.textSecondary} />
              <Text style={styles.directorStatText}>
                {director.votesReceived.toLocaleString()} votes
              </Text>
            </View>
            <View style={styles.directorStatRow}>
              <Icon name="zap" size={14} color={colors.textSecondary} />
              <Text style={styles.directorStatText}>
                {director.votingPowerReceived.toLocaleString()} VP
              </Text>
            </View>
          </View>
        </View>
      ))}
    </View>
  );
}

function StakingTab({onStakePress}: {onStakePress: () => void}) {
  return (
    <View style={styles.tabContent}>
      <View style={styles.stakingCard}>
        <View style={styles.stakingHeader}>
          <View>
            <Text style={styles.stakingLabel}>Staked Amount</Text>
            <Text style={styles.stakingValue}>5,000.00 ETR</Text>
          </View>
          <View style={styles.stakingRewards}>
            <Text style={styles.stakingLabel}>Rewards</Text>
            <Text style={styles.stakingRewardsValue}>+25.75 ETR</Text>
          </View>
        </View>
        <View style={styles.stakingInfo}>
          <View style={styles.stakingInfoRow}>
            <Text style={styles.stakingInfoLabel}>Validator</Text>
            <Text style={styles.stakingInfoValue}>ETRID Foundation</Text>
          </View>
          <View style={styles.stakingInfoRow}>
            <Text style={styles.stakingInfoLabel}>Multiplier</Text>
            <Text style={styles.stakingInfoValuePrimary}>1.25x</Text>
          </View>
        </View>
      </View>

      <View style={styles.stakingActions}>
        <TouchableOpacity style={styles.stakeButton} onPress={onStakePress}>
          <Icon name="lock" size={18} color="#FFFFFF" />
          <Text style={styles.stakeButtonText}>Stake</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.unstakeButton}>
          <Icon name="unlock" size={18} color={colors.primary} />
          <Text style={styles.unstakeButtonText}>Unstake</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.benefitsCard}>
        <Text style={styles.benefitsTitle}>Staking Benefits</Text>
        <BenefitRow icon="zap" title="Voting Power" description="Increase your governance influence" />
        <BenefitRow icon="gift" title="Rewards" description="Earn ETR from network fees" />
        <BenefitRow icon="clock" title="Duration Bonus" description="Up to +20% for long-term staking" />
      </View>
    </View>
  );
}

function BenefitRow({icon, title, description}: {icon: string; title: string; description: string}) {
  return (
    <View style={styles.benefitRow}>
      <Icon name={icon} size={20} color={colors.primary} style={styles.benefitIcon} />
      <View>
        <Text style={styles.benefitTitle}>{title}</Text>
        <Text style={styles.benefitDescription}>{description}</Text>
      </View>
    </View>
  );
}

function RewardsTab({rewards}: {rewards: any[]}) {
  if (rewards.length === 0) {
    return (
      <View style={styles.emptyState}>
        <Icon name="gift" size={48} color={colors.textSecondary} />
        <Text style={styles.emptyStateTitle}>No Pending Rewards</Text>
        <Text style={styles.emptyStateText}>
          Participate in governance to earn rewards
        </Text>
      </View>
    );
  }

  return (
    <View style={styles.tabContent}>
      {rewards.map(reward => (
        <View key={reward.id} style={styles.rewardCard}>
          <View style={styles.rewardIcon}>
            <Icon
              name={reward.type === 'voting' ? 'check-circle' : 'lock'}
              size={20}
              color={colors.primary}
            />
          </View>
          <View style={styles.rewardInfo}>
            <Text style={styles.rewardType}>
              {reward.type === 'voting' ? 'Voting Reward' : 'Staking Reward'}
            </Text>
            <Text style={styles.rewardYear}>
              Consensus Day {reward.consensusDayYear}
            </Text>
          </View>
          <View style={styles.rewardAmountContainer}>
            <Text style={styles.rewardAmount}>{reward.amount.toFixed(4)} ETR</Text>
            {reward.isClaimed ? (
              <Text style={styles.rewardClaimed}>Claimed</Text>
            ) : (
              <TouchableOpacity style={styles.claimButton}>
                <Text style={styles.claimButtonText}>Claim</Text>
              </TouchableOpacity>
            )}
          </View>
        </View>
      ))}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  phaseBanner: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: theme.spacing.md,
    marginHorizontal: theme.spacing.md,
    marginTop: theme.spacing.sm,
    borderRadius: theme.borderRadius.md,
  },
  phaseBannerLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 12,
  },
  phaseBannerText: {},
  phaseBannerTitle: {
    color: '#FFFFFF',
    fontSize: theme.fontSize.md,
    fontWeight: '700',
  },
  phaseBannerSubtitle: {
    color: 'rgba(255,255,255,0.8)',
    fontSize: theme.fontSize.sm,
  },
  phaseBannerRight: {
    alignItems: 'flex-end',
  },
  phaseCountdown: {
    color: '#FFFFFF',
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
  },
  phaseCountdownSub: {
    color: 'rgba(255,255,255,0.8)',
    fontSize: theme.fontSize.xs,
  },
  votingPowerCard: {
    backgroundColor: colors.glass,
    margin: theme.spacing.md,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
  },
  votingPowerMain: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: theme.spacing.md,
  },
  votingPowerLabel: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
    marginBottom: 4,
  },
  votingPowerValue: {
    color: colors.primary,
    fontSize: 28,
    fontWeight: '700',
  },
  votingPowerMultipliers: {
    alignItems: 'flex-end',
    gap: 4,
  },
  multiplierRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  multiplierText: {
    color: colors.success,
    fontSize: theme.fontSize.sm,
  },
  multiplierTextWarning: {
    color: colors.warning,
    fontSize: theme.fontSize.sm,
  },
  votingPowerStats: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    borderTopWidth: 1,
    borderTopColor: colors.border,
    paddingTop: theme.spacing.md,
  },
  statItem: {
    alignItems: 'center',
  },
  statValue: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  statLabel: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
    marginTop: 4,
  },
  tabsContainer: {
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.sm,
    gap: 8,
  },
  tabChip: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    backgroundColor: colors.glass,
    borderRadius: 20,
    marginRight: 8,
  },
  tabChipSelected: {
    backgroundColor: colors.primary,
  },
  tabChipText: {
    color: colors.text,
    fontSize: theme.fontSize.sm,
    fontWeight: '500',
  },
  tabChipTextSelected: {
    color: '#FFFFFF',
    fontWeight: '600',
  },
  contentScrollView: {
    flex: 1,
  },
  tabContent: {
    padding: theme.spacing.md,
  },
  emptyState: {
    alignItems: 'center',
    paddingVertical: 40,
  },
  emptyStateTitle: {
    color: colors.text,
    fontSize: theme.fontSize.lg,
    fontWeight: '600',
    marginTop: theme.spacing.md,
  },
  emptyStateText: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
    textAlign: 'center',
    marginTop: theme.spacing.sm,
  },
  submitProposalButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
    gap: 8,
  },
  submitProposalText: {
    color: '#FFFFFF',
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  proposalCard: {
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  proposalHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  proposalCategoryBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: `${colors.primary}20`,
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 8,
    gap: 4,
  },
  proposalCategoryText: {
    color: colors.primary,
    fontSize: theme.fontSize.xs,
    fontWeight: '500',
  },
  proposalStatusRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  proposalStatusText: {
    color: colors.info,
    fontSize: theme.fontSize.xs,
    fontWeight: '500',
    textTransform: 'capitalize',
  },
  proposalTitle: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    marginBottom: 4,
  },
  proposalDescription: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
    marginBottom: theme.spacing.sm,
  },
  proposalVotes: {
    marginBottom: theme.spacing.sm,
  },
  voteRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 4,
  },
  voteFor: {
    color: colors.success,
    fontSize: theme.fontSize.xs,
  },
  voteAgainst: {
    color: colors.error,
    fontSize: theme.fontSize.xs,
  },
  progressBar: {
    height: 6,
    backgroundColor: `${colors.error}30`,
    borderRadius: 3,
    marginBottom: 4,
  },
  progressFill: {
    height: 6,
    backgroundColor: colors.success,
    borderRadius: 3,
  },
  approvalText: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
    textAlign: 'center',
  },
  timeRemaining: {
    color: colors.warning,
    fontSize: theme.fontSize.xs,
  },
  directorCard: {
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  directorHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  directorAvatar: {
    width: 50,
    height: 50,
    borderRadius: 25,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.sm,
  },
  directorAvatarText: {
    color: '#FFFFFF',
    fontSize: 20,
    fontWeight: '700',
  },
  directorInfo: {
    flex: 1,
  },
  directorName: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  directorAddress: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
  },
  directorManifesto: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
    marginBottom: theme.spacing.sm,
  },
  directorStats: {
    flexDirection: 'row',
    gap: 16,
  },
  directorStatRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  directorStatText: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
  },
  stakingCard: {
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
    marginBottom: theme.spacing.md,
  },
  stakingHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: theme.spacing.md,
  },
  stakingLabel: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
    marginBottom: 4,
  },
  stakingValue: {
    color: colors.text,
    fontSize: 24,
    fontWeight: '700',
  },
  stakingRewards: {
    alignItems: 'flex-end',
  },
  stakingRewardsValue: {
    color: colors.success,
    fontSize: theme.fontSize.lg,
    fontWeight: '600',
  },
  stakingInfo: {
    borderTopWidth: 1,
    borderTopColor: colors.border,
    paddingTop: theme.spacing.md,
  },
  stakingInfoRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 8,
  },
  stakingInfoLabel: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.sm,
  },
  stakingInfoValue: {
    color: colors.text,
    fontSize: theme.fontSize.sm,
  },
  stakingInfoValuePrimary: {
    color: colors.primary,
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
  },
  stakingActions: {
    flexDirection: 'row',
    gap: 12,
    marginBottom: theme.spacing.md,
  },
  stakeButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 8,
  },
  stakeButtonText: {
    color: '#FFFFFF',
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  unstakeButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 8,
  },
  unstakeButtonText: {
    color: colors.primary,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  benefitsCard: {
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.lg,
  },
  benefitsTitle: {
    color: colors.text,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    marginBottom: theme.spacing.md,
  },
  benefitRow: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    marginBottom: 12,
  },
  benefitIcon: {
    marginRight: 12,
    marginTop: 2,
  },
  benefitTitle: {
    color: colors.text,
    fontSize: theme.fontSize.sm,
    fontWeight: '500',
  },
  benefitDescription: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
  },
  rewardCard: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  rewardIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: `${colors.primary}20`,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.sm,
  },
  rewardInfo: {
    flex: 1,
  },
  rewardType: {
    color: colors.text,
    fontSize: theme.fontSize.sm,
    fontWeight: '500',
  },
  rewardYear: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
  },
  rewardAmountContainer: {
    alignItems: 'flex-end',
  },
  rewardAmount: {
    color: colors.success,
    fontSize: theme.fontSize.md,
    fontWeight: '600',
  },
  rewardClaimed: {
    color: colors.textSecondary,
    fontSize: theme.fontSize.xs,
  },
  claimButton: {
    marginTop: 4,
  },
  claimButtonText: {
    color: colors.primary,
    fontSize: theme.fontSize.xs,
    fontWeight: '600',
  },
});
