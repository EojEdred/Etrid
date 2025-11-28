# PWA Beta Testing Program

Progressive Web App beta testing with staged rollouts, feature flags, and A/B testing.

## Beta URL Structure

### Environment URLs

```
Production:  https://wallet.etrid.com
Beta:        https://beta.wallet.etrid.com
Staging:     https://staging.wallet.etrid.com
Dev:         https://dev.wallet.etrid.com
```

### URL Configuration

**Vercel Configuration** (`vercel.json`):
```json
{
  "git": {
    "deploymentEnabled": {
      "main": true,
      "develop": true,
      "staging": true,
      "beta": true
    }
  },
  "regions": ["sfo1", "syd1"],
  "env": {
    "NEXT_PUBLIC_ENV": "production"
  },
  "build": {
    "env": {
      "NEXT_PUBLIC_VERCEL_ENV": "@vercel-env"
    }
  }
}
```

### Branch-to-URL Mapping

| Branch | URL | Purpose |
|--------|-----|---------|
| `main` | wallet.etrid.com | Production |
| `beta` | beta.wallet.etrid.com | Beta testing |
| `staging` | staging.wallet.etrid.com | Pre-production |
| `develop` | dev.wallet.etrid.com | Development |
| PR branches | `*.vercel.app` | Preview deploys |

## Vercel Preview Deployments

### Automatic Preview URLs

Every PR automatically creates a preview deployment:

**Format:**
```
https://etrid-wallet-git-[branch]-[team].vercel.app
https://etrid-wallet-[commit-hash].vercel.app
```

**Example:**
```
PR #123 → feature/au-bloccard
Preview: https://etrid-wallet-git-feature-au-bloccard-etrid.vercel.app
```

### Preview Deployment Workflow

1. **Create PR** → Automatic deployment triggered
2. **Vercel bot** comments with preview URL
3. **Test on preview** → Isolated environment
4. **Iterate** → Each commit creates new preview
5. **Merge** → Preview deployed to target branch URL

### Configuration for Preview Deploys

**`.github/workflows/preview-comment.yml`:**
```yaml
name: Preview Deployment Comment

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  comment:
    runs-on: ubuntu-latest
    steps:
      - name: Comment Preview URL
        uses: actions/github-script@v6
        with:
          script: |
            const pr = context.payload.pull_request;
            const branch = pr.head.ref;
            const previewUrl = `https://etrid-wallet-git-${branch}-etrid.vercel.app`;

            github.rest.issues.createComment({
              issue_number: pr.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: `## 🚀 Preview Deployment

              Your preview deployment is ready!

              🔗 **Preview URL:** ${previewUrl}

              ### Test Checklist
              - [ ] Wallet creation works
              - [ ] Transactions functional
              - [ ] UI looks correct
              - [ ] No console errors
              - [ ] Mobile responsive

              This preview updates automatically with each commit.`
            });
```

## Feature Flags System

### Feature Flags Implementation

**`lib/featureFlags.ts`:**
```typescript
/**
 * Feature Flags Configuration
 *
 * Controls gradual rollout of new features to users
 */

export interface FeatureFlag {
  enabled: boolean;
  beta: boolean;
  rolloutPercentage: number;
  requiredRole?: 'admin' | 'beta' | 'user';
  startDate?: Date;
  endDate?: Date;
  description: string;
}

