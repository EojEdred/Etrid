# Ëtrid Validator Dashboard

A comprehensive, real-time monitoring and management interface for Ëtrid Protocol validators. Built with Next.js, TypeScript, and Polkadot.js, this dashboard provides validators with complete visibility into their operations, performance metrics, and nominator management.

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Next.js](https://img.shields.io/badge/Next.js-14.0-black.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.3-blue.svg)

## Features

### Real-Time Monitoring
- **Live Validator Status**: Monitor your validator's active/inactive state, election status, and blocking status
- **Network Statistics**: Track total issuance, staking rate, active validators, and network health
- **Session & Era Information**: Real-time progress tracking with countdown timers
- **Auto-Refresh**: Data updates every 30 seconds automatically

### Performance Analytics
- **Uptime Tracking**: Monitor validator uptime with 99.9%+ accuracy
- **Block Production**: Track blocks produced, missed blocks, and production rate
- **Era Points**: View points earned per era with historical trends
- **Performance Score**: Overall performance rating based on multiple metrics
- **Validator Ranking**: See your position among all network validators
- **Historical Charts**: 30-day performance history with interactive visualizations

### Nominator Management
- **Complete Nominator List**: View all nominators with stake amounts and activity status
- **Sorting & Filtering**: Sort by stake, rewards, or time; search by address
- **Stake Distribution**: Visual breakdown of stake percentages
- **Nominator History**: Track when nominators joined and their reward history
- **Export Capabilities**: Export nominator data for analysis

### Reward Tracking
- **Historical Rewards**: View up to 90 days of reward history
- **Interactive Charts**: Line, area, and bar chart visualizations
- **Total Earnings**: Lifetime and period-specific reward calculations
- **Average Rewards**: Track average earnings per era
- **Commission Impact**: See how commission affects earnings
- **Export Reports**: Download reward data for accounting

### Commission Management
- **Dynamic Adjustment**: Adjust commission rate from 0-100%
- **Real-Time Preview**: See estimated annual income at different rates
- **Impact Analysis**: Understand how commission affects nominator attraction
- **Payment Destination**: Configure where rewards are sent
- **Auto-Compound**: Enable automatic reward staking
- **Best Practices**: Built-in guidance for optimal commission settings

### Alert System
- **Real-Time Notifications**: Instant alerts for critical events
- **Multiple Alert Types**:
  - Missed blocks
  - Low nominator count
  - Commission changes
  - Node connectivity issues
  - Performance degradation
- **Notification Channels**:
  - In-app notifications
  - Email alerts
  - Discord webhooks
- **Alert History**: Review past notifications and actions taken

### Settings & Configuration
- **Node Configuration**: Connect to custom Ëtrid node endpoints
- **Security Management**: View and rotate session keys
- **Notification Preferences**: Customize alert types and delivery methods
- **Validator Settings**: Manage commission, payment destination, and auto-compound
- **Account Management**: Configure stash and controller accounts

## Tech Stack

- **Frontend Framework**: Next.js 14 with Pages Router
- **Language**: TypeScript 5.3
- **Styling**: TailwindCSS 3.4 with custom Ëtrid theme
- **Blockchain Integration**: Polkadot.js API v10.11+
- **Charts & Visualization**: Recharts 2.10
- **State Management**: Zustand 4.4
- **Date Handling**: date-fns 2.30
- **Icons**: Lucide React 0.294

## Installation

### Prerequisites

- Node.js 18.0.0 or higher
- npm 9.0.0 or higher
- Running Ëtrid node (local or remote)
- Validator account with active stake

### Setup

1. **Clone the repository**
   ```bash
   cd apps/validator-dashboard
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your configuration:
   ```env
   NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
   NEXT_PUBLIC_NETWORK_NAME=Ëtrid
   NEXT_PUBLIC_CHAIN_DECIMALS=18
   NEXT_PUBLIC_CHAIN_TOKEN=ETRID
   NEXT_PUBLIC_VALIDATOR_ADDRESS=your_validator_address
   ```

4. **Start development server**
   ```bash
   npm run dev
   ```

   The dashboard will be available at `http://localhost:3002`

5. **Build for production**
   ```bash
   npm run build
   npm start
   ```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `NEXT_PUBLIC_WS_PROVIDER` | WebSocket endpoint for Ëtrid node | `ws://localhost:9944` |
| `NEXT_PUBLIC_NETWORK_NAME` | Network display name | `Ëtrid` |
| `NEXT_PUBLIC_CHAIN_DECIMALS` | Token decimal places | `18` |
| `NEXT_PUBLIC_CHAIN_TOKEN` | Token symbol | `ETRID` |
| `NEXT_PUBLIC_VALIDATOR_ADDRESS` | Your validator stash address | - |
| `NEXT_PUBLIC_ALERT_EMAIL` | Email for alerts (optional) | - |
| `NEXT_PUBLIC_DISCORD_WEBHOOK` | Discord webhook URL (optional) | - |

### Node Connection

The dashboard connects to an Ëtrid node via WebSocket. You can connect to:

- **Local node**: `ws://localhost:9944`
- **Remote node**: `wss://your-node.example.com`
- **Public endpoint**: `wss://rpc.etrid.network` (if available)

Ensure your node has:
- RPC and WebSocket enabled
- Proper firewall rules
- Sufficient resources for queries

## Usage

### Dashboard Overview

The main dashboard provides a comprehensive view of your validator:

1. **Session Banner**: Current era/session with progress indicators
2. **Validator Stats**: Total stake, nominators, commission, era points
3. **Reward History**: Interactive charts showing earnings over time
4. **Nominator List**: All nominators with sorting and filtering
5. **Alerts Panel**: Recent notifications and warnings
6. **Quick Stats**: Uptime, rank, blocks produced/missed
7. **Network Stats**: Staking rate, inflation, validator counts

### Performance Page

Deep dive into validator performance:

1. **Performance Metrics**: Detailed stats on blocks, uptime, and era points
2. **Overall Score**: Composite performance rating (0-100)
3. **Uptime History**: 30-day uptime and block production charts
4. **Block Production**: Pie chart of successful vs missed blocks
5. **Validator Ranking**: Current rank and top percentile
6. **Performance Insights**: AI-generated recommendations

### Settings Page

Configure your validator:

1. **Commission Settings**:
   - Adjust rate with visual slider
   - View estimated income at different rates
   - Configure payment destination
   - Enable auto-compound

2. **Notifications**:
   - Enable/disable alerts
   - Configure email notifications
   - Set up Discord webhooks
   - Choose alert types

3. **Node Configuration**:
   - Set custom node endpoint
   - Test connection
   - View connection status

4. **Security**:
   - View session keys
   - Rotate keys (requires node access)
   - Manage account security

## Screenshots Descriptions

### Dashboard (Main Page)
**Layout**: Three-column responsive grid with hero banner
- **Top**: Session information banner with era/session progress, countdown timers
- **Row 1**: Four stat cards showing Total Stake, Nominators, Commission, Era Points
- **Row 2**: Left 2/3 - Reward history chart with time range selector (7d/30d/90d/all)
- **Row 2**: Right 1/3 - Alerts panel with notification badges, Quick Stats card, Network Stats card
- **Row 3**: Left 2/3 - Nominator table with search, sorting, stake amounts, addresses
- **Colors**: Gradient blues/purples for headers, white cards with subtle shadows
- **Interactions**: Hover effects on cards, sortable tables, clickable chart data points

### Performance Page
**Layout**: Full-width analytical dashboard
- **Top**: Time range selector (24h/7d/30d/90d)
- **Row 1**: Four metric cards with trend indicators (Blocks Produced, Uptime, Avg Block Time, Era Points)
- **Row 2**: Large gradient banner showing overall performance score (98.5/100) with progress bar
- **Row 3**: Left 2/3 - Dual-axis area chart showing uptime % and blocks produced over 30 days
- **Row 3**: Right 1/3 - Donut chart of block production (successful vs missed)
- **Row 4**: Three cards showing validator ranking stats (Current Rank, Total Validators, Top Percentile)
- **Row 5**: Performance insights with colored alert boxes (green success, blue info, yellow warning)
- **Colors**: Green for success metrics, red for errors, blue/purple gradients for scores
- **Interactions**: Hover tooltips on charts, animated progress bars, filterable time ranges

### Settings Page
**Layout**: Stacked full-width sections
- **Section 1**: Commission Settings
  - Slider control (0-100%) with numeric input
  - Real-time commission rate display
  - Payment destination dropdown
  - Auto-compound toggle switch
  - Three cards showing estimated annual income at different stake levels
  - Save button with loading state
- **Section 2**: Notification Settings
  - Master enable/disable toggle
  - Email notifications toggle + email input field
  - Discord notifications toggle + webhook URL input
  - Alert types checklist (6 options: Missed Blocks, Low Nominators, etc.)
  - Save button
- **Section 3**: Node Configuration
  - WebSocket endpoint text input
  - Connection status indicator (green checkmark or red X)
  - Info box with connection details
- **Section 4**: Security
  - Session keys display (monospace font, copyable)
  - "Rotate Session Keys" button
  - Warning alerts about key management
- **Colors**: Section headers with colored icons (blue bell, purple server, orange shield)
- **Interactions**: Toggle switches animate, input validation, save confirmations

## API Integration

### Polkadot.js Integration

The dashboard uses `@polkadot/api` to interact with the Ëtrid blockchain:

```typescript
// Example: Fetching validator info
const exposure = await api.query.staking.erasStakers(era, validatorAddress);
const commission = await api.query.staking.validators(validatorAddress);
const sessionKeys = await api.query.session.nextKeys(validatorAddress);
```

### Custom Hooks

**`useValidatorStats`**: Main hook for fetching validator data

```typescript
const {
  api,                  // Polkadot.js API instance
  isConnected,          // Connection status
  isLoading,            // Loading state
  error,                // Error message
  validatorInfo,        // Validator details
  nominators,           // Nominator list
  rewards,              // Reward history
  performance,          // Performance metrics
  sessionInfo,          // Session/era data
  networkStats,         // Network statistics
  refreshData,          // Manual refresh function
} = useValidatorStats(validatorAddress);
```

### Data Refresh

- **Automatic**: Data refreshes every 30 seconds
- **Manual**: Click refresh button in dashboard header
- **On-Demand**: Call `refreshData()` function

## Development

### Project Structure

```
validator-dashboard/
├── src/
│   ├── components/       # React components
│   │   ├── Layout.tsx
│   │   ├── ValidatorStats.tsx
│   │   ├── NominatorList.tsx
│   │   ├── RewardHistory.tsx
│   │   ├── CommissionSettings.tsx
│   │   └── AlertsPanel.tsx
│   ├── pages/           # Next.js pages
│   │   ├── index.tsx    # Dashboard
│   │   ├── performance.tsx
│   │   ├── settings.tsx
│   │   ├── _app.tsx
│   │   └── _document.tsx
│   ├── hooks/           # Custom React hooks
│   │   └── useValidatorStats.ts
│   ├── types/           # TypeScript types
│   │   └── index.ts
│   ├── utils/           # Utility functions
│   │   └── format.ts
│   └── styles/          # CSS styles
│       └── globals.css
├── public/              # Static assets
├── package.json
├── tsconfig.json
├── tailwind.config.js
├── next.config.js
└── README.md
```

### Adding New Features

1. **New Component**:
   ```typescript
   // src/components/NewFeature.tsx
   import React from 'react';
   import type { YourType } from '@/types';

   interface NewFeatureProps {
     data: YourType;
   }

   export default function NewFeature({ data }: NewFeatureProps) {
     return <div>{/* Your component */}</div>;
   }
   ```

2. **New Page**:
   ```typescript
   // src/pages/new-page.tsx
   import Layout from '@/components/Layout';

   export default function NewPage() {
     return (
       <Layout>
         {/* Your page content */}
       </Layout>
     );
   }
   ```

3. **New Hook**:
   ```typescript
   // src/hooks/useNewFeature.ts
   import { useState, useEffect } from 'react';

   export function useNewFeature() {
     const [data, setData] = useState(null);
     // Hook logic
     return { data };
   }
   ```

### Testing

```bash
# Type checking
npm run type-check

# Linting
npm run lint

# Build test
npm run build
```

## Deployment

### Production Build

```bash
npm run build
npm start
```

### Docker Deployment

Create `Dockerfile`:
```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
RUN npm run build
EXPOSE 3002
CMD ["npm", "start"]
```

Build and run:
```bash
docker build -t etrid-validator-dashboard .
docker run -p 3002:3002 etrid-validator-dashboard
```

### Environment-Specific Configuration

For production, ensure:
- Use secure WebSocket connection (wss://)
- Set proper CORS policies
- Enable rate limiting
- Use environment variables for secrets
- Configure proper logging

## Troubleshooting

### Connection Issues

**Problem**: Cannot connect to node
**Solution**:
- Verify node is running: `curl http://localhost:9944`
- Check WebSocket is enabled in node config
- Ensure firewall allows connection
- Try alternative endpoint

### Performance Issues

**Problem**: Dashboard is slow
**Solution**:
- Reduce auto-refresh interval
- Limit historical data range
- Check node performance
- Clear browser cache

### Missing Data

**Problem**: Some metrics show as 0 or N/A
**Solution**:
- Verify validator is active and elected
- Check validator address is correct
- Ensure node is fully synced
- Wait for next era/session

### Build Errors

**Problem**: Build fails with type errors
**Solution**:
```bash
rm -rf node_modules .next
npm install
npm run type-check
npm run build
```

## Security Considerations

- **Never expose private keys** in the dashboard or environment variables
- **Use read-only RPC** connections when possible
- **Validate all inputs** before sending transactions
- **Keep dependencies updated** for security patches
- **Use HTTPS/WSS** in production
- **Implement rate limiting** to prevent abuse
- **Audit smart contract interactions** before enabling

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

- **Documentation**: [https://docs.etrid.network](https://docs.etrid.network)
- **Issues**: GitHub Issues
- **Discord**: [https://discord.gg/etrid](https://discord.gg/etrid)
- **Email**: support@etrid.network

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Next.js](https://nextjs.org/)
- Powered by [Polkadot.js](https://polkadot.js.org/)
- Charts by [Recharts](https://recharts.org/)
- Icons by [Lucide](https://lucide.dev/)

---

**Ëtrid Protocol** - Building the future of decentralized governance and multichain interoperability.
