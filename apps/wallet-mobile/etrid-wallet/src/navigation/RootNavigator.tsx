import React from 'react';
import { ActivityIndicator, View, StyleSheet } from 'react-native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { useAuth } from '../contexts/AuthContext';
import { colors } from '../theme/theme';

// Auth Screens
import { WelcomeScreen } from '../screens/auth/WelcomeScreen';
import { CreateWalletScreen } from '../screens/auth/CreateWalletScreen';
import { ImportWalletScreen } from '../screens/auth/ImportWalletScreen';
import { BackupPhraseScreen } from '../screens/auth/BackupPhraseScreen';
import { VerifyPhraseScreen } from '../screens/auth/VerifyPhraseScreen';
import { BiometricSetupScreen } from '../screens/auth/BiometricSetupScreen';

// Main Screens
import HomeScreen from '../screens/HomeScreen';
import AccountsScreen from '../screens/AccountsScreen';
import GovernanceScreen from '../screens/GovernanceScreen';
import PortfolioScreen from '../screens/PortfolioScreen';
import SettingsScreen from '../screens/SettingsScreen';

// Wallet Screens
import { SendScreen } from '../screens/wallet/SendScreen';
import { ReceiveScreen } from '../screens/wallet/ReceiveScreen';
import { TransactionHistoryScreen } from '../screens/wallet/TransactionHistoryScreen';

const Stack = createNativeStackNavigator();
const Tab = createBottomTabNavigator();
const AuthStack = createNativeStackNavigator();
const MainStack = createNativeStackNavigator();

/**
 * Auth Stack - Onboarding flow
 */
function AuthNavigator() {
  return (
    <AuthStack.Navigator
      screenOptions={{
        headerShown: false,
        animation: 'slide_from_right',
      }}
    >
      <AuthStack.Screen name="Welcome" component={WelcomeScreen} />
      <AuthStack.Screen name="CreateWallet" component={CreateWalletScreen} />
      <AuthStack.Screen name="ImportWallet" component={ImportWalletScreen} />
      <AuthStack.Screen name="BackupPhrase" component={BackupPhraseScreen} />
      <AuthStack.Screen name="VerifyPhrase" component={VerifyPhraseScreen} />
      <AuthStack.Screen name="BiometricSetup" component={BiometricSetupScreen} />
    </AuthStack.Navigator>
  );
}

/**
 * Main Tabs - Primary navigation
 */
function MainTabs() {
  return (
    <Tab.Navigator
      screenOptions={{
        tabBarActiveTintColor: colors.primary,
        tabBarInactiveTintColor: colors.gray400,
        tabBarStyle: {
          borderTopWidth: 1,
          borderTopColor: colors.gray200,
          paddingBottom: 5,
          paddingTop: 5,
          height: 60,
        },
        headerShown: false,
      }}
    >
      <Tab.Screen
        name="HomeTab"
        component={HomeScreen}
        options={{
          tabBarLabel: 'Home',
          tabBarIcon: ({ color }) => '🏠',
        }}
      />
      <Tab.Screen
        name="PortfolioTab"
        component={PortfolioScreen}
        options={{
          tabBarLabel: 'Portfolio',
          tabBarIcon: ({ color }) => '📊',
        }}
      />
      <Tab.Screen
        name="AccountsTab"
        component={AccountsScreen}
        options={{
          tabBarLabel: 'Accounts',
          tabBarIcon: ({ color }) => '💰',
        }}
      />
      <Tab.Screen
        name="GovernanceTab"
        component={GovernanceScreen}
        options={{
          tabBarLabel: 'Vote',
          tabBarIcon: ({ color }) => '🗳️',
        }}
      />
      <Tab.Screen
        name="SettingsTab"
        component={SettingsScreen}
        options={{
          tabBarLabel: 'Settings',
          tabBarIcon: ({ color }) => '⚙️',
        }}
      />
    </Tab.Navigator>
  );
}

/**
 * Main Stack - Includes tabs + modal screens
 */
function MainNavigator() {
  return (
    <MainStack.Navigator
      screenOptions={{
        headerShown: false,
      }}
    >
      <MainStack.Screen name="MainTabs" component={MainTabs} />

      {/* Modal Screens */}
      <MainStack.Group
        screenOptions={{
          presentation: 'modal',
          animation: 'slide_from_bottom',
        }}
      >
        <MainStack.Screen name="Send" component={SendScreen} />
        <MainStack.Screen name="Receive" component={ReceiveScreen} />
        <MainStack.Screen name="TransactionHistory" component={TransactionHistoryScreen} />
      </MainStack.Group>
    </MainStack.Navigator>
  );
}

/**
 * Root Navigator - Handles auth state
 */
export function RootNavigator() {
  const { isAuthenticated, isLoading } = useAuth();

  // Show loading spinner while checking auth state
  if (isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={colors.primary} />
      </View>
    );
  }

  return (
    <Stack.Navigator screenOptions={{ headerShown: false }}>
      {isAuthenticated ? (
        <Stack.Screen name="Main" component={MainNavigator} />
      ) : (
        <Stack.Screen name="Auth" component={AuthNavigator} />
      )}
    </Stack.Navigator>
  );
}

const styles = StyleSheet.create({
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: colors.background,
  },
});
