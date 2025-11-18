/**
 * LedgerAccountsScreen - View and select Ledger accounts
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useEffect, useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, FlatList, SafeAreaView, ActivityIndicator } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useLedger } from '../../hooks/useLedger';
import { HardwareAccount } from '../../types/hardware.types';

const LedgerAccountsScreen: React.FC = () => {
  const navigation = useNavigation();
  const { accounts, getAccounts } = useLedger();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadAccounts();
  }, []);

  const loadAccounts = async () => {
    try {
      await getAccounts();
    } finally {
      setLoading(false);
    }
  };

  const handleSelectAccount = (account: HardwareAccount) => {
    navigation.navigate('Home' as never);
  };

  const renderAccount = ({ item }: { item: HardwareAccount }) => (
    <TouchableOpacity style={styles.accountCard} onPress={() => handleSelectAccount(item)}>
      <View style={styles.accountIcon}>
        <Text style={styles.accountNumber}>{item.index + 1}</Text>
      </View>
      <View style={styles.accountInfo}>
        <Text style={styles.accountLabel}>{item.label}</Text>
        <Text style={styles.accountAddress}>{item.address}</Text>
        <Text style={styles.accountPath}>{item.derivationPath}</Text>
      </View>
    </TouchableOpacity>
  );

  if (loading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#4CAF50" />
        <Text style={styles.loadingText}>Loading accounts...</Text>
      </View>
    );
  }

  return (
    <SafeAreaView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Ledger Accounts</Text>
        <Text style={styles.subtitle}>Select an account to use</Text>
      </View>

      <FlatList data={accounts} keyExtractor={(item) => item.index.toString()} renderItem={renderAccount} />
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: '#f5f5f5' },
  loadingContainer: { flex: 1, justifyContent: 'center', alignItems: 'center' },
  loadingText: { marginTop: 12, fontSize: 16, color: '#666' },
  header: { padding: 20, backgroundColor: '#fff' },
  title: { fontSize: 28, fontWeight: 'bold', color: '#1a1a1a' },
  subtitle: { fontSize: 14, color: '#666', marginTop: 4 },
  accountCard: { flexDirection: 'row', backgroundColor: '#fff', padding: 16, marginHorizontal: 16, marginVertical: 8, borderRadius: 12, shadowColor: '#000', shadowOffset: { width: 0, height: 2 }, shadowOpacity: 0.1, shadowRadius: 4, elevation: 3 },
  accountIcon: { width: 50, height: 50, borderRadius: 25, backgroundColor: '#4CAF50', justifyContent: 'center', alignItems: 'center', marginRight: 16 },
  accountNumber: { color: '#fff', fontSize: 20, fontWeight: 'bold' },
  accountInfo: { flex: 1 },
  accountLabel: { fontSize: 16, fontWeight: 'bold', color: '#1a1a1a', marginBottom: 4 },
  accountAddress: { fontSize: 12, color: '#666', marginBottom: 4, fontFamily: 'monospace' },
  accountPath: { fontSize: 11, color: '#999', fontFamily: 'monospace' },
});

export default LedgerAccountsScreen;
