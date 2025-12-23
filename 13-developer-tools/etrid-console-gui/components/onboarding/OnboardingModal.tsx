'use client';

import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from '@/components/ui/dialog';
import { getExplorerBaseUrl, getPrimearcWsEndpoints, setExplorerBaseUrl, setPrimearcWsEndpoints } from '@/lib/runtime-config';

export function OnboardingModal({ open, onComplete }: { open: boolean; onComplete: () => void }) {
  const [step, setStep] = useState(1);
  const [agreedCharter, setAgreedCharter] = useState(false);
  const [agreedDocs, setAgreedDocs] = useState(false);
  const [primearcWsEndpointsText, setPrimearcWsEndpointsText] = useState('');
  const [explorerBaseUrlText, setExplorerBaseUrlText] = useState('');
  const [configError, setConfigError] = useState<string | null>(null);

  useEffect(() => {
    if (!open) return;
    setStep(1);
    setAgreedCharter(false);
    setAgreedDocs(false);
    setConfigError(null);
    setPrimearcWsEndpointsText(getPrimearcWsEndpoints().join(','));
    setExplorerBaseUrlText(getExplorerBaseUrl());
  }, [open]);

  const parseWsEndpoints = (value: string) =>
    value
      .split(',')
      .map((s) => s.trim())
      .filter(Boolean);

  const isValidExplorerUrl = (value: string) => {
    const trimmed = value.trim();
    if (!trimmed) return true;
    try {
      const url = new URL(trimmed);
      return url.protocol === 'http:' || url.protocol === 'https:';
    } catch {
      return false;
    }
  };

  const validateNetworkConfig = (): string | null => {
    const endpoints = parseWsEndpoints(primearcWsEndpointsText);
    if (endpoints.length === 0) return 'Enter at least one Primearc Core WS endpoint.';
    const bad = endpoints.find((e) => !(e.startsWith('ws://') || e.startsWith('wss://')));
    if (bad) return `Invalid WS endpoint: ${bad}`;
    if (!isValidExplorerUrl(explorerBaseUrlText)) return 'Explorer base URL must be a valid http(s) URL.';
    return null;
  };

  const handleNext = () => {
    setConfigError(null);

    if (step === 3) {
      const validationError = validateNetworkConfig();
      if (validationError) {
        setConfigError(validationError);
        return;
      }

      setPrimearcWsEndpoints(parseWsEndpoints(primearcWsEndpointsText));
      const explorerTrimmed = explorerBaseUrlText.trim();
      if (explorerTrimmed) {
        setExplorerBaseUrl(explorerTrimmed);
      }
    }

    if (step < 4) {
      setStep(step + 1);
      return;
    }

    onComplete();
  };

  return (
    <Dialog open={open}>
      <DialogContent className="max-w-3xl glass-card border-white/20 [&>button]:hidden pointer-events-auto">
        <DialogHeader>
          <DialogTitle className="text-2xl font-bold gradient-text">
            {step === 1
              ? 'Protocol Charter'
              : step === 2
                ? 'Disclosure Documentation'
                : step === 3
                  ? 'Network & Explorer Setup'
                  : 'How-To Guide'}
          </DialogTitle>
          <DialogDescription className="sr-only">
            Complete the required onboarding steps before using the console.
          </DialogDescription>
        </DialogHeader>

        <div className="py-4">
          <ScrollArea className="h-[400px] w-full rounded-md border border-white/10 bg-black/20 p-4">
            {step === 1 && (
              <div className="space-y-4 text-white/80">
                <h3 className="text-xl font-bold text-cyan-400">1. Decentralization</h3>
                <p>The Etrid Protocol is a decentralized network. No single entity controls the network.</p>
                <h3 className="text-xl font-bold text-cyan-400">2. Self-Custody</h3>
                <p>You are solely responsible for your private keys. Losing your keys means losing your funds.</p>
                <h3 className="text-xl font-bold text-cyan-400">3. Risks</h3>
                <p>Participating in DeFi and staking involves risks, including slashing and smart contract bugs.</p>
              </div>
            )}
            {step === 2 && (
              <div className="space-y-4 text-white/80">
                <h3 className="text-xl font-bold text-purple-400">Risk Disclosure</h3>
                <p>This software is provided "as is", without warranty of any kind.</p>
                <h3 className="text-xl font-bold text-purple-400">Financial Advice</h3>
                <p>Nothing in this application constitutes financial advice.</p>
              </div>
            )}
            {step === 3 && (
              <div className="space-y-4 text-white/80">
                <h3 className="text-xl font-bold text-amber-400">Primearc Core Connectivity</h3>
                <p>
                  The desktop app can’t rely on server-side rendering, and some hostnames may not resolve in all
                  environments. Configure the RPC endpoints and explorer you want to use.
                </p>

                <div className="space-y-2">
                  <Label htmlFor="primearc-ws-endpoints">Primearc WS endpoints (comma-separated)</Label>
                  <Input
                    id="primearc-ws-endpoints"
                    value={primearcWsEndpointsText}
                    onChange={(e) => setPrimearcWsEndpointsText(e.target.value)}
                    className="font-mono"
                    placeholder="wss://rpc.etrid.org,ws://157.173.200.80:9944"
                  />
                  <p className="text-xs text-white/50">
                    Example: <span className="font-mono">wss://rpc.etrid.org</span>
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="explorer-base-url">Explorer base URL</Label>
                  <Input
                    id="explorer-base-url"
                    value={explorerBaseUrlText}
                    onChange={(e) => setExplorerBaseUrlText(e.target.value)}
                    className="font-mono"
                    placeholder="https://explorer.etrid.org"
                  />
                </div>

                {configError ? (
                  <div className="text-sm text-red-400">{configError}</div>
                ) : null}
              </div>
            )}
            {step === 4 && (
              <div className="space-y-4 text-white/80">
                <h3 className="text-xl font-bold text-green-400">Getting Started</h3>
                <ol className="list-decimal pl-5 space-y-2">
                  <li>Create a Wallet and save your mnemonic phrase securely.</li>
                  <li>Fund your wallet with ETR tokens.</li>
                  <li>Stake your tokens to earn rewards and secure the network.</li>
                  <li>Register as a validator if you wish to run a node.</li>
                </ol>
              </div>
            )}
          </ScrollArea>
        </div>

        <DialogFooter className="flex-col sm:justify-between sm:flex-row gap-4">
          <div className="flex items-center space-x-2">
            {step === 1 && (
              <>
                <Checkbox id="charter" checked={agreedCharter} onCheckedChange={(c) => setAgreedCharter(c as boolean)} />
                <label htmlFor="charter" className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-white/80">
                  I agree to the Protocol Charter
                </label>
              </>
            )}
            {step === 2 && (
              <>
                <Checkbox id="docs" checked={agreedDocs} onCheckedChange={(c) => setAgreedDocs(c as boolean)} />
                <label htmlFor="docs" className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-white/80">
                  I acknowledge the Disclosures
                </label>
              </>
            )}
          </div>
          
          <Button 
            onClick={handleNext} 
            disabled={
              (step === 1 && !agreedCharter) ||
              (step === 2 && !agreedDocs) ||
              (step === 3 && Boolean(validateNetworkConfig()))
            }
            className="btn-primary"
          >
            {step === 4 ? 'Complete Setup' : 'Next'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
