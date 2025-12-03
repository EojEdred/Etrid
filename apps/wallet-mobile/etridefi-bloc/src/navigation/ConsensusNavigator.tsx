import React from 'react';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import {colors} from '@/theme';

import ConsensusScreen from '@/screens/ConsensusScreen';
import ProposalDetailScreen from '@/screens/ProposalDetailScreen';
import SubmitProposalScreen from '@/screens/SubmitProposalScreen';
import StakingScreen from '@/screens/StakingScreen';

const Stack = createNativeStackNavigator();

export default function ConsensusNavigator() {
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
        name="ConsensusMain"
        component={ConsensusScreen}
        options={{
          title: 'Consënsus',
          headerShown: true,
        }}
      />
      <Stack.Screen
        name="ProposalDetail"
        component={ProposalDetailScreen}
        options={{
          title: 'Proposal',
        }}
      />
      <Stack.Screen
        name="SubmitProposal"
        component={SubmitProposalScreen}
        options={{
          title: 'Submit Proposal',
          presentation: 'modal',
        }}
      />
      <Stack.Screen
        name="Staking"
        component={StakingScreen}
        options={{
          title: 'Staking',
        }}
      />
    </Stack.Navigator>
  );
}
