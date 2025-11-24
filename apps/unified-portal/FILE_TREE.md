# Etrid Unified Portal - Complete File Structure

**Generated**: November 22, 2025
**Total Source Files**: 20
**Total Lines of Code**: ~1,200+

---

## Directory Structure

```
unified-portal/
│
├── 📁 app/                           # Next.js App Router
│   ├── 📄 layout.tsx                 # Root layout with theme provider
│   ├── 📄 page.tsx                   # Dashboard home page
│   ├── 📄 globals.css                # Global styles + Tailwind
│   ├── 📁 lightning/                 # Lightning Network feature
│   │   └── 📄 page.tsx               # Lightning placeholder
│   ├── 📁 validator/                 # Validator operations
│   │   └── 📄 page.tsx               # Validator placeholder
│   ├── 📁 watchtower/                # Watchtower monitoring
│   │   └── 📄 page.tsx               # Watchtower placeholder
│   ├── 📁 wallet/                    # Wallet & DeFi
│   │   └── 📄 page.tsx               # Wallet placeholder
│   ├── 📁 governance/                # Governance
│   │   └── 📄 page.tsx               # Governance placeholder
│   ├── 📁 monitoring/                # Network monitoring
│   │   └── 📄 page.tsx               # Monitoring placeholder
│   └── 📁 masterchef/                # MasterChef DeFi
│       └── 📄 page.tsx               # MasterChef placeholder
│
├── 📁 components/                    # React components
│   ├── 📁 layout/                    # Layout components
│   │   ├── 📄 portal-header.tsx      # Header with wallet/theme
│   │   ├── 📄 portal-nav.tsx         # Tab navigation (8 items)
│   │   └── 📄 status-bar.tsx         # Footer with network stats
│   ├── 📁 dashboard/                 # Dashboard components
│   │   ├── 📄 stat-card.tsx          # Network statistics card
│   │   └── 📄 service-card.tsx       # Service navigation card
│   ├── 📁 ui/                        # Shared UI components
│   │   ├── 📄 button.tsx             # Button component (CVA)
│   │   ├── 📄 card.tsx               # Card component
│   │   └── 📄 badge.tsx              # Badge component
│   └── 📄 theme-provider.tsx         # Next-themes wrapper
│
├── 📁 lib/                           # Utilities
│   └── 📄 utils.ts                   # cn() helper + utils
│
├── 📁 public/                        # Static assets
│   ├── 📄 file.svg
│   ├── 📄 globe.svg
│   ├── 📄 vercel.svg
│   └── 📄 window.svg
│
├── 📄 package.json                   # Dependencies
├── 📄 tsconfig.json                  # TypeScript config
├── 📄 next.config.ts                 # Next.js config
├── 📄 postcss.config.mjs             # PostCSS config
├── 📄 eslint.config.mjs              # ESLint config
├── 📄 README.md                      # Documentation
├── 📄 AGENT_1_SUMMARY.md             # Agent 1 report
└── 📄 FILE_TREE.md                   # This file
```

---

## File Details

### App Router Pages (10 files)

| File | Lines | Description |
|------|-------|-------------|
| `app/layout.tsx` | 42 | Root layout with theme provider |
| `app/page.tsx` | 105 | Dashboard with stats and services |
| `app/globals.css` | 24 | Global styles + Tailwind CSS |
| `app/lightning/page.tsx` | 25 | Lightning Network placeholder |
| `app/validator/page.tsx` | 25 | Validator dashboard placeholder |
| `app/watchtower/page.tsx` | 25 | Watchtower monitor placeholder |
| `app/wallet/page.tsx` | 25 | Wallet & DeFi placeholder |
| `app/governance/page.tsx` | 25 | Governance placeholder |
| `app/monitoring/page.tsx` | 25 | Monitoring placeholder |
| `app/masterchef/page.tsx` | 25 | MasterChef DeFi placeholder |

**Total**: ~350 lines

### Layout Components (3 files)

| File | Lines | Description |
|------|-------|-------------|
| `components/layout/portal-header.tsx` | 52 | Header with logo, wallet, theme toggle |
| `components/layout/portal-nav.tsx` | 75 | Navigation with 8 service tabs |
| `components/layout/status-bar.tsx` | 55 | Footer with network statistics |

**Total**: ~180 lines

### Dashboard Components (2 files)

| File | Lines | Description |
|------|-------|-------------|
| `components/dashboard/stat-card.tsx` | 42 | Statistics card with status badges |
| `components/dashboard/service-card.tsx` | 40 | Service card with navigation |

**Total**: ~80 lines

### UI Components (3 files)

| File | Lines | Description |
|------|-------|-------------|
| `components/ui/button.tsx` | 60 | Button with variants (CVA) |
| `components/ui/card.tsx` | 85 | Card + CardHeader + CardContent |
| `components/ui/badge.tsx` | 55 | Badge with color variants |

**Total**: ~200 lines

### Utilities (2 files)

| File | Lines | Description |
|------|-------|-------------|
| `lib/utils.ts` | 6 | cn() classname helper |
| `components/theme-provider.tsx` | 12 | Next-themes wrapper |

**Total**: ~18 lines

---

## Component Dependencies

