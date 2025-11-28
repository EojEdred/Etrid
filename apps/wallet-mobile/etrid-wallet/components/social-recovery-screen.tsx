/**
 * Social Recovery Screen
 * Manage guardians and recovery process for account security
 */

'use client';

import { useState } from 'react';
import { ArrowLeft, Plus, Shield, AlertTriangle, Info, Clock } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from '@/components/ui/alert';
import { Slider } from '@/components/ui/slider';
import { GuardianCard } from '@/components/social/guardian-card';
import { useSocialRecovery } from '@/hooks/useSocialRecovery';
import { useWallet } from '@/lib/polkadot/useWallet';
import { formatDistanceToNow } from 'date-fns';

interface SocialRecoveryScreenProps {
  onBack: () => void;
}

export function SocialRecoveryScreen({ onBack }: SocialRecoveryScreenProps) {
  const { selectedAccount } = useWallet();
  const walletAddress = selectedAccount?.address || '';

  const {
    guardians,
    activeGuardians,
    pendingGuardians,
    config,
    activeRecovery,
    pendingApprovals,
    isLoading,
    addGuardian,
    removeGuardian,
    setThreshold,
    resendInvitation,
    initiateRecovery,
    approveRecovery,
    cancelRecovery,
    getRecommendedThreshold,
  } = useSocialRecovery(walletAddress);

  const [isAddDialogOpen, setIsAddDialogOpen] = useState(false);
  const [isRecoveryDialogOpen, setIsRecoveryDialogOpen] = useState(false);
  const [newGuardianAddress, setNewGuardianAddress] = useState('');
  const [newGuardianUsername, setNewGuardianUsername] = useState('');
  const [newDeviceId, setNewDeviceId] = useState('');

  const handleAddGuardian = async () => {
    try {
      await addGuardian(newGuardianAddress, newGuardianUsername);
      setIsAddDialogOpen(false);
      setNewGuardianAddress('');
      setNewGuardianUsername('');
    } catch (error) {
      console.error('Failed to add guardian:', error);
    }
  };

  const handleRemoveGuardian = async (guardian: any) => {
    try {
      await removeGuardian(guardian.id);
    } catch (error) {
      console.error('Failed to remove guardian:', error);
    }
  };

  const handleThresholdChange = async (value: number[]) => {
    try {
      await setThreshold(value[0]);
    } catch (error) {
      console.error('Failed to set threshold:', error);
    }
  };

  const handleInitiateRecovery = async () => {
    try {
      await initiateRecovery(newDeviceId);
      setIsRecoveryDialogOpen(false);
      setNewDeviceId('');
    } catch (error) {
      console.error('Failed to initiate recovery:', error);
    }
  };

  const recommendedThreshold = getRecommendedThreshold(activeGuardians.length);

  return (
    <div className="min-h-screen bg-background pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 glass-strong border-b border-border">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div className="flex-1">
            <h1 className="text-xl font-bold">Social Recovery</h1>
            <p className="text-sm text-muted-foreground">
              Secure your account with trusted guardians
            </p>
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="p-6 space-y-6 max-w-2xl mx-auto">
        {/* Info Alert */}
        <Alert>
          <Info className="w-4 h-4" />
          <AlertTitle>What is Social Recovery?</AlertTitle>
          <AlertDescription>
            Add trusted friends or family as guardians who can help you recover your account if you
            lose access. A minimum number of guardians must approve any recovery attempt.
          </AlertDescription>
        </Alert>

        {/* Active Recovery Warning */}
        {activeRecovery && (
          <Alert variant="destructive">
            <AlertTriangle className="w-4 h-4" />
            <AlertTitle>Active Recovery Process</AlertTitle>
            <AlertDescription>
              A recovery process is currently active. It will complete in{' '}
              {activeRecovery.completesAt &&
                formatDistanceToNow(activeRecovery.completesAt, { addSuffix: true })}
              . {activeRecovery.approvals.length}/{activeRecovery.requiredApprovals} guardians have
              approved.
              <Button
                variant="outline"
                size="sm"
                className="mt-2"
                onClick={() => cancelRecovery(activeRecovery.id)}
              >
                Cancel Recovery
              </Button>
            </AlertDescription>
          </Alert>
        )}

        {/* Pending Approvals */}
        {pendingApprovals.length > 0 && (
          <Alert>
            <Clock className="w-4 h-4" />
            <AlertTitle>Pending Approvals</AlertTitle>
            <AlertDescription>
              You have {pendingApprovals.length} recovery{' '}
              {pendingApprovals.length === 1 ? 'request' : 'requests'} waiting for your approval as
              a guardian.
            </AlertDescription>
          </Alert>
        )}

        <Tabs defaultValue="guardians" className="space-y-6">
          <TabsList className="grid w-full grid-cols-2">
            <TabsTrigger value="guardians">
              <Shield className="w-4 h-4 mr-2" />
              Guardians ({guardians.length})
            </TabsTrigger>
            <TabsTrigger value="settings">
              Settings
            </TabsTrigger>
          </TabsList>

          {/* Guardians Tab */}
          <TabsContent value="guardians" className="space-y-4">
            {/* Add Guardian Button */}
            <Dialog open={isAddDialogOpen} onOpenChange={setIsAddDialogOpen}>
              <DialogTrigger asChild>
                <Button className="w-full">
                  <Plus className="w-4 h-4 mr-2" />
                  Add Guardian
                </Button>
              </DialogTrigger>
              <DialogContent>
                <DialogHeader>
                  <DialogTitle>Add Guardian</DialogTitle>
                  <DialogDescription>
                    Add a trusted friend or family member as a guardian
                  </DialogDescription>
                </DialogHeader>
                <div className="space-y-4 py-4">
                  <div className="space-y-2">
                    <Label htmlFor="username">Username (optional)</Label>
                    <Input
                      id="username"
                      placeholder="alice.etrid"
                      value={newGuardianUsername}
                      onChange={(e) => setNewGuardianUsername(e.target.value)}
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="address">Wallet Address</Label>
                    <Input
                      id="address"
                      placeholder="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
                      value={newGuardianAddress}
                      onChange={(e) => setNewGuardianAddress(e.target.value)}
                    />
                  </div>
                </div>
                <DialogFooter>
                  <Button variant="outline" onClick={() => setIsAddDialogOpen(false)}>
                    Cancel
                  </Button>
                  <Button onClick={handleAddGuardian} disabled={!newGuardianAddress}>
                    Add Guardian
                  </Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>

            {/* Guardian List */}
            {isLoading ? (
              <div className="text-center py-12">
                <p className="text-muted-foreground">Loading guardians...</p>
              </div>
            ) : guardians.length === 0 ? (
              <div className="text-center py-12">
                <Shield className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No guardians yet</p>
                <p className="text-sm text-muted-foreground mt-2">
                  Add 3-5 trusted guardians to secure your account
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {/* Active Guardians */}
                {activeGuardians.length > 0 && (
                  <div className="space-y-2">
                    <h3 className="text-sm font-medium text-muted-foreground">
                      Active ({activeGuardians.length})
                    </h3>
                    {activeGuardians.map((guardian) => (
                      <GuardianCard
                        key={guardian.id}
                        guardian={guardian}
                        onRemove={handleRemoveGuardian}
                      />
                    ))}
                  </div>
                )}

                {/* Pending Guardians */}
                {pendingGuardians.length > 0 && (
                  <div className="space-y-2">
                    <h3 className="text-sm font-medium text-muted-foreground">
                      Pending ({pendingGuardians.length})
                    </h3>
                    {pendingGuardians.map((guardian) => (
                      <GuardianCard
                        key={guardian.id}
                        guardian={guardian}
                        onRemove={handleRemoveGuardian}
                        onResendInvitation={resendInvitation}
                      />
                    ))}
                  </div>
                )}
              </div>
            )}
          </TabsContent>

          {/* Settings Tab */}
          <TabsContent value="settings" className="space-y-4">
            {/* Recovery Threshold */}
            <Card>
              <CardHeader>
                <CardTitle>Recovery Threshold</CardTitle>
                <CardDescription>
                  Number of guardians required to approve recovery
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Required Approvals</span>
                  <span className="text-2xl font-bold text-accent">
                    {config?.threshold || 0} of {activeGuardians.length}
                  </span>
                </div>

                {activeGuardians.length > 0 && (
                  <div className="space-y-2">
                    <Slider
                      min={1}
                      max={activeGuardians.length}
                      step={1}
                      value={[config?.threshold || recommendedThreshold]}
                      onValueChange={handleThresholdChange}
                      disabled={activeGuardians.length === 0}
                    />
                    <p className="text-xs text-muted-foreground">
                      Recommended: {recommendedThreshold} (
                      {Math.round((recommendedThreshold / activeGuardians.length) * 100)}%)
                    </p>
                  </div>
                )}
              </CardContent>
            </Card>

            {/* Security Delay */}
            <Card>
              <CardHeader>
                <CardTitle>Security Delay</CardTitle>
                <CardDescription>
                  Time delay before recovery completes
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="flex items-center justify-between">
                  <span className="text-sm">Delay Period</span>
                  <span className="text-lg font-semibold">48 hours</span>
                </div>
                <p className="text-xs text-muted-foreground mt-2">
                  This gives you time to cancel if someone tries to recover your account without
                  permission
                </p>
              </CardContent>
            </Card>

            {/* Initiate Recovery */}
            <Card className="border-destructive/20">
              <CardHeader>
                <CardTitle className="text-destructive">Initiate Recovery</CardTitle>
                <CardDescription>
                  Lost access to your wallet? Start the recovery process
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Dialog open={isRecoveryDialogOpen} onOpenChange={setIsRecoveryDialogOpen}>
                  <DialogTrigger asChild>
                    <Button
                      variant="destructive"
                      className="w-full"
                      disabled={activeGuardians.length < (config?.threshold || 1)}
                    >
                      Start Recovery Process
                    </Button>
                  </DialogTrigger>
                  <DialogContent>
                    <DialogHeader>
                      <DialogTitle>Initiate Account Recovery</DialogTitle>
                      <DialogDescription>
                        This will notify your guardians to approve recovery to a new device
                      </DialogDescription>
                    </DialogHeader>
                    <div className="space-y-4 py-4">
                      <Alert>
                        <AlertTriangle className="w-4 h-4" />
                        <AlertDescription>
                          Make sure you have access to your new device before starting recovery
                        </AlertDescription>
                      </Alert>
                      <div className="space-y-2">
                        <Label htmlFor="deviceId">New Device ID</Label>
                        <Input
                          id="deviceId"
                          placeholder="Enter new device identifier"
                          value={newDeviceId}
                          onChange={(e) => setNewDeviceId(e.target.value)}
                        />
                      </div>
                    </div>
                    <DialogFooter>
                      <Button variant="outline" onClick={() => setIsRecoveryDialogOpen(false)}>
                        Cancel
                      </Button>
                      <Button
                        variant="destructive"
                        onClick={handleInitiateRecovery}
                        disabled={!newDeviceId}
                      >
                        Initiate Recovery
                      </Button>
                    </DialogFooter>
                  </DialogContent>
                </Dialog>

                {activeGuardians.length < (config?.threshold || 1) && (
                  <p className="text-xs text-destructive mt-2">
                    You need at least {config?.threshold || 1} active guardians to enable recovery
                  </p>
                )}
              </CardContent>
            </Card>

            {/* Best Practices */}
            <Card className="bg-accent/5 border-accent/20">
              <CardHeader>
                <CardTitle className="text-sm">Best Practices</CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="space-y-2 text-sm text-muted-foreground">
                  <li>• Add 3-5 trusted guardians</li>
                  <li>• Choose people you trust completely</li>
                  <li>• Set threshold to 60-70% of total guardians</li>
                  <li>• Inform guardians of their responsibility</li>
                  <li>• Keep guardian list updated</li>
                </ul>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </main>
    </div>
  );
}
