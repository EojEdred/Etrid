# Beta Testing Dashboard

Unified dashboard to track beta testing across all platforms (iOS, Android, PWA).

## Overview

The beta dashboard provides real-time insights into:
- User metrics across all platforms
- Quality and performance metrics
- Feature adoption and usage
- Feedback and bug reports
- Rollout progress and health

## Dashboard Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Beta Dashboard                          │
├─────────────────────────────────────────────────────────────┤
│  Platform Overview    │  iOS │ Android │  PWA  │   Total   │
├───────────────────────┴──────┴─────────┴───────┴───────────┤
│                                                               │
│  📊 Key Metrics                                              │
│    - Active Beta Testers                                     │
│    - Crash-Free Rate                                         │
│    - Daily Active Users                                      │
│    - Bug Reports                                             │
│                                                               │
│  📈 Charts & Graphs                                          │
│    - User Growth Over Time                                   │
│    - Feature Usage                                           │
│    - Error Rate Trends                                       │
│    - Retention Curves                                        │
│                                                               │
│  🐛 Recent Issues                                            │
│  💡 Feature Requests                                         │
│  📱 Device Breakdown                                         │
│  🌍 Geographic Distribution                                  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

## Metrics to Track

### User Metrics

#### Acquisition
```typescript
interface AcquisitionMetrics {
  // Total beta testers per platform
  totalTesters: {
    ios: number;
    android: number;
    pwa: number;
    total: number;
  };

  // New testers per day/week/month
  newTesters: {
    daily: number[];
    weekly: number[];
    monthly: number[];
  };

  // Invite acceptance rate
  inviteAcceptanceRate: number; // Target: >60%

  // Platform distribution
  platformDistribution: {
    ios: number;      // percentage
    android: number;
    pwa: number;
  };
}
```

#### Engagement
```typescript
interface EngagementMetrics {
  // Active users
  dau: number;  // Daily Active Users
  wau: number;  // Weekly Active Users
  mau: number;  // Monthly Active Users

  // Stickiness (DAU/MAU ratio)
  stickiness: number; // Target: >20%

  // Session metrics
  avgSessionLength: number; // seconds, Target: >180s
  sessionsPerDay: number;   // Target: >2
  sessionsPerUser: number;

  // Feature usage
  featureUsage: {
    [feature: string]: {
      users: number;
      sessions: number;
      avgTimeSpent: number;
    };
  };
}
```

#### Retention
```typescript
interface RetentionMetrics {
  // Day N retention
  d1Retention: number; // Target: >40%
  d7Retention: number; // Target: >20%
  d30Retention: number; // Target: >10%

  // Cohort retention
  cohortRetention: {
    cohort: string; // e.g., "2024-W01"
    day1: number;
    day7: number;
    day30: number;
  }[];

  // Churn rate
  churnRate: number; // Target: <10%

  // Time to churn
  avgTimeToChurn: number; // days
}
```

### Quality Metrics

#### Stability
```typescript
interface StabilityMetrics {
  // Crash metrics
  crashRate: number;        // Target: <1%
  crashFreeUsers: number;   // Target: >99%
  crashesPerUser: number;   // Target: <0.01

  // Platform-specific
  ios: {
    crashRate: number;
    memoryWarnings: number;
    watchdogEvents: number;
  };

  android: {
    crashRate: number;
    anrRate: number; // Application Not Responding, Target: <0.47%
    excessiveWakeups: number;
  };

  pwa: {
    errorRate: number;
    unhandledRejections: number;
    resourceLoadFailures: number;
  };
}
```

#### Performance
```typescript
interface PerformanceMetrics {
  // Load times
  appStartup: {
    cold: number; // milliseconds, Target: <2000ms
    warm: number; // Target: <1000ms
  };

  // API performance
  apiResponseTime: {
    p50: number; // 50th percentile, Target: <200ms
    p95: number; // 95th percentile, Target: <500ms
    p99: number; // 99th percentile, Target: <1000ms
  };

  // UI performance
  timeToInteractive: number; // Target: <3000ms
  firstContentfulPaint: number; // Target: <1500ms
  largestContentfulPaint: number; // Target: <2500ms

  // Network
  networkErrorRate: number; // Target: <2%
  avgBandwidthUsage: number; // MB per session
}
```

