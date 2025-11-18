import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  Alert,
  Clipboard,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation, useRoute } from '@react-navigation/native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { BlurView } from '@react-native-community/blur';

export const BackupPhraseScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const route = useRoute<any>();
  const { mnemonic, address } = route.params;

  const [isRevealed, setIsRevealed] = useState(false);
  const [copied, setCopied] = useState(false);

  const words = mnemonic.split(' ');

  const handleCopy = () => {
    Clipboard.setString(mnemonic);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleContinue = () => {
    navigation.navigate('VerifyPhrase', { mnemonic, address });
  };

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        {/* Header */}
        <View style={styles.header}>
          <Text style={styles.title}>Backup Recovery Phrase</Text>
        </View>

        {/* Warning */}
        <View style={styles.warningBox}>
          <Text style={styles.warningIcon}>⚠️</Text>
          <View style={styles.warningContent}>
            <Text style={styles.warningTitle}>Keep it secret, keep it safe</Text>
            <Text style={styles.warningText}>
              Anyone with this phrase can access your funds. Never share it.
            </Text>
          </View>
        </View>

        {/* Recovery Phrase */}
        <View style={styles.phraseContainer}>
          <View style={styles.phraseHeader}>
            <Text style={styles.phraseTitle}>Your Recovery Phrase</Text>
            <TouchableOpacity
              onPress={() => setIsRevealed(!isRevealed)}
              style={styles.revealButton}
              accessibilityLabel={isRevealed ? 'Hide phrase' : 'Reveal phrase'}
              accessibilityRole="button"
            >
              <Text style={styles.revealButtonText}>{isRevealed ? '👁️' : '👁️‍🗨️'}</Text>
            </TouchableOpacity>
          </View>

          <View style={styles.wordsGrid}>
            {words.map((word, index) => (
              <View key={index} style={styles.wordContainer}>
                {!isRevealed && (
                  <BlurView
                    style={styles.wordBlur}
                    blurType="light"
                    blurAmount={10}
                  />
                )}
                <Text style={styles.wordNumber}>{index + 1}</Text>
                <Text style={styles.word}>{word}</Text>
              </View>
            ))}
          </View>

          {isRevealed && (
            <TouchableOpacity
              onPress={handleCopy}
              style={styles.copyButton}
              accessibilityLabel="Copy recovery phrase"
              accessibilityRole="button"
            >
              <Text style={styles.copyButtonText}>
                {copied ? '✓ Copied!' : '📋 Copy to Clipboard'}
              </Text>
            </TouchableOpacity>
          )}
        </View>

        {/* Instructions */}
        <View style={styles.instructions}>
          <Text style={styles.instructionsTitle}>How to backup:</Text>
          <Instruction number="1" text="Write down all 12 words in order" />
          <Instruction number="2" text="Store in a secure location" />
          <Instruction number="3" text="Never take a screenshot" />
          <Instruction number="4" text="Verify your backup on the next screen" />
        </View>

        {/* Continue Button */}
        <TouchableOpacity
          style={[styles.continueButton, !isRevealed && styles.continueButtonDisabled]}
          onPress={handleContinue}
          disabled={!isRevealed}
          accessibilityLabel="Continue to verification"
          accessibilityRole="button"
        >
          <Text style={styles.continueButtonText}>I've Written It Down</Text>
        </TouchableOpacity>
      </ScrollView>
    </SafeAreaView>
  );
};

interface InstructionProps {
  number: string;
  text: string;
}

const Instruction: React.FC<InstructionProps> = ({ number, text }) => (
  <View style={styles.instruction}>
    <View style={styles.instructionNumber}>
      <Text style={styles.instructionNumberText}>{number}</Text>
    </View>
    <Text style={styles.instructionText}>{text}</Text>
  </View>
);

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  scrollContent: {
    flexGrow: 1,
    paddingBottom: spacing.xl,
  },
  header: {
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.lg,
  },
  title: {
    ...typography.h2,
    color: colors.text,
  },
  warningBox: {
    flexDirection: 'row',
    backgroundColor: '#FFEBEE',
    borderRadius: borderRadius.md,
    padding: spacing.md,
    marginHorizontal: spacing.lg,
    marginBottom: spacing.lg,
    borderLeftWidth: 4,
    borderLeftColor: colors.error,
  },
  warningIcon: {
    fontSize: 24,
    marginRight: spacing.sm,
  },
  warningContent: {
    flex: 1,
  },
  warningTitle: {
    ...typography.h3,
    color: '#C62828',
    marginBottom: spacing.xs / 2,
  },
  warningText: {
    ...typography.bodySmall,
    color: '#C62828',
    lineHeight: 20,
  },
  phraseContainer: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.lg,
    marginHorizontal: spacing.lg,
    marginBottom: spacing.lg,
  },
  phraseHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  phraseTitle: {
    ...typography.h3,
    color: colors.text,
  },
  revealButton: {
    padding: spacing.sm,
  },
  revealButtonText: {
    fontSize: 24,
  },
  wordsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    marginHorizontal: -spacing.xs,
  },
  wordContainer: {
    width: '33.33%',
    padding: spacing.xs,
  },
  wordBlur: {
    position: 'absolute',
    top: spacing.xs,
    left: spacing.xs,
    right: spacing.xs,
    bottom: spacing.xs,
    borderRadius: borderRadius.md,
    zIndex: 1,
  },
  wordNumber: {
    ...typography.caption,
    color: colors.textSecondary,
    marginBottom: spacing.xs / 2,
  },
  word: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
    backgroundColor: colors.background,
    paddingVertical: spacing.sm,
    paddingHorizontal: spacing.sm,
    borderRadius: borderRadius.md,
    textAlign: 'center',
  },
  copyButton: {
    backgroundColor: colors.primary,
    paddingVertical: spacing.sm,
    borderRadius: borderRadius.md,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  copyButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
  instructions: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.lg,
  },
  instructionsTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  instruction: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  instructionNumber: {
    width: 28,
    height: 28,
    borderRadius: 14,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: spacing.sm,
  },
  instructionNumberText: {
    ...typography.bodySmall,
    color: colors.background,
    fontWeight: 'bold',
  },
  instructionText: {
    ...typography.body,
    color: colors.text,
    flex: 1,
  },
  continueButton: {
    backgroundColor: colors.primary,
    paddingVertical: spacing.md,
    borderRadius: borderRadius.lg,
    alignItems: 'center',
    marginHorizontal: spacing.lg,
    shadowColor: colors.primary,
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 8,
    elevation: 8,
  },
  continueButtonDisabled: {
    backgroundColor: colors.gray300,
    shadowOpacity: 0,
  },
  continueButtonText: {
    ...typography.h3,
    color: colors.background,
  },
});
