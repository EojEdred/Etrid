import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TextInput, TouchableOpacity, Alert } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useGovernance } from '../../hooks/useGovernance';
import { ConvictionLevel } from '../../types/defi.types';

export default function DelegateVotesScreen({ navigation }: any) {
  const [delegateAddress, setDelegateAddress] = useState('');
  const [amount, setAmount] = useState('');
  const [conviction, setConviction] = useState<ConvictionLevel>(3);
  const { delegate, delegations, undelegate } = useGovernance();

  const handleDelegate = async () => {
    if (!delegateAddress || !amount) {
      Alert.alert('Error', 'Please fill all fields');
      return;
    }

    const result = await delegate(
      delegateAddress,
      (parseFloat(amount) * 1e12).toString(),
      conviction
    );

    if (result.success) {
      Alert.alert('Success', 'Votes delegated successfully!', [
        { text: 'OK', onPress: () => navigation.goBack() },
      ]);
    } else {
      Alert.alert('Error', result.error || 'Failed to delegate');
    }
  };

  const handleUndelegate = async () => {
    Alert.alert(
      'Confirm Undelegation',
      'Are you sure you want to remove all delegations?',
      [
        { text: 'Cancel', style: 'cancel' },
        {
          text: 'Undelegate',
          style: 'destructive',
          onPress: async () => {
            const result = await undelegate();
            if (result.success) {
              Alert.alert('Success', 'Votes undelegated');
            } else {
              Alert.alert('Error', result.error || 'Failed to undelegate');
            }
          },
        },
      ]
    );
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.title}>Delegate Votes</Text>

      <View style={styles.infoCard}>
        <Text style={styles.infoText}>
          Delegate your voting power to a trusted address. They can vote on your behalf.
        </Text>
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Delegate To (Address)</Text>
        <TextInput
          style={styles.input}
          value={delegateAddress}
          onChangeText={setDelegateAddress}
          placeholder="5Gxy..."
          placeholderTextColor={colors.textSecondary}
          autoCapitalize="none"
        />
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Amount (ETR)</Text>
        <TextInput
          style={styles.input}
          value={amount}
          onChangeText={setAmount}
          placeholder="0.00"
          keyboardType="decimal-pad"
          placeholderTextColor={colors.textSecondary}
        />
      </View>

      <TouchableOpacity style={styles.delegateButton} onPress={handleDelegate}>
        <Text style={styles.delegateButtonText}>Delegate Votes</Text>
      </TouchableOpacity>

      {delegations.length > 0 && (
        <View style={styles.delegationsSection}>
          <Text style={styles.sectionTitle}>Current Delegations</Text>
          {delegations.map((del, index) => (
            <View key={index} style={styles.delegationCard}>
              <View style={styles.delegationInfo}>
                <Text style={styles.delegationAddress}>
                  {del.delegateTo.slice(0, 12)}...{del.delegateTo.slice(-6)}
                </Text>
                <Text style={styles.delegationAmount}>{del.amountETR.toFixed(2)} ETR</Text>
              </View>
            </View>
          ))}
          <TouchableOpacity
            style={styles.undelegateButton}
            onPress={handleUndelegate}
          >
            <Text style={styles.undelegateButtonText}>Undelegate All</Text>
          </TouchableOpacity>
        </View>
      )}
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
  infoCard: {
    backgroundColor: colors.info + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  infoText: {
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
    color: colors.text,
    fontWeight: '600',
    marginBottom: spacing.sm,
  },
  input: {
    ...typography.body,
    color: colors.text,
    borderWidth: 2,
    borderColor: colors.gray200,
    borderRadius: borderRadius.md,
    padding: spacing.md,
  },
  delegateButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginBottom: spacing.xl,
  },
  delegateButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
  delegationsSection: {
    marginTop: spacing.md,
  },
  sectionTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  delegationCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.sm,
  },
  delegationInfo: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  delegationAddress: {
    ...typography.body,
    color: colors.text,
  },
  delegationAmount: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  undelegateButton: {
    backgroundColor: colors.error,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  undelegateButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
