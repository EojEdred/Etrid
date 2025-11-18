import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, ActivityIndicator } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import BiometricService from '../../services/BiometricService';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const BiometricSetupScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { enableBiometric } = useAuth();
  const [biometricType, setBiometricType] = useState('Biometric');
  const [canUseBiometric, setCanUseBiometric] = useState(false);
  const [isEnabling, setIsEnabling] = useState(false);

  useEffect(() => {
    checkBiometric();
  }, []);

  const checkBiometric = async () => {
    const result = await BiometricService.canUseBiometric();
    setCanUseBiometric(result.canUse);
    if (result.canUse) {
      const type = await BiometricService.getBiometricTypeName();
      setBiometricType(type);
    }
  };

  const handleEnable = async () => {
    setIsEnabling(true);
    const success = await enableBiometric();
    setIsEnabling(false);
    if (success) {
      navigation.reset({ index: 0, routes: [{ name: 'MainTabs' }] });
    }
  };

  const handleSkip = () => {
    navigation.reset({ index: 0, routes: [{ name: 'MainTabs' }] });
  };

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <View style={styles.content}>
        <View style={styles.iconContainer}>
          <Text style={styles.icon}>{biometricType === 'Face ID' ? '🔓' : '👆'}</Text>
        </View>

        <Text style={styles.title}>Enable {biometricType}</Text>
        <Text style={styles.description}>
          {canUseBiometric
            ? `Use ${biometricType} for quick and secure access to your wallet`
            : `${biometricType} is not available on this device`}
        </Text>

        {canUseBiometric && (
          <View style={styles.features}>
            <Feature icon="⚡" text="Instant authentication" />
            <Feature icon="🔒" text="Extra security layer" />
            <Feature icon="✨" text="Convenient transactions" />
          </View>
        )}
      </View>

      <View style={styles.buttonContainer}>
        {canUseBiometric ? (
          <>
            <TouchableOpacity style={styles.enableButton} onPress={handleEnable} disabled={isEnabling}>
              {isEnabling ? <ActivityIndicator color={colors.background} /> : <Text style={styles.enableButtonText}>Enable {biometricType}</Text>}
            </TouchableOpacity>
            <TouchableOpacity style={styles.skipButton} onPress={handleSkip}>
              <Text style={styles.skipButtonText}>Skip for Now</Text>
            </TouchableOpacity>
          </>
        ) : (
          <TouchableOpacity style={styles.enableButton} onPress={handleSkip}>
            <Text style={styles.enableButtonText}>Continue</Text>
          </TouchableOpacity>
        )}
      </View>
    </SafeAreaView>
  );
};

const Feature: React.FC<{ icon: string; text: string }> = ({ icon, text }) => (
  <View style={styles.feature}>
    <Text style={styles.featureIcon}>{icon}</Text>
    <Text style={styles.featureText}>{text}</Text>
  </View>
);

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  content: { flex: 1, paddingHorizontal: spacing.lg, paddingTop: spacing.xxl, alignItems: 'center' },
  iconContainer: { marginBottom: spacing.xl },
  icon: { fontSize: 100 },
  title: { ...typography.h1, color: colors.text, textAlign: 'center', marginBottom: spacing.md },
  description: { ...typography.body, color: colors.textSecondary, textAlign: 'center', marginBottom: spacing.xl },
  features: { width: '100%', backgroundColor: colors.surface, borderRadius: borderRadius.lg, padding: spacing.lg },
  feature: { flexDirection: 'row', alignItems: 'center', marginBottom: spacing.sm },
  featureIcon: { fontSize: 24, marginRight: spacing.md },
  featureText: { ...typography.body, color: colors.text, flex: 1 },
  buttonContainer: { paddingHorizontal: spacing.lg, paddingBottom: spacing.xl },
  enableButton: { backgroundColor: colors.primary, paddingVertical: spacing.md, borderRadius: borderRadius.lg, alignItems: 'center', marginBottom: spacing.md },
  enableButtonText: { ...typography.h3, color: colors.background },
  skipButton: { paddingVertical: spacing.md, alignItems: 'center' },
  skipButtonText: { ...typography.body, color: colors.textSecondary, fontWeight: '600' },
});
