import React, { useState, useEffect } from 'react';
import { View, Text, TextInput, StyleSheet, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../theme/theme';

interface AmountInputProps {
  value: string;
  onChangeValue: (value: string) => void;
  currency: string;
  usdPrice?: number;
  maxBalance?: string;
  placeholder?: string;
}

export const AmountInput: React.FC<AmountInputProps> = ({
  value,
  onChangeValue,
  currency,
  usdPrice = 0,
  maxBalance,
  placeholder = '0.00',
}) => {
  const [usdValue, setUsdValue] = useState('0.00');

  useEffect(() => {
    const numValue = parseFloat(value) || 0;
    const usd = numValue * usdPrice;
    setUsdValue(usd.toFixed(2));
  }, [value, usdPrice]);

  const handleMaxPress = () => {
    if (maxBalance) {
      onChangeValue(maxBalance);
    }
  };

  return (
    <View style={styles.container}>
      <View style={styles.inputContainer}>
        <TextInput
          style={styles.input}
          value={value}
          onChangeText={onChangeValue}
          placeholder={placeholder}
          placeholderTextColor={colors.gray300}
          keyboardType="decimal-pad"
          autoFocus
        />
        <Text style={styles.currency}>{currency}</Text>
      </View>
      <View style={styles.footer}>
        <Text style={styles.usdValue}>≈ ${usdValue}</Text>
        {maxBalance && (
          <TouchableOpacity onPress={handleMaxPress} style={styles.maxButton}>
            <Text style={styles.maxButtonText}>MAX</Text>
          </TouchableOpacity>
        )}
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: { backgroundColor: colors.surface, borderRadius: borderRadius.lg, padding: spacing.lg },
  inputContainer: { flexDirection: 'row', alignItems: 'center', marginBottom: spacing.sm },
  input: { ...typography.h1, fontSize: 40, color: colors.text, flex: 1 },
  currency: { ...typography.h2, color: colors.textSecondary, marginLeft: spacing.sm },
  footer: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' },
  usdValue: { ...typography.body, color: colors.textSecondary },
  maxButton: { backgroundColor: colors.primary, paddingHorizontal: spacing.sm, paddingVertical: spacing.xs / 2, borderRadius: borderRadius.sm },
  maxButtonText: { ...typography.bodySmall, color: colors.background, fontWeight: 'bold' },
});
