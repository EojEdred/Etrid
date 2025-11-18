import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, TextInput, ActivityIndicator, ScrollView, Alert } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import KeychainService from '../../services/KeychainService';
import { isValidMnemonic } from '../../utils/validators';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const ImportWalletScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { login, importWallet } = useAuth();
  const [mnemonic, setMnemonic] = useState('');
  const [isImporting, setIsImporting] = useState(false);

  const handleImport = async () => {
    try {
      const trimmedMnemonic = mnemonic.trim();

      if (!isValidMnemonic(trimmedMnemonic)) {
        Alert.alert('Invalid Phrase', 'Please enter a valid 12-word recovery phrase');
        return;
      }

      setIsImporting(true);
      const address = await importWallet(trimmedMnemonic);
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('Failed to load keypair');
      }

      await login(keypair, address);
      navigation.navigate('BiometricSetup');
    } catch (error) {
      console.error('Error importing wallet:', error);
      Alert.alert('Error', 'Failed to import wallet. Please check your recovery phrase.');
    } finally {
      setIsImporting(false);
    }
  };

  const wordCount = mnemonic.trim().split(/\s+/).filter(w => w.length > 0).length;
  const isValid = wordCount === 12 && isValidMnemonic(mnemonic.trim());

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        <View style={styles.header}>
          <TouchableOpacity onPress={() => navigation.goBack()} style={styles.backButton}>
            <Text style={styles.backButtonText}>←</Text>
          </TouchableOpacity>
          <Text style={styles.title}>Import Wallet</Text>
        </View>

        <View style={styles.content}>
          <Text style={styles.description}>
            Enter your 12-word recovery phrase to restore your wallet
          </Text>

          <TextInput
            style={styles.input}
            placeholder="Enter recovery phrase..."
            placeholderTextColor={colors.gray300}
            value={mnemonic}
            onChangeText={setMnemonic}
            multiline
            numberOfLines={4}
            autoCorrect={false}
            autoCapitalize="none"
            accessibilityLabel="Recovery phrase input"
          />

          <View style={styles.wordCounter}>
            <Text style={styles.wordCounterText}>
              {wordCount} / 12 words
            </Text>
          </View>

          <View style={styles.tipBox}>
            <Text style={styles.tipIcon}>💡</Text>
            <Text style={styles.tipText}>
              Separate words with spaces. The phrase should be exactly 12 words.
            </Text>
          </View>
        </View>

        <TouchableOpacity
          style={[styles.importButton, (!isValid || isImporting) && styles.importButtonDisabled]}
          onPress={handleImport}
          disabled={!isValid || isImporting}
        >
          {isImporting ? <ActivityIndicator color={colors.background} /> : <Text style={styles.importButtonText}>Import Wallet</Text>}
        </TouchableOpacity>
      </ScrollView>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  scrollContent: { flexGrow: 1, paddingBottom: spacing.xl },
  header: { flexDirection: 'row', alignItems: 'center', paddingHorizontal: spacing.lg, paddingVertical: spacing.md },
  backButton: { marginRight: spacing.md, padding: spacing.sm },
  backButtonText: { fontSize: 28, color: colors.text },
  title: { ...typography.h2, color: colors.text },
  content: { flex: 1, paddingHorizontal: spacing.lg, paddingTop: spacing.xl },
  description: { ...typography.body, color: colors.textSecondary, marginBottom: spacing.lg },
  input: { ...typography.body, backgroundColor: colors.surface, borderRadius: borderRadius.md, padding: spacing.md, minHeight: 120, textAlignVertical: 'top', borderWidth: 2, borderColor: colors.gray300, color: colors.text },
  wordCounter: { alignItems: 'flex-end', marginTop: spacing.sm, marginBottom: spacing.md },
  wordCounterText: { ...typography.bodySmall, color: colors.textSecondary },
  tipBox: { flexDirection: 'row', backgroundColor: colors.info + '20', borderRadius: borderRadius.md, padding: spacing.md },
  tipIcon: { fontSize: 20, marginRight: spacing.sm },
  tipText: { ...typography.bodySmall, color: colors.text, flex: 1 },
  importButton: { backgroundColor: colors.primary, paddingVertical: spacing.md, borderRadius: borderRadius.lg, alignItems: 'center', marginHorizontal: spacing.lg },
  importButtonDisabled: { backgroundColor: colors.gray300 },
  importButtonText: { ...typography.h3, color: colors.background },
});
