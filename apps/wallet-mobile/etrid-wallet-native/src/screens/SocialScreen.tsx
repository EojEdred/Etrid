import React from 'react';
import {View, Text, StyleSheet} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import {colors, theme} from '@/theme';

export default function SocialScreen() {
  return (
    <LinearGradient colors={[colors.gradientStart, colors.gradientEnd]} style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Social</Text>
      </View>
      <View style={styles.content}>
        <Text style={styles.text}>Social features coming soon...</Text>
      </View>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {flex: 1},
  header: {padding: theme.spacing.md, paddingTop: 50},
  headerTitle: {fontSize: theme.fontSize.xxl, fontWeight: theme.fontWeight.bold, color: colors.text},
  content: {flex: 1, justifyContent: 'center', alignItems: 'center'},
  text: {color: colors.textSecondary},
});
