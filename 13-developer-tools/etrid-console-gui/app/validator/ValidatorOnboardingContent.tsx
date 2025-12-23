'use client';

import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Textarea } from '@/components/ui/textarea';
import { useWallet } from '@/contexts/WalletContext';
import { useValidate, useStakingInfo } from '@/hooks/useStaking';
import { useToast } from '@/hooks/use-toast';
import { Shield, CheckCircle } from 'lucide-react';
import Link from 'next/link';

export default function ValidatorOnboardingContent() {
  const { account } = useWallet();
  const address = account?.address;
  const { toast } = useToast();
  
  const [moniker, setMoniker] = useState('');
  const [commission, setCommission] = useState('10');
  const [description, setDescription] = useState('');

  const validateMutation = useValidate();
  const { data: stakingInfo } = useStakingInfo(address || undefined);

  const handleRegister = async () => {
    if (!address) return;
    
    try {
      await validateMutation.mutateAsync({
        address,
        commission: parseFloat(commission),
      });
      
      toast({
        title: "Validator Registered",
        description: "You have successfully registered as a validator.",
      });
    } catch (error) {
      toast({
        title: "Registration Failed",
        description: error instanceof Error ? error.message : "Unknown error",
        variant: "destructive",
      });
    }
  };

  if (!address) {
    return (
      <div className="flex flex-col items-center justify-center min-h-[50vh]">
        <div className="text-center space-y-4">
          <Shield className="w-16 h-16 text-white/20 mx-auto" />
          <h2 className="text-2xl font-bold">Connect Wallet to Continue</h2>
          <p className="text-white/60">You need to connect a wallet to register as a validator.</p>
          <Link href="/wallet">
            <Button className="btn-primary">Connect Wallet</Button>
          </Link>
        </div>
      </div>
    );
  }

  // If already validating (checking staking info or similar)
  // For now, simple form.

  return (
    <div className="container mx-auto px-4 py-8 max-w-2xl">
      <Card className="glass-card">
        <div className="p-6">
          <div className="flex items-center gap-3 mb-6">
            <div className="w-10 h-10 rounded-full bg-purple-500/20 flex items-center justify-center">
              <Shield className="w-6 h-6 text-purple-400" />
            </div>
            <div>
              <h2 className="text-xl font-bold">Register Validator</h2>
              <p className="text-white/60 text-sm">Join the network consensus</p>
            </div>
          </div>

          <div className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="moniker">Validator Moniker</Label>
              <Input 
                id="moniker" 
                placeholder="e.g. My Node" 
                value={moniker}
                onChange={(e) => setMoniker(e.target.value)}
              />
              <p className="text-xs text-white/40">This name will be public on the explorer.</p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="commission">Commission Rate (%)</Label>
              <Input 
                id="commission" 
                type="number" 
                min="0" 
                max="100" 
                step="0.1"
                value={commission}
                onChange={(e) => setCommission(e.target.value)}
              />
              <p className="text-xs text-white/40">Percentage of rewards you keep.</p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">Description</Label>
              <Textarea 
                id="description" 
                placeholder="Brief description of your validator..." 
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="glass-input"
              />
            </div>

            <div className="pt-4">
              <Button 
                onClick={handleRegister} 
                disabled={validateMutation.isPending || !moniker}
                className="w-full btn-primary bg-gradient-to-r from-purple-500 to-pink-600"
              >
                {validateMutation.isPending ? 'Registering...' : 'Register Validator'}
              </Button>
            </div>
          </div>
        </div>
      </Card>
      
      {/* Active Validators List (Placeholder or reuse existing component) */}
      <div className="mt-8">
        <h3 className="text-lg font-bold mb-4">Network Status</h3>
        <Card className="glass-card p-4">
            <p className="text-white/60">
              Note: To become an active validator, you must bond tokens (self-stake) in the Staking section first.
            </p>
            <div className="mt-4 flex gap-4">
                <Link href="/staking">
                    <Button variant="outline">Go to Staking</Button>
                </Link>
            </div>
        </Card>
      </div>
    </div>
  );
}
