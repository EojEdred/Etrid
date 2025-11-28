# Etrid Unified Portal

**A Next.js 15 unified control center for the Etrid Protocol**

Built with Next.js 15, React 19, and Tailwind CSS 4. This portal consolidates all Etrid services into a single, modern interface.

## Features

- **Dashboard**: Overview of network stats and all services
- **Lightning Network**: Channel management and payments (coming soon)
- **Validator Dashboard**: Performance monitoring and rewards (coming soon)
- **Watchtower Monitor**: Fraud detection and earnings (coming soon)
- **Wallet & DeFi**: Asset management and swaps (coming soon)
- **Governance**: Protocol voting and proposals (coming soon)
- **Monitoring**: Network telemetry and metrics (coming soon)
- **MasterChef**: LP rewards and yield farming (coming soon)

## Tech Stack

- **Framework**: Next.js 15.2.4 with App Router
- **Runtime**: React 19
- **Styling**: Tailwind CSS 4.1.9
- **UI Components**: Radix UI + shadcn/ui patterns
- **Icons**: Lucide React
- **Theme**: Dark/Light mode with next-themes
- **TypeScript**: Full type safety

## Architecture

```
unified-portal/
├── app/                      # Next.js App Router
│   ├── (dashboard)/         # Dashboard pages
│   ├── lightning/           # Lightning Network
│   ├── validator/           # Validator operations
│   ├── watchtower/          # Watchtower monitoring
│   ├── wallet/              # Wallet & DeFi
│   ├── governance/          # Governance
│   ├── monitoring/          # Network monitoring
│   └── masterchef/          # MasterChef DeFi
├── components/
│   ├── layout/              # Layout components
│   │   ├── portal-header.tsx
│   │   ├── portal-nav.tsx
│   │   └── status-bar.tsx
│   ├── dashboard/           # Dashboard components
│   │   ├── stat-card.tsx
│   │   └── service-card.tsx
│   ├── ui/                  # Shared UI components
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   └── badge.tsx
│   └── theme-provider.tsx
└── lib/
    └── utils.ts             # Utility functions
```

## Getting Started

### Prerequisites

- Node.js 18+ or 20+
- npm, yarn, or pnpm

### Installation

```bash
cd /Users/macbook/Desktop/etrid/apps/unified-portal
npm install
```

### Development

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the portal.

### Build

```bash
npm run build
npm start
```

## Features in Detail

### Portal Layout

- **Header**: Wallet connection, theme toggle, network status badge
- **Navigation**: Tab-based navigation for all services
- **Status Bar**: Real-time network stats (block height, peers, TPS)
- **Responsive**: Mobile-friendly design

### Theme System

Supports dark and light modes with purple/blue gradient accents:
- Primary color: Purple 600 (#9333ea)
- Accent color: Blue 600 (#2563eb)
- Auto-detects system preference
- Manual toggle available

### Routing Structure

All routes are set up and ready for feature migration:

```
/                 - Dashboard overview
/lightning        - Lightning Network
/validator        - Validator dashboard
/watchtower       - Watchtower monitor
/wallet           - Wallet & DeFi
/governance       - Governance
/monitoring       - Network monitoring
/masterchef       - MasterChef DeFi
```

## Migration Status

| Feature | Status | Agent |
|---------|--------|-------|
| Core Portal | ✅ Complete | Agent 1 |
| Lightning | ⏳ Pending | Agent 2 |
| MasterChef | ⏳ Pending | Agent 2 |
| Validator | ⏳ Pending | Agent 3 |
| Watchtower | ⏳ Pending | Agent 3 |
| Wallet | ⏳ Pending | Agent 4 |
| Governance | ⏳ Pending | Agent 5 |
| Monitoring | ⏳ Pending | Agent 6 |

## Next Steps

This foundation is ready for Agents 2-6 to migrate features:

1. **Agent 2**: Lightning + MasterChef migration
2. **Agent 3**: Validator + Watchtower migration
3. **Agent 4**: Wallet features migration
4. **Agent 5**: Governance integration (micro-frontend)
5. **Agent 6**: Monitoring integration
6. **Agent 7**: Shared package extraction

## Development Guidelines

### Adding New Features

1. Create route in `app/{feature}/page.tsx`
2. Add navigation item in `components/layout/portal-nav.tsx`
3. Use shared UI components from `components/ui/`
4. Follow the existing TypeScript patterns

### Component Structure

- Use client components (`'use client'`) for interactivity
- Server components by default for static content
- Keep components small and focused
- Extract reusable logic to custom hooks

### Styling

- Use Tailwind utility classes
- Follow the design system colors (purple/blue gradients)
- Dark mode variants with `dark:` prefix
- Responsive breakpoints: `sm:`, `md:`, `lg:`, `xl:`

## License

Part of the Etrid Protocol ecosystem.

---

**Built by Agent 1 for the Etrid Unified Portal Migration**
