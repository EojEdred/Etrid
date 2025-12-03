import React, {useState} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  Alert,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * ProposalDetailScreen - View and vote on governance proposals
 * Matches iOS Swift ProposalDetailView
 */
export default function ProposalDetailScreen({route, navigation}: any) {
  const {
    id = 'EIP-001',
    title = 'Sample Proposal',
    status = 'active',
    yesVotes = 65,
    noVotes = 35,
  } = route.params || {};

  const [hasVoted, setHasVoted] = useState(false);
  const [userVote, setUserVote] = useState<'yes' | 'no' | null>(null);

  const handleVote = (vote: 'yes' | 'no') => {
    Alert.alert(
      'Confirm Vote',
      `Are you sure you want to vote "${vote === 'yes' ? 'Yes' : 'No'}" on this proposal?`,
      [
        {text: 'Cancel', style: 'cancel'},
        {
          text: 'Confirm',
          onPress: () => {
            setHasVoted(true);
            setUserVote(vote);
            Alert.alert('Vote Recorded', 'Your vote has been submitted successfully.');
          },
        },
      ]
    );
  };

  const getStatusColor = () => {
    switch (status) {
      case 'active': return colors.success;
      case 'passed': return colors.info;
      case 'rejected': return colors.error;
      case 'pending': return colors.warning;
      default: return colors.textSecondary;
    }
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Proposal Header */}
        <View style={styles.header}>
          <View style={[styles.statusBadge, {backgroundColor: getStatusColor() + '30'}]}>
            <Text style={[styles.statusText, {color: getStatusColor()}]}>
              {status.toUpperCase()}
            </Text>
          </View>
          <Text style={styles.proposalId}>{id}</Text>
          <Text style={styles.proposalTitle}>{title}</Text>
        </View>

        {/* Voting Progress */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Current Votes</Text>

          <View style={styles.voteBar}>
            <View style={[styles.yesBar, {flex: yesVotes}]} />
            <View style={[styles.noBar, {flex: noVotes}]} />
          </View>

          <View style={styles.voteLabels}>
            <View style={styles.voteLabel}>
              <View style={[styles.voteDot, {backgroundColor: colors.success}]} />
              <Text style={styles.voteLabelText}>Yes: {yesVotes}%</Text>
            </View>
            <View style={styles.voteLabel}>
              <View style={[styles.voteDot, {backgroundColor: colors.error}]} />
              <Text style={styles.voteLabelText}>No: {noVotes}%</Text>
            </View>
          </View>
        </View>

        {/* Proposal Details */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Description</Text>
          <Text style={styles.description}>
            This proposal outlines changes to the Ëtrid network governance parameters.
            It includes modifications to voting thresholds, quorum requirements, and
            proposal submission guidelines to improve the democratic process.
          </Text>
        </View>

        {/* Proposal Info */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Details</Text>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Proposer</Text>
            <Text style={styles.infoValue}>0x1234...5678</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Created</Text>
            <Text style={styles.infoValue}>Dec 1, 2024</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Voting Ends</Text>
            <Text style={styles.infoValue}>Dec 8, 2024</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Quorum Required</Text>
            <Text style={styles.infoValue}>51%</Text>
          </View>
          <View style={styles.infoRow}>
            <Text style={styles.infoLabel}>Total Votes</Text>
            <Text style={styles.infoValue}>1,234,567 ETR</Text>
          </View>
        </View>

        {/* Your Vote */}
        {hasVoted ? (
          <View style={styles.card}>
            <Text style={styles.cardTitle}>Your Vote</Text>
            <View style={styles.votedInfo}>
              <Icon
                name={userVote === 'yes' ? 'check-circle' : 'x-circle'}
                size={32}
                color={userVote === 'yes' ? colors.success : colors.error}
              />
              <Text style={styles.votedText}>
                You voted {userVote === 'yes' ? 'Yes' : 'No'}
              </Text>
            </View>
          </View>
        ) : status === 'active' ? (
          <View style={styles.voteActions}>
            <TouchableOpacity
              style={[styles.voteButton, styles.yesButton]}
              onPress={() => handleVote('yes')}>
              <Icon name="check" size={20} color="#FFFFFF" />
              <Text style={styles.voteButtonText}>Vote Yes</Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[styles.voteButton, styles.noButton]}
              onPress={() => handleVote('no')}>
              <Icon name="x" size={20} color="#FFFFFF" />
              <Text style={styles.voteButtonText}>Vote No</Text>
            </TouchableOpacity>
          </View>
        ) : (
          <View style={styles.votingClosedBadge}>
            <Icon name="lock" size={16} color={colors.textSecondary} />
            <Text style={styles.votingClosedText}>Voting has ended</Text>
          </View>
        )}

        {/* Discussion Link */}
        <TouchableOpacity
          style={styles.discussionLink}
          onPress={() => Alert.alert('Forum', 'Discussion forum coming soon')}>
          <Icon name="message-circle" size={20} color={colors.primary} />
          <Text style={styles.discussionLinkText}>View Discussion</Text>
          <Icon name="external-link" size={16} color={colors.textSecondary} />
        </TouchableOpacity>
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
  header: {
    marginBottom: theme.spacing.lg,
  },
  statusBadge: {
    alignSelf: 'flex-start',
    paddingHorizontal: theme.spacing.sm,
    paddingVertical: 4,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.sm,
  },
  statusText: {
    fontSize: theme.fontSize.xs,
    fontWeight: '700',
  },
  proposalId: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 4,
  },
  proposalTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: '700',
    color: colors.text,
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
  voteBar: {
    flexDirection: 'row',
    height: 8,
    borderRadius: 4,
    overflow: 'hidden',
    backgroundColor: colors.border,
  },
  yesBar: {
    backgroundColor: colors.success,
  },
  noBar: {
    backgroundColor: colors.error,
  },
  voteLabels: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: theme.spacing.sm,
  },
  voteLabel: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
  },
  voteDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
  },
  voteLabelText: {
    fontSize: theme.fontSize.sm,
    color: colors.text,
    fontWeight: '600',
  },
  description: {
    fontSize: theme.fontSize.md,
    color: colors.text,
    lineHeight: 24,
  },
  infoRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: theme.spacing.xs,
  },
  infoLabel: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  infoValue: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  votedInfo: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: theme.spacing.md,
  },
  votedText: {
    fontSize: theme.fontSize.lg,
    fontWeight: '600',
    color: colors.text,
  },
  voteActions: {
    flexDirection: 'row',
    gap: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  voteButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 8,
  },
  yesButton: {
    backgroundColor: colors.success,
  },
  noButton: {
    backgroundColor: colors.error,
  },
  voteButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: '#FFFFFF',
  },
  votingClosedBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.md,
    gap: 8,
  },
  votingClosedText: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  discussionLink: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
    gap: 8,
  },
  discussionLinkText: {
    fontSize: theme.fontSize.md,
    color: colors.primary,
    fontWeight: '600',
  },
});
