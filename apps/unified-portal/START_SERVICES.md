# Start All Services for Unified Portal

## Required Services

To see real data in the portal, you need these services running:

### 1. Primearc Core Chain Node (Port 9944)
Your main blockchain node that validators connect to.

**Start with Pinokio:**
```bash
# If you have Pinokio managing your validators:
# Use your Pinokio launcher to start the Primearc Core Chain node
```

**Or start manually:**
```bash
cd /Users/macbook/Desktop/etrid
./target/release/primearc-core-chain-node \
  --base-path /tmp/primearc-core-chain-data \
  --chain primearc-core-chain_mainnet.json \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  --validator \
  --name "Primearc Core Chain-Node"
```

### 2. Network Telemetry (Port 8080)
Status: ✅ **RUNNING** (detected on port 8080)

### 3. Governance UI (Port 8082)
Status: ✅ **RUNNING** (detected on port 8082)

### 4. Grafana (Port 3100)
For monitoring dashboards.

**Start:**
```bash
cd /Users/macbook/Desktop/etrid/monitoring
docker-compose up -d grafana
```

### 5. Prometheus (Port 9090)
For metrics collection.

**Start:**
```bash
cd /Users/macbook/Desktop/etrid/monitoring
docker-compose up -d prometheus
```

---

## Quick Start All Services

**Option 1: Using Docker (Recommended)**
```bash
cd /Users/macbook/Desktop/etrid/monitoring
docker-compose up -d
```

**Option 2: Using Pinokio**
If you have Pinokio configured:
```bash
# Start your 21 validators through Pinokio
# This should also start the Primearc Core Chain node
```

---

## Verify Services Are Running

After starting services, check they're accessible:

```bash
# Check Primearc Core Chain node
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944

# Check Telemetry
curl http://localhost:8080/health

# Check Governance
curl http://localhost:8082

# Check Prometheus
curl http://localhost:9090/-/healthy

# Check Grafana
curl http://localhost:3100/api/health
```

---

## Restart Portal to Connect

After services are running, restart the portal:

```bash
cd /Users/macbook/Desktop/etrid/apps/unified-portal

# Kill current dev server
# Then restart:
npm run dev
```

The portal will now connect to:
- ✅ Primearc Core Chain validators (real-time data)
- ✅ Network telemetry (live stats)
- ✅ Governance proposals (real data)
- ✅ Monitoring dashboards (Grafana/Prometheus)

---

## Troubleshooting

### "Cannot connect to ws://127.0.0.1:9944"
- Primearc Core Chain node is not running
- Start the node (see step 1 above)

### "Telemetry shows no data"
- Network telemetry service not running
- Check if port 8080 is accessible

### "Grafana not loading"
- Grafana not running on port 3100
- Start with `docker-compose up -d grafana`

### "Validator stats show 0"
- Node is starting up (wait 30 seconds)
- OR validator address in .env.local is wrong
- Update NEXT_PUBLIC_VALIDATOR_ADDRESS in .env.local

---

## Update Your Validator Address

Edit `.env.local` and set your actual validator address:

```bash
NEXT_PUBLIC_VALIDATOR_ADDRESS=YOUR_ACTUAL_VALIDATOR_ADDRESS_HERE
```

You can find your validator addresses with:
```bash
# If using Pinokio, check your validator list
# OR query the chain:
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "session_validators"}' \
  http://localhost:9944
```
