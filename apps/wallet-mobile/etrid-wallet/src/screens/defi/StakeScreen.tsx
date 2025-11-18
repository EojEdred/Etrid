import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TextInput, TouchableOpacity, Alert } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useStaking } from '../../hooks/useStaking';
import { useValidators } from '../../hooks/useValidators';

export default function StakeScreen({ navigation }: any) {
  const [amount, setAmount] = useState('');
  const [selectedValidator, setSelectedValidator] = useState<string | undefined>();
  const [autoCompound, setAutoCompound] = useState(true);
  const { stake, estimateRewards } = useStaking();
  const { getTopValidators } = useValidators();

  const estimate = amount ? estimateRewards(parseFloat(amount)) : null;

  const handleStake = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      Alert.alert('Error', 'Please enter a valid amount');
      return;
    }

    const result = await stake({
      amount: (parseFloat(amount) * 1e12).toString(),
      validatorAddress: selectedValidator,
      autoCompound,
    });

    if (result.success) {
      Alert.alert('Success', 'Tokens staked successfully!', [
        { text: 'OK', onPress: () => navigation.goBack() },
      ]);
    } else {
      Alert.alert('Error', result.error || 'Failed to stake');
    }
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.title}>Stake ETR</Text>

      <View style={styles.card}>
        <Text style={styles.label}>Amount to Stake</Text>
        <View style={styles.inputContainer}>
          <TextInput
            style={styles.input}
            value={amount}
            onChangeText={setAmount}
            placeholder="0.00"
            keyboardType="decimal-pad"
            placeholderTextColor={colors.textSecondary}
          />
          <Text style={styles.currency}>ETR</Text>
        </View>
        <TouchableOpacity style={styles.maxButton}>
          <Text style={styles.maxButtonText}>MAX</Text>
        </TouchableOpacity>
      </View>

      {estimate && (
        <View style={styles.estimateCard}>
          <Text style={styles.estimateTitle}>Estimated Earnings</Text>
          <View style={styles.estimateRow}>
            <Text style={styles.estimateLabel}>Daily</Text>
            <Text style={styles.estimateValue}>{estimate.dailyReward.toFixed(4)} ETR</Text>
          </View>
          <View style={styles.estimateRow}>
            <Text style={styles.estimateLabel}>Monthly</Text>
            <Text style={styles.estimateValue}>{estimate.monthlyReward.toFixed(2)} ETR</Text>
          </View>
          <View style={styles.estimateRow}>
            <Text style={styles.estimateLabel}>Yearly</Text>
            <Text style={[styles.estimateValue, { color: colors.success }]}>
              {estimate.yearlyReward.toFixed(2)} ETR
            </Text>
          </View>
          <Text style={styles.apyText}>APY: {estimate.effectiveAPY.toFixed(1)}%</Text>
        </View>
      )}

      <View style={styles.card}>
        <Text style={styles.label}>Validator Selection</Text>
        <TouchableOpacity
          style={styles.validatorButton}
          onPress={() => navigation.navigate('ValidatorList', { selectMode: true })}
        >
          <Text style={styles.validatorButtonText}>
            {selectedValidator ? 'Change Validator' : 'Auto-select (Recommended)'}
          </Text>
        </TouchableOpacity>
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Rewards Preference</Text>
        <View style={styles.optionRow}>
          <TouchableOpacity
            style={[styles.option, autoCompound && styles.optionSelected]}
            onPress={() => setAutoCompound(true)}
          >
            <Text style={[styles.optionText, autoCompound && styles.optionTextSelected]}>
              Auto-compound
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.option, !autoCompound && styles.optionSelected]}
            onPress={() => setAutoCompound(false)}
          >
            <Text style={[styles.optionText, !autoCompound && styles.optionTextSelected]}>
              Send to Wallet
            </Text>
          </TouchableOpacity>
        </View>
      </View>

      <TouchableOpacity style={styles.stakeButton} onPress={handleStake}>
        <Text style={styles.stakeButtonText}>Stake ETR</Text>
      </TouchableOpacity>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.lg,
  },
  title: {
    ...typography.h1,
    color: colors.text,
    marginBottom: spacing.lg,
  },
  card: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  label: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
    marginBottom: spacing.sm,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    borderWidth: 2,
    borderColor: colors.gray200,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
  },
  input: {
    flex: 1,
    ...typography.h2,
    color: colors.text,
  },
  currency: {
    ...typography.h3,
    color: colors.textSecondary,
  },
  maxButton: {
    position: 'absolute',
    right: spacing.md,
    top: spacing.md,
    backgroundColor: colors.primary,
    paddingHorizontal: spacing.sm,
    paddingVertical: 4,
    borderRadius: borderRadius.sm,
  },
  maxButtonText: {
    ...typography.caption,
    color: colors.background,
    fontWeight: '600',
  },
  estimateCard: {
    backgroundColor: colors.primary + '10',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  estimateTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.sm,
  },
  estimateRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.xs,
  },
  estimateLabel: {
    ...typography.body,
    color: colors.textSecondary,
  },
  estimateValue: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  apyText: {
    ...typography.caption,
    color: colors.primary,
    marginTop: spacing.sm,
    textAlign: 'center',
  },
  validatorButton: {
    backgroundColor: colors.gray100,
    borderRadius: borderRadius.md,
    padding: spacing.md,
  },
  validatorButtonText: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
    textAlign: 'center',
  },
  optionRow: {
    flexDirection: 'row',
  },
  option: {
    flex: 1,
    backgroundColor: colors.gray100,
    borderRadius: borderRadius.md,
    padding: spacing.md,
    marginRight: spacing.sm,
    alignItems: 'center',
  },
  optionSelected: {
    backgroundColor: colors.primary,
  },
  optionText: {
    ...typography.body,
    color: colors.text,
  },
  optionTextSelected: {
    color: colors.background,
    fontWeight: '600',
  },
  stakeButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  stakeButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