export const featureFlags: Record<string, FeatureFlag> = {
  // AU Bloccard
  auBloccard: {
    enabled: process.env.NEXT_PUBLIC_FEATURE_BLOCCARD === 'true',
    beta: true,
    rolloutPercentage: 50, // 50% of users
    description: 'Crypto debit card feature',
  },

  // NFT Marketplace
  nftMarketplace: {
    enabled: process.env.NEXT_PUBLIC_FEATURE_NFT_MARKETPLACE === 'true',
    beta: true,
    rolloutPercentage: 25,
    description: 'Buy/sell NFTs in marketplace',
  },

  // Advanced Trading
  advancedTrading: {
    enabled: process.env.NEXT_PUBLIC_FEATURE_ADVANCED_TRADING === 'true',
    beta: true,
    rolloutPercentage: 10,
    requiredRole: 'beta',
    description: 'Advanced trading features (limit orders, stop loss)',
  },

  // Hardware Wallet
  hardwareWallet: {
    enabled: process.env.NEXT_PUBLIC_FEATURE_HARDWARE_WALLET === 'true',
    beta: true,
    rolloutPercentage: 100,
    description: 'Ledger/Trezor support',
  },

  // Multi-signature Wallets
  multiSig: {
    enabled: false,
    beta: true,
    rolloutPercentage: 5,
    requiredRole: 'beta',
    description: 'Multi-signature wallet support',
  },

  // Social Recovery
  socialRecovery: {
    enabled: false,
    beta: true,
    rolloutPercentage: 0,
    description: 'Social recovery for lost seed phrases',
  },

  // DeFi Dashboard v2
  defiDashboardV2: {
    enabled: process.env.NEXT_PUBLIC_FEATURE_DEFI_V2 === 'true',
    beta: true,
    rolloutPercentage: 30,
    description: 'New DeFi dashboard with improved UX',
  },

  // Push Notifications
  pushNotifications: {
    enabled: true,
    beta: false,
    rolloutPercentage: 100,
    description: 'Transaction and price alerts',
  },

  // Dark Mode
  darkMode: {
    enabled: true,
    beta: false,
    rolloutPercentage: 100,
    description: 'Dark theme support',
  },
};

/**
 * Check if feature is enabled for user
 * @param feature Feature flag name
 * @param userId User ID for consistent rollout
 * @param userRole User role (admin, beta, user)
 * @returns Whether feature is enabled for this user
 */
export function isFeatureEnabled(
  feature: keyof typeof featureFlags,
  userId: string,
  userRole: 'admin' | 'beta' | 'user' = 'user'
): boolean {
  const flag = featureFlags[feature];

  // Feature not enabled globally
  if (!flag.enabled) return false;

  // Check role requirement
  if (flag.requiredRole) {
    if (userRole === 'admin') return true;
    if (flag.requiredRole === 'beta' && userRole !== 'beta') return false;
  }

  // Check date range
  if (flag.startDate && new Date() < flag.startDate) return false;
  if (flag.endDate && new Date() > flag.endDate) return false;

  // Admins see all enabled features
  if (userRole === 'admin') return true;

  // Percentage-based rollout (consistent for same user)
  const hash = hashUserId(userId);
  const userPercentile = hash % 100;
  return userPercentile < flag.rolloutPercentage;
}

/**
 * Get all enabled features for user
 */
export function getEnabledFeatures(
  userId: string,
  userRole: 'admin' | 'beta' | 'user' = 'user'
): string[] {
  return Object.keys(featureFlags).filter((feature) =>
    isFeatureEnabled(feature as keyof typeof featureFlags, userId, userRole)
  );
}

/**
 * Simple hash function for consistent user distribution
 */
function hashUserId(userId: string): number {
  let hash = 0;
  for (let i = 0; i < userId.length; i++) {
    const char = userId.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash);
}

/**
 * Override feature flags (for testing)
 */
export function overrideFeatureFlag(
  feature: keyof typeof featureFlags,
  enabled: boolean
) {
  if (process.env.NODE_ENV === 'development') {
    featureFlags[feature].enabled = enabled;
  }
}
```

### Feature Flag React Hook

**`hooks/useFeatureFlag.ts`:**
```typescript
'use client';

import { useEffect, useState } from 'react';
import { isFeatureEnabled } from '@/lib/featureFlags';
import { useUser } from '@/hooks/useUser';

/**
 * Hook to check if feature is enabled for current user
 */
export function useFeatureFlag(feature: string): boolean {
  const { user } = useUser();
  const [isEnabled, setIsEnabled] = useState(false);

  useEffect(() => {
    if (user?.id) {
      const enabled = isFeatureEnabled(
        feature,
        user.id,
        user.role || 'user'
      );
      setIsEnabled(enabled);
    }
  }, [feature, user]);

  return isEnabled;
}

