import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  Alert,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * StakingScreen - Stake ETR tokens for governance voting power
 * Matches iOS Swift StakingView
 */

interface StakeOption {
  id: string;
  duration: string;
  multiplier: string;
  apy: string;
}

export default function StakingScreen({navigation}: any) {
  const [amount, setAmount] = useState('');
  const [selectedOption, setSelectedOption] = useState<string>('flexible');

  const stakeOptions: StakeOption[] = [
    {id: 'flexible', duration: 'Flexible', multiplier: '1x', apy: '5%'},
    {id: '30days', duration: '30 Days', multiplier: '1.5x', apy: '8%'},
    {id: '90days', duration: '90 Days', multiplier: '2x', apy: '12%'},
    {id: '365days', duration: '1 Year', multiplier: '3x', apy: '18%'},
  ];

  const mockStakingData = {
    totalStaked: '10,500',
    votingPower: '21,000',
    pendingRewards: '125.50',
    nextReward: '2h 15m',
  };

  const handleStake = () => {
    if (!amount || parseFloat(amount) <= 0) {
      Alert.alert('Invalid Amount', 'Please enter a valid amount to stake.');
      return;
    }

    const option = stakeOptions.find(o => o.id === selectedOption);
    Alert.alert(
      'Confirm Stake',
      `Stake ${amount} ETR for ${option?.duration}?\n\nVoting Power: ${option?.multiplier}\nEstimated APY: ${option?.apy}`,
      [
        {text: 'Cancel', style: 'cancel'},
        {
          text: 'Stake',
          onPress: () => {
            Alert.alert('Staking Successful', 'Your ETR has been staked successfully.');
            setAmount('');
          },
        },
      ]
    );
  };

  const handleUnstake = () => {
    Alert.alert(
      'Unstake ETR',
      'Are you sure you want to unstake? Early unstaking may result in reduced rewards.',
      [
        {text: 'Cancel', style: 'cancel'},
        {text: 'Unstake', style: 'destructive', onPress: () => {
          Alert.alert('Unstaking Initiated', 'Your unstaking request has been submitted.');
        }},
      ]
    );
  };

  const handleClaimRewards = () => {
    Alert.alert(
      'Claim Rewards',
      `Claim ${mockStakingData.pendingRewards} ETR in pending rewards?`,
      [
        {text: 'Cancel', style: 'cancel'},
        {text: 'Claim', onPress: () => {
          Alert.alert('Rewards Claimed', 'Your rewards have been claimed successfully.');
        }},
      ]
    );
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Staking Overview */}
        <View style={styles.overviewCard}>
          <Text style={styles.overviewTitle}>Your Staking</Text>
          <View style={styles.statsGrid}>
            <View style={styles.statItem}>
              <Text style={styles.statValue}>{mockStakingData.totalStaked}</Text>
              <Text style={styles.statLabel}>ETR Staked</Text>
            </View>
            <View style={styles.statItem}>
              <Text style={styles.statValue}>{mockStakingData.votingPower}</Text>
              <Text style={styles.statLabel}>Voting Power</Text>
            </View>
          </View>
        </View>

        {/* Pending Rewards */}
        <View style={styles.rewardsCard}>
          <View style={styles.rewardsInfo}>
            <View>
              <Text style={styles.rewardsLabel}>Pending Rewards</Text>
              <Text style={styles.rewardsValue}>{mockStakingData.pendingRewards} ETR</Text>
              <Text style={styles.rewardsTimer}>Next reward in {mockStakingData.nextReward}</Text>
            </View>
            <TouchableOpacity style={styles.claimButton} onPress={handleClaimRewards}>
              <Text style={styles.claimButtonText}>Claim</Text>
            </TouchableOpacity>
          </View>
        </View>

        {/* Stake Amount */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Stake Amount</Text>
          <View style={styles.inputContainer}>
            <TextInput
              style={styles.amountInput}
              value={amount}
              onChangeText={setAmount}
              placeholder="0.00"
              placeholderTextColor={colors.textMuted}
              keyboardType="decimal-pad"
            />
            <View style={styles.inputRight}>
              <Text style={styles.tokenSymbol}>ETR</Text>
              <TouchableOpacity style={styles.maxButton}>
                <Text style={styles.maxButtonText}>MAX</Text>
              </TouchableOpacity>
            </View>
          </View>
          <Text style={styles.balanceText}>Available: 5,000 ETR</Text>
        </View>

        {/* Lock Duration */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Lock Duration</Text>
          {stakeOptions.map(option => (
            <TouchableOpacity
              key={option.id}
              style={[
                styles.optionRow,
                selectedOption === option.id && styles.optionRowActive,
              ]}
              onPress={() => setSelectedOption(option.id)}>
              <View style={styles.optionLeft}>
                <Icon
                  name={selectedOption === option.id ? 'check-circle' : 'circle'}
                  size={20}
                  color={selectedOption === option.id ? colors.primary : colors.textSecondary}
                />
                <View>
                  <Text style={styles.optionDuration}>{option.duration}</Text>
                  <Text style={styles.optionMultiplier}>{option.multiplier} Voting Power</Text>
                </View>
              </View>
              <Text style={styles.optionApy}>{option.apy} APY</Text>
            </TouchableOpacity>
          ))}
        </View>

        {/* Stake Summary */}
        {amount && parseFloat(amount) > 0 && (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Summary</Text>
            <View style={styles.summaryRow}>
              <Text style={styles.summaryLabel}>You'll Stake</Text>
              <Text style={styles.summaryValue}>{amount} ETR</Text>
            </View>
            <View style={styles.summaryRow}>
              <Text style={styles.summaryLabel}>Voting Power</Text>
              <Text style={styles.summaryValue}>
                {(parseFloat(amount) * parseFloat(stakeOptions.find(o => o.id === selectedOption)?.multiplier || '1')).toFixed(2)}
              </Text>
            </View>
            <View style={styles.summaryRow}>
              <Text style={styles.summaryLabel}>Estimated Yearly Rewards</Text>
              <Text style={styles.summaryValue}>
                ~{(parseFloat(amount) * parseFloat(stakeOptions.find(o => o.id === selectedOption)?.apy || '5') / 100).toFixed(2)} ETR
              </Text>
            </View>
          </View>
        )}

        {/* Action Buttons */}
        <TouchableOpacity style={styles.stakeButton} onPress={handleStake}>
          <Icon name="lock" size={20} color={colors.text} />
          <Text style={styles.stakeButtonText}>Stake ETR</Text>
        </TouchableOpacity>

        <TouchableOpacity style={styles.unstakeButton} onPress={handleUnstake}>
          <Icon name="unlock" size={20} color={colors.error} />
          <Text style={styles.unstakeButtonText}>Unstake</Text>
        </TouchableOpacity>

        {/* Info Section */}
        <View style={styles.infoSection}>
          <Text style={styles.infoTitle}>About Staking</Text>
          <Text style={styles.infoText}>
            Staking ETR gives you voting power in Ëtrid governance. Longer lock
            periods provide higher voting multipliers and APY rewards. Staked ETR
            contributes to network security and decentralization.
          </Text>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
    padding: theme.spacing.md,
  },
  overviewCard: {
    backgroundColor: colors.primary,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.lg,
    marginBottom: theme.spacing.md,
  },
  overviewTitle: {
    fontSize: theme.fontSize.md,
    color: colors.text,
    opacity: 0.8,
    marginBottom: theme.spacing.md,
  },
  statsGrid: {
    flexDirection: 'row',
    gap: theme.spacing.lg,
  },
  statItem: {
    flex: 1,
  },
  statValue: {
    fontSize: 28,
    fontWeight: '700',
    color: colors.text,
  },
  statLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.text,
    opacity: 0.7,
  },
  rewardsCard: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  rewardsInfo: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  rewardsLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  rewardsValue: {
    fontSize: theme.fontSize.xl,
    fontWeight: '700',
    color: colors.success,
  },
  rewardsTimer: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: 4,
  },
  claimButton: {
    backgroundColor: colors.success,
    paddingHorizontal: theme.spacing.lg,
    paddingVertical: theme.spacing.sm,
    borderRadius: theme.borderRadius.sm,
  },
  claimButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: '#FFFFFF',
  },
  card: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  cardTitle: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    paddingHorizontal: theme.spacing.md,
  },
  amountInput: {
    flex: 1,
    fontSize: 24,
    fontWeight: '600',
    color: colors.text,
    paddingVertical: theme.spacing.md,
  },
  inputRight: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: theme.spacing.sm,
  },
  tokenSymbol: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.textSecondary,
  },
  maxButton: {
    backgroundColor: colors.primary,
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: 6,
    borderRadius: theme.borderRadius.sm,
  },
  maxButtonText: {
    fontSize: theme.fontSize.xs,
    fontWeight: '700',
    color: colors.text,
  },
  balanceText: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: theme.spacing.sm,
  },
  optionRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    padding: theme.spacing.md,
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.sm,
  },
  optionRowActive: {
    borderWidth: 1,
    borderColor: colors.primary,
  },
  optionLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: theme.spacing.md,
  },
  optionDuration: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  optionMultiplier: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  optionApy: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: colors.success,
  },
  summaryRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: theme.spacing.xs,
  },
  summaryLabel: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  summaryValue: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  stakeButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
    gap: 8,
  },
  stakeButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: colors.text,
  },
  unstakeButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: `${colors.error}20`,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.lg,
    gap: 8,
  },
  unstakeButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.error,
  },
  infoSection: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.xl,
  },
  infoTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
    marginBottom: theme.spacing.sm,
  },
  infoText: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    lineHeight: 20,
  },
});
