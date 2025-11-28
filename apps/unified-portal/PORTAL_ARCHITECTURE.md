# Unified Portal Design
## Single Integrated App with Tabs

**Date:** November 22, 2025
**Question:** "Do all UIs open at once or is it an integrated app with tabs?"
**Answer:** We'll build an integrated app with tabs!

---

## Design Overview

Instead of 9 separate browser tabs, you get **ONE unified portal**:

```
Single Browser Tab: http://localhost:3000
  │
  └─→ Ëtrid Control Center
       ├─ Dashboard (overview)
       ├─ Lightning (tab)
       ├─ Validator (tab)
       ├─ Watchtower (tab)
       ├─ Wallet (tab)
       ├─ Governance (tab)
       ├─ Monitoring (tab)
       └─ Settings (tab)
```

---

## Portal Architecture

### Layout Structure

```
┌─────────────────────────────────────────────────────────────┐
│  ËTRID CONTROL CENTER                    [🔌 Connected]    │ ← Header
├─────────────────────────────────────────────────────────────┤
│ 📊 Dashboard │ ⚡ Lightning │ 🎯 Validator │ More ▼        │ ← Navigation Tabs
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                                                             │
│                   [Active Tab Content]                      │
│                                                             │
│                                                             │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│ 🟢 All Systems Operational │ Block: #12,450 │ Peers: 21   │ ← Status Bar
└─────────────────────────────────────────────────────────────┘
```

---

## Implementation: Quick iframe Portal

**Location:** `/Desktop/etrid/apps/unified-portal/`

### File Structure

```
unified-portal/
├── app/
│   ├── layout.tsx          # Main layout with header/sidebar
│   ├── page.tsx            # Dashboard (overview of all systems)
│   ├── lightning/
│   │   └── page.tsx        # iframe: localhost:3001
│   ├── validator/
│   │   └── page.tsx        # iframe: localhost:3002
│   ├── watchtower/
│   │   └── page.tsx        # iframe: localhost:3003
│   ├── wallet/
│   │   └── page.tsx        # iframe: localhost:3004
│   ├── governance/
│   │   └── page.tsx        # iframe: localhost:8082
│   └── monitoring/
│       ├── telemetry/
│       │   └── page.tsx    # iframe: localhost:8080
│       ├── grafana/
│       │   └── page.tsx    # iframe: localhost:3100
│       └── prometheus/
│           └── page.tsx    # iframe: localhost:9090
├── components/
│   ├── portal-header.tsx   # Header with wallet connection
│   ├── portal-nav.tsx      # Navigation tabs
│   ├── status-bar.tsx      # Status indicators
│   └── service-iframe.tsx  # Reusable iframe wrapper
├── lib/
│   └── service-manager.ts  # Check which services are running
├── package.json
└── README.md
```

### Code Example: Portal Layout

**File:** `app/layout.tsx`

```typescript
import PortalHeader from '@/components/portal-header'
import PortalNav from '@/components/portal-nav'
import StatusBar from '@/components/status-bar'

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <div className="flex flex-col h-screen">
          {/* Header with wallet connection */}
          <PortalHeader />

          {/* Navigation tabs */}
          <PortalNav />

          {/* Main content area */}
          <main className="flex-1 overflow-hidden">
            {children}
          </main>

          {/* Status bar */}
          <StatusBar />
        </div>
      </body>
    </html>
  )
}
```

### Code Example: Dashboard Page

**File:** `app/page.tsx`

```typescript
'use client'

import { useEffect, useState } from 'react'
import { checkServiceStatus } from '@/lib/service-manager'

export default function Dashboard() {
  const [services, setServices] = useState([])

  useEffect(() => {
    async function loadServices() {
      const status = await checkServiceStatus()
      setServices(status)
    }
    loadServices()
    const interval = setInterval(loadServices, 5000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-6">Ëtrid Control Center</h1>

      {/* Quick Stats Grid */}
      <div className="grid grid-cols-4 gap-4 mb-8">
        <StatCard
          title="Validators"
          value="21"
          status="healthy"
          icon="🎯"
        />
        <StatCard
          title="Block Height"
          value="#12,450"
          status="syncing"
          icon="📦"
        />
        <StatCard
          title="Peers"
          value="21"
          status="healthy"
          icon="🌐"
        />
        <StatCard
          title="Network TPS"
          value="1,250"
          status="healthy"
          icon="⚡"
        />
      </div>

      {/* Service Status */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4">Service Status</h2>
        <div className="grid grid-cols-3 gap-4">
          {services.map(service => (
            <ServiceCard
              key={service.name}
              name={service.name}
              port={service.port}
              status={service.status}
              url={service.url}
            />
          ))}
        </div>
      </div>

      {/* Recent Alerts */}
      <div className="mt-8 bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold mb-4">Recent Alerts</h2>
        <AlertsList />
      </div>
    </div>
  )
}

function ServiceCard({ name, port, status, url }) {
  const statusColor = status === 'running' ? 'bg-green-500' : 'bg-red-500'

  return (
    <a
      href={url}
      className="flex items-center p-4 border rounded-lg hover:shadow-md transition"
    >
      <div className={`w-3 h-3 rounded-full ${statusColor} mr-3`} />
      <div className="flex-1">
        <div className="font-semibold">{name}</div>
        <div className="text-sm text-gray-500">:{port}</div>
      </div>
      <div className="text-blue-500">→</div>
    </a>
  )
}
```

