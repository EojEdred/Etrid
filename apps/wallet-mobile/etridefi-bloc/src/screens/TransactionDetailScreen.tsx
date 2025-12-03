import React from 'react';
import {View, Text, StyleSheet, TouchableOpacity} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

export default function TransactionDetailScreen({navigation}: any) {
  return (
    <LinearGradient colors={[colors.gradientStart, colors.gradientEnd]} style={styles.container}>
      <View style={styles.header}>
        <TouchableOpacity onPress={() => navigation?.goBack()}>
          <Icon name="arrow-left" size={24} color={colors.text} />
        </TouchableOpacity>
        <Text style={styles.headerTitle}>TransactionDetail</Text>
        <View style={{width: 24}} />
      </View>
      <View style={styles.content}>
        <Text style={styles.text}>TransactionDetail screen coming soon...</Text>
      </View>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {flex: 1},
  header: {flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', padding: theme.spacing.md, paddingTop: 50},
  headerTitle: {fontSize: theme.fontSize.xl, fontWeight: theme.fontWeight.bold, color: colors.text},
  content: {flex: 1, justifyContent: 'center', alignItems: 'center', padding: theme.spacing.md},
  text: {color: colors.textSecondary, textAlign: 'center'},
});
