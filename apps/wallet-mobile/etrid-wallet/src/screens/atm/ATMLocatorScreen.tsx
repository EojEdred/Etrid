/**
 * ATMLocatorScreen - Find nearby ATMs with map and list views
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useState, useEffect } from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  FlatList,
  TextInput,
  ActivityIndicator,
  SafeAreaView,
} from 'react-native';
import MapView, { PROVIDER_GOOGLE } from 'react-native-maps';
import { useNavigation } from '@react-navigation/native';
import { useATMLocations } from '../../hooks/useATMLocations';
import { useLocation } from '../../hooks/useLocation';
import { ATMLocation, ATMFilter } from '../../types/atm.types';
import ATMMarker from '../../components/atm/ATMMarker';
import ATMCard from '../../components/atm/ATMCard';

type ViewMode = 'map' | 'list';

const ATMLocatorScreen: React.FC = () => {
  const navigation = useNavigation();
  const [viewMode, setViewMode] = useState<ViewMode>('map');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedFilter, setSelectedFilter] = useState<ATMFilter>({});

  const { location, loading: locationLoading, requestPermission } = useLocation(true);
  const { atms, loading, error, searchATMs, filterATMs } = useATMLocations(
    location || undefined,
    10
  );

  useEffect(() => {
    requestPermission();
  }, []);

  const handleSearch = (query: string) => {
    setSearchQuery(query);
    if (query.length >= 3) {
      searchATMs(query);
    }
  };

  const handleFilterChange = (filter: ATMFilter) => {
    setSelectedFilter(filter);
    filterATMs(filter);
  };

  const handleATMPress = (atm: ATMLocation) => {
    navigation.navigate('ATMDetail' as never, { atm } as never);
  };

  const renderMapView = () => {
    if (!location) {
      return (
        <View style={styles.loadingContainer}>
          <ActivityIndicator size="large" color="#4CAF50" />
          <Text style={styles.loadingText}>Getting your location...</Text>
        </View>
      );
    }

    return (
      <MapView
        provider={PROVIDER_GOOGLE}
        style={styles.map}
        initialRegion={{
          latitude: location.latitude,
          longitude: location.longitude,
          latitudeDelta: 0.1,
          longitudeDelta: 0.1,
        }}
        showsUserLocation
        showsMyLocationButton
      >
        {atms.map((atm) => (
          <ATMMarker key={atm.id} atm={atm} onPress={handleATMPress} />
        ))}
      </MapView>
    );
  };

  const renderListView = () => {
    if (loading) {
      return (
        <View style={styles.loadingContainer}>
          <ActivityIndicator size="large" color="#4CAF50" />
          <Text style={styles.loadingText}>Finding ATMs...</Text>
        </View>
      );
    }

    if (atms.length === 0) {
      return (
        <View style={styles.emptyContainer}>
          <Text style={styles.emptyIcon}>📍</Text>
          <Text style={styles.emptyTitle}>No ATMs found</Text>
          <Text style={styles.emptyText}>
            Try adjusting your search or filters
          </Text>
        </View>
      );
    }

    return (
      <FlatList
        data={atms}
        keyExtractor={(item) => item.id}
        renderItem={({ item }) => (
          <ATMCard atm={item} onPress={handleATMPress} />
        )}
        contentContainerStyle={styles.listContainer}
      />
    );
  };

  const renderFilterButtons = () => (
    <View style={styles.filterContainer}>
      <TouchableOpacity
        style={[
          styles.filterButton,
          !selectedFilter.partner && styles.filterButtonActive,
        ]}
        onPress={() => handleFilterChange({})}
      >
        <Text
          style={[
            styles.filterButtonText,
            !selectedFilter.partner && styles.filterButtonTextActive,
          ]}
        >
          All
        </Text>
      </TouchableOpacity>

      <TouchableOpacity
        style={[
          styles.filterButton,
          selectedFilter.partner === 'Coinme' && styles.filterButtonActive,
        ]}
        onPress={() => handleFilterChange({ partner: 'Coinme' })}
      >
        <Text
          style={[
            styles.filterButtonText,
            selectedFilter.partner === 'Coinme' && styles.filterButtonTextActive,
          ]}
        >
          Coinme
        </Text>
      </TouchableOpacity>

      <TouchableOpacity
        style={[
          styles.filterButton,
          selectedFilter.partner === 'Bitcoin Depot' && styles.filterButtonActive,
        ]}
        onPress={() => handleFilterChange({ partner: 'Bitcoin Depot' })}
      >
        <Text
          style={[
            styles.filterButtonText,
            selectedFilter.partner === 'Bitcoin Depot' &&
              styles.filterButtonTextActive,
          ]}
        >
          Bitcoin Depot
        </Text>
      </TouchableOpacity>

      <TouchableOpacity
        style={[
          styles.filterButton,
          selectedFilter.partner === 'CoinFlip' && styles.filterButtonActive,
        ]}
        onPress={() => handleFilterChange({ partner: 'CoinFlip' })}
      >
        <Text
          style={[
            styles.filterButtonText,
            selectedFilter.partner === 'CoinFlip' && styles.filterButtonTextActive,
          ]}
        >
          CoinFlip
        </Text>
      </TouchableOpacity>
    </View>
  );

  return (
    <SafeAreaView style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.title}>Find ATM</Text>
        <Text style={styles.subtitle}>
          {atms.length} location{atms.length !== 1 ? 's' : ''} nearby
        </Text>
      </View>

      {/* Search Bar */}
      <View style={styles.searchContainer}>
        <TextInput
          style={styles.searchInput}
          placeholder="Search by address or zip code"
          value={searchQuery}
          onChangeText={handleSearch}
        />
      </View>

      {/* Filters */}
      {renderFilterButtons()}

      {/* View Toggle */}
      <View style={styles.viewToggle}>
        <TouchableOpacity
          style={[styles.toggleButton, viewMode === 'map' && styles.toggleButtonActive]}
          onPress={() => setViewMode('map')}
        >
          <Text
            style={[
              styles.toggleButtonText,
              viewMode === 'map' && styles.toggleButtonTextActive,
            ]}
          >
            Map
          </Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={[styles.toggleButton, viewMode === 'list' && styles.toggleButtonActive]}
          onPress={() => setViewMode('list')}
        >
          <Text
            style={[
              styles.toggleButtonText,
              viewMode === 'list' && styles.toggleButtonTextActive,
            ]}
          >
            List
          </Text>
        </TouchableOpacity>
      </View>

      {/* Content */}
      <View style={styles.content}>
        {viewMode === 'map' ? renderMapView() : renderListView()}
      </View>

      {/* Error Message */}
      {error && (
        <View style={styles.errorContainer}>
          <Text style={styles.errorText}>{error}</Text>
        </View>
      )}
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    padding: 20,
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#1a1a1a',
  },
  subtitle: {
    fontSize: 14,
    color: '#666',
    marginTop: 4,
  },
  searchContainer: {
    paddingHorizontal: 20,
    paddingVertical: 12,
    backgroundColor: '#fff',
  },
  searchInput: {
    backgroundColor: '#f5f5f5',
    borderRadius: 24,
    paddingHorizontal: 20,
    paddingVertical: 12,
    fontSize: 16,
  },
  filterContainer: {
    flexDirection: 'row',
    paddingHorizontal: 20,
    paddingVertical: 12,
    backgroundColor: '#fff',
    gap: 8,
  },
  filterButton: {
    paddingHorizontal: 16,
    paddingVertical: 8,
    borderRadius: 20,
    borderWidth: 1,
    borderColor: '#ddd',
    backgroundColor: '#fff',
  },
  filterButtonActive: {
    backgroundColor: '#4CAF50',
    borderColor: '#4CAF50',
  },
  filterButtonText: {
    fontSize: 14,
    color: '#666',
    fontWeight: '500',
  },
  filterButtonTextActive: {
    color: '#fff',
  },
  viewToggle: {
    flexDirection: 'row',
    marginHorizontal: 20,
    marginVertical: 12,
    backgroundColor: '#f5f5f5',
    borderRadius: 24,
    padding: 4,
  },
  toggleButton: {
    flex: 1,
    paddingVertical: 10,
    alignItems: 'center',
    borderRadius: 20,
  },
  toggleButtonActive: {
    backgroundColor: '#fff',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 2,
  },
  toggleButtonText: {
    fontSize: 16,
    fontWeight: '500',
    color: '#666',
  },
  toggleButtonTextActive: {
    color: '#1a1a1a',
  },
  content: {
    flex: 1,
  },
  map: {
    flex: 1,
  },
  listContainer: {
    paddingVertical: 8,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    marginTop: 12,
    fontSize: 16,
    color: '#666',
  },
  emptyContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    paddingHorizontal: 40,
  },
  emptyIcon: {
    fontSize: 60,
    marginBottom: 16,
  },
  emptyTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 8,
  },
  emptyText: {
    fontSize: 14,
    color: '#666',
    textAlign: 'center',
  },
  errorContainer: {
    position: 'absolute',
    bottom: 20,
    left: 20,
    right: 20,
    backgroundColor: '#F44336',
    borderRadius: 12,
    padding: 16,
  },
  errorText: {
    color: '#fff',
    fontSize: 14,
    textAlign: 'center',
  },
});

export default ATMLocatorScreen;
