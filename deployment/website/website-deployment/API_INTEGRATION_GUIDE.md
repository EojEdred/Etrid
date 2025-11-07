# Telemetry API Integration Guide

## Overview

The Interactive Flame Telemetry System is currently running with realistic mock data. When your telemetry API is ready at `https://telemetry.etrid.org`, follow this guide to integrate it.

## Current Status

✅ **Frontend Complete** - All UI, animations, and data display working perfectly
⏳ **API Integration** - Ready to connect when backend is available
🎯 **Mock Data** - Realistic simulated data for all chains

## API Endpoints Expected

### 1. FlareChain Data
```
GET https://telemetry.etrid.org/api/flarechain

Expected Response:
{
  "nodes": 21,
  "uptime": 99.8,
  "blockHeight": 8342156,
  "tps": 847,
  "avgTps": 723,
  "peakTps": 1000,
  "minTps": 512,
  "health": "excellent",
  "validators": [
    {
      "name": "Validator-01",
      "location": "US-East",
      "uptime": 99.9,
      "stake": "125K ETR"
    },
    ...
  ]
}
```

### 2. PBC Chains Data
```
GET https://telemetry.etrid.org/api/pbcs

Expected Response:
{
  "chains": {
    "pbc-btc": {
      "name": "PBC-BTC",
      "desc": "Bitcoin bridge chain",
      "nodes": 5,
      "uptime": 99.5,
      "blockHeight": 2341234,
      "tps": 142,
      "transfers": "1,234",
      "volume": "45.2 BTC",
      "bridge": "active",
      "collators": [...]
    },
    "pbc-eth": { ... },
    ...
  }
}
```

### 3. Lightning-Bloc Data
```
GET https://telemetry.etrid.org/api/lightning

Expected Response:
{
  "channels": 1523,
  "nodes": 342,
  "uptime": 99.9,
  "tps": 12453,
  "avgTps": 9876,
  "peakTps": 18234,
  "minTps": 5432,
  "volume24h": "2.3M ETR",
  "txs24h": "8.7M",
  "avgFee": "0.0001 ETR"
}
```

### 4. TPS History (Optional but Recommended)
```
GET https://telemetry.etrid.org/api/tps-history?chain=flarechain&range=60

Expected Response:
{
  "chain": "flarechain",
  "dataPoints": [
    { "timestamp": 1699099200, "tps": 723 },
    { "timestamp": 1699099205, "tps": 847 },
    ...
  ]
}
```

## Integration Steps

### Step 1: Create API Service Module

In your `index.html`, replace the mock data section with:

```javascript
// ========================================
// TELEMETRY API SERVICE
// ========================================

const TelemetryAPI = {
    baseUrl: 'https://telemetry.etrid.org/api',

    async fetchFlarechainData() {
        try {
            const response = await fetch(`${this.baseUrl}/flarechain`);
            if (!response.ok) throw new Error('Failed to fetch FlareChain data');
            return await response.json();
        } catch (error) {
            console.error('FlareChain API error:', error);
            return this.getFallbackFlarechainData();
        }
    },

    async fetchPBCData() {
        try {
            const response = await fetch(`${this.baseUrl}/pbcs`);
            if (!response.ok) throw new Error('Failed to fetch PBC data');
            return await response.json();
        } catch (error) {
            console.error('PBC API error:', error);
            return this.getFallbackPBCData();
        }
    },

    async fetchLightningData() {
        try {
            const response = await fetch(`${this.baseUrl}/lightning`);
            if (!response.ok) throw new Error('Failed to fetch Lightning data');
            return await response.json();
        } catch (error) {
            console.error('Lightning API error:', error);
            return this.getFallbackLightningData();
        }
    },

    async fetchTPSHistory(chain, range = 60) {
        try {
            const response = await fetch(`${this.baseUrl}/tps-history?chain=${chain}&range=${range}`);
            if (!response.ok) throw new Error('Failed to fetch TPS history');
            return await response.json();
        } catch (error) {
            console.error('TPS History API error:', error);
            return this.generateMockTPSHistory(chain, range);
        }
    },

    // Fallback to mock data if API fails
    getFallbackFlarechainData() {
        return mockTelemetryData.flarechain;
    },

    getFallbackPBCData() {
        return mockTelemetryData.pbcs;
    },

    getFallbackLightningData() {
        return mockTelemetryData.lightning;
    },

    generateMockTPSHistory(chain, range) {
        const data = [];
        const baseValue = chain === 'flarechain' ? 700 : 10000;
        for (let i = 0; i < range; i++) {
            data.push(baseValue + Math.random() * (baseValue / 2));
        }
        return { dataPoints: data };
    }
};
```

