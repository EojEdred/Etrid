import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { ConvictionLevel, CONVICTION_LEVELS } from '../../types/defi.types';

interface ConvictionSelectorProps {
  selected: ConvictionLevel;
  onSelect: (level: ConvictionLevel) => void;
}

export function ConvictionSelector({ selected, onSelect }: ConvictionSelectorProps) {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>Conviction Level</Text>
      <Text style={styles.subtitle}>
        Higher conviction = more voting power, but longer token lock
      </Text>

      {CONVICTION_LEVELS.map(conviction => (
        <TouchableOpacity
          key={conviction.level}
          style={[
            styles.option,
            selected === conviction.level && styles.selectedOption,
            conviction.recommended && styles.recommendedOption,
          ]}
          onPress={() => onSelect(conviction.level)}
          activeOpacity={0.7}
        >
          <View style={styles.optionHeader}>
            <View style={styles.optionLeft}>
              <Text style={[
                styles.optionTitle,
                selected === conviction.level && styles.selectedText,
              ]}>
                {conviction.multiplier}x Voting Power
              </Text>
              <Text style={[
                styles.optionSubtitle,
                selected === conviction.level && styles.selectedSubtext,
              ]}>
                {conviction.description}
              </Text>
            </View>
            {conviction.recommended && (
              <View style={styles.recommendedBadge}>
                <Text style={styles.recommendedText}>Recommended</Text>
              </View>
            )}
          </View>

          <View style={[
            styles.radio,
            selected === conviction.level && styles.radioSelected,
          ]}>
            {selected === conviction.level && (
              <View style={styles.radioInner} />
            )}
          </View>
        </TouchableOpacity>
      ))}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    marginVertical: spacing.md,
  },
  title: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  subtitle: {
    ...typography.bodySmall,
    color: colors.textSecondary,
    marginBottom: spacing.md,
  },
  option: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.sm,
    borderWidth: 2,
    borderColor: 'transparent',
  },
  selectedOption: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + '10',
  },
  recommendedOption: {
    borderColor: colors.success + '40',
  },
  optionHeader: {
    flex: 1,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginRight: spacing.md,
  },
  optionLeft: {
    flex: 1,
  },
  optionTitle: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
    marginBottom: 4,
  },
  optionSubtitle: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  selectedText: {
    color: colors.primary,
  },
  selectedSubtext: {
    color: colors.primary + 'CC',
  },
  recommendedBadge: {
    backgroundColor: colors.success,
    paddingHorizontal: spacing.sm,
    paddingVertical: 4,
    borderRadius: borderRadius.sm,
  },
  recommendedText: {
    ...typography.caption,
    color: colors.background,
    fontWeight: '600',
  },
  radio: {
    width: 24,
    height: 24,
    borderRadius: 12,
    borderWidth: 2,
    borderColor: colors.gray300,
    justifyContent: 'center',
    alignItems: 'center',
  },
  radioSelected: {
    borderColor: colors.primary,
  },
  radioInner: {
    width: 12,
    height: 12,
    borderRadius: 6,
    backgroundColor: colors.primary,
  },
});
