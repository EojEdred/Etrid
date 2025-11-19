/**
 * Social Hub
 * Main social features hub with quick access to all social functionality
 */

'use client';

import { useState } from 'react';
import {
  Users,
  AtSign,
  Receipt,
  Shield,
  Activity,
  ArrowRight,
  TrendingUp,
  DollarSign,
  UserPlus,
} from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { UsernameSetupScreen } from '@/components/username-setup-screen';
import { ContactsScreen } from '@/components/contacts-screen';
import { SocialFeedScreen } from '@/components/social-feed-screen';
import { BillSplitScreen } from '@/components/bill-split-screen';
import { SocialRecoveryScreen } from '@/components/social-recovery-screen';
import { useContacts } from '@/hooks/useContacts';
import { useBillSplit } from '@/hooks/useBillSplit';
import { useSocialRecovery } from '@/hooks/useSocialRecovery';
import { useWallet } from '@/lib/polkadot/useWallet';
import { BottomNav } from './bottom-nav';

interface SocialHubProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

export function SocialHub({ activeTab, onTabChange }: SocialHubProps) {
  const { selectedAccount } = useWallet();
  const userId = selectedAccount?.address || '';

  const [currentScreen, setCurrentScreen] = useState<
    'hub' | 'username' | 'contacts' | 'feed' | 'billsplit' | 'recovery'
  >('hub');

  const { contacts } = useContacts(userId);
  const { summary, owedSplits } = useBillSplit(userId);
  const { activeGuardians, pendingApprovals } = useSocialRecovery(userId);

  // Render specific screens
  if (currentScreen === 'username') {
    return <UsernameSetupScreen onBack={() => setCurrentScreen('hub')} />;
  }

  if (currentScreen === 'contacts') {
    return <ContactsScreen onBack={() => setCurrentScreen('hub')} />;
  }

  if (currentScreen === 'feed') {
    return <SocialFeedScreen onBack={() => setCurrentScreen('hub')} />;
  }

  if (currentScreen === 'billsplit') {
    return <BillSplitScreen onBack={() => setCurrentScreen('hub')} />;
  }

  if (currentScreen === 'recovery') {
    return <SocialRecoveryScreen onBack={() => setCurrentScreen('hub')} />;
  }

  // Main Hub Screen
  return (
    <div className="min-h-screen pb-24">
      {/* Header */}
      <header className="p-6">
        <div className="flex items-center gap-3 mb-2">
          <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center">
            <Users className="w-5 h-5 text-accent" />
          </div>
          <div>
            <h1 className="text-2xl font-bold">Social</h1>
            <p className="text-sm text-muted-foreground">Connect, share, and collaborate</p>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="px-6 space-y-6">
        {/* Quick Stats */}
        <div className="grid grid-cols-3 gap-3">
          <Card className="bg-accent/5 border-accent/20">
            <CardContent className="pt-6 text-center">
              <Users className="w-6 h-6 mx-auto text-accent mb-2" />
              <p className="text-2xl font-bold text-foreground">{contacts.length}</p>
              <p className="text-xs text-muted-foreground">Contacts</p>
            </CardContent>
          </Card>

          <Card className="bg-accent/5 border-accent/20">
            <CardContent className="pt-6 text-center">
              <DollarSign className="w-6 h-6 mx-auto text-accent mb-2" />
              <p className="text-2xl font-bold text-foreground">
                {summary?.totalOwed.toFixed(0) || 0}
              </p>
              <p className="text-xs text-muted-foreground">ÉTR Owed</p>
            </CardContent>
          </Card>

          <Card className="bg-accent/5 border-accent/20">
            <CardContent className="pt-6 text-center">
              <Shield className="w-6 h-6 mx-auto text-accent mb-2" />
              <p className="text-2xl font-bold text-foreground">{activeGuardians.length}</p>
              <p className="text-xs text-muted-foreground">Guardians</p>
            </CardContent>
          </Card>
        </div>

        {/* Username Setup */}
        <Card className="bg-gradient-to-br from-accent/20 to-accent/5 border-accent/30 cursor-pointer hover:bg-accent/10 transition-colors">
          <CardHeader className="pb-3" onClick={() => setCurrentScreen('username')}>
            <div className="flex items-start justify-between">
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center flex-shrink-0">
                  <AtSign className="w-5 h-5 text-accent" />
                </div>
                <div>
                  <CardTitle>Claim Your Username</CardTitle>
                  <CardDescription className="mt-1">
                    Get your unique @username.etrid identifier
                  </CardDescription>
                </div>
              </div>
              <ArrowRight className="w-5 h-5 text-accent" />
            </div>
          </CardHeader>
        </Card>

        {/* Primary Features */}
        <div className="space-y-3">
          {/* Contacts */}
          <Card className="cursor-pointer hover:bg-accent/5 transition-colors">
            <CardHeader className="pb-3" onClick={() => setCurrentScreen('contacts')}>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-blue-500/10 flex items-center justify-center">
                    <Users className="w-5 h-5 text-blue-500" />
                  </div>
                  <div>
                    <CardTitle className="text-base">Contacts</CardTitle>
                    <CardDescription>
                      {contacts.length} {contacts.length === 1 ? 'contact' : 'contacts'}
                    </CardDescription>
                  </div>
                </div>
                <ArrowRight className="w-5 h-5 text-muted-foreground" />
              </div>
            </CardHeader>
          </Card>

          {/* Bill Split */}
          <Card className="cursor-pointer hover:bg-accent/5 transition-colors">
            <CardHeader className="pb-3" onClick={() => setCurrentScreen('billsplit')}>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-green-500/10 flex items-center justify-center">
                    <Receipt className="w-5 h-5 text-green-500" />
                  </div>
                  <div>
                    <CardTitle className="text-base">Bill Splitting</CardTitle>
                    <CardDescription>
                      {owedSplits.length > 0 ? `${owedSplits.length} pending` : 'No pending bills'}
                    </CardDescription>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {owedSplits.length > 0 && (
                    <Badge variant="destructive">{owedSplits.length}</Badge>
                  )}
                  <ArrowRight className="w-5 h-5 text-muted-foreground" />
                </div>
              </div>
            </CardHeader>
          </Card>

          {/* Social Feed */}
          <Card className="cursor-pointer hover:bg-accent/5 transition-colors">
            <CardHeader className="pb-3" onClick={() => setCurrentScreen('feed')}>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-purple-500/10 flex items-center justify-center">
                    <Activity className="w-5 h-5 text-purple-500" />
                  </div>
                  <div>
                    <CardTitle className="text-base">Social Feed</CardTitle>
                    <CardDescription>Activity & achievements</CardDescription>
                  </div>
                </div>
                <ArrowRight className="w-5 h-5 text-muted-foreground" />
              </div>
            </CardHeader>
          </Card>

          {/* Social Recovery */}
          <Card className="cursor-pointer hover:bg-accent/5 transition-colors">
            <CardHeader className="pb-3" onClick={() => setCurrentScreen('recovery')}>
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-full bg-orange-500/10 flex items-center justify-center">
                    <Shield className="w-5 h-5 text-orange-500" />
                  </div>
                  <div>
                    <CardTitle className="text-base">Social Recovery</CardTitle>
                    <CardDescription>
                      {activeGuardians.length} active{' '}
                      {activeGuardians.length === 1 ? 'guardian' : 'guardians'}
                    </CardDescription>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  {pendingApprovals.length > 0 && (
                    <Badge variant="default">{pendingApprovals.length}</Badge>
                  )}
                  <ArrowRight className="w-5 h-5 text-muted-foreground" />
                </div>
              </div>
            </CardHeader>
          </Card>
        </div>

        {/* Info Card */}
        <Card className="bg-muted/50">
          <CardHeader>
            <CardTitle className="text-sm">Why Use Social Features?</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm text-muted-foreground">
            <p>• Share payments easily with @usernames</p>
            <p>• Split bills with friends and track payments</p>
            <p>• Recover your account with trusted guardians</p>
            <p>• See community activity and milestones</p>
          </CardContent>
        </Card>
      </main>

      {/* Bottom Navigation */}
      <BottomNav activeTab={activeTab} onTabChange={onTabChange} />
    </div>
  );
}
