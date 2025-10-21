# EDSC Bridge - Docker Setup

## Overview

Spin up the complete EDSC cross-chain bridge infrastructure with a single Docker Compose command. Perfect for local development and testing.

## What's Included

The Docker setup includes:

- **Hardhat** - Local Ethereum network (port 8545)
- **FlareChain** - Substrate node (ports 9944, 9933)
- **Redis** - State storage (port 6379)
- **3 Attestation Services** - Sign cross-chain messages (ports 3000-3002)
- **Relayer Service** - Relay messages between chains (port 3010)
- **Prometheus** - Metrics collection (port 9099)
- **Grafana** - Metrics visualization (port 3030)

## Prerequisites

- Docker Desktop or Docker Engine (v20.10+)
- Docker Compose (v2.0+)
- 8GB RAM minimum
- 20GB disk space

## Quick Start

### 1. Build FlareChain Node

First, ensure the FlareChain node is built:

```bash
cd /Users/macbook/Desktop/etrid
cargo build --release
```

### 2. Start All Services

```bash
docker-compose -f docker-compose.bridge.yml up
```

This will:
1. Start Hardhat local Ethereum network
2. Deploy all smart contracts
3. Register attesters
4. Start FlareChain node
5. Start Redis
6. Start 3 attestation services
7. Start relayer service
8. Start Prometheus and Grafana

**Note**: First startup takes ~5-10 minutes to build images and deploy contracts.

### 3. Verify Services

Open these URLs in your browser:

- **Hardhat**: http://localhost:8545
- **FlareChain**: ws://localhost:9944
- **Attestation 1**: http://localhost:3000/health
- **Attestation 2**: http://localhost:3001/health
- **Attestation 3**: http://localhost:3002/health
- **Relayer**: http://localhost:3010/health
- **Prometheus**: http://localhost:9099
- **Grafana**: http://localhost:3030 (admin/admin)

## Usage

### Check Logs

```bash
# All services
docker-compose -f docker-compose.bridge.yml logs -f

# Specific service
docker-compose -f docker-compose.bridge.yml logs -f attestation-1
docker-compose -f docker-compose.bridge.yml logs -f relayer
```

### Restart Services

```bash
# All services
docker-compose -f docker-compose.bridge.yml restart

# Specific service
docker-compose -f docker-compose.bridge.yml restart attestation-1
```

### Stop Services

```bash
docker-compose -f docker-compose.bridge.yml down
```

### Clean Everything (including volumes)

```bash
docker-compose -f docker-compose.bridge.yml down -v
```

## Testing

### Execute Test Transfer

From another terminal while services are running:

```bash
cd contracts/ethereum

# Test transfer (from host machine to Hardhat in Docker)
npx hardhat run scripts/test-transfer.js --network localhost
```

**Note**: The script connects to `localhost:8545` which is exposed from the Docker container.

### Monitor the Transfer

1. Watch attestation service logs:
   ```bash
   docker-compose -f docker-compose.bridge.yml logs -f attestation-1 attestation-2 attestation-3
   ```

2. Watch relayer logs:
   ```bash
   docker-compose -f docker-compose.bridge.yml logs -f relayer
   ```

3. Check Grafana dashboards:
   - Open http://localhost:3030
   - Login: admin/admin
   - View metrics

## Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    Docker Network                         │
│                                                           │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │ Hardhat  │◄───│ Attestation  │───►│  FlareChain  │  │
│  │  :8545   │    │  Services    │    │   :9944      │  │
│  └──────────┘    │  :3000-3002  │    └──────────────┘  │
│                   └──────────────┘                       │
│                          ▲                               │
│                          │                               │
│                   ┌──────┴──────┐                        │
│                   │   Relayer   │                        │
│                   │    :3010    │                        │
│                   └─────────────┘                        │
│                                                           │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │  Redis   │    │  Prometheus  │───►│   Grafana    │  │
│  │  :6379   │    │    :9099     │    │    :3030     │  │
│  └──────────┘    └──────────────┘    └──────────────┘  │
└──────────────────────────────────────────────────────────┘
```

## Ports

| Service | Port | Description |
|---------|------|-------------|
| Hardhat | 8545 | Ethereum JSON-RPC |
| FlareChain | 9944 | WebSocket RPC |
| FlareChain | 9933 | HTTP RPC |
| FlareChain | 30333 | P2P |
| Redis | 6379 | Key-value store |
| Attestation 1 | 3000 | API endpoint |
| Attestation 1 | 9090 | Metrics |
| Attestation 2 | 3001 | API endpoint |
| Attestation 2 | 9091 | Metrics |
| Attestation 3 | 3002 | API endpoint |
| Attestation 3 | 9092 | Metrics |
| Relayer | 3010 | API endpoint |
| Relayer | 9093 | Metrics |
| Prometheus | 9099 | Metrics UI |
| Grafana | 3030 | Dashboards |

## Troubleshooting

### Services Won't Start

1. Check Docker resources:
   ```bash
   docker system df
   docker system prune  # Clean up if needed
   ```

2. Check logs:
   ```bash
   docker-compose -f docker-compose.bridge.yml logs
   ```

### FlareChain Binary Not Found

Make sure you've built the FlareChain node:

```bash
cargo build --release
ls -la target/release/flarechain-node  # Should exist
```

### Port Already in Use

If ports are already in use, you can either:

1. Stop conflicting services:
   ```bash
   lsof -ti:8545 | xargs kill -9
   lsof -ti:9944 | xargs kill -9
   ```

2. Or modify ports in `docker-compose.bridge.yml`

### Contract Deployment Fails

This usually means Hardhat isn't ready. The deployment service waits for Hardhat's health check, but you can manually deploy:

```bash
# Stop deployment service
docker-compose -f docker-compose.bridge.yml stop hardhat-deploy

