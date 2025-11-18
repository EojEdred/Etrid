import React from 'react';
import { View, Text, StyleSheet, FlatList, TextInput, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useValidators } from '../../hooks/useValidators';
import { ValidatorCard } from '../../components/defi/ValidatorCard';

export default function ValidatorListScreen({ navigation }: any) {
  const { validators, loading, sortBy, changeSortBy, searchValidators } = useValidators();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Validators</Text>

      <TextInput
        style={styles.searchInput}
        placeholder="Search validators..."
        placeholderTextColor={colors.textSecondary}
        onChangeText={searchValidators}
      />

      <View style={styles.sortRow}>
        {['APY', 'Commission', 'Uptime'].map(sort => (
          <TouchableOpacity
            key={sort}
            style={[styles.sortButton, sortBy === sort.toLowerCase() && styles.sortButtonActive]}
            onPress={() => changeSortBy(sort.toLowerCase() as any)}
          >
            <Text style={[styles.sortText, sortBy === sort.toLowerCase() && styles.sortTextActive]}>
              {sort}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <FlatList
        data={validators}
        renderItem={({ item }) => (
          <ValidatorCard
            validator={item}
            onPress={() => navigation.navigate('ValidatorDetail', { validator: item })}
          />
        )}
        keyExtractor={item => item.address}
        contentContainerStyle={styles.list}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  title: {
    ...typography.h1,
    color: colors.text,
    padding: spacing.lg,
    paddingBottom: spacing.md,
  },
  searchInput: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
    ...typography.body,
    color: colors.text,
  },
  sortRow: {
    flexDirection: 'row',
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  sortButton: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
    marginRight: spacing.sm,
    borderRadius: borderRadius.md,
    backgroundColor: colors.surface,
  },
  sortButtonActive: {
    backgroundColor: colors.primary,
  },
  sortText: {
    ...typography.bodySmall,
    color: colors.text,
  },
  sortTextActive: {
    color: colors.background,
    fontWeight: '600',
  },
  list: {
    paddingHorizontal: spacing.lg,
  },
});
