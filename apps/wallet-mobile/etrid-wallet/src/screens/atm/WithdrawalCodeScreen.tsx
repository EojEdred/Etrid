/**
 * WithdrawalCodeScreen - Display withdrawal code with QR and timer
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity, SafeAreaView, Alert } from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { WithdrawalResponse } from '../../types/atm.types';
import WithdrawalCodeDisplay from '../../components/atm/WithdrawalCodeDisplay';
import ExpirationTimer from '../../components/atm/ExpirationTimer';
import LocationService from '../../services/LocationService';

const WithdrawalCodeScreen: React.FC = () => {
  const navigation = useNavigation();
  const route = useRoute();
  const withdrawal = (route.params as { withdrawal: WithdrawalResponse }).withdrawal;

  const handleGetDirections = async () => {
    const atm = withdrawal.atmLocation;
    await LocationService.openNavigation({ latitude: atm.lat, longitude: atm.lng }, atm.name);
  };

  const handleCopy = () => {
    Alert.alert('Copied!', 'Withdrawal code copied to clipboard');
  };

  const handleExpire = () => {
    Alert.alert('Code Expired', 'This withdrawal code has expired. Please create a new one.', [
      { text: 'OK', onPress: () => navigation.navigate('ATMLocator' as never) },
    ]);
  };

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.navigate('Home' as never)}>
          <Text style={styles.closeButton}>×</Text>
        </TouchableOpacity>
        <Text style={styles.headerTitle}>Withdrawal Code</Text>
      </View>

      <ExpirationTimer expiresAt={withdrawal.expiresAt} onExpire={handleExpire} />

      <WithdrawalCodeDisplay
        code={withdrawal.withdrawalCode}
        expiresAt={withdrawal.expiresAt}
        onCopy={handleCopy}
      />

      <View style={styles.atmInfo}>
        <Text style={styles.atmName}>{withdrawal.atmLocation.name}</Text>
        <Text style={styles.atmAddress}>{withdrawal.atmLocation.address}</Text>
      </View>

      <TouchableOpacity style={styles.directionsButton} onPress={handleGetDirections}>
        <Text style={styles.directionsButtonText}>Get Directions</Text>
      </TouchableOpacity>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f5f5', padding: 20 },
  header: { flexDirection: 'row', alignItems: 'center', marginBottom: 20 },
  closeButton: { fontSize: 40, color: '#1a1a1a', marginRight: 16 },
  headerTitle: { fontSize: 20, fontWeight: 'bold', color: '#1a1a1a' },
  atmInfo: { alignItems: 'center', marginTop: 20, padding: 16, backgroundColor: '#fff', borderRadius: 12 },
  atmName: { fontSize: 18, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 4 },
  atmAddress: { fontSize: 14, color: '#666' },
  directionsButton: { backgroundColor: '#4CAF50', padding: 16, borderRadius: 12, marginTop: 20 },
  directionsButtonText: { color: '#fff', fontSize: 16, fontWeight: 'bold', textAlign: 'center' },
});

export default WithdrawalCodeScreen;
