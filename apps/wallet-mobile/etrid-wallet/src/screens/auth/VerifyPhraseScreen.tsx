import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  Alert,
  ActivityIndicator,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation, useRoute } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import KeychainService from '../../services/KeychainService';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const VerifyPhraseScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const route = useRoute<any>();
  const { mnemonic, address } = route.params;
  const { login } = useAuth();

  const words = mnemonic.split(' ');
  const [testIndices, setTestIndices] = useState<number[]>([]);
  const [selectedWords, setSelectedWords] = useState<string[]>(['', '', '']);
  const [shuffledWords, setShuffledWords] = useState<string[]>([]);
  const [isVerifying, setIsVerifying] = useState(false);

  useEffect(() => {
    // Select 3 random indices to test
    const indices = [2, 6, 10]; // Test words #3, #7, #11
    setTestIndices(indices);

    // Create shuffled array with correct words and decoys
    const testWords = indices.map(i => words[i]);
    const decoys = words.filter((_, i) => !indices.includes(i)).slice(0, 6);
    const allWords = [...testWords, ...decoys].sort(() => Math.random() - 0.5);
    setShuffledWords(allWords);
  }, []);

  const handleWordSelect = (word: string, index: number) => {
    const newSelected = [...selectedWords];

    // Find if word is already selected
    const existingIndex = newSelected.indexOf(word);

    if (existingIndex !== -1) {
      // Deselect
      newSelected[existingIndex] = '';
    } else {
      // Select in first empty slot
      const emptyIndex = newSelected.findIndex(w => w === '');
      if (emptyIndex !== -1) {
        newSelected[emptyIndex] = word;
      }
    }

    setSelectedWords(newSelected);
  };

  const handleVerify = async () => {
    try {
      setIsVerifying(true);

      // Check if all words are selected
      if (selectedWords.some(w => w === '')) {
        Alert.alert('Incomplete', 'Please select all words');
        setIsVerifying(false);
        return;
      }

      // Verify selected words match
      const isCorrect = testIndices.every((index, i) => {
        return selectedWords[i] === words[index];
      });

      if (!isCorrect) {
        Alert.alert(
          'Incorrect',
          'The selected words do not match your recovery phrase. Please try again.',
          [
            {
              text: 'Try Again',
              onPress: () => {
                setSelectedWords(['', '', '']);
              },
            },
          ]
        );
        setIsVerifying(false);
        return;
      }

      // Verification successful - load keypair and login
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('Failed to load keypair');
      }

      await login(keypair, address);

      // Navigate to biometric setup
      navigation.navigate('BiometricSetup');
    } catch (error) {
      console.error('Error verifying phrase:', error);
      Alert.alert('Error', 'Failed to verify phrase. Please try again.');
    } finally {
      setIsVerifying(false);
    }
  };

  const isSelected = (word: string) => selectedWords.includes(word);
  const allSelected = selectedWords.every(w => w !== '');

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        <View style={styles.header}>
          <Text style={styles.title}>Verify Recovery Phrase</Text>
          <Text style={styles.subtitle}>
            Select the correct words to verify your backup
          </Text>
        </View>

        {/* Word Slots */}
        <View style={styles.slotsContainer}>
          {testIndices.map((wordIndex, i) => (
            <View key={i} style={styles.slot}>
              <Text style={styles.slotLabel}>Word #{wordIndex + 1}</Text>
              <View style={styles.slotBox}>
                <Text style={styles.slotWord}>
                  {selectedWords[i] || '...'}
                </Text>
              </View>
            </View>
          ))}
        </View>

        {/* Word Options */}
        <View style={styles.optionsContainer}>
          <Text style={styles.optionsTitle}>Select words:</Text>
          <View style={styles.optionsGrid}>
            {shuffledWords.map((word, index) => (
              <TouchableOpacity
                key={index}
                style={[
                  styles.optionButton,
                  isSelected(word) && styles.optionButtonSelected,
                ]}
                onPress={() => handleWordSelect(word, index)}
                accessibilityLabel={`Select word ${word}`}
                accessibilityRole="button"
              >
                <Text
                  style={[
                    styles.optionText,
                    isSelected(word) && styles.optionTextSelected,
                  ]}
                >
                  {word}
                </Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>

        {/* Verify Button */}
        <TouchableOpacity
          style={[
            styles.verifyButton,
            (!allSelected || isVerifying) && styles.verifyButtonDisabled,
          ]}
          onPress={handleVerify}
          disabled={!allSelected || isVerifying}
          accessibilityLabel="Verify recovery phrase"
          accessibilityRole="button"
        >
          {isVerifying ? (
            <ActivityIndicator color={colors.background} />
          ) : (
            <Text style={styles.verifyButtonText}>Verify & Continue</Text>
          )}
        </TouchableOpacity>
      </ScrollView>
    </SafeAreaView>
  );
};

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
    marginBottom: spacing.xs,
  },
  subtitle: {
    ...typography.body,
    color: colors.textSecondary,
  },
  slotsContainer: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.xl,
  },
  slot: {
    marginBottom: spacing.md,
  },
  slotLabel: {
    ...typography.bodySmall,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  slotBox: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    borderWidth: 2,
    borderColor: colors.primary,
    paddingVertical: spacing.md,
    paddingHorizontal: spacing.lg,
    minHeight: 56,
    justifyContent: 'center',
  },
  slotWord: {
    ...typography.h3,
    color: colors.text,
    textAlign: 'center',
  },
  optionsContainer: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.xl,
  },
  optionsTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  optionsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    marginHorizontal: -spacing.xs,
  },
  optionButton: {
    width: '33.33%',
    padding: spacing.xs,
  },
  optionButtonSelected: {
    opacity: 0.5,
  },
  optionText: {
    ...typography.body,
    color: colors.text,
    backgroundColor: colors.surface,
    paddingVertical: spacing.sm,
    paddingHorizontal: spacing.sm,
    borderRadius: borderRadius.md,
    textAlign: 'center',
    borderWidth: 2,
    borderColor: colors.gray300,
  },
  optionTextSelected: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + '20',
  },
  verifyButton: {
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
  verifyButtonDisabled: {
    backgroundColor: colors.gray300,
    shadowOpacity: 0,
  },
  verifyButtonText: {
    ...typography.h3,
    color: colors.background,
  },
});
