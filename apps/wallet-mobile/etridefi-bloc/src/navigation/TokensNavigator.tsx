import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

import TokensScreen from '@/screens/TokensScreen';
import TokenDetailScreen from '@/screens/TokenDetailScreen';
import BuyTokenScreen from '@/screens/BuyTokenScreen';
import BridgeScreen from '@/screens/BridgeScreen';

const Stack = createNativeStackNavigator();

export default function TokensNavigator() {
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
        name="TokensMain"
        component={TokensScreen}
        options={{
          title: 'Tokens',
          headerShown: true,
        }}
      />
      <Stack.Screen
        name="TokenDetail"
        component={TokenDetailScreen}
        options={{
          title: 'Token Details',
        }}
      />
      <Stack.Screen
        name="BuyToken"
        component={BuyTokenScreen}
        options={{
          title: 'Buy Token',
          presentation: 'modal',
        }}
      />
      <Stack.Screen
        name="Bridge"
        component={BridgeScreen}
        options={{
          title: 'Bridge',
          presentation: 'modal',
        }}
      />
    </Stack.Navigator>
  );
}
