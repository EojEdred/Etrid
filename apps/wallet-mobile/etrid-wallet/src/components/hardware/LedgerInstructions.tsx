/**
 * LedgerInstructions - Step-by-step Ledger usage instructions
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';

interface LedgerInstructionsProps {
  type: 'connect' | 'sign' | 'unlock';
}

const LedgerInstructions: React.FC<LedgerInstructionsProps> = ({ type }) => {
  const getInstructions = () => {
    switch (type) {
      case 'connect':
        return {
          title: 'Connect Your Ledger',
          steps: [
            'Turn on your Ledger device',
            'Enable Bluetooth in Ledger settings',
            'Open the Ëtrid app on your Ledger',
            'Tap "Scan" to search for your device',
            'Select your Ledger when it appears',
            'Confirm pairing on both devices',
          ],
        };

      case 'sign':
        return {
          title: 'Sign Transaction',
          steps: [
            'Check transaction details on Ledger screen',
            'Verify the recipient address matches',
            'Verify the amount is correct',
            'Press right button to approve',
            'Press both buttons to confirm',
            'Wait for signature confirmation',
          ],
        };

      case 'unlock':
        return {
          title: 'Unlock Your Ledger',
          steps: [
            'Press both buttons to wake up',
            'Enter your PIN code',
            'Press both buttons to confirm PIN',
            'Navigate to Ëtrid app',
            'Press both buttons to open app',
          ],
        };

      default:
        return { title: '', steps: [] };
    }
  };

  const { title, steps } = getInstructions();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>{title}</Text>

      <ScrollView style={styles.stepsContainer}>
        {steps.map((step, index) => (
          <View key={index} style={styles.stepRow}>
            <View style={styles.stepNumber}>
              <Text style={styles.stepNumberText}>{index + 1}</Text>
            </View>
            <Text style={styles.stepText}>{step}</Text>
          </View>
        ))}
      </ScrollView>

      <View style={styles.tipContainer}>
        <Text style={styles.tipIcon}>💡</Text>
        <Text style={styles.tipText}>
          {type === 'sign'
            ? 'Always verify transaction details on your Ledger screen before approving.'
            : 'Need help? Visit support.etrid.io for detailed guides.'}
        </Text>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    backgroundColor: '#f9f9f9',
    borderRadius: 12,
    padding: 20,
    marginVertical: 16,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 16,
  },
  stepsContainer: {
    maxHeight: 300,
  },
  stepRow: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    marginBottom: 16,
  },
  stepNumber: {
    width: 28,
    height: 28,
    borderRadius: 14,
    backgroundColor: '#4CAF50',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 12,
  },
  stepNumberText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: 'bold',
  },
  stepText: {
    flex: 1,
    fontSize: 15,
    color: '#333',
    lineHeight: 22,
    paddingTop: 4,
  },
  tipContainer: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    backgroundColor: '#E3F2FD',
    borderRadius: 8,
    padding: 12,
    marginTop: 12,
  },
  tipIcon: {
    fontSize: 20,
    marginRight: 8,
  },
  tipText: {
    flex: 1,
    fontSize: 13,
    color: '#1976D2',
    lineHeight: 18,
  },
});

export default LedgerInstructions;
