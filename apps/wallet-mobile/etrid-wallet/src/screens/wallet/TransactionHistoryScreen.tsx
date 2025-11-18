import React, { useState } from 'react';
import { View, Text, StyleSheet, FlatList, TouchableOpacity, RefreshControl } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../../contexts/AuthContext';
import { useTransactions } from '../../hooks/useTransactions';
import { TransactionItem } from '../../components/TransactionItem';
import { Transaction } from '../../services/TransactionService';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';

export const TransactionHistoryScreen: React.FC = () => {
  const navigation = useNavigation<any>();
  const { address } = useAuth();
  const [filter, setFilter] = useState<Transaction['type'] | undefined>(undefined);
  const { transactions, loading, refresh, loadMore, hasMore } = useTransactions(20, filter);

  const filters: Array<{ label: string; value: Transaction['type'] | undefined }> = [
    { label: 'All', value: undefined },
    { label: 'Sent', value: 'sent' },
    { label: 'Received', value: 'received' },
  ];

  return (
    <SafeAreaView style={styles.container} edges={['top']}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Text style={styles.backButton}>←</Text>
        </TouchableOpacity>
        <Text style={styles.title}>Transactions</Text>
        <View style={{ width: 40 }} />
      </View>

      <View style={styles.filters}>
        {filters.map(f => (
          <TouchableOpacity
            key={f.label}
            style={[styles.filterButton, filter === f.value && styles.filterButtonActive]}
            onPress={() => setFilter(f.value)}
          >
            <Text style={[styles.filterText, filter === f.value && styles.filterTextActive]}>
              {f.label}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <FlatList
        data={transactions}
        keyExtractor={item => item.id}
        renderItem={({ item }) => (
          <TransactionItem transaction={item} currentAddress={address || ''} />
        )}
        contentContainerStyle={styles.list}
        refreshControl={<RefreshControl refreshing={loading} onRefresh={refresh} />}
        onEndReached={loadMore}
        onEndReachedThreshold={0.5}
        ListEmptyComponent={
          <View style={styles.empty}>
            <Text style={styles.emptyText}>No transactions yet</Text>
          </View>
        }
      />
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: colors.background },
  header: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', paddingHorizontal: spacing.lg, paddingVertical: spacing.md },
  backButton: { fontSize: 28, color: colors.text },
  title: { ...typography.h2, color: colors.text },
  filters: { flexDirection: 'row', paddingHorizontal: spacing.lg, marginBottom: spacing.md },
  filterButton: { paddingHorizontal: spacing.md, paddingVertical: spacing.sm, borderRadius: borderRadius.md, marginRight: spacing.sm, backgroundColor: colors.surface },
  filterButtonActive: { backgroundColor: colors.primary },
  filterText: { ...typography.bodySmall, color: colors.text },
  filterTextActive: { color: colors.background, fontWeight: '600' },
  list: { paddingHorizontal: spacing.lg },
  empty: { paddingVertical: spacing.xxl, alignItems: 'center' },
  emptyText: { ...typography.body, color: colors.textSecondary },
});