#### Bugs & Issues
```typescript
interface BugMetrics {
  // Bug counts
  totalBugs: number;
  openBugs: number;
  resolvedBugs: number;

  // By severity
  critical: number; // P0
  high: number;     // P1
  medium: number;   // P2
  low: number;      // P3

  // By platform
  byPlatform: {
    ios: number;
    android: number;
    pwa: number;
    all: number; // affects all platforms
  };

  // Resolution metrics
  avgTimeToResolution: number; // hours
  bugBacklog: number;
}
```

### Feature Metrics

#### Feature Adoption
```typescript
interface FeatureAdoptionMetrics {
  features: {
    [feature: string]: {
      // Usage
      totalUsers: number;
      activeUsers: number; // used in last 7 days
      adoptionRate: number; // percentage of all users

      // Engagement
      avgUsagePerUser: number; // times per week
      avgTimeSpent: number; // seconds per session

      // Satisfaction
      rating: number; // 1-5 stars
      feedbackCount: number;

      // Funnel
      started: number;
      completed: number;
      conversionRate: number;
    };
  };

  // Rollout tracking
  rollout: {
    [feature: string]: {
      rolloutPercentage: number;
      usersInRollout: number;
      errorRate: number;
      rollbackTriggered: boolean;
    };
  };
}
```

### Feedback Metrics

```typescript
interface FeedbackMetrics {
  // Volume
  totalFeedback: number;
  bugReports: number;
  featureRequests: number;
  generalFeedback: number;

  // Trends
  feedbackPerUser: number;
  feedbackGrowth: number; // percentage change

  // Sentiment
  positive: number;
  neutral: number;
  negative: number;

  // NPS (Net Promoter Score)
  npsScore: number; // Target: >50
  promoters: number; // 9-10 rating
  passives: number;  // 7-8 rating
  detractors: number; // 0-6 rating

  // Response time
  avgResponseTime: number; // hours
  resolvedFeedback: number;
}
```

## Dashboard Implementation

### Tech Stack

**Frontend:**
- Next.js 14+ (App Router)
- React Server Components
- Tailwind CSS
- Chart.js / Recharts for visualizations
- Shadcn/ui for components

**Backend:**
- Next.js API Routes
- PostgreSQL for data storage
- Redis for caching
- Prisma ORM

**Data Sources:**
- Google Analytics
- Firebase Analytics
- Mixpanel / Amplitude
- Sentry
- Custom tracking APIs

### Dashboard Components

**`app/dashboard/beta/page.tsx`:**
```typescript
import { Suspense } from 'react';
import {
  MetricsOverview,
  PlatformComparison,
  UserGrowthChart,
  FeatureUsageChart,
  RecentIssues,
  RetentionCurve,
  DeviceBreakdown,
  GeographicMap,
} from '@/components/beta-dashboard';

export default async function BetaDashboard() {
  return (
    <div className="min-h-screen bg-gray-50 p-6">
      <div className="max-w-7xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">
            Beta Testing Dashboard
          </h1>
          <p className="text-gray-600">
            Real-time insights across iOS, Android, and PWA
          </p>
        </header>

        {/* Key Metrics */}
        <Suspense fallback={<MetricsSkeleton />}>
          <MetricsOverview />
        </Suspense>

        {/* Platform Comparison */}
        <Suspense fallback={<ChartSkeleton />}>
          <PlatformComparison />
        </Suspense>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
          {/* User Growth */}
          <Suspense fallback={<ChartSkeleton />}>
            <UserGrowthChart />
          </Suspense>

          {/* Feature Usage */}
          <Suspense fallback={<ChartSkeleton />}>
            <FeatureUsageChart />
          </Suspense>

          {/* Retention Curve */}
          <Suspense fallback={<ChartSkeleton />}>
            <RetentionCurve />
          </Suspense>

          {/* Device Breakdown */}
          <Suspense fallback={<ChartSkeleton />}>
            <DeviceBreakdown />
          </Suspense>
        </div>

        {/* Recent Issues */}
        <Suspense fallback={<TableSkeleton />}>
          <RecentIssues />
        </Suspense>

        {/* Geographic Map */}
        <Suspense fallback={<MapSkeleton />}>
          <GeographicMap />
        </Suspense>
      </div>
    </div>
  );
}
```

