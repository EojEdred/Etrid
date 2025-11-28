/**
 * USAGE EXAMPLE: Ëtrid Wallet Onboarding Flow
 *
 * This file demonstrates how to integrate the onboarding flow
 * into your Next.js application.
 */

// ============================================================================
// EXAMPLE 1: Basic Usage with Next.js App Router
// ============================================================================

// File: app/onboarding/page.tsx
import { OnboardingContainer } from '@/components/onboarding';

export default function OnboardingPage() {
  return <OnboardingContainer />;
}

// ============================================================================
// EXAMPLE 2: With Metadata (SEO)
// ============================================================================

// File: app/onboarding/page.tsx
import { OnboardingContainer } from '@/components/onboarding';
import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Get Started - Ëtrid Wallet',
  description: 'Create your secure crypto wallet in minutes',
  robots: 'noindex', // Don't index onboarding pages
};

export default function OnboardingPage() {
  return <OnboardingContainer />;
}

// ============================================================================
// EXAMPLE 3: Protected Route (Only for New Users)
// ============================================================================

// File: app/onboarding/page.tsx
'use client';

import { OnboardingContainer } from '@/components/onboarding';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

export default function OnboardingPage() {
  const router = useRouter();
  const [hasWallet, setHasWallet] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Check if user already has a wallet
    const checkWallet = async () => {
      const walletExists = localStorage.getItem('etrid_wallet_exists');
      if (walletExists) {
        // Redirect to home if wallet already exists
        router.push('/');
      } else {
        setLoading(false);
      }
    };

    checkWallet();
  }, [router]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-b from-[#1a0033] to-[#4a0080]">
        <div className="text-white text-lg">Loading...</div>
      </div>
    );
  }

  return <OnboardingContainer />;
}

// ============================================================================
// EXAMPLE 4: With Custom Success Handler
// ============================================================================

// File: app/onboarding/page.tsx
'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import {
  OnboardingContainer,
  WelcomeScreen,
  SecurityScreen,
  CreateWalletScreen,
  BackupPhraseScreen,
  VerifyPhraseScreen,
  SetupBiometricsScreen,
  FeatureTourScreen,
  CompleteScreen,
  ProgressIndicator,
} from '@/components/onboarding';

const TOTAL_STEPS = 8;

export default function CustomOnboardingPage() {
  const router = useRouter();
  const [currentStep, setCurrentStep] = useState(1);
  const [walletData, setWalletData] = useState<any>({});

  const nextStep = (data?: any) => {
    if (data) {
      setWalletData({ ...walletData, ...data });
    }

    // If completing the last step, save wallet and redirect
    if (currentStep === TOTAL_STEPS) {
      handleOnboardingComplete();
    } else {
      setCurrentStep((prev) => Math.min(prev + 1, TOTAL_STEPS));
    }
  };

  const prevStep = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1));
  };

  const handleOnboardingComplete = async () => {
    try {
      // Save wallet data to secure storage
      localStorage.setItem('etrid_wallet_exists', 'true');

      // Send analytics event
      if (typeof window !== 'undefined' && (window as any).analytics) {
        (window as any).analytics.track('Onboarding Completed', {
          timestamp: new Date().toISOString(),
          biometricsEnabled: walletData.biometricsEnabled || false,
          tourCompleted: walletData.tourCompleted || false,
        });
      }

      // Redirect to home after short delay
      setTimeout(() => {
        router.push('/?onboarding=success');
      }, 2000);
    } catch (error) {
      console.error('Error completing onboarding:', error);
      // Handle error appropriately
    }
  };

  const renderStep = () => {
    switch (currentStep) {
      case 1:
        return <WelcomeScreen onNext={nextStep} />;
      case 2:
        return <SecurityScreen onNext={nextStep} onBack={prevStep} />;
      case 3:
        return <CreateWalletScreen onNext={nextStep} onBack={prevStep} />;
      case 4:
        return <BackupPhraseScreen onNext={nextStep} onBack={prevStep} seedPhrase={walletData.seedPhrase} />;
      case 5:
        return <VerifyPhraseScreen onNext={nextStep} onBack={prevStep} seedPhrase={walletData.seedPhrase} />;
      case 6:
        return <SetupBiometricsScreen onNext={nextStep} onSkip={nextStep} onBack={prevStep} />;
      case 7:
        return <FeatureTourScreen onNext={nextStep} onSkip={nextStep} onBack={prevStep} />;
      case 8:
        return <CompleteScreen />;
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-[#1a0033] to-[#4a0080] flex flex-col">
      <div className="flex-1 flex items-center justify-center p-4">
        <div className="w-full max-w-md">
          {renderStep()}
        </div>
      </div>

      {currentStep < TOTAL_STEPS && (
        <div className="p-4">
          <ProgressIndicator current={currentStep} total={TOTAL_STEPS} />
        </div>
      )}
    </div>
  );
}

// ============================================================================
// EXAMPLE 5: With State Persistence (Resume Onboarding)
// ============================================================================

// File: app/onboarding/page.tsx
'use client';

import { useState, useEffect } from 'react';
import { OnboardingContainer } from '@/components/onboarding';

export default function ResumableOnboardingPage() {
  const [isHydrated, setIsHydrated] = useState(false);

  useEffect(() => {
    // Restore onboarding progress from localStorage
    const savedProgress = localStorage.getItem('onboarding_progress');
    if (savedProgress) {
      // Resume from saved state
      console.log('Resuming onboarding from step:', savedProgress);
    }
    setIsHydrated(true);
  }, []);

  if (!isHydrated) {
    return null; // or loading spinner
  }

  return <OnboardingContainer />;
}

// ============================================================================
// EXAMPLE 6: With Analytics Tracking
// ============================================================================

// File: app/onboarding/page.tsx
'use client';

import { OnboardingContainer } from '@/components/onboarding';
import { useEffect } from 'react';

export default function TrackedOnboardingPage() {
  useEffect(() => {
    // Track page view
    if (typeof window !== 'undefined' && (window as any).analytics) {
      (window as any).analytics.page('Onboarding Started');
    }
  }, []);

  return <OnboardingContainer />;
}

// ============================================================================
// EXAMPLE 7: Mobile App Integration (React Native)
// ============================================================================

// File: screens/OnboardingScreen.tsx (React Native)
/*
import React from 'react';
import { SafeAreaView } from 'react-native';
import { OnboardingContainer } from '@/components/onboarding';

export default function OnboardingScreen() {
  return (
    <SafeAreaView style={{ flex: 1 }}>
      <OnboardingContainer />
    </SafeAreaView>
  );
}
*/

// ============================================================================
// NOTES
// ============================================================================

/**
 * STYLING:
 * The onboarding components use Tailwind CSS classes that are compatible
 * with Next.js. Make sure your globals.css includes the onboarding animations.
 *
 * ROUTING:
 * After onboarding completes, users are typically redirected to the home page.
 * Customize the CompleteScreen component to change this behavior.
 *
 * SECURITY:
 * In production, replace the mock seed phrase generation with a proper
 * BIP39 implementation and secure storage mechanism.
 *
 * STATE MANAGEMENT:
 * For complex apps, consider using a state management solution (Redux, Zustand)
 * to manage onboarding state across the application.
 *
 * TESTING:
 * Write tests for each step to ensure smooth user experience.
 * Use Playwright or Cypress for E2E testing.
 */