/**
 * Hook to get all enabled features
 */
export function useEnabledFeatures(): string[] {
  const { user } = useUser();
  const [features, setFeatures] = useState<string[]>([]);

  useEffect(() => {
    if (user?.id) {
      // Fetch from API to get server-side feature flags
      fetch('/api/features', {
        headers: {
          'user-id': user.id,
        },
      })
        .then((res) => res.json())
        .then((data) => setFeatures(data.features));
    }
  }, [user]);

  return features;
}
```

### Usage in Components

**Example: Conditional Feature Rendering**
```typescript
'use client';

import { useFeatureFlag } from '@/hooks/useFeatureFlag';

export function WalletDashboard() {
  const hasBlockcard = useFeatureFlag('auBloccard');
  const hasAdvancedTrading = useFeatureFlag('advancedTrading');
  const hasNFTMarketplace = useFeatureFlag('nftMarketplace');

  return (
    <div>
      <h1>Wallet Dashboard</h1>

      {/* Always visible */}
      <WalletBalance />
      <TransactionHistory />

      {/* Conditional features */}
      {hasBlockcard && (
        <section>
          <h2>AU Bloccard</h2>
          <BlockcardWidget />
        </section>
      )}

      {hasAdvancedTrading && (
        <section>
          <h2>Advanced Trading</h2>
          <TradingInterface />
        </section>
      )}

      {hasNFTMarketplace && (
        <section>
          <h2>NFT Marketplace</h2>
          <NFTMarketplace />
        </section>
      )}
    </div>
  );
}
```

### Feature Flag API Endpoint

**`app/api/features/route.ts`:**
```typescript
import { NextRequest, NextResponse } from 'next/server';
import { getEnabledFeatures } from '@/lib/featureFlags';
import { getUserRole } from '@/lib/auth';

export async function GET(request: NextRequest) {
  const userId = request.headers.get('user-id');

  if (!userId) {
    return NextResponse.json({ error: 'User ID required' }, { status: 400 });
  }

  const userRole = await getUserRole(userId);
  const features = getEnabledFeatures(userId, userRole);

  return NextResponse.json({
    features,
    timestamp: new Date().toISOString(),
  });
}
```

## Beta Access Methods

### Method 1: Invite-Only Beta

**Middleware** (`middleware.ts`):
```typescript
import { NextRequest, NextResponse } from 'next/server';

export function middleware(request: NextRequest) {
  // Check if accessing beta environment
  if (request.nextUrl.hostname === 'beta.wallet.etrid.com') {
    const betaToken = request.cookies.get('beta_token')?.value;
    const validTokens = process.env.BETA_ACCESS_TOKENS?.split(',') || [];

    // Public paths (don't require beta access)
    const publicPaths = ['/beta/signup', '/beta/access-denied', '/api/beta'];
    const isPublicPath = publicPaths.some((path) =>
      request.nextUrl.pathname.startsWith(path)
    );

    if (!isPublicPath && (!betaToken || !validTokens.includes(betaToken))) {
      return NextResponse.redirect(
        new URL('/beta/access-denied', request.url)
      );
    }
  }

  return NextResponse.next();
}

export const config = {
  matcher: [
    '/((?!_next/static|_next/image|favicon.ico).*)',
  ],
};
```

**Beta Access Page** (`app/beta/signup/page.tsx`):
```typescript
'use client';

import { useState } from 'react';

