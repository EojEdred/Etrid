import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

import SettingsScreen from '@/screens/SettingsScreen';
import SecurityScreen from '@/screens/SecurityScreen';
import PrivacyScreen from '@/screens/PrivacyScreen';
import NotificationsScreen from '@/screens/NotificationsScreen';

const Stack = createNativeStackNavigator();

export default function SettingsNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerStyle: {
          backgroundColor: colors.background,
        },
        headerTintColor: colors.text,
        headerTitleStyle: {
          fontWeight: '600',
        },
      }}>
      <Stack.Screen
        name="SettingsMain"
        component={SettingsScreen}
        options={{
          title: 'Settings',
          headerShown: true,
        }}
      />
      <Stack.Screen
        name="Security"
        component={SecurityScreen}
        options={{
          title: 'Security Settings',
        }}
      />
      <Stack.Screen
        name="Privacy"
        component={PrivacyScreen}
        options={{
          title: 'Privacy Policy',
        }}
      />
      <Stack.Screen
        name="Notifications"
        component={NotificationsScreen}
        options={{
          title: 'Notifications',
        }}
      />
    </Stack.Navigator>
  );
}
