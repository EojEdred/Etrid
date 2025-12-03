import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

import DeFiScreen from '@/screens/DeFiScreen';
import SwapScreen from '@/screens/SwapScreen';

const Stack = createNativeStackNavigator();

export default function DeFiNavigator() {
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
        name="DeFiMain"
        component={DeFiScreen}
        options={{
          title: 'DeFi',
          headerShown: true,
        }}
      />
      <Stack.Screen
        name="Swap"
        component={SwapScreen}
        options={{
          title: 'Swap',
        }}
      />
    </Stack.Navigator>
  );
}