export default function BetaSignup() {
  const [email, setEmail] = useState('');
  const [status, setStatus] = useState<'idle' | 'loading' | 'success' | 'error'>('idle');

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setStatus('loading');

    try {
      const response = await fetch('/api/beta/signup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email }),
      });

      if (response.ok) {
        setStatus('success');
      } else {
        setStatus('error');
      }
    } catch (error) {
      setStatus('error');
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-600 to-blue-600">
      <div className="bg-white rounded-lg shadow-xl p-8 max-w-md w-full">
        <h1 className="text-3xl font-bold mb-4">Join the Beta</h1>
        <p className="text-gray-600 mb-6">
          Test new features before everyone else
        </p>

        {status === 'success' ? (
          <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded">
            Thanks! Check your email for beta access instructions.
          </div>
        ) : (
          <form onSubmit={handleSubmit}>
            <input
              type="email"
              placeholder="your@email.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              required
              className="w-full px-4 py-2 border border-gray-300 rounded-lg mb-4"
            />
            <button
              type="submit"
              disabled={status === 'loading'}
              className="w-full bg-purple-600 text-white py-2 rounded-lg hover:bg-purple-700 disabled:opacity-50"
            >
              {status === 'loading' ? 'Requesting...' : 'Request Access'}
            </button>
          </form>
        )}

        {status === 'error' && (
          <div className="mt-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
            Something went wrong. Please try again.
          </div>
        )}
      </div>
    </div>
  );
}
```

**Beta Signup API** (`app/api/beta/signup/route.ts`):
```typescript
import { NextRequest, NextResponse } from 'next/server';
import { z } from 'zod';

const signupSchema = z.object({
  email: z.string().email(),
});

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { email } = signupSchema.parse(body);

    // Add to waitlist (database or email service)
    await addToWaitlist(email);

    // Send welcome email with beta access
    await sendBetaAccessEmail(email);

    return NextResponse.json({ success: true });
  } catch (error) {
    return NextResponse.json(
      { error: 'Invalid request' },
      { status: 400 }
    );
  }
}

async function addToWaitlist(email: string) {
  // Implementation: Add to database or email service
  console.log('Added to waitlist:', email);
}

async function sendBetaAccessEmail(email: string) {
  // Generate unique beta token
  const token = generateBetaToken();

  // Send email with access link
  const accessUrl = `https://beta.wallet.etrid.com/beta/activate?token=${token}`;

  // Implementation: Send via SendGrid, Resend, etc.
  console.log('Beta access email sent:', email, accessUrl);
}

function generateBetaToken(): string {
  return Math.random().toString(36).substring(2, 15);
}
```

### Method 2: Email Whitelist

**`lib/betaAccess.ts`:**
```typescript
/**
 * Check if email is whitelisted for beta access
 */
export async function checkBetaAccess(email: string): Promise<boolean> {
  try {
    const response = await fetch('/api/beta/whitelist', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email }),
    });

    const data = await response.json();
    return data.hasAccess;
  } catch (error) {
    return false;
  }
}

/**
 * Add email to whitelist
 */
export async function addToBetaWhitelist(email: string): Promise<void> {
  // Implementation: Add to database
}

/**
 * Remove email from whitelist
 */
export async function removeFromBetaWhitelist(email: string): Promise<void> {
  // Implementation: Remove from database
}
```

### Method 3: Public Beta Link

**Anyone with link can access:**
```
https://beta.wallet.etrid.com?access=public-beta-2024
```

**Middleware check:**
```typescript
export function middleware(request: NextRequest) {
  const accessCode = request.nextUrl.searchParams.get('access');

  if (accessCode === 'public-beta-2024') {
    // Set beta access cookie
    const response = NextResponse.next();
    response.cookies.set('beta_access', 'true', {
      httpOnly: true,
      secure: true,
      maxAge: 60 * 60 * 24 * 30, // 30 days
    });
    return response;
  }

  // Check existing cookie
  const hasBetaAccess = request.cookies.get('beta_access')?.value === 'true';
  if (!hasBetaAccess) {
    return NextResponse.redirect(new URL('/beta/access-denied', request.url));
  }

  return NextResponse.next();
}
```

## Beta Tester Recruitment

### Landing Page CTA

**`components/BetaSignupCTA.tsx`:**
```typescript
'use client';