# Deploy manually
docker-compose -f docker-compose.bridge.yml exec hardhat sh
npx hardhat run scripts/deploy.js --network localhost
npx hardhat run scripts/register-attesters.js --network localhost
npx hardhat run scripts/authorize-token-messenger.js --network localhost
exit
```

### Attestation Services Not Detecting Events

This is expected with Hardhat's event system. For full end-to-end testing, use Sepolia testnet (see [EMBER_DEPLOYMENT_CHECKLIST.md](./EMBER_DEPLOYMENT_CHECKLIST.md)).

## Development Workflow

### 1. Make Code Changes

Edit files in:
- `contracts/ethereum/src/` - Smart contracts
- `services/attestation-service/src/` - Attestation logic
- `services/relayer-service/src/` - Relayer logic

### 2. Rebuild and Restart

```bash
# Rebuild specific service
docker-compose -f docker-compose.bridge.yml build attestation-1
docker-compose -f docker-compose.bridge.yml up -d attestation-1

# Or rebuild all
docker-compose -f docker-compose.bridge.yml build
docker-compose -f docker-compose.bridge.yml up -d
```

### 3. Test Changes

```bash
npx hardhat run scripts/test-transfer.js --network localhost
```

## Production Differences

This Docker setup is for **local development only**. For production/testnet:

1. Use real RPC endpoints (Infura, Alchemy)
2. Use separate VPS instances for each service
3. Enable TLS/SSL
4. Use proper key management
5. Set up monitoring/alerting
6. Use PM2 or systemd instead of Docker Compose

See [EMBER_DEPLOYMENT_CHECKLIST.md](./EMBER_DEPLOYMENT_CHECKLIST.md) for production deployment.

## Advanced Usage

### Custom Configuration

Create a `.env.docker` file:

```env
ETHEREUM_CHAIN_ID=1337
SUBSTRATE_CHAIN_ID=2
CONFIRMATIONS_REQUIRED=1
MIN_SIGNATURES=3
TOTAL_ATTESTERS=5
```

Then reference in docker-compose:

```yaml
attestation-1:
  env_file:
    - .env.docker
```

### Scale Attestation Services

To add more attesters:

```bash
docker-compose -f docker-compose.bridge.yml up -d --scale attestation-3=5
```

**Note**: You'll need to configure unique private keys for each instance.

## Clean Up

### Remove All Containers

```bash
docker-compose -f docker-compose.bridge.yml down
```

### Remove Volumes (Reset State)

```bash
docker-compose -f docker-compose.bridge.yml down -v
```

### Remove Images

```bash
docker-compose -f docker-compose.bridge.yml down --rmi all
```

## Resources

- [EMBER_TESTNET_README.md](./EMBER_TESTNET_README.md) - User guide
- [EMBER_DEPLOYMENT_PLAN.md](./contracts/ethereum/EMBER_DEPLOYMENT_PLAN.md) - Deployment roadmap
- [EMBER_DEPLOYMENT_CHECKLIST.md](./EMBER_DEPLOYMENT_CHECKLIST.md) - Step-by-step checklist

## Support

For issues with Docker setup:
- Check logs: `docker-compose -f docker-compose.bridge.yml logs`
- Check container status: `docker-compose -f docker-compose.bridge.yml ps`
- Open GitHub issue with logs attached

---

**Quick Commands Reference:**

```bash
# Start
docker-compose -f docker-compose.bridge.yml up

# Start in background
docker-compose -f docker-compose.bridge.yml up -d

# Stop
docker-compose -f docker-compose.bridge.yml down

# Restart
docker-compose -f docker-compose.bridge.yml restart

# Logs
docker-compose -f docker-compose.bridge.yml logs -f

# Clean everything
docker-compose -f docker-compose.bridge.yml down -v --rmi all
```
