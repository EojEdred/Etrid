import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

// Import screens
import NFTGalleryScreen from '@/screens/NFTGalleryScreen';
import NFTDetailScreen from '@/screens/NFTDetailScreen';
import NFTMarketplaceScreen from '@/screens/NFTMarketplaceScreen';

const Stack = createNativeStackNavigator();

export default function NFTNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: {
          backgroundColor: colors.background,
        },
      }}>
      <Stack.Screen name="NFTGalleryMain" component={NFTGalleryScreen} />
      <Stack.Screen name="NFTDetail" component={NFTDetailScreen} />
      <Stack.Screen name="NFTMarketplace" component={NFTMarketplaceScreen} />
    </Stack.Navigator>
  );
}
