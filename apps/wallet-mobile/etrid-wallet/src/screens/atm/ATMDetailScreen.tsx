/**
 * ATMDetailScreen - Detailed ATM information
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  SafeAreaView,
  Linking,
} from 'react-native';
import { useNavigation, useRoute } from '@react-navigation/native';
import { ATMLocation } from '../../types/atm.types';
import LocationService from '../../services/LocationService';

const ATMDetailScreen: React.FC = () => {
  const navigation = useNavigation();
  const route = useRoute();
  const atm = (route.params as { atm: ATMLocation }).atm;

  const handleGetDirections = async () => {
    try {
      await LocationService.openNavigation(
        { latitude: atm.lat, longitude: atm.lng },
        atm.name
      );
    } catch (error) {
      console.error('Failed to open navigation:', error);
    }
  };

  const handleWithdrawCash = () => {
    navigation.navigate('WithdrawCash' as never, { atm } as never);
  };

  const getPartnerColor = (partner: string): string => {
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
    <SafeAreaView style={styles.container}>
      <ScrollView>
        {/* Header */}
        <View style={styles.header}>
          <TouchableOpacity
            style={styles.backButton}
            onPress={() => navigation.goBack()}
          >
            <Text style={styles.backButtonText}>←</Text>
          </TouchableOpacity>

          <View style={styles.headerContent}>
            <Text style={styles.name}>{atm.name}</Text>
            <View
              style={[
                styles.partnerBadge,
                { backgroundColor: getPartnerColor(atm.partner) },
              ]}
            >
              <Text style={styles.partnerText}>{atm.partner}</Text>
            </View>
          </View>
        </View>

        {/* Distance */}
        {atm.distance && (
          <View style={styles.distanceContainer}>
            <Text style={styles.distanceLabel}>Distance</Text>
            <Text style={styles.distance}>{atm.distance.toFixed(1)} miles</Text>
          </View>
        )}

        {/* Address */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Location</Text>
          <Text style={styles.address}>{atm.address}</Text>
          <Text style={styles.cityState}>
            {atm.city}, {atm.state} {atm.zip}
          </Text>
        </View>

        {/* Hours */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Hours</Text>
          <View style={styles.hoursContainer}>
            {atm.is24Hours && <Text style={styles.badge24}>24/7</Text>}
            <Text style={styles.hours}>{atm.hours}</Text>
          </View>
        </View>

        {/* Fees & Limits */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Fees & Limits</Text>

          <View style={styles.detailGrid}>
            <View style={styles.detailCard}>
              <Text style={styles.detailLabel}>Fee</Text>
              <Text style={styles.detailValue}>{atm.fee}%</Text>
            </View>

            <View style={styles.detailCard}>
              <Text style={styles.detailLabel}>Daily Limit</Text>
              <Text style={styles.detailValue}>${atm.dailyLimit.toLocaleString()}</Text>
            </View>

            <View style={styles.detailCard}>
              <Text style={styles.detailLabel}>Rating</Text>
              <View style={styles.ratingContainer}>
                <Text style={styles.star}>★</Text>
                <Text style={styles.detailValue}>{atm.rating.toFixed(1)}</Text>
              </View>
            </View>

            <View style={styles.detailCard}>
              <Text style={styles.detailLabel}>Reviews</Text>
              <Text style={styles.detailValue}>{atm.reviewCount}</Text>
            </View>
          </View>
        </View>

        {/* Features */}
        {atm.features && atm.features.length > 0 && (
          <View style={styles.section}>
            <Text style={styles.sectionTitle}>Features</Text>
            <View style={styles.featuresContainer}>
              {atm.features.map((feature, index) => (
                <View key={index} style={styles.featureBadge}>
                  <Text style={styles.featureText}>
                    {feature.replace(/([A-Z])/g, ' $1').trim()}
                  </Text>
                </View>
              ))}
            </View>
          </View>
        )}
      </ScrollView>

      {/* Action Buttons */}
      <View style={styles.actionContainer}>
        <TouchableOpacity
          style={styles.directionsButton}
          onPress={handleGetDirections}
        >
          <Text style={styles.directionsButtonText}>Get Directions</Text>
        </TouchableOpacity>

        <TouchableOpacity style={styles.withdrawButton} onPress={handleWithdrawCash}>
          <Text style={styles.withdrawButtonText}>Withdraw Cash</Text>
        </TouchableOpacity>
      </View>
    </SafeAreaView>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    backgroundColor: '#fff',
    padding: 20,
    flexDirection: 'row',
    alignItems: 'center',
  },
  backButton: {
    marginRight: 16,
  },
  backButtonText: {
    fontSize: 28,
    color: '#1a1a1a',
  },
  headerContent: {
    flex: 1,
  },
  name: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 8,
  },
  partnerBadge: {
    alignSelf: 'flex-start',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 8,
  },
  partnerText: {
    color: '#fff',
    fontSize: 14,
    fontWeight: '600',
  },
  distanceContainer: {
    backgroundColor: '#4CAF50',
    padding: 20,
    alignItems: 'center',
  },
  distanceLabel: {
    color: '#fff',
    fontSize: 14,
    marginBottom: 4,
  },
  distance: {
    color: '#fff',
    fontSize: 32,
    fontWeight: 'bold',
  },
  section: {
    backgroundColor: '#fff',
    padding: 20,
    marginTop: 12,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 12,
  },
  address: {
    fontSize: 16,
    color: '#333',
    marginBottom: 4,
  },
  cityState: {
    fontSize: 16,
    color: '#666',
  },
  hoursContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  badge24: {
    backgroundColor: '#4CAF50',
    color: '#fff',
    paddingHorizontal: 12,
    paddingVertical: 4,
    borderRadius: 12,
    fontSize: 14,
    fontWeight: 'bold',
    marginRight: 12,
  },
  hours: {
    fontSize: 16,
    color: '#333',
  },
  detailGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 12,
  },
  detailCard: {
    flex: 1,
    minWidth: '45%',
    backgroundColor: '#f5f5f5',
    padding: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  detailLabel: {
    fontSize: 12,
    color: '#666',
    marginBottom: 8,
  },
  detailValue: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a1a1a',
  },
  ratingContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  star: {
    color: '#FFB300',
    fontSize: 20,
    marginRight: 4,
  },
  featuresContainer: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  featureBadge: {
    backgroundColor: '#E3F2FD',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 16,
  },
  featureText: {
    color: '#1976D2',
    fontSize: 14,
    fontWeight: '500',
  },
  actionContainer: {
    flexDirection: 'row',
    padding: 20,
    backgroundColor: '#fff',
    gap: 12,
    borderTopWidth: 1,
    borderTopColor: '#f0f0f0',
  },
  directionsButton: {
    flex: 1,
    backgroundColor: '#fff',
    borderWidth: 2,
    borderColor: '#4CAF50',
    paddingVertical: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  directionsButtonText: {
    color: '#4CAF50',
    fontSize: 16,
    fontWeight: 'bold',
  },
  withdrawButton: {
    flex: 1,
    backgroundColor: '#4CAF50',
    paddingVertical: 16,
    borderRadius: 12,
    alignItems: 'center',
  },
  withdrawButtonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: 'bold',
  },
});

export default ATMDetailScreen;
