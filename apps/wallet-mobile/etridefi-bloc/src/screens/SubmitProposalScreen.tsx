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
 * SubmitProposalScreen - Submit new governance proposals
 * Matches iOS Swift SubmitProposalView
 */

type ProposalCategory = 'governance' | 'technical' | 'treasury' | 'community';

export default function SubmitProposalScreen({navigation}: any) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [category, setCategory] = useState<ProposalCategory>('governance');
  const [discussionLink, setDiscussionLink] = useState('');

  const categories: {id: ProposalCategory; name: string; icon: string}[] = [
    {id: 'governance', name: 'Governance', icon: 'users'},
    {id: 'technical', name: 'Technical', icon: 'code'},
    {id: 'treasury', name: 'Treasury', icon: 'dollar-sign'},
    {id: 'community', name: 'Community', icon: 'heart'},
  ];

  const requirements = [
    {met: title.length >= 10, text: 'Title at least 10 characters'},
    {met: description.length >= 100, text: 'Description at least 100 characters'},
    {met: true, text: 'Hold minimum 1,000 ETR stake'},
    {met: true, text: 'Account verified for 30+ days'},
  ];

  const allRequirementsMet = requirements.every(r => r.met);

  const handleSubmit = () => {
    if (!allRequirementsMet) {
      Alert.alert('Requirements Not Met', 'Please complete all requirements before submitting.');
      return;
    }

    Alert.alert(
      'Submit Proposal',
      'Are you sure you want to submit this proposal? A deposit of 100 ETR will be required.',
      [
        {text: 'Cancel', style: 'cancel'},
        {
          text: 'Submit',
          onPress: () => {
            Alert.alert(
              'Proposal Submitted',
              'Your proposal has been submitted for review. It will be visible in the Proposals list once approved.',
              [{text: 'OK', onPress: () => navigation.goBack()}]
            );
          },
        },
      ]
    );
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Header */}
        <View style={styles.header}>
          <Icon name="edit-3" size={32} color={colors.primary} />
          <Text style={styles.headerTitle}>New Proposal</Text>
          <Text style={styles.headerSubtitle}>
            Submit a proposal for community governance
          </Text>
        </View>

        {/* Title Input */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Proposal Title</Text>
          <TextInput
            style={styles.textInput}
            value={title}
            onChangeText={setTitle}
            placeholder="Enter a clear, descriptive title"
            placeholderTextColor={colors.textMuted}
            maxLength={100}
          />
          <Text style={styles.charCount}>{title.length}/100</Text>
        </View>

        {/* Category Selection */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Category</Text>
          <View style={styles.categoryGrid}>
            {categories.map(cat => (
              <TouchableOpacity
                key={cat.id}
                style={[
                  styles.categoryButton,
                  category === cat.id && styles.categoryButtonActive,
                ]}
                onPress={() => setCategory(cat.id)}>
                <Icon
                  name={cat.icon}
                  size={20}
                  color={category === cat.id ? colors.text : colors.textSecondary}
                />
                <Text
                  style={[
                    styles.categoryText,
                    category === cat.id && styles.categoryTextActive,
                  ]}>
                  {cat.name}
                </Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>

        {/* Description Input */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Description</Text>
          <TextInput
            style={[styles.textInput, styles.descriptionInput]}
            value={description}
            onChangeText={setDescription}
            placeholder="Provide a detailed description of your proposal, including rationale, implementation details, and expected outcomes."
            placeholderTextColor={colors.textMuted}
            multiline
            textAlignVertical="top"
            maxLength={2000}
          />
          <Text style={styles.charCount}>{description.length}/2000</Text>
        </View>

        {/* Discussion Link */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Discussion Link (Optional)</Text>
          <TextInput
            style={styles.textInput}
            value={discussionLink}
            onChangeText={setDiscussionLink}
            placeholder="https://forum.etrid.network/..."
            placeholderTextColor={colors.textMuted}
            keyboardType="url"
            autoCapitalize="none"
          />
        </View>

        {/* Requirements Checklist */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Requirements</Text>
          {requirements.map((req, index) => (
            <View key={index} style={styles.requirementRow}>
              <Icon
                name={req.met ? 'check-circle' : 'circle'}
                size={20}
                color={req.met ? colors.success : colors.textSecondary}
              />
              <Text
                style={[
                  styles.requirementText,
                  req.met && styles.requirementMet,
                ]}>
                {req.text}
              </Text>
            </View>
          ))}
        </View>

        {/* Deposit Info */}
        <View style={styles.depositInfo}>
          <Icon name="info" size={16} color={colors.info} />
          <Text style={styles.depositText}>
            A refundable deposit of 100 ETR is required. It will be returned if the proposal
            passes or reaches quorum.
          </Text>
        </View>

        {/* Submit Button */}
        <TouchableOpacity
          style={[
            styles.submitButton,
            !allRequirementsMet && styles.submitButtonDisabled,
          ]}
          onPress={handleSubmit}
          disabled={!allRequirementsMet}>
          <Text style={styles.submitButtonText}>Submit Proposal</Text>
        </TouchableOpacity>

        {/* Preview Button */}
        <TouchableOpacity
          style={styles.previewButton}
          onPress={() => Alert.alert('Preview', 'Proposal preview coming soon')}>
          <Icon name="eye" size={20} color={colors.primary} />
          <Text style={styles.previewButtonText}>Preview Proposal</Text>
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
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  headerTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: '700',
    color: colors.text,
    marginTop: theme.spacing.sm,
  },
  headerSubtitle: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
    marginTop: 4,
    textAlign: 'center',
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
  textInput: {
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    padding: theme.spacing.md,
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  descriptionInput: {
    height: 150,
  },
  charCount: {
    fontSize: theme.fontSize.xs,
    color: colors.textMuted,
    textAlign: 'right',
    marginTop: 4,
  },
  categoryGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: theme.spacing.sm,
  },
  categoryButton: {
    flex: 1,
    minWidth: '45%',
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    gap: 8,
  },
  categoryButtonActive: {
    backgroundColor: colors.primary,
  },
  categoryText: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    fontWeight: '600',
  },
  categoryTextActive: {
    color: colors.text,
  },
  requirementRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: theme.spacing.sm,
    paddingVertical: theme.spacing.xs,
  },
  requirementText: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  requirementMet: {
    color: colors.text,
  },
  depositInfo: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    backgroundColor: `${colors.info}20`,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  depositText: {
    flex: 1,
    fontSize: theme.fontSize.sm,
    color: colors.info,
    lineHeight: 20,
  },
  submitButton: {
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  submitButtonDisabled: {
    backgroundColor: colors.border,
  },
  submitButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: colors.text,
  },
  previewButton: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: theme.spacing.md,
    gap: 8,
    marginBottom: theme.spacing.xl,
  },
  previewButtonText: {
    fontSize: theme.fontSize.md,
    color: colors.primary,
    fontWeight: '600',
  },
});