### Code Example: iframe Wrapper

**File:** `components/service-iframe.tsx`

```typescript
'use client'

import { useEffect, useState } from 'react'

interface ServiceIframeProps {
  url: string
  title: string
}

export default function ServiceIframe({ url, title }: ServiceIframeProps) {
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(false)

  useEffect(() => {
    // Check if service is accessible
    fetch(url)
      .then(() => {
        setLoading(false)
        setError(false)
      })
      .catch(() => {
        setLoading(false)
        setError(true)
      })
  }, [url])

  if (error) {
    return (
      <div className="flex items-center justify-center h-full bg-gray-100">
        <div className="text-center">
          <div className="text-6xl mb-4">⚠️</div>
          <h2 className="text-2xl font-bold mb-2">Service Not Running</h2>
          <p className="text-gray-600 mb-4">
            {title} is not accessible at {url}
          </p>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Retry
          </button>
        </div>
      </div>
    )
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full bg-gray-100">
        <div className="text-center">
          <div className="animate-spin text-6xl mb-4">⚙️</div>
          <p className="text-gray-600">Loading {title}...</p>
        </div>
      </div>
    )
  }

  return (
    <iframe
      src={url}
      className="w-full h-full border-0"
      title={title}
      sandbox="allow-same-origin allow-scripts allow-forms allow-popups"
    />
  )
}
```

### Code Example: Validator Tab

**File:** `app/validator/page.tsx`

```typescript
import ServiceIframe from '@/components/service-iframe'

export default function ValidatorPage() {
  return (
    <ServiceIframe
      url="http://localhost:3002"
      title="Validator Dashboard"
    />
  )
}
```

---

## Navigation Structure

### Main Navigation Tabs

```typescript
const navigation = [
  {
    name: 'Dashboard',
    href: '/',
    icon: '📊',
    description: 'Overview of all systems'
  },
  {
    name: 'Lightning',
    href: '/lightning',
    icon: '⚡',
    description: 'Lightning Landing'
  },
  {
    name: 'Validator',
    href: '/validator',
    icon: '🎯',
    description: 'Validator operations'
  },
  {
    name: 'Watchtower',
    href: '/watchtower',
    icon: '👁️',
    description: 'Channel monitoring'
  },
  {
    name: 'Wallet',
    href: '/wallet',
    icon: '💰',
    description: 'Wallet & staking'
  },
  {
    name: 'Governance',
    href: '/governance',
    icon: '🗳️',
    description: 'Voting & proposals',
    children: [
      { name: 'Proposals', href: '/governance' },
      { name: 'Directors', href: '/governance/directors' }
    ]
  },
  {
    name: 'Monitoring',
    href: '/monitoring',
    icon: '📈',
    description: 'Network monitoring',
    children: [
      { name: 'Network Telemetry', href: '/monitoring/telemetry' },
      { name: 'Grafana', href: '/monitoring/grafana' },
      { name: 'Prometheus', href: '/monitoring/prometheus' }
    ]
  },
  {
    name: 'MasterChef',
    href: '/masterchef',
    icon: '👨‍🍳',
    description: 'DeFi dashboard'
  }
]
```

---

## Service Manager

**File:** `lib/service-manager.ts`