export function BetaSignupCTA() {
  return (
    <section className="bg-gradient-to-r from-purple-600 to-blue-600 py-20">
      <div className="container mx-auto px-4 text-center text-white">
        <h2 className="text-4xl font-bold mb-4">Join the Beta</h2>
        <p className="text-xl mb-8">
          Test new features before everyone else and shape the future of Ëtrid Wallet
        </p>

        <div className="flex flex-col md:flex-row gap-4 justify-center items-center max-w-md mx-auto">
          <input
            type="email"
            placeholder="your@email.com"
            className="px-6 py-3 rounded-lg text-gray-900 w-full md:flex-1"
          />
          <button className="bg-white text-purple-600 px-8 py-3 rounded-lg font-semibold hover:bg-gray-100 transition w-full md:w-auto">
            Request Access
          </button>
        </div>

        <p className="mt-4 text-sm opacity-80">
          Join 5,000+ beta testers • Available on iOS, Android, and Web
        </p>
      </div>
    </section>
  );
}
```

### Social Media Campaigns

**Twitter/X:**
```
🎉 Ëtrid Wallet Beta is LIVE!

Test the future of DeFi:
✅ Crypto debit card
✅ Advanced trading
✅ NFT marketplace
✅ Multi-chain support

Join 10,000 beta testers → [link]

#DeFi #Crypto #Web3
```

**Discord:**
```
📢 Beta Testing Announcement

React with 🧪 to get access to beta.wallet.etrid.com

Features to test:
• AU Bloccard (crypto debit card)
• Trading features
• NFT gallery
• New UI improvements

Report bugs in #beta-testing
```

**Reddit:**
```
Title: [Beta] We're testing our new DeFi wallet - Looking for testers!

Hi r/CryptoCurrency,

We're looking for beta testers for Ëtrid Wallet, a complete DeFi wallet with:
- Crypto debit card integration
- Built-in DEX trading
- NFT support
- Multi-chain (ETH, Polygon, BSC)

Looking for feedback on UX, features, and any bugs you find.

Interested? Drop a comment or DM for access.

(Not asking for funds/keys, just UI/UX testing)
```

### Email Campaign

**Subject:** You're invited to test Ëtrid Wallet Beta

**Body:**
```html
<div style="max-width: 600px; margin: 0 auto; font-family: sans-serif;">
  <h1>You're invited to our Beta!</h1>

  <p>We're excited to invite you to test the new Ëtrid Wallet before anyone else.</p>

  <h2>What's New:</h2>
  <ul>
    <li>🎴 AU Bloccard - Crypto debit card</li>
    <li>📈 Advanced trading features</li>
    <li>🖼️ NFT marketplace</li>
    <li>🔗 Multi-chain support</li>
  </ul>

  <a href="https://beta.wallet.etrid.com?invite=XXXXX"
     style="display: inline-block; background: #7c3aed; color: white; padding: 12px 24px; text-decoration: none; border-radius: 8px; margin: 20px 0;">
    Access Beta
  </a>

  <p>We'd love your feedback on what works, what doesn't, and what features you'd like to see.</p>

  <p>
    <strong>How to report bugs:</strong><br>
    Email: beta@etrid.com<br>
    Discord: #beta-testing<br>
    In-app: Settings → Report Bug
  </p>

  <p>Thanks for being part of our community!</p>

  <p>— The Ëtrid Team</p>
</div>
```

## Feedback Collection

### In-App Feedback Widget

**`components/BetaFeedback.tsx`:**
```typescript
'use client';

import { useState } from 'react';
import { X, Bug, Lightbulb } from 'lucide-react';

