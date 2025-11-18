/**
 * WithdrawalCodeDisplay - Large withdrawal code display with QR code
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity, Clipboard } from 'react-native';
import QRCode from 'react-native-qrcode-svg';

interface WithdrawalCodeDisplayProps {
  code: string;
  expiresAt: string;
  onCopy?: () => void;
}

const WithdrawalCodeDisplay: React.FC<WithdrawalCodeDisplayProps> = ({
  code,
  expiresAt,
  onCopy,
}) => {
  const formatCode = (code: string): string => {
    if (code.length === 8) {
      return `${code.slice(0, 4)}-${code.slice(4)}`;
    }
    return code;
  };

  const handleCopy = () => {
    Clipboard.setString(code);
    onCopy?.();
  };

  return (
    <View style={styles.container}>
      {/* QR Code */}
      <View style={styles.qrContainer}>
        <QRCode value={code} size={200} backgroundColor="#fff" color="#000" />
      </View>

      {/* Withdrawal Code */}
      <View style={styles.codeContainer}>
        <Text style={styles.codeLabel}>Withdrawal Code</Text>
        <Text style={styles.code}>{formatCode(code)}</Text>
      </View>

      {/* Copy Button */}
      <TouchableOpacity style={styles.copyButton} onPress={handleCopy}>
        <Text style={styles.copyButtonText}>Copy Code</Text>
      </TouchableOpacity>

      {/* Instructions */}
      <View style={styles.instructions}>
        <Text style={styles.instructionTitle}>How to use:</Text>
        <Text style={styles.instructionText}>
          1. Go to the selected ATM
        </Text>
        <Text style={styles.instructionText}>
          2. Select "Withdraw Cash" on the ATM screen
        </Text>
        <Text style={styles.instructionText}>
          3. Scan QR code or enter code: {formatCode(code)}
        </Text>
        <Text style={styles.instructionText}>4. Collect your cash</Text>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    paddingVertical: 20,
  },
  qrContainer: {
    backgroundColor: '#fff',
    padding: 20,
    borderRadius: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.15,
    shadowRadius: 8,
    elevation: 5,
    marginBottom: 24,
  },
  codeContainer: {
    alignItems: 'center',
    marginBottom: 20,
  },
  codeLabel: {
    fontSize: 14,
    color: '#666',
    marginBottom: 8,
    textTransform: 'uppercase',
    letterSpacing: 1,
  },
  code: {
    fontSize: 36,
    fontWeight: 'bold',
    color: '#1a1a1a',
    letterSpacing: 4,
    fontFamily: 'monospace',
  },
  copyButton: {
    backgroundColor: '#4CAF50',
    paddingHorizontal: 32,
    paddingVertical: 12,
    borderRadius: 24,
    marginBottom: 32,
  },
  copyButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
  },
  instructions: {
    backgroundColor: '#f5f5f5',
    borderRadius: 12,
    padding: 20,
    width: '100%',
  },
  instructionTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 12,
  },
  instructionText: {
    fontSize: 14,
    color: '#666',
    marginBottom: 8,
    lineHeight: 20,
  },
});

export default WithdrawalCodeDisplay;
