import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

// Import screens
import HomeScreen from '@/screens/HomeScreen';
import SendScreen from '@/screens/SendScreen';
import ReceiveScreen from '@/screens/ReceiveScreen';
import TransactionDetailScreen from '@/screens/TransactionDetailScreen';
import NotificationsScreen from '@/screens/NotificationsScreen';
import SettingsScreen from '@/screens/SettingsScreen';

const Stack = createNativeStackNavigator();

export default function HomeNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: {
          backgroundColor: colors.background,
        },
      }}>
      <Stack.Screen name="HomeMain" component={HomeScreen} />
      <Stack.Screen
        name="Send"
        component={SendScreen}
        options={{
          presentation: 'modal',
        }}
      />
      <Stack.Screen
        name="Receive"
        component={ReceiveScreen}
        options={{
          presentation: 'modal',
        }}
      />
      <Stack.Screen name="TransactionDetail" component={TransactionDetailScreen} />
      <Stack.Screen name="Notifications" component={NotificationsScreen} />
      <Stack.Screen name="Settings" component={SettingsScreen} />
    </Stack.Navigator>
  );
}
