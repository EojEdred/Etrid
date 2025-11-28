# Ëtrid Wallet Onboarding Flow

A comprehensive, educational onboarding experience for first-time users of the Ëtrid wallet. This flow guides users through wallet creation, security setup, and feature discovery in an engaging, step-by-step manner.

## Overview

The onboarding flow consists of 8 carefully designed steps that ensure users understand wallet security, properly back up their recovery phrase, and discover key features of the Ëtrid wallet.

## Components

### 1. OnboardingContainer
**File:** `OnboardingContainer.tsx`

Main container that manages the onboarding flow state and navigation between screens.

**Features:**
- Centralized step management
- Data persistence across steps
- Progress tracking
- Responsive layout with gradient background

### 2. WelcomeScreen (Step 1)
**File:** `WelcomeScreen.tsx`

Introduces users to the Ëtrid wallet with a warm welcome and key value propositions.

**Features:**
- Eye-catching hero section with wallet icon
- Three key benefits highlighted:
  - Bank-grade security
  - 18 powerful features
  - Non-custodial control
- Terms of Service and Privacy Policy links

### 3. SecurityScreen (Step 2)
**File:** `SecurityScreen.tsx`

Educates users about wallet security fundamentals and best practices.

**Features:**
- Key concepts explained:
  - Self-custody principles
  - Recovery phrase importance
  - Personal responsibility
- Security best practices with DO/DON'T format
- Visual indicators (green checkmarks, red warnings)

### 4. CreateWalletScreen (Step 3)
**File:** `CreateWalletScreen.tsx`

Allows users to create a new wallet or import an existing one.

**Features:**
- Two paths: Create New or Import Existing
- New wallet: Generates 12-word BIP39 mnemonic
- Import wallet: Validates 12 or 24-word recovery phrases
- Security reminder when importing
- Input validation and error handling

### 5. BackupPhraseScreen (Step 4)
**File:** `BackupPhraseScreen.tsx`

Displays the recovery phrase and ensures users back it up safely.

**Features:**
- Blurred phrase with reveal button
- 12-word grid display with numbered positions
- Copy-to-clipboard functionality
- Critical security warnings
- Confirmation checkbox before proceeding
- Visual feedback (checkmarks, colors)

### 6. VerifyPhraseScreen (Step 5)
**File:** `VerifyPhraseScreen.tsx`

Verifies that users properly backed up their recovery phrase.

**Features:**
- Randomly selects 3 words to verify
- Interactive word selection interface
- Shuffled word options (correct + decoy words)
- 3 attempts maximum
- Instant feedback (success/error states)
- Visual confirmation animations

### 7. SetupBiometricsScreen (Step 6)
**File:** `SetupBiometricsScreen.tsx`

Optional biometric authentication setup for quick access.

**Features:**
- Benefits explanation (security, convenience)
- Support for Touch ID and Face ID
- Skippable step
- Platform-specific biometric integration
- Loading states and success confirmation

### 8. FeatureTourScreen (Step 7)
**File:** `FeatureTourScreen.tsx`

Interactive tour showcasing the wallet's key features.

**Features:**
- 4 feature highlights:
  - AU Bloccard (crypto debit card)
  - Trade & Earn (DeFi features)
  - NFTs & Metaverse
  - Social Features
- Swipeable carousel interface
- Progress dots indicator
- Skippable tour

### 9. CompleteScreen (Step 8)
**File:** `CompleteScreen.tsx`

Celebrates successful onboarding and provides next steps.

**Features:**
- Success animation
- Completion checklist
- Next steps guidance
- Call-to-action to start using the wallet

### 10. ProgressIndicator
**File:** `ProgressIndicator.tsx`

Visual progress indicator shown throughout the flow.

**Features:**
- Animated progress bar
- Step counter (e.g., "Step 3 of 8")
- Percentage complete
- Dot indicators for each step

## Usage

### Basic Implementation

```tsx
import { OnboardingContainer } from '@/components/onboarding';

export default function OnboardingPage() {
  return <OnboardingContainer />;
}
```

### With Next.js App Router

```tsx
// app/onboarding/page.tsx
import { OnboardingContainer } from '@/components/onboarding';

export default function Page() {
  return <OnboardingContainer />;
}
```

### Custom Integration

