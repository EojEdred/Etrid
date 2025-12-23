'use client';

import { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog';
import {
  Wallet,
  Plus,
  Download,
  Lock,
  Eye,
  EyeOff,
  Copy,
  Check,
  AlertTriangle,
  ArrowRight,
  ArrowLeft,
  Shield,
  Loader2,
} from 'lucide-react';
import { useWallet } from '@/contexts/WalletContext';

type OnboardingStep = 'choose' | 'create_show' | 'create_confirm' | 'create_password' | 'import' | 'unlock';

interface WalletOnboardingProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSuccess?: () => void;
}

export default function WalletOnboarding({ open, onOpenChange, onSuccess }: WalletOnboardingProps) {
  const {
    status,
    storedWallet,
    createNewWallet,
    importExistingWallet,
    unlock,
    error,
    generateNewMnemonic,
  } = useWallet();

  const [step, setStep] = useState<OnboardingStep>('choose');
  const [mnemonic, setMnemonic] = useState('');
  const [mnemonicConfirm, setMnemonicConfirm] = useState('');
  const [password, setPassword] = useState('');
  const [passwordConfirm, setPasswordConfirm] = useState('');
  const [importMnemonic, setImportMnemonic] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [copied, setCopied] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [localError, setLocalError] = useState<string | null>(null);
  const [walletName, setWalletName] = useState('My ETRID Wallet');

  // Set initial step based on wallet status
  useEffect(() => {
    if (status === 'locked' && storedWallet) {
      setStep('unlock');
    } else if (status === 'no_wallet') {
      setStep('choose');
    }
  }, [status, storedWallet]);

  const resetState = () => {
    setMnemonic('');
    setMnemonicConfirm('');
    setPassword('');
    setPasswordConfirm('');
    setImportMnemonic('');
    setLocalError(null);
    setCopied(false);
  };

  const handleClose = () => {
    resetState();
    onOpenChange(false);
  };

  const handleCopyMnemonic = () => {
    navigator.clipboard.writeText(mnemonic);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleStartCreate = () => {
    // Generate mnemonic when user starts create flow
    const newMnemonic = generateNewMnemonic();
    setMnemonic(newMnemonic);
    setStep('create_show');
  };

  const handleConfirmMnemonic = () => {
    if (mnemonicConfirm.trim().toLowerCase() !== mnemonic.trim().toLowerCase()) {
      setLocalError('Recovery phrase does not match. Please try again.');
      return;
    }
    setLocalError(null);
    setStep('create_password');
  };

  const handleCreateWallet = async () => {
    if (password.length < 8) {
      setLocalError('Password must be at least 8 characters');
      return;
    }
    if (password !== passwordConfirm) {
      setLocalError('Passwords do not match');
      return;
    }

    setIsLoading(true);
    setLocalError(null);

    try {
      await createNewWallet(password, walletName);
      handleClose();
      onSuccess?.();
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : 'Failed to create wallet');
    } finally {
      setIsLoading(false);
    }
  };

  const handleImportWallet = async () => {
    if (password.length < 8) {
      setLocalError('Password must be at least 8 characters');
      return;
    }
    if (password !== passwordConfirm) {
      setLocalError('Passwords do not match');
      return;
    }

    setIsLoading(true);
    setLocalError(null);

    try {
      await importExistingWallet(importMnemonic.trim(), password, walletName);
      handleClose();
      onSuccess?.();
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : 'Failed to import wallet');
    } finally {
      setIsLoading(false);
    }
  };

  const handleUnlock = async () => {
    setIsLoading(true);
    setLocalError(null);

    try {
      await unlock(password);
      handleClose();
      onSuccess?.();
    } catch (err) {
      setLocalError(err instanceof Error ? err.message : 'Invalid password');
    } finally {
      setIsLoading(false);
    }
  };

  const renderChooseStep = () => (
    <div className="space-y-6">
      <div className="grid grid-cols-1 gap-4">
        <button
          onClick={handleStartCreate}
          className="p-6 glass-card border border-white/10 rounded-xl text-left hover:border-cyan-500/50 transition-all group"
        >
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-cyan-500 to-blue-500 flex items-center justify-center">
              <Plus className="w-6 h-6 text-white" />
            </div>
            <div className="flex-1">
              <h3 className="font-semibold text-lg mb-1 group-hover:text-cyan-400 transition-colors">
                Create New Wallet
              </h3>
              <p className="text-sm text-white/60">
                Generate a new wallet with a secure recovery phrase
              </p>
            </div>
            <ArrowRight className="w-5 h-5 text-white/40 group-hover:text-cyan-400 transition-colors mt-1" />
          </div>
        </button>

        <button
          onClick={() => setStep('import')}
          className="p-6 glass-card border border-white/10 rounded-xl text-left hover:border-purple-500/50 transition-all group"
        >
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center">
              <Download className="w-6 h-6 text-white" />
            </div>
            <div className="flex-1">
              <h3 className="font-semibold text-lg mb-1 group-hover:text-purple-400 transition-colors">
                Import Existing Wallet
              </h3>
              <p className="text-sm text-white/60">
                Restore a wallet using your 12-word recovery phrase
              </p>
            </div>
            <ArrowRight className="w-5 h-5 text-white/40 group-hover:text-purple-400 transition-colors mt-1" />
          </div>
        </button>
      </div>
    </div>
  );

  const renderShowMnemonicStep = () => (
    <div className="space-y-6">
      <div className="p-4 glass rounded-lg border border-amber-500/30">
        <div className="flex items-start gap-3">
          <AlertTriangle className="w-5 h-5 text-amber-500 flex-shrink-0 mt-0.5" />
          <div>
            <p className="font-medium text-amber-500">Write this down!</p>
            <p className="text-sm text-white/60 mt-1">
              This is your wallet's recovery phrase. Store it safely - you'll need it to recover your wallet.
              Never share it with anyone.
            </p>
          </div>
        </div>
      </div>

      <div className="p-4 glass-card rounded-xl border border-white/10">
        <div className="grid grid-cols-3 gap-3">
          {mnemonic.split(' ').map((word, index) => (
            <div key={index} className="flex items-center gap-2 p-2 glass rounded-lg">
              <span className="text-xs text-white/40 w-5">{index + 1}.</span>
              <span className="font-mono text-sm">{word}</span>
            </div>
          ))}
        </div>
      </div>

      <Button
        variant="outline"
        onClick={handleCopyMnemonic}
        className="w-full glass border-white/20"
      >
        {copied ? (
          <>
            <Check className="w-4 h-4 mr-2 text-green-500" />
            Copied!
          </>
        ) : (
          <>
            <Copy className="w-4 h-4 mr-2" />
            Copy to Clipboard
          </>
        )}
      </Button>

      <div className="flex gap-3">
        <Button variant="outline" onClick={() => setStep('choose')} className="glass border-white/20">
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>
        <Button onClick={() => setStep('create_confirm')} className="flex-1 btn-primary">
          I've Written It Down
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  );

  const renderConfirmMnemonicStep = () => (
    <div className="space-y-6">
      <p className="text-white/60">
        Enter your recovery phrase to confirm you've saved it correctly.
      </p>

      <div className="space-y-2">
        <label className="text-sm font-medium">Recovery Phrase</label>
        <textarea
          value={mnemonicConfirm}
          onChange={(e) => setMnemonicConfirm(e.target.value)}
          placeholder="Enter your 12-word recovery phrase..."
          className="w-full h-24 p-3 glass border border-white/20 rounded-lg bg-transparent text-white placeholder:text-white/40 resize-none font-mono text-sm"
        />
      </div>

      {localError && (
        <div className="p-3 glass rounded-lg border border-red-500/30">
          <p className="text-sm text-red-400">{localError}</p>
        </div>
      )}

      <div className="flex gap-3">
        <Button variant="outline" onClick={() => setStep('create_show')} className="glass border-white/20">
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>
        <Button onClick={handleConfirmMnemonic} className="flex-1 btn-primary">
          Confirm
          <ArrowRight className="w-4 h-4 ml-2" />
        </Button>
      </div>
    </div>
  );

  const renderPasswordStep = () => (
    <div className="space-y-6">
      <div className="p-4 glass rounded-lg border border-blue-500/30">
        <div className="flex items-start gap-3">
          <Shield className="w-5 h-5 text-blue-400 flex-shrink-0 mt-0.5" />
          <div>
            <p className="font-medium text-blue-400">Secure your wallet</p>
            <p className="text-sm text-white/60 mt-1">
              Create a strong password to protect your wallet. You'll need this password to unlock your wallet.
            </p>
          </div>
        </div>
      </div>

      <div className="space-y-4">
        <div className="space-y-2">
          <label className="text-sm font-medium">Wallet Name</label>
          <Input
            value={walletName}
            onChange={(e) => setWalletName(e.target.value)}
            placeholder="My ETRID Wallet"
            className="glass border-white/20 bg-transparent"
          />
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Password</label>
          <div className="relative">
            <Input
              type={showPassword ? 'text' : 'password'}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter password (min. 8 characters)"
              className="glass border-white/20 bg-transparent pr-10"
            />
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-white/40 hover:text-white/60"
            >
              {showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
            </button>
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Confirm Password</label>
          <Input
            type={showPassword ? 'text' : 'password'}
            value={passwordConfirm}
            onChange={(e) => setPasswordConfirm(e.target.value)}
            placeholder="Confirm your password"
            className="glass border-white/20 bg-transparent"
          />
        </div>
      </div>

      {localError && (
        <div className="p-3 glass rounded-lg border border-red-500/30">
          <p className="text-sm text-red-400">{localError}</p>
        </div>
      )}

      <div className="flex gap-3">
        <Button
          variant="outline"
          onClick={() => setStep('create_confirm')}
          className="glass border-white/20"
          disabled={isLoading}
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>
        <Button
          onClick={handleCreateWallet}
          disabled={isLoading || !password || !passwordConfirm}
          className="flex-1 btn-primary"
        >
          {isLoading ? (
            <>
              <Loader2 className="w-4 h-4 mr-2 animate-spin" />
              Creating...
            </>
          ) : (
            <>
              <Wallet className="w-4 h-4 mr-2" />
              Create Wallet
            </>
          )}
        </Button>
      </div>
    </div>
  );

  const renderImportStep = () => (
    <div className="space-y-6">
      <p className="text-white/60">
        Enter your 12-word recovery phrase to restore your wallet.
      </p>

      <div className="space-y-4">
        <div className="space-y-2">
          <label className="text-sm font-medium">Recovery Phrase</label>
          <textarea
            value={importMnemonic}
            onChange={(e) => setImportMnemonic(e.target.value)}
            placeholder="Enter your 12-word recovery phrase, separated by spaces..."
            className="w-full h-24 p-3 glass border border-white/20 rounded-lg bg-transparent text-white placeholder:text-white/40 resize-none font-mono text-sm"
          />
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Wallet Name</label>
          <Input
            value={walletName}
            onChange={(e) => setWalletName(e.target.value)}
            placeholder="My ETRID Wallet"
            className="glass border-white/20 bg-transparent"
          />
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">New Password</label>
          <div className="relative">
            <Input
              type={showPassword ? 'text' : 'password'}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter password (min. 8 characters)"
              className="glass border-white/20 bg-transparent pr-10"
            />
            <button
              type="button"
              onClick={() => setShowPassword(!showPassword)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-white/40 hover:text-white/60"
            >
              {showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
            </button>
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Confirm Password</label>
          <Input
            type={showPassword ? 'text' : 'password'}
            value={passwordConfirm}
            onChange={(e) => setPasswordConfirm(e.target.value)}
            placeholder="Confirm your password"
            className="glass border-white/20 bg-transparent"
          />
        </div>
      </div>

      {localError && (
        <div className="p-3 glass rounded-lg border border-red-500/30">
          <p className="text-sm text-red-400">{localError}</p>
        </div>
      )}

      <div className="flex gap-3">
        <Button
          variant="outline"
          onClick={() => { resetState(); setStep('choose'); }}
          className="glass border-white/20"
          disabled={isLoading}
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>
        <Button
          onClick={handleImportWallet}
          disabled={isLoading || !importMnemonic || !password || !passwordConfirm}
          className="flex-1 btn-primary"
        >
          {isLoading ? (
            <>
              <Loader2 className="w-4 h-4 mr-2 animate-spin" />
              Importing...
            </>
          ) : (
            <>
              <Download className="w-4 h-4 mr-2" />
              Import Wallet
            </>
          )}
        </Button>
      </div>
    </div>
  );

  const renderUnlockStep = () => (
    <div className="space-y-6">
      <div className="flex flex-col items-center text-center mb-6">
        <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-cyan-500 to-purple-500 flex items-center justify-center mb-4">
          <Lock className="w-8 h-8 text-white" />
        </div>
        <p className="text-white/60">
          Enter your password to unlock{' '}
          <span className="text-white font-medium">{storedWallet?.name || 'your wallet'}</span>
        </p>
        <p className="text-xs text-white/40 mt-1 font-mono">
          {storedWallet?.address ? `${storedWallet.address.slice(0, 8)}...${storedWallet.address.slice(-6)}` : ''}
        </p>
      </div>

      <div className="space-y-2">
        <label className="text-sm font-medium">Password</label>
        <div className="relative">
          <Input
            type={showPassword ? 'text' : 'password'}
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            placeholder="Enter your password"
            className="glass border-white/20 bg-transparent pr-10"
            onKeyDown={(e) => e.key === 'Enter' && handleUnlock()}
          />
          <button
            type="button"
            onClick={() => setShowPassword(!showPassword)}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-white/40 hover:text-white/60"
          >
            {showPassword ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
          </button>
        </div>
      </div>

      {localError && (
        <div className="p-3 glass rounded-lg border border-red-500/30">
          <p className="text-sm text-red-400">{localError}</p>
        </div>
      )}

      <Button
        onClick={handleUnlock}
        disabled={isLoading || !password}
        className="w-full btn-primary"
      >
        {isLoading ? (
          <>
            <Loader2 className="w-4 h-4 mr-2 animate-spin" />
            Unlocking...
          </>
        ) : (
          <>
            <Lock className="w-4 h-4 mr-2" />
            Unlock Wallet
          </>
        )}
      </Button>

      <p className="text-center text-xs text-white/40">
        Forgot password?{' '}
        <button
          onClick={() => setStep('choose')}
          className="text-cyan-400 hover:underline"
        >
          Import with recovery phrase
        </button>
      </p>
    </div>
  );

  const getStepTitle = () => {
    switch (step) {
      case 'choose':
        return 'Connect to ETRID';
      case 'create_show':
        return 'Your Recovery Phrase';
      case 'create_confirm':
        return 'Confirm Recovery Phrase';
      case 'create_password':
        return 'Create Password';
      case 'import':
        return 'Import Wallet';
      case 'unlock':
        return 'Welcome Back';
      default:
        return 'Wallet';
    }
  };

  const getStepDescription = () => {
    switch (step) {
      case 'choose':
        return 'Create or import a wallet to get started';
      case 'create_show':
        return 'Save these 12 words in a safe place';
      case 'create_confirm':
        return 'Verify you saved your recovery phrase';
      case 'create_password':
        return 'Protect your wallet with a password';
      case 'import':
        return 'Restore your wallet from a recovery phrase';
      case 'unlock':
        return 'Unlock your wallet to continue';
      default:
        return '';
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="glass-card border-white/20 max-w-md">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-xl gradient-text">
            <Wallet className="w-5 h-5" />
            {getStepTitle()}
          </DialogTitle>
          <DialogDescription className="text-white/60">
            {getStepDescription()}
          </DialogDescription>
        </DialogHeader>

        <div className="py-4">
          {step === 'choose' && renderChooseStep()}
          {step === 'create_show' && renderShowMnemonicStep()}
          {step === 'create_confirm' && renderConfirmMnemonicStep()}
          {step === 'create_password' && renderPasswordStep()}
          {step === 'import' && renderImportStep()}
          {step === 'unlock' && renderUnlockStep()}
        </div>
      </DialogContent>
    </Dialog>
  );
}
