import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

// Import screens
import MoreScreen from '@/screens/MoreScreen';
import AUBloccardScreen from '@/screens/AUBloccardScreen';
import SavingsGoalsScreen from '@/screens/SavingsGoalsScreen';
import FiatRampScreen from '@/screens/FiatRampScreen';
import SecurityScreen from '@/screens/SecurityScreen';
import PrivacyScreen from '@/screens/PrivacyScreen';

const Stack = createNativeStackNavigator();

export default function MoreNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: {
          backgroundColor: colors.background,
        },
      }}>
      <Stack.Screen name="MoreMain" component={MoreScreen} />
      <Stack.Screen name="AUBloccard" component={AUBloccardScreen} />
      <Stack.Screen name="SavingsGoals" component={SavingsGoalsScreen} />
      <Stack.Screen name="FiatRamp" component={FiatRampScreen} />
      <Stack.Screen name="Security" component={SecurityScreen} />
      <Stack.Screen name="Privacy" component={PrivacyScreen} />
    </Stack.Navigator>
  );
}