**`components/beta-dashboard/MetricsOverview.tsx`:**
```typescript
import { getBetaMetrics } from '@/lib/beta-analytics';
import { MetricCard } from './MetricCard';
import { TrendIndicator } from './TrendIndicator';

export async function MetricsOverview() {
  const metrics = await getBetaMetrics();

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <MetricCard
        title="Beta Testers"
        value={metrics.totalTesters.toLocaleString()}
        icon="👥"
        trend={
          <TrendIndicator
            value={metrics.testerGrowth}
            positive={metrics.testerGrowth > 0}
          />
        }
        breakdown={{
          iOS: metrics.ios,
          Android: metrics.android,
          PWA: metrics.pwa,
        }}
      />

      <MetricCard
        title="Crash-Free Rate"
        value={`${(metrics.crashFreeRate * 100).toFixed(2)}%`}
        icon="✅"
        trend={
          <TrendIndicator
            value={metrics.crashFreeRateChange}
            positive={metrics.crashFreeRateChange > 0}
          />
        }
        status={metrics.crashFreeRate > 0.99 ? 'good' : 'warning'}
      />

      <MetricCard
        title="Daily Active Users"
        value={metrics.dau.toLocaleString()}
        icon="📊"
        trend={
          <TrendIndicator
            value={metrics.dauChange}
            positive={metrics.dauChange > 0}
          />
        }
        subtitle={`${((metrics.dau / metrics.totalTesters) * 100).toFixed(1)}% of testers`}
      />

      <MetricCard
        title="Bug Reports"
        value={metrics.openBugs}
        icon="🐛"
        trend={
          <TrendIndicator
            value={metrics.bugChange}
            positive={metrics.bugChange < 0} // negative is good for bugs
          />
        }
        breakdown={{
          Critical: metrics.criticalBugs,
          High: metrics.highBugs,
          Medium: metrics.mediumBugs,
        }}
      />
    </div>
  );
}
```

**`components/beta-dashboard/UserGrowthChart.tsx`:**
```typescript
'use client';

import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
);

interface UserGrowthChartProps {
  data: {
    date: string;
    ios: number;
    android: number;
    pwa: number;
  }[];
}

export function UserGrowthChart({ data }: UserGrowthChartProps) {
  const chartData = {
    labels: data.map((d) => d.date),
    datasets: [
      {
        label: 'iOS',
        data: data.map((d) => d.ios),
        borderColor: 'rgb(59, 130, 246)',
        backgroundColor: 'rgba(59, 130, 246, 0.1)',
        tension: 0.4,
      },
      {
        label: 'Android',
        data: data.map((d) => d.android),
        borderColor: 'rgb(34, 197, 94)',
        backgroundColor: 'rgba(34, 197, 94, 0.1)',
        tension: 0.4,
      },
      {
        label: 'PWA',
        data: data.map((d) => d.pwa),
        borderColor: 'rgb(168, 85, 247)',
        backgroundColor: 'rgba(168, 85, 247, 0.1)',
        tension: 0.4,
      },
    ],
  };

  const options = {
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Beta Tester Growth',
      },
    },
    scales: {
      y: {
        beginAtZero: true,
      },
    },
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div style={{ height: '300px' }}>
        <Line data={chartData} options={options} />
      </div>
    </div>
  );
}
```

