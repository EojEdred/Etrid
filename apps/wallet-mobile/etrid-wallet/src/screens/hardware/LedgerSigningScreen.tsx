/**
 * LedgerSigningScreen - Sign transactions with Ledger
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, SafeAreaView, ActivityIndicator, Alert } from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { useLedger } from '../../hooks/useLedger';
import { TransactionSigningRequest } from '../../types/hardware.types';
import LedgerInstructions from '../../components/hardware/LedgerInstructions';

const LedgerSigningScreen: React.FC = () => {
  const navigation = useNavigation();
  const route = useRoute();
  const txRequest = (route.params as { txRequest: TransactionSigningRequest }).txRequest;
  const { signTransaction, connectedDevice } = useLedger();
  const [signing, setSigning] = useState(false);
  const [signed, setSigned] = useState(false);

  const handleSign = async () => {
    if (!connectedDevice) {
      Alert.alert('Error', 'No Ledger connected');
      return;
    }

    setSigning(true);
    try {
      const signature = await signTransaction(txRequest);
      setSigned(true);
      Alert.alert('Success', 'Transaction signed successfully!', [
        { text: 'OK', onPress: () => navigation.goBack() },
      ]);
    } catch (error) {
      Alert.alert('Error', 'Failed to sign transaction');
    } finally {
      setSigning(false);
    }
  };

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Sign Transaction</Text>
        <Text style={styles.subtitle}>Approve on your Ledger device</Text>
      </View>

      <View style={styles.txDetails}>
        <Text style={styles.sectionTitle}>Transaction Details</Text>
        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>To</Text>
          <Text style={styles.detailValue} numberOfLines={1}>{txRequest.to}</Text>
        </View>
        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>Amount</Text>
          <Text style={styles.detailValue}>{txRequest.amount} {txRequest.asset}</Text>
        </View>
        <View style={styles.detailRow}>
          <Text style={styles.detailLabel}>Fee</Text>
          <Text style={styles.detailValue}>{txRequest.fee}</Text>
        </View>
      </View>

      <LedgerInstructions type="sign" />

      {signing && (
        <View style={styles.signingContainer}>
          <ActivityIndicator size="large" color="#4CAF50" />
          <Text style={styles.signingText}>Waiting for approval...</Text>
        </View>
      )}

      {!signed && !signing && (
        <TouchableOpacity style={styles.signButton} onPress={handleSign}>
          <Text style={styles.signButtonText}>Request Signature</Text>
        </TouchableOpacity>
      )}

      {signed && (
        <View style={styles.successContainer}>
          <Text style={styles.successIcon}>✅</Text>
          <Text style={styles.successText}>Transaction signed!</Text>
        </View>
      )}
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f5f5', padding: 20 },
  header: { marginBottom: 20 },
  title: { fontSize: 28, fontWeight: 'bold', color: '#1a1a1a' },
  subtitle: { fontSize: 14, color: '#666', marginTop: 4 },
  txDetails: { backgroundColor: '#fff', borderRadius: 12, padding: 20, marginBottom: 20 },
  sectionTitle: { fontSize: 18, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 16 },
  detailRow: { flexDirection: 'row', justifyContent: 'space-between', marginBottom: 12 },
  detailLabel: { fontSize: 16, color: '#666' },
  detailValue: { fontSize: 16, fontWeight: '600', color: '#1a1a1a', flex: 1, textAlign: 'right' },
  signingContainer: { alignItems: 'center', marginVertical: 32 },
  signingText: { marginTop: 12, fontSize: 16, color: '#666' },
  signButton: { backgroundColor: '#4CAF50', padding: 16, borderRadius: 12, marginTop: 20 },
  signButtonText: { color: '#fff', fontSize: 16, fontWeight: 'bold', textAlign: 'center' },
  successContainer: { alignItems: 'center', marginVertical: 32 },
  successIcon: { fontSize: 80, marginBottom: 12 },
  successText: { fontSize: 20, fontWeight: 'bold', color: '#4CAF50' },
});

export default LedgerSigningScreen;
