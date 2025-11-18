import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  FlatList,
  TouchableOpacity,
  StyleSheet,
  TextInput,
  ActivityIndicator,
  RefreshControl,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useGPU } from '../../hooks/useGPU';
import { useScreenTracking } from '../../hooks/useAnalytics';
import { GPUSearchFilters } from '../../services/GPUService';

export default function GPUMarketplaceScreen() {
  useScreenTracking('GPUMarketplace');

  const navigation = useNavigation();
  const { gpus, loading, error, searchGPUs, refresh } = useGPU();

  const [filters, setFilters] = useState<GPUSearchFilters>({
    sortBy: 'price',
    sortOrder: 'asc',
  });
  const [searchText, setSearchText] = useState('');
  const [refreshing, setRefreshing] = useState(false);

  useEffect(() => {
    searchGPUs(filters);
  }, [filters]);

  const handleRefresh = async () => {
    setRefreshing(true);
    await refresh();
    setRefreshing(false);
  };

  const handleGPUPress = (gpuId: string) => {
    navigation.navigate('GPUDetail' as never, { gpuId } as never);
  };

  const handleSearch = () => {
    searchGPUs({ ...filters, model: searchText });
  };

  const handleFilterChange = (key: keyof GPUSearchFilters, value: any) => {
    setFilters({ ...filters, [key]: value });
  };

  const renderGPUCard = ({ item }: any) => (
    <TouchableOpacity
      style={styles.gpuCard}
      onPress={() => handleGPUPress(item.id)}
    >
      <View style={styles.gpuHeader}>
        <Text style={styles.gpuModel}>{item.model}</Text>
        <View style={[styles.badge, styles[`badge${item.reputation}`]]}>
          <Text style={styles.badgeText}>{item.reputation}</Text>
        </View>
      </View>

      <View style={styles.gpuSpec}>
        <Text style={styles.specLabel}>VRAM:</Text>
        <Text style={styles.specValue}>{item.vram}GB</Text>
      </View>

      <View style={styles.gpuSpec}>
        <Text style={styles.specLabel}>Compute:</Text>
        <Text style={styles.specValue}>{item.computeUnits} TFLOPS</Text>
      </View>

      <View style={styles.gpuSpec}>
        <Text style={styles.specLabel}>Location:</Text>
        <Text style={styles.specValue}>{item.location}</Text>
      </View>

      <View style={styles.gpuSpec}>
        <Text style={styles.specLabel}>Uptime:</Text>
        <Text style={styles.specValue}>{item.uptime}%</Text>
      </View>

      <View style={styles.gpuFooter}>
        <View>
          <Text style={styles.priceLabel}>Price per hour</Text>
          <Text style={styles.price}>{item.pricePerHour} ËDSC</Text>
        </View>
        <View style={[styles.statusBadge, item.available ? styles.available : styles.unavailable]}>
          <Text style={styles.statusText}>
            {item.available ? 'Available' : 'In Use'}
          </Text>
        </View>
      </View>
    </TouchableOpacity>
  );

  return (
    <View style={styles.container}>
      <Text style={styles.title}>GPU Marketplace</Text>

      {/* Search Bar */}
      <View style={styles.searchContainer}>
        <TextInput
          style={styles.searchInput}
          placeholder="Search GPU models..."
          value={searchText}
          onChangeText={setSearchText}
          onSubmitEditing={handleSearch}
        />
      </View>

      {/* Filters */}
      <View style={styles.filterContainer}>
        <TouchableOpacity
          style={styles.filterButton}
          onPress={() => handleFilterChange('sortBy', filters.sortBy === 'price' ? 'compute' : 'price')}
        >
          <Text style={styles.filterText}>
            Sort: {filters.sortBy === 'price' ? 'Price' : 'Compute'}
          </Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.filterButton}
          onPress={() => handleFilterChange('sortOrder', filters.sortOrder === 'asc' ? 'desc' : 'asc')}
        >
          <Text style={styles.filterText}>
            {filters.sortOrder === 'asc' ? '↑' : '↓'}
          </Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={styles.filterButton}
          onPress={() => handleFilterChange('minVRAM', filters.minVRAM ? undefined : 16)}
        >
          <Text style={styles.filterText}>
            {filters.minVRAM ? `${filters.minVRAM}GB+` : 'All VRAM'}
          </Text>
        </TouchableOpacity>
      </View>

      {/* GPU List */}
      {loading && !refreshing ? (
        <View style={styles.loadingContainer}>
          <ActivityIndicator size="large" color="#007AFF" />
        </View>
      ) : error ? (
        <View style={styles.errorContainer}>
          <Text style={styles.errorText}>{error}</Text>
          <TouchableOpacity style={styles.retryButton} onPress={() => searchGPUs(filters)}>
            <Text style={styles.retryText}>Retry</Text>
          </TouchableOpacity>
        </View>
      ) : (
        <FlatList
          data={gpus}
          renderItem={renderGPUCard}
          keyExtractor={(item) => item.id || ''}
          contentContainerStyle={styles.listContent}
          refreshControl={
            <RefreshControl refreshing={refreshing} onRefresh={handleRefresh} />
          }
          ListEmptyComponent={
            <View style={styles.emptyContainer}>
              <Text style={styles.emptyText}>No GPUs found</Text>
            </View>
          }
        />
      )}

      {/* Register GPU Button (for providers) */}
      <TouchableOpacity
        style={styles.fab}
        onPress={() => navigation.navigate('RegisterGPU' as never)}
      >
        <Text style={styles.fabText}>+</Text>
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#F5F5F5',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    marginTop: 60,
    marginHorizontal: 20,
    marginBottom: 20,
  },
  searchContainer: {
    marginHorizontal: 20,
    marginBottom: 16,
  },
  searchInput: {
    backgroundColor: '#FFF',
    borderRadius: 12,
    padding: 12,
    fontSize: 16,
    borderWidth: 1,
    borderColor: '#E0E0E0',
  },
  filterContainer: {
    flexDirection: 'row',
    marginHorizontal: 20,
    marginBottom: 16,
    gap: 8,
  },
  filterButton: {
    backgroundColor: '#FFF',
    borderRadius: 8,
    paddingVertical: 8,
    paddingHorizontal: 16,
    borderWidth: 1,
    borderColor: '#E0E0E0',
  },
  filterText: {
    fontSize: 14,
    color: '#333',
  },
  listContent: {
    paddingHorizontal: 20,
    paddingBottom: 100,
  },
  gpuCard: {
    backgroundColor: '#FFF',
    borderRadius: 16,
    padding: 16,
    marginBottom: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  gpuHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 12,
  },
  gpuModel: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#000',
  },
  badge: {
    paddingVertical: 4,
    paddingHorizontal: 12,
    borderRadius: 12,
  },
  badgeBronze: { backgroundColor: '#CD7F32' },
  badgeSilver: { backgroundColor: '#C0C0C0' },
  badgeGold: { backgroundColor: '#FFD700' },
  badgePlatinum: { backgroundColor: '#E5E4E2' },
  badgeText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#FFF',
  },
  gpuSpec: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 8,
  },
  specLabel: {
    fontSize: 14,
    color: '#666',
  },
  specValue: {
    fontSize: 14,
    fontWeight: '600',
    color: '#000',
  },
  gpuFooter: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginTop: 12,
    paddingTop: 12,
    borderTopWidth: 1,
    borderTopColor: '#F0F0F0',
  },
  priceLabel: {
    fontSize: 12,
    color: '#666',
    marginBottom: 4,
  },
  price: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#007AFF',
  },
  statusBadge: {
    paddingVertical: 6,
    paddingHorizontal: 12,
    borderRadius: 8,
  },
  available: { backgroundColor: '#34C759' },
  unavailable: { backgroundColor: '#FF3B30' },
  statusText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#FFF',
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  errorContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  errorText: {
    fontSize: 16,
    color: '#FF3B30',
    textAlign: 'center',
    marginBottom: 16,
  },
  retryButton: {
    backgroundColor: '#007AFF',
    borderRadius: 8,
    paddingVertical: 12,
    paddingHorizontal: 24,
  },
  retryText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#FFF',
  },
  emptyContainer: {
    padding: 40,
    alignItems: 'center',
  },
  emptyText: {
    fontSize: 16,
    color: '#999',
  },
  fab: {
    position: 'absolute',
    right: 20,
    bottom: 20,
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: '#007AFF',
    justifyContent: 'center',
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.3,
    shadowRadius: 4,
    elevation: 8,
  },
  fabText: {
    fontSize: 32,
    fontWeight: '300',
    color: '#FFF',
  },
});