### Layout Flow
```
app/layout.tsx
  ├── ThemeProvider (theme-provider.tsx)
  ├── PortalHeader (layout/portal-header.tsx)
  │   ├── Button (ui/button.tsx)
  │   └── Badge (ui/badge.tsx)
  ├── PortalNav (layout/portal-nav.tsx)
  ├── [Page Content]
  └── StatusBar (layout/status-bar.tsx)
      └── Badge (ui/badge.tsx)
```

### Dashboard Flow
```
app/page.tsx
  ├── StatCard (dashboard/stat-card.tsx)
  │   ├── Card (ui/card.tsx)
  │   └── Badge (ui/badge.tsx)
  └── ServiceCard (dashboard/service-card.tsx)
      ├── Card (ui/card.tsx)
      └── Button (ui/button.tsx)
```

---

## Routes & Navigation

### Implemented Routes

| Route | Page | Status | Icon |
|-------|------|--------|------|
| `/` | Dashboard | ✅ Complete | LayoutDashboard |
| `/lightning` | Lightning Network | ⏳ Placeholder | Zap |
| `/validator` | Validator Dashboard | ⏳ Placeholder | Shield |
| `/watchtower` | Watchtower Monitor | ⏳ Placeholder | Eye |
| `/wallet` | Wallet & DeFi | ⏳ Placeholder | Wallet |
| `/governance` | Governance | ⏳ Placeholder | Vote |
| `/monitoring` | Network Monitoring | ⏳ Placeholder | Activity |
| `/masterchef` | MasterChef DeFi | ⏳ Placeholder | Coins |

**Total Routes**: 8

---

## Tech Stack Breakdown

### Core Framework
```json
{
  "next": "16.0.3",
  "react": "19.x",
  "react-dom": "19.x",
  "typescript": "5.x"
}
```

### Styling
```json
{
  "tailwindcss": "4.1.9",
  "@tailwindcss/postcss": "latest",
  "class-variance-authority": "latest",
  "clsx": "latest",
  "tailwind-merge": "latest"
}
```

### UI Components
```json
{
  "@radix-ui/react-slot": "latest",
  "@radix-ui/react-tabs": "latest",
  "@radix-ui/react-dropdown-menu": "latest",
  "@radix-ui/react-dialog": "latest",
  "lucide-react": "latest"
}
```

### Theme Management
```json
{
  "next-themes": "latest"
}
```

---

## Color Palette

### Light Mode
```css
--background: hsl(0 0% 100%)        /* White */
--foreground: hsl(0 0% 3.9%)        /* Near Black */
--primary: hsl(271 91% 65%)         /* Purple 600 */
--accent: hsl(221 83% 53%)          /* Blue 600 */
```

### Dark Mode
```css
--background: hsl(0 0% 3.9%)        /* Near Black */
--foreground: hsl(0 0% 98%)         /* Near White */
--primary: hsl(271 91% 65%)         /* Purple 600 (same) */
--accent: hsl(221 83% 53%)          /* Blue 600 (same) */
```

### Status Colors
```css
--success: hsl(142 76% 36%)         /* Green */
--warning: hsl(45 93% 47%)          /* Yellow */
--error: hsl(0 84% 60%)             /* Red */
```

---

## Component Variants

### Button Variants
- `default` - Purple primary button
- `destructive` - Red danger button
- `outline` - Border only
- `secondary` - Gray background
- `ghost` - No background
- `link` - Underlined text

### Badge Variants
- `default` - Dark background
- `secondary` - Gray background
- `destructive` - Red background
- `success` - Green background
- `warning` - Yellow background
- `outline` - Border only

---

## Build Output

### Production Build Stats
```
Route (app)
┌ ○ /                  # Dashboard
├ ○ /_not-found        # 404 page
├ ○ /governance        # Governance
├ ○ /lightning         # Lightning
├ ○ /masterchef        # MasterChef
├ ○ /monitoring        # Monitoring
├ ○ /validator         # Validator
├ ○ /wallet            # Wallet
└ ○ /watchtower        # Watchtower

○  (Static)  prerendered as static content
```

**Build Time**: ~2.3s (compilation) + 625ms (static generation)
**Total Pages**: 9 (all static)

---

## Next Steps for Migration

### Agent 2 - Lightning + MasterChef
- Replace `/lightning/page.tsx` with actual features
- Replace `/masterchef/page.tsx` with LP rewards UI
- Use existing UI components
- Maintain purple/blue theme

### Agent 3 - Validator + Watchtower
- Replace `/validator/page.tsx` with dashboard
- Replace `/watchtower/page.tsx` with monitoring UI
- Integrate Polkadot.js
- Add sub-routes as needed

### Agent 4 - Wallet Features
- Replace `/wallet/page.tsx` with swap/stake UI
- Add `/wallet/swap`, `/wallet/staking` routes
- Connect to blockchain
- Integrate DeFi protocols

### Agent 5 - Governance
- Replace `/governance/page.tsx` with Snapshot integration
- Add director dashboard
- Implement postMessage bridge for Vue app
- Add proposal voting UI

### Agent 6 - Monitoring
- Replace `/monitoring/page.tsx` with telemetry
- Add Prometheus/Grafana integration
- Build network map
- Add real-time stats

### Agent 7 - Shared Packages
- Extract components to `@etrid/ui`
- Create `@etrid/hooks` library
- Set up monorepo workspace
- Document component APIs

---

**End of File Structure Documentation**
