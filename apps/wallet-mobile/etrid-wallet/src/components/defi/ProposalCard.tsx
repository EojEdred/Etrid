import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { Proposal } from '../../types/defi.types';

interface ProposalCardProps {
  proposal: Proposal;
  onPress?: () => void;
}

export function ProposalCard({ proposal, onPress }: ProposalCardProps) {
  const approvalPercentage = proposal.currentApproval;

  return (
    <TouchableOpacity style={styles.container} onPress={onPress} activeOpacity={0.7}>
      <View style={styles.header}>
        <Text style={styles.proposalId}>#{proposal.id}</Text>
        <Text style={styles.timeRemaining}>{proposal.timeRemaining}</Text>
      </View>

      <Text style={styles.title} numberOfLines={2}>
        {proposal.title}
      </Text>

      <View style={styles.progressContainer}>
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
        <View style={styles.progressLabels}>
          <Text style={[styles.progressText, { color: colors.success }]}>
            YES {approvalPercentage.toFixed(1)}%
          </Text>
          <Text style={[styles.progressText, { color: colors.error }]}>
            NO {(100 - approvalPercentage).toFixed(1)}%
          </Text>
        </View>
      </View>

      {proposal.hasVoted && (
        <View style={styles.votedBadge}>
          <Text style={styles.votedText}>
            Voted {proposal.userVote?.toUpperCase()}
          </Text>
        </View>
      )}
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  proposalId: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  timeRemaining: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  title: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  progressContainer: {
    marginTop: spacing.sm,
  },
  progressBar: {
    height: 8,
    backgroundColor: colors.gray200,
    borderRadius: borderRadius.full,
    overflow: 'hidden',
    marginBottom: spacing.sm,
  },
  progressFill: {
    height: '100%',
  },
  progressLabels: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  progressText: {
    ...typography.caption,
    fontWeight: '600',
  },
  votedBadge: {
    marginTop: spacing.sm,
    alignSelf: 'flex-start',
    backgroundColor: colors.primary + '20',
    paddingHorizontal: spacing.sm,
    paddingVertical: 4,
    borderRadius: borderRadius.sm,
  },
  votedText: {
    ...typography.caption,
    color: colors.primary,
    fontWeight: '600',
  },
});
