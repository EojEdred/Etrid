import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

// Import screens
import TradingScreen from '@/screens/TradingScreen';
import SwapScreen from '@/screens/SwapScreen';
import LendingScreen from '@/screens/LendingScreen';

const Stack = createNativeStackNavigator();

export default function TradeNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: {
          backgroundColor: colors.background,
        },
      }}>
      <Stack.Screen name="TradingMain" component={TradingScreen} />
      <Stack.Screen name="Swap" component={SwapScreen} />
      <Stack.Screen name="Lending" component={LendingScreen} />
    </Stack.Navigator>
  );
}