**`components/beta-dashboard/RecentIssues.tsx`:**
```typescript
import { getRecentIssues } from '@/lib/beta-analytics';
import { Badge } from '@/components/ui/badge';

export async function RecentIssues() {
  const issues = await getRecentIssues({ limit: 10 });

  return (
    <div className="bg-white rounded-lg shadow p-6 mt-6">
      <h2 className="text-xl font-bold mb-4">Recent Issues</h2>

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead className="border-b">
            <tr className="text-left text-gray-600">
              <th className="pb-2">ID</th>
              <th className="pb-2">Title</th>
              <th className="pb-2">Platform</th>
              <th className="pb-2">Severity</th>
              <th className="pb-2">Status</th>
              <th className="pb-2">Reported</th>
            </tr>
          </thead>
          <tbody>
            {issues.map((issue) => (
              <tr key={issue.id} className="border-b last:border-0">
                <td className="py-3 text-sm text-gray-600">
                  #{issue.id}
                </td>
                <td className="py-3">
                  <a
                    href={`/dashboard/beta/issues/${issue.id}`}
                    className="text-blue-600 hover:underline"
                  >
                    {issue.title}
                  </a>
                </td>
                <td className="py-3">
                  <Badge variant="outline">{issue.platform}</Badge>
                </td>
                <td className="py-3">
                  <Badge
                    variant={
                      issue.severity === 'critical'
                        ? 'destructive'
                        : issue.severity === 'high'
                        ? 'warning'
                        : 'secondary'
                    }
                  >
                    {issue.severity}
                  </Badge>
                </td>
                <td className="py-3">
                  <Badge
                    variant={
                      issue.status === 'resolved'
                        ? 'success'
                        : issue.status === 'in_progress'
                        ? 'warning'
                        : 'secondary'
                    }
                  >
                    {issue.status}
                  </Badge>
                </td>
                <td className="py-3 text-sm text-gray-600">
                  {formatRelativeTime(issue.createdAt)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function formatRelativeTime(date: Date): string {
  const seconds = Math.floor((Date.now() - date.getTime()) / 1000);

  if (seconds < 60) return 'just now';
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}h ago`;
  return `${Math.floor(seconds / 86400)}d ago`;
}
```

### Data Collection API

**`app/api/beta/metrics/route.ts`:**
```typescript
import { NextRequest, NextResponse } from 'next/server';
import { getBetaMetrics } from '@/lib/beta-analytics';

export async function GET(request: NextRequest) {
  try {
    const metrics = await getBetaMetrics();

    return NextResponse.json({
      success: true,
      data: metrics,
      timestamp: new Date().toISOString(),
    });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to fetch metrics' },
      { status: 500 }
    );
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();

    // Track custom event
    await trackBetaEvent(body);

    return NextResponse.json({ success: true });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to track event' },
      { status: 500 }
    );
  }
}
```

**`lib/beta-analytics.ts`:**
```typescript
import { prisma } from '@/lib/prisma';
import { redis } from '@/lib/redis';

/**
 * Get aggregated beta metrics
 */
export async function getBetaMetrics() {
  // Check cache first
  const cached = await redis.get('beta:metrics');
  if (cached) {
    return JSON.parse(cached);
  }

  // Fetch from database
  const [
    totalTesters,
    activeUsers,
    crashes,
    bugs,
    feedback,
  ] = await Promise.all([
    getTotalTesters(),
    getActiveUsers(),
    getCrashMetrics(),
    getBugMetrics(),
    getFeedbackMetrics(),
  ]);

  const metrics = {
    totalTesters,
    activeUsers,
    crashes,
    bugs,
    feedback,
    updatedAt: new Date().toISOString(),
  };

  // Cache for 5 minutes
  await redis.setex('beta:metrics', 300, JSON.stringify(metrics));

  return metrics;
}

async function getTotalTesters() {
  const result = await prisma.betaTester.groupBy({
    by: ['platform'],
    _count: true,
  });

  return {
    ios: result.find((r) => r.platform === 'ios')?._count || 0,
    android: result.find((r) => r.platform === 'android')?._count || 0,
    pwa: result.find((r) => r.platform === 'pwa')?._count || 0,
    total: result.reduce((sum, r) => sum + r._count, 0),
  };
}

