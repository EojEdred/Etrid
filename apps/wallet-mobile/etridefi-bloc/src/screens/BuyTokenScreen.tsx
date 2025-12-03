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
 * BuyTokenScreen - Purchase tokens with fiat or crypto
 * Matches iOS Swift equivalent
 */
export default function BuyTokenScreen({route, navigation}: any) {
  const {symbol = 'ETR', name = 'Etrid'} = route.params || {};
  const [amount, setAmount] = useState('');
  const [paymentMethod, setPaymentMethod] = useState<'card' | 'crypto'>('card');

  const quickAmounts = ['50', '100', '250', '500'];

  const handleBuy = () => {
    Alert.alert(
      'Coming Soon',
      `Token purchases will be available soon. You selected to buy ${amount || '0'} USD worth of ${symbol}.`,
      [{text: 'OK', onPress: () => navigation.goBack()}]
    );
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Token Info */}
        <View style={styles.tokenHeader}>
          <View style={styles.tokenIcon}>
            <Text style={styles.tokenIconText}>{symbol[0]}</Text>
          </View>
          <View>
            <Text style={styles.tokenName}>{name}</Text>
            <Text style={styles.tokenSymbol}>{symbol}</Text>
          </View>
        </View>

        {/* Amount Input */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Amount (USD)</Text>
          <View style={styles.inputContainer}>
            <Text style={styles.currencyPrefix}>$</Text>
            <TextInput
              style={styles.amountInput}
              value={amount}
              onChangeText={setAmount}
              placeholder="0.00"
              placeholderTextColor={colors.textMuted}
              keyboardType="decimal-pad"
            />
          </View>

          {/* Quick Amount Buttons */}
          <View style={styles.quickAmounts}>
            {quickAmounts.map(amt => (
              <TouchableOpacity
                key={amt}
                style={[
                  styles.quickAmountButton,
                  amount === amt && styles.quickAmountButtonActive,
                ]}
                onPress={() => setAmount(amt)}>
                <Text
                  style={[
                    styles.quickAmountText,
                    amount === amt && styles.quickAmountTextActive,
                  ]}>
                  ${amt}
                </Text>
              </TouchableOpacity>
            ))}
          </View>
        </View>

        {/* Payment Method */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Payment Method</Text>
          <TouchableOpacity
            style={[
              styles.methodOption,
              paymentMethod === 'card' && styles.methodOptionActive,
            ]}
            onPress={() => setPaymentMethod('card')}>
            <Icon name="credit-card" size={20} color={colors.primary} />
            <View style={styles.methodInfo}>
              <Text style={styles.methodTitle}>Credit/Debit Card</Text>
              <Text style={styles.methodSubtitle}>Visa, Mastercard, AMEX</Text>
            </View>
            {paymentMethod === 'card' && (
              <Icon name="check-circle" size={20} color={colors.primary} />
            )}
          </TouchableOpacity>
          <TouchableOpacity
            style={[
              styles.methodOption,
              paymentMethod === 'crypto' && styles.methodOptionActive,
            ]}
            onPress={() => setPaymentMethod('crypto')}>
            <Icon name="repeat" size={20} color={colors.primary} />
            <View style={styles.methodInfo}>
              <Text style={styles.methodTitle}>Crypto Swap</Text>
              <Text style={styles.methodSubtitle}>Use existing crypto</Text>
            </View>
            {paymentMethod === 'crypto' && (
              <Icon name="check-circle" size={20} color={colors.primary} />
            )}
          </TouchableOpacity>
        </View>

        {/* Estimated Output */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>You Will Receive</Text>
          <Text style={styles.estimatedAmount}>
            ~{amount ? (parseFloat(amount) / 2.45).toFixed(4) : '0.0000'} {symbol}
          </Text>
          <Text style={styles.estimatedNote}>
            Price: $2.45 per {symbol} (includes fees)
          </Text>
        </View>

        {/* Coming Soon Badge */}
        <View style={styles.comingSoonBadge}>
          <Icon name="clock" size={16} color={colors.warning} />
          <Text style={styles.comingSoonText}>Fiat purchases coming soon</Text>
        </View>

        {/* Buy Button */}
        <TouchableOpacity style={styles.buyButton} onPress={handleBuy}>
          <Text style={styles.buyButtonText}>Buy {symbol}</Text>
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
  tokenHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
    gap: theme.spacing.md,
  },
  tokenIcon: {
    width: 48,
    height: 48,
    borderRadius: 24,
    backgroundColor: colors.primary,
    alignItems: 'center',
    justifyContent: 'center',
  },
  tokenIconText: {
    fontSize: 20,
    fontWeight: '700',
    color: colors.text,
  },
  tokenName: {
    fontSize: theme.fontSize.lg,
    fontWeight: '700',
    color: colors.text,
  },
  tokenSymbol: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
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
  inputContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    paddingHorizontal: theme.spacing.md,
  },
  currencyPrefix: {
    fontSize: 24,
    fontWeight: '600',
    color: colors.text,
  },
  amountInput: {
    flex: 1,
    fontSize: 24,
    fontWeight: '600',
    color: colors.text,
    paddingVertical: theme.spacing.md,
    marginLeft: 4,
  },
  quickAmounts: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
    marginTop: theme.spacing.md,
  },
  quickAmountButton: {
    flex: 1,
    paddingVertical: theme.spacing.sm,
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    alignItems: 'center',
  },
  quickAmountButtonActive: {
    backgroundColor: colors.primary,
  },
  quickAmountText: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.textSecondary,
  },
  quickAmountTextActive: {
    color: colors.text,
  },
  methodOption: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: theme.spacing.md,
    backgroundColor: colors.background,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.sm,
    gap: theme.spacing.md,
  },
  methodOptionActive: {
    borderWidth: 1,
    borderColor: colors.primary,
  },
  methodInfo: {
    flex: 1,
  },
  methodTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  methodSubtitle: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  estimatedAmount: {
    fontSize: 24,
    fontWeight: '700',
    color: colors.text,
  },
  estimatedNote: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginTop: 4,
  },
  comingSoonBadge: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: `${colors.warning}20`,
    padding: theme.spacing.sm,
    borderRadius: theme.borderRadius.sm,
    marginBottom: theme.spacing.md,
    gap: 8,
  },
  comingSoonText: {
    fontSize: theme.fontSize.sm,
    color: colors.warning,
    fontWeight: '600',
  },
  buyButton: {
    backgroundColor: colors.primary,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  buyButtonText: {
    fontSize: theme.fontSize.md,
    fontWeight: '700',
    color: colors.text,
  },
});
