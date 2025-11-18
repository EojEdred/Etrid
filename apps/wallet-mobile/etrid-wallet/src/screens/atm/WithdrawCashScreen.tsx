/**
 * WithdrawCashScreen - Multi-step cash withdrawal wizard
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  TextInput,
  ScrollView,
  SafeAreaView,
  Alert,
} from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { useWithdrawal } from '../../hooks/useWithdrawal';
import ATMService from '../../services/ATMService';
import { ATMLocation, WithdrawalRequest } from '../../types/atm.types';

type Asset = 'ETR' | 'BTC' | 'ETH' | 'USDT';

const WithdrawCashScreen: React.FC = () => {
  const navigation = useNavigation();
  const route = useRoute();
  const atm = (route.params as { atm: ATMLocation }).atm;
  const { withdrawal, loading, createWithdrawal } = useWithdrawal();

  const [step, setStep] = useState(1);
  const [amount, setAmount] = useState('');
  const [selectedAsset, setSelectedAsset] = useState<Asset>('ETR');

  // Mock exchange rates
  const exchangeRates = { ETR: 1.2, BTC: 45000, ETH: 2800, USDT: 1 };
  const mockBalance = { ETR: 1000, BTC: 0.5, ETH: 5, USDT: 5000 };

  const quickAmounts = [100, 200, 500, 1000];

  const calculateAssetAmount = (): number => {
    const usdAmount = parseFloat(amount) || 0;
    const total = ATMService.calculateTotal(usdAmount, atm.fee);
    return total / exchangeRates[selectedAsset];
  };

  const handleQuickAmount = (value: number) => {
    setAmount(value.toString());
  };

  const handleContinue = async () => {
    if (step === 1) {
      if (!amount || parseFloat(amount) <= 0) {
        Alert.alert('Error', 'Please enter a valid amount');
        return;
      }
      if (!ATMService.isWithinLimits(parseFloat(amount), atm)) {
        Alert.alert('Error', `Amount exceeds daily limit of $${atm.dailyLimit}`);
        return;
      }
      setStep(2);
    } else if (step === 2) {
      setStep(3);
    } else if (step === 3) {
      const request: WithdrawalRequest = {
        user: '0x1234...', // Mock user address
        amount: parseFloat(amount),
        asset: selectedAsset,
        atmPartner: atm.partner,
        atmLocationId: atm.id,
      };

      try {
        await createWithdrawal(request, 'mock-auth-token');
        setStep(4);
      } catch (error) {
        Alert.alert('Error', 'Failed to create withdrawal');
      }
    } else if (step === 4) {
      navigation.navigate('WithdrawalCode' as never, { withdrawal } as never);
    }
  };

  const renderStep1 = () => (
    <View style={styles.stepContainer}>
      <Text style={styles.stepTitle}>Select Amount</Text>

      <TextInput
        style={styles.amountInput}
        value={amount}
        onChangeText={setAmount}
        keyboardType="numeric"
        placeholder="0"
        placeholderTextColor="#ccc"
      />

      <Text style={styles.amountLabel}>USD</Text>

      <View style={styles.quickAmounts}>
        {quickAmounts.map((value) => (
          <TouchableOpacity
            key={value}
            style={styles.quickButton}
            onPress={() => handleQuickAmount(value)}
          >
            <Text style={styles.quickButtonText}>${value}</Text>
          </TouchableOpacity>
        ))}
      </View>

      <View style={styles.balanceInfo}>
        <Text style={styles.balanceLabel}>Available Balance</Text>
        <Text style={styles.balanceValue}>
          ${(mockBalance[selectedAsset] * exchangeRates[selectedAsset]).toFixed(2)}
        </Text>
      </View>
    </View>
  );

  const renderStep2 = () => (
    <View style={styles.stepContainer}>
      <Text style={styles.stepTitle}>Select Asset</Text>

      {(['ETR', 'BTC', 'ETH', 'USDT'] as Asset[]).map((asset) => (
        <TouchableOpacity
          key={asset}
          style={[
            styles.assetCard,
            selectedAsset === asset && styles.assetCardSelected,
          ]}
          onPress={() => setSelectedAsset(asset)}
        >
          <View style={styles.assetInfo}>
            <Text style={styles.assetName}>{asset}</Text>
            <Text style={styles.assetBalance}>
              Balance: {mockBalance[asset]} {asset}
            </Text>
          </View>
          <View
            style={[
              styles.radio,
              selectedAsset === asset && styles.radioSelected,
            ]}
          />
        </TouchableOpacity>
      ))}

      {amount && (
        <View style={styles.conversionInfo}>
          <Text style={styles.conversionText}>
            You will pay: {calculateAssetAmount().toFixed(6)} {selectedAsset}
          </Text>
          <Text style={styles.conversionRate}>
            Rate: 1 {selectedAsset} = ${exchangeRates[selectedAsset].toLocaleString()}
          </Text>
        </View>
      )}
    </View>
  );

  const renderStep3 = () => {
    const usdAmount = parseFloat(amount);
    const fee = ATMService.calculateFee(usdAmount, atm.fee);
    const total = ATMService.calculateTotal(usdAmount, atm.fee);
    const assetAmount = calculateAssetAmount();

    return (
      <View style={styles.stepContainer}>
        <Text style={styles.stepTitle}>Review</Text>

        <View style={styles.reviewCard}>
          <View style={styles.reviewRow}>
            <Text style={styles.reviewLabel}>Cash Amount</Text>
            <Text style={styles.reviewValue}>${usdAmount.toFixed(2)}</Text>
          </View>

          <View style={styles.reviewRow}>
            <Text style={styles.reviewLabel}>Fee ({atm.fee}%)</Text>
            <Text style={styles.reviewValue}>${fee.toFixed(2)}</Text>
          </View>

          <View style={styles.reviewDivider} />

          <View style={styles.reviewRow}>
            <Text style={styles.reviewLabelTotal}>Total</Text>
            <Text style={styles.reviewValueTotal}>${total.toFixed(2)}</Text>
          </View>

          <View style={styles.reviewDivider} />

          <View style={styles.reviewRow}>
            <Text style={styles.reviewLabelAsset}>You pay</Text>
            <Text style={styles.reviewValueAsset}>
              {assetAmount.toFixed(6)} {selectedAsset}
            </Text>
          </View>
        </View>

        <View style={styles.atmInfo}>
          <Text style={styles.atmInfoLabel}>ATM Location</Text>
          <Text style={styles.atmInfoName}>{atm.name}</Text>
          <Text style={styles.atmInfoAddress}>{atm.address}</Text>
        </View>
      </View>
    );
  };

  const renderStep4 = () => (
    <View style={styles.stepContainer}>
      <View style={styles.successIcon}>
        <Text style={styles.successEmoji}>✅</Text>
      </View>
      <Text style={styles.successTitle}>Withdrawal Created!</Text>
      <Text style={styles.successText}>
        Your withdrawal code has been generated. Use it within 30 minutes.
      </Text>
    </View>
  );

  return (
    <SafeAreaView style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Text style={styles.backButton}>←</Text>
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Withdraw Cash</Text>
      </View>

      {/* Progress */}
      <View style={styles.progress}>
        {[1, 2, 3, 4].map((s) => (
          <View
            key={s}
            style={[styles.progressDot, s <= step && styles.progressDotActive]}
          />
        ))}
      </View>

      {/* Content */}
      <ScrollView style={styles.content}>
        {step === 1 && renderStep1()}
        {step === 2 && renderStep2()}
        {step === 3 && renderStep3()}
        {step === 4 && renderStep4()}
      </ScrollView>

      {/* Actions */}
      <View style={styles.actions}>
        {step > 1 && step < 4 && (
          <TouchableOpacity
            style={styles.backActionButton}
            onPress={() => setStep(step - 1)}
          >
            <Text style={styles.backActionText}>Back</Text>
          </TouchableOpacity>
        )}

        <TouchableOpacity
          style={[styles.continueButton, step > 1 && step < 4 && { flex: 1 }]}
          onPress={handleContinue}
          disabled={loading}
        >
          <Text style={styles.continueButtonText}>
            {loading ? 'Processing...' : step < 4 ? 'Continue' : 'View Code'}
          </Text>
        </TouchableOpacity>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: 20,
    backgroundColor: '#fff',
  },
  backButton: {
    fontSize: 28,
    color: '#1a1a1a',
    marginRight: 16,
  },
  headerTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a1a1a',
  },
  progress: {
    flexDirection: 'row',
    justifyContent: 'center',
    padding: 20,
    gap: 8,
  },
  progressDot: {
    width: 40,
    height: 4,
    borderRadius: 2,
    backgroundColor: '#ddd',
  },
  progressDotActive: {
    backgroundColor: '#4CAF50',
  },
  content: {
    flex: 1,
  },
  stepContainer: {
    padding: 20,
  },
  stepTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 24,
  },
  amountInput: {
    fontSize: 48,
    fontWeight: 'bold',
    color: '#1a1a1a',
    textAlign: 'center',
    marginBottom: 8,
  },
  amountLabel: {
    fontSize: 20,
    color: '#666',
    textAlign: 'center',
    marginBottom: 24,
  },
  quickAmounts: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 24,
    gap: 8,
  },
  quickButton: {
    flex: 1,
    backgroundColor: '#fff',
    paddingVertical: 12,
    borderRadius: 8,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: '#ddd',
  },
  quickButtonText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1a1a1a',
  },
  balanceInfo: {
    backgroundColor: '#f9f9f9',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  balanceLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 4,
  },
  balanceValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  assetCard: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: '#fff',
    padding: 16,
    borderRadius: 12,
    marginBottom: 12,
    borderWidth: 2,
    borderColor: '#f0f0f0',
  },
  assetCardSelected: {
    borderColor: '#4CAF50',
    backgroundColor: '#f1f8f4',
  },
  assetInfo: {
    flex: 1,
  },
  assetName: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 4,
  },
  assetBalance: {
    fontSize: 14,
    color: '#666',
  },
  radio: {
    width: 24,
    height: 24,
    borderRadius: 12,
    borderWidth: 2,
    borderColor: '#ddd',
  },
  radioSelected: {
    borderColor: '#4CAF50',
    backgroundColor: '#4CAF50',
  },
  conversionInfo: {
    backgroundColor: '#f9f9f9',
    padding: 16,
    borderRadius: 12,
    marginTop: 12,
  },
  conversionText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1a1a1a',
    marginBottom: 4,
  },
  conversionRate: {
    fontSize: 14,
    color: '#666',
  },
  reviewCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 20,
    marginBottom: 20,
  },
  reviewRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 12,
  },
  reviewLabel: {
    fontSize: 16,
    color: '#666',
  },
  reviewValue: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1a1a1a',
  },
  reviewDivider: {
    height: 1,
    backgroundColor: '#f0f0f0',
    marginVertical: 12,
  },
  reviewLabelTotal: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
  },
  reviewValueTotal: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
  },
  reviewLabelAsset: {
    fontSize: 16,
    fontWeight: '600',
    color: '#4CAF50',
  },
  reviewValueAsset: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  atmInfo: {
    backgroundColor: '#f9f9f9',
    padding: 16,
    borderRadius: 12,
  },
  atmInfoLabel: {
    fontSize: 12,
    color: '#666',
    marginBottom: 8,
  },
  atmInfoName: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 4,
  },
  atmInfoAddress: {
    fontSize: 14,
    color: '#666',
  },
  successIcon: {
    alignItems: 'center',
    marginBottom: 20,
  },
  successEmoji: {
    fontSize: 80,
  },
  successTitle: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a1a1a',
    textAlign: 'center',
    marginBottom: 12,
  },
  successText: {
    fontSize: 16,
    color: '#666',
    textAlign: 'center',
    lineHeight: 24,
  },
  actions: {
    flexDirection: 'row',
    padding: 20,
    backgroundColor: '#fff',
    gap: 12,
    borderTopWidth: 1,
    borderTopColor: '#f0f0f0',
  },
  backActionButton: {
    paddingVertical: 16,
    paddingHorizontal: 24,
    borderRadius: 12,
    borderWidth: 1,
    borderColor: '#ddd',
    alignItems: 'center',
  },
  backActionText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#666',
  },
  continueButton: {
    flex: 2,
    backgroundColor: '#4CAF50',
    paddingVertical: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  continueButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  },
});

export default WithdrawCashScreen;