async function getActiveUsers() {
  const oneDayAgo = new Date(Date.now() - 24 * 60 * 60 * 1000);
  const oneWeekAgo = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000);
  const oneMonthAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);

  const [dau, wau, mau] = await Promise.all([
    prisma.betaActivity.findMany({
      where: { lastActiveAt: { gte: oneDayAgo } },
      distinct: ['userId'],
    }).then((r) => r.length),
    prisma.betaActivity.findMany({
      where: { lastActiveAt: { gte: oneWeekAgo } },
      distinct: ['userId'],
    }).then((r) => r.length),
    prisma.betaActivity.findMany({
      where: { lastActiveAt: { gte: oneMonthAgo } },
      distinct: ['userId'],
    }).then((r) => r.length),
  ]);

  return { dau, wau, mau, stickiness: mau > 0 ? dau / mau : 0 };
}

async function getCrashMetrics() {
  const crashes = await prisma.crashReport.groupBy({
    by: ['platform'],
    _count: true,
    where: {
      createdAt: {
        gte: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
      },
    },
  });

  return {
    total: crashes.reduce((sum, c) => sum + c._count, 0),
    byPlatform: {
      ios: crashes.find((c) => c.platform === 'ios')?._count || 0,
      android: crashes.find((c) => c.platform === 'android')?._count || 0,
      pwa: crashes.find((c) => c.platform === 'pwa')?._count || 0,
    },
  };
}

async function getBugMetrics() {
  const bugs = await prisma.bugReport.groupBy({
    by: ['severity', 'status'],
    _count: true,
  });

  return {
    total: bugs.reduce((sum, b) => sum + b._count, 0),
    open: bugs
      .filter((b) => b.status === 'open')
      .reduce((sum, b) => sum + b._count, 0),
    critical: bugs
      .filter((b) => b.severity === 'critical')
      .reduce((sum, b) => sum + b._count, 0),
    high: bugs
      .filter((b) => b.severity === 'high')
      .reduce((sum, b) => sum + b._count, 0),
  };
}

async function getFeedbackMetrics() {
  const feedback = await prisma.feedback.groupBy({
    by: ['type'],
    _count: true,
    where: {
      createdAt: {
        gte: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
      },
    },
  });

  return {
    total: feedback.reduce((sum, f) => sum + f._count, 0),
    bugs: feedback.find((f) => f.type === 'bug')?._count || 0,
    features: feedback.find((f) => f.type === 'feature')?._count || 0,
  };
}
```

## Dashboard Tools Integration

### Google Analytics 4

**Setup:**
```typescript
// lib/analytics/google.ts
export function initGoogleAnalytics() {
  if (typeof window !== 'undefined') {
    window.gtag('config', process.env.NEXT_PUBLIC_GA_ID, {
      custom_map: {
        dimension1: 'platform',
        dimension2: 'beta_tester',
        dimension3: 'feature_flag',
      },
    });
  }
}

export function trackBetaEvent(event: string, params: Record<string, any>) {
  if (typeof window !== 'undefined') {
    window.gtag('event', event, {
      ...params,
      beta_tester: true,
    });
  }
}
```

### Firebase Analytics

**Setup:**
```typescript
// lib/analytics/firebase.ts
import { getAnalytics, logEvent } from 'firebase/analytics';

export function trackBetaEvent(event: string, params: Record<string, any>) {
  const analytics = getAnalytics();
  logEvent(analytics, event, {
    ...params,
    is_beta: true,
  });
}
```

### Mixpanel

**Setup:**
```typescript
// lib/analytics/mixpanel.ts
import mixpanel from 'mixpanel-browser';

mixpanel.init(process.env.NEXT_PUBLIC_MIXPANEL_TOKEN!);

export function identifyBetaTester(userId: string, properties: Record<string, any>) {
  mixpanel.identify(userId);
  mixpanel.people.set({
    ...properties,
    is_beta_tester: true,
  });
}

export function trackBetaEvent(event: string, properties: Record<string, any>) {
  mixpanel.track(event, {
    ...properties,
    environment: 'beta',
  });
}
```

### Sentry

**Setup:**
```typescript
// lib/sentry.ts
import * as Sentry from '@sentry/nextjs';

