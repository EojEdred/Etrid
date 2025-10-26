# MasterChef Dashboard

Real-time dashboard for monitoring ÉTR MasterChef LP Rewards program.

## Features

- ✅ Real-time pool statistics (TVL, APR, staked amounts)
- ✅ Emissions tracking (daily, monthly, yearly)
- ✅ MasterChef balance and days remaining
- ✅ TVL distribution chart
- ✅ Auto-refresh every 60 seconds
- ✅ Responsive design
- ✅ Price feed integration

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Styling**: Tailwind CSS
- **Charts**: Recharts
- **Blockchain**: ethers.js v6
- **Data Fetching**: SWR

## Quick Start

### 1. Install Dependencies

```bash
cd apps/masterchef-dashboard
npm install
```

### 2. Configure Metrics Source

The dashboard fetches metrics from `/public/metrics.json`. You have several options:

**Option A: Copy metrics manually (development)**
```bash
# Export metrics from BSC scripts
cd ../../05-multichain/bridge/adapters/bsc
npm run export-metrics:mainnet

# Copy to dashboard
cp metrics-*.json ../../apps/masterchef-dashboard/public/metrics.json
```

**Option B: Automated sync (production)**

Set up a cron job to automatically export and sync metrics:

```bash
# Add to crontab
0 * * * * cd /path/to/bsc && npm run export-metrics:mainnet && cp metrics-*.json /path/to/dashboard/public/metrics.json
```

**Option C: API endpoint (recommended for production)**

Create `app/api/metrics/route.ts`:

```typescript
import { NextResponse } from 'next/server';
import { readFileSync } from 'fs';
import { join } from 'path';

export async function GET() {
  try {
    const metricsPath = join(process.cwd(), '../../../05-multichain/bridge/adapters/bsc/metrics-latest.json');
    const data = readFileSync(metricsPath, 'utf-8');
    const metrics = JSON.parse(data);

    return NextResponse.json(metrics, {
      headers: {
        'Cache-Control': 'public, s-maxage=60',
      },
    });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to load metrics' }, { status: 500 });
  }
}
```

Then update `lib/api.ts` to fetch from `/api/metrics`.

### 3. Run Development Server

```bash
npm run dev
```

Visit http://localhost:3001

### 4. Build for Production

```bash
npm run build
npm start
```

## Deployment

### Option 1: Vercel (Recommended)

1. Push to GitHub
2. Import project in Vercel
3. Set build settings:
   - **Framework Preset**: Next.js
   - **Root Directory**: `apps/masterchef-dashboard`
   - **Build Command**: `npm run build`
4. Deploy

### Option 2: Docker

```dockerfile
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .
RUN npm run build

EXPOSE 3001

CMD ["npm", "start"]
```

Build and run:
```bash
docker build -t masterchef-dashboard .
docker run -p 3001:3001 masterchef-dashboard
```

### Option 3: Static Export

For hosting on Nginx, Apache, or CDN:

```bash
npm run export
```

This creates a static site in the `out/` directory.

## Configuration

### Environment Variables

Create `.env.local`:

```env
# Metrics source (if using API)
METRICS_API_URL=https://your-api.com/metrics

# Analytics (optional)
NEXT_PUBLIC_GA_ID=G-XXXXXXXXXX

# Network (optional override)
NEXT_PUBLIC_NETWORK=mainnet
```

### Customization

**Colors**: Edit `tailwind.config.js`
**Chart colors**: Edit `components/TVLChart.tsx`
**Refresh interval**: Edit `app/page.tsx` (default: 60000ms)

## Monitoring

The dashboard automatically:
- Refreshes data every 60 seconds
- Shows loading states
- Handles errors gracefully
- Displays network status

## Security

**Important:**
- Never expose private keys
- Use read-only RPC endpoints
- Implement rate limiting for API routes
- Enable CORS only for your domain

## Troubleshooting

### "Failed to load metrics"

- Check that `metrics.json` exists in `public/`
- Verify the JSON format is valid
- Check browser console for errors

### Stale data

- Ensure your metrics export script is running
- Check the timestamp in the dashboard header
- Verify cron job is executing

### Charts not displaying

- Check that TVL data is available (requires price feeds)
- Verify pools have `tvlUSD` values
- Check browser console for Recharts errors

## Development

### Project Structure

```
apps/masterchef-dashboard/
├── app/
│   ├── page.tsx          # Main dashboard page
│   └── layout.tsx        # Root layout
├── components/
│   ├── PoolCard.tsx      # Pool statistics card
│   ├── StatsOverview.tsx # Overview metrics
│   └── TVLChart.tsx      # TVL distribution chart
├── lib/
│   └── api.ts            # API client for fetching metrics
├── types/
│   └── index.ts          # TypeScript types
├── public/
│   └── metrics.json      # Metrics data (symlink or copy)
└── package.json
```

### Adding New Features

1. **New Chart**: Create component in `components/`, import in `app/page.tsx`
2. **New Metric**: Add to types in `types/index.ts`, display in components
3. **Historical Data**: Store metrics in database, create API endpoint

## Performance

- **Bundle size**: ~200KB gzipped
- **First load**: < 2s
- **Metrics refresh**: 60s interval
- **Caching**: 60s CDN cache recommended

## License

MIT

## Support

- **Issues**: https://github.com/etrid/etrid-protocol/issues
- **Discord**: #masterchef-dashboard
- **Email**: support@etrid.io