### Step 2: Update Data Loading Functions

Replace the current `loadFlarechainData()`:

```javascript
async function loadFlarechainData() {
    // Show loading state
    document.getElementById('fc-nodes').innerHTML = '<span class="loading-spinner"></span>';
    document.getElementById('fc-uptime').innerHTML = '<span class="loading-spinner"></span>';
    document.getElementById('fc-height').innerHTML = '<span class="loading-spinner"></span>';
    document.getElementById('fc-tps').innerHTML = '<span class="loading-spinner"></span>';

    // Fetch real data
    const data = await TelemetryAPI.fetchFlarechainData();

    // Update UI with real data
    document.getElementById('fc-nodes').textContent = data.nodes;
    document.getElementById('fc-uptime').textContent = data.uptime + '%';
    document.getElementById('fc-height').textContent = data.blockHeight.toLocaleString();
    document.getElementById('fc-tps').textContent = data.tps.toLocaleString();

    // Update health
    document.getElementById('fc-health').textContent = data.health.charAt(0).toUpperCase() + data.health.slice(1);
    const healthIndicator = document.getElementById('fc-health-indicator');
    healthIndicator.className = 'health-indicator health-' + data.health;

    // Update TPS stats
    document.getElementById('fc-avg-tps').textContent = data.avgTps.toLocaleString();
    document.getElementById('fc-peak-tps').textContent = data.peakTps.toLocaleString();
    document.getElementById('fc-min-tps').textContent = data.minTps.toLocaleString();

    // Load validators
    const validatorList = document.getElementById('fc-validators');
    validatorList.innerHTML = data.validators.map(v => `
        <div class="node-item">
            <div class="flex-1">
                <div class="font-semibold text-white">${v.name}</div>
                <div class="text-sm text-gray-400">${v.location}</div>
            </div>
            <div class="text-right">
                <div class="text-green-400 font-semibold">${v.uptime}%</div>
                <div class="text-sm text-gray-400">${v.stake}</div>
            </div>
        </div>
    `).join('');

    // Load TPS history
    const history = await TelemetryAPI.fetchTPSHistory('flarechain');
    if (history.dataPoints) {
        tpsHistory.flarechain = history.dataPoints;
        drawChart('fc-chart-canvas', tpsHistory.flarechain, '#3B82F6');
    }
}
```

### Step 3: Update Real-Time Updates

Replace `updateTelemetryData()`:

```javascript
async function updateTelemetryData() {
    if (currentModal?.id === 'flarechain-modal') {
        const data = await TelemetryAPI.fetchFlarechainData();
        document.getElementById('fc-tps').textContent = data.tps.toLocaleString();
        document.getElementById('fc-height').textContent = data.blockHeight.toLocaleString();

        const history = await TelemetryAPI.fetchTPSHistory('flarechain');
        if (history.dataPoints) {
            tpsHistory.flarechain = history.dataPoints;
            drawChart('fc-chart-canvas', tpsHistory.flarechain, '#3B82F6');
        }
    }

    if (currentModal?.id === 'lightning-modal') {
        const data = await TelemetryAPI.fetchLightningData();
        document.getElementById('lb-tps').textContent = data.tps.toLocaleString();

        const history = await TelemetryAPI.fetchTPSHistory('lightning');
        if (history.dataPoints) {
            tpsHistory.lightning = history.dataPoints;
            drawChart('lb-chart-canvas', tpsHistory.lightning, '#FBBF24');
        }
    }

    if (currentModal?.id === 'pbc-modal' && selectedPBC) {
        const data = await TelemetryAPI.fetchPBCData();
        const pbc = data.chains[selectedPBC];
        if (pbc) {
            document.getElementById('pbc-height').textContent = pbc.blockHeight.toLocaleString();
            document.getElementById('pbc-tps').textContent = pbc.tps.toLocaleString();
        }
    }
}
```

### Step 4: Add Error Handling UI

Add error state displays:

```javascript
function showError(containerId, message) {
    const container = document.getElementById(containerId);
    if (container) {
        container.innerHTML = `
            <div class="text-center py-4">
                <div class="text-red-400 mb-2">⚠️ Error Loading Data</div>
                <div class="text-sm text-gray-400">${message}</div>
                <button onclick="retryLoad('${containerId}')" class="mt-2 px-4 py-2 bg-red-500/20 text-red-400 rounded hover:bg-red-500/30 transition-all">
                    Retry
                </button>
            </div>
        `;
    }
}

function retryLoad(containerId) {
    if (currentModal?.id === 'flarechain-modal') {
        loadFlarechainData();
    } else if (currentModal?.id === 'pbc-modal') {
        loadPBCSelector();
    } else if (currentModal?.id === 'lightning-modal') {
        loadLightningData();
    }
}
```

