/**
 * ATMCard - ATM info card for list view
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { ATMLocation } from '../../types/atm.types';

interface ATMCardProps {
  atm: ATMLocation;
  onPress: (atm: ATMLocation) => void;
}

const ATMCard: React.FC<ATMCardProps> = ({ atm, onPress }) => {
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
    <TouchableOpacity
      style={styles.card}
      onPress={() => onPress(atm)}
      activeOpacity={0.7}
    >
      <View style={styles.header}>
        <View style={styles.nameContainer}>
          <Text style={styles.name}>{atm.name}</Text>
          <View
            style={[styles.partnerBadge, { backgroundColor: getPartnerColor(atm.partner) }]}
          >
            <Text style={styles.partnerText}>{atm.partner}</Text>
          </View>
        </View>
        {atm.distance && (
          <Text style={styles.distance}>{atm.distance.toFixed(1)} mi</Text>
        )}
      </View>

      <Text style={styles.address} numberOfLines={2}>
        {atm.address}
      </Text>

      <View style={styles.details}>
        <View style={styles.detailItem}>
          <Text style={styles.detailLabel}>Hours</Text>
          <Text style={styles.detailValue}>
            {atm.is24Hours ? '24/7' : atm.hours}
          </Text>
        </View>

        <View style={styles.detailItem}>
          <Text style={styles.detailLabel}>Fee</Text>
          <Text style={styles.detailValue}>{atm.fee}%</Text>
        </View>

        <View style={styles.detailItem}>
          <Text style={styles.detailLabel}>Daily Limit</Text>
          <Text style={styles.detailValue}>${atm.dailyLimit.toLocaleString()}</Text>
        </View>

        <View style={styles.detailItem}>
          <Text style={styles.detailLabel}>Rating</Text>
          <View style={styles.ratingContainer}>
            <Text style={styles.star}>★</Text>
            <Text style={styles.detailValue}>{atm.rating.toFixed(1)}</Text>
          </View>
        </View>
      </View>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  card: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginHorizontal: 16,
    marginVertical: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: 8,
  },
  nameContainer: {
    flex: 1,
    marginRight: 12,
  },
  name: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#1a1a1a',
    marginBottom: 6,
  },
  partnerBadge: {
    alignSelf: 'flex-start',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 6,
  },
  partnerText: {
    color: '#fff',
    fontSize: 12,
    fontWeight: '600',
  },
  distance: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#4CAF50',
  },
  address: {
    fontSize: 14,
    color: '#666',
    marginBottom: 12,
    lineHeight: 20,
  },
  details: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingTop: 12,
    borderTopWidth: 1,
    borderTopColor: '#f0f0f0',
  },
  detailItem: {
    alignItems: 'center',
  },
  detailLabel: {
    fontSize: 11,
    color: '#999',
    marginBottom: 4,
  },
  detailValue: {
    fontSize: 14,
    fontWeight: '600',
    color: '#1a1a1a',
  },
  ratingContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  star: {
    color: '#FFB300',
    fontSize: 14,
    marginRight: 2,
  },
});

export default ATMCard;
