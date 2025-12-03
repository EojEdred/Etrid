import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

// Import screens
import SocialScreen from '@/screens/SocialScreen';
import ContactsScreen from '@/screens/ContactsScreen';
import BillSplitScreen from '@/screens/BillSplitScreen';

const Stack = createNativeStackNavigator();

export default function SocialNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: {
          backgroundColor: colors.background,
        },
      }}>
      <Stack.Screen name="SocialMain" component={SocialScreen} />
      <Stack.Screen name="Contacts" component={ContactsScreen} />
      <Stack.Screen name="BillSplit" component={BillSplitScreen} />
    </Stack.Navigator>
  );
}