export function BetaFeedback() {
  const [isOpen, setIsOpen] = useState(false);
  const [type, setType] = useState<'bug' | 'feature'>('bug');
  const [feedback, setFeedback] = useState('');
  const [submitted, setSubmitted] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    await fetch('/api/feedback', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        type,
        feedback,
        url: window.location.href,
        userAgent: navigator.userAgent,
        timestamp: new Date().toISOString(),
      }),
    });

    setSubmitted(true);
    setTimeout(() => {
      setIsOpen(false);
      setSubmitted(false);
      setFeedback('');
    }, 2000);
  }

  return (
    <>
      {/* Floating button */}
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 z-50 bg-purple-500 text-white p-4 rounded-full shadow-lg hover:bg-purple-600 transition"
        title="Send Feedback"
      >
        <Bug size={24} />
      </button>

      {/* Modal */}
      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
          <div className="bg-white rounded-lg shadow-xl p-6 max-w-md w-full mx-4">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-2xl font-bold">Beta Feedback</h2>
              <button
                onClick={() => setIsOpen(false)}
                className="text-gray-500 hover:text-gray-700"
              >
                <X size={24} />
              </button>
            </div>

            {submitted ? (
              <div className="text-center py-8">
                <div className="text-green-500 text-6xl mb-4">✓</div>
                <p className="text-xl font-semibold">Thank you!</p>
                <p className="text-gray-600">Your feedback has been submitted.</p>
              </div>
            ) : (
              <form onSubmit={handleSubmit}>
                <div className="mb-4">
                  <label className="block text-sm font-medium mb-2">
                    Feedback Type
                  </label>
                  <div className="flex gap-4">
                    <button
                      type="button"
                      onClick={() => setType('bug')}
                      className={`flex-1 p-3 rounded-lg border-2 ${
                        type === 'bug'
                          ? 'border-purple-500 bg-purple-50'
                          : 'border-gray-300'
                      }`}
                    >
                      <Bug className="mx-auto mb-1" size={24} />
                      Bug Report
                    </button>
                    <button
                      type="button"
                      onClick={() => setType('feature')}
                      className={`flex-1 p-3 rounded-lg border-2 ${
                        type === 'feature'
                          ? 'border-purple-500 bg-purple-50'
                          : 'border-gray-300'
                      }`}
                    >
                      <Lightbulb className="mx-auto mb-1" size={24} />
                      Feature Request
                    </button>
                  </div>
                </div>

                <div className="mb-4">
                  <label className="block text-sm font-medium mb-2">
                    {type === 'bug' ? 'Describe the bug' : 'Describe your idea'}
                  </label>
                  <textarea
                    value={feedback}
                    onChange={(e) => setFeedback(e.target.value)}
                    required
                    rows={5}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                    placeholder={
                      type === 'bug'
                        ? 'What happened? What did you expect to happen?'
                        : 'What feature would you like to see?'
                    }
                  />
                </div>

                <button
                  type="submit"
                  className="w-full bg-purple-600 text-white py-2 rounded-lg hover:bg-purple-700 transition"
                >
                  Submit Feedback
                </button>
              </form>
            )}
          </div>
        </div>
      )}
    </>
  );
}
```

### Automated Surveys

**`lib/surveys.ts`:**
```typescript
/**
 * Trigger surveys based on user behavior
 */

export function shouldShowSurvey(
  userId: string,
  surveyType: 'onboarding' | 'feature' | 'nps' | 'exit'
): boolean {
  const lastSurvey = localStorage.getItem(`survey_${surveyType}_${userId}`);

  if (!lastSurvey) return true;

  const daysSinceLastSurvey =
    (Date.now() - parseInt(lastSurvey)) / (1000 * 60 * 60 * 24);

  // Survey frequency rules
  switch (surveyType) {
    case 'onboarding':
      return daysSinceLastSurvey > 7; // Week after signup
    case 'feature':
      return daysSinceLastSurvey > 14; // After using feature
    case 'nps':
      return daysSinceLastSurvey > 30; // Monthly
    case 'exit':
      return true; // Always show on exit intent
  }
}

export function markSurveyShown(userId: string, surveyType: string) {
  localStorage.setItem(
    `survey_${surveyType}_${userId}`,
    Date.now().toString()
  );
}
```

### Analytics Events

**`lib/analytics.ts`:**
```typescript
/**
 * Track everything for beta testing
 */

