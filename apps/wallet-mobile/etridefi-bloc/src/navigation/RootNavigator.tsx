import React from 'react';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {NavigationContainer} from '@react-navigation/native';
import Icon from 'react-native-vector-icons/Feather';
import {colors} from '@/theme';

// Import navigators matching iOS Swift app structure
import HomeNavigator from './HomeNavigator';
import TokensNavigator from './TokensNavigator';
import DeFiNavigator from './DeFiNavigator';
import ConsensusNavigator from './ConsensusNavigator';
import SettingsNavigator from './SettingsNavigator';

const Tab = createBottomTabNavigator();

/**
 * RootNavigator - Main tab navigation matching iOS Swift app exactly
 *
 * iOS Swift Tabs:
 * 1. Home - Balance card, quick actions (Send/Receive/Swap/Bridge), holdings, activity
 * 2. Tokens - Token list with categories (All, Native, wETR, Bridged, Stablecoins)
 * 3. DeFi - Virtual Card (Coming Soon), ATM Locator, Crypto Loans (Soon), Yield Farming (Soon)
 * 4. Consënsus - Governance proposals, directors, staking, rewards
 * 5. Settings - Wallet, security, network, preferences, about
 */
export default function RootNavigator() {
  return (
    <NavigationContainer>
      <Tab.Navigator
        screenOptions={{
          headerShown: false,
          tabBarActiveTintColor: colors.primary,
          tabBarInactiveTintColor: colors.textSecondary,
          tabBarStyle: {
            backgroundColor: colors.backgroundLight,
            borderTopColor: colors.border,
            borderTopWidth: 1,
            height: 60,
            paddingBottom: 8,
            paddingTop: 8,
          },
          tabBarLabelStyle: {
            fontSize: 11,
            fontWeight: '600',
          },
        }}>
        <Tab.Screen
          name="Home"
          component={HomeNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="home" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="Tokens"
          component={TokensNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="disc" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="DeFi"
          component={DeFiNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="credit-card" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="Consënsus"
          component={ConsensusNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="activity" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="Settings"
          component={SettingsNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="settings" size={size} color={color} />
            ),
          }}
        />
      </Tab.Navigator>
    </NavigationContainer>
  );
}
