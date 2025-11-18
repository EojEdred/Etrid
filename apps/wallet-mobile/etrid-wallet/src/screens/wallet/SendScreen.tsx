import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, TextInput, ScrollView, Alert, ActivityIndicator } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import { useBalance } from '../../hooks/useBalance';
import { AmountInput } from '../../components/AmountInput';
import { isValidAddress, isValidAmount, hasSufficientBalance } from '../../utils/validators';
import { TRANSACTION_SPEEDS } from '../../utils/constants';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import EtridSDKService from '../../services/EtridSDKService';
import BiometricService from '../../services/BiometricService';

export const SendScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { keypair } = useAuth();
  const { balance } = useBalance('ETR');

  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [amount, setAmount] = useState('');
  const [recipient, setRecipient] = useState('');
  const [speed, setSpeed] = useState<'INSTANT' | 'FAST' | 'STANDARD'>('STANDARD');
  const [isSending, setIsSending] = useState(false);

  const sdk = EtridSDKService.getInstance();

  const handleContinueStep1 = () => {
    const validation = isValidAmount(amount, { min: '0.000001', max: balance?.balance || '0' });
    if (!validation.valid) {
      Alert.alert('Invalid Amount', validation.error);
      return;
    }
    const sufficient = hasSufficientBalance(balance?.balance || '0', amount, '0.001');
    if (!sufficient.sufficient) {
      Alert.alert('Insufficient Balance', sufficient.error);
      return;
    }
    setStep(2);
  };

  const handleContinueStep2 = () => {
    if (!isValidAddress(recipient)) {
      Alert.alert('Invalid Address', 'Please enter a valid Ëtrid address');
      return;
    }
    setStep(3);
  };

  const handleSend = async () => {
    try {
      setIsSending(true);

      // Biometric authentication
      const bioAuth = await BiometricService.authenticateForTransaction(amount, recipient);
      if (!bioAuth) {
        setIsSending(false);
        return;
      }

      if (!keypair) {
        throw new Error('No keypair found');
      }

      // Send transaction
      const hash = await sdk.accounts.transfer(recipient, amount, keypair);

      Alert.alert('Success', `Transaction sent!\n\nHash: ${hash.slice(0, 16)}...`, [
        { text: 'OK', onPress: () => navigation.goBack() }
      ]);
    } catch (error) {
      console.error('Error sending transaction:', error);
      Alert.alert('Error', 'Failed to send transaction. Please try again.');
    } finally {
      setIsSending(false);
    }
  };

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => step > 1 ? setStep((step - 1) as any) : navigation.goBack()}>
          <Text style={styles.backButton}>←</Text>
        </TouchableOpacity>
        <Text style={styles.title}>Send ETR</Text>
        <View style={{ width: 40 }} />
      </View>

      <View style={styles.progress}>
        {[1, 2, 3].map(i => (
          <View key={i} style={[styles.progressDot, i <= step && styles.progressDotActive]} />
        ))}
      </View>

      <ScrollView contentContainerStyle={styles.content}>
        {step === 1 && (
          <View>
            <Text style={styles.stepTitle}>How much?</Text>
            <AmountInput
              value={amount}
              onChangeValue={setAmount}
              currency="ETR"
              usdPrice={2.45}
              maxBalance={balance?.balanceFormatted.replace(' ETR', '')}
            />
            <View style={styles.balanceInfo}>
              <Text style={styles.balanceLabel}>Available:</Text>
              <Text style={styles.balanceValue}>{balance?.balanceFormatted || '0 ETR'}</Text>
            </View>
            <TouchableOpacity style={styles.continueButton} onPress={handleContinueStep1}>
              <Text style={styles.continueButtonText}>Continue</Text>
            </TouchableOpacity>
          </View>
        )}

        {step === 2 && (
          <View>
            <Text style={styles.stepTitle}>Who to?</Text>
            <TextInput
              style={styles.addressInput}
              value={recipient}
              onChangeText={setRecipient}
              placeholder="Enter Ëtrid address"
              placeholderTextColor={colors.gray300}
              autoCorrect={false}
              autoCapitalize="none"
            />
            <TouchableOpacity style={styles.continueButton} onPress={handleContinueStep2}>
              <Text style={styles.continueButtonText}>Continue</Text>
            </TouchableOpacity>
          </View>
        )}

        {step === 3 && (
          <View>
            <Text style={styles.stepTitle}>Review & Send</Text>
            <View style={styles.reviewCard}>
              <ReviewItem label="Amount" value={`${amount} ETR`} />
              <ReviewItem label="To" value={`${recipient.slice(0, 12)}...${recipient.slice(-8)}`} />
              <ReviewItem label="Speed" value={TRANSACTION_SPEEDS[speed].label} />
              <ReviewItem label="Fee" value="0.001 ETR" />
            </View>
            <TouchableOpacity
              style={[styles.sendButton, isSending && styles.sendButtonDisabled]}
              onPress={handleSend}
              disabled={isSending}
            >
              {isSending ? <ActivityIndicator color={colors.background} /> : <Text style={styles.sendButtonText}>Send Now</Text>}
            </TouchableOpacity>
          </View>
        )}
      </ScrollView>
    </SafeAreaView>
  );
};

const ReviewItem: React.FC<{ label: string; value: string }> = ({ label, value }) => (
  <View style={styles.reviewItem}>
    <Text style={styles.reviewLabel}>{label}</Text>
    <Text style={styles.reviewValue}>{value}</Text>
  </View>
);

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  header: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', paddingHorizontal: spacing.lg, paddingVertical: spacing.md },
  backButton: { fontSize: 28, color: colors.text },
  title: { ...typography.h2, color: colors.text },
  progress: { flexDirection: 'row', justifyContent: 'center', paddingHorizontal: spacing.lg, paddingBottom: spacing.lg },
  progressDot: { width: 8, height: 8, borderRadius: 4, backgroundColor: colors.gray300, marginHorizontal: spacing.xs },
  progressDotActive: { backgroundColor: colors.primary, width: 24 },
  content: { padding: spacing.lg },
  stepTitle: { ...typography.h2, color: colors.text, marginBottom: spacing.lg },
  balanceInfo: { flexDirection: 'row', justifyContent: 'space-between', marginTop: spacing.md, marginBottom: spacing.xl },
  balanceLabel: { ...typography.body, color: colors.textSecondary },
  balanceValue: { ...typography.body, color: colors.text, fontWeight: '600' },
  addressInput: { ...typography.body, backgroundColor: colors.surface, borderRadius: borderRadius.md, padding: spacing.md, borderWidth: 2, borderColor: colors.gray300, color: colors.text },
  continueButton: { backgroundColor: colors.primary, paddingVertical: spacing.md, borderRadius: borderRadius.lg, alignItems: 'center', marginTop: spacing.xl },
  continueButtonText: { ...typography.h3, color: colors.background },
  reviewCard: { backgroundColor: colors.surface, borderRadius: borderRadius.lg, padding: spacing.lg, marginBottom: spacing.xl },
  reviewItem: { flexDirection: 'row', justifyContent: 'space-between', paddingVertical: spacing.sm },
  reviewLabel: { ...typography.body, color: colors.textSecondary },
  reviewValue: { ...typography.body, color: colors.text, fontWeight: '600' },
  sendButton: { backgroundColor: colors.primary, paddingVertical: spacing.md, borderRadius: borderRadius.lg, alignItems: 'center' },
  sendButtonDisabled: { backgroundColor: colors.gray300 },
  sendButtonText: { ...typography.h3, color: colors.background },
});
