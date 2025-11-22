# Ëtrid Wallet Onboarding Flow Diagram

## Visual Flow

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        ONBOARDING CONTAINER                             │
│                     (State Management & Navigation)                      │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
        ┌───────────────────────────────────────────────────┐
        │                                                   │
        │              STEP 1: WELCOME                      │
        │                                                   │
        │  • Ëtrid logo & branding                         │
        │  • Three key benefits                            │
        │  • Get Started CTA                               │
        │  • Terms & Privacy links                         │
        │                                                   │
        │              [Get Started] ──────────────────┐   │
        │                                               │   │
        └───────────────────────────────────────────────┘   │
                                                            ▼
        ┌───────────────────────────────────────────────────────┐
        │                                                       │
        │           STEP 2: SECURITY EDUCATION                  │
        │                                                       │
        │  • What you need to know                             │
        │  • Key concepts explained                            │
        │  • DO's and DON'Ts                                   │
        │  • Security best practices                           │
        │                                                       │
        │      [Back]          [I Understand] ────────────┐    │
        │                                                  │    │
        └──────────────────────────────────────────────────┘    │
                                                                ▼
        ┌───────────────────────────────────────────────────────────┐
        │                                                           │
        │            STEP 3: CREATE/IMPORT WALLET                   │
        │                                                           │
        │  Option A: [Create New Wallet]                           │
        │    └──> Generate 12-word seed phrase ──────────────┐     │
        │                                                     │     │
        │  Option B: [Import Existing Wallet]                │     │
        │    └──> Validate 12/24-word phrase ─────────────┐  │     │
        │                                                  │  │     │
        │      [Back]                    [Continue] ────┐  │  │     │
        │                                               │  │  │     │
        └───────────────────────────────────────────────┘  │  │     │
                                                           │  │     │
                                                           ▼  ▼     ▼
        ┌──────────────────────────────────────────────────────────────┐
        │                                                              │
        │              STEP 4: BACKUP SEED PHRASE                      │
        │                                                              │
        │  • Security warnings (red box)                              │
        │  • Blurred phrase display                                   │
        │  • [Reveal Phrase] button                                   │
        │  • 12 words in 3x4 grid                                     │
        │  • [Copy to Clipboard]                                      │
        │  • ☑ Confirmation checkbox                                  │
        │                                                              │
        │      [Back]          [Continue] ─────────────────┐          │
        │                                                   │          │
        └───────────────────────────────────────────────────┘          │
                                                                       ▼
        ┌──────────────────────────────────────────────────────────────┐
        │                                                              │
        │           STEP 5: VERIFY BACKUP (QUIZ)                       │
        │                                                              │
        │  • Select words at positions #3, #7, #11                    │
        │  • 3 empty slots to fill                                    │
        │  • 9 shuffled word options                                  │
        │  • Tap to select words                                      │
        │  • [Verify] button                                          │
        │  • 3 attempts maximum                                       │
        │  • ✓ Success or ✗ Error feedback                           │
        │                                                              │
        │      [Back]          [Verify] ──────────────────┐           │
        │                                                  │           │
        └──────────────────────────────────────────────────┘           │
                                                                       ▼
        ┌──────────────────────────────────────────────────────────────┐
        │                                                              │
        │           STEP 6: BIOMETRICS (OPTIONAL)                      │
        │                                                              │
        │  • Benefits explanation                                      │
        │  • Touch ID / Face ID icons                                 │
        │  • Security features                                         │
        │  • [Enable Biometrics]                                      │
        │  • OR [Skip for Now]                                        │
        │                                                              │
        │      [Back]   [Skip]   [Enable] ───────────────┐            │
        │                                                 │            │
        └─────────────────────────────────────────────────┘            │
                                                                       ▼
        ┌──────────────────────────────────────────────────────────────┐
        │                                                              │
        │           STEP 7: FEATURE TOUR (OPTIONAL)                    │
        │                                                              │
        │  Slide 1/4: AU Bloccard                                     │
        │  Slide 2/4: Trade & Earn                                    │
        │  Slide 3/4: NFTs & Metaverse                                │
        │  Slide 4/4: Social Features                                 │
        │                                                              │
        │  • Swipeable carousel                                       │
        │  • Progress dots                                            │
        │  • [Previous] [Next]                                        │
        │  • [Skip Tour] option                                       │
        │                                                              │
        │      [Back]   [Skip]   [Finish Tour] ──────────┐            │
        │                                                 │            │
        └─────────────────────────────────────────────────┘            │
                                                                       ▼
        ┌──────────────────────────────────────────────────────────────┐
        │                                                              │
        │              STEP 8: COMPLETION                              │
        │                                                              │
        │  • Success animation                                         │
        │  • ✓ Wallet created                                         │
        │  • ✓ Recovery phrase backed up                              │
        │  • ✓ Biometrics enabled                                     │
        │  • ✓ Features tour completed                                │
        │  • Next steps info                                          │
        │                                                              │
        │         [Start Using Ëtrid] ────────────────┐               │
        │                                             │               │
        └─────────────────────────────────────────────┘               │
                                                                      ▼
                                                            ┌──────────────┐
                                                            │   HOME PAGE  │
                                                            │  (Logged In) │
                                                            └──────────────┘
```

---

## Progress Indicator (Bottom of Screen)

```
┌────────────────────────────────────────────────────────────────┐
│                                                                │
│  [████████████████████░░░░░░░░░░░░░░░░]  70% Complete         │
│                                                                │
│  Step 5 of 8                        ●●●●●○○○                  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## Decision Tree

