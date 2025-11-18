import React from 'react';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { colors } from '../theme/theme';

// Placeholder screens - to be implemented
import HomeScreen from '../screens/HomeScreen';
import AccountsScreen from '../screens/AccountsScreen';
import GovernanceScreen from '../screens/GovernanceScreen';
import PortfolioScreen from '../screens/PortfolioScreen';
import SettingsScreen from '../screens/SettingsScreen';

const Stack = createNativeStackNavigator();
const Tab = createBottomTabNavigator();

function HomeTabs() {
  return (
    <Tab.Navigator
      screenOptions={{
        tabBarActiveTintColor: colors.primary,
        tabBarInactiveTintColor: colors.gray400,
        tabBarStyle: {
          borderTopWidth: 1,
          borderTopColor: colors.gray200,
        },
      }}
    >
      <Tab.Screen
        name="Home"
        component={HomeScreen}
        options={{
          tabBarLabel: 'Home',
          tabBarIcon: ({ color }) => '🏠',
          headerShown: false,
        }}
      />
      <Tab.Screen
        name="Accounts"
        component={AccountsScreen}
        options={{
          tabBarLabel: 'Accounts',
          tabBarIcon: ({ color }) => '💰',
        }}
      />
      <Tab.Screen
        name="Governance"
        component={GovernanceScreen}
        options={{
          tabBarLabel: 'Vote',
          tabBarIcon: ({ color }) => '🗳️',
        }}
      />
      <Tab.Screen
        name="Portfolio"
        component={PortfolioScreen}
        options={{
          tabBarLabel: 'Portfolio',
          tabBarIcon: ({ color }) => '📊',
        }}
      />
      <Tab.Screen
        name="Settings"
        component={SettingsScreen}
        options={{
          tabBarLabel: 'Settings',
          tabBarIcon: ({ color }) => '⚙️',
        }}
      />
    </Tab.Navigator>
  );
}

export function RootNavigator() {
  return (
    <Stack.Navigator>
      <Stack.Screen
        name="Main"
        component={HomeTabs}
        options={{ headerShown: false }}
      />
    </Stack.Navigator>
  );
}
