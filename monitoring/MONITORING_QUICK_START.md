# Monitoring Quick Start Guide

## Access Monitoring Dashboard

Navigate to: **http://localhost:3000/monitoring**

---

## Routes

| Route | Description | Status |
|-------|-------------|--------|
| `/monitoring` | Overview dashboard with health cards | ✅ Ready |
| `/monitoring/telemetry` | Network map & node explorer | ✅ Ready |
| `/monitoring/grafana` | Embedded Grafana dashboards | ✅ Ready |
| `/monitoring/prometheus` | Prometheus metrics & queries | ✅ Ready |

---

## Prerequisites

### 1. Start PrimeArc Node (for live data)

```bash
# Ensure at least one node is running
# Default endpoint: ws://20.186.91.207:9944
```

### 2. Start Prometheus (optional)

```bash
# Default: http://localhost:9090
prometheus --config.file=prometheus.yml
```

### 3. Start Grafana (optional)

```bash
# Default: http://localhost:3100
grafana-server
```

---

## Features

### Network Telemetry
- ✅ Live network map with node locations
- ✅ Real-time block height updates
- ✅ Validator status monitoring
- ✅ Node explorer table
- ✅ Auto-refresh every 10 seconds

### Grafana Integration
- ✅ PrimeArc Core Chain dashboard
- ✅ Validator performance metrics
- ✅ Network overview dashboard
- ✅ Kiosk mode embedding
- ✅ External link to full UI

### Prometheus Metrics
- ✅ Quick metric cards
- ✅ Custom PromQL query builder
- ✅ Query history
- ✅ Metric documentation
- ✅ Auto-refresh every 5 seconds

---

## Common Issues

### "Failed to connect to network"
**Solution:** Check if PrimeArc node is running on `ws://20.186.91.207:9944`
- The app will fall back to mock data if connection fails

### "Prometheus metrics not loading"
**Solution:** Start Prometheus on `http://localhost:9090`
- Or update the endpoint in `usePrometheusQuery.ts`

### "Grafana dashboards not visible"
**Solution:** Ensure Grafana is running on `http://localhost:3100`
- Check that dashboard IDs exist in Grafana

---

## Development

### Start Dev Server
```bash
cd /Users/macbook/Desktop/etrid/apps/unified-portal
npm run dev
```

### Build for Production
```bash
npm run build
```

---

## Component Usage Examples

### Using StatCard
```tsx
import { StatCard } from '@/components/monitoring/stat-card'

<StatCard
  title="Best Block"
  value={12450}
  subtitle="Finalized: 12430"
/>
```

### Using Network Stats Hook
```tsx
import { useNetworkStats } from '@/lib/hooks/useNetworkStats'

const { stats, nodes, isLoading } = useNetworkStats()
```

### Using Prometheus Query Hook
```tsx
import { usePrometheusQuery } from '@/lib/hooks/usePrometheusQuery'

const { value, loading, error } = usePrometheusQuery('substrate_block_height')
```

---

## File Locations

```
/apps/unified-portal/
├── app/monitoring/              # Pages
├── components/monitoring/       # Components
└── lib/hooks/                  # Hooks
```

---

**Ready to monitor!** 🚀