```
Start
  │
  ▼
Welcome Screen
  │
  ▼
Security Education
  │
  ▼
Create or Import? ─────┐
  │                    │
  ├── New Wallet       │
  │     │              │
  │     ▼              │
  │   Generate         │
  │   12 words         │
  │     │              │
  │     └──────────────┤
  │                    │
  └── Import Wallet    │
        │              │
        ▼              │
      Validate         │
      Input            │
        │              │
        └──────────────┤
                       ▼
                  Backup Phrase
                       │
                       ▼
                  Verify Phrase ────┐
                       │            │
                       ├─ Pass      │
                       │    │       │
                       │    ▼       │
                       │  Continue  │
                       │            │
                       └─ Fail (3x) │
                            │       │
                            ▼       │
                          Go Back ──┘
                                    │
                                    ▼
                            Biometrics? ───┐
                                    │      │
                                    ├─ Enable
                                    │      │
                                    └─ Skip
                                           │
                                           ▼
                                   Feature Tour? ───┐
                                           │        │
                                           ├─ Watch │
                                           │        │
                                           └─ Skip  │
                                                    │
                                                    ▼
                                              Complete!
                                                    │
                                                    ▼
                                                  Home
```

---

## Data Flow

```
┌────────────────┐
│ User Actions   │
└────────┬───────┘
         │
         ▼
┌────────────────────────┐
│ OnboardingContainer    │
│                        │
│  State:                │
│  • currentStep: 1-8    │
│  • walletData: {}      │
│                        │
│  Methods:              │
│  • nextStep(data?)     │
│  • prevStep()          │
│  • renderStep()        │
└────────┬───────────────┘
         │
         ├─────> WelcomeScreen
         │
         ├─────> SecurityScreen
         │
         ├─────> CreateWalletScreen
         │            │
         │            └──> walletData.seedPhrase = "word1 word2..."
         │
         ├─────> BackupPhraseScreen
         │            │
         │            └──> Uses: walletData.seedPhrase
         │
         ├─────> VerifyPhraseScreen
         │            │
         │            └──> Validates: walletData.seedPhrase
         │
         ├─────> SetupBiometricsScreen
         │            │
         │            └──> walletData.biometricsEnabled = true/false
         │
         ├─────> FeatureTourScreen
         │            │
         │            └──> walletData.tourCompleted = true/false
         │
         └─────> CompleteScreen
                      │
                      └──> Save to localStorage
                           Redirect to home
```

---

## Component Dependencies

```
OnboardingContainer
  ├── WelcomeScreen
  │     └── lucide-react (icons)
  │
  ├── SecurityScreen
  │     └── lucide-react (icons)
  │
  ├── CreateWalletScreen
  │     ├── lucide-react (icons)
  │     └── ImportWalletForm
  │
  ├── BackupPhraseScreen
  │     ├── lucide-react (icons)
  │     └── navigator.clipboard API
  │
  ├── VerifyPhraseScreen
  │     ├── lucide-react (icons)
  │     └── Array.sort() for shuffling
  │
  ├── SetupBiometricsScreen
  │     ├── lucide-react (icons)
  │     └── Biometric APIs (future)
  │
  ├── FeatureTourScreen
  │     └── lucide-react (icons)
  │
  ├── CompleteScreen
  │     ├── lucide-react (icons)
  │     └── next/link
  │
  └── ProgressIndicator
        └── Pure CSS progress bar
```

---

## Animation Timeline

```
Screen Load
    │
    ├─ 0ms: Component mounts
    │
    ├─ 0-500ms: fadeIn animation
    │   └─> opacity: 0 → 1
    │   └─> translateY: 10px → 0
    │
    ├─ User Interaction
    │   └─> Button hover: scale(1.05)
    │   └─> Input focus: border color change
    │
    ├─ State Changes
    │   └─> Success: pulse animation
    │   └─> Error: shake animation
    │
    └─ Screen Exit
        └─> Next screen fadeIn begins
```

---

## Error Handling Flow

```
User Action
    │
    ▼
Validation ────┐
    │          │
    ├─ Valid   │
    │   │      │
    │   ▼      │
    │ Success  │
    │   │      │
    │   ▼      │
    │ Next     │
    │ Step     │
    │          │
    └─ Invalid │
        │      │
        ▼      │
    Show Error │
    Message    │
        │      │
        ▼      │
    User Retry │
        │      │
        └──────┘
```

---

## State Management

```
┌─────────────────────────────────────┐
│         OnboardingContainer         │
│                                     │
│  const [currentStep, setCurrentStep]│
│  const [walletData, setWalletData]  │
│                                     │
│  walletData: {                      │
│    mode: 'create' | 'import'        │
│    seedPhrase: string               │
│    biometricsEnabled: boolean       │
│    tourCompleted: boolean           │
│  }                                  │
└─────────────────────────────────────┘
```

---

## Security Checkpoints

```
Checkpoint 1: Security Education
    ↓
Checkpoint 2: Phrase Generation/Import
    ↓
Checkpoint 3: Backup Warning & Confirmation
    ↓
Checkpoint 4: Verification Quiz (3 attempts)
    ↓
Checkpoint 5: Optional Biometrics
    ↓
Completion: User ready to use wallet
```

---

This flow diagram provides a complete visual reference for understanding the onboarding journey!
