import React from 'react';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {NavigationContainer} from '@react-navigation/native';
import Icon from 'react-native-vector-icons/Feather';
import {colors} from '@/theme';

// Import navigators
import HomeNavigator from './HomeNavigator';
import SocialNavigator from './SocialNavigator';
import NFTNavigator from './NFTNavigator';
import TradeNavigator from './TradeNavigator';
import MoreNavigator from './MoreNavigator';

const Tab = createBottomTabNavigator();

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
            fontSize: 12,
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
          name="Social"
          component={SocialNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="users" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="NFTs"
          component={NFTNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="image" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="Trade"
          component={TradeNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="trending-up" size={size} color={color} />
            ),
          }}
        />
        <Tab.Screen
          name="More"
          component={MoreNavigator}
          options={{
            tabBarIcon: ({color, size}) => (
              <Icon name="menu" size={size} color={color} />
            ),
          }}
        />
      </Tab.Navigator>
    </NavigationContainer>
  );
}
