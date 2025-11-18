/**
 * ATMMarker - Custom map marker for ATM locations
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Marker } from 'react-native-maps';
import { ATMLocation } from '../../types/atm.types';

interface ATMMarkerProps {
  atm: ATMLocation;
  onPress: (atm: ATMLocation) => void;
}

const ATMMarker: React.FC<ATMMarkerProps> = ({ atm, onPress }) => {
  const getMarkerColor = (partner: string): string => {
    switch (partner) {
      case 'Coinme':
        return '#4CAF50';
      case 'Bitcoin Depot':
        return '#2196F3';
      case 'CoinFlip':
        return '#FF9800';
      default:
        return '#9C27B0';
    }
  };

  return (
    <Marker
      coordinate={{
        latitude: atm.lat,
        longitude: atm.lng,
      }}
      onPress={() => onPress(atm)}
      title={atm.name}
      description={`${atm.distance?.toFixed(1)} mi • ${atm.partner}`}
    >
      <View style={styles.markerContainer}>
        <View
          style={[
            styles.markerCircle,
            { backgroundColor: getMarkerColor(atm.partner) },
          ]}
        >
          <Text style={styles.markerText}>ATM</Text>
        </View>
        {atm.distance && atm.distance < 1 && (
          <View style={styles.distanceBadge}>
            <Text style={styles.distanceText}>{atm.distance.toFixed(1)} mi</Text>
          </View>
        )}
      </View>
    </Marker>
  );
};

const styles = StyleSheet.create({
  markerContainer: {
    alignItems: 'center',
  },
  markerCircle: {
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 2,
    borderColor: '#fff',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.3,
    shadowRadius: 4,
    elevation: 5,
  },
  markerText: {
    color: '#fff',
    fontSize: 10,
    fontWeight: 'bold',
  },
  distanceBadge: {
    marginTop: 4,
    backgroundColor: '#fff',
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 10,
    borderWidth: 1,
    borderColor: '#ddd',
  },
  distanceText: {
    fontSize: 10,
    color: '#333',
    fontWeight: '600',
  },
});

export default ATMMarker;