```typescript
const SERVICES = [
  { name: 'Lightning Landing', port: 3001, path: '/lightning' },
  { name: 'Validator Dashboard', port: 3002, path: '/validator' },
  { name: 'Watchtower Monitor', port: 3003, path: '/watchtower' },
  { name: 'Wallet Web', port: 3004, path: '/wallet' },
  { name: 'Network Telemetry', port: 8080, path: '/monitoring/telemetry' },
  { name: 'Governance UI', port: 8082, path: '/governance' },
  { name: 'Prometheus', port: 9090, path: '/monitoring/prometheus' },
  { name: 'Grafana', port: 3100, path: '/monitoring/grafana' },
  { name: 'MasterChef', port: 3001, path: '/masterchef' }
]

export async function checkServiceStatus() {
  const results = await Promise.all(
    SERVICES.map(async (service) => {
      try {
        const response = await fetch(`http://localhost:${service.port}`, {
          method: 'HEAD',
          mode: 'no-cors'
        })
        return {
          ...service,
          status: 'running',
          url: `http://localhost:${service.port}`
        }
      } catch (error) {
        return {
          ...service,
          status: 'stopped',
          url: service.path
        }
      }
    })
  )
  return results
}
```

---

## Benefits of Unified Portal

### User Experience

✅ **Single Browser Tab**
- No more juggling 9 tabs
- Cleaner browser experience
- Easier to bookmark

✅ **Seamless Navigation**
- Click tabs to switch between apps
- Consistent UI/UX
- Shared header/footer

✅ **Unified Authentication**
- Connect wallet once
- Session shared across all apps
- Single logout

✅ **Global Features**
- Notifications from all systems in one place
- Global search across all apps
- Unified theme switcher (dark/light mode)

✅ **Better Mobile Support**
- Responsive design
- Mobile-friendly navigation
- Progressive Web App (PWA) capabilities

### Developer Experience

✅ **Easy Integration**
- iframe approach = no refactoring existing apps
- Add new apps by creating one page
- Services stay independent

✅ **Consistent Branding**
- Portal enforces PrimeArc theme
- Shared components (header, nav, status bar)
- Unified design system

✅ **Service Monitoring**
- Portal detects which services are running
- Shows friendly error if service is down
- "Start Service" button to launch missing services

---

## Portal Features

### 1. Dashboard (Overview)

Shows at-a-glance status:
- Service health (green/red indicators)
- Validator status (21 validators, health scores)
- Network stats (block height, TPS, peers)
- Recent alerts (from all systems)
- Quick links to each app

### 2. Service Status Detection

Portal automatically detects:
- Which services are running
- Which ports are accessible
- Shows "Service Not Running" if unavailable
- Provides "Start Service" button

### 3. Unified Wallet Connection

Connect Polkadot.js wallet once:
- Header shows connection status
- All iframed apps can access wallet via postMessage
- Single disconnect button

### 4. Global Search

Search bar in header:
- Search across all apps
- Find validators by ID
- Jump to specific metrics
- Search proposals

### 5. Notifications Center

Bell icon in header:
- Aggregates alerts from all systems
- Priority-based sorting (critical first)
- Click to jump to relevant app
- Mark as read/unread

---

## Implementation Timeline

### Phase 1: Portal Shell (2 hours)

- Create Next.js app with layout
- Build header, navigation, status bar
- Add routing for all tabs
- Deploy at localhost:3000

### Phase 2: iframe Integration (2 hours)

- Create ServiceIframe component
- Add pages for each service
- Implement service detection
- Add loading/error states

### Phase 3: Dashboard (2 hours)

- Build overview page
- Service status cards
- Quick stats
- Recent alerts

### Phase 4: Enhanced Features (3 hours)

- Unified wallet connection
- Global search
- Notifications center
- Theme switcher

### Phase 5: Polish (1 hour)

- Responsive design
- Mobile navigation
- PWA setup
- Documentation

**Total: ~10 hours**

---

## Updated Pinokio Integration

### Modified Startup Flow

**Before:**
```bash
npm run start:all
# Opens 9 separate services on 9 ports
# User opens 9 browser tabs manually
```

**After:**
```bash
npm run start:all
# Starts 8 backend services (ports 3001-9090)
# Starts unified portal (port 3000)
# User opens ONE tab: http://localhost:3000
# Portal shows all services in tabs
```

### Updated npm Scripts

```json
{
  "scripts": {
    "start:all": "npm run start:services && npm run start:portal",
    "start:services": "npm run web:start && npm run monitor:all && npm run gov:all",
    "start:portal": "cd ../apps/unified-portal && npm run dev",
    "portal:build": "cd ../apps/unified-portal && npm run build",
    "portal:dev": "cd ../apps/unified-portal && npm run dev"
  }
}
```

---

## Comparison: iframe vs Native

### iframe Portal (Recommended for now)

**Pros:**
- ✅ Quick to build (10 hours)
- ✅ No refactoring existing apps
- ✅ Each app stays independent
- ✅ Easy to add/remove apps

**Cons:**
- ❌ Slight performance overhead
- ❌ Can't share React state between apps
- ❌ Each iframe reloads on navigation
- ❌ Cross-origin issues if apps on different domains

### Native Portal (Future improvement)

**Pros:**
- ✅ Best performance
- ✅ Shared React state
- ✅ Single page app (SPA) experience
- ✅ No iframe limitations

**Cons:**
- ❌ Requires refactoring all apps
- ❌ Merge into monorepo
- ❌ Longer development time (weeks)
- ❌ More complex to maintain

---

## Recommendation

**Start with iframe portal** for quick wins:

1. Build unified portal (10 hours)
2. Get immediate UX benefits (single tab!)
3. Keep existing apps independent
4. Later: Gradually migrate to native components

**Migration Path:**
```
Phase 1: iframe portal (10 hours) → Launch
Phase 2: Use for 2-3 months → Gather feedback
Phase 3: Identify most-used apps
Phase 4: Convert high-traffic apps to native (optional)
```

---

## Next Steps

Want me to build this unified portal? I can:

1. **Create the portal app** (Next.js with iframes)
2. **Integrate all your services** (9 tabs)
3. **Add dashboard** (overview page)
4. **Update Pinokio** to launch portal

Takes ~10 hours total, gives you ONE browser tab instead of 9!

**Ready to start?** 🚀
