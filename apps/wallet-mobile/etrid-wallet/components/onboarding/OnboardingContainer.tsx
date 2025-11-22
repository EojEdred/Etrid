'use client';

import { useState } from 'react';
import WelcomeScreen from './WelcomeScreen';
import SecurityScreen from './SecurityScreen';
import CreateWalletScreen from './CreateWalletScreen';
import BackupPhraseScreen from './BackupPhraseScreen';
import VerifyPhraseScreen from './VerifyPhraseScreen';
import SetupBiometricsScreen from './SetupBiometricsScreen';
import FeatureTourScreen from './FeatureTourScreen';
import CompleteScreen from './CompleteScreen';
import ProgressIndicator from './ProgressIndicator';

const TOTAL_STEPS = 8;

export default function OnboardingContainer() {
  const [currentStep, setCurrentStep] = useState(1);
  const [walletData, setWalletData] = useState<any>({});

  const nextStep = (data?: any) => {
    if (data) {
      setWalletData({ ...walletData, ...data });
    }
    setCurrentStep((prev) => Math.min(prev + 1, TOTAL_STEPS));
  };

  const prevStep = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1));
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