```tsx
import {
  OnboardingContainer,
  WelcomeScreen,
  SecurityScreen
} from '@/components/onboarding';

// Use individual screens with custom logic
function CustomOnboarding() {
  const [step, setStep] = useState(1);

  return (
    <div>
      {step === 1 && <WelcomeScreen onNext={() => setStep(2)} />}
      {step === 2 && <SecurityScreen onNext={() => setStep(3)} onBack={() => setStep(1)} />}
      {/* ... */}
    </div>
  );
}
```

## Animations

All screens use smooth animations defined in `globals.css`:

- **fadeIn**: Fade in with slight upward motion
- **slideInFromRight**: Slide in from right side
- **slideInFromLeft**: Slide in from left side
- **scaleIn**: Scale up with fade in
- **pulse**: Pulsing opacity animation

## Security Considerations

### Recovery Phrase Handling

1. **Generation**: Uses BIP39 standard for mnemonic generation (mock implementation provided)
2. **Display**: Blurred by default, requires explicit user action to reveal
3. **Validation**: Verifies users actually wrote down the phrase
4. **Storage**: Never stored in plain text, encrypted with user's device security

### Best Practices Implemented

- Multiple security warnings throughout flow
- Copy-paste functionality with user awareness
- Verification step to prevent user error
- Biometric authentication option
- Clear communication about non-custodial nature

## Customization

### Styling

All components use Tailwind CSS with consistent design tokens:

```tsx
// Primary gradient
bg-gradient-to-r from-purple-500 to-blue-500

// Glass morphism
bg-white/5 backdrop-blur-sm border border-white/10

// Success states
bg-green-500/10 border-green-500/30

// Warning states
bg-yellow-500/10 border-yellow-500/30

// Error states
bg-red-500/10 border-red-500/30
```

### Configuration

Modify the onboarding flow in `OnboardingContainer.tsx`:

```tsx
const TOTAL_STEPS = 8; // Change number of steps

// Add/remove steps in renderStep()
const renderStep = () => {
  switch (currentStep) {
    case 1: return <WelcomeScreen onNext={nextStep} />;
    // Add custom steps here
  }
};
```

## Accessibility

- Semantic HTML structure
- Keyboard navigation support
- ARIA labels on interactive elements
- Screen reader friendly
- High contrast colors
- Clear focus indicators

## Future Enhancements

### Planned Features

1. **Multi-language support**: i18n integration
2. **Animations library**: Framer Motion for advanced transitions
3. **Video tutorials**: Embedded video guides for each step
4. **Social recovery**: Guardian-based wallet recovery
5. **Hardware wallet**: Integration with Ledger/Trezor
6. **Analytics**: Track drop-off points to optimize flow
7. **A/B testing**: Test different onboarding approaches

### Integration Points

- **Wallet SDK**: Connect to actual wallet generation library
- **Biometric API**: Platform-specific implementations (iOS/Android)
- **Analytics**: Add tracking events for monitoring
- **Backend**: Store encrypted wallet metadata
- **KYC/AML**: Optional identity verification step

## File Structure

```
components/onboarding/
├── OnboardingContainer.tsx    # Main container
├── WelcomeScreen.tsx          # Step 1
├── SecurityScreen.tsx         # Step 2
├── CreateWalletScreen.tsx     # Step 3
├── BackupPhraseScreen.tsx     # Step 4
├── VerifyPhraseScreen.tsx     # Step 5
├── SetupBiometricsScreen.tsx  # Step 6
├── FeatureTourScreen.tsx      # Step 7
├── CompleteScreen.tsx         # Step 8
├── ProgressIndicator.tsx      # Shared component
├── index.ts                   # Exports
└── README.md                  # This file
```

## Dependencies

- React 18+
- Next.js 14+ (App Router)
- Tailwind CSS 3+
- lucide-react (icons)

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Mobile Support

- iOS 13+
- Android 8+
- Responsive design for all screen sizes
- Touch-optimized interactions

## Testing

### Manual Testing Checklist

- [ ] Complete flow from start to finish
- [ ] Test back navigation at each step
- [ ] Verify seed phrase validation
- [ ] Test biometric skip functionality
- [ ] Check responsive design on mobile
- [ ] Verify animations play smoothly
- [ ] Test error states and validation
- [ ] Check accessibility with screen reader

### Automated Tests (TODO)

```bash
# Unit tests
npm run test:unit

# E2E tests
npm run test:e2e

# Visual regression tests
npm run test:visual
```

## License

Part of the Ëtrid Wallet project. All rights reserved.

## Support

For issues or questions, contact the Ëtrid development team.
