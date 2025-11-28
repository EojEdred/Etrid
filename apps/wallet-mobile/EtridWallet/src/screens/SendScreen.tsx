import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  ScrollView,
  Alert,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';
import QRScanner from '@/components/QRScanner';
import {biometricService} from '@/services/BiometricService';

export default function SendScreen({navigation}: any) {
  const [token, setToken] = useState<'ETR' | 'EDSC'>('ETR');
  const [amount, setAmount] = useState('');
  const [address, setAddress] = useState('');
  const [fee, setFee] = useState('standard');
  const [isSending, setIsSending] = useState(false);
  const [showScanner, setShowScanner] = useState(false);

  const availableBalance = token === 'ETR' ? 1234.56 : 2469.13;
  const feeAmount = fee === 'standard' ? 0.001 : 0.002;
  const totalAmount = parseFloat(amount || '0') + feeAmount;
  const isValid =
    address.length > 0 &&
    parseFloat(amount || '0') > 0 &&
    totalAmount <= availableBalance;

  const handleSend = async () => {
    if (!isValid) return;

    // Authenticate with biometrics
    const authenticated = await biometricService.authenticate(
      'Authenticate to send transaction',
    );

    if (!authenticated) {
      Alert.alert('Authentication Failed', 'Please try again');
      return;
    }

    setIsSending(true);
    // Simulate transaction
    setTimeout(() => {
      setIsSending(false);
      Alert.alert('Success', 'Transaction sent successfully!', [
        {
          text: 'OK',
          onPress: () => navigation.goBack(),
        },
      ]);
    }, 2000);
  };

  if (showScanner) {
    return (
      <QRScanner
        onScan={data => {
          setAddress(data);
          setShowScanner(false);
        }}
        onClose={() => setShowScanner(false)}
      />
    );
  }

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Icon name="arrow-left" size={24} color={colors.text} />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Send</Text>
        <View style={styles.tokenSelector}>
          <TouchableOpacity
            style={[styles.tokenButton, token === 'ETR' && styles.tokenButtonActive]}
            onPress={() => setToken('ETR')}>
            <Text
              style={[
                styles.tokenButtonText,
                token === 'ETR' && styles.tokenButtonTextActive,
              ]}>
              ÉTR
            </Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[
              styles.tokenButton,
              token === 'EDSC' && styles.tokenButtonActive,
            ]}
            onPress={() => setToken('EDSC')}>
            <Text
              style={[
                styles.tokenButtonText,
                token === 'EDSC' && styles.tokenButtonTextActive,
              ]}>
              EDSC
            </Text>
          </TouchableOpacity>
        </View>
      </View>

      <ScrollView style={styles.content}>
        {/* Recipient Section */}
        <View style={styles.section}>
          <Text style={styles.label}>To</Text>
          <View style={styles.inputContainer}>
            <TextInput
              style={styles.input}
              placeholder="Enter address or username"
              placeholderTextColor={colors.textSecondary}
              value={address}
              onChangeText={setAddress}
            />
            <TouchableOpacity
              style={styles.inputButton}
              onPress={() => setShowScanner(true)}>
              <Icon name="camera" size={20} color={colors.primary} />
            </TouchableOpacity>
          </View>

          {/* Recent Contacts */}
          <View style={styles.contacts}>
            {[1, 2, 3].map(i => (
              <TouchableOpacity
                key={i}
                style={styles.contact}
                onPress={() => setAddress(`0x${i}a2b...4f5g`)}>
                <View style={styles.contactAvatar}>
                  <Text style={styles.contactAvatarText}>{i}</Text>
                </View>
                <Text style={styles.contactName}>Contact {i}</Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>

        {/* Amount Section */}
        <View style={styles.section}>
          <View style={styles.labelRow}>
            <Text style={styles.label}>Amount</Text>
            <Text style={styles.availableText}>
              Available: {availableBalance.toFixed(2)} {token}
            </Text>
          </View>
          <View style={styles.amountCard}>
            <View style={styles.amountInputContainer}>
              <TextInput
                style={styles.amountInput}
                placeholder="0.00"
                placeholderTextColor={colors.textSecondary}
                value={amount}
                onChangeText={setAmount}
                keyboardType="numeric"
              />
              <Text style={styles.tokenText}>{token}</Text>
            </View>
            <Text style={styles.usdAmount}>
              ≈ $
              {(
                parseFloat(amount || '0') * (token === 'ETR' ? 8 : 1)
              ).toFixed(2)}
            </Text>
            <TouchableOpacity
              style={styles.maxButton}
              onPress={() => setAmount(availableBalance.toString())}>
              <Text style={styles.maxButtonText}>Max</Text>
            </TouchableOpacity>
          </View>
        </View>

        {/* Fee Section */}
        <View style={styles.section}>
          <Text style={styles.label}>Network Fee</Text>
          <View style={styles.feeOptions}>
            <TouchableOpacity
              style={[
                styles.feeOption,
                fee === 'standard' && styles.feeOptionActive,
              ]}
              onPress={() => setFee('standard')}>
              <Text
                style={[
                  styles.feeOptionTitle,
                  fee === 'standard' && styles.feeOptionTitleActive,
                ]}>
                Standard
              </Text>
              <Text
                style={[
                  styles.feeOptionSubtitle,
                  fee === 'standard' && styles.feeOptionSubtitleActive,
                ]}>
                ~5 sec · 0.001 {token}
              </Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[styles.feeOption, fee === 'fast' && styles.feeOptionActive]}
              onPress={() => setFee('fast')}>
              <Text
                style={[
                  styles.feeOptionTitle,
                  fee === 'fast' && styles.feeOptionTitleActive,
                ]}>
                Fast
              </Text>
              <Text
                style={[
                  styles.feeOptionSubtitle,
                  fee === 'fast' && styles.feeOptionSubtitleActive,
                ]}>
                ~3 sec · 0.002 {token}
              </Text>
            </TouchableOpacity>
          </View>
        </View>

        {/* Total Section */}
        <View style={styles.totalCard}>
          <View style={styles.totalRow}>
            <Text style={styles.totalLabel}>Total</Text>
            <View style={styles.totalAmountContainer}>
              <Text style={styles.totalAmount}>
                {totalAmount.toFixed(3)} {token}
              </Text>
              <Text style={styles.totalUsd}>
                ≈ ${(totalAmount * (token === 'ETR' ? 8 : 1)).toFixed(2)}
              </Text>
            </View>
          </View>
        </View>

        {/* Send Button */}
        <TouchableOpacity
          style={[styles.sendButton, !isValid && styles.sendButtonDisabled]}
          onPress={handleSend}
          disabled={!isValid || isSending}>
          {isSending ? (
            <Text style={styles.sendButtonText}>Sending...</Text>
          ) : (
            <>
              <Icon name="send" size={20} color="#000" />
              <Text style={styles.sendButtonText}>Send Transaction</Text>
            </>
          )}
        </TouchableOpacity>
      </ScrollView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: theme.spacing.md,
    paddingTop: 50,
  },
  headerTitle: {
    fontSize: theme.fontSize.xl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  tokenSelector: {
    flexDirection: 'row',
    backgroundColor: colors.surface,
    borderRadius: theme.borderRadius.md,
    padding: 4,
  },
  tokenButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: theme.borderRadius.sm,
  },
  tokenButtonActive: {
    backgroundColor: colors.primary,
  },
  tokenButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.textSecondary,
  },
  tokenButtonTextActive: {
    color: colors.text,
  },
  content: {
    flex: 1,
  },
  section: {
    padding: theme.spacing.md,
  },
  label: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 8,
  },
  labelRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  availableText: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: colors.border,
    paddingRight: 12,
  },
  input: {
    flex: 1,
    height: 56,
    paddingHorizontal: theme.spacing.md,
    fontSize: theme.fontSize.md,
    color: colors.text,
  },
  inputButton: {
    padding: 8,
  },
  contacts: {
    flexDirection: 'row',
    gap: theme.spacing.md,
    marginTop: theme.spacing.md,
  },
  contact: {
    alignItems: 'center',
  },
  contactAvatar: {
    width: 48,
    height: 48,
    borderRadius: 24,
    backgroundColor: colors.primary + '33',
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 4,
  },
  contactAvatarText: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.primary,
  },
  contactName: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  amountCard: {
    backgroundColor: colors.glassStrong,
    borderRadius: theme.borderRadius.xl,
    padding: theme.spacing.lg,
  },
  amountInputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  amountInput: {
    flex: 1,
    fontSize: 36,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    padding: 0,
  },
  tokenText: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.semibold,
    color: colors.textSecondary,
  },
  usdAmount: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
    marginTop: 8,
  },
  maxButton: {
    alignSelf: 'flex-start',
    backgroundColor: colors.primary + '33',
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: theme.borderRadius.sm,
    marginTop: 12,
  },
  maxButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.primary,
  },
  feeOptions: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
  },
  feeOption: {
    flex: 1,
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    borderWidth: 1,
    borderColor: colors.border,
  },
  feeOptionActive: {
    backgroundColor: colors.primary + '33',
    borderColor: colors.primary,
  },
  feeOptionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  feeOptionTitleActive: {
    color: colors.primary,
  },
  feeOptionSubtitle: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  feeOptionSubtitleActive: {
    color: colors.primary,
  },
  totalCard: {
    margin: theme.spacing.md,
    backgroundColor: colors.glassStrong,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
  },
  totalRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  totalLabel: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  totalAmountContainer: {
    alignItems: 'flex-end',
  },
  totalAmount: {
    fontSize: theme.fontSize.xl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: 4,
  },
  totalUsd: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  sendButton: {
    flexDirection: 'row',
    justifyContent: 'center',
    alignItems: 'center',
    gap: 8,
    margin: theme.spacing.md,
    backgroundColor: colors.accent,
    height: 56,
    borderRadius: theme.borderRadius.md,
  },
  sendButtonDisabled: {
    backgroundColor: colors.textMuted,
  },
  sendButtonText: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: '#000',
  },
});