Sentry.init({
  dsn: process.env.NEXT_PUBLIC_SENTRY_DSN,
  environment: process.env.NEXT_PUBLIC_ENV,
  enabled: process.env.NODE_ENV === 'production',
  tracesSampleRate: 1.0,
  beforeSend(event) {
    // Tag beta events
    if (event.user) {
      event.tags = {
        ...event.tags,
        beta_tester: true,
      };
    }
    return event;
  },
});
```

## Alerting System

**`lib/alerts.ts`:**
```typescript
/**
 * Alert when metrics exceed thresholds
 */

export async function checkMetricsAndAlert() {
  const metrics = await getBetaMetrics();

  // Critical alerts
  if (metrics.crashes.crashFreeRate < 0.99) {
    await sendAlert({
      severity: 'critical',
      title: 'Crash rate too high',
      message: `Crash-free rate is ${(metrics.crashes.crashFreeRate * 100).toFixed(2)}% (target: >99%)`,
      channel: ['slack', 'email', 'pagerduty'],
    });
  }

  if (metrics.bugs.critical > 0) {
    await sendAlert({
      severity: 'critical',
      title: 'Critical bugs reported',
      message: `${metrics.bugs.critical} critical bugs need immediate attention`,
      channel: ['slack', 'email'],
    });
  }

  // Warning alerts
  if (metrics.activeUsers.stickiness < 0.2) {
    await sendAlert({
      severity: 'warning',
      title: 'User engagement low',
      message: `Stickiness is ${(metrics.activeUsers.stickiness * 100).toFixed(1)}% (target: >20%)`,
      channel: ['slack'],
    });
  }
}

async function sendAlert(alert: Alert) {
  // Send to Slack
  if (alert.channel.includes('slack')) {
    await fetch(process.env.SLACK_WEBHOOK_URL!, {
      method: 'POST',
      body: JSON.stringify({
        text: `🚨 ${alert.title}`,
        blocks: [
          {
            type: 'section',
            text: {
              type: 'mrkdwn',
              text: `*${alert.title}*\n${alert.message}`,
            },
          },
        ],
      }),
    });
  }

  // Send email (via SendGrid, Resend, etc.)
  if (alert.channel.includes('email')) {
    // Implementation
  }

  // PagerDuty for critical alerts
  if (alert.channel.includes('pagerduty')) {
    // Implementation
  }
}
```

## Automated Reports

**Weekly Beta Report:**
```typescript
// scripts/generate-beta-report.ts

export async function generateWeeklyReport() {
  const metrics = await getBetaMetrics();
  const previousWeek = await getBetaMetrics({ daysAgo: 7 });

  const report = `
# Beta Testing Weekly Report
${new Date().toLocaleDateString()}

## Summary
- Total Beta Testers: ${metrics.totalTesters.total} (+${metrics.totalTesters.total - previousWeek.totalTesters.total})
- Daily Active Users: ${metrics.activeUsers.dau}
- Crash-Free Rate: ${(metrics.crashes.crashFreeRate * 100).toFixed(2)}%
- Open Bugs: ${metrics.bugs.open}

## Platform Breakdown
- iOS: ${metrics.totalTesters.ios} testers
- Android: ${metrics.totalTesters.android} testers
- PWA: ${metrics.totalTesters.pwa} testers

## Top Issues This Week
${await getTopIssues()}

## Feature Highlights
${await getFeatureHighlights()}

## Action Items
${await getActionItems()}

---
Full dashboard: https://wallet.etrid.com/dashboard/beta
  `;

  // Send via email
  await sendEmailReport(report);

  // Post to Slack
  await postToSlack(report);

  return report;
}
```

## Resources

- [Building Analytics Dashboards](https://vercel.com/templates/next.js/admin-dashboard-tailwind-postgres)
- [Chart.js Documentation](https://www.chartjs.org/docs/)
- [Recharts Guide](https://recharts.org/)
- [Beta Metrics Best Practices](https://amplitude.com/blog/beta-testing-metrics)

## Support

Questions? Contact:
- Email: dev@etrid.com
- Discord: #beta-dashboard
- Internal Wiki: https://wiki.etrid.com/beta-dashboard
