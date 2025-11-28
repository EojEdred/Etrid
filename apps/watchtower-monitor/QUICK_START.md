# Watchtower Monitor - Quick Start Guide

## Overview

A Next.js 14 application for real-time monitoring and fraud detection in Lightning-Bloc payment channels.

## Quick Start

### 1. Install Dependencies
```bash
cd apps/watchtower-monitor
npm install
```

### 2. Run Development Server
```bash
npm run dev
```

Access at: **http://localhost:3003**

### 3. Explore the Application

#### Monitor Dashboard (/)
- View active channels and system stats
- Monitor real-time fraud alerts
- Track watchtower reputation and earnings

#### Reports (/reports)
- Browse fraud detection history
- Filter by severity, status, or search term
- Export reports as JSON

#### Settings (/settings)
- Configure node endpoints
- Set alert thresholds
- Manage notification preferences

## Key Features

### Real-Time Monitoring
- WebSocket-based live updates
- Auto-reconnection with exponential backoff
- Connection status indicator
- Heartbeat mechanism for health checks

### Fraud Detection
- 4 fraud pattern types: old state broadcast, double spend, invalid signature, unauthorized close
- 4 severity levels: low, medium, high, critical
- Real-time alerting with notifications
- Intervention tracking and penalty calculation

### Data Visualization
- Recharts integration for time-series data
- Line, Area, and Bar chart support
- Responsive design with custom tooltips
- Stats cards with real-time updates

## Configuration

### Node Settings (via /settings page)
```typescript
{
  nodeEndpoint: 'http://localhost:9944',
  wsEndpoint: 'ws://localhost:9944',
  pollingInterval: 5000, // ms
  alertThresholds: {
    responseTime: 1000,      // ms
    balanceDeviation: 0.1    // 0-1 (10%)
  },
  notifications: {
    email: true,
    push: false,
    webhook: 'https://your-webhook-url.com/alerts'
  }
}
```

Configuration is saved to **localStorage** as `watchtower-config`.

## Project Structure

```
src/
├── app/                    # Next.js 14 App Router
│   ├── layout.tsx         # Root layout
│   ├── page.tsx           # Monitor dashboard
│   ├── reports/page.tsx   # Fraud reports
│   └── settings/page.tsx  # Configuration
├── components/            # React components
│   ├── Navigation.tsx
│   ├── Footer.tsx
│   ├── WebSocketStatus.tsx
│   ├── MonitoringChart.tsx
│   └── ... (existing components)
├── hooks/                 # Custom React hooks
│   ├── useChannelMonitoring.ts
│   └── useFraudDetection.ts
├── lib/                   # Utilities
│   ├── websocket.ts      # WebSocket service
│   └── utils.ts
└── types/                 # TypeScript types
    └── index.ts
```

## WebSocket Integration

### Client Service
```typescript
import { getWatchtowerWebSocket } from '@/lib/websocket';

const ws = getWatchtowerWebSocket();

// Subscribe to messages
ws.onMessage((message) => {
  console.log('Received:', message);
});

// Subscribe to status changes
ws.onStatusChange((status) => {
  console.log('Status:', status);
});

// Connect
ws.connect();
```

### Message Format
```typescript
interface WebSocketMessage {
  type: 'channel_update' | 'fraud_alert' | 'subscription_update' | 'earnings_update';
  payload: any;
  timestamp: Date;
}
```

## Mock Data vs Production

The app currently runs with **mock data** for demonstration. To connect to a real blockchain node:

1. Update settings (/settings) with your node endpoints
2. Replace mock data in hooks with actual API calls
3. Connect WebSocket service to real server
4. Implement authentication if needed

### Example: Replace Mock Data
```typescript
// In useChannelMonitoring.ts
const fetchChannels = async () => {
  // Replace this:
  const mockChannels = generateMockChannels(15);

  // With this:
  const response = await fetch('/api/channels');
  const channels = await response.json();

  setChannels(channels);
};
```

## Development Commands

```bash
# Development server
npm run dev

# Production build
npm run build
npm start

# Type checking
npm run type-check

# Linting
npm run lint
```

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Charts**: Recharts
- **Icons**: Lucide React
- **UI Components**: Radix UI
- **State Management**: React Hooks
- **WebSocket**: Custom service with auto-reconnect

## Dependencies

```json
{
  "@tanstack/react-query": "latest",    // (Ready for data fetching)
  "socket.io-client": "latest",          // WebSocket client
  "recharts": "2.15.4",                  // Charts
  "lucide-react": "^0.454.0",            // Icons
  "next": "15.2.4",                      // Framework
  "react": "^19",                        // UI library
  // ... + Radix UI components
}
```

## Troubleshooting

### WebSocket Connection Issues
- Check that node endpoints are correct in Settings
- Verify firewall/network allows WebSocket connections
- Check browser console for connection errors
- Use WebSocketStatus component to monitor connection

### Build Errors
```bash
# Clear Next.js cache
rm -rf .next

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

### Type Errors
```bash
# Run type checking
npm run type-check

# Check TypeScript version
npx tsc --version
```

## Next Steps

1. **Connect to Real Node**: Update endpoints and replace mock data
2. **Add Authentication**: Implement user login and access control
3. **Backend Integration**: Create API routes for data persistence
4. **Testing**: Add unit/integration tests with Jest
5. **Deployment**: Deploy to Vercel, Netlify, or custom server

## Support

- **Documentation**: See IMPLEMENTATION_REPORT.md for full details
- **Issues**: Check browser console and Next.js error overlay
- **Configuration**: All settings available in /settings page

---

**Built with Next.js 14 + TypeScript + Tailwind CSS**

Ready for development and production deployment!
