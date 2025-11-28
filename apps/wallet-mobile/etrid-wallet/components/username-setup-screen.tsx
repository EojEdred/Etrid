/**
 * Username Setup Screen
 * Allows users to claim and register a username
 */

'use client';

import { useState } from 'react';
import { ArrowLeft, Sparkles, Check } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { UsernameInput } from '@/components/social/username-input';
import { useUsername } from '@/hooks/useUsername';
import { useWallet } from '@/lib/polkadot/useWallet';

interface UsernameSetupScreenProps {
  onBack: () => void;
}

export function UsernameSetupScreen({ onBack }: UsernameSetupScreenProps) {
  const [username, setUsername] = useState('');
  const [isValid, setIsValid] = useState(false);
  const [isRegistered, setIsRegistered] = useState(false);

  const { pricing, isRegistering, registerUsername, error } = useUsername();
  const { selectedAccount } = useWallet();

  const handleRegister = async () => {
    if (!selectedAccount || !isValid) return;

    try {
      await registerUsername(username, selectedAccount.address);
      setIsRegistered(true);
    } catch (err) {
      console.error('Failed to register username:', err);
    }
  };

  if (isRegistered) {
    return (
      <div className="min-h-screen bg-background">
        {/* Success State */}
        <div className="flex flex-col items-center justify-center min-h-screen p-6">
          <div className="w-20 h-20 rounded-full bg-green-500/10 flex items-center justify-center mb-6">
            <Check className="w-10 h-10 text-green-500" />
          </div>

          <h1 className="text-3xl font-bold text-center mb-3">
            Welcome, @{username}!
          </h1>

          <p className="text-center text-muted-foreground mb-8 max-w-md">
            Your username has been successfully registered. You can now receive payments using{' '}
            <span className="font-mono text-accent">{username}.etrid</span>
          </p>

          <div className="w-full max-w-md space-y-3">
            <Card className="bg-accent/5 border-accent/20">
              <CardContent className="pt-6">
                <div className="space-y-3 text-sm">
                  <div className="flex items-start gap-3">
                    <div className="w-6 h-6 rounded-full bg-accent/20 flex items-center justify-center flex-shrink-0 mt-0.5">
                      <Check className="w-4 h-4 text-accent" />
                    </div>
                    <div>
                      <p className="font-medium">Easy to Share</p>
                      <p className="text-muted-foreground">
                        Share @{username} instead of your long address
                      </p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="w-6 h-6 rounded-full bg-accent/20 flex items-center justify-center flex-shrink-0 mt-0.5">
                      <Check className="w-4 h-4 text-accent" />
                    </div>
                    <div>
                      <p className="font-medium">Registered for 1 Year</p>
                      <p className="text-muted-foreground">
                        Valid until {new Date(Date.now() + 365 * 24 * 60 * 60 * 1000).toLocaleDateString()}
                      </p>
                    </div>
                  </div>

                  <div className="flex items-start gap-3">
                    <div className="w-6 h-6 rounded-full bg-accent/20 flex items-center justify-center flex-shrink-0 mt-0.5">
                      <Check className="w-4 h-4 text-accent" />
                    </div>
                    <div>
                      <p className="font-medium">Transferable</p>
                      <p className="text-muted-foreground">
                        You can transfer or sell your username
                      </p>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Button onClick={onBack} className="w-full" size="lg">
              Continue to Wallet
            </Button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-background pb-24">
      {/* Header */}
      <header className="sticky top-0 z-10 glass-strong border-b border-border">
        <div className="flex items-center gap-4 p-6">
          <Button variant="ghost" size="icon" onClick={onBack}>
            <ArrowLeft className="w-5 h-5" />
          </Button>
          <div>
            <h1 className="text-xl font-bold">Claim Your Username</h1>
            <p className="text-sm text-muted-foreground">Get your unique @username.etrid</p>
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="p-6 space-y-6 max-w-2xl mx-auto">
        {/* Hero Card */}
        <Card className="bg-gradient-to-br from-accent/20 to-accent/5 border-accent/30">
          <CardHeader>
            <div className="flex items-center gap-2 mb-2">
              <Sparkles className="w-5 h-5 text-accent" />
              <CardTitle>ENS-Style Usernames</CardTitle>
            </div>
            <CardDescription>
              Claim your unique username on the Ëtrid network. Share @username instead of your
              long wallet address. Usernames are NFTs that you truly own.
            </CardDescription>
          </CardHeader>
        </Card>

        {/* Username Input Card */}
        <Card>
          <CardHeader>
            <CardTitle>Choose Your Username</CardTitle>
            <CardDescription>
              Pick a memorable username. Shorter usernames cost more.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <UsernameInput
              value={username}
              onChange={setUsername}
              onValidationChange={setIsValid}
            />
          </CardContent>
        </Card>

        {/* Pricing Tiers Info */}
        <Card>
          <CardHeader>
            <CardTitle>Pricing Tiers</CardTitle>
            <CardDescription>Username prices vary by length</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              <div className="flex items-center justify-between p-3 rounded-lg border border-border">
                <div>
                  <p className="font-medium">Premium (1-3 characters)</p>
                  <p className="text-sm text-muted-foreground">e.g., @eoj, @ai</p>
                </div>
                <p className="text-lg font-bold text-accent">1,000 ÉTR/yr</p>
              </div>

              <div className="flex items-center justify-between p-3 rounded-lg border border-border">
                <div>
                  <p className="font-medium">Standard (4-5 characters)</p>
                  <p className="text-sm text-muted-foreground">e.g., @alice, @web3</p>
                </div>
                <p className="text-lg font-bold text-accent">100 ÉTR/yr</p>
              </div>

              <div className="flex items-center justify-between p-3 rounded-lg border border-border">
                <div>
                  <p className="font-medium">Basic (6+ characters)</p>
                  <p className="text-sm text-muted-foreground">e.g., @myusername</p>
                </div>
                <p className="text-lg font-bold text-accent">10 ÉTR/yr</p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Benefits */}
        <Card>
          <CardHeader>
            <CardTitle>Benefits</CardTitle>
          </CardHeader>
          <CardContent>
            <ul className="space-y-3">
              <li className="flex items-start gap-3">
                <Check className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-medium">Easy to Remember</p>
                  <p className="text-sm text-muted-foreground">
                    Share @username instead of long addresses
                  </p>
                </div>
              </li>
              <li className="flex items-start gap-3">
                <Check className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-medium">Truly Yours</p>
                  <p className="text-sm text-muted-foreground">
                    Usernames are NFTs you fully own
                  </p>
                </div>
              </li>
              <li className="flex items-start gap-3">
                <Check className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-medium">Subdomains</p>
                  <p className="text-sm text-muted-foreground">
                    Create subdomains like alice.yourname.etrid
                  </p>
                </div>
              </li>
              <li className="flex items-start gap-3">
                <Check className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="font-medium">Transferable</p>
                  <p className="text-sm text-muted-foreground">
                    Sell or transfer your username to others
                  </p>
                </div>
              </li>
            </ul>
          </CardContent>
        </Card>

        {/* Error Message */}
        {error && (
          <div className="p-4 rounded-lg bg-destructive/10 border border-destructive/20">
            <p className="text-sm text-destructive">{error}</p>
          </div>
        )}

        {/* Register Button */}
        <Button
          size="lg"
          className="w-full"
          disabled={!isValid || isRegistering || !selectedAccount}
          onClick={handleRegister}
        >
          {isRegistering ? (
            'Registering...'
          ) : pricing ? (
            `Register for ${pricing.price} ÉTR/year`
          ) : (
            'Enter a username'
          )}
        </Button>

        {!selectedAccount && (
          <p className="text-sm text-center text-destructive">
            Please connect your wallet to register a username
          </p>
        )}
      </main>
    </div>
  );
}