## Testing the Integration

### 1. Test API Endpoints First

```bash
# Test FlareChain endpoint
curl https://telemetry.etrid.org/api/flarechain

# Test PBC endpoint
curl https://telemetry.etrid.org/api/pbcs

# Test Lightning endpoint
curl https://telemetry.etrid.org/api/lightning

# Test TPS history
curl "https://telemetry.etrid.org/api/tps-history?chain=flarechain&range=60"
```

### 2. Verify Response Structure

Check that your API returns data in the expected format. If your structure is different, update the mapping code.

### 3. Test Error Handling

```javascript
// Test with invalid endpoint
TelemetryAPI.baseUrl = 'https://invalid.url/api';
// Should fall back to mock data gracefully
```

### 4. Monitor Console

```javascript
// Enable verbose logging during integration
console.log('[Telemetry] Fetching FlareChain data...');
console.log('[Telemetry] Response:', data);
```

## CORS Configuration

Make sure your API has proper CORS headers:

```
Access-Control-Allow-Origin: https://etrid.org
Access-Control-Allow-Methods: GET
Access-Control-Allow-Headers: Content-Type
Access-Control-Max-Age: 86400
```

## Rate Limiting

Current update interval: 5 seconds

If your API has rate limits:
```javascript
// Adjust the interval
telemetryInterval = setInterval(updateTelemetryData, 10000); // 10 seconds

// Or implement exponential backoff
let retryDelay = 5000;
async function fetchWithBackoff() {
    try {
        const data = await TelemetryAPI.fetchFlarechainData();
        retryDelay = 5000; // Reset on success
        return data;
    } catch (error) {
        retryDelay = Math.min(retryDelay * 2, 60000); // Max 1 minute
        setTimeout(fetchWithBackoff, retryDelay);
    }
}
```

## Caching Strategy

Add client-side caching to reduce API calls:

```javascript
const cache = {
    data: {},
    timestamps: {},
    ttl: 5000, // 5 seconds

    set(key, value) {
        this.data[key] = value;
        this.timestamps[key] = Date.now();
    },

    get(key) {
        if (this.data[key] && (Date.now() - this.timestamps[key]) < this.ttl) {
            return this.data[key];
        }
        return null;
    }
};

// Use in API calls
async fetchFlarechainData() {
    const cached = cache.get('flarechain');
    if (cached) return cached;

    const data = await fetch(...);
    cache.set('flarechain', data);
    return data;
}
```

## Monitoring & Debugging

### Add Telemetry Status Indicator

```html
<!-- Add to page footer or header -->
<div id="telemetry-status" class="fixed bottom-4 right-4 text-xs">
    <span class="health-indicator health-excellent"></span>
    <span>Telemetry: Live</span>
</div>
```

```javascript
function updateTelemetryStatus(status) {
    const indicator = document.querySelector('#telemetry-status .health-indicator');
    const text = document.querySelector('#telemetry-status span:last-child');

    if (status === 'live') {
        indicator.className = 'health-indicator health-excellent';
        text.textContent = 'Telemetry: Live';
    } else if (status === 'error') {
        indicator.className = 'health-indicator health-critical';
        text.textContent = 'Telemetry: Error';
    } else if (status === 'fallback') {
        indicator.className = 'health-indicator health-warning';
        text.textContent = 'Telemetry: Mock Data';
    }
}
```

## Deployment Checklist

- [ ] API endpoints are live and accessible
- [ ] CORS is properly configured
- [ ] Response format matches expected structure
- [ ] Error handling is implemented
- [ ] Fallback to mock data works
- [ ] Rate limiting is respected
- [ ] Console logs are removed (or set to debug mode)
- [ ] Testing completed on staging
- [ ] Performance tested under load
- [ ] Monitoring/alerting is set up

## Contact for API Questions

When ready to integrate:
1. Check API documentation
2. Test endpoints with curl/Postman
3. Update the baseUrl in code
4. Deploy to staging first
5. Monitor for 24 hours
6. Deploy to production

---

**Current Mode**: Mock Data (works perfectly!)
**Ready for**: Real API integration when backend is ready
**Location**: `/Users/macbook/Desktop/etrid/etrid-hostinger-deployment/website/index.html`

**Lines to Update**: Search for "mockTelemetryData" and replace with API calls following this guide.
