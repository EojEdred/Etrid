import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, TouchableOpacity, Alert } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useGovernance } from '../../hooks/useGovernance';
import { ConvictionSelector } from '../../components/defi/ConvictionSelector';
import { VotingPowerIndicator } from '../../components/defi/VotingPowerIndicator';
import { ConvictionLevel } from '../../types/defi.types';

export default function VoteScreen({ route, navigation }: any) {
  const { proposal, vote: initialVote } = route.params;
  const [conviction, setConviction] = useState<ConvictionLevel>(3);
  const { vote, votingPower } = useGovernance();

  const handleVote = async () => {
    const result = await vote({
      proposalId: proposal.id,
      vote: initialVote,
      conviction,
    });

    if (result.success) {
      Alert.alert('Success', 'Voted ' + initialVote.toUpperCase() + ' successfully!', [
        { text: 'OK', onPress: () => navigation.popToTop() },
      ]);
    } else {
      Alert.alert('Error', result.error || 'Failed to vote');
    }
  };

  const voteColor = initialVote === 'yes' ? colors.success : colors.error;

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.title}>Cast Your Vote</Text>

      <View style={styles.proposalCard}>
        <Text style={styles.proposalTitle}>{proposal.title}</Text>
        <Text style={styles.proposalId}>Proposal #{proposal.id}</Text>
      </View>

      <View style={styles.voteCard}>
        <Text style={styles.voteLabel}>Your Vote</Text>
        <Text style={[styles.voteValue, { color: voteColor }]}>
          {initialVote.toUpperCase()}
        </Text>
      </View>

      <ConvictionSelector selected={conviction} onSelect={setConviction} />

      {votingPower && (
        <VotingPowerIndicator
          balance={votingPower.availableBalanceETR}
          conviction={conviction}
        />
      )}

      <TouchableOpacity
        style={[styles.submitButton, { backgroundColor: voteColor }]}
        onPress={handleVote}
      >
        <Text style={styles.submitButtonText}>Submit Vote</Text>
      </TouchableOpacity>

      <Text style={styles.disclaimer}>
        Your vote is final and cannot be changed. Make sure you understand the proposal before voting.
      </Text>
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
  proposalCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  proposalTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  proposalId: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  voteCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.lg,
    marginBottom: spacing.md,
    alignItems: 'center',
  },
  voteLabel: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  voteValue: {
    ...typography.h1,
    fontWeight: 'bold',
  },
  submitButton: {
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginTop: spacing.lg,
  },
  submitButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
  disclaimer: {
    ...typography.caption,
    color: colors.textSecondary,
    textAlign: 'center',
    marginTop: spacing.md,
  },
});
