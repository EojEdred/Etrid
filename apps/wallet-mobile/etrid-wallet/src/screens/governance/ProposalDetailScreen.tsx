import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet, ScrollView, TouchableOpacity, ActivityIndicator } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useGovernance } from '../../hooks/useGovernance';
import { Proposal } from '../../types/defi.types';

export default function ProposalDetailScreen({ route, navigation }: any) {
  const { proposalId } = route.params;
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const { getProposal } = useGovernance();

  useEffect(() => {
    loadProposal();
  }, [proposalId]);

  const loadProposal = async () => {
    const data = await getProposal(proposalId);
    setProposal(data);
  };

  if (!proposal) {
    return (
      <View style={styles.loading}>
        <ActivityIndicator size="large" color={colors.primary} />
      </View>
    );
  }

  const approvalPercentage = proposal.currentApproval;

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.proposalId}>Proposal #{proposal.id}</Text>
        <Text style={styles.timeRemaining}>{proposal.timeRemaining}</Text>
      </View>

      <Text style={styles.title}>{proposal.title}</Text>

      <View style={styles.progressCard}>
        <View style={styles.progressBar}>
          <View
            style={[
              styles.progressFill,
              {
                width: `${approvalPercentage}%`,
                backgroundColor:
                  approvalPercentage >= proposal.threshold ? colors.success : colors.warning,
              },
            ]}
          />
        </View>
        <View style={styles.voteStats}>
          <View style={styles.voteStat}>
            <Text style={[styles.voteLabel, { color: colors.success }]}>YES</Text>
            <Text style={styles.voteValue}>{approvalPercentage.toFixed(1)}%</Text>
            <Text style={styles.voteAmount}>{proposal.votesYesETR.toLocaleString()} ETR</Text>
          </View>
          <View style={styles.voteStat}>
            <Text style={[styles.voteLabel, { color: colors.error }]}>NO</Text>
            <Text style={styles.voteValue}>{(100 - approvalPercentage).toFixed(1)}%</Text>
            <Text style={styles.voteAmount}>{proposal.votesNoETR.toLocaleString()} ETR</Text>
          </View>
        </View>
      </View>

      <View style={styles.section}>
        <Text style={styles.sectionTitle}>Description</Text>
        <Text style={styles.description}>{proposal.description}</Text>
      </View>

      {proposal.impactSummary && (
        <View style={styles.impactCard}>
          <Text style={styles.impactTitle}>Impact</Text>
          <Text style={styles.impactText}>{proposal.impactSummary}</Text>
        </View>
      )}

      {!proposal.hasVoted && (
        <View style={styles.actions}>
          <TouchableOpacity
            style={[styles.voteButton, { backgroundColor: colors.success }]}
            onPress={() => navigation.navigate('Vote', { proposal, vote: 'yes' })}
          >
            <Text style={styles.voteButtonText}>Vote YES</Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.voteButton, { backgroundColor: colors.error }]}
            onPress={() => navigation.navigate('Vote', { proposal, vote: 'no' })}
          >
            <Text style={styles.voteButtonText}>Vote NO</Text>
          </TouchableOpacity>
        </View>
      )}

      {proposal.hasVoted && (
        <View style={styles.votedBanner}>
          <Text style={styles.votedText}>
            You voted {proposal.userVote?.toUpperCase()} on this proposal
          </Text>
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
  loading: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  proposalId: {
    ...typography.h3,
    color: colors.primary,
  },
  timeRemaining: {
    ...typography.bodySmall,
    color: colors.textSecondary,
  },
  title: {
    ...typography.h1,
    color: colors.text,
    marginBottom: spacing.lg,
  },
  progressCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  progressBar: {
    height: 12,
    backgroundColor: colors.gray200,
    borderRadius: borderRadius.full,
    overflow: 'hidden',
    marginBottom: spacing.md,
  },
  progressFill: {
    height: '100%',
  },
  voteStats: {
    flexDirection: 'row',
    justifyContent: 'space-around',
  },
  voteStat: {
    alignItems: 'center',
  },
  voteLabel: {
    ...typography.caption,
    fontWeight: '600',
    marginBottom: 4,
  },
  voteValue: {
    ...typography.h2,
    color: colors.text,
    marginBottom: 4,
  },
  voteAmount: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  section: {
    marginBottom: spacing.md,
  },
  sectionTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.sm,
  },
  description: {
    ...typography.body,
    color: colors.text,
    lineHeight: 24,
  },
  impactCard: {
    backgroundColor: colors.info + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  impactTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.sm,
  },
  impactText: {
    ...typography.body,
    color: colors.text,
  },
  actions: {
    flexDirection: 'row',
    marginTop: spacing.lg,
  },
  voteButton: {
    flex: 1,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
    marginHorizontal: spacing.xs,
  },
  voteButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
  votedBanner: {
    backgroundColor: colors.primary + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginTop: spacing.lg,
    alignItems: 'center',
  },
  votedText: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
});
