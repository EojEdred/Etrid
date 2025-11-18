import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TextInput, TouchableOpacity, Alert } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useLightning } from '../../hooks/useLightning';

export default function OpenChannelScreen({ navigation }: any) {
  const [counterparty, setCounterparty] = useState('');
  const [capacity, setCapacity] = useState('');
  const { openChannel, estimateOpenChannelFee } = useLightning();

  const fees = capacity ? estimateOpenChannelFee((parseFloat(capacity) * 1e12).toString()) : null;

  const handleOpenChannel = async () => {
    if (!counterparty || !capacity) {
      Alert.alert('Error', 'Please fill all fields');
      return;
    }

    if (parseFloat(capacity) <= 0) {
      Alert.alert('Error', 'Capacity must be greater than 0');
      return;
    }

    const result = await openChannel({
      counterparty,
      capacity: (parseFloat(capacity) * 1e12).toString(),
    });

    if (result.success) {
      Alert.alert('Success', result.message || 'Channel opening successfully!', [
        { text: 'OK', onPress: () => navigation.goBack() },
      ]);
    } else {
      Alert.alert('Error', result.error || 'Failed to open channel');
    }
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.title}>Open Lightning Channel</Text>

      <View style={styles.infoCard}>
        <Text style={styles.infoTitle}>What is a Lightning Channel?</Text>
        <Text style={styles.infoText}>
          Lightning channels enable instant, low-fee payments between you and another address. Lock
          funds in a channel and make unlimited payments until you close it.
        </Text>
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Counterparty Address</Text>
        <TextInput
          style={styles.input}
          value={counterparty}
          onChangeText={setCounterparty}
          placeholder="5Gxy..."
          placeholderTextColor={colors.textSecondary}
          autoCapitalize="none"
        />
      </View>

      <View style={styles.card}>
        <Text style={styles.label}>Channel Capacity (ETR)</Text>
        <TextInput
          style={styles.input}
          value={capacity}
          onChangeText={setCapacity}
          placeholder="0.00"
          keyboardType="decimal-pad"
          placeholderTextColor={colors.textSecondary}
        />
        <Text style={styles.hint}>Amount to lock in the channel</Text>
      </View>

      {fees && (
        <View style={styles.feesCard}>
          <Text style={styles.feesTitle}>Estimated Fees</Text>
          <View style={styles.feeRow}>
            <Text style={styles.feeLabel}>Opening Fee</Text>
            <Text style={styles.feeValue}>{fees.openingFee.toFixed(4)} ETR</Text>
          </View>
          <View style={styles.feeRow}>
            <Text style={styles.feeLabel}>Closing Fee (when closing)</Text>
            <Text style={styles.feeValue}>{fees.closingFee.toFixed(4)} ETR</Text>
          </View>
          <View style={styles.divider} />
          <View style={styles.feeRow}>
            <Text style={styles.feeTotalLabel}>Total Initial Cost</Text>
            <Text style={styles.feeTotalValue}>
              {(parseFloat(capacity) + fees.openingFee).toFixed(4)} ETR
            </Text>
          </View>
        </View>
      )}

      <TouchableOpacity style={styles.openButton} onPress={handleOpenChannel}>
        <Text style={styles.openButtonText}>Open Channel</Text>
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
  infoCard: {
    backgroundColor: colors.info + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  infoTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.sm,
  },
  infoText: {
    ...typography.body,
    color: colors.text,
    lineHeight: 22,
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
  hint: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  feesCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  feesTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  feeRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.sm,
  },
  feeLabel: {
    ...typography.body,
    color: colors.textSecondary,
  },
  feeValue: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  divider: {
    height: 1,
    backgroundColor: colors.gray200,
    marginVertical: spacing.sm,
  },
  feeTotalLabel: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  feeTotalValue: {
    ...typography.h3,
    color: colors.primary,
  },
  openButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  openButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