export const analytics = {
  // Feature usage
  trackFeatureUsed(feature: string, metadata?: Record<string, any>) {
    track('feature_used', {
      feature,
      ...metadata,
      environment: process.env.NEXT_PUBLIC_ENV,
    });
  },

  // Errors
  trackError(error: Error, context?: Record<string, any>) {
    track('error_occurred', {
      error: error.message,
      stack: error.stack,
      ...context,
    });
  },

  // User flows
  trackFlow(flowName: string, step: string, completed: boolean) {
    track('user_flow', {
      flow: flowName,
      step,
      completed,
    });
  },

  // Performance
  trackPerformance(metric: string, value: number) {
    track('performance_metric', {
      metric,
      value,
    });
  },

  // Feedback
  trackFeedback(type: 'bug' | 'feature' | 'general', content: string) {
    track('feedback_submitted', {
      type,
      content_length: content.length,
    });
  },
};

function track(event: string, properties: Record<string, any>) {
  // Send to analytics service (Mixpanel, Amplitude, etc.)
  if (typeof window !== 'undefined' && (window as any).analytics) {
    (window as any).analytics.track(event, properties);
  }
}
```

## Rollout Plan

### Phase 1: Closed Beta (Weeks 1-2)
**Target: 100 users**

- Internal team (10)
- Friends & family (20)
- Early supporters (70)
- **Rollout:** 100% to whitelist
- **Updates:** Daily
- **Focus:** Critical bugs, UX feedback

### Phase 2: Private Beta (Weeks 3-4)
**Target: 1,000 users**

- Email whitelist expanded
- Discord community members
- Twitter followers
- **Rollout:** 100% to whitelist
- **Updates:** 2-3x per week
- **Focus:** Feature refinement, performance

### Phase 3: Public Beta (Weeks 5-6)
**Target: 10,000 users**

- Public signup link
- Landing page promotion
- Social media campaign
- **Rollout:** Gradual (feature flags)
- **Updates:** Weekly
- **Focus:** Scalability, edge cases

### Phase 4: Soft Launch (Weeks 7-8)
**Target: 100,000 users**

- Feature flags at 10% → 25% → 50% → 100%
- Monitor metrics closely
- **Rollout:** Staged by feature
- **Updates:** Bi-weekly
- **Focus:** Stability, performance at scale

### Phase 5: General Availability (Week 9+)
**Target: Unlimited**

- 100% rollout all features
- Marketing campaign
- Press release
- App store launch
- **Updates:** Monthly (with hotfixes as needed)

## Monitoring & Metrics

### Key Metrics Dashboard

**Critical Metrics:**
```typescript
export interface BetaMetrics {
  // Quality
  errorRate: number;          // Target: <1%
  crashRate: number;          // Target: <0.1%
  apiErrorRate: number;       // Target: <2%

  // Performance
  pageLoadTime: number;       // Target: <2s
  apiResponseTime: number;    // Target: <500ms
  timeToInteractive: number;  // Target: <3s

  // Engagement
  dailyActiveUsers: number;
  weeklyActiveUsers: number;
  avgSessionLength: number;   // Target: >3min
  sessionsPerUser: number;    // Target: >2

  // Retention
  d1Retention: number;        // Target: >40%
  d7Retention: number;        // Target: >20%
  d30Retention: number;       // Target: >10%

  // Feedback
  feedbackCount: number;
  bugReports: number;
  featureRequests: number;
  npsScore: number;           // Target: >50
}
```

### Real-Time Monitoring

**Vercel Analytics:**
- Real-time traffic
- Geographic distribution
- Device breakdown
- Error tracking

**Sentry:**
- Error tracking
- Performance monitoring
- Release health
- User feedback

**Mixpanel/Amplitude:**
- User behavior
- Funnel analysis
- Cohort analysis
- Retention curves

## Resources

- [Vercel Deployment Docs](https://vercel.com/docs)
- [Feature Flags Guide](https://martinfowler.com/articles/feature-toggles.html)
- [PWA Best Practices](https://web.dev/pwa/)
- [Beta Testing Guide](https://www.nngroup.com/articles/beta-testing/)

## Support

Questions? Contact:
- Email: dev@etrid.com
- Discord: #web-development
- Internal Wiki: https://wiki.etrid.com/pwa-beta
