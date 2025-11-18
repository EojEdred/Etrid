import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ActivityIndicator,
  ScrollView,
  Alert,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const CreateWalletScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { createWallet } = useAuth();
  const [isCreating, setIsCreating] = useState(false);

  const handleCreateWallet = async () => {
    try {
      setIsCreating(true);

      // Create new wallet
      const { address, mnemonic } = await createWallet();

      // Navigate to backup phrase screen
      navigation.navigate('BackupPhrase', { mnemonic, address });
    } catch (error) {
      console.error('Error creating wallet:', error);
      Alert.alert(
        'Error',
        'Failed to create wallet. Please try again.',
        [{ text: 'OK' }]
      );
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <ScrollView contentContainerStyle={styles.scrollContent}>
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity
            onPress={() => navigation.goBack()}
            style={styles.backButton}
            accessibilityLabel="Go back"
            accessibilityRole="button"
          >
            <Text style={styles.backButtonText}>←</Text>
          </TouchableOpacity>
          <Text style={styles.title}>Create Wallet</Text>
        </View>

        {/* Content */}
        <View style={styles.content}>
          {/* Icon */}
          <View style={styles.iconContainer}>
            <Text style={styles.icon}>🔐</Text>
          </View>

          {/* Info */}
          <Text style={styles.heading}>Secure Your Wallet</Text>
          <Text style={styles.description}>
            Your wallet will be protected with a 12-word recovery phrase. This phrase is the only
            way to restore your wallet if you lose access to your device.
          </Text>

          {/* Security Tips */}
          <View style={styles.tipsContainer}>
            <Text style={styles.tipsTitle}>Important:</Text>
            <Tip icon="✅" text="Write down your recovery phrase" />
            <Tip icon="🔒" text="Keep it in a safe place" />
            <Tip icon="🚫" text="Never share it with anyone" />
            <Tip icon="⚠️" text="Ëtrid will never ask for your phrase" />
          </View>

          {/* Warning */}
          <View style={styles.warningBox}>
            <Text style={styles.warningIcon}>⚠️</Text>
            <Text style={styles.warningText}>
              If you lose your recovery phrase, you will lose access to your funds permanently.
              There is no way to recover it.
            </Text>
          </View>
        </View>

        {/* Button */}
        <View style={styles.buttonContainer}>
          <TouchableOpacity
            style={[styles.createButton, isCreating && styles.createButtonDisabled]}
            onPress={handleCreateWallet}
            disabled={isCreating}
            accessibilityLabel="Create new wallet"
            accessibilityRole="button"
          >
            {isCreating ? (
              <ActivityIndicator color={colors.background} />
            ) : (
              <Text style={styles.createButtonText}>Create New Wallet</Text>
            )}
          </TouchableOpacity>

          <TouchableOpacity
            style={styles.importButton}
            onPress={() => navigation.navigate('ImportWallet')}
            disabled={isCreating}
            accessibilityLabel="Import existing wallet"
            accessibilityRole="button"
          >
            <Text style={styles.importButtonText}>Import Existing Wallet</Text>
          </TouchableOpacity>
        </View>
      </ScrollView>
    </SafeAreaView>
  );
};

interface TipProps {
  icon: string;
  text: string;
}

const Tip: React.FC<TipProps> = ({ icon, text }) => (
  <View style={styles.tip}>
    <Text style={styles.tipIcon}>{icon}</Text>
    <Text style={styles.tipText}>{text}</Text>
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
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
  },
  backButton: {
    marginRight: spacing.md,
    padding: spacing.sm,
  },
  backButtonText: {
    fontSize: 28,
    color: colors.text,
  },
  title: {
    ...typography.h2,
    color: colors.text,
  },
  content: {
    flex: 1,
    paddingHorizontal: spacing.lg,
    paddingTop: spacing.xl,
  },
  iconContainer: {
    alignItems: 'center',
    marginBottom: spacing.lg,
  },
  icon: {
    fontSize: 80,
  },
  heading: {
    ...typography.h2,
    color: colors.text,
    textAlign: 'center',
    marginBottom: spacing.md,
  },
  description: {
    ...typography.body,
    color: colors.textSecondary,
    textAlign: 'center',
    marginBottom: spacing.xl,
    lineHeight: 24,
  },
  tipsContainer: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.lg,
    marginBottom: spacing.lg,
  },
  tipsTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  tip: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  tipIcon: {
    fontSize: 20,
    marginRight: spacing.sm,
  },
  tipText: {
    ...typography.body,
    color: colors.text,
    flex: 1,
  },
  warningBox: {
    flexDirection: 'row',
    backgroundColor: '#FFF4E5',
    borderRadius: borderRadius.md,
    padding: spacing.md,
    borderLeftWidth: 4,
    borderLeftColor: colors.warning,
  },
  warningIcon: {
    fontSize: 24,
    marginRight: spacing.sm,
  },
  warningText: {
    ...typography.bodySmall,
    color: '#8B6914',
    flex: 1,
    lineHeight: 20,
  },
  buttonContainer: {
    paddingHorizontal: spacing.lg,
    paddingTop: spacing.xl,
  },
  createButton: {
    backgroundColor: colors.primary,
    paddingVertical: spacing.md,
    borderRadius: borderRadius.lg,
    alignItems: 'center',
    marginBottom: spacing.md,
    shadowColor: colors.primary,
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 8,
    elevation: 8,
  },
  createButtonDisabled: {
    opacity: 0.6,
  },
  createButtonText: {
    ...typography.h3,
    color: colors.background,
  },
  importButton: {
    paddingVertical: spacing.md,
    borderRadius: borderRadius.lg,
    alignItems: 'center',
    borderWidth: 2,
    borderColor: colors.gray300,
  },
  importButtonText: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
});
