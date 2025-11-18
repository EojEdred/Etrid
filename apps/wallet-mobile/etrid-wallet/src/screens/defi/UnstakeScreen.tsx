import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TextInput, TouchableOpacity, Alert } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useStaking } from '../../hooks/useStaking';

export default function UnstakeScreen({ navigation }: any) {
  const [amount, setAmount] = useState('');
  const { stakingInfo, unstake } = useStaking();

  const handleUnstake = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      Alert.alert('Error', 'Please enter a valid amount');
      return;
    }

    const result = await unstake((parseFloat(amount) * 1e12).toString());

    if (result.success) {
      Alert.alert('Success', result.message || 'Unstaking initiated', [
        { text: 'OK', onPress: () => navigation.goBack() },
      ]);
    } else {
      Alert.alert('Error', result.error || 'Failed to unstake');
    }
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.title}>Unstake ETR</Text>

      <View style={styles.warningCard}>
        <Text style={styles.warningTitle}>⚠️ Important</Text>
        <Text style={styles.warningText}>
          Unstaking has a {stakingInfo?.unbondingPeriod || 28} day waiting period. Your tokens will be locked and you won't earn rewards during this time.
        </Text>
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Current Stake</Text>
        <Text style={styles.currentStake}>
          {stakingInfo?.totalStakedETR.toFixed(2) || '0.00'} ETR
        </Text>
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Amount to Unstake</Text>
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
      </View>

      <TouchableOpacity style={styles.unstakeButton} onPress={handleUnstake}>
        <Text style={styles.unstakeButtonText}>Unstake ETR</Text>
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
  warningCard: {
    backgroundColor: colors.warning + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
    borderWidth: 1,
    borderColor: colors.warning,
  },
  warningTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.sm,
  },
  warningText: {
    ...typography.body,
    color: colors.text,
  },
  card: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  label: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.sm,
  },
  currentStake: {
    ...typography.h2,
    color: colors.text,
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
  unstakeButton: {
    backgroundColor: colors.error,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  unstakeButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
